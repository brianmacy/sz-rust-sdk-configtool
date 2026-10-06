//! C-boundary coverage for the fragment, data-source, feature, behavior-override,
//! validate/render, element, expression/comparison/distinct call and matching
//! function exports (`SzConfigTool_getFragment` .. `SzConfigTool_setMatchingFunction`).
//!
//! Every export is driven through the real library against the real Senzing
//! template fixture: the success path (response JSON / config effect), a NULL
//! for each pointer argument, invalid UTF-8 for each string argument, and the
//! library / JSON-parameter error paths, asserting the return code, the
//! response pointer and the per-thread last-error accessors.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde_json::Value;

use super::*;

fn template() -> String {
    let path = format!(
        "{}/../tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

fn cs(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn bad_utf8() -> CString {
    CString::new(vec![0xff, 0xfe]).unwrap()
}

fn last_error() -> Option<String> {
    let ptr = SzConfigTool_getLastError();
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

fn last_reason() -> Option<String> {
    let ptr = SzConfigTool_getLastErrorReasonCode();
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

/// Assert a successful result, free it, and return the response text.
fn ok(res: SzConfigTool_result) -> String {
    assert_eq!(res.returnCode, 0, "unexpected failure: {:?}", last_error());
    assert!(!res.response.is_null());
    let text = unsafe { CStr::from_ptr(res.response) }
        .to_str()
        .unwrap()
        .to_string();
    unsafe { SzConfigTool_free(res.response) };
    assert!(
        SzConfigTool_getLastError().is_null(),
        "success clears error"
    );
    assert_eq!(SzConfigTool_getLastErrorCode(), 0);
    text
}

/// Assert a failed result with `code`; return the recorded last-error message.
fn err(res: SzConfigTool_result, code: i64) -> String {
    assert_eq!(res.returnCode, code, "last error: {:?}", last_error());
    assert!(res.response.is_null());
    assert_eq!(SzConfigTool_getLastErrorCode(), code);
    last_error().expect("an error must record a message")
}

/// Assert a failed result whose message contains `needle`.
fn err_has(res: SzConfigTool_result, code: i64, needle: &str) {
    let msg = err(res, code);
    assert!(msg.contains(needle), "'{msg}' lacks '{needle}'");
}

/// Assert a library error routed through `handle_result!` (-2, classified).
fn lib_err(res: SzConfigTool_result, needle: &str) {
    err_has(res, -2, needle);
    assert!(
        last_reason().is_some(),
        "library errors carry a reason code"
    );
}

fn json(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

fn section(config: &str, name: &str) -> Vec<Value> {
    json(config)["G2_CONFIG"][name].as_array().unwrap().clone()
}

// ============================================================================
// Pointer-argument boundary checks (table-driven)
// ============================================================================

type Call = fn(&[*const c_char]) -> SzConfigTool_result;

/// An export, how to call it from a pointer slice, and its pointer arguments
/// as `(name, required)`; a non-required argument accepts NULL.
struct Boundary {
    call: Call,
    args: &'static [(&'static str, bool)],
}

const CFG: (&str, bool) = ("config_json", true);

fn boundaries() -> Vec<Boundary> {
    vec![
        Boundary {
            call: |a| SzConfigTool_getFragment(a[0], a[1]),
            args: &[CFG, ("code_or_id", true)],
        },
        Boundary {
            call: |a| SzConfigTool_listFragments(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_addFragment(a[0], a[1]),
            args: &[CFG, ("fragment_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_deleteFragment(a[0], a[1]),
            args: &[CFG, ("fragment_code", true)],
        },
        Boundary {
            call: |a| SzConfigTool_getDataSource(a[0], a[1]),
            args: &[CFG, ("code", true)],
        },
        Boundary {
            call: |a| SzConfigTool_setDataSource(a[0], a[1], a[2]),
            args: &[CFG, ("code", true), ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addFeature(a[0], a[1], a[2]),
            args: &[CFG, ("feature_code", true), ("feature_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_deleteFeature(a[0], a[1]),
            args: &[CFG, ("feature_code_or_id", true)],
        },
        Boundary {
            call: |a| SzConfigTool_setFeature(a[0], a[1], a[2]),
            args: &[CFG, ("feature_code_or_id", true), ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addBehaviorOverride(a[0], a[1], a[2], a[3]),
            args: &[
                CFG,
                ("feature_code", true),
                ("usage_type", true),
                ("behavior", true),
            ],
        },
        Boundary {
            call: |a| SzConfigTool_deleteBehaviorOverride(a[0], a[1], a[2]),
            args: &[CFG, ("feature_code", true), ("usage_type", true)],
        },
        Boundary {
            call: |a| SzConfigTool_listBehaviorOverrides(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_listBehaviorOverridesResolved(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_validateConfig(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_renderConfig(a[0], 2),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_addElement(a[0], a[1], a[2]),
            args: &[CFG, ("element_code", true), ("element_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_deleteElement(a[0], a[1]),
            args: &[CFG, ("element_code", true)],
        },
        Boundary {
            call: |a| SzConfigTool_setElement(a[0], a[1], a[2]),
            args: &[CFG, ("element_code", true), ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addExpressionCall(a[0], a[1], a[2], -1, a[3], a[4], a[5], a[6]),
            args: &[
                CFG,
                ("ftype_code", false),
                ("felem_code", false),
                ("efunc_code", true),
                ("element_list_json", true),
                ("expression_feature", false),
                ("is_virtual", true),
            ],
        },
        Boundary {
            call: |a| SzConfigTool_deleteExpressionCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_getExpressionCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_listExpressionCalls(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_setExpressionCall(a[0], 1, a[1]),
            args: &[CFG, ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addComparisonCall(a[0], a[1], a[2], a[3]),
            args: &[
                CFG,
                ("ftype_code", true),
                ("cfunc_code", true),
                ("element_list_json", true),
            ],
        },
        Boundary {
            call: |a| SzConfigTool_deleteComparisonCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_getComparisonCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_listComparisonCalls(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_setComparisonCall(a[0], 1, a[1]),
            args: &[CFG, ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addDistinctCall(a[0], a[1], a[2], a[3]),
            args: &[
                CFG,
                ("ftype_code", true),
                ("dfunc_code", true),
                ("element_list_json", true),
            ],
        },
        Boundary {
            call: |a| SzConfigTool_deleteDistinctCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_getDistinctCall(a[0], 1),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_listDistinctCalls(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_setDistinctCall(a[0], 1, a[1]),
            args: &[CFG, ("updates_json", true)],
        },
        Boundary {
            call: |a| SzConfigTool_addMatchingFunction(a[0], a[1], a[2]),
            args: &[CFG, ("rtype_code", true), ("matching_func", true)],
        },
        Boundary {
            call: |a| SzConfigTool_deleteMatchingFunction(a[0], a[1]),
            args: &[CFG, ("rtype_code", true)],
        },
        Boundary {
            call: |a| SzConfigTool_getMatchingFunction(a[0], a[1]),
            args: &[CFG, ("rtype_code", true)],
        },
        Boundary {
            call: |a| SzConfigTool_listMatchingFunctions(a[0]),
            args: &[CFG],
        },
        Boundary {
            call: |a| SzConfigTool_setMatchingFunction(a[0], a[1], a[2]),
            args: &[CFG, ("rtype_code", true), ("matching_func", false)],
        },
    ]
}

#[test]
fn test_null_required_pointer_is_rejected() {
    let filler = cs("x");
    for b in boundaries() {
        for (idx, (name, required)) in b.args.iter().enumerate() {
            if !required {
                continue;
            }
            let mut ptrs = vec![filler.as_ptr(); b.args.len()];
            ptrs[idx] = std::ptr::null();
            let msg = err((b.call)(&ptrs), -1);
            assert_eq!(msg, format!("{name} is null"));
            assert!(last_reason().is_none(), "boundary errors carry no reason");
        }
    }
}

#[test]
fn test_invalid_utf8_pointer_is_rejected() {
    let filler = cs("x");
    let bad = bad_utf8();
    for b in boundaries() {
        for (idx, (name, _)) in b.args.iter().enumerate() {
            let mut ptrs = vec![filler.as_ptr(); b.args.len()];
            ptrs[idx] = bad.as_ptr();
            let msg = err((b.call)(&ptrs), -2);
            assert!(
                msg.starts_with(&format!("Invalid UTF-8 in {name}: ")),
                "unexpected message: {msg}"
            );
        }
    }
}

// ============================================================================
// Fragments
// ============================================================================

#[test]
fn test_get_fragment_by_code_and_id() {
    let config = cs(&template());
    for key in ["SAME_NAME", "11"] {
        let k = cs(key);
        let frag = json(&ok(SzConfigTool_getFragment(config.as_ptr(), k.as_ptr())));
        assert_eq!(frag["fragment"], "SAME_NAME");
        assert_eq!(frag["id"], 11);
    }
}

#[test]
fn test_get_fragment_errors() {
    let config = cs(&template());
    let nope = cs("NOPE");
    err_has(
        SzConfigTool_getFragment(config.as_ptr(), nope.as_ptr()),
        -5,
        "Fragment not found",
    );
    let bogus = cs("not json");
    err(
        (SzConfigTool_getFragment)(bogus.as_ptr(), nope.as_ptr()),
        -5,
    );
}

#[test]
fn test_list_fragments() {
    let config = cs(&template());
    let list = json(&ok(SzConfigTool_listFragments(config.as_ptr())));
    let items = list.as_array().unwrap();
    assert!(items.iter().any(|f| f["fragment"] == "SAME_NAME"));
    let bogus = cs("not json");
    err(SzConfigTool_listFragments(bogus.as_ptr()), -5);
}

#[test]
fn test_add_fragment() {
    let config = cs(&template());
    let frag = cs(r#"{"ERFRAG_CODE":"COV_FRAG","ERFRAG_SOURCE":"./FRAGMENT[./SAME_NAME>0]"}"#);
    let out = cs(&ok(SzConfigTool_addFragment(
        config.as_ptr(),
        frag.as_ptr(),
    )));
    let code = cs("COV_FRAG");
    let got = json(&ok(SzConfigTool_getFragment(out.as_ptr(), code.as_ptr())));
    assert_eq!(got["source"], "./FRAGMENT[./SAME_NAME>0]");

    let bad_json = cs("{");
    err_has(
        SzConfigTool_addFragment(config.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in fragment_json",
    );
    let missing = cs("{}");
    err_has(
        SzConfigTool_addFragment(config.as_ptr(), missing.as_ptr()),
        -5,
        "ERFRAG_CODE",
    );
}

#[test]
fn test_delete_fragment() {
    let config = cs(&template());
    let code = cs("SAME_NAME");
    let out = cs(&ok(SzConfigTool_deleteFragment(
        config.as_ptr(),
        code.as_ptr(),
    )));
    err(SzConfigTool_getFragment(out.as_ptr(), code.as_ptr()), -5);
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_deleteFragment(config.as_ptr(), nope.as_ptr()),
        "Fragment not found",
    );
}

// ============================================================================
// Data sources
// ============================================================================

#[test]
fn test_get_data_source() {
    let config = cs(&template());
    let code = cs("TEST");
    let ds = ok(SzConfigTool_getDataSource(config.as_ptr(), code.as_ptr()));
    assert!(ds.contains("TEST"), "{ds}");
    let nope = cs("NOPE");
    err(
        SzConfigTool_getDataSource(config.as_ptr(), nope.as_ptr()),
        -5,
    );
}

#[test]
fn test_set_data_source() {
    let config = cs(&template());
    let code = cs("TEST");
    let updates = cs(r#"{"retentionLevel":"Remember"}"#);
    let out = ok(SzConfigTool_setDataSource(
        config.as_ptr(),
        code.as_ptr(),
        updates.as_ptr(),
    ));
    let dsrc = section(&out, "CFG_DSRC");
    let test = dsrc.iter().find(|d| d["DSRC_CODE"] == "TEST").unwrap();
    assert_eq!(test["RETENTION_LEVEL"], "Remember");

    let empty = cs("{}");
    ok(SzConfigTool_setDataSource(
        config.as_ptr(),
        code.as_ptr(),
        empty.as_ptr(),
    ));
    let bad_json = cs("[");
    err_has(
        SzConfigTool_setDataSource(config.as_ptr(), code.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in updates_json",
    );
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_setDataSource(config.as_ptr(), nope.as_ptr(), empty.as_ptr()),
        "Data source not found",
    );
}

/// A NUL smuggled in as a JSON escape is re-escaped by serde_json on output,
/// so a returned config never holds an interior NUL (the exports' "Failed to
/// create C string" arms cannot be reached).
#[test]
fn test_escaped_nul_round_trips_without_interior_nul() {
    let config = cs(&template());
    let code = cs("TEST");
    let updates = cs(r#"{"retentionLevel":"a\u0000b"}"#);
    let out = ok(SzConfigTool_setDataSource(
        config.as_ptr(),
        code.as_ptr(),
        updates.as_ptr(),
    ));
    assert!(out.contains(r#""RETENTION_LEVEL":"a\u0000b""#));
    let ds = ok(SzConfigTool_getDataSource(cs(&out).as_ptr(), code.as_ptr()));
    assert!(!ds.contains('\0'));
}

// ============================================================================
// Features
// ============================================================================

fn feature_row(config: &str, code: &str) -> Option<Value> {
    section(config, "CFG_FTYPE")
        .into_iter()
        .find(|f| f["FTYPE_CODE"] == code)
}

#[test]
fn test_add_feature_with_every_option() {
    let config = cs(&template());
    let code = cs("COV_FEAT_A");
    let spec = cs(r#"{
        "elementList":[{"element":"FULL_NAME","expressed":"Yes","compared":"Yes"}],
        "class":"OTHER","behavior":"FM","candidates":"No","anonymize":"No",
        "derived":"No","history":"No","matchkey":"Yes",
        "standardize":"PARSE_NAME","expression":"EXPRESS_BOM","comparison":"STR_COMP",
        "version":2,"rtype_id":0,"id":2001}"#);
    let out = ok(SzConfigTool_addFeature(
        config.as_ptr(),
        code.as_ptr(),
        spec.as_ptr(),
    ));
    let row = feature_row(&out, "COV_FEAT_A").expect("feature added");
    assert_eq!(row["FTYPE_ID"], 2001);
}

#[test]
fn test_add_feature_with_alternate_keys() {
    let config = cs(&template());
    let code = cs("COV_FEAT_B");
    let spec = cs(r#"{"element_list":["FULL_NAME"],"matchKey":"No","rtypeId":0}"#);
    let out = ok(SzConfigTool_addFeature(
        config.as_ptr(),
        code.as_ptr(),
        spec.as_ptr(),
    ));
    assert!(feature_row(&out, "COV_FEAT_B").is_some());
}

#[test]
fn test_add_feature_errors() {
    let config = cs(&template());
    let code = cs("COV_FEAT_C");
    let bad_json = cs("{");
    err_has(
        SzConfigTool_addFeature(config.as_ptr(), code.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in feature_json",
    );
    let missing = cs("{}");
    let msg = err(
        SzConfigTool_addFeature(config.as_ptr(), code.as_ptr(), missing.as_ptr()),
        -3,
    );
    assert_eq!(msg, "Missing required field: elementList");
    let dup = cs("NAME");
    let spec = cs(r#"{"elementList":["FULL_NAME"]}"#);
    err(
        SzConfigTool_addFeature(config.as_ptr(), dup.as_ptr(), spec.as_ptr()),
        -5,
    );
}

#[test]
fn test_delete_feature() {
    let config = cs(&template());
    let code = cs("COV_FEAT_D");
    let spec = cs(r#"{"elementList":["FULL_NAME"]}"#);
    let added = cs(&ok(SzConfigTool_addFeature(
        config.as_ptr(),
        code.as_ptr(),
        spec.as_ptr(),
    )));
    let out = ok(SzConfigTool_deleteFeature(added.as_ptr(), code.as_ptr()));
    assert!(feature_row(&out, "COV_FEAT_D").is_none());
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_deleteFeature(config.as_ptr(), nope.as_ptr()),
        "Feature not found",
    );
}

#[test]
fn test_set_feature() {
    let config = cs(&template());
    let code = cs("NAME");
    let full = cs(
        r#"{"candidates":"Yes","anonymize":"No","derived":"No","history":"Yes",
        "matchkey":"Yes","behavior":"NAME","class":"NAME","version":3,"rtypeId":0}"#,
    );
    let out = ok(SzConfigTool_setFeature(
        config.as_ptr(),
        code.as_ptr(),
        full.as_ptr(),
    ));
    assert_eq!(feature_row(&out, "NAME").unwrap()["FTYPE_EXCL"], "No");
    let alt = cs(r#"{"matchKey":"No","RTYPE_ID":0}"#);
    ok(SzConfigTool_setFeature(
        config.as_ptr(),
        code.as_ptr(),
        alt.as_ptr(),
    ));
    let bad_json = cs("{");
    err_has(
        SzConfigTool_setFeature(config.as_ptr(), code.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in updates_json",
    );
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_setFeature(config.as_ptr(), nope.as_ptr(), alt.as_ptr()),
        "Feature not found",
    );
}

// ============================================================================
// Behavior overrides, validate, render
// ============================================================================

#[test]
fn test_add_and_delete_behavior_override() {
    let config = cs(&template());
    let (name, utype, f1) = (cs("NAME"), cs("COVTYPE"), cs("F1"));
    let out = ok(SzConfigTool_addBehaviorOverride(
        config.as_ptr(),
        name.as_ptr(),
        utype.as_ptr(),
        f1.as_ptr(),
    ));
    let fbovr = section(&out, "CFG_FBOVR");
    assert!(fbovr.iter().any(|r| r["UTYPE_CODE"] == "COVTYPE"));

    let (addr, business) = (cs("ADDRESS"), cs("BUSINESS"));
    lib_err(
        SzConfigTool_addBehaviorOverride(
            config.as_ptr(),
            addr.as_ptr(),
            business.as_ptr(),
            f1.as_ptr(),
        ),
        "already exists",
    );

    let out = ok(SzConfigTool_deleteBehaviorOverride(
        config.as_ptr(),
        addr.as_ptr(),
        business.as_ptr(),
    ));
    assert_eq!(section(&out, "CFG_FBOVR").len(), 3);
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_deleteBehaviorOverride(config.as_ptr(), name.as_ptr(), nope.as_ptr()),
        "NOPE",
    );
}

#[test]
fn test_list_behavior_overrides_both_shapes() {
    let config = cs(&template());
    let raw = json(&ok(SzConfigTool_listBehaviorOverrides(config.as_ptr())));
    assert_eq!(raw.as_array().unwrap().len(), 4);
    let resolved = json(&ok(SzConfigTool_listBehaviorOverridesResolved(
        config.as_ptr(),
    )));
    let rows = resolved.as_array().unwrap();
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|r| r.get("usageType").is_some()));

    let bogus = cs("not json");
    err(SzConfigTool_listBehaviorOverrides(bogus.as_ptr()), -2);
    err(
        SzConfigTool_listBehaviorOverridesResolved(bogus.as_ptr()),
        -2,
    );
}

#[test]
fn test_validate_config() {
    let config = cs(&template());
    assert_eq!(ok(SzConfigTool_validateConfig(config.as_ptr())), "OK");
    let empty = cs("{}");
    lib_err(SzConfigTool_validateConfig(empty.as_ptr()), "G2_CONFIG");
}

#[test]
fn test_render_config_indent_and_clamp() {
    let config = cs(r#"{"G2_CONFIG":{"B":1,"A":2}}"#);
    let two = ok(SzConfigTool_renderConfig(config.as_ptr(), 2));
    assert_eq!(
        two,
        "{\n  \"G2_CONFIG\": {\n    \"A\": 2,\n    \"B\": 1\n  }\n}"
    );
    let clamped = ok(SzConfigTool_renderConfig(config.as_ptr(), -5));
    assert_eq!(clamped, "{\n\"G2_CONFIG\": {\n\"A\": 2,\n\"B\": 1\n}\n}");
    let bogus = cs("not json");
    err(SzConfigTool_renderConfig(bogus.as_ptr(), 2), -2);
}

// ============================================================================
// Elements
// ============================================================================

fn element_row(config: &str, code: &str) -> Option<Value> {
    section(config, "CFG_FELEM")
        .into_iter()
        .find(|f| f["FELEM_CODE"] == code)
}

#[test]
fn test_add_element_key_variants() {
    let config = cs(&template());
    let cases = [
        (
            "COV_ELEM_A",
            r#"{"description":"desc a","dataType":"string","id":5001}"#,
            "desc a",
        ),
        (
            "COV_ELEM_B",
            r#"{"FELEM_DESC":"desc b","DATA_TYPE":"string"}"#,
            "desc b",
        ),
    ];
    for (code, spec, desc) in cases {
        let (c, s) = (cs(code), cs(spec));
        let out = ok(SzConfigTool_addElement(
            config.as_ptr(),
            c.as_ptr(),
            s.as_ptr(),
        ));
        assert_eq!(element_row(&out, code).unwrap()["FELEM_DESC"], desc);
    }
    let code = cs("COV_ELEM_C");
    let empty = cs("{}");
    let out = ok(SzConfigTool_addElement(
        config.as_ptr(),
        code.as_ptr(),
        empty.as_ptr(),
    ));
    assert!(element_row(&out, "COV_ELEM_C").is_some());
}

#[test]
fn test_add_element_errors() {
    let config = cs(&template());
    let code = cs("COV_ELEM_X");
    let bad_json = cs("{");
    err_has(
        SzConfigTool_addElement(config.as_ptr(), code.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in element_json",
    );
    let dup = cs("FULL_NAME");
    let empty = cs("{}");
    lib_err(
        SzConfigTool_addElement(config.as_ptr(), dup.as_ptr(), empty.as_ptr()),
        "FULL_NAME",
    );
}

#[test]
fn test_delete_element() {
    let config = cs(&template());
    let code = cs("COV_ELEM_D");
    let empty = cs("{}");
    let added = cs(&ok(SzConfigTool_addElement(
        config.as_ptr(),
        code.as_ptr(),
        empty.as_ptr(),
    )));
    let out = ok(SzConfigTool_deleteElement(added.as_ptr(), code.as_ptr()));
    assert!(element_row(&out, "COV_ELEM_D").is_none());
    let nope = cs("NOPE");
    lib_err(
        SzConfigTool_deleteElement(config.as_ptr(), nope.as_ptr()),
        "Element does not exist",
    );
}

#[test]
fn test_set_element() {
    let config = cs(&template());
    let code = cs("FULL_NAME");
    let cases = [
        (r#"{"description":"new a","dataType":"string"}"#, "new a"),
        (r#"{"FELEM_DESC":"new b","DATA_TYPE":"string"}"#, "new b"),
    ];
    for (spec, desc) in cases {
        let s = cs(spec);
        let out = ok(SzConfigTool_setElement(
            config.as_ptr(),
            code.as_ptr(),
            s.as_ptr(),
        ));
        assert_eq!(element_row(&out, "FULL_NAME").unwrap()["FELEM_DESC"], desc);
    }
    let bad_json = cs("{");
    err_has(
        SzConfigTool_setElement(config.as_ptr(), code.as_ptr(), bad_json.as_ptr()),
        -3,
        "Invalid JSON in updates_json",
    );
    let nope = cs("NOPE");
    let empty = cs("{}");
    lib_err(
        SzConfigTool_setElement(config.as_ptr(), nope.as_ptr(), empty.as_ptr()),
        "NOPE",
    );
}

// ============================================================================
// Expression calls
// ============================================================================

/// Optional C string: `None` becomes NULL.
fn opt_ptr(s: &Option<CString>) -> *const c_char {
    s.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

struct ExprCall {
    ftype: Option<&'static str>,
    felem: Option<&'static str>,
    exec_order: i64,
    element_list: &'static str,
    expression_feature: Option<&'static str>,
}

fn add_expression_call(config: &CString, call: &ExprCall) -> SzConfigTool_result {
    let ftype = call.ftype.map(cs);
    let felem = call.felem.map(cs);
    let expr = call.expression_feature.map(cs);
    let (efunc, list, virt) = (cs("EXPRESS_BOM"), cs(call.element_list), cs("No"));
    SzConfigTool_addExpressionCall(
        config.as_ptr(),
        opt_ptr(&ftype),
        opt_ptr(&felem),
        call.exec_order,
        efunc.as_ptr(),
        list.as_ptr(),
        opt_ptr(&expr),
        virt.as_ptr(),
    )
}

#[test]
fn test_add_expression_call_element_list_shapes() {
    let template = template();
    let config = cs(&template);
    // Array items (3, 2 and too-short), object items (full and defaulted) and a
    // non-array/non-object item that is skipped.
    let call = ExprCall {
        ftype: None,
        felem: Some("FULL_NAME"),
        exec_order: -1,
        element_list: r#"[["FULL_NAME","Yes","PARENT"],["ORG_NAME","No"],["SKIPPED"],
            {"element":"SUR_NAME","required":"No","feature":"NAME"},
            {"element":"GIVEN_NAME"},42]"#,
        expression_feature: None,
    };
    let out = ok(add_expression_call(&config, &call));
    let before = section(&template, "CFG_EFBOM").len();
    assert_eq!(section(&out, "CFG_EFBOM").len(), before + 4);
    assert_eq!(
        section(&out, "CFG_EFCALL").len(),
        section(&template, "CFG_EFCALL").len() + 1
    );
}

#[test]
fn test_add_expression_call_feature_mode_with_exec_order() {
    let config = cs(&template());
    let call = ExprCall {
        ftype: Some("NAME"),
        felem: Some("N/A"),
        exec_order: 900,
        element_list: r#"[["FULL_NAME","Yes"]]"#,
        expression_feature: Some("NAME"),
    };
    let out = ok(add_expression_call(&config, &call));
    let rows = section(&out, "CFG_EFCALL");
    assert!(
        rows.iter()
            .any(|r| r["EXEC_ORDER"] == 900 && r["FTYPE_ID"] == 1)
    );
}

#[test]
fn test_add_expression_call_errors() {
    let config = cs(&template());
    let base = |element_list| ExprCall {
        ftype: None,
        felem: Some("FULL_NAME"),
        exec_order: -1,
        element_list,
        expression_feature: None,
    };
    err_has(
        add_expression_call(&config, &base("[")),
        -3,
        "Invalid JSON in element_list_json",
    );
    let msg = err(add_expression_call(&config, &base(r#"{"a":1}"#)), -3);
    assert_eq!(msg, "element_list_json must be a JSON array");
    let neither = ExprCall {
        felem: None,
        ..base("[]")
    };
    err_has(
        add_expression_call(&config, &neither),
        -5,
        "Either a feature or an element",
    );
}

// ============================================================================
// Expression / comparison / distinct call get, list, delete, set
// ============================================================================

type IdCall = extern "C" fn(*const c_char, i64) -> SzConfigTool_result;
type ListCall = extern "C" fn(*const c_char) -> SzConfigTool_result;
type SetCall = extern "C" fn(*const c_char, i64, *const c_char) -> SzConfigTool_result;

/// One call family's id-based exports, its section, and the code the library
/// error paths return for get/delete (delete via `handle_result!` is -2).
struct CallFamily {
    section: &'static str,
    id_key: &'static str,
    get: IdCall,
    list: ListCall,
    delete: IdCall,
    delete_err: i64,
    set: SetCall,
    set_parse_msg: &'static str,
}

fn call_families() -> [CallFamily; 3] {
    [
        CallFamily {
            section: "CFG_EFCALL",
            id_key: "EFCALL_ID",
            get: SzConfigTool_getExpressionCall,
            list: SzConfigTool_listExpressionCalls,
            delete: SzConfigTool_deleteExpressionCall,
            delete_err: -2,
            set: SzConfigTool_setExpressionCall,
            set_parse_msg: "Invalid JSON in updates_json",
        },
        CallFamily {
            section: "CFG_CFCALL",
            id_key: "CFCALL_ID",
            get: SzConfigTool_getComparisonCall,
            list: SzConfigTool_listComparisonCalls,
            delete: SzConfigTool_deleteComparisonCall,
            delete_err: -2,
            set: SzConfigTool_setComparisonCall,
            set_parse_msg: "Invalid JSON in updates_json",
        },
        CallFamily {
            section: "CFG_DFCALL",
            id_key: "DFCALL_ID",
            get: SzConfigTool_getDistinctCall,
            list: SzConfigTool_listDistinctCalls,
            delete: SzConfigTool_deleteDistinctCall,
            delete_err: -5,
            set: SzConfigTool_setDistinctCall,
            set_parse_msg: "Failed to parse updates_json",
        },
    ]
}

#[test]
fn test_call_get_list_delete_set() {
    let template = template();
    let config = cs(&template);
    let bogus = cs("not json");
    for fam in call_families() {
        let rows = section(&template, fam.section);

        let record = json(&ok((fam.get)(config.as_ptr(), 1)));
        assert!(record.is_object(), "{}: {record}", fam.section);
        err((fam.get)(config.as_ptr(), 999_999), -5);

        let list = json(&ok((fam.list)(config.as_ptr())));
        assert_eq!(
            list.as_array().unwrap().len(),
            rows.len(),
            "{}",
            fam.section
        );
        err((fam.list)(bogus.as_ptr()), -5);

        let out = ok((fam.delete)(config.as_ptr(), 1));
        let remaining = section(&out, fam.section);
        assert_eq!(remaining.len(), rows.len() - 1);
        assert!(remaining.iter().all(|r| r[fam.id_key] != 1));
        err((fam.delete)(config.as_ptr(), 999_999), fam.delete_err);

        // The set-call library functions are documented stubs: the config is
        // returned unchanged.
        for updates in [r#"{"execOrder":5}"#, "{}"] {
            let u = cs(updates);
            let out = ok((fam.set)(config.as_ptr(), 1, u.as_ptr()));
            assert_eq!(out, template);
        }
        let bad_json = cs("{");
        err_has(
            (fam.set)(config.as_ptr(), 1, bad_json.as_ptr()),
            -3,
            fam.set_parse_msg,
        );
    }
}

// ============================================================================
// Comparison / distinct call add
// ============================================================================

type AddCall = extern "C" fn(
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
) -> SzConfigTool_result;

#[test]
fn test_add_comparison_and_distinct_calls() {
    let template = template();
    let config = cs(&template);
    let cases: [(AddCall, &str, &str, &str); 2] = [
        (
            SzConfigTool_addComparisonCall,
            "STR_COMP",
            "CFG_CFCALL",
            "Invalid JSON in element_list_json",
        ),
        (
            SzConfigTool_addDistinctCall,
            "FELEM_STRICT_SUBSET",
            "CFG_DFCALL",
            "Failed to parse element_list_json",
        ),
    ];
    let name_key = cs("NAME_KEY");
    // Non-string items are skipped by the list conversion.
    let list = cs(r#"["FULL_NAME",7,"ORG_NAME"]"#);
    for (add, func, sect, parse_msg) in cases {
        let f = cs(func);
        let out = ok(add(
            config.as_ptr(),
            name_key.as_ptr(),
            f.as_ptr(),
            list.as_ptr(),
        ));
        assert_eq!(
            section(&out, sect).len(),
            section(&template, sect).len() + 1
        );

        let bad_json = cs("[");
        err_has(
            add(
                config.as_ptr(),
                name_key.as_ptr(),
                f.as_ptr(),
                bad_json.as_ptr(),
            ),
            -3,
            parse_msg,
        );
        let not_array = cs(r#"{"a":1}"#);
        let msg = err(
            add(
                config.as_ptr(),
                name_key.as_ptr(),
                f.as_ptr(),
                not_array.as_ptr(),
            ),
            -3,
        );
        assert_eq!(msg, "element_list_json must be a JSON array");
        // NAME already has a call of each kind (only one allowed per feature).
        let name = cs("NAME");
        err(
            add(config.as_ptr(), name.as_ptr(), f.as_ptr(), list.as_ptr()),
            -5,
        );
    }
}

// ============================================================================
// Matching functions (library placeholders: always "not implemented")
// ============================================================================

#[test]
fn test_matching_functions_report_not_implemented() {
    let config = cs(&template());
    let (rtype, func) = (cs("RESOLVED"), cs("x"));
    let (c, r, f) = (config.as_ptr(), rtype.as_ptr(), func.as_ptr());
    let calls: [&dyn Fn() -> SzConfigTool_result; 6] = [
        &|| SzConfigTool_addMatchingFunction(c, r, f),
        &|| SzConfigTool_deleteMatchingFunction(c, r),
        &|| SzConfigTool_getMatchingFunction(c, r),
        &|| SzConfigTool_listMatchingFunctions(c),
        &|| SzConfigTool_setMatchingFunction(c, r, f),
        &|| SzConfigTool_setMatchingFunction(c, r, std::ptr::null()),
    ];
    for call in calls {
        let msg = err(call(), -5);
        assert!(msg.to_lowercase().contains("not implemented"), "{msg}");
    }
}
