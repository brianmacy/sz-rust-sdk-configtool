//! Coverage tests for the C ABI exports in the config-section / rule /
//! function / standardize-call / threshold / fragment block of `lib.rs`.
//!
//! Every export is driven through its real implementation against the real
//! g2config template fixture: success (response + config effect), NULL and
//! invalid UTF-8 for each pointer argument, unparseable JSON parameters, and
//! library errors — asserting the return code, the response pointer, and the
//! per-thread last-error accessors.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use serde_json::Value;

use super::*;

// ============================================================================
// Helpers
// ============================================================================

fn fixture() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../tests/fixtures/g2config_template.json"
    );
    std::fs::read_to_string(path).expect("read g2config template fixture")
}

fn cs(s: &str) -> CString {
    CString::new(s).expect("no interior NUL")
}

/// A C string that is not valid UTF-8.
fn bad_utf8() -> CString {
    CString::new(vec![0xff_u8, 0xfe]).expect("no interior NUL")
}

fn opt_str(ptr: *const c_char) -> Option<String> {
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string())
}

/// Consume a result: `Ok(response)` on success (asserting the last error was
/// cleared), `Err((code, message))` on failure (asserting a NULL response and a
/// last-error code equal to the return code). Always frees the response.
fn take(res: SzConfigTool_result) -> Result<String, (i64, String)> {
    if res.returnCode == 0 {
        assert!(!res.response.is_null(), "success must carry a response");
        let s = unsafe { CStr::from_ptr(res.response) }
            .to_str()
            .unwrap()
            .to_string();
        unsafe { SzConfigTool_free(res.response) };
        assert_eq!(SzConfigTool_getLastErrorCode(), 0);
        assert!(SzConfigTool_getLastError().is_null());
        assert!(SzConfigTool_getLastErrorReasonCode().is_null());
        Ok(s)
    } else {
        assert!(res.response.is_null(), "failure must carry no response");
        assert_eq!(SzConfigTool_getLastErrorCode(), res.returnCode);
        let msg = opt_str(SzConfigTool_getLastError()).expect("error message recorded");
        Err((res.returnCode, msg))
    }
}

fn ok(res: SzConfigTool_result) -> String {
    take(res).unwrap_or_else(|e| panic!("expected success, got {e:?}"))
}

fn ok_json(res: SzConfigTool_result) -> Value {
    serde_json::from_str(&ok(res)).expect("response is JSON")
}

/// Assert a failure with `code` whose message contains `needle`.
fn fails(res: SzConfigTool_result, code: i64, needle: &str) -> String {
    let (rc, msg) = take(res).expect_err("expected failure");
    assert_eq!(rc, code, "unexpected code; message: {msg}");
    assert!(msg.contains(needle), "message {msg:?} lacks {needle:?}");
    msg
}

/// Assert a library error surfaced through `handle_result!` (code -2) with the
/// given stable reason code.
fn fails_lib(res: SzConfigTool_result, reason: &str) -> String {
    let msg = fails(res, -2, "");
    assert_eq!(
        opt_str(SzConfigTool_getLastErrorReasonCode()).as_deref(),
        Some(reason)
    );
    msg
}

fn section<'a>(config: &'a Value, name: &str) -> &'a Vec<Value> {
    config["G2_CONFIG"][name].as_array().expect("section array")
}

fn find_row<'a>(config: &'a Value, name: &str, key: &str, val: &Value) -> Option<&'a Value> {
    section(config, name).iter().find(|r| &r[key] == val)
}

