//! Coverage of the distinct / candidate / validation / scoring function
//! exports, the wave-4 feature-element, settings, cascade-delete, by-feature
//! call and call-element exports, and the JSON function setters: success
//! paths (with their effect on the config), every NULL / invalid-UTF-8
//! argument, and library error paths, each checked against the return code
//! and the per-thread last-error accessors.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde_json::{Value, json};

use super::*;

type P = *const c_char;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../tests/fixtures/g2config_template.json"
);

fn fixture() -> CString {
    CString::new(std::fs::read_to_string(FIXTURE).expect("read fixture")).unwrap()
}

fn cs(s: &str) -> CString {
    CString::new(s).unwrap()
}

/// A C string that is not valid UTF-8.
fn bad_utf8() -> CString {
    CString::new(vec![0xff, 0xfe]).unwrap()
}

fn borrowed(ptr: P) -> Option<String> {
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

fn last_error() -> Option<String> {
    borrowed(SzConfigTool_getLastError())
}

fn last_reason() -> Option<String> {
    borrowed(SzConfigTool_getLastErrorReasonCode())
}

/// Assert success, free the response and return it; success clears the
/// thread's last error.
fn take_ok(result: SzConfigTool_result) -> String {
    let message = last_error();
    assert_eq!(
        result.returnCode, 0,
        "expected success, last error: {message:?}"
    );
    assert!(!result.response.is_null(), "success response was null");
    let s = borrowed(result.response).unwrap();
    unsafe { SzConfigTool_free(result.response) };
    assert_eq!(SzConfigTool_getLastErrorCode(), 0);
    assert!(last_error().is_none(), "success must clear the last error");
    s
}

/// Assert an error result: `code`, NULL response, and a last error with the
/// same code whose message contains `needle`.
fn assert_err(result: SzConfigTool_result, code: i64, needle: &str) {
    let message = last_error();
    assert!(
        result.response.is_null(),
        "expected error {code} ({needle}) but got a response"
    );
    assert_eq!(
        result.returnCode, code,
        "return code, last error: {message:?}"
    );
    assert_eq!(SzConfigTool_getLastErrorCode(), code);
    let message = message.expect("an error must record a last-error message");
    assert!(
        message.contains(needle),
        "last error {message:?} does not contain {needle:?}"
    );
}

/// [`assert_err`] plus the expected stable reason code (None = unclassified).
fn assert_err_reason(result: SzConfigTool_result, code: i64, needle: &str, reason: Option<&str>) {
    assert_err(result, code, needle);
    assert_eq!(last_reason().as_deref(), reason);
}

fn rows<'a>(config: &'a Value, section: &str) -> &'a Vec<Value> {
    config["G2_CONFIG"][section].as_array().unwrap()
}

fn find_row<'a>(config: &'a Value, section: &str, col: &str, code: &str) -> Option<&'a Value> {
    rows(config, section).iter().find(|r| r[col] == json!(code))
}

fn parse(s: &str) -> Value {
    serde_json::from_str(s).unwrap()
}

// ---------------------------------------------------------------------------
// Boundary probing: NULL and invalid UTF-8 for every pointer argument.
// ---------------------------------------------------------------------------

/// One C-string argument of an export.
struct Arg {
    name: &'static str,
    value: CString,
    /// NULL is rejected with -1 "<name> is null" (otherwise NULL is allowed).
    required: bool,
    /// Return code for invalid UTF-8 in this argument.
    utf8_code: i64,
}

fn req(name: &'static str, value: &str) -> Arg {
    Arg {
        name,
        value: cs(value),
        required: true,
        utf8_code: -2,
    }
}

fn opt(name: &'static str, value: &str) -> Arg {
    Arg {
        name,
        value: cs(value),
        required: false,
        utf8_code: -2,
    }
}

