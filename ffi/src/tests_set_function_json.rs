//! JSON-based `set*FunctionWithJson` wrappers: CONNECT_STR is tri-state
//! (absent = leave, null = clear, string = set), which the direct-arg
//! `set*Function` forms cannot express.

use std::ffi::{CStr, CString};

use serde_json::{Value, json};

use super::{
    SzConfigTool_free, SzConfigTool_getLastError, SzConfigTool_getLastErrorReasonCode,
    SzConfigTool_result, SzConfigTool_setComparisonFunctionWithJson,
    SzConfigTool_setDistinctFunctionWithJson, SzConfigTool_setExpressionFunctionWithJson,
    SzConfigTool_setStandardizeFunctionWithJson,
};

type SetFn = extern "C" fn(
    *const std::os::raw::c_char,
    *const std::os::raw::c_char,
    *const std::os::raw::c_char,
) -> SzConfigTool_result;

/// (section, code column, desc column, wrapper) for each implemented wrapper.
fn cases() -> [(&'static str, &'static str, &'static str, SetFn); 4] {
    [
        (
            "CFG_SFUNC",
            "SFUNC_CODE",
            "SFUNC_DESC",
            SzConfigTool_setStandardizeFunctionWithJson,
        ),
        (
            "CFG_EFUNC",
            "EFUNC_CODE",
            "EFUNC_DESC",
            SzConfigTool_setExpressionFunctionWithJson,
        ),
        (
            "CFG_CFUNC",
            "CFUNC_CODE",
            "CFUNC_DESC",
            SzConfigTool_setComparisonFunctionWithJson,
        ),
        (
            "CFG_DFUNC",
            "DFUNC_CODE",
            "DFUNC_DESC",
            SzConfigTool_setDistinctFunctionWithJson,
        ),
    ]
}

fn config_with_row(section: &str, code_col: &str, desc_col: &str) -> String {
    let id_col = code_col.replace("_CODE", "_ID");
    json!({"G2_CONFIG": {section: [{
        id_col: 1,
        code_col: "MY_FUNC",
        desc_col: "Original",
        "CONNECT_STR": "orig_connect",
        "LANGUAGE": "C"
    }]}})
    .to_string()
}

fn call(f: SetFn, config: &str, updates: &str) -> Result<Value, String> {
    let config = CString::new(config).unwrap();
    let code = CString::new("my_func").unwrap();
    let updates = CString::new(updates).unwrap();
    let res = f(config.as_ptr(), code.as_ptr(), updates.as_ptr());
    if res.returnCode != 0 {
        let msg = unsafe { CStr::from_ptr(SzConfigTool_getLastError()) };
        return Err(msg.to_str().unwrap().to_string());
    }
    let out = unsafe { CStr::from_ptr(res.response) }
        .to_str()
        .unwrap()
        .to_string();
    unsafe { SzConfigTool_free(res.response) };
    Ok(serde_json::from_str(&out).unwrap())
}

#[test]
fn test_with_json_null_clears_connect_str() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        let v = call(f, &cfg, r#"{"CONNECT_STR": null}"#).unwrap();
        let row = &v["G2_CONFIG"][section][0];
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null, "{section}");
        assert_eq!(
            row[desc_col],
            json!("Original"),
            "{section}: desc untouched"
        );
    }
}

#[test]
fn test_with_json_sets_fields_and_absent_leaves() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        let updates = json!({desc_col: "New desc", "LANGUAGE": "JAVA"}).to_string();
        let v = call(f, &cfg, &updates).unwrap();
        let row = &v["G2_CONFIG"][section][0];
        assert_eq!(row[desc_col], json!("New desc"), "{section}");
        assert_eq!(row["LANGUAGE"], json!("JAVA"), "{section}");
        assert_eq!(row["CONNECT_STR"], json!("orig_connect"), "{section}");

        let v = call(f, &cfg, r#"{"connectStr": "new_connect"}"#).unwrap();
        assert_eq!(
            v["G2_CONFIG"][section][0]["CONNECT_STR"],
            json!("new_connect"),
            "{section}: camelCase alias"
        );
    }
}

#[test]
fn test_with_json_rejects_non_object_and_unknown_function() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        let err = call(f, &cfg, "[1,2]").unwrap_err();
        assert!(err.contains("JSON object"), "{section}: {err}");
        let err = call(f, &cfg, "{not json").unwrap_err();
        assert!(err.contains("updates_json"), "{section}: {err}");

        let other = cfg.replace("MY_FUNC", "OTHER");
        let err = call(f, &other, r#"{"CONNECT_STR": null}"#).unwrap_err();
        assert!(err.to_lowercase().contains("not found"), "{section}: {err}");
    }
}