/// How a pointer argument is checked at the boundary.
#[derive(Clone, Copy)]
enum Arg {
    /// Required; NULL is rejected with "<name> is null".
    Req(&'static str),
    /// Required; NULL is rejected with the combined "Required parameter is null".
    ReqCombined(&'static str),
    /// Optional; NULL is accepted, invalid UTF-8 is still rejected.
    Opt(&'static str),
}

/// For each pointer argument in turn (others valid): NULL (when required) is
/// rejected with -1 and the exact message, and invalid UTF-8 with -2.
fn check_boundaries(args: &[(Arg, &str)], call: impl Fn(&[*const c_char]) -> SzConfigTool_result) {
    let valid: Vec<CString> = args.iter().map(|(_, v)| cs(v)).collect();
    let bad = bad_utf8();
    for (i, (arg, _)) in args.iter().enumerate() {
        let mut ptrs: Vec<*const c_char> = valid.iter().map(|c| c.as_ptr()).collect();
        let (name, null_msg) = match *arg {
            Arg::Req(n) => (n, Some(format!("{n} is null"))),
            Arg::ReqCombined(n) => (n, Some("Required parameter is null".to_string())),
            Arg::Opt(n) => (n, None),
        };
        if let Some(expected) = null_msg {
            ptrs[i] = ptr::null();
            let msg = fails(call(&ptrs), -1, &expected);
            assert_eq!(msg, expected);
            assert!(SzConfigTool_getLastErrorReasonCode().is_null());
        }
        ptrs[i] = bad.as_ptr();
        fails(call(&ptrs), -2, &format!("Invalid UTF-8 in {name}"));
    }
}

const BAD_CONFIG: &str = "not json";
const CONFIG: Arg = Arg::Req("config_json");

/// A config-only export: boundaries, success through `check`, and an
/// unparseable config surfacing as `bad_code`.
fn check_config_only(
    arg: Arg,
    call: impl Fn(*const c_char) -> SzConfigTool_result,
    bad_code: i64,
    check: impl Fn(Value),
) {
    check_boundaries(&[(arg, "{}")], |p| call(p[0]));
    let config = cs(&fixture());
    check(ok_json(call(config.as_ptr())));
    let bad = cs(BAD_CONFIG);
    fails(call(bad.as_ptr()), bad_code, "");
}

// ============================================================================
// Config sections
// ============================================================================

#[test]
fn test_config_section_is_empty_results_and_missing_section() {
    let config = cs(&fixture());
    let attr = cs("CFG_ATTR");
    assert_eq!(
        ok(unsafe { SzConfigTool_configSectionIsEmpty(config.as_ptr(), attr.as_ptr()) }),
        "false"
    );
    let empty = cs(r#"{"G2_CONFIG":{"CFG_X":[]}}"#);
    let x = cs("CFG_X");
    assert_eq!(
        ok(unsafe { SzConfigTool_configSectionIsEmpty(empty.as_ptr(), x.as_ptr()) }),
        "true"
    );
    let missing = cs("CFG_NOPE");
    fails(
        unsafe { SzConfigTool_configSectionIsEmpty(config.as_ptr(), missing.as_ptr()) },
        -5,
        "Configuration section 'CFG_NOPE' not found",
    );
}

#[test]
fn test_list_config_sections() {
    check_config_only(
        Arg::ReqCombined("config_json"),
        |c| unsafe { SzConfigTool_listConfigSections(c) },
        -5,
        |v| {
            let names: Vec<&str> = v
                .as_array()
                .unwrap()
                .iter()
                .map(|n| n.as_str().unwrap())
                .collect();
            assert!(names.contains(&"CFG_ATTR") && names.contains(&"CFG_ERRULE"));
        },
    );
}

#[test]
fn test_add_config_section_field() {
    check_boundaries(
        &[
            (Arg::ReqCombined("config_json"), "{}"),
            (Arg::ReqCombined("section_name"), "CFG_DSRC"),
            (Arg::ReqCombined("field_name"), "NEW_FIELD"),
            (Arg::ReqCombined("field_value_json"), "1"),
        ],
        |p| unsafe { SzConfigTool_addConfigSectionField(p[0], p[1], p[2], p[3]) },
    );
    let config = cs(&fixture());
    let sec = cs("CFG_DSRC");
    let field = cs("NEW_FIELD");
    let value = cs(r#""dflt""#);
    let out = ok_json(unsafe {
        SzConfigTool_addConfigSectionField(
            config.as_ptr(),
            sec.as_ptr(),
            field.as_ptr(),
            value.as_ptr(),
        )
    });
    let rows = section(&out, "CFG_DSRC");
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|r| r["NEW_FIELD"] == "dflt"));

    let not_json = cs("{oops");
    fails(
        unsafe {
            SzConfigTool_addConfigSectionField(
                config.as_ptr(),
                sec.as_ptr(),
                field.as_ptr(),
                not_json.as_ptr(),
            )
        },
        -3,
        "Failed to parse field_value_json",
    );
    let missing = cs("CFG_NOPE");
    fails(
        unsafe {
            SzConfigTool_addConfigSectionField(
                config.as_ptr(),
                missing.as_ptr(),
                field.as_ptr(),
                value.as_ptr(),
            )
        },
        -5,
        "Section not found or not an array: CFG_NOPE",
    );
}

#[test]
fn test_remove_config_section_field() {
    check_boundaries(
        &[
            (Arg::ReqCombined("config_json"), "{}"),
            (Arg::ReqCombined("section_name"), "CFG_DSRC"),
            (Arg::ReqCombined("field_name"), "DSRC_DESC"),
        ],
        |p| unsafe { SzConfigTool_removeConfigSectionField(p[0], p[1], p[2]) },
    );
    let config = cs(&fixture());
    let sec = cs("CFG_DSRC");
    let field = cs("DSRC_DESC");
    let out = ok_json(unsafe {
        SzConfigTool_removeConfigSectionField(config.as_ptr(), sec.as_ptr(), field.as_ptr())
    });
    assert!(
        section(&out, "CFG_DSRC")
            .iter()
            .all(|r| r.get("DSRC_DESC").is_none())
    );
    let missing = cs("CFG_NOPE");
    fails(
        unsafe {
            SzConfigTool_removeConfigSectionField(config.as_ptr(), missing.as_ptr(), field.as_ptr())
        },
        -5,
        "Section not found or not an array: CFG_NOPE",
    );
}

// ============================================================================
// Rules
// ============================================================================

#[test]
fn test_add_rule() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("rule_json"), "{}"),
        ],
        |p| SzConfigTool_addRule(p[0], p[1]),
    );
    let config = cs(&fixture());
    let rule = cs(
        r#"{"ERRULE_ID":2000,"ERRULE_CODE":"cov_rule","RESOLVE":"Yes","RELATE":"No","RTYPE_ID":1,"QUAL_ERFRAG_CODE":"SAME_NAME","ERRULE_TIER":5}"#,
    );
    let out = ok_json(SzConfigTool_addRule(config.as_ptr(), rule.as_ptr()));
    let row = find_row(&out, "CFG_ERRULE", "ERRULE_CODE", &"COV_RULE".into()).expect("rule");
    assert_eq!(row["ERRULE_ID"], 2000);

    let not_json = cs("[");
    fails(
        SzConfigTool_addRule(config.as_ptr(), not_json.as_ptr()),
        -3,
        "Invalid JSON in rule_json",
    );
    let no_code = cs(r#"{"RESOLVE":"Yes"}"#);
    fails(
        SzConfigTool_addRule(config.as_ptr(), no_code.as_ptr()),
        -5,
        "ERRULE_CODE",
    );
}

