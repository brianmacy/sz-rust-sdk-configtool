//! C-boundary coverage for the structured validation-error surface (#59):
//! a bogus-behaviour add must set the `VALIDATION_ERRORS` reason code and carry
//! the versioned, namespaced JSON details, and the validate entry point must
//! return the staged check as versioned JSON. Exercised against the REAL Senzing
//! v4 template, not synthetic config.
//!
//! All assertions live in a SINGLE test: the FFI last-error store is a
//! process-global static, so keeping the calls sequential within one test avoids
//! races with any other FFI call in this binary.

use std::ffi::{CStr, CString};

use sz_configtool_lib::ffi::{
    SzConfigTool_addGenericThreshold, SzConfigTool_free, SzConfigTool_getLastErrorDetails,
    SzConfigTool_getLastErrorReasonCode, SzConfigTool_validateGenericThreshold,
};

fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

#[test]
fn ffi_validation_errors_surface_reason_code_and_details() {
    let config = cstr(&template());
    let plan = cstr("INGEST");
    let bogus = cstr("BOGUS");
    let redo = cstr("No");

    // (1) Bogus behaviour add -> error return, VALIDATION_ERRORS reason code,
    // versioned structured details.
    let res = SzConfigTool_addGenericThreshold(
        config.as_ptr(),
        plan.as_ptr(),
        bogus.as_ptr(),
        20,
        10,
        redo.as_ptr(),
        std::ptr::null(),
    );
    assert!(
        res.returnCode < 0,
        "bogus behaviour must be an error return"
    );
    assert!(res.response.is_null());

    let reason = SzConfigTool_getLastErrorReasonCode();
    assert!(!reason.is_null(), "reason code must be set");
    let reason_str = unsafe { CStr::from_ptr(reason) }.to_str().unwrap();
    assert_eq!(reason_str, "VALIDATION_ERRORS");

    let details = SzConfigTool_getLastErrorDetails();
    assert!(!details.is_null(), "structured details must be set");
    let details_str = unsafe { CStr::from_ptr(details) }.to_str().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(details_str).unwrap();
    assert_eq!(parsed["schema"], "sz-configtool.validation-errors/v1");
    let failures = parsed["failures"].as_array().unwrap();
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0]["field"], "behavior");
    assert_eq!(failures[0]["reasonCode"], "UNKNOWN_REFERENCE_CODE");
    assert_eq!(failures[0]["offendingValue"], "BOGUS");

    // (2) validate entry point returns the staged check as versioned JSON.
    let redo_ok = cstr("Yes");
    let name = cstr("NAME");
    let res2 = SzConfigTool_validateGenericThreshold(
        config.as_ptr(),
        plan.as_ptr(),
        name.as_ptr(),
        redo_ok.as_ptr(),
        std::ptr::null(),
    );
    assert_eq!(res2.returnCode, 0, "validate must succeed as data");
    assert!(!res2.response.is_null());
    let check_str = unsafe { CStr::from_ptr(res2.response) }.to_str().unwrap();
    let check: serde_json::Value = serde_json::from_str(check_str).unwrap();
    assert_eq!(check["schema"], "sz-configtool.generic-threshold-check/v1");
    // INGEST/NAME/all exists in the template -> duplicate (warning-success).
    assert_eq!(check["result"], "duplicate");
    unsafe { SzConfigTool_free(res2.response) };
}
