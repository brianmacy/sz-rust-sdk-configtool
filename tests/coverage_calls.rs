//! Coverage of `sz_configtool_lib::calls` (CFG_SFCALL, CFG_EFCALL/CFG_EFBOM,
//! CFG_CFCALL/CFG_CFBOM, CFG_DFCALL/CFG_DFBOM), exercised against the real
//! template fixture or minimal inline configs. Every error path asserts the
//! exact `SzErrorKind` and message.

mod cov_support;

use cov_support::{BAD_JSON, assert_err, assert_kind, rows, template};
use serde_json::{Value, json};
use sz_configtool_lib::calls::CallSelector;
use sz_configtool_lib::error::SzErrorKind;

/// Minimal config shared by the BOM-backed families: one feature (NAME, id 1)
/// with one member element (FIRST_NAME, id 11), a non-member element
/// (OTHER, id 12), and one function of every kind (id 7).
fn mini(extra: Value) -> String {
    let mut base = json!({"G2_CONFIG": {
        "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
        "CFG_FELEM": [
            {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
            {"FELEM_ID": 12, "FELEM_CODE": "OTHER"}
        ],
        "CFG_FBOM": [{"FTYPE_ID": 1, "FELEM_ID": 11}],
        "CFG_CFUNC": [{"CFUNC_ID": 7, "CFUNC_CODE": "CF"}],
        "CFG_DFUNC": [{"DFUNC_ID": 7, "DFUNC_CODE": "DF"}],
        "CFG_EFUNC": [{"EFUNC_ID": 7, "EFUNC_CODE": "EF"}],
        "CFG_SFUNC": [{"SFUNC_ID": 7, "SFUNC_CODE": "SF"}]
    }});
    if let (Some(g), Some(extra)) = (base["G2_CONFIG"].as_object_mut(), extra.as_object()) {
        for (k, v) in extra {
            g.insert(k.clone(), v.clone());
        }
    }
    base.to_string()
}

// ===========================================================================
// calls/mod.rs
// ===========================================================================

#[test]
fn element_feature_with_unknown_element_is_not_in_feature() {
    use sz_configtool_lib::calls::comparison::delete_comparison_call_element;
    let config = mini(json!({
        "CFG_CFCALL": [{"CFCALL_ID": 1, "FTYPE_ID": 1, "CFUNC_ID": 7}],
        "CFG_CFBOM": []
    }));
    assert_err(
        delete_comparison_call_element(&config, CallSelector::Id(1), "NOPE", Some("NAME")),
        SzErrorKind::NotInFeature,
        "NOPE is not an element of NAME",
    );
}

// ===========================================================================
// Comparison calls
// ===========================================================================

mod comparison {
    use super::*;
    use sz_configtool_lib::calls::comparison::*;

    fn with_call(bom: Value) -> String {
        mini(json!({
            "CFG_CFCALL": [{"CFCALL_ID": 1, "FTYPE_ID": 1, "CFUNC_ID": 7}],
            "CFG_CFBOM": bom
        }))
    }

    fn add_params(elements: &[&str]) -> AddComparisonCallParams {
        AddComparisonCallParams {
            ftype_code: "NAME".into(),
            cfunc_code: "CF".into(),
            element_list: elements.iter().map(|s| s.to_string()).collect(),
            id: None,
        }
    }

    #[test]
    fn params_try_from() {
        let p = AddComparisonCallParams::try_from(&json!({
            "ftypeCode": "NAME", "cfuncCode": "CF", "elementList": ["A", 3, "B"], "id": 5
        }))
        .unwrap();
        assert_eq!(
            (p.ftype_code.as_str(), p.cfunc_code.as_str(), p.id),
            ("NAME", "CF", Some(5))
        );
        assert_eq!(p.element_list, vec!["A", "B"]);
        let p = AddComparisonCallParams::try_from(&json!({"ftypeCode": "N", "cfuncCode": "C"}))
            .unwrap();
        assert!(p.element_list.is_empty());
        assert_eq!(p.id, None);
        assert_err(
            AddComparisonCallParams::try_from(&json!({"cfuncCode": "C"})),
            SzErrorKind::MissingField,
            "ftypeCode",
        );
        assert_err(
            AddComparisonCallParams::try_from(&json!({"ftypeCode": "N"})),
            SzErrorKind::MissingField,
            "cfuncCode",
        );

        let p = AddComparisonCallElementParams::try_from(&json!({
            "cfcallId": 1, "ftypeId": 2, "felemId": 3, "execOrder": 4
        }))
        .unwrap();
        assert_eq!(
            (p.cfcall_id, p.ftype_id, p.felem_id, p.exec_order),
            (1, 2, 3, Some(4))
        );
        let p = AddComparisonCallElementParams::try_from(&json!({
            "cfcallId": 1, "ftypeId": 2, "felemId": 3
        }))
        .unwrap();
        assert_eq!(p.exec_order, None);
        for (json, field) in [
            (json!({"ftypeId": 2, "felemId": 3}), "cfcallId"),
            (json!({"cfcallId": 1, "felemId": 3}), "ftypeId"),
            (json!({"cfcallId": 1, "ftypeId": 2}), "felemId"),
        ] {
            assert_err(
                AddComparisonCallElementParams::try_from(&json),
                SzErrorKind::MissingField,
                field,
            );
        }
    }

    #[test]
    fn add_call_error_paths() {
        assert_kind(
            add_comparison_call(BAD_JSON, add_params(&["FIRST_NAME"])),
            SzErrorKind::JsonParse,
        );
        assert_err(
            add_comparison_call(&mini(json!({"CFG_CFBOM": []})), add_params(&["FIRST_NAME"])),
            SzErrorKind::MissingSection,
            "Section path 'G2_CONFIG.CFG_CFCALL' not found",
        );
        let empty = mini(json!({"CFG_CFCALL": [], "CFG_CFBOM": []}));
        let mut p = add_params(&["FIRST_NAME"]);
        p.ftype_code = "NOPE".into();
        assert_err(
            add_comparison_call(&empty, p),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            add_comparison_call(&with_call(json!([])), add_params(&["FIRST_NAME"])),
            SzErrorKind::AlreadyPresent,
            "Comparison call for feature NAME already set",
        );
        let mut p = add_params(&["FIRST_NAME"]);
        p.cfunc_code = "NOPE".into();
        assert_err(
            add_comparison_call(&empty, p),
            SzErrorKind::NotFound,
            "Comparison function 'NOPE' not found",
        );
        assert_err(
            add_comparison_call(&empty, add_params(&[])),
            SzErrorKind::InvalidInput,
            "No elements were found in the elementList",
        );
        assert_err(
            add_comparison_call(&empty, add_params(&["FIRST_NAME", "  "])),
            SzErrorKind::InvalidInput,
            "Element cannot be blank in item 2 on the element list",
        );
        assert_err(
            add_comparison_call(&empty, add_params(&["NOPE"])),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        assert_err(
            add_comparison_call(
                &mini(json!({"CFG_CFCALL": []})),
                add_params(&["FIRST_NAME"]),
            ),
            SzErrorKind::MissingSection,
            "CFG_CFBOM",
        );
    }

    #[test]
    fn add_call_on_template() {
        let mut p = add_params(&["ADDR_FULL"]);
        p.ftype_code = "ADDR_KEY".into();
        p.cfunc_code = "STR_COMP".into();
        let (out, rec) = add_comparison_call(&template(), p).unwrap();
        assert_eq!(rec["CFCALL_ID"], 1000);
        assert!(rows(&out, "CFG_CFCALL").contains(&rec));
        assert!(
            rows(&out, "CFG_CFBOM")
                .iter()
                .any(|r| r["CFCALL_ID"] == 1000 && r["EXEC_ORDER"] == 1)
        );
    }

    #[test]
    fn delete_call_paths() {
        assert_kind(delete_comparison_call(BAD_JSON, 1), SzErrorKind::JsonParse);
        assert_err(
            delete_comparison_call(&mini(json!({})), 1),
            SzErrorKind::NotFound,
            "Comparison call ID 1 does not exist",
        );
        let config = with_call(json!([
            {"CFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 11, "EXEC_ORDER": 1},
            {"CFCALL_ID": 2, "FTYPE_ID": 1, "FELEM_ID": 11, "EXEC_ORDER": 1}
        ]));
        let out = delete_comparison_call(&config, 1).unwrap();
        assert!(rows(&out, "CFG_CFCALL").is_empty());
        assert_eq!(rows(&out, "CFG_CFBOM").len(), 1);
        // A config without CFG_CFBOM still deletes the call.
        let no_bom = mini(json!({"CFG_CFCALL": [{"CFCALL_ID": 1, "FTYPE_ID": 1}]}));
        let out = delete_comparison_call(&no_bom, 1).unwrap();
        assert!(rows(&out, "CFG_CFCALL").is_empty());
    }

    #[test]
    fn get_call_paths() {
        assert_kind(
            get_comparison_call(BAD_JSON, CallSelector::Id(1)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            get_comparison_call(&template(), CallSelector::Feature("NOPE")),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            get_comparison_call(&template(), CallSelector::Id(99999)),
            SzErrorKind::NotFound,
            "Comparison call ID 99999 does not exist",
        );
        let call = get_comparison_call(&template(), CallSelector::Feature("NAME")).unwrap();
        assert_eq!(call["CFCALL_ID"], 1);
    }

    #[test]
    fn list_calls() {
        assert_kind(list_comparison_calls(BAD_JSON), SzErrorKind::JsonParse);
        assert!(list_comparison_calls("{}").unwrap().is_empty());
        // Rows with unresolvable ids / missing columns project "unknown"/0.
        let config = json!({"G2_CONFIG": {
            "CFG_CFCALL": [{"CFCALL_ID": 2, "FTYPE_ID": 5, "CFUNC_ID": 5}, {}],
            "CFG_CFBOM": [{"CFCALL_ID": 2, "FELEM_ID": 99}, {"CFCALL_ID": 2, "EXEC_ORDER": 0}]
        }})
        .to_string();
        let items = list_comparison_calls(&config).unwrap();
        assert_eq!(
            items[0],
            json!({"id": 0, "feature": "unknown", "function": "unknown", "elementList": []})
        );
        assert_eq!(items[1]["elementList"], json!(["unknown", "unknown"]));
        let items = list_comparison_calls(&template()).unwrap();
        assert_eq!(items.len(), rows(&template(), "CFG_CFCALL").len());
        assert_eq!(items[0]["feature"], "NAME");
        assert_eq!(items[0]["function"], "GNR_COMP");
    }

    fn elem(ftype_id: i64, exec_order: Option<i64>) -> AddComparisonCallElementParams {
        AddComparisonCallElementParams {
            cfcall_id: 1,
            ftype_id,
            felem_id: 11,
            exec_order,
        }
    }

    #[test]
    fn add_element_paths() {
        assert_kind(
            add_comparison_call_element(BAD_JSON, elem(1, None)),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"CFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 12, "EXEC_ORDER": 1}
        ]));
        assert_err(
            add_comparison_call_element(&config, elem(-1, None)),
            SzErrorKind::InvalidInput,
            "-1 is not a valid feature ID",
        );
        assert_err(
            add_comparison_call_element(&config, elem(1, Some(1))),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        let (out, rec) = add_comparison_call_element(&config, elem(1, None)).unwrap();
        assert_eq!(rec["EXEC_ORDER"], 2);
        assert_err(
            add_comparison_call_element(&out, elem(1, None)),
            SzErrorKind::AlreadyPresent,
            "Feature/element already exists for call",
        );
        assert_err(
            add_comparison_call_element(&mini(json!({})), elem(1, None)),
            SzErrorKind::MissingSection,
            "CFG_CFBOM",
        );
    }

    #[test]
    fn delete_element_paths() {
        assert_kind(
            delete_comparison_call_element(BAD_JSON, CallSelector::Id(1), "FIRST_NAME", None),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"CFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 11, "EXEC_ORDER": 1},
            {"CFCALL_ID": 1, "FTYPE_ID": 2, "FELEM_ID": 11, "EXEC_ORDER": 1}
        ]));
        assert_err(
            delete_comparison_call_element(&config, CallSelector::Feature("NOPE"), "X", None),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_comparison_call_element(&config, CallSelector::Id(5), "FIRST_NAME", None),
            SzErrorKind::NotFound,
            "Comparison call ID 5 does not exist",
        );
        assert_err(
            delete_comparison_call_element(&config, CallSelector::Id(1), "X", Some("NOPE")),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_comparison_call_element(&config, CallSelector::Id(1), "NOPE", None),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        // Feature-scoped delete removes only the NAME-feature row.
        let out = delete_comparison_call_element(
            &config,
            CallSelector::Feature("NAME"),
            "FIRST_NAME",
            Some("NAME"),
        )
        .unwrap();
        assert_eq!(
            rows(&out, "CFG_CFBOM"),
            vec![json!({"CFCALL_ID": 1, "FTYPE_ID": 2, "FELEM_ID": 11, "EXEC_ORDER": 1})]
        );
        // Unscoped delete of the remaining single row.
        let out =
            delete_comparison_call_element(&out, CallSelector::Id(1), "FIRST_NAME", None).unwrap();
        assert!(rows(&out, "CFG_CFBOM").is_empty());
    }
}

// ===========================================================================
// Distinct calls
// ===========================================================================

mod distinct {
    use super::*;
    use sz_configtool_lib::calls::distinct::*;

    fn with_call(bom: Value) -> String {
        mini(json!({
            "CFG_DFCALL": [{"DFCALL_ID": 1, "FTYPE_ID": 1, "DFUNC_ID": 7}],
            "CFG_DFBOM": bom
        }))
    }

    fn add_params(elements: &[&str]) -> AddDistinctCallParams {
        AddDistinctCallParams {
            ftype_code: "NAME".into(),
            dfunc_code: "DF".into(),
            element_list: elements.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn params_try_from() {
        let p = AddDistinctCallParams::try_from(&json!({
            "ftypeCode": "NAME", "dfuncCode": "DF", "elementList": ["A", 3]
        }))
        .unwrap();
        assert_eq!(
            (p.ftype_code.as_str(), p.dfunc_code.as_str()),
            ("NAME", "DF")
        );
        assert_eq!(p.element_list, vec!["A"]);
        let p =
            AddDistinctCallParams::try_from(&json!({"ftypeCode": "N", "dfuncCode": "D"})).unwrap();
        assert!(p.element_list.is_empty());
        assert_err(
            AddDistinctCallParams::try_from(&json!({"dfuncCode": "D"})),
            SzErrorKind::MissingField,
            "ftypeCode",
        );
        assert_err(
            AddDistinctCallParams::try_from(&json!({"ftypeCode": "N"})),
            SzErrorKind::MissingField,
            "dfuncCode",
        );
    }

    #[test]
    fn add_call_error_paths() {
        assert_kind(
            add_distinct_call(BAD_JSON, add_params(&["FIRST_NAME"])),
            SzErrorKind::JsonParse,
        );
        let empty = mini(json!({"CFG_DFCALL": [], "CFG_DFBOM": []}));
        assert_err(
            add_distinct_call(&empty, add_params(&[])),
            SzErrorKind::InvalidInput,
            "No elements were found in the elementList",
        );
        assert_err(
            add_distinct_call(&empty, add_params(&["FIRST_NAME", ""])),
            SzErrorKind::InvalidInput,
            "Element cannot be blank in item 2 on the element list",
        );
        assert_err(
            add_distinct_call(&mini(json!({})), add_params(&["FIRST_NAME"])),
            SzErrorKind::MissingSection,
            "Section path 'G2_CONFIG.CFG_DFCALL' not found",
        );
        let mut p = add_params(&["FIRST_NAME"]);
        p.ftype_code = "NOPE".into();
        assert_err(
            add_distinct_call(&empty, p),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            add_distinct_call(&with_call(json!([])), add_params(&["FIRST_NAME"])),
            SzErrorKind::AlreadyPresent,
            "Distinct call for feature NAME already set",
        );
        let mut p = add_params(&["FIRST_NAME"]);
        p.dfunc_code = "NOPE".into();
        assert_err(
            add_distinct_call(&empty, p),
            SzErrorKind::NotFound,
            "Distinct function 'NOPE' not found",
        );
        assert_err(
            add_distinct_call(&empty, add_params(&["NOPE"])),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        // CFG_DFCALL present but not an array: id allocation tolerates it, the
        // write does not.
        assert_err(
            add_distinct_call(
                &mini(json!({"CFG_DFCALL": null, "CFG_DFBOM": []})),
                add_params(&["FIRST_NAME"]),
            ),
            SzErrorKind::MissingSection,
            "CFG_DFCALL",
        );
        assert_err(
            add_distinct_call(
                &mini(json!({"CFG_DFCALL": []})),
                add_params(&["FIRST_NAME"]),
            ),
            SzErrorKind::MissingSection,
            "CFG_DFBOM",
        );
    }

    #[test]
    fn add_call_on_template() {
        let mut p = add_params(&["GENDER"]);
        p.ftype_code = "GENDER".into();
        p.dfunc_code = "FELEM_STRICT_SUBSET".into();
        let (out, rec) = add_distinct_call(&template(), p).unwrap();
        assert_eq!(rec["DFCALL_ID"], 1000);
        assert!(rows(&out, "CFG_DFCALL").contains(&rec));
        assert!(
            rows(&out, "CFG_DFBOM")
                .iter()
                .any(|r| r["DFCALL_ID"] == 1000)
        );
    }

    #[test]
    fn delete_call_paths() {
        assert_kind(delete_distinct_call(BAD_JSON, 1), SzErrorKind::JsonParse);
        assert_err(
            delete_distinct_call(&mini(json!({})), 1),
            SzErrorKind::NotFound,
            "Distinct call ID 1 does not exist",
        );
        let config = with_call(json!([{"DFCALL_ID": 1}, {"DFCALL_ID": 2}]));
        let out = delete_distinct_call(&config, 1).unwrap();
        assert!(rows(&out, "CFG_DFCALL").is_empty());
        assert_eq!(rows(&out, "CFG_DFBOM"), vec![json!({"DFCALL_ID": 2})]);
        let no_bom = mini(json!({"CFG_DFCALL": [{"DFCALL_ID": 1}]}));
        assert!(rows(&delete_distinct_call(&no_bom, 1).unwrap(), "CFG_DFCALL").is_empty());
    }

    #[test]
    fn get_call_paths() {
        assert_kind(
            get_distinct_call(BAD_JSON, CallSelector::Id(1)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            get_distinct_call(&template(), CallSelector::Feature("NOPE")),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            get_distinct_call(&template(), CallSelector::Id(99999)),
            SzErrorKind::NotFound,
            "Distinct call ID 99999 does not exist",
        );
        let call = get_distinct_call(&template(), CallSelector::Feature("NAME")).unwrap();
        assert_eq!(call["DFCALL_ID"], 1);
    }

    #[test]
    fn list_calls() {
        assert_kind(list_distinct_calls(BAD_JSON), SzErrorKind::JsonParse);
        assert!(list_distinct_calls("{}").unwrap().is_empty());
        let config = json!({"G2_CONFIG": {
            "CFG_DFCALL": [{"DFCALL_ID": 2, "FTYPE_ID": 5, "DFUNC_ID": 5, "EXEC_ORDER": 3}, {}],
            "CFG_DFBOM": [{"DFCALL_ID": 2, "FELEM_ID": 99}, {"DFCALL_ID": 2}]
        }})
        .to_string();
        let items = list_distinct_calls(&config).unwrap();
        assert_eq!(
            items[0],
            json!({"id": 0, "feature": "unknown", "function": "unknown",
                   "execOrder": 1, "elementList": []})
        );
        assert_eq!(items[1]["execOrder"], 3);
        assert_eq!(items[1]["elementList"], json!(["unknown", "unknown"]));
        let items = list_distinct_calls(&template()).unwrap();
        assert_eq!(items[0]["feature"], "NAME");
        assert_eq!(items[0]["elementList"][0], "FULL_NAME");
    }

    fn elem(exec_order: Option<i64>) -> AddDistinctCallElementParams {
        AddDistinctCallElementParams {
            dfcall_id: 1,
            ftype_id: 1,
            felem_id: 11,
            exec_order,
        }
    }

    #[test]
    fn add_element_paths() {
        assert_kind(
            add_distinct_call_element(BAD_JSON, elem(None)),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"DFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 12, "EXEC_ORDER": 1}
        ]));
        assert_err(
            add_distinct_call_element(&config, elem(Some(1))),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        let (out, rec) = add_distinct_call_element(&config, elem(None)).unwrap();
        assert_eq!(rec["EXEC_ORDER"], 2);
        assert_err(
            add_distinct_call_element(&out, elem(None)),
            SzErrorKind::AlreadyPresent,
            "Distinct call element already exists",
        );
        assert_err(
            add_distinct_call_element(&mini(json!({})), elem(None)),
            SzErrorKind::MissingSection,
            "CFG_DFBOM",
        );
    }

    #[test]
    fn delete_element_paths() {
        assert_kind(
            delete_distinct_call_element(BAD_JSON, CallSelector::Id(1), "FIRST_NAME", None),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"DFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 11, "EXEC_ORDER": 1},
            {"DFCALL_ID": 1, "FTYPE_ID": 2, "FELEM_ID": 11, "EXEC_ORDER": 1}
        ]));
        assert_err(
            delete_distinct_call_element(&config, CallSelector::Feature("NOPE"), "X", None),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_distinct_call_element(&config, CallSelector::Id(5), "FIRST_NAME", None),
            SzErrorKind::NotFound,
            "Distinct call ID 5 does not exist",
        );
        assert_err(
            delete_distinct_call_element(&config, CallSelector::Id(1), "X", Some("NOPE")),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_distinct_call_element(&config, CallSelector::Id(1), "OTHER", Some("NAME")),
            SzErrorKind::NotInFeature,
            "OTHER is not an element of NAME",
        );
        assert_err(
            delete_distinct_call_element(&config, CallSelector::Id(1), "NOPE", None),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        let out = delete_distinct_call_element(
            &config,
            CallSelector::Feature("NAME"),
            "FIRST_NAME",
            Some("NAME"),
        )
        .unwrap();
        assert_eq!(rows(&out, "CFG_DFBOM").len(), 1);
        let out =
            delete_distinct_call_element(&out, CallSelector::Id(1), "FIRST_NAME", None).unwrap();
        assert!(rows(&out, "CFG_DFBOM").is_empty());
    }
}

// ===========================================================================
// Expression calls
// ===========================================================================

mod expression {
    use super::*;
    use sz_configtool_lib::calls::expression::*;

    fn with_call(bom: Value) -> String {
        mini(json!({
            "CFG_EFCALL": [{"EFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": -1, "EFUNC_ID": 7,
                            "EXEC_ORDER": 1, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"}],
            "CFG_EFBOM": bom
        }))
    }

    fn el(code: &str, feature: Option<&str>) -> (String, String, Option<String>) {
        (
            code.to_string(),
            "Yes".to_string(),
            feature.map(str::to_string),
        )
    }

    fn feature_params<'a>(
        elements: Vec<(String, String, Option<String>)>,
    ) -> AddExpressionCallParams<'a> {
        let mut p = AddExpressionCallParams::new("EF", elements);
        p.ftype_code = Some("NAME");
        p
    }

    #[test]
    fn params_constructors() {
        let p = AddExpressionCallParams::new("EF", vec![]);
        assert_eq!(
            (
                p.ftype_code,
                p.felem_code,
                p.exec_order,
                p.expression_feature,
                p.is_virtual
            ),
            (None, None, None, None, "No")
        );
        let e = ExpressionCallElementParams::new(1, 2, Some(3), "Yes".into());
        assert_eq!(
            (e.ftype_id, e.felem_id, e.exec_order, e.felem_req.as_str()),
            (1, 2, Some(3), "Yes")
        );
    }

    #[test]
    fn add_call_error_paths() {
        let empty = mini(json!({"CFG_EFCALL": [], "CFG_EFBOM": []}));
        assert_kind(
            add_expression_call(BAD_JSON, feature_params(vec![])),
            SzErrorKind::JsonParse,
        );
        assert_err(
            add_expression_call(&mini(json!({})), feature_params(vec![])),
            SzErrorKind::MissingSection,
            "Section path 'G2_CONFIG.CFG_EFCALL' not found",
        );
        let mut p = feature_params(vec![]);
        p.efunc_code = "NOPE";
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::NotFound,
            "Expression function 'NOPE' not found",
        );
        let mut p = feature_params(vec![]);
        p.ftype_code = Some("NOPE");
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        let mut p = feature_params(vec![]);
        p.felem_code = Some("NOPE");
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        let both_or_neither = "Either a feature or an element must be specified, but not both";
        let mut p = feature_params(vec![]);
        p.felem_code = Some("FIRST_NAME");
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::InvalidInput,
            both_or_neither,
        );
        let mut p = AddExpressionCallParams::new("EF", vec![]);
        p.ftype_code = Some("all");
        p.felem_code = Some("n/a");
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::InvalidInput,
            both_or_neither,
        );
        let mut p = feature_params(vec![]);
        p.exec_order = Some(1);
        assert_err(
            add_expression_call(&with_call(json!([])), p),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        let mut p = feature_params(vec![]);
        p.expression_feature = Some("NOPE");
        assert_err(
            add_expression_call(&empty, p),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            add_expression_call(&empty, feature_params(vec![el("FIRST_NAME", Some("NOPE"))])),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            add_expression_call(&empty, feature_params(vec![el("NOPE", None)])),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        assert_err(
            add_expression_call(
                &mini(json!({"CFG_EFCALL": null, "CFG_EFBOM": []})),
                feature_params(vec![]),
            ),
            SzErrorKind::MissingSection,
            "CFG_EFCALL",
        );
        assert_err(
            add_expression_call(&mini(json!({"CFG_EFCALL": []})), feature_params(vec![])),
            SzErrorKind::MissingSection,
            "CFG_EFBOM",
        );
    }

    #[test]
    fn add_call_builds_rows() {
        let empty = mini(json!({"CFG_EFCALL": [], "CFG_EFBOM": []}));
        let mut p = feature_params(vec![
            el("FIRST_NAME", Some("parent")),
            el("FIRST_NAME", Some("NAME")),
            el("OTHER", None),
        ]);
        p.expression_feature = Some("NAME");
        p.is_virtual = "Yes";
        let (out, rec) = add_expression_call(&empty, p).unwrap();
        assert_eq!(rec["EFCALL_ID"], 1000);
        assert_eq!(rec["EFEAT_FTYPE_ID"], 1);
        assert_eq!(rec["IS_VIRTUAL"], "Yes");
        let ftypes: Vec<Value> = rows(&out, "CFG_EFBOM")
            .iter()
            .map(|r| r["FTYPE_ID"].clone())
            .collect();
        assert_eq!(ftypes, vec![json!(0), json!(1), json!(-1)]);

        // Element-bound call with an explicit N/A expression feature.
        let mut p = AddExpressionCallParams::new("EF", vec![]);
        p.felem_code = Some("OTHER");
        p.expression_feature = Some("N/A");
        let (_, rec) = add_expression_call(&empty, p).unwrap();
        assert_eq!(
            (rec["FTYPE_ID"].clone(), rec["FELEM_ID"].clone()),
            (json!(-1), json!(12))
        );
        assert_eq!(rec["EFEAT_FTYPE_ID"], -1);
    }

    #[test]
    fn delete_call_paths() {
        assert_kind(delete_expression_call(BAD_JSON, 1), SzErrorKind::JsonParse);
        assert_err(
            delete_expression_call(&mini(json!({})), 1),
            SzErrorKind::NotFound,
            "Expression call ID 1 does not exist",
        );
        let out =
            delete_expression_call(&with_call(json!([{"EFCALL_ID": 1}, {"EFCALL_ID": 2}])), 1)
                .unwrap();
        assert!(rows(&out, "CFG_EFCALL").is_empty());
        assert_eq!(rows(&out, "CFG_EFBOM"), vec![json!({"EFCALL_ID": 2})]);
        let no_bom = mini(json!({"CFG_EFCALL": [{"EFCALL_ID": 1}]}));
        assert!(rows(&delete_expression_call(&no_bom, 1).unwrap(), "CFG_EFCALL").is_empty());
    }

    #[test]
    fn get_call_paths() {
        assert_kind(
            get_expression_call(BAD_JSON, CallSelector::Id(1)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            get_expression_call(&template(), CallSelector::Id(99999)),
            SzErrorKind::NotFound,
            "Expression call ID 99999 does not exist",
        );
        let call = get_expression_call(&template(), CallSelector::Feature("PHONE")).unwrap();
        assert_eq!(call["EFCALL_ID"], 1);
    }

    #[test]
    fn list_calls() {
        assert_kind(list_expression_calls(BAD_JSON), SzErrorKind::JsonParse);
        assert!(list_expression_calls("{}").unwrap().is_empty());
        let config = json!({"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
            "CFG_EFCALL": [
                {"EFCALL_ID": 2, "FTYPE_ID": 5, "FELEM_ID": 99, "EFUNC_ID": 5,
                 "EFEAT_FTYPE_ID": 1, "IS_VIRTUAL": "Yes", "EXEC_ORDER": 4},
                {}
            ],
            "CFG_EFBOM": [{"EFCALL_ID": 2, "FELEM_ID": 11}, {"EFCALL_ID": 2, "FELEM_ID": 98}, {"EFCALL_ID": 2}]
        }})
        .to_string();
        let items = list_expression_calls(&config).unwrap();
        assert_eq!(
            items[0],
            json!({"id": 0, "feature": "all", "element": "n/a", "execOrder": 0,
                   "function": "unknown", "isVirtual": "No", "expressionFeature": "n/a",
                   "elementList": []})
        );
        assert_eq!(
            items[1],
            json!({"id": 2, "feature": "all", "element": "n/a", "execOrder": 4,
                   "function": "unknown", "isVirtual": "Yes", "expressionFeature": "NAME",
                   "elementList": ["FIRST_NAME", "n/a", "n/a"]})
        );
        let items = list_expression_calls(&template()).unwrap();
        assert!(items.iter().any(|i| i["expressionFeature"] != "n/a"));
    }

    fn elem(ftype_id: i64, exec_order: Option<i64>) -> ExpressionCallElementParams {
        ExpressionCallElementParams::new(ftype_id, 11, exec_order, "Yes".into())
    }

    #[test]
    fn add_element_paths() {
        assert_kind(
            add_expression_call_element(BAD_JSON, 1, elem(1, None)),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"EFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 12, "EXEC_ORDER": 1, "FELEM_REQ": "No"}
        ]));
        assert_err(
            add_expression_call_element(&config, 1, elem(-1, None)),
            SzErrorKind::InvalidInput,
            "-1 is not a valid feature ID",
        );
        assert_err(
            add_expression_call_element(&config, 1, elem(1, Some(1))),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        let (out, rec) = add_expression_call_element(&config, 1, elem(1, None)).unwrap();
        assert_eq!(
            (rec["EXEC_ORDER"].clone(), rec["FELEM_REQ"].clone()),
            (json!(2), json!("Yes"))
        );
        assert_err(
            add_expression_call_element(&out, 1, elem(1, None)),
            SzErrorKind::AlreadyPresent,
            "Feature/element already exists for call",
        );
        assert_err(
            add_expression_call_element(&mini(json!({})), 1, elem(1, None)),
            SzErrorKind::MissingSection,
            "CFG_EFBOM",
        );
    }

    #[test]
    fn delete_element_paths() {
        assert_kind(
            delete_expression_call_element(BAD_JSON, CallSelector::Id(1), "FIRST_NAME", None),
            SzErrorKind::JsonParse,
        );
        let config = with_call(json!([
            {"EFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 11, "EXEC_ORDER": 1},
            {"EFCALL_ID": 1, "FTYPE_ID": 2, "FELEM_ID": 11, "EXEC_ORDER": 1}
        ]));
        assert_err(
            delete_expression_call_element(&config, CallSelector::Feature("NOPE"), "X", None),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_expression_call_element(&config, CallSelector::Id(5), "FIRST_NAME", None),
            SzErrorKind::NotFound,
            "Expression call ID 5 does not exist",
        );
        assert_err(
            delete_expression_call_element(&config, CallSelector::Id(1), "X", Some("NOPE")),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        assert_err(
            delete_expression_call_element(&config, CallSelector::Id(1), "NOPE", None),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        let out = delete_expression_call_element(
            &config,
            CallSelector::Feature("NAME"),
            "FIRST_NAME",
            Some("NAME"),
        )
        .unwrap();
        assert_eq!(rows(&out, "CFG_EFBOM").len(), 1);
        let out =
            delete_expression_call_element(&out, CallSelector::Id(1), "FIRST_NAME", None).unwrap();
        assert!(rows(&out, "CFG_EFBOM").is_empty());
    }
}

// ===========================================================================
// Standardize calls
// ===========================================================================

mod standardize {
    use super::*;
    use sz_configtool_lib::calls::standardize::*;

    fn with_call() -> String {
        mini(json!({
            "CFG_SFCALL": [{"SFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": -1, "SFUNC_ID": 7,
                            "EXEC_ORDER": 1}]
        }))
    }

    fn feature_params<'a>() -> AddStandardizeCallParams<'a> {
        let mut p = AddStandardizeCallParams::new("SF");
        p.ftype_code = Some("NAME");
        p
    }

    #[test]
    fn params_constructors() {
        let p = AddStandardizeCallParams::new("SF");
        assert_eq!(
            (p.ftype_code, p.felem_code, p.exec_order, p.sfunc_code),
            (None, None, None, "SF")
        );
    }

    #[test]
    fn add_call_error_paths() {
        let empty = mini(json!({"CFG_SFCALL": []}));
        assert_kind(
            add_standardize_call(BAD_JSON, feature_params()),
            SzErrorKind::JsonParse,
        );
        assert_err(
            add_standardize_call(&mini(json!({})), feature_params()),
            SzErrorKind::MissingSection,
            "Section path 'G2_CONFIG.CFG_SFCALL' not found",
        );
        let mut p = feature_params();
        p.sfunc_code = "NOPE";
        assert_err(
            add_standardize_call(&empty, p),
            SzErrorKind::NotFound,
            "Standardize function 'NOPE' not found",
        );
        let mut p = feature_params();
        p.ftype_code = Some("NOPE");
        assert_err(
            add_standardize_call(&empty, p),
            SzErrorKind::NotFound,
            "Feature 'NOPE' not found",
        );
        let mut p = feature_params();
        p.felem_code = Some("NOPE");
        assert_err(
            add_standardize_call(&empty, p),
            SzErrorKind::NotFound,
            "Element 'NOPE' not found",
        );
        let both_or_neither = "Either a feature or an element must be specified, but not both";
        let mut p = feature_params();
        p.felem_code = Some("FIRST_NAME");
        assert_err(
            add_standardize_call(&empty, p),
            SzErrorKind::InvalidInput,
            both_or_neither,
        );
        let mut p = AddStandardizeCallParams::new("SF");
        p.ftype_code = Some("ALL");
        p.felem_code = Some("N/A");
        assert_err(
            add_standardize_call(&empty, p),
            SzErrorKind::InvalidInput,
            both_or_neither,
        );
        let mut p = feature_params();
        p.exec_order = Some(1);
        assert_err(
            add_standardize_call(&with_call(), p),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        assert_err(
            add_standardize_call(&mini(json!({"CFG_SFCALL": null})), feature_params()),
            SzErrorKind::MissingSection,
            "CFG_SFCALL",
        );
    }

    #[test]
    fn add_call_on_template() {
        let mut p = AddStandardizeCallParams::new("PARSE_NAME");
        p.felem_code = Some("FULL_NAME");
        let (out, rec) = add_standardize_call(&template(), p).unwrap();
        assert_eq!(
            (rec["SFCALL_ID"].clone(), rec["FTYPE_ID"].clone()),
            (json!(1000), json!(-1))
        );
        assert!(rows(&out, "CFG_SFCALL").contains(&rec));
    }

    #[test]
    fn delete_call_paths() {
        assert_kind(delete_standardize_call(BAD_JSON, 1), SzErrorKind::JsonParse);
        assert_err(
            delete_standardize_call(&mini(json!({})), 1),
            SzErrorKind::NotFound,
            "Standardize call ID 1 does not exist",
        );
        let out = delete_standardize_call(&with_call(), 1).unwrap();
        assert!(rows(&out, "CFG_SFCALL").is_empty());
    }

    #[test]
    fn get_call_paths() {
        assert_kind(
            get_standardize_call(BAD_JSON, CallSelector::Id(1)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            get_standardize_call(&template(), CallSelector::Id(99999)),
            SzErrorKind::NotFound,
            "Standardize call ID 99999 does not exist",
        );
        let call = get_standardize_call(&template(), CallSelector::Id(19)).unwrap();
        assert_eq!(call["FELEM_ID"], 28);
    }

    #[test]
    fn list_calls() {
        assert_kind(list_standardize_calls(BAD_JSON), SzErrorKind::JsonParse);
        assert!(list_standardize_calls("{}").unwrap().is_empty());
        let config = json!({"G2_CONFIG": {
            "CFG_SFCALL": [
                {"SFCALL_ID": 2, "FTYPE_ID": 5, "FELEM_ID": 99, "SFUNC_ID": 5, "EXEC_ORDER": 4},
                {}
            ]
        }})
        .to_string();
        let items = list_standardize_calls(&config).unwrap();
        assert_eq!(
            items[0],
            json!({"id": 0, "feature": "all", "element": "n/a", "execOrder": 0,
                   "function": "unknown"})
        );
        assert_eq!(
            items[1],
            json!({"id": 2, "feature": "all", "element": "n/a", "execOrder": 4,
                   "function": "unknown"})
        );
        let items = list_standardize_calls(&template()).unwrap();
        assert!(items.iter().any(|i| i["element"] != "n/a"));
        assert!(items.iter().any(|i| i["feature"] == "NAME"));
    }

    fn elem(felem_id: Option<i64>, exec_order: Option<i64>) -> AddStandardizeCallElementParams {
        AddStandardizeCallElementParams {
            ftype_id: 1,
            sfunc_id: 8,
            felem_id,
            exec_order,
        }
    }

    #[test]
    fn add_element_paths() {
        assert_kind(
            add_standardize_call_element(BAD_JSON, elem(None, None)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            add_standardize_call_element(&mini(json!({})), elem(None, None)),
            SzErrorKind::MissingSection,
            "Section path 'G2_CONFIG.CFG_SFCALL' not found",
        );
        assert_err(
            add_standardize_call_element(&with_call(), elem(None, Some(1))),
            SzErrorKind::AlreadyExists,
            "The specified EXEC_ORDER 1 is already taken",
        );
        let (out, rec) = add_standardize_call_element(&with_call(), elem(None, None)).unwrap();
        assert_eq!(
            (rec["SFCALL_ID"].clone(), rec["EXEC_ORDER"].clone()),
            (json!(1000), json!(2))
        );
        assert_err(
            add_standardize_call_element(&out, elem(None, None)),
            SzErrorKind::AlreadyPresent,
            "Standardize call element already exists",
        );
        let (_, rec) = add_standardize_call_element(&with_call(), elem(Some(11), None)).unwrap();
        assert_eq!(
            (rec["FELEM_ID"].clone(), rec["EXEC_ORDER"].clone()),
            (json!(11), json!(1))
        );
        assert_err(
            add_standardize_call_element(&mini(json!({"CFG_SFCALL": null})), elem(None, None)),
            SzErrorKind::MissingSection,
            "CFG_SFCALL",
        );
    }

    #[test]
    fn delete_element_paths() {
        let del = |felem_id| DeleteStandardizeCallElementParams {
            ftype_id: 1,
            sfunc_id: 7,
            felem_id,
        };
        assert_kind(
            delete_standardize_call_element(BAD_JSON, del(None)),
            SzErrorKind::JsonParse,
        );
        assert_err(
            delete_standardize_call_element(&with_call(), del(Some(11))),
            SzErrorKind::NotFound,
            "Standardize call element not found",
        );
        assert_err(
            delete_standardize_call_element(&mini(json!({})), del(None)),
            SzErrorKind::NotFound,
            "Standardize call element not found",
        );
        let out = delete_standardize_call_element(&with_call(), del(None)).unwrap();
        assert!(rows(&out, "CFG_SFCALL").is_empty());
    }
}