#[test]
fn test_with_json_null_pointer_is_error() {
    for (_, _, _, f) in cases() {
        let res = f(std::ptr::null(), std::ptr::null(), std::ptr::null());
        assert_eq!(res.returnCode, -1);
        assert!(res.response.is_null());
    }
}

/// Call expecting failure; returns (returnCode, reason code, message).
fn call_err(f: SetFn, config: &str, updates: &str) -> (i64, String, String) {
    let config = CString::new(config).unwrap();
    let code = CString::new("my_func").unwrap();
    let updates = CString::new(updates).unwrap();
    let res = f(config.as_ptr(), code.as_ptr(), updates.as_ptr());
    assert!(res.response.is_null(), "expected failure for {updates:?}");
    let text = |p: *const std::os::raw::c_char| {
        if p.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(p) }.to_str().unwrap().to_string()
        }
    };
    (
        res.returnCode,
        text(SzConfigTool_getLastErrorReasonCode()),
        text(SzConfigTool_getLastError()),
    )
}

#[test]
fn test_with_json_non_string_values_are_invalid_input() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        for (key, value) in [
            (desc_col, json!(5)),
            ("description", json!(true)),
            ("LANGUAGE", json!(["C"])),
            ("language", json!({"x": 1})),
            ("CONNECT_STR", json!(7)),
            ("connectStr", json!(1.5)),
        ] {
            let updates = json!({key: value}).to_string();
            let (rc, reason, msg) = call_err(f, &cfg, &updates);
            assert_eq!(
                (rc, reason.as_str()),
                (-2, "INVALID_INPUT"),
                "{section} {updates}: {msg}"
            );
            assert!(msg.contains(key), "{section} {updates}: {msg}");
        }
    }
}

#[test]
fn test_with_json_key_and_alias_together_are_invalid_input() {
    // `{"<DESC>": null, "description": "x"}` used to stop at the null key and
    // silently ignore "x".
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        for updates in [
            json!({desc_col: null, "description": "x"}),
            json!({desc_col: "a", "description": "b"}),
            json!({"LANGUAGE": "C", "language": null}),
            json!({"CONNECT_STR": null, "connectStr": "x"}),
        ] {
            let (rc, reason, msg) = call_err(f, &cfg, &updates.to_string());
            assert_eq!(
                (rc, reason.as_str()),
                (-2, "INVALID_INPUT"),
                "{section} {updates}: {msg}"
            );
        }
    }
}

#[test]
fn test_with_json_anon_support_only_where_supported() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        let supported = matches!(section, "CFG_CFUNC" | "CFG_DFUNC");
        for key in ["ANON_SUPPORT", "anonSupport"] {
            let updates = json!({key: "Yes"}).to_string();
            if supported {
                let v = call(f, &cfg, &updates).unwrap();
                assert_eq!(
                    v["G2_CONFIG"][section][0]["ANON_SUPPORT"],
                    json!("Yes"),
                    "{section}"
                );
            } else {
                let (rc, reason, msg) = call_err(f, &cfg, &updates);
                assert_eq!(
                    (rc, reason.as_str()),
                    (-2, "INVALID_INPUT"),
                    "{section}: {msg}"
                );
                assert!(msg.contains(key), "{section}: {msg}");
            }
        }
    }
}

#[test]
fn test_with_json_unknown_keys_are_invalid_input() {
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        // A typo of the desc column, and another section's desc column.
        let other_desc = if desc_col == "SFUNC_DESC" {
            "CFUNC_DESC"
        } else {
            "SFUNC_DESC"
        };
        for key in [format!("{desc_col}X"), other_desc.to_string()] {
            let updates = json!({key.as_str(): "x"}).to_string();
            let (rc, reason, msg) = call_err(f, &cfg, &updates);
            assert_eq!(
                (rc, reason.as_str()),
                (-2, "INVALID_INPUT"),
                "{section} {updates}: {msg}"
            );
            assert!(msg.contains(&key), "{section}: {msg}");
        }
    }
}

#[test]
fn test_with_json_null_non_connect_fields_still_leave() {
    // Valid-input behaviour is unchanged: null for a non-tri-state field leaves it.
    for (section, code_col, desc_col, f) in cases() {
        let cfg = config_with_row(section, code_col, desc_col);
        let updates = json!({desc_col: null, "language": null}).to_string();
        let v = call(f, &cfg, &updates).unwrap();
        let row = &v["G2_CONFIG"][section][0];
        assert_eq!(row[desc_col], json!("Original"), "{section}");
        assert_eq!(row["LANGUAGE"], json!("C"), "{section}");
        let v = call(f, &cfg, r#"{"description": "D", "language": "L"}"#).unwrap();
        assert_eq!(
            v["G2_CONFIG"][section][0][desc_col],
            json!("D"),
            "{section}: alias"
        );
    }
}