#[test]
fn test_delete_rule() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("rule_code"), "X"),
        ],
        |p| SzConfigTool_deleteRule(p[0], p[1]),
    );
    let config = cs(&fixture());
    let code = cs("SAME_A1");
    let out = ok_json(SzConfigTool_deleteRule(config.as_ptr(), code.as_ptr()));
    assert!(find_row(&out, "CFG_ERRULE", "ERRULE_CODE", &"SAME_A1".into()).is_none());
    let unknown = cs("NO_SUCH_RULE");
    fails_lib(
        SzConfigTool_deleteRule(config.as_ptr(), unknown.as_ptr()),
        "NOT_FOUND",
    );
}

#[test]
fn test_get_rule() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("code_or_id"), "X"),
        ],
        |p| SzConfigTool_getRule(p[0], p[1]),
    );
    let config = cs(&fixture());
    let code = cs("SAME_A1");
    let rule = ok_json(SzConfigTool_getRule(config.as_ptr(), code.as_ptr()));
    let text = rule.to_string();
    assert!(text.contains("SAME_A1") && text.contains("100"), "{rule}");
    let unknown = cs("NO_SUCH_RULE");
    fails(
        SzConfigTool_getRule(config.as_ptr(), unknown.as_ptr()),
        -5,
        "NO_SUCH_RULE",
    );
}

#[test]
fn test_list_rules() {
    check_config_only(
        CONFIG,
        |c| SzConfigTool_listRules(c),
        -5,
        |v| assert_eq!(v.as_array().unwrap().len(), 38),
    );
}

#[test]
fn test_set_rule() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("rule_code"), "X"),
            (Arg::Req("rule_json"), "{}"),
        ],
        |p| SzConfigTool_setRule(p[0], p[1], p[2]),
    );
    let config = cs(&fixture());
    let code = cs("SAME_A1");

    // camelCase keys (first lookup hits).
    let camel = cs(
        r#"{"resolve":"No","relate":"Yes","rtypeId":2,"fragment":"SAME_NAME","disqualifier":null,"tier":7}"#,
    );
    let out = ok_json(SzConfigTool_setRule(
        config.as_ptr(),
        code.as_ptr(),
        camel.as_ptr(),
    ));
    let row = find_row(&out, "CFG_ERRULE", "ERRULE_CODE", &"SAME_A1".into()).unwrap();
    assert_eq!(row["RESOLVE"], "No");
    assert_eq!(row["RELATE"], "Yes");
    assert_eq!(row["ERRULE_TIER"], 7);

    // UPPER_CASE keys (the `or_else` fallbacks).
    let upper = cs(r#"{"RESOLVE":"Yes","RELATE":"No","RTYPE_ID":1,"TIER":3}"#);
    let out = ok_json(SzConfigTool_setRule(
        config.as_ptr(),
        code.as_ptr(),
        upper.as_ptr(),
    ));
    let row = find_row(&out, "CFG_ERRULE", "ERRULE_CODE", &"SAME_A1".into()).unwrap();
    assert_eq!(row["RESOLVE"], "Yes");
    assert_eq!(row["ERRULE_TIER"], 3);

    let not_json = cs("{");
    fails(
        SzConfigTool_setRule(config.as_ptr(), code.as_ptr(), not_json.as_ptr()),
        -3,
        "Invalid JSON in rule_json",
    );
    let unknown = cs("NO_SUCH_RULE");
    fails_lib(
        SzConfigTool_setRule(config.as_ptr(), unknown.as_ptr(), upper.as_ptr()),
        "NOT_FOUND",
    );
}

