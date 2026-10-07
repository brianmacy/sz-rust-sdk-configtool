//! FFI calls from a thread-local destructor (thread teardown) must not abort.
//!
//! The per-thread last-error slot may already be destroyed when another TLS
//! destructor calls into the library (e.g. a C++ `thread_local` object or a
//! Rust TLS value that cleans up through this API). Accessing it with
//! `LocalKey::with` would panic; `ffi_guard` would then panic AGAIN while
//! recording that panic, outside `catch_unwind` — inside a TLS destructor
//! that is an abort. The scenario runs in a child process so an abort is
//! reported as a test failure instead of killing the test harness.

use std::ffi::CString;
use std::process::Command;

use super::{
    SzConfigTool_addDataSource, SzConfigTool_clearLastError, SzConfigTool_getAbiVersion,
    SzConfigTool_getLastError, SzConfigTool_getLastErrorCode, SzConfigTool_getLastErrorDetails,
    SzConfigTool_getLastErrorReasonCode, SzConfigTool_getLibraryVersion, SzConfigTool_invoke,
};

/// Set in the child process to run the scenario instead of re-spawning.
const CHILD_ENV: &str = "SZCONFIGTOOL_TLS_TEARDOWN_CHILD";
const TEST_NAME: &str = "tests_tls_teardown::test_ffi_calls_from_tls_destructor_do_not_abort";

/// Calls the FFI (error and success paths, every last-error getter) on drop.
struct CallsFfiOnDrop;

impl Drop for CallsFfiOnDrop {
    fn drop(&mut self) {
        let code = CString::new("TEST").unwrap();
        // NULL config: an error path that records a last error.
        let r = unsafe { SzConfigTool_addDataSource(std::ptr::null(), code.as_ptr()) };
        assert_eq!(r.returnCode, -1);
        let _ = SzConfigTool_getLastError();
        let _ = SzConfigTool_getLastErrorCode();
        let _ = SzConfigTool_getLastErrorReasonCode();
        let _ = SzConfigTool_getLastErrorDetails();
        SzConfigTool_clearLastError();
        assert!(!SzConfigTool_getLibraryVersion().is_null());
        assert_eq!(SzConfigTool_getAbiVersion(), sz_configtool_api::ABI_VERSION);
        // A library error through invoke (records reason code + message).
        let name = CString::new("get_data_source").unwrap();
        let config = CString::new(r#"{"G2_CONFIG":{"CFG_DSRC":[]}}"#).unwrap();
        let args = CString::new(r#"{"code":"NOPE"}"#).unwrap();
        let r = unsafe { SzConfigTool_invoke(name.as_ptr(), config.as_ptr(), args.as_ptr()) };
        assert_eq!(r.returnCode, -2);
        eprintln!("TLS-DESTRUCTOR-FFI-OK");
    }
}

thread_local! {
    static GUARD: CallsFfiOnDrop = const { CallsFfiOnDrop };
}

/// Child body: register GUARD's destructor BEFORE the last-error slot's, so
/// the slot is torn down first (destructors run in reverse registration
/// order) and GUARD's drop then calls the FFI with the slot gone.
fn run_scenario() {
    std::thread::spawn(|| {
        GUARD.with(|_| {});
        // First touch of the last-error slot registers its destructor.
        let code = CString::new("TEST").unwrap();
        let _ = unsafe { SzConfigTool_addDataSource(std::ptr::null(), code.as_ptr()) };
    })
    .join()
    .expect("thread exits cleanly");
}

#[test]
fn test_ffi_calls_from_tls_destructor_do_not_abort() {
    if std::env::var_os(CHILD_ENV).is_some() {
        run_scenario();
        return;
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .args([TEST_NAME, "--exact", "--nocapture", "--test-threads=1"])
        .env(CHILD_ENV, "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "child failed ({:?}):\n{stderr}",
        out.status
    );
    assert!(
        stderr.contains("TLS-DESTRUCTOR-FFI-OK"),
        "destructor did not complete:\n{stderr}"
    );
}
