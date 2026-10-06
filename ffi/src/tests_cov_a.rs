//! Coverage of the C ABI's infrastructure (last-error storage, panic guard,
//! `SzConfigTool_invoke`) and the hand-written exports defined before the
//! config-section helpers end (data sources, attributes, features, elements,
//! fragments, generic plans, thresholds, system parameters, versioning and
//! config sections). Every pointer argument is exercised NULL and as invalid
//! UTF-8, and every reachable library error path asserts its exact return code.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde_json::Value;

use super::*;

/// The Senzing g2config template (`api/manifest/project.yaml` key `fixture`).
const FIXTURE: &str = include_str!("../../tests/fixtures/g2config_template.json");

/// A config whose version strings decode to text with an interior NUL, so the
/// raw-string responses cannot be converted to a C string (-4 paths).
const NUL_VERSION_CFG: &str = r#"{"G2_CONFIG":{"CONFIG_BASE_VERSION":{"VERSION":"4\u0000x",
    "COMPATIBILITY_VERSION":{"CONFIG_VERSION":"1\u0000x"}}}}"#;

const NOT_JSON: &str = "not json";

fn cs(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn bad_utf8() -> CString {
    CString::new(vec![0xff, 0xfe]).unwrap()
}

fn opt_str(ptr: *const c_char) -> Option<String> {
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

fn last_message() -> Option<String> {
    opt_str(SzConfigTool_getLastError())
}

fn last_reason() -> Option<String> {
    opt_str(SzConfigTool_getLastErrorReasonCode())
}

/// Assert success, take ownership of the response and free it.
fn take_ok(res: SzConfigTool_result) -> String {
    assert_eq!(res.returnCode, 0, "unexpected error: {:?}", last_message());
    assert!(!res.response.is_null());
    let text = unsafe { CStr::from_ptr(res.response) }
        .to_str()
        .unwrap()
        .to_string();
    unsafe { SzConfigTool_free(res.response) };
    assert_eq!(SzConfigTool_getLastErrorCode(), 0);
    assert_eq!(last_message(), None);
    text
}

fn take_json(res: SzConfigTool_result) -> Value {
    serde_json::from_str(&take_ok(res)).expect("response is JSON")
}

/// Assert an error result with `code` and return the recorded message.
fn take_err(res: SzConfigTool_result, code: i64) -> String {
    assert_eq!(res.returnCode, code, "last error: {:?}", last_message());
    assert!(res.response.is_null());
    assert_eq!(SzConfigTool_getLastErrorCode(), code);
    last_message().expect("error message recorded")
}

/// Assert a library error routed through `handle_result!` (-2 + reason code).
fn take_lib_err(res: SzConfigTool_result, reason: &str) -> String {
    let msg = take_err(res, -2);
    assert_eq!(last_reason().as_deref(), Some(reason), "message: {msg}");
    msg
}

/// Assert a library error routed through `set_error` (-5, no reason code).
fn take_plain_err(res: SzConfigTool_result) -> String {
    let msg = take_err(res, -5);
    assert_eq!(last_reason(), None);
    msg
}

// ---------------------------------------------------------------------------
// Table-driven boundary checks (NULL / invalid UTF-8 per pointer argument)
// ---------------------------------------------------------------------------

type Call = fn(&[*const c_char]) -> SzConfigTool_result;

struct Boundary {
    name: &'static str,
    /// A valid value for every pointer argument, in order.
    args: &'static [&'static str],
    /// Indexes of arguments that accept NULL.
    optional: &'static [usize],
    null_msg: &'static str,
    utf8_code: i64,
    call: Call,
}

fn run_with(spec: &Boundary, index: usize, replacement: Option<&CString>) -> SzConfigTool_result {
    let owned: Vec<CString> = spec.args.iter().map(|s| cs(s)).collect();
    let ptrs: Vec<*const c_char> = owned
        .iter()
        .enumerate()
        .map(|(i, c)| match (i == index, replacement) {
            (true, Some(r)) => r.as_ptr(),
            (true, None) => std::ptr::null(),
            (false, _) => c.as_ptr(),
        })
        .collect();
    (spec.call)(&ptrs)
}

fn check_boundary(spec: &Boundary) {
    let bad = bad_utf8();
    for index in 0..spec.args.len() {
        if !spec.optional.contains(&index) {
            let msg = take_err(run_with(spec, index, None), -1);
            assert_eq!(msg, spec.null_msg, "{} NULL arg {index}", spec.name);
            assert_eq!(last_reason(), None);
        }
        let msg = take_err(run_with(spec, index, Some(&bad)), spec.utf8_code);
        assert!(
            msg.starts_with("Invalid UTF-8 in "),
            "{} bad UTF-8 arg {index}: {msg}",
            spec.name
        );
    }
}

const NULL_PTR: &str = "Null pointer provided";
const NULL_REQ: &str = "Required parameter is null";

const BOUNDARIES: &[Boundary] = &[
    Boundary {
        name: "addDataSource",
        args: &[FIXTURE, "NEW_DS"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_addDataSource(a[0], a[1]) },
    },
    Boundary {
        name: "deleteDataSource",
        args: &[FIXTURE, "TEST"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_deleteDataSource(a[0], a[1]) },
    },
    Boundary {
        name: "listDataSources",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_listDataSources(a[0]) },
    },
    Boundary {
        name: "addAttribute",
        args: &[
            FIXTURE,
            "X_ATTR",
            "NAME",
            "FULL_NAME",
            "NAME",
            "d",
            "No",
            "No",
        ],
        optional: &[5, 6, 7],
        null_msg: NULL_REQ,
        utf8_code: -1,
        call: |a| unsafe {
            SzConfigTool_addAttribute(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7])
        },
    },
    Boundary {
        name: "deleteAttribute",
        args: &[FIXTURE, "NAME_FULL"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_deleteAttribute(a[0], a[1]) },
    },
    Boundary {
        name: "getAttribute",
        args: &[FIXTURE, "NAME_FULL"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_getAttribute(a[0], a[1]) },
    },
    Boundary {
        name: "listAttributes",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_listAttributes(a[0]) },
    },
    Boundary {
        name: "setAttribute",
        args: &[FIXTURE, "NAME_FULL", "{}"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_setAttribute(a[0], a[1], a[2]) },
    },
    Boundary {
        name: "getFeature",
        args: &[FIXTURE, "NAME"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_getFeature(a[0], a[1]) },
    },
    Boundary {
        name: "listFeatures",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_listFeatures(a[0]) },
    },
    Boundary {
        name: "getElement",
        args: &[FIXTURE, "FULL_NAME"],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_getElement(a[0], a[1]) },
    },
    Boundary {
        name: "listElements",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_PTR,
        utf8_code: -1,
        call: |a| unsafe { SzConfigTool_listElements(a[0]) },
    },
    Boundary {
        name: "setFragmentWithJson",
        args: &[FIXTURE, "SAME_NAME", "{}"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_setFragmentWithJson(a[0], a[1], a[2]) },
    },
    Boundary {
        name: "cloneGenericPlan",
        args: &[FIXTURE, "INGEST", "NEW_PLAN", "desc"],
        optional: &[3],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_cloneGenericPlan(a[0], a[1], a[2], a[3]) },
    },
    Boundary {
        name: "setGenericPlan",
        args: &[FIXTURE, "INGEST", "desc"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_setGenericPlan(a[0], a[1], a[2]) },
    },
    Boundary {
        name: "listGenericPlans",
        args: &[FIXTURE, "INGEST"],
        optional: &[1],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_listGenericPlans(a[0], a[1]) },
    },
    Boundary {
        name: "getThreshold",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_getThreshold(a[0], 1) },
    },
    Boundary {
        name: "listSystemParameters",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_listSystemParameters(a[0]) },
    },
    Boundary {
        name: "setSystemParameterWithJson",
        args: &[FIXTURE, "relationshipsBreakMatches", "\"Yes\""],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_setSystemParameterWithJson(a[0], a[1], a[2]) },
    },
    Boundary {
        name: "getVersion",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_getVersion(a[0]) },
    },
    Boundary {
        name: "getCompatibilityVersion",
        args: &[FIXTURE],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_getCompatibilityVersion(a[0]) },
    },
    Boundary {
        name: "updateCompatibilityVersion",
        args: &[FIXTURE, "12"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_updateCompatibilityVersion(a[0], a[1]) },
    },
    Boundary {
        name: "updateFeatureVersion",
        args: &[FIXTURE, "12"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_updateFeatureVersion(a[0], a[1]) },
    },
    Boundary {
        name: "verifyCompatibilityVersion",
        args: &[FIXTURE, "11"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_verifyCompatibilityVersion(a[0], a[1]) },
    },
    Boundary {
        name: "addConfigSection",
        args: &[FIXTURE, "CFG_NEW"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_addConfigSection(a[0], a[1]) },
    },
    Boundary {
        name: "removeConfigSection",
        args: &[FIXTURE, "CFG_DSRC_INTEREST"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_removeConfigSection(a[0], a[1]) },
    },
    Boundary {
        name: "getConfigSection",
        args: &[FIXTURE, "CFG_DSRC", "TEST"],
        optional: &[2],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_getConfigSection(a[0], a[1], a[2]) },
    },
    Boundary {
        name: "configSectionIsEmpty",
        args: &[FIXTURE, "CFG_DSRC"],
        optional: &[],
        null_msg: NULL_REQ,
        utf8_code: -2,
        call: |a| unsafe { SzConfigTool_configSectionIsEmpty(a[0], a[1]) },
    },
];