// ============================================================================
// Standardize / expression / comparison functions (same shape, table-driven)
// ============================================================================

type AddSetFn = fn(&[*const c_char]) -> SzConfigTool_result;
type CodeFn = extern "C" fn(*const c_char, *const c_char) -> SzConfigTool_result;
type ListFn = extern "C" fn(*const c_char) -> SzConfigTool_result;

struct Family {
    section: &'static str,
    code_key: &'static str,
    code_arg: &'static str,
    desc_arg: &'static str,
    existing: &'static str,
    existing_count: usize,
    has_anon: bool,
    /// args: config, code, connect_str, desc, language, anon_support (ignored
    /// when the family has no anon_support argument).
    add: AddSetFn,
    set: AddSetFn,
    delete: CodeFn,
    get: CodeFn,
    list: ListFn,
}

fn families() -> [Family; 3] {
    [
        Family {
            section: "CFG_SFUNC",
            code_key: "SFUNC_CODE",
            code_arg: "sfunc_code",
            desc_arg: "sfunc_desc",
            existing: "PARSE_NAME",
            existing_count: 12,
            has_anon: false,
            add: |p| SzConfigTool_addStandardizeFunction(p[0], p[1], p[2], p[3], p[4]),
            set: |p| SzConfigTool_setStandardizeFunction(p[0], p[1], p[2], p[3], p[4]),
            delete: SzConfigTool_deleteStandardizeFunction,
            get: SzConfigTool_getStandardizeFunction,
            list: SzConfigTool_listStandardizeFunctions,
        },
        Family {
            section: "CFG_EFUNC",
            code_key: "EFUNC_CODE",
            code_arg: "efunc_code",
            desc_arg: "efunc_desc",
            existing: "EXPRESS_BOM",
            existing_count: 9,
            has_anon: false,
            add: |p| SzConfigTool_addExpressionFunction(p[0], p[1], p[2], p[3], p[4]),
            set: |p| SzConfigTool_setExpressionFunction(p[0], p[1], p[2], p[3], p[4]),
            delete: SzConfigTool_deleteExpressionFunction,
            get: SzConfigTool_getExpressionFunction,
            list: SzConfigTool_listExpressionFunctions,
        },
        Family {
            section: "CFG_CFUNC",
            code_key: "CFUNC_CODE",
            code_arg: "cfunc_code",
            desc_arg: "cfunc_desc",
            existing: "STR_COMP",
            existing_count: 16,
            has_anon: true,
            add: |p| SzConfigTool_addComparisonFunction(p[0], p[1], p[2], p[3], p[4], p[5]),
            set: |p| SzConfigTool_setComparisonFunction(p[0], p[1], p[2], p[3], p[4], p[5]),
            delete: SzConfigTool_deleteComparisonFunction,
            get: SzConfigTool_getComparisonFunction,
            list: SzConfigTool_listComparisonFunctions,
        },
    ]
}

/// Pointer list for an add/set call; `None` entries become NULL.
fn ptrs(args: &[Option<&CString>]) -> Vec<*const c_char> {
    args.iter()
        .map(|a| a.map_or(ptr::null(), |c| c.as_ptr()))
        .collect()
}

fn func_row(config: &Value, fam: &Family, code: &str) -> Value {
    find_row(config, fam.section, fam.code_key, &code.into())
        .unwrap_or_else(|| panic!("{} row {code}", fam.section))
        .clone()
}

fn add_set_boundary_args(fam: &Family) -> Vec<(Arg, &'static str)> {
    let mut args = vec![
        (Arg::Req("config_json"), "{}"),
        (Arg::Req(fam.code_arg), "X"),
        (Arg::Opt("connect_str"), "c"),
        (Arg::Opt(fam.desc_arg), "d"),
        (Arg::Opt("language"), "l"),
    ];
    if fam.has_anon {
        args.push((Arg::Opt("anon_support"), "Yes"));
    }
    args
}

