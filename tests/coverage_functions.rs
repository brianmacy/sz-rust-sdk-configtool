//! Coverage of `sz_configtool_lib::functions` (CFG_SFUNC / CFG_EFUNC / CFG_CFUNC /
//! CFG_DFUNC operations and the CFG_RTYPE / CFG_ATTR placeholder modules),
//! exercised against the real template fixture or minimal inline configs.
//! Every error path asserts the exact `SzErrorKind` and message.

mod cov_support;

use cov_support::{BAD_JSON, assert_err, assert_kind, parse, rows, template};
use serde_json::{Value, json};
use sz_configtool_lib::error::SzErrorKind;
use sz_configtool_lib::helpers::FieldUpdate;

/// Generates the shared add/delete/get/list/set test suite for one function
/// family. All four families share the same shape: `{Label} function ...`
/// messages, a `CFG_?FUNC` section keyed by `?FUNC_CODE`, and params structs
/// carrying `connect_str` / `description` / `language`.
macro_rules! function_family {
    (
        $modname:ident, $fam:ident,
        add = $add:ident, del = $del:ident, get = $get:ident,
        list = $list:ident, set = $set:ident,
        add_params = $ap:ident, set_params = $sp:ident,
        section = $sec:literal, code = $codef:literal, id = $idf:literal,
        desc = $descf:literal, label = $label:literal, existing = $existing:literal
    ) => {
        mod $modname {
            use super::*;
            use sz_configtool_lib::functions::$fam::{$ap, $sp, $add, $del, $get, $list, $set};

            fn numeric_code_config() -> String {
                // A row whose code is a JSON number: the lookup helper matches it
                // numerically but the delete helper only matches strings, so the
                // post-lookup delete step reports NotFound.
                json!({"G2_CONFIG": {$sec: [{$idf: 5, $codef: 5}]}}).to_string()
            }

            #[test]
            fn add_rejects_bad_json() {
                assert_kind($add(BAD_JSON, "X", $ap::default()), SzErrorKind::JsonParse);
            }

            #[test]
            fn add_rejects_existing_code() {
                assert_err(
                    $add(&template(), &$existing.to_lowercase(), $ap::default()),
                    SzErrorKind::InvalidInput,
                    &format!("{} function already exists: {}", $label, $existing),
                );
            }

            #[test]
            fn add_rejects_missing_section() {
                assert_err(
                    $add(r#"{"G2_CONFIG": {}}"#, "X", $ap::default()),
                    SzErrorKind::MissingSection,
                    &format!("Section path 'G2_CONFIG.{}' not found", $sec),
                );
            }

            #[test]
            fn add_rejects_non_array_section() {
                let config = json!({"G2_CONFIG": {$sec: {}}}).to_string();
                assert_err(
                    $add(&config, "X", $ap::default()),
                    SzErrorKind::MissingSection,
                    $sec,
                );
            }

            #[test]
            fn add_writes_complete_row() {
                let mut params = $ap::default();
                params.connect_str = Some("conn");
                params.description = Some("desc");
                params.language = Some("lang");
                let (out, record) = $add(&template(), "new_fn", params).unwrap();
                assert_eq!(record[$codef], "NEW_FN");
                assert_eq!(record["CONNECT_STR"], "conn");
                assert_eq!(record[$descf], "desc");
                assert_eq!(record["LANGUAGE"], "lang");
                assert!(rows(&out, $sec).contains(&record));
            }

            #[test]
            fn add_params_try_from_json() {
                let full = json!({"connectStr": "c", "description": "d", "language": "l"});
                let p = $ap::try_from(&full).unwrap();
                assert_eq!(
                    (p.connect_str, p.description, p.language),
                    (Some("c"), Some("d"), Some("l"))
                );
                let empty = json!({"connectStr": null});
                let p = $ap::try_from(&empty).unwrap();
                assert_eq!((p.connect_str, p.description, p.language), (None, None, None));
            }

            #[test]
            fn delete_paths() {
                assert_kind($del(BAD_JSON, "X"), SzErrorKind::JsonParse);
                assert_err(
                    $del(&template(), "nope"),
                    SzErrorKind::NotFound,
                    &format!("{} function not found: NOPE", $label),
                );
                assert_err(
                    $del(&numeric_code_config(), "5"),
                    SzErrorKind::NotFound,
                    &format!("{} '5' not found", $sec),
                );
                let (out, removed) = $del(&template(), $existing).unwrap();
                assert_eq!(removed[$codef], $existing);
                assert!(!rows(&out, $sec).iter().any(|r| r[$codef] == $existing));
            }

            #[test]
            fn get_paths() {
                assert_kind($get(BAD_JSON, "X"), SzErrorKind::JsonParse);
                assert_err(
                    $get(&template(), "nope"),
                    SzErrorKind::NotFound,
                    &format!("{} function not found: NOPE", $label),
                );
                assert_eq!($get(&template(), $existing).unwrap()[$codef], $existing);
            }

            #[test]
            fn list_paths() {
                assert_kind($list(BAD_JSON), SzErrorKind::JsonParse);
                assert!($list(r#"{"G2_CONFIG": {}}"#).unwrap().is_empty());
                assert!($list(r#"{}"#).unwrap().is_empty());
                let config = json!({"G2_CONFIG": {$sec: {}}}).to_string();
                assert!($list(&config).unwrap().is_empty());
                let items = $list(&template()).unwrap();
                assert_eq!(items.len(), rows(&template(), $sec).len());
                assert!(items.iter().any(|i| i["function"] == $existing));
                // A row without a code projects an empty function name.
                let config = json!({"G2_CONFIG": {$sec: [{$idf: 9}]}}).to_string();
                assert_eq!($list(&config).unwrap()[0]["function"], "");
            }

            #[test]
            fn set_rejects_missing_and_undeletable() {
                assert_kind($set(BAD_JSON, "X", $sp::default()), SzErrorKind::JsonParse);
                assert_err(
                    $set(&template(), "nope", $sp::default()),
                    SzErrorKind::NotFound,
                    &format!("{} function not found: NOPE", $label),
                );
                assert_err(
                    $set(&numeric_code_config(), "5", $sp::default()),
                    SzErrorKind::NotFound,
                    &format!("{} '5' not found", $sec),
                );
            }

            #[test]
            fn set_updates_fields() {
                let mut params = $sp::default();
                params.connect_str = FieldUpdate::Set("new_conn");
                params.description = Some("new desc");
                params.language = Some("new lang");
                let (out, rec) = $set(&template(), $existing, params).unwrap();
                assert_eq!(rec["CONNECT_STR"], "new_conn");
                assert_eq!(rec[$descf], "new desc");
                assert_eq!(rec["LANGUAGE"], "new lang");
                assert!(rows(&out, $sec).contains(&rec));

                let clear = $sp {
                    connect_str: FieldUpdate::Clear,
                    ..Default::default()
                };
                let (_, rec) = $set(&out, $existing, clear).unwrap();
                assert_eq!(rec["CONNECT_STR"], Value::Null);

                let (_, rec) = $set(&out, $existing, $sp::default()).unwrap();
                assert_eq!(rec["CONNECT_STR"], "new_conn", "Leave keeps the value");
            }
        }
    };
}

function_family!(
    standardize_family,
    standardize,
    add = add_standardize_function,
    del = delete_standardize_function,
    get = get_standardize_function,
    list = list_standardize_functions,
    set = set_standardize_function,
    add_params = AddStandardizeFunctionParams,
    set_params = SetStandardizeFunctionParams,
    section = "CFG_SFUNC",
    code = "SFUNC_CODE",
    id = "SFUNC_ID",
    desc = "SFUNC_DESC",
    label = "Standardize",
    existing = "PARSE_NAME"
);

function_family!(
    expression_family,
    expression,
    add = add_expression_function,
    del = delete_expression_function,
    get = get_expression_function,
    list = list_expression_functions,
    set = set_expression_function,
    add_params = AddExpressionFunctionParams,
    set_params = SetExpressionFunctionParams,
    section = "CFG_EFUNC",
    code = "EFUNC_CODE",
    id = "EFUNC_ID",
    desc = "EFUNC_DESC",
    label = "Expression",
    existing = "EXPRESS_BOM"
);

function_family!(
    comparison_family,
    comparison,
    add = add_comparison_function,
    del = delete_comparison_function,
    get = get_comparison_function,
    list = list_comparison_functions,
    set = set_comparison_function,
    add_params = AddComparisonFunctionParams,
    set_params = SetComparisonFunctionParams,
    section = "CFG_CFUNC",
    code = "CFUNC_CODE",
    id = "CFUNC_ID",
    desc = "CFUNC_DESC",
    label = "Comparison",
    existing = "STR_COMP"
);

function_family!(
    distinct_family,
    distinct,
    add = add_distinct_function,
    del = delete_distinct_function,
    get = get_distinct_function,
    list = list_distinct_functions,
    set = set_distinct_function,
    add_params = AddDistinctFunctionParams,
    set_params = SetDistinctFunctionParams,
    section = "CFG_DFUNC",
    code = "DFUNC_CODE",
    id = "DFUNC_ID",
    desc = "DFUNC_DESC",
    label = "Distinct",
    existing = "FELEM_STRICT_SUBSET"
);

// ---------------------------------------------------------------------------
// ANON_SUPPORT (comparison + distinct only)
// ---------------------------------------------------------------------------

mod anon_support {
    use super::*;
    use sz_configtool_lib::functions::comparison::{
        AddComparisonFunctionParams, SetComparisonFunctionParams, add_comparison_function,
        set_comparison_function,
    };
    use sz_configtool_lib::functions::distinct::{
        AddDistinctFunctionParams, SetDistinctFunctionParams, add_distinct_function,
        set_distinct_function,
    };

    #[test]
    fn comparison_anon_support_domain() {
        for (input, stored) in [("yes", "Yes"), ("NO", "No")] {
            let params = AddComparisonFunctionParams {
                anon_support: Some(input),
                ..Default::default()
            };
            let (_, rec) = add_comparison_function(&template(), "CMP_ANON", params).unwrap();
            assert_eq!(rec["ANON_SUPPORT"], stored);
        }
        let (_, rec) = add_comparison_function(&template(), "CMP_DEF", Default::default()).unwrap();
        assert_eq!(rec["ANON_SUPPORT"], "No");
        let bad = AddComparisonFunctionParams {
            anon_support: Some("maybe"),
            ..Default::default()
        };
        assert_err(
            add_comparison_function(&template(), "CMP_BAD", bad),
            SzErrorKind::InvalidInput,
            "Invalid ANON_SUPPORT value 'maybe'. Must be 'Yes' or 'No'",
        );
        let set = SetComparisonFunctionParams {
            anon_support: Some("No"),
            ..Default::default()
        };
        let (_, rec) = set_comparison_function(&template(), "STR_COMP", set).unwrap();
        assert_eq!(rec["ANON_SUPPORT"], "No");
        let full = json!({"anonSupport": "Yes"});
        let p = AddComparisonFunctionParams::try_from(&full).unwrap();
        assert_eq!(p.anon_support, Some("Yes"));
    }

    #[test]
    fn distinct_anon_support_domain() {
        for (input, stored) in [("yes", "Yes"), ("NO", "No")] {
            let params = AddDistinctFunctionParams {
                anon_support: Some(input),
                ..Default::default()
            };
            let (_, rec) = add_distinct_function(&template(), "DF_ANON", params).unwrap();
            assert_eq!(rec["ANON_SUPPORT"], stored);
        }
        let bad = AddDistinctFunctionParams {
            anon_support: Some("maybe"),
            ..Default::default()
        };
        assert_err(
            add_distinct_function(&template(), "DF_BAD", bad),
            SzErrorKind::InvalidInput,
            "Invalid ANON_SUPPORT value 'maybe'. Must be 'Yes' or 'No'",
        );
        let set = SetDistinctFunctionParams {
            anon_support: Some("No"),
            ..Default::default()
        };
        let (_, rec) = set_distinct_function(&template(), "FELEM_STRICT_SUBSET", set).unwrap();
        assert_eq!(rec["ANON_SUPPORT"], "No");
        let full = json!({"anonSupport": "Yes"});
        let p = AddDistinctFunctionParams::try_from(&full).unwrap();
        assert_eq!(p.anon_support, Some("Yes"));
    }
}

// ---------------------------------------------------------------------------
// Cascade deletes
// ---------------------------------------------------------------------------

mod cascade {
    use super::*;
    use sz_configtool_lib::functions::comparison::delete_comparison_function_cascade;
    use sz_configtool_lib::functions::expression::delete_expression_function_cascade;
    use sz_configtool_lib::functions::standardize::delete_standardize_function_cascade;

    type Cascade = fn(&str, &str) -> sz_configtool_lib::Result<(String, Value)>;

    /// (cascade fn, section, code field, id field, label)
    const FAMILIES: [(Cascade, &str, &str, &str, &str); 3] = [
        (
            delete_standardize_function_cascade,
            "CFG_SFUNC",
            "SFUNC_CODE",
            "SFUNC_ID",
            "Standardize",
        ),
        (
            delete_expression_function_cascade,
            "CFG_EFUNC",
            "EFUNC_CODE",
            "EFUNC_ID",
            "Expression",
        ),
        (
            delete_comparison_function_cascade,
            "CFG_CFUNC",
            "CFUNC_CODE",
            "CFUNC_ID",
            "Comparison",
        ),
    ];

    #[test]
    fn cascade_error_paths() {
        for (cascade, section, code, id, label) in FAMILIES {
            assert_kind(cascade(BAD_JSON, "X"), SzErrorKind::JsonParse);
            assert_err(
                cascade(&template(), "nope"),
                SzErrorKind::NotFound,
                &format!("{label} function not found: NOPE"),
            );
            // Row with no id column -> MissingField.
            let no_id = json!({"G2_CONFIG": {section: [{code: "X"}]}}).to_string();
            assert_err(cascade(&no_id, "x"), SzErrorKind::MissingField, id);
            // Numeric code: found by lookup, but the final piece-wise delete
            // (string match only) reports NotFound.
            let numeric = json!({"G2_CONFIG": {section: [{id: 5, code: 5}]}}).to_string();
            assert_err(
                cascade(&numeric, "5"),
                SzErrorKind::NotFound,
                &format!("{section} '5' not found"),
            );
        }
    }

    #[test]
    fn standardize_cascade_removes_calls() {
        let config = json!({"G2_CONFIG": {
            "CFG_SFUNC": [{"SFUNC_ID": 1, "SFUNC_CODE": "S1"}, {"SFUNC_ID": 2, "SFUNC_CODE": "S2"}],
            "CFG_SFCALL": [{"SFCALL_ID": 1, "SFUNC_ID": 1}, {"SFCALL_ID": 2, "SFUNC_ID": 2}]
        }})
        .to_string();
        let (out, removed) = delete_standardize_function_cascade(&config, "s1").unwrap();
        assert_eq!(removed["SFUNC_ID"], 1);
        assert_eq!(
            rows(&out, "CFG_SFCALL"),
            vec![json!({"SFCALL_ID": 2, "SFUNC_ID": 2})]
        );
        assert_eq!(rows(&out, "CFG_SFUNC").len(), 1);
        // Missing CFG_SFCALL is tolerated.
        let bare = json!({"G2_CONFIG": {"CFG_SFUNC": [{"SFUNC_ID": 1, "SFUNC_CODE": "S1"}]}});
        let (out, _) = delete_standardize_function_cascade(&bare.to_string(), "S1").unwrap();
        assert!(rows(&out, "CFG_SFUNC").is_empty());
    }

    #[test]
    fn expression_cascade_removes_calls_and_boms() {
        let config = json!({"G2_CONFIG": {
            "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "E1"}, {"EFUNC_ID": 2, "EFUNC_CODE": "E2"}],
            "CFG_EFCALL": [
                {"EFCALL_ID": 10, "EFUNC_ID": 1},
                {"EFCALL_ID": 20, "EFUNC_ID": 2},
                {"EFUNC_ID": 1}
            ],
            "CFG_EFBOM": [{"EFCALL_ID": 10}, {"EFCALL_ID": 20}, {"FELEM_ID": 3}]
        }})
        .to_string();
        let (out, _) = delete_expression_function_cascade(&config, "E1").unwrap();
        assert_eq!(
            rows(&out, "CFG_EFCALL"),
            vec![json!({"EFCALL_ID": 20, "EFUNC_ID": 2})]
        );
        assert_eq!(
            rows(&out, "CFG_EFBOM"),
            vec![json!({"EFCALL_ID": 20}), json!({"FELEM_ID": 3})]
        );
        // Missing call/BOM sections are tolerated.
        let bare = json!({"G2_CONFIG": {"CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "E1"}]}});
        let (out, _) = delete_expression_function_cascade(&bare.to_string(), "E1").unwrap();
        assert!(rows(&out, "CFG_EFUNC").is_empty());
    }

    #[test]
    fn comparison_cascade_removes_all_dependents() {
        let config = json!({"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "C1"}, {"CFUNC_ID": 2, "CFUNC_CODE": "C2"}],
            "CFG_CFCALL": [{"CFCALL_ID": 10, "CFUNC_ID": 1}, {"CFCALL_ID": 20, "CFUNC_ID": 2}],
            "CFG_CFBOM": [{"CFCALL_ID": 10}, {"CFCALL_ID": 20}, {"FELEM_ID": 3}],
            "CFG_CFRTN": [
                {"CFRTN_ID": 1, "CFUNC_ID": 1, "FTYPE_ID": 0, "CFUNC_RTNVAL": "FULL"},
                {"CFRTN_ID": 2, "CFUNC_ID": 1, "FTYPE_ID": 1, "CFUNC_RTNVAL": "FULL"},
                {"CFRTN_ID": 3, "CFUNC_ID": 1, "FTYPE_ID": 1, "CFUNC_RTNVAL": "full"},
                {"CFRTN_ID": 4, "CFUNC_ID": 1, "FTYPE_ID": 99, "CFUNC_RTNVAL": "ORPHAN"},
                {"CFRTN_ID": 5, "CFUNC_ID": 1, "FTYPE_ID": 0, "CFUNC_RTNVAL": null},
                {"CFRTN_ID": 6, "CFUNC_ID": 2, "FTYPE_ID": 0, "CFUNC_RTNVAL": "KEEP"}
            ]
        }})
        .to_string();
        let (out, removed) = delete_comparison_function_cascade(&config, "c1").unwrap();
        assert_eq!(removed["CFUNC_ID"], 1);
        assert_eq!(
            rows(&out, "CFG_CFCALL"),
            vec![json!({"CFCALL_ID": 20, "CFUNC_ID": 2})]
        );
        assert_eq!(
            rows(&out, "CFG_CFBOM"),
            vec![json!({"CFCALL_ID": 20}), json!({"FELEM_ID": 3})]
        );
        let cfrtn = rows(&out, "CFG_CFRTN");
        assert_eq!(cfrtn.len(), 1);
        assert_eq!(cfrtn[0]["CFRTN_ID"], 6);
        assert_eq!(rows(&out, "CFG_CFUNC").len(), 1);

        // Missing dependent sections are tolerated.
        let bare = json!({"G2_CONFIG": {"CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "C1"}]}});
        let (out, _) = delete_comparison_function_cascade(&bare.to_string(), "C1").unwrap();
        assert!(rows(&out, "CFG_CFUNC").is_empty());
    }

    #[test]
    fn comparison_cascade_propagates_threshold_delete_error() {
        // The function is found by its numeric code, but the per-row threshold
        // delete resolves the code as a string and fails.
        let config = json!({"G2_CONFIG": {
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": 5}],
            "CFG_CFRTN": [{"CFRTN_ID": 1, "CFUNC_ID": 5, "FTYPE_ID": 0, "CFUNC_RTNVAL": "X"}]
        }})
        .to_string();
        assert_err(
            delete_comparison_function_cascade(&config, "5"),
            SzErrorKind::NotFound,
            "Comparison function '5' not found",
        );
    }

    #[test]
    fn comparison_cascade_on_template() {
        let template = template();
        let (out, _) = delete_comparison_function_cascade(&template, "STR_COMP").unwrap();
        let cfunc_id = 1;
        for section in ["CFG_CFCALL", "CFG_CFRTN"] {
            assert!(
                !rows(&out, section)
                    .iter()
                    .any(|r| r["CFUNC_ID"] == cfunc_id),
                "{section} still references STR_COMP"
            );
        }
        assert_eq!(
            parse(&out)["G2_CONFIG"]["CFG_CFUNC"]
                .as_array()
                .unwrap()
                .len()
                + 1,
            rows(&template, "CFG_CFUNC").len()
        );
    }
}

// ---------------------------------------------------------------------------
// Placeholder (not-yet-implemented) modules
// ---------------------------------------------------------------------------

mod placeholders {
    use super::*;
    use sz_configtool_lib::functions::{candidate, matching, scoring, validation};

    fn assert_stub<T: std::fmt::Debug>(r: sz_configtool_lib::Result<T>, label: &str) {
        assert_err(
            r,
            SzErrorKind::NotImplemented,
            &format!("{label} functions are not yet fully implemented"),
        );
    }

    macro_rules! stub_suite {
        ($test:ident, $m:ident, $label:literal,
         $add:ident, $del:ident, $get:ident, $list:ident, $set:ident, $remove:ident) => {
            #[test]
            fn $test() {
                let c = template();
                assert_stub($m::$add(&c, "X", "F"), $label);
                assert_stub($m::$del(&c, "X"), $label);
                assert_stub($m::$get(&c, "X"), $label);
                assert_stub($m::$list(&c), $label);
                assert_stub($m::$set(&c, "X", Some("F")), $label);
                assert_stub($m::$set(&c, "X", None), $label);
                assert_stub($m::$remove(&c, "X"), $label);
            }
        };
    }

    stub_suite!(
        candidate_stubs,
        candidate,
        "Candidate",
        add_candidate_function,
        delete_candidate_function,
        get_candidate_function,
        list_candidate_functions,
        set_candidate_function,
        remove_candidate_function
    );
    stub_suite!(
        matching_stubs,
        matching,
        "Matching",
        add_matching_function,
        delete_matching_function,
        get_matching_function,
        list_matching_functions,
        set_matching_function,
        remove_matching_function
    );
    stub_suite!(
        scoring_stubs,
        scoring,
        "Scoring",
        add_scoring_function,
        delete_scoring_function,
        get_scoring_function,
        list_scoring_functions,
        set_scoring_function,
        remove_scoring_function
    );
    stub_suite!(
        validation_stubs,
        validation,
        "Validation",
        add_validation_function,
        delete_validation_function,
        get_validation_function,
        list_validation_functions,
        set_validation_function,
        remove_validation_function
    );
}