/// For each argument in turn (all others valid): a NULL (when required) must
/// fail with -1 "<name> is null", and invalid UTF-8 must fail with the
/// argument's code and "Invalid UTF-8 in <name>". Arguments are validated in
/// order, so the probed one is always the one reported.
fn probe_args(args: &[Arg], call: &dyn Fn(&[P]) -> SzConfigTool_result) {
    let bad = bad_utf8();
    for (i, arg) in args.iter().enumerate() {
        let with = |sub: P| -> Vec<P> {
            args.iter()
                .enumerate()
                .map(|(j, a)| if j == i { sub } else { a.value.as_ptr() })
                .collect()
        };
        if arg.required {
            assert_err_reason(
                call(&with(std::ptr::null())),
                -1,
                &format!("{} is null", arg.name),
                None,
            );
        }
        assert_err_reason(
            call(&with(bad.as_ptr())),
            arg.utf8_code,
            &format!("Invalid UTF-8 in {}", arg.name),
            None,
        );
    }
}

// ---------------------------------------------------------------------------
// Distinct functions (direct-arg forms)
// ---------------------------------------------------------------------------

#[test]
fn test_distinct_function_lifecycle() {
    let config = fixture();
    let code = cs("my_dfunc");
    let (connect, desc, lang) = (cs("myConnect"), cs("My desc"), cs("C"));

    let added = take_ok(SzConfigTool_addDistinctFunction(
        config.as_ptr(),
        code.as_ptr(),
        connect.as_ptr(),
        desc.as_ptr(),
        lang.as_ptr(),
    ));
    let v = parse(&added);
    let row = find_row(&v, "CFG_DFUNC", "DFUNC_CODE", "MY_DFUNC").expect("added row");
    assert_eq!(row["CONNECT_STR"], json!("myConnect"));
    assert_eq!(row["DFUNC_DESC"], json!("My desc"));
    assert_eq!(row["LANGUAGE"], json!("C"));
    let added_c = cs(&added);

    let got = parse(&take_ok(SzConfigTool_getDistinctFunction(
        added_c.as_ptr(),
        code.as_ptr(),
    )));
    assert_eq!(got["DFUNC_CODE"], json!("MY_DFUNC"));

    let listed = parse(&take_ok(SzConfigTool_listDistinctFunctions(
        added_c.as_ptr(),
    )));
    let listed = listed.as_array().unwrap();
    assert_eq!(listed.len(), rows(&v, "CFG_DFUNC").len());
    assert!(listed.iter().any(|r| r["function"] == json!("MY_DFUNC")));

    let (connect2, desc2, lang2) = (cs("otherConnect"), cs("Other desc"), cs("Java"));
    let set = parse(&take_ok(SzConfigTool_setDistinctFunction(
        added_c.as_ptr(),
        code.as_ptr(),
        connect2.as_ptr(),
        desc2.as_ptr(),
        lang2.as_ptr(),
    )));
    let row = find_row(&set, "CFG_DFUNC", "DFUNC_CODE", "MY_DFUNC").unwrap();
    assert_eq!(row["CONNECT_STR"], json!("otherConnect"));
    assert_eq!(row["DFUNC_DESC"], json!("Other desc"));
    assert_eq!(row["LANGUAGE"], json!("Java"));

    // All optional args NULL: every field is left untouched.
    let left = parse(&take_ok(SzConfigTool_setDistinctFunction(
        added_c.as_ptr(),
        code.as_ptr(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
    )));
    assert_eq!(
        find_row(&left, "CFG_DFUNC", "DFUNC_CODE", "MY_DFUNC"),
        find_row(&v, "CFG_DFUNC", "DFUNC_CODE", "MY_DFUNC")
    );

    let deleted = parse(&take_ok(SzConfigTool_deleteDistinctFunction(
        added_c.as_ptr(),
        code.as_ptr(),
    )));
    assert!(find_row(&deleted, "CFG_DFUNC", "DFUNC_CODE", "MY_DFUNC").is_none());
    assert_eq!(
        rows(&deleted, "CFG_DFUNC").len(),
        rows(&v, "CFG_DFUNC").len() - 1
    );
}

#[test]
fn test_distinct_function_library_errors() {
    let config = fixture();
    let existing = cs("FELEM_STRICT_SUBSET");
    let missing = cs("NO_SUCH_DFUNC");
    let connect = cs("x");
    let not_json = cs("not json");
    let null = std::ptr::null();

    assert_err_reason(
        SzConfigTool_addDistinctFunction(
            config.as_ptr(),
            existing.as_ptr(),
            connect.as_ptr(),
            null,
            null,
        ),
        -5,
        "FELEM_STRICT_SUBSET",
        None,
    );
    assert_err_reason(
        SzConfigTool_deleteDistinctFunction(config.as_ptr(), missing.as_ptr()),
        -5,
        "NO_SUCH_DFUNC",
        None,
    );
    assert_err_reason(
        SzConfigTool_getDistinctFunction(config.as_ptr(), missing.as_ptr()),
        -5,
        "Distinct function not found: NO_SUCH_DFUNC",
        None,
    );
    assert_err_reason(
        SzConfigTool_listDistinctFunctions(not_json.as_ptr()),
        -5,
        "JSON parse error",
        None,
    );
    assert_err_reason(
        SzConfigTool_setDistinctFunction(
            config.as_ptr(),
            missing.as_ptr(),
            connect.as_ptr(),
            null,
            null,
        ),
        -5,
        "NO_SUCH_DFUNC",
        None,
    );
}

#[test]
fn test_distinct_function_boundaries() {
    // NULL optional args are allowed (exercised by the lifecycle test).
    let args = [
        req("config_json", "{}"),
        req("dfunc_code", "X"),
        opt("connect_str", "c"),
        opt("dfunc_desc", "d"),
        opt("language", "l"),
    ];
    probe_args(&args, &|p| {
        SzConfigTool_addDistinctFunction(p[0], p[1], p[2], p[3], p[4])
    });
    probe_args(&args, &|p| {
        SzConfigTool_setDistinctFunction(p[0], p[1], p[2], p[3], p[4])
    });
    let two = [req("config_json", "{}"), req("dfunc_code", "X")];
    probe_args(&two, &|p| SzConfigTool_deleteDistinctFunction(p[0], p[1]));
    probe_args(&two, &|p| SzConfigTool_getDistinctFunction(p[0], p[1]));
    probe_args(&two[..1], &|p| SzConfigTool_listDistinctFunctions(p[0]));
}

// ---------------------------------------------------------------------------
// Candidate / validation / scoring functions (library placeholders that
// always return NotImplemented)
// ---------------------------------------------------------------------------

type Fn1 = extern "C" fn(P) -> SzConfigTool_result;
type Fn2 = extern "C" fn(P, P) -> SzConfigTool_result;
type Fn3 = extern "C" fn(P, P, P) -> SzConfigTool_result;

struct StubFamily {
    key: &'static str,
    func: &'static str,
    add: Fn3,
    delete: Fn2,
    get: Fn2,
    list: Fn1,
    set: Fn3,
    message: &'static str,
}

fn stub_families() -> [StubFamily; 3] {
    [
        StubFamily {
            key: "rtype_code",
            func: "candidate_func",
            add: SzConfigTool_addCandidateFunction,
            delete: SzConfigTool_deleteCandidateFunction,
            get: SzConfigTool_getCandidateFunction,
            list: SzConfigTool_listCandidateFunctions,
            set: SzConfigTool_setCandidateFunction,
            message: "Candidate functions",
        },
        StubFamily {
            key: "attr_code",
            func: "validation_func",
            add: SzConfigTool_addValidationFunction,
            delete: SzConfigTool_deleteValidationFunction,
            get: SzConfigTool_getValidationFunction,
            list: SzConfigTool_listValidationFunctions,
            set: SzConfigTool_setValidationFunction,
            message: "Validation functions",
        },
        StubFamily {
            key: "rtype_code",
            func: "scoring_func",
            add: SzConfigTool_addScoringFunction,
            delete: SzConfigTool_deleteScoringFunction,
            get: SzConfigTool_getScoringFunction,
            list: SzConfigTool_listScoringFunctions,
            set: SzConfigTool_setScoringFunction,
            message: "Scoring functions",
        },
    ]
}

#[test]
fn test_placeholder_functions_report_not_implemented() {
    let config = fixture();
    let (code, func) = (cs("X"), cs("F"));
    let (c, k, f) = (config.as_ptr(), code.as_ptr(), func.as_ptr());
    for fam in stub_families() {
        let needle = format!("Not implemented: {}", fam.message);
        assert_err_reason((fam.add)(c, k, f), -5, &needle, None);
        assert_err_reason((fam.delete)(c, k), -5, &needle, None);
        assert_err_reason((fam.get)(c, k), -5, &needle, None);
        assert_err_reason((fam.list)(c), -5, &needle, None);
        assert_err_reason((fam.set)(c, k, f), -5, &needle, None);
        // The set form's function argument may be NULL.
        assert_err_reason((fam.set)(c, k, std::ptr::null()), -5, &needle, None);
    }
}

#[test]
fn test_placeholder_function_boundaries() {
    for fam in stub_families() {
        let three = [
            req("config_json", "{}"),
            req(fam.key, "X"),
            req(fam.func, "F"),
        ];
        probe_args(&three, &|p| (fam.add)(p[0], p[1], p[2]));
        let set_args = [
            req("config_json", "{}"),
            req(fam.key, "X"),
            opt(fam.func, "F"),
        ];
        probe_args(&set_args, &|p| (fam.set)(p[0], p[1], p[2]));
        probe_args(&three[..2], &|p| (fam.delete)(p[0], p[1]));
        probe_args(&three[..2], &|p| (fam.get)(p[0], p[1]));
        probe_args(&three[..1], &|p| (fam.list)(p[0]));
    }
}

// ---------------------------------------------------------------------------
// Feature elements, settings, cascade deletes
// ---------------------------------------------------------------------------

const FEATURE_CONFIG: &str = r#"{"G2_CONFIG": {
    "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
    "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
    "CFG_FBOM": []
}}"#;