#[test]
fn test_function_families_add() {
    for fam in families() {
        let boundary = add_set_boundary_args(&fam);
        check_boundaries(&boundary, |p| {
            let mut v = p.to_vec();
            v.resize(6, ptr::null());
            (fam.add)(&v)
        });

        let config = cs(&fixture());
        let (code, conn, desc, lang, anon) = (
            cs("COV_FUNC"),
            cs("covConn"),
            cs("Cov desc"),
            cs("covLang"),
            cs("No"),
        );
        // All values supplied.
        let out = ok_json((fam.add)(&ptrs(&[
            Some(&config),
            Some(&code),
            Some(&conn),
            Some(&desc),
            Some(&lang),
            Some(&anon),
        ])));
        let row = func_row(&out, &fam, "COV_FUNC");
        assert_eq!(row["CONNECT_STR"], "covConn", "{}", fam.section);
        assert_eq!(row["LANGUAGE"], "covLang", "{}", fam.section);
        if fam.has_anon {
            assert_eq!(row["ANON_SUPPORT"], "No");
        }

        // NULL optionals: connect_str stored as JSON null, others default.
        let out = ok_json((fam.add)(&ptrs(&[
            Some(&config),
            Some(&code),
            None,
            None,
            None,
            None,
        ])));
        let row = func_row(&out, &fam, "COV_FUNC");
        assert!(row["CONNECT_STR"].is_null(), "{}", fam.section);
        assert!(row["LANGUAGE"].is_null(), "{}", fam.section);

        // Empty strings: connect_str stores "", the rest are treated as unset.
        let empty = cs("");
        let out = ok_json((fam.add)(&ptrs(&[
            Some(&config),
            Some(&code),
            Some(&empty),
            Some(&empty),
            Some(&empty),
            Some(&empty),
        ])));
        let row = func_row(&out, &fam, "COV_FUNC");
        assert_eq!(row["CONNECT_STR"], "", "{}", fam.section);
        assert!(row["LANGUAGE"].is_null(), "{}", fam.section);

        // Duplicate code is a library error.
        let existing = cs(fam.existing);
        fails(
            (fam.add)(&ptrs(&[
                Some(&config),
                Some(&existing),
                Some(&conn),
                None,
                None,
                None,
            ])),
            -5,
            fam.existing,
        );
    }
}

#[test]
fn test_function_families_set() {
    for fam in families() {
        let boundary = add_set_boundary_args(&fam);
        check_boundaries(&boundary, |p| {
            let mut v = p.to_vec();
            v.resize(6, ptr::null());
            (fam.set)(&v)
        });

        let config = cs(&fixture());
        let code = cs(fam.existing);
        let (conn, desc, lang, anon) = (cs("newConn"), cs("New desc"), cs("newLang"), cs("No"));
        let before = func_row(
            &serde_json::from_str(&fixture()).unwrap(),
            &fam,
            fam.existing,
        );

        let out = ok_json((fam.set)(&ptrs(&[
            Some(&config),
            Some(&code),
            Some(&conn),
            Some(&desc),
            Some(&lang),
            Some(&anon),
        ])));
        let row = func_row(&out, &fam, fam.existing);
        assert_eq!(row["CONNECT_STR"], "newConn", "{}", fam.section);
        assert_eq!(row["LANGUAGE"], "newLang", "{}", fam.section);
        if fam.has_anon {
            assert_eq!(row["ANON_SUPPORT"], "No");
        }

        // NULL connect_str leaves the stored value; empty optionals are unset.
        let empty = cs("");
        let out = ok_json((fam.set)(&ptrs(&[
            Some(&config),
            Some(&code),
            None,
            Some(&empty),
            Some(&empty),
            Some(&empty),
        ])));
        let row = func_row(&out, &fam, fam.existing);
        assert_eq!(row["CONNECT_STR"], before["CONNECT_STR"], "{}", fam.section);
        assert_eq!(row["LANGUAGE"], before["LANGUAGE"], "{}", fam.section);

        let unknown = cs("NO_SUCH_FUNC");
        fails(
            (fam.set)(&ptrs(&[
                Some(&config),
                Some(&unknown),
                Some(&conn),
                None,
                None,
                None,
            ])),
            -5,
            "NO_SUCH_FUNC",
        );
    }
}

#[test]
fn test_function_families_delete_get_list() {
    for fam in families() {
        for call in [fam.delete, fam.get] {
            check_boundaries(
                &[
                    (Arg::Req("config_json"), "{}"),
                    (Arg::Req(fam.code_arg), "X"),
                ],
                |p| call(p[0], p[1]),
            );
        }
        let config_str = fixture();
        let config = cs(&config_str);
        let unknown = cs("NO_SUCH_FUNC");

        // get
        let existing = cs(fam.existing);
        let got = ok_json((fam.get)(config.as_ptr(), existing.as_ptr()));
        assert!(got.to_string().contains(fam.existing), "{got}");
        fails(
            (fam.get)(config.as_ptr(), unknown.as_ptr()),
            -5,
            "NO_SUCH_FUNC",
        );

        // delete a freshly added (unreferenced) function
        let code = cs("COV_DEL");
        let conn = cs("c");
        let added = ok((fam.add)(&ptrs(&[
            Some(&config),
            Some(&code),
            Some(&conn),
            None,
            None,
            None,
        ])));
        let added = cs(&added);
        let out = ok_json((fam.delete)(added.as_ptr(), code.as_ptr()));
        assert!(find_row(&out, fam.section, fam.code_key, &"COV_DEL".into()).is_none());
        fails(
            (fam.delete)(config.as_ptr(), unknown.as_ptr()),
            -5,
            "NO_SUCH_FUNC",
        );

        // list
        let n = fam.existing_count;
        check_config_only(
            CONFIG,
            |c| (fam.list)(c),
            -5,
            |v| assert_eq!(v.as_array().unwrap().len(), n),
        );
    }
}

