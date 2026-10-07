//! Boundary-hardening tests: last-error strings are NUL-terminated and
//! per-thread, panics never unwind across `extern "C"`, every exported function
//! is panic-guarded, and the version entry points report the crate version.

use std::ffi::{CStr, CString};

use super::{
    SZCONFIGTOOL_ABI_VERSION, SzConfigTool_addDataSource, SzConfigTool_clearLastError,
    SzConfigTool_getAbiVersion, SzConfigTool_getLastError, SzConfigTool_getLastErrorCode,
    SzConfigTool_getLibraryVersion, SzConfigTool_result, ffi_guard,
};

/// Force an error on the current thread: a NULL config pointer is rejected.
fn trigger_null_pointer_error() -> SzConfigTool_result {
    let code = CString::new("TEST").unwrap();
    unsafe { SzConfigTool_addDataSource(std::ptr::null(), code.as_ptr()) }
}

fn last_error_string() -> Option<String> {
    let ptr = SzConfigTool_getLastError();
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

#[test]
fn test_last_error_is_nul_terminated_exact_message() {
    // The test build's PoisonAllocator (see lib.rs) pads every allocation
    // with non-NUL bytes, so a stored message lacking its terminator is read
    // past its end deterministically instead of by luck of zeroed memory.
    let res = trigger_null_pointer_error();
    assert_eq!(res.returnCode, -1);
    // CStr::from_ptr reads up to the first NUL: without a terminator this would
    // read past the message into unrelated memory.
    assert_eq!(
        last_error_string().as_deref(),
        Some("Null pointer provided")
    );
    assert_eq!(SzConfigTool_getLastErrorCode(), -1);
}

#[test]
fn test_last_error_pointer_stable_until_next_call() {
    trigger_null_pointer_error();
    let first = SzConfigTool_getLastError();
    let again = SzConfigTool_getLastError();
    assert_eq!(first, again, "pointer must stay valid between calls");
    assert_eq!(
        unsafe { CStr::from_ptr(first) }.to_str().unwrap(),
        "Null pointer provided"
    );
}

#[test]
fn test_last_error_is_thread_local() {
    trigger_null_pointer_error();
    assert!(last_error_string().is_some());

    // A fresh thread must not observe this thread's error.
    let other = std::thread::spawn(|| (last_error_string(), SzConfigTool_getLastErrorCode()))
        .join()
        .unwrap();
    assert_eq!(other, (None, 0));

    // And an error set on another thread must not clobber this one.
    std::thread::spawn(|| {
        SzConfigTool_clearLastError();
    })
    .join()
    .unwrap();
    assert_eq!(
        last_error_string().as_deref(),
        Some("Null pointer provided")
    );
}

#[test]
fn test_ffi_guard_converts_panic_to_error_result() {
    let res: SzConfigTool_result = ffi_guard("SzConfigTool_testPanic", || panic!("boom"));
    assert_eq!(res.returnCode, -2);
    assert!(res.response.is_null());
    let msg = last_error_string().expect("panic must set last error");
    assert!(
        msg.contains("internal panic") && msg.contains("SzConfigTool_testPanic"),
        "unexpected message: {msg}"
    );
    assert!(msg.contains("boom"), "payload missing: {msg}");
    assert_eq!(SzConfigTool_getLastErrorCode(), -2);
}

#[test]
fn test_ffi_guard_panic_for_pointer_and_scalar_returns() {
    let p: *const std::os::raw::c_char = ffi_guard("p", || panic!("x"));
    assert!(p.is_null());
    let n: i64 = ffi_guard("n", || panic!("x"));
    assert_eq!(n, -2);
    let i: i32 = ffi_guard("i", || panic!("x"));
    assert_eq!(i, -2);
    let (): () = ffi_guard("unit", || -> () { panic!("x") });
    assert!(last_error_string().unwrap().contains("internal panic"));
}

#[test]
fn test_ffi_guard_passes_through_success() {
    let n: i64 = ffi_guard("ok", || 7);
    assert_eq!(n, 7);
}

/// Every `extern "C"` function body must be wrapped in `ffi_guard`, so no
/// future wrapper can silently reintroduce unwinding across the C boundary.
#[test]
fn test_every_extern_fn_is_panic_guarded() {
    let src = include_str!("lib.rs");
    let mut checked = 0;
    for (idx, _) in src.match_indices("extern \"C\" fn ") {
        let rest = &src[idx..];
        let name: String = rest["extern \"C\" fn ".len()..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let open = rest.find('{').expect("fn body");
        let body = rest[open + 1..].trim_start();
        assert!(
            body.starts_with(&format!("ffi_guard(\"{name}\"")),
            "{name} is not wrapped in ffi_guard(\"{name}\", ..)"
        );
        checked += 1;
    }
    assert!(checked > 100, "parsed only {checked} extern fns");
}

#[test]
fn test_library_version_matches_crate_version() {
    let ptr = SzConfigTool_getLibraryVersion();
    assert!(!ptr.is_null());
    let v = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
    assert_eq!(v, env!("CARGO_PKG_VERSION"));
    // Static storage: the same pointer every call, never freed by the caller.
    assert_eq!(ptr, SzConfigTool_getLibraryVersion());
}

#[test]
fn test_library_version_tracks_root_crate() {
    let root_manifest = include_str!("../../Cargo.toml");
    let ws_version = root_manifest
        .lines()
        .skip_while(|l| l.trim() != "[workspace.package]")
        .find_map(|l| l.strip_prefix("version = "))
        .map(|v| v.trim().trim_matches('"'))
        .expect("[workspace.package] version");
    assert_eq!(env!("CARGO_PKG_VERSION"), ws_version);
}

#[test]
fn test_abi_version() {
    assert_eq!(SzConfigTool_getAbiVersion(), SZCONFIGTOOL_ABI_VERSION);
    // The header's compile-time constant must equal the runtime value, so a
    // C caller can compare SzConfigTool_getAbiVersion() against it.
    let header = include_str!("../include/libSzConfigTool.h");
    let declared: i32 = header
        .lines()
        .find_map(|l| l.trim().strip_prefix("#define SZCONFIGTOOL_ABI_VERSION "))
        .expect("header must #define SZCONFIGTOOL_ABI_VERSION")
        .trim()
        .parse()
        .expect("numeric ABI version");
    assert_eq!(declared, SZCONFIGTOOL_ABI_VERSION);
}