#[test]
fn test_add_element_to_feature_options() {
    let config = cs(FEATURE_CONFIG);
    let (feature, element) = (cs("NAME"), cs("FULL_NAME"));
    let options = cs(r#"{"displayLevel": 3, "displayDelim": "|", "derived": "Yes"}"#);
    let out = parse(&take_ok(SzConfigTool_addElementToFeature(
        config.as_ptr(),
        feature.as_ptr(),
        element.as_ptr(),
        options.as_ptr(),
    )));
    let fbom = rows(&out, "CFG_FBOM");
    assert_eq!(fbom.len(), 1);
    assert_eq!(fbom[0]["DISPLAY_LEVEL"], json!(3));
    assert_eq!(fbom[0]["DISPLAY_DELIM"], json!("|"));
    assert_eq!(fbom[0]["DERIVED"], json!("Yes"));

    // Option keys of the wrong type are ignored (treated as absent).
    let typed = cs(r#"{"displayLevel": "3", "displayDelim": 1, "derived": true}"#);
    let out = parse(&take_ok(SzConfigTool_addElementToFeature(
        config.as_ptr(),
        feature.as_ptr(),
        element.as_ptr(),
        typed.as_ptr(),
    )));
    assert_eq!(rows(&out, "CFG_FBOM").len(), 1);
    assert_ne!(rows(&out, "CFG_FBOM")[0]["DISPLAY_LEVEL"], json!(3));
}

#[test]
fn test_add_element_to_feature_errors() {
    let config = cs(FEATURE_CONFIG);
    let (feature, element) = (cs("NAME"), cs("FULL_NAME"));
    let bad_json = cs("{not json");
    assert_err_reason(
        SzConfigTool_addElementToFeature(
            config.as_ptr(),
            feature.as_ptr(),
            element.as_ptr(),
            bad_json.as_ptr(),
        ),
        -3,
        "Invalid JSON in options_json",
        None,
    );
    let missing = cs("NO_SUCH_FEATURE");
    assert_err_reason(
        SzConfigTool_addElementToFeature(
            config.as_ptr(),
            missing.as_ptr(),
            element.as_ptr(),
            std::ptr::null(),
        ),
        -2,
        "NO_SUCH_FEATURE",
        Some("NOT_FOUND"),
    );
    let args = [
        req("config_json", "{}"),
        req("feature_code", "NAME"),
        req("element_code", "FULL_NAME"),
        opt("options_json", "{}"),
    ];
    probe_args(&args, &|p| {
        SzConfigTool_addElementToFeature(p[0], p[1], p[2], p[3])
    });
}

#[test]
fn test_delete_element_from_feature_errors() {
    let config = cs(FEATURE_CONFIG);
    let (feature, element) = (cs("NAME"), cs("FULL_NAME"));
    // Not mapped yet: a library error with its reason code.
    assert_err_reason(
        SzConfigTool_deleteElementFromFeature(config.as_ptr(), feature.as_ptr(), element.as_ptr()),
        -2,
        "Feature element mapping not found: FTYPE_ID=1, FELEM_ID=2",
        Some("NOT_FOUND"),
    );
    let args = [
        req("config_json", "{}"),
        req("feature_code", "NAME"),
        req("element_code", "FULL_NAME"),
    ];
    probe_args(&args, &|p| {
        SzConfigTool_deleteElementFromFeature(p[0], p[1], p[2])
    });
}

#[test]
fn test_set_setting_errors() {
    let config = cs(r#"{"G2_CONFIG": {}}"#);
    let name = cs("bad");
    let value = cs("not json");
    assert_err_reason(
        SzConfigTool_setSetting(config.as_ptr(), name.as_ptr(), value.as_ptr()),
        -2,
        "Invalid input: value is not valid JSON",
        Some("INVALID_INPUT"),
    );
    let not_json = cs("not json");
    let one = cs("1");
    assert_err_reason(
        SzConfigTool_setSetting(not_json.as_ptr(), name.as_ptr(), one.as_ptr()),
        -2,
        "JSON parse error",
        Some("JSON_PARSE"),
    );
    let args = [
        req("config_json", "{}"),
        req("name", "N"),
        req("value", "1"),
    ];
    probe_args(&args, &|p| SzConfigTool_setSetting(p[0], p[1], p[2]));
}

/// (wrapper, code arg name, function section, code column, call section,
/// function id column, a function code the fixture's calls reference)
type CascadeCase = (
    Fn2,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

fn cascade_cases() -> [CascadeCase; 3] {
    [
        (
            SzConfigTool_deleteComparisonFunctionCascade,
            "cfunc_code",
            "CFG_CFUNC",
            "CFUNC_CODE",
            "CFG_CFCALL",
            "CFUNC_ID",
            "GNR_COMP",
        ),
        (
            SzConfigTool_deleteExpressionFunctionCascade,
            "efunc_code",
            "CFG_EFUNC",
            "EFUNC_CODE",
            "CFG_EFCALL",
            "EFUNC_ID",
            "PHONE_HASHER",
        ),
        (
            SzConfigTool_deleteStandardizeFunctionCascade,
            "sfunc_code",
            "CFG_SFUNC",
            "SFUNC_CODE",
            "CFG_SFCALL",
            "SFUNC_ID",
            "PARSE_NAME",
        ),
    ]
}

#[test]
fn test_cascade_deletes() {
    let config = fixture();
    let before = parse(config.to_str().unwrap());
    let missing = cs("NO_SUCH_FUNC");
    let not_json = cs("not json");
    for (call, arg, section, code_col, call_section, id_col, used) in cascade_cases() {
        let id = find_row(&before, section, code_col, used).unwrap()[id_col].clone();
        assert!(rows(&before, call_section).iter().any(|r| r[id_col] == id));
        let code = cs(used);
        let out = parse(&take_ok(call(config.as_ptr(), code.as_ptr())));
        assert!(
            find_row(&out, section, code_col, used).is_none(),
            "{section}"
        );
        assert!(
            !rows(&out, call_section).iter().any(|r| r[id_col] == id),
            "{call_section} rows of {used} must be cascaded"
        );

        assert_err_reason(
            call(config.as_ptr(), missing.as_ptr()),
            -2,
            "NO_SUCH_FUNC",
            Some("NOT_FOUND"),
        );
        assert_err_reason(
            call(not_json.as_ptr(), code.as_ptr()),
            -2,
            "JSON parse error",
            Some("JSON_PARSE"),
        );
        let args = [req("config_json", "{}"), req(arg, "X")];
        probe_args(&args, &|p| call(p[0], p[1]));
    }
}

// ---------------------------------------------------------------------------
// By-feature call gets
// ---------------------------------------------------------------------------

/// (wrapper, a fixture feature with exactly one such call, call id column)
fn by_feature_cases() -> [(Fn2, &'static str, &'static str); 4] {
    [
        (SzConfigTool_getComparisonCallByFeature, "DOB", "CFCALL_ID"),
        (SzConfigTool_getDistinctCallByFeature, "NAME", "DFCALL_ID"),
        (
            SzConfigTool_getStandardizeCallByFeature,
            "PHONE",
            "SFCALL_ID",
        ),
        (
            SzConfigTool_getExpressionCallByFeature,
            "PHONE",
            "EFCALL_ID",
        ),
    ]
}

#[test]
fn test_get_call_by_feature() {
    let config = fixture();
    let fixture_v = parse(config.to_str().unwrap());
    let ftype_of = |code: &str| {
        find_row(&fixture_v, "CFG_FTYPE", "FTYPE_CODE", code).unwrap()["FTYPE_ID"].clone()
    };
    let missing = cs("NO_SUCH_FEATURE");
    for (call, feature, id_col) in by_feature_cases() {
        let f = cs(feature);
        let got = parse(&take_ok(call(config.as_ptr(), f.as_ptr())));
        assert!(got[id_col].is_i64(), "{id_col} in {got}");
        assert_eq!(got["FTYPE_ID"], ftype_of(feature), "{id_col}");

        // ffi_json_value! records library errors unclassified, code -2.
        assert_err_reason(
            call(config.as_ptr(), missing.as_ptr()),
            -2,
            "NO_SUCH_FEATURE",
            None,
        );
        let args = [req("config_json", "{}"), req("feature_code", "NAME")];
        probe_args(&args, &|p| call(p[0], p[1]));
    }
}

// ---------------------------------------------------------------------------
// Call-element deletes
// ---------------------------------------------------------------------------

type Fn4 = extern "C" fn(P, P, P, P) -> SzConfigTool_result;

/// Element-feature UTF-8 errors are reported with -1 (not -2).
fn call_element_args() -> [Arg; 4] {
    [
        req("config_json", "{}"),
        req("feature_code", "NAME"),
        req("element_code", "FULL_NAME"),
        Arg {
            utf8_code: -1,
            ..opt("element_feature", "NAME")
        },
    ]
}

/// (wrapper, BOM section, call id column, call feature, element on its BOM)
fn by_feature_element_cases() -> [(Fn4, &'static str, &'static str, &'static str, &'static str); 2]
{
    [
        (
            SzConfigTool_deleteComparisonCallElement,
            "CFG_CFBOM",
            "CFCALL_ID",
            "DOB",
            "DATE",
        ),
        (
            SzConfigTool_deleteDistinctCallElement,
            "CFG_DFBOM",
            "DFCALL_ID",
            "NAME",
            "FULL_NAME",
        ),
    ]
}

fn bom_has(config: &Value, section: &str, id_col: &str, id: &Value, felem: &Value) -> bool {
    rows(config, section)
        .iter()
        .any(|r| &r[id_col] == id && &r["FELEM_ID"] == felem)
}

#[test]
fn test_delete_call_element_by_feature() {
    let config = fixture();
    let before = parse(config.to_str().unwrap());
    let felem_of = |code: &str| {
        find_row(&before, "CFG_FELEM", "FELEM_CODE", code).unwrap()["FELEM_ID"].clone()
    };
    let get_by_feature = [
        SzConfigTool_getComparisonCallByFeature,
        SzConfigTool_getDistinctCallByFeature,
    ];
    for ((call, bom, id_col, feature, element), get) in
        by_feature_element_cases().into_iter().zip(get_by_feature)
    {
        let (f, e) = (cs(feature), cs(element));
        let call_id = parse(&take_ok(get(config.as_ptr(), f.as_ptr())))[id_col].clone();
        let felem = felem_of(element);
        assert!(
            bom_has(&before, bom, id_col, &call_id, &felem),
            "{bom} precondition"
        );

        // Without and with the (disambiguating) element feature.
        for elem_feature in [std::ptr::null(), f.as_ptr()] {
            let out = parse(&take_ok(call(
                config.as_ptr(),
                f.as_ptr(),
                e.as_ptr(),
                elem_feature,
            )));
            assert!(
                !bom_has(&out, bom, id_col, &call_id, &felem),
                "{bom} row removed"
            );
            assert_eq!(rows(&out, bom).len(), rows(&before, bom).len() - 1);
        }

        let missing = cs("NO_SUCH_FEATURE");
        assert_err_reason(
            call(
                config.as_ptr(),
                missing.as_ptr(),
                e.as_ptr(),
                std::ptr::null(),
            ),
            -2,
            "NO_SUCH_FEATURE",
            Some("NOT_FOUND"),
        );
        probe_args(&call_element_args(), &|p| call(p[0], p[1], p[2], p[3]));
    }
}

#[test]
fn test_delete_expression_call_element() {
    let config = fixture();
    let before = parse(config.to_str().unwrap());
    let (element, feature) = (cs("PHONE_LAST_10"), cs("PHONE"));
    let felem =
        find_row(&before, "CFG_FELEM", "FELEM_CODE", "PHONE_LAST_10").unwrap()["FELEM_ID"].clone();
    assert!(bom_has(
        &before,
        "CFG_EFBOM",
        "EFCALL_ID",
        &json!(1),
        &felem
    ));
    for elem_feature in [std::ptr::null(), feature.as_ptr()] {
        let out = parse(&take_ok(SzConfigTool_deleteExpressionCallElement(
            config.as_ptr(),
            1,
            element.as_ptr(),
            elem_feature,
        )));
        assert!(!bom_has(&out, "CFG_EFBOM", "EFCALL_ID", &json!(1), &felem));
        assert_eq!(
            rows(&out, "CFG_EFBOM").len(),
            rows(&before, "CFG_EFBOM").len() - 1
        );
    }
    assert_err_reason(
        SzConfigTool_deleteExpressionCallElement(
            config.as_ptr(),
            999_999,
            element.as_ptr(),
            std::ptr::null(),
        ),
        -2,
        "999999",
        Some("NOT_FOUND"),
    );
    let args = [
        req("config_json", "{}"),
        req("element_code", "PHONE_LAST_10"),
        Arg {
            utf8_code: -1,
            ..opt("element_feature", "PHONE")
        },
    ];
    probe_args(&args, &|p| {
        SzConfigTool_deleteExpressionCallElement(p[0], 1, p[1], p[2])
    });
}

// ---------------------------------------------------------------------------
// JSON function setters: an ANON_SUPPORT that is neither string nor null
// ---------------------------------------------------------------------------

#[test]
fn test_set_function_with_json_rejects_non_string_anon_support() {
    let config = fixture();
    let cases: [(Fn3, &str, &str, &str); 2] = [
        (
            SzConfigTool_setComparisonFunctionWithJson,
            "CFG_CFUNC",
            "CFUNC_CODE",
            "STR_COMP",
        ),
        (
            SzConfigTool_setDistinctFunctionWithJson,
            "CFG_DFUNC",
            "DFUNC_CODE",
            "FELEM_STRICT_SUBSET",
        ),
    ];
    for (call, section, code_col, code_str) in cases {
        let code = cs(code_str);
        for updates in [r#"{"ANON_SUPPORT": 5}"#, r#"{"anonSupport": ["Yes"]}"#] {
            let updates = cs(updates);
            assert_err_reason(
                call(config.as_ptr(), code.as_ptr(), updates.as_ptr()),
                -2,
                "must be a string or null",
                Some("INVALID_INPUT"),
            );
        }
        let ok = cs(r#"{"anonSupport": "No"}"#);
        let out = parse(&take_ok(call(config.as_ptr(), code.as_ptr(), ok.as_ptr())));
        let row = find_row(&out, section, code_col, code_str).unwrap();
        assert_eq!(row["ANON_SUPPORT"], json!("No"));
    }
}

// ---------------------------------------------------------------------------
// Test allocator
// ---------------------------------------------------------------------------

/// A layout that is valid by itself but whose padded form (size + PAD) is
/// not: the poison allocator must return NULL rather than panic or abort.
#[test]
fn test_poison_allocator_rejects_unpaddable_layout() {
    let layout = std::alloc::Layout::from_size_align(isize::MAX as usize - 8, 1).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    assert!(ptr.is_null());
}

/// A paddable layout far beyond any address space: the system allocator
/// fails, and the poison allocator must pass the NULL through untouched
/// (no poison write through a null pointer).
#[test]
fn test_poison_allocator_passes_through_system_failure() {
    let layout = std::alloc::Layout::from_size_align(1 << 62, 1).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    assert!(ptr.is_null());
}