// ============================================================================
// Standardize calls
// ============================================================================

#[test]
fn test_add_standardize_call() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Opt("ftype_code"), "NAME"),
            (Arg::Opt("felem_code"), ""),
            (Arg::Req("sfunc_code"), "PARSE_NAME"),
        ],
        |p| SzConfigTool_addStandardizeCall(p[0], p[1], p[2], -1, p[3]),
    );
    let config = cs(&fixture());
    let (feature, element, sfunc, empty) = (cs("NAME"), cs("FULL_NAME"), cs("PARSE_NAME"), cs(""));

    // Feature-scoped, auto exec order (negative = None).
    let out = ok_json(SzConfigTool_addStandardizeCall(
        config.as_ptr(),
        feature.as_ptr(),
        ptr::null(),
        -1,
        sfunc.as_ptr(),
    ));
    assert!(find_row(&out, "CFG_SFCALL", "SFCALL_ID", &1000.into()).is_some());

    // Element-scoped, explicit exec order; empty feature treated as unset.
    let out = ok_json(SzConfigTool_addStandardizeCall(
        config.as_ptr(),
        empty.as_ptr(),
        element.as_ptr(),
        77,
        sfunc.as_ptr(),
    ));
    let row = find_row(&out, "CFG_SFCALL", "SFCALL_ID", &1000.into()).unwrap();
    assert_eq!(row["EXEC_ORDER"], 77);
    assert_eq!(row["FTYPE_ID"], -1);

    // Neither feature nor element: empty element also treated as unset.
    fails(
        SzConfigTool_addStandardizeCall(
            config.as_ptr(),
            ptr::null(),
            empty.as_ptr(),
            -1,
            sfunc.as_ptr(),
        ),
        -5,
        "Either a feature or an element must be specified",
    );
}

#[test]
fn test_delete_get_standardize_call() {
    check_boundaries(&[(Arg::Req("config_json"), "{}")], |p| {
        SzConfigTool_deleteStandardizeCall(p[0], 1)
    });
    check_boundaries(&[(Arg::Req("config_json"), "{}")], |p| {
        SzConfigTool_getStandardizeCall(p[0], 1)
    });
    let config = cs(&fixture());

    let got = ok_json(SzConfigTool_getStandardizeCall(config.as_ptr(), 1));
    assert_eq!(got["SFCALL_ID"], 1, "{got}");
    fails(
        SzConfigTool_getStandardizeCall(config.as_ptr(), 999_999),
        -5,
        "999999",
    );

    let out = ok_json(SzConfigTool_deleteStandardizeCall(config.as_ptr(), 1));
    assert!(find_row(&out, "CFG_SFCALL", "SFCALL_ID", &1.into()).is_none());
    fails_lib(
        SzConfigTool_deleteStandardizeCall(config.as_ptr(), 999_999),
        "NOT_FOUND",
    );
}

#[test]
fn test_list_standardize_calls() {
    check_config_only(
        CONFIG,
        |c| SzConfigTool_listStandardizeCalls(c),
        -5,
        |v| assert_eq!(v.as_array().unwrap().len(), 24),
    );
}

// ============================================================================
// Comparison thresholds
// ============================================================================

#[test]
fn test_add_comparison_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("cfunc_rtnval"), "X"),
        ],
        |p| SzConfigTool_addComparisonThreshold(p[0], 1, p[1], -1, -1, -1, -1, -1, -1, -1),
    );
    let config = cs(&fixture());

    // All optionals negative -> None.
    let rtn = cs("cov_none");
    let out = ok_json(SzConfigTool_addComparisonThreshold(
        config.as_ptr(),
        1,
        rtn.as_ptr(),
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
        -1,
    ));
    let row = find_row(&out, "CFG_CFRTN", "CFUNC_RTNVAL", &"COV_NONE".into()).unwrap();
    assert_eq!(row["FTYPE_ID"], 0);
    assert!(row["SAME_SCORE"].is_null());

    // All optionals supplied.
    let rtn = cs("cov_all");
    let out = ok_json(SzConfigTool_addComparisonThreshold(
        config.as_ptr(),
        1,
        rtn.as_ptr(),
        1,
        9,
        95,
        85,
        75,
        65,
        55,
    ));
    let row = find_row(&out, "CFG_CFRTN", "CFUNC_RTNVAL", &"COV_ALL".into()).unwrap();
    assert_eq!(row["FTYPE_ID"], 1);
    assert_eq!(row["SAME_SCORE"], 95);
    assert_eq!(row["UN_LIKELY_SCORE"], 55);

    // Duplicate (CFUNC_ID 1, all features, FULL_SCORE).
    let dup = cs("FULL_SCORE");
    fails_lib(
        SzConfigTool_addComparisonThreshold(
            config.as_ptr(),
            1,
            dup.as_ptr(),
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
        ),
        "ALREADY_EXISTS",
    );
}