#[test]
fn test_null_and_invalid_utf8_arguments_are_rejected() {
    for spec in BOUNDARIES {
        check_boundary(spec);
    }
}

/// Library parse failures for every single-config export that routes through
/// `handle_result!` (-2, JSON_PARSE) or `set_error` (-5).
#[test]
fn test_unparsable_config_is_a_library_error() {
    for spec in BOUNDARIES {
        let mut args: Vec<&str> = spec.args.to_vec();
        args[0] = NOT_JSON;
        let owned: Vec<CString> = args.iter().map(|s| cs(s)).collect();
        let ptrs: Vec<*const c_char> = owned.iter().map(|c| c.as_ptr()).collect();
        let res = (spec.call)(&ptrs);
        match spec.utf8_code {
            -1 => {
                take_lib_err(res, "JSON_PARSE");
            }
            _ => {
                assert!(res.response.is_null(), "{}", spec.name);
                assert!(
                    [-2, -5].contains(&res.returnCode),
                    "{}: {}",
                    spec.name,
                    res.returnCode
                );
                assert_eq!(SzConfigTool_getLastErrorCode(), res.returnCode);
                assert!(last_message().is_some());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Infrastructure
// ---------------------------------------------------------------------------

#[test]
fn test_free_null_is_a_no_op() {
    unsafe { SzConfigTool_free(std::ptr::null_mut()) };
}

#[test]
fn test_clear_last_error_resets_every_slot() {
    let r = unsafe { SzConfigTool_getAttribute(cs(FIXTURE).as_ptr(), cs("NOPE").as_ptr()) };
    take_lib_err(r, "NOT_FOUND");
    SzConfigTool_clearLastError();
    assert_eq!(last_message(), None);
    assert_eq!(last_reason(), None);
    assert!(SzConfigTool_getLastErrorDetails().is_null());
    assert_eq!(SzConfigTool_getLastErrorCode(), 0);
}

#[test]
fn test_to_cstring_lossy_escapes_interior_nul() {
    assert_eq!(
        to_cstring_lossy("a\0b".to_string()).to_str().unwrap(),
        "a\\0b"
    );
    assert_eq!(
        to_cstring_lossy("plain".to_string()).to_str().unwrap(),
        "plain"
    );
}

#[test]
fn test_library_error_message_with_nul_is_escaped() {
    // A JSON-escaped NUL in a code reaches the library's NotFound message.
    let name = cs("get_data_source");
    let config = cs(FIXTURE);
    let args = cs(r#"{"code":"NO\u0000PE"}"#);
    let r = unsafe { SzConfigTool_invoke(name.as_ptr(), config.as_ptr(), args.as_ptr()) };
    let msg = take_lib_err(r, "NOT_FOUND");
    assert!(msg.contains("NO\\0PE"), "{msg}");
}

#[test]
fn test_ffi_guard_reports_string_and_opaque_panic_payloads() {
    let owned = String::from("formatted");
    let r: SzConfigTool_result = ffi_guard("fmt", || panic!("{owned}"));
    assert_eq!(take_err(r, -2), "internal panic in fmt: formatted");
    let r: SzConfigTool_result = ffi_guard("opaque", || std::panic::panic_any(7_u8));
    assert_eq!(
        take_err(r, -2),
        "internal panic in opaque: non-string panic payload"
    );
}

/// A library panic on a non-object config (JSON `IndexMut` on an array) is
/// converted to -2 by the guard instead of unwinding into C.
#[test]
fn test_library_panic_is_caught_by_export_guard() {
    let r = unsafe { SzConfigTool_updateFeatureVersion(cs("[]").as_ptr(), cs("1").as_ptr()) };
    let msg = take_err(r, -2);
    assert!(
        msg.starts_with("internal panic in SzConfigTool_updateFeatureVersion: "),
        "{msg}"
    );
    assert_eq!(last_reason(), None);
}

// ---------------------------------------------------------------------------
// SzConfigTool_invoke
// ---------------------------------------------------------------------------

fn invoke(name: &CString, config: &CString, args: Option<&CString>) -> SzConfigTool_result {
    let args = args.map_or(std::ptr::null(), |a| a.as_ptr());
    unsafe { SzConfigTool_invoke(name.as_ptr(), config.as_ptr(), args) }
}

#[test]
fn test_invoke_null_args_means_no_arguments() {
    let env = take_json(invoke(&cs("list_data_sources"), &cs(FIXTURE), None));
    assert_eq!(env["kind"], "json");
    assert_eq!(env["result"].as_array().unwrap().len(), 2);
}

#[test]
fn test_invoke_invalid_utf8_arguments() {
    let (name, config, args, bad) = (cs("list_data_sources"), cs(FIXTURE), cs("{}"), bad_utf8());
    for (n, c, a, what) in [
        (&bad, &config, &args, "name"),
        (&name, &bad, &args, "config_json"),
        (&name, &config, &bad, "args_json"),
    ] {
        let msg = take_err(invoke(n, c, Some(a)), -1);
        assert!(
            msg.starts_with(&format!("Invalid UTF-8 in {what}: ")),
            "{msg}"
        );
        assert_eq!(last_reason(), None);
    }
}

#[test]
fn test_invoke_handler_panic_is_internal() {
    let r = invoke(
        &cs("update_feature_version"),
        &cs("[]"),
        Some(&cs(r#"{"version":"1"}"#)),
    );
    let msg = take_lib_err(r, "INTERNAL");
    assert!(msg.contains("panic in update_feature_version"), "{msg}");
    assert!(SzConfigTool_getLastErrorDetails().is_null());
}

// ---------------------------------------------------------------------------
// Data sources
// ---------------------------------------------------------------------------

fn codes(list: &Value, key: &str) -> Vec<String> {
    list.as_array()
        .unwrap()
        .iter()
        .map(|v| v[key].as_str().unwrap().to_string())
        .collect()
}

fn list_data_sources(config: &str) -> Vec<String> {
    let list = take_json(unsafe { SzConfigTool_listDataSources(cs(config).as_ptr()) });
    codes(&list, "dataSource")
}

#[test]
fn test_data_source_round_trip() {
    assert_eq!(list_data_sources(FIXTURE), ["TEST", "SEARCH"]);
    let added =
        take_ok(unsafe { SzConfigTool_addDataSource(cs(FIXTURE).as_ptr(), cs("CRM").as_ptr()) });
    assert_eq!(list_data_sources(&added), ["TEST", "SEARCH", "CRM"]);
    let deleted =
        take_ok(unsafe { SzConfigTool_deleteDataSource(cs(&added).as_ptr(), cs("CRM").as_ptr()) });
    assert_eq!(list_data_sources(&deleted), ["TEST", "SEARCH"]);
}

#[test]
fn test_data_source_library_errors() {
    let cfg = cs(FIXTURE);
    take_lib_err(
        unsafe { SzConfigTool_addDataSource(cfg.as_ptr(), cs("TEST").as_ptr()) },
        "ALREADY_EXISTS",
    );
    take_lib_err(
        unsafe { SzConfigTool_deleteDataSource(cfg.as_ptr(), cs("NOPE").as_ptr()) },
        "NOT_FOUND",
    );
}

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

fn get_attribute(config: &str, code: &str) -> Value {
    take_json(unsafe { SzConfigTool_getAttribute(cs(config).as_ptr(), cs(code).as_ptr()) })
}

#[test]
fn test_add_attribute_with_and_without_optionals() {
    let (cfg, attr, feat, elem, class) = (
        cs(FIXTURE),
        cs("X_ATTR"),
        cs("NAME"),
        cs("FULL_NAME"),
        cs("NAME"),
    );
    let (dv, int, req) = (cs("dflt"), cs("Yes"), cs("Yes"));
    let with = take_ok(unsafe {
        SzConfigTool_addAttribute(
            cfg.as_ptr(),
            attr.as_ptr(),
            feat.as_ptr(),
            elem.as_ptr(),
            class.as_ptr(),
            dv.as_ptr(),
            int.as_ptr(),
            req.as_ptr(),
        )
    });
    let a = get_attribute(&with, "X_ATTR");
    assert_eq!(a["DEFAULT_VALUE"], "dflt");
    assert_eq!(a["INTERNAL"], "Yes");
    assert_eq!(a["FELEM_REQ"], "Yes");

    let null = std::ptr::null();
    let without = take_ok(unsafe {
        SzConfigTool_addAttribute(
            cfg.as_ptr(),
            attr.as_ptr(),
            feat.as_ptr(),
            elem.as_ptr(),
            class.as_ptr(),
            null,
            null,
            null,
        )
    });
    assert_eq!(
        get_attribute(&without, "X_ATTR")["DEFAULT_VALUE"],
        Value::Null
    );

    take_lib_err(
        unsafe {
            SzConfigTool_addAttribute(
                cfg.as_ptr(),
                cs("NAME_FULL").as_ptr(),
                feat.as_ptr(),
                elem.as_ptr(),
                class.as_ptr(),
                null,
                null,
                null,
            )
        },
        "ALREADY_EXISTS",
    );
}

#[test]
fn test_attribute_get_list_delete() {
    assert_eq!(
        get_attribute(FIXTURE, "NAME_FULL")["ATTR_CODE"],
        "NAME_FULL"
    );
    let list = take_json(unsafe { SzConfigTool_listAttributes(cs(FIXTURE).as_ptr()) });
    assert!(codes(&list, "attribute").contains(&"NAME_FULL".to_string()));

    let deleted = take_ok(unsafe {
        SzConfigTool_deleteAttribute(cs(FIXTURE).as_ptr(), cs("NAME_FULL").as_ptr())
    });
    let r = unsafe { SzConfigTool_getAttribute(cs(&deleted).as_ptr(), cs("NAME_FULL").as_ptr()) };
    take_lib_err(r, "NOT_FOUND");
    let r =
        unsafe { SzConfigTool_deleteAttribute(cs(&deleted).as_ptr(), cs("NAME_FULL").as_ptr()) };
    take_lib_err(r, "NOT_FOUND");
}

#[test]
fn test_set_attribute_updates_and_errors() {
    let (cfg, code) = (cs(FIXTURE), cs("NAME_FULL"));
    let updates = cs(r#"{"internal":"Yes","required":"Yes","default":"zz"}"#);
    let out = take_ok(unsafe {
        SzConfigTool_setAttribute(cfg.as_ptr(), code.as_ptr(), updates.as_ptr())
    });
    let a = get_attribute(&out, "NAME_FULL");
    assert_eq!(
        (&a["INTERNAL"], &a["FELEM_REQ"], &a["DEFAULT_VALUE"]),
        (&"Yes".into(), &"Yes".into(), &"zz".into())
    );

    let msg = take_err(
        unsafe { SzConfigTool_setAttribute(cfg.as_ptr(), code.as_ptr(), cs("{oops").as_ptr()) },
        -1,
    );
    assert!(msg.starts_with("Invalid JSON in updatesJson: "), "{msg}");

    let r =
        unsafe { SzConfigTool_setAttribute(cfg.as_ptr(), cs("NOPE").as_ptr(), cs("{}").as_ptr()) };
    take_lib_err(r, "NOT_FOUND");
}

// ---------------------------------------------------------------------------
// Features and elements
// ---------------------------------------------------------------------------

#[test]
fn test_feature_and_element_reads() {
    let cfg = cs(FIXTURE);
    let f = take_json(unsafe { SzConfigTool_getFeature(cfg.as_ptr(), cs("NAME").as_ptr()) });
    assert_eq!(f["feature"], "NAME");
    let list = take_json(unsafe { SzConfigTool_listFeatures(cfg.as_ptr()) });
    assert!(codes(&list, "feature").contains(&"NAME".to_string()));
    take_lib_err(
        unsafe { SzConfigTool_getFeature(cfg.as_ptr(), cs("NOPE").as_ptr()) },
        "NOT_FOUND",
    );

    let e = take_json(unsafe { SzConfigTool_getElement(cfg.as_ptr(), cs("FULL_NAME").as_ptr()) });
    assert_eq!(e["element"], "FULL_NAME");
    let list = take_json(unsafe { SzConfigTool_listElements(cfg.as_ptr()) });
    assert!(codes(&list, "element").contains(&"FULL_NAME".to_string()));
    take_lib_err(
        unsafe { SzConfigTool_getElement(cfg.as_ptr(), cs("NOPE").as_ptr()) },
        "NOT_FOUND",
    );
}

// ---------------------------------------------------------------------------
// Fragments and generic plans
// ---------------------------------------------------------------------------

fn erfrag_desc(config: &str, code: &str) -> Value {
    let v: Value = serde_json::from_str(config).unwrap();
    v["G2_CONFIG"]["CFG_ERFRAG"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["ERFRAG_CODE"] == code)
        .unwrap()["ERFRAG_DESC"]
        .clone()
}

#[test]
fn test_set_fragment_with_json() {
    let (cfg, code) = (cs(FIXTURE), cs("SAME_NAME"));
    let out = take_ok(unsafe {
        SzConfigTool_setFragmentWithJson(
            cfg.as_ptr(),
            code.as_ptr(),
            cs(r#"{"description":"d2"}"#).as_ptr(),
        )
    });
    assert_eq!(erfrag_desc(&out, "SAME_NAME"), "d2");

    let msg = take_err(
        unsafe { SzConfigTool_setFragmentWithJson(cfg.as_ptr(), code.as_ptr(), cs("{x").as_ptr()) },
        -3,
    );
    assert!(
        msg.starts_with("Failed to parse fragment_config_json: "),
        "{msg}"
    );

    let r = unsafe {
        SzConfigTool_setFragmentWithJson(cfg.as_ptr(), cs("NOPE").as_ptr(), cs("{}").as_ptr())
    };
    take_lib_err(r, "NOT_FOUND");
}

fn plans(config: &str, filter: Option<&str>) -> Vec<String> {
    let filter = filter.map(cs);
    let fptr = filter.as_ref().map_or(std::ptr::null(), |f| f.as_ptr());
    let list = take_json(unsafe { SzConfigTool_listGenericPlans(cs(config).as_ptr(), fptr) });
    codes(&list, "plan")
}

#[test]
fn test_generic_plans() {
    assert_eq!(plans(FIXTURE, None), ["INGEST", "SEARCH"]);
    assert_eq!(plans(FIXTURE, Some("INGEST")), ["INGEST"]);

    let cfg = cs(FIXTURE);
    let (src, new) = (cs("INGEST"), cs("COPY"));
    let with_desc = take_ok(unsafe {
        SzConfigTool_cloneGenericPlan(
            cfg.as_ptr(),
            src.as_ptr(),
            new.as_ptr(),
            cs("Copy").as_ptr(),
        )
    });
    assert_eq!(plans(&with_desc, None), ["INGEST", "SEARCH", "COPY"]);
    let no_desc = take_ok(unsafe {
        SzConfigTool_cloneGenericPlan(cfg.as_ptr(), src.as_ptr(), new.as_ptr(), std::ptr::null())
    });
    assert_eq!(plans(&no_desc, None), ["INGEST", "SEARCH", "COPY"]);
    take_plain_err(unsafe {
        SzConfigTool_cloneGenericPlan(
            cfg.as_ptr(),
            cs("NOPE").as_ptr(),
            new.as_ptr(),
            std::ptr::null(),
        )
    });

    let set = take_ok(unsafe {
        SzConfigTool_setGenericPlan(cfg.as_ptr(), cs("NEWPLAN").as_ptr(), cs("New").as_ptr())
    });
    assert_eq!(plans(&set, None), ["INGEST", "SEARCH", "NEWPLAN"]);
}

// ---------------------------------------------------------------------------
// Thresholds and system parameters
// ---------------------------------------------------------------------------

#[test]
fn test_get_threshold_is_not_implemented() {
    let msg = take_plain_err(unsafe { SzConfigTool_getThreshold(cs(FIXTURE).as_ptr(), 1) });
    assert_eq!(msg, "Invalid input: get_threshold not yet implemented");
}

#[test]
fn test_system_parameters() {
    let cfg = cs(FIXTURE);
    let name = cs("relationshipsBreakMatches");
    let before = take_json(unsafe { SzConfigTool_listSystemParameters(cfg.as_ptr()) });
    assert!(before.is_object(), "{before}");
    let out = take_ok(unsafe {
        SzConfigTool_setSystemParameterWithJson(cfg.as_ptr(), name.as_ptr(), cs("1").as_ptr())
    });
    let after = take_json(unsafe { SzConfigTool_listSystemParameters(cs(&out).as_ptr()) });
    assert_eq!(after["relationshipsBreakMatches"], "1");

    let msg = take_err(
        unsafe {
            SzConfigTool_setSystemParameterWithJson(cfg.as_ptr(), name.as_ptr(), cs("{").as_ptr())
        },
        -3,
    );
    assert!(
        msg.starts_with("Failed to parse parameter_value_json: "),
        "{msg}"
    );
    let r = unsafe {
        SzConfigTool_setSystemParameterWithJson(cfg.as_ptr(), cs("nope").as_ptr(), cs("1").as_ptr())
    };
    take_lib_err(r, "INVALID_CONFIG");
}

// ---------------------------------------------------------------------------
// Versioning
// ---------------------------------------------------------------------------

#[test]
fn test_version_reads() {
    let cfg = cs(FIXTURE);
    assert_eq!(
        take_ok(unsafe { SzConfigTool_getVersion(cfg.as_ptr()) }),
        "4.4.0"
    );
    assert_eq!(
        take_ok(unsafe { SzConfigTool_getCompatibilityVersion(cfg.as_ptr()) }),
        "11"
    );
    let r = unsafe { SzConfigTool_verifyCompatibilityVersion(cfg.as_ptr(), cs("12").as_ptr()) };
    assert_eq!(take_ok(r), "11");
}

/// A versioning export taking (config, expected version).
type VersionCall = fn(&CString, &CString) -> SzConfigTool_result;

#[test]
fn test_version_reads_missing_and_nul() {
    let empty = cs(r#"{"G2_CONFIG":{}}"#);
    let nul = cs(NUL_VERSION_CFG);
    let want = cs("1");
    let cases: [(&str, VersionCall); 3] = [
        ("getVersion", |c, _| unsafe {
            SzConfigTool_getVersion(c.as_ptr())
        }),
        ("getCompatibilityVersion", |c, _| unsafe {
            SzConfigTool_getCompatibilityVersion(c.as_ptr())
        }),
        ("verifyCompatibilityVersion", |c, v| unsafe {
            SzConfigTool_verifyCompatibilityVersion(c.as_ptr(), v.as_ptr())
        }),
    ];
    for (name, call) in cases {
        let msg = take_plain_err(call(&empty, &want));
        assert!(msg.contains("not found"), "{name}: {msg}");
        let msg = take_err(call(&nul, &want), -4);
        assert!(
            msg.starts_with("Failed to convert result: "),
            "{name}: {msg}"
        );
    }
}

#[test]
fn test_version_updates() {
    let cfg = cs(FIXTURE);
    let out = take_ok(unsafe {
        SzConfigTool_updateCompatibilityVersion(cfg.as_ptr(), cs("12").as_ptr())
    });
    assert_eq!(
        take_ok(unsafe { SzConfigTool_getCompatibilityVersion(cs(&out).as_ptr()) }),
        "12"
    );
    let bad = cs(r#"{"G2_CONFIG":{"CONFIG_BASE_VERSION":{"COMPATIBILITY_VERSION":1}}}"#);
    take_lib_err(
        unsafe { SzConfigTool_updateCompatibilityVersion(bad.as_ptr(), cs("12").as_ptr()) },
        "INVALID_CONFIG",
    );

    let out = take_ok(unsafe { SzConfigTool_updateFeatureVersion(cfg.as_ptr(), cs("9").as_ptr()) });
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        v["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["FEATURE_VERSION"],
        "9"
    );
    take_lib_err(
        unsafe {
            SzConfigTool_updateFeatureVersion(cs(r#"{"G2_CONFIG":{}}"#).as_ptr(), cs("9").as_ptr())
        },
        "MISSING_SECTION",
    );
}

// ---------------------------------------------------------------------------
// Config sections
// ---------------------------------------------------------------------------

fn section_is_empty(config: &str, section: &str) -> SzConfigTool_result {
    unsafe { SzConfigTool_configSectionIsEmpty(cs(config).as_ptr(), cs(section).as_ptr()) }
}

#[test]
fn test_config_section_add_remove_empty() {
    let added = take_ok(unsafe {
        SzConfigTool_addConfigSection(cs(FIXTURE).as_ptr(), cs("cfg_new").as_ptr())
    });
    assert_eq!(take_ok(section_is_empty(&added, "CFG_NEW")), "true");
    assert_eq!(take_ok(section_is_empty(&added, "CFG_DSRC")), "false");
    take_plain_err(section_is_empty(FIXTURE, "CFG_NEW"));
    take_lib_err(
        unsafe { SzConfigTool_addConfigSection(cs(&added).as_ptr(), cs("CFG_NEW").as_ptr()) },
        "ALREADY_EXISTS",
    );

    let removed = take_ok(unsafe {
        SzConfigTool_removeConfigSection(cs(&added).as_ptr(), cs("CFG_NEW").as_ptr())
    });
    take_plain_err(section_is_empty(&removed, "CFG_NEW"));
    take_lib_err(
        unsafe { SzConfigTool_removeConfigSection(cs(&removed).as_ptr(), cs("CFG_NEW").as_ptr()) },
        "NOT_FOUND",
    );
}

#[test]
fn test_get_config_section() {
    let (cfg, sec) = (cs(FIXTURE), cs("CFG_DSRC"));
    let all = take_json(unsafe {
        SzConfigTool_getConfigSection(cfg.as_ptr(), sec.as_ptr(), std::ptr::null())
    });
    assert_eq!(codes(&all, "DSRC_CODE"), ["TEST", "SEARCH"]);
    let one = take_json(unsafe {
        SzConfigTool_getConfigSection(cfg.as_ptr(), sec.as_ptr(), cs("SEARCH").as_ptr())
    });
    assert_eq!(codes(&one, "DSRC_CODE"), ["SEARCH"]);
    take_plain_err(unsafe {
        SzConfigTool_getConfigSection(cfg.as_ptr(), cs("CFG_NOPE").as_ptr(), std::ptr::null())
    });
}
