//! `SzConfigTool_invoke`: the manifest-driven dynamic entry point. Success
//! returns the `sz-configtool-api` JSON envelope; errors set the last-error
//! slots with the stable reason code.

use std::ffi::{CStr, CString};

use serde_json::Value;

use super::{
    SzConfigTool_free, SzConfigTool_getLastError, SzConfigTool_getLastErrorCode,
    SzConfigTool_getLastErrorReasonCode, SzConfigTool_invoke, SzConfigTool_result,
};

const CFG: &str = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;

fn call(name: Option<&str>, config: Option<&str>, args: Option<&str>) -> SzConfigTool_result {
    let name = name.map(|s| CString::new(s).unwrap());
    let config = config.map(|s| CString::new(s).unwrap());
    let args = args.map(|s| CString::new(s).unwrap());
    let ptr = |c: &Option<CString>| c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
    unsafe { SzConfigTool_invoke(ptr(&name), ptr(&config), ptr(&args)) }
}

/// Take ownership of a successful response and parse the envelope.
fn envelope(res: SzConfigTool_result) -> Value {
    assert_eq!(res.returnCode, 0, "unexpected error: {:?}", last_error());
    assert!(!res.response.is_null());
    let text = unsafe { CStr::from_ptr(res.response) }
        .to_str()
        .unwrap()
        .to_string();
    unsafe { SzConfigTool_free(res.response) };
    serde_json::from_str(&text).expect("envelope is JSON")
}

fn cstr(ptr: *const std::os::raw::c_char) -> Option<String> {
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

fn last_error() -> (Option<String>, Option<String>, i64) {
    (
        cstr(SzConfigTool_getLastError()),
        cstr(SzConfigTool_getLastErrorReasonCode()),
        SzConfigTool_getLastErrorCode(),
    )
}

#[test]
fn test_invoke_config_result_is_exact_library_output() {
    let env = envelope(call(
        Some("add_data_source"),
        Some(CFG),
        Some(r#"{"code":"crm"}"#),
    ));
    assert_eq!(env["kind"], "config");
    let direct = sz_configtool_lib::datasources::add_data_source(
        CFG,
        sz_configtool_lib::datasources::AddDataSourceParams {
            code: "crm",
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(env["config"].as_str(), Some(direct.as_str()));
    assert_eq!(
        last_error(),
        (None, None, 0),
        "success clears the last error"
    );
}

#[test]
fn test_invoke_null_args_means_no_args() {
    let env = envelope(call(Some("list_data_sources"), Some(CFG), None));
    assert_eq!(env, serde_json::json!({"kind": "json", "result": []}));
}

#[test]
fn test_invoke_unknown_function_sets_reason_code() {
    let res = call(Some("no_such_function"), Some(CFG), Some("{}"));
    assert_eq!(res.returnCode, -2);
    assert!(res.response.is_null());
    let (msg, reason, code) = last_error();
    assert_eq!(reason.as_deref(), Some("INVALID_INPUT"));
    assert_eq!(code, -2);
    assert!(msg.unwrap().contains("unknown function 'no_such_function'"));
}

#[test]
fn test_invoke_bad_args_json_sets_reason_code() {
    for bad in ["{not json", "[1,2]"] {
        let res = call(Some("list_data_sources"), Some(CFG), Some(bad));
        assert_eq!(res.returnCode, -2, "args {bad:?}");
        assert_eq!(
            last_error().1.as_deref(),
            Some("INVALID_INPUT"),
            "args {bad:?}"
        );
    }
    let res = call(Some("add_data_source"), Some(CFG), Some("{}"));
    assert_eq!(res.returnCode, -2);
    assert_eq!(last_error().1.as_deref(), Some("MISSING_FIELD"));
}

#[test]
fn test_invoke_library_error_keeps_library_reason() {
    let res = call(
        Some("get_data_source"),
        Some(CFG),
        Some(r#"{"code":"NOPE"}"#),
    );
    assert_eq!(res.returnCode, -2);
    assert_eq!(last_error().1.as_deref(), Some("NOT_FOUND"));
    let res = call(Some("list_data_sources"), Some("not json"), Some("{}"));
    assert_eq!(res.returnCode, -2);
    assert_eq!(last_error().1.as_deref(), Some("JSON_PARSE"));
}

#[test]
fn test_invoke_null_name_or_config_is_argument_error() {
    for res in [
        call(None, Some(CFG), Some("{}")),
        call(Some("list_data_sources"), None, Some("{}")),
    ] {
        assert_eq!(res.returnCode, -1);
        assert!(res.response.is_null());
        let (msg, reason, code) = last_error();
        assert_eq!(code, -1);
        assert_eq!(reason, None);
        assert!(msg.unwrap().contains("is null"));
    }
}

#[test]
fn test_invoke_keeps_both_halves_of_config_and_json() {
    let config = r#"{"G2_CONFIG": {"CFG_ATTR": [],
        "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
        "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}]}}"#;
    let args = r#"{"attribute":"X_ATTR","feature":"NAME","element":"FULL_NAME","class":"NAME"}"#;
    let env = envelope(call(Some("add_attribute"), Some(config), Some(args)));
    assert_eq!(env["kind"], "config_and_json");
    assert_eq!(env["result"]["ATTR_CODE"], "X_ATTR");
    assert!(env["config"].as_str().unwrap().contains("X_ATTR"));
}