#[test]
fn test_delete_comparison_threshold() {
    check_boundaries(&[(Arg::Req("config_json"), "{}")], |p| {
        SzConfigTool_deleteComparisonThreshold(p[0], 1)
    });
    let config = cs(&fixture());
    let out = ok_json(SzConfigTool_deleteComparisonThreshold(config.as_ptr(), 1));
    assert!(find_row(&out, "CFG_CFRTN", "CFRTN_ID", &1.into()).is_none());
    fails_lib(
        SzConfigTool_deleteComparisonThreshold(config.as_ptr(), 999_999),
        "NOT_FOUND",
    );
}

#[test]
fn test_set_comparison_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("updates_json"), "{}"),
        ],
        |p| SzConfigTool_setComparisonThreshold(p[0], 1, p[1]),
    );
    let config = cs(&fixture());
    let updates = cs(
        r#"{"sameScore":99,"closeScore":89,"likelyScore":79,"plausibleScore":69,"unlikelyScore":59}"#,
    );
    let out = ok_json(SzConfigTool_setComparisonThreshold(
        config.as_ptr(),
        1,
        updates.as_ptr(),
    ));
    let row = find_row(&out, "CFG_CFRTN", "CFRTN_ID", &1.into()).unwrap();
    assert_eq!(row["SAME_SCORE"], 99);
    assert_eq!(row["CLOSE_SCORE"], 89);
    assert_eq!(row["LIKELY_SCORE"], 79);
    assert_eq!(row["PLAUSIBLE_SCORE"], 69);
    assert_eq!(row["UN_LIKELY_SCORE"], 59);

    let not_json = cs("{");
    fails(
        SzConfigTool_setComparisonThreshold(config.as_ptr(), 1, not_json.as_ptr()),
        -3,
        "Invalid JSON in updates_json",
    );
    fails_lib(
        SzConfigTool_setComparisonThreshold(config.as_ptr(), 999_999, updates.as_ptr()),
        "NOT_FOUND",
    );
}

#[test]
fn test_list_comparison_thresholds() {
    check_config_only(
        CONFIG,
        |c| SzConfigTool_listComparisonThresholds(c),
        -5,
        |v| assert_eq!(v.as_array().unwrap().len(), 24),
    );
}

// ============================================================================
// Generic thresholds
// ============================================================================

fn generic_rows(config: &Value, gplan: i64, behavior: &str, ftype: i64) -> usize {
    section(config, "CFG_GENERIC_THRESHOLD")
        .iter()
        .filter(|r| r["GPLAN_ID"] == gplan && r["BEHAVIOR"] == behavior && r["FTYPE_ID"] == ftype)
        .count()
}

#[test]
fn test_add_generic_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("plan"), "INGEST"),
            (Arg::Req("behavior"), "NAME"),
            (Arg::Req("send_to_redo"), "Yes"),
            (Arg::Opt("feature"), "NAME"),
        ],
        |p| SzConfigTool_addGenericThreshold(p[0], p[1], p[2], 10, 10, p[3], p[4]),
    );
    let config = cs(&fixture());
    let (plan, behavior, redo) = (cs("INGEST"), cs("FM"), cs("No"));

    // Feature-scoped (NAME = FTYPE_ID 1); the template has no such row.
    let feature = cs("NAME");
    let out = ok_json(SzConfigTool_addGenericThreshold(
        config.as_ptr(),
        plan.as_ptr(),
        behavior.as_ptr(),
        7,
        8,
        redo.as_ptr(),
        feature.as_ptr(),
    ));
    assert_eq!(generic_rows(&out, 1, "FM", 1), 1);

    // NULL feature = ALL; (INGEST, NAME, ALL) already exists -> library error.
    let name = cs("NAME");
    fails_lib(
        SzConfigTool_addGenericThreshold(
            config.as_ptr(),
            plan.as_ptr(),
            name.as_ptr(),
            7,
            8,
            redo.as_ptr(),
            ptr::null(),
        ),
        "ALREADY_EXISTS",
    );
}

#[test]
fn test_validate_generic_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("plan"), "INGEST"),
            (Arg::Req("behavior"), "NAME"),
            (Arg::Req("send_to_redo"), "Yes"),
            (Arg::Opt("feature"), "NAME"),
        ],
        |p| SzConfigTool_validateGenericThreshold(p[0], p[1], p[2], p[3], p[4]),
    );
    let config = cs(&fixture());
    let (plan, behavior, redo, feature) = (cs("INGEST"), cs("NAME"), cs("Yes"), cs("NAME"));
    for feat in [ptr::null(), feature.as_ptr()] {
        let check = ok_json(SzConfigTool_validateGenericThreshold(
            config.as_ptr(),
            plan.as_ptr(),
            behavior.as_ptr(),
            redo.as_ptr(),
            feat,
        ));
        assert_eq!(
            check["schema"], "sz-configtool.generic-threshold-check/v1",
            "{check}"
        );
    }
    let bad = cs(BAD_CONFIG);
    fails_lib(
        SzConfigTool_validateGenericThreshold(
            bad.as_ptr(),
            plan.as_ptr(),
            behavior.as_ptr(),
            redo.as_ptr(),
            ptr::null(),
        ),
        "JSON_PARSE",
    );
}

#[test]
fn test_delete_generic_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("plan"), "INGEST"),
            (Arg::Req("behavior"), "NAME"),
            (Arg::Opt("feature"), "NAME"),
        ],
        |p| SzConfigTool_deleteGenericThreshold(p[0], p[1], p[2], p[3]),
    );
    let config = cs(&fixture());
    let (plan, behavior) = (cs("INGEST"), cs("NAME"));
    let out = ok_json(SzConfigTool_deleteGenericThreshold(
        config.as_ptr(),
        plan.as_ptr(),
        behavior.as_ptr(),
        ptr::null(),
    ));
    assert_eq!(generic_rows(&out, 1, "NAME", 0), 0);

    let feature = cs("NAME");
    fails_lib(
        SzConfigTool_deleteGenericThreshold(
            config.as_ptr(),
            plan.as_ptr(),
            behavior.as_ptr(),
            feature.as_ptr(),
        ),
        "NOT_FOUND",
    );
}

#[test]
fn test_set_generic_threshold() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("behavior"), "NAME"),
            (Arg::Req("updates_json"), "{}"),
        ],
        |p| SzConfigTool_setGenericThreshold(p[0], 1, p[1], p[2]),
    );
    let config = cs(&fixture());
    let behavior = cs("NAME");
    let updates = cs(r#"{"candidateCap":42}"#);
    let out = ok_json(SzConfigTool_setGenericThreshold(
        config.as_ptr(),
        1,
        behavior.as_ptr(),
        updates.as_ptr(),
    ));
    let row = section(&out, "CFG_GENERIC_THRESHOLD")
        .iter()
        .find(|r| r["GPLAN_ID"] == 1 && r["BEHAVIOR"] == "NAME" && r["FTYPE_ID"] == 0)
        .unwrap();
    assert_eq!(row["CANDIDATE_CAP"], 42);

    let not_json = cs("{");
    fails(
        SzConfigTool_setGenericThreshold(config.as_ptr(), 1, behavior.as_ptr(), not_json.as_ptr()),
        -3,
        "Invalid JSON in updates_json",
    );
    fails(
        SzConfigTool_setGenericThreshold(config.as_ptr(), 999, behavior.as_ptr(), updates.as_ptr()),
        -4,
        "999",
    );
    let wrong_type = cs(r#"{"candidateCap":"500"}"#);
    fails_lib(
        SzConfigTool_setGenericThreshold(
            config.as_ptr(),
            1,
            behavior.as_ptr(),
            wrong_type.as_ptr(),
        ),
        "INVALID_INPUT",
    );
}

#[test]
fn test_list_generic_thresholds() {
    check_config_only(
        CONFIG,
        |c| SzConfigTool_listGenericThresholds(c),
        -5,
        |v| assert_eq!(v.as_array().unwrap().len(), 46),
    );
}

// ============================================================================
// Fragments
// ============================================================================

#[test]
fn test_get_fragment() {
    check_boundaries(
        &[
            (Arg::Req("config_json"), "{}"),
            (Arg::Req("code_or_id"), "X"),
        ],
        |p| SzConfigTool_getFragment(p[0], p[1]),
    );
    let config = cs(&fixture());
    let code = cs("SAME_NAME");
    let frag = ok_json(SzConfigTool_getFragment(config.as_ptr(), code.as_ptr()));
    assert!(frag.to_string().contains("SAME_NAME"), "{frag}");
    let unknown = cs("NO_SUCH_FRAG");
    fails(
        SzConfigTool_getFragment(config.as_ptr(), unknown.as_ptr()),
        -5,
        "NO_SUCH_FRAG",
    );
}
