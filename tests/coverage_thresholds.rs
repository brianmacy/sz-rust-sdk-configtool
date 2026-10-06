//! Coverage of `sz_configtool_lib::thresholds` (CFG_CFRTN comparison thresholds
//! and CFG_GENERIC_THRESHOLD generic thresholds), exercised against the real
//! template fixture or minimal inline configs. Every error path asserts the
//! exact `SzErrorKind` and message.

mod cov_support;

use cov_support::{BAD_JSON, assert_err, assert_kind, rows, template};
use serde_json::{Value, json};
use sz_configtool_lib::error::{SzErrorKind, ValidationFailure, ValidationReason};
use sz_configtool_lib::thresholds::*;

/// Minimal config: feature NAME (1), comparison function CF (7) with an
/// all-features tier row `FULL` at EXEC_ORDER 3, plan INGEST (1) with one
/// generic threshold `NAME`/all.
fn mini() -> Value {
    json!({"G2_CONFIG": {
        "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
        "CFG_CFUNC": [{"CFUNC_ID": 7, "CFUNC_CODE": "CF"}],
        "CFG_CFRTN": [{"CFRTN_ID": 4, "CFUNC_ID": 7, "FTYPE_ID": 0, "CFUNC_RTNVAL": "FULL",
                       "EXEC_ORDER": 3, "SAME_SCORE": 100}],
        "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
        "CFG_GENERIC_THRESHOLD": [{"GPLAN_ID": 1, "BEHAVIOR": "NAME", "FTYPE_ID": 0,
                                   "CANDIDATE_CAP": 10, "SCORING_CAP": -1, "SEND_TO_REDO": "Yes"}]
    }})
}

/// `mini()` with `section` removed.
fn without(section: &str) -> String {
    let mut v = mini();
    v["G2_CONFIG"].as_object_mut().unwrap().remove(section);
    v.to_string()
}

fn cfg() -> String {
    mini().to_string()
}

// ===========================================================================
// Parameter structs
// ===========================================================================

#[test]
fn comparison_param_constructors_and_try_from() {
    let p = AddComparisonThresholdParams::new("CF", "NAME", "FULL");
    assert_eq!(
        (p.cfunc_code, p.ftype_code, p.cfunc_rtnval, p.exec_order),
        (Some("CF"), Some("NAME"), Some("FULL"), None)
    );
    let full = json!({
        "cfuncCode": "CF", "ftypeCode": "NAME", "cfuncRtnval": "FULL", "execOrder": 1,
        "sameScore": 2, "closeScore": 3, "likelyScore": 4, "plausibleScore": 5,
        "unlikelyScore": 6
    });
    let a = AddComparisonThresholdParams::try_from(&full).unwrap();
    assert_eq!(
        (
            a.exec_order,
            a.same_score,
            a.close_score,
            a.likely_score,
            a.plausible_score,
            a.un_likely_score
        ),
        (Some(1), Some(2), Some(3), Some(4), Some(5), Some(6))
    );
    let s = SetComparisonThresholdParams::try_from(&full).unwrap();
    assert_eq!(
        (
            s.cfunc_code,
            s.exec_order,
            s.same_score,
            s.close_score,
            s.likely_score,
            s.plausible_score,
            s.un_likely_score
        ),
        (
            Some("CF"),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6)
        )
    );
    // Null and absent integer fields are None.
    let nulls = json!({"sameScore": null});
    let s = SetComparisonThresholdParams::try_from(&nulls).unwrap();
    assert_eq!((s.same_score, s.cfunc_code), (None, None));

    // Every integer field rejects a wrongly typed value.
    for key in [
        "execOrder",
        "sameScore",
        "closeScore",
        "likelyScore",
        "plausibleScore",
        "unlikelyScore",
    ] {
        let bad = json!({ key: "1" });
        let msg = format!("{key} must be an integer");
        assert_err(
            AddComparisonThresholdParams::try_from(&bad),
            SzErrorKind::InvalidInput,
            &msg,
        );
        assert_err(
            SetComparisonThresholdParams::try_from(&bad),
            SzErrorKind::InvalidInput,
            &msg,
        );
    }
}

#[test]
fn generic_param_constructors_and_try_from() {
    let p = AddGenericThresholdParams::new("INGEST", "NAME", 1, 2, "Yes");
    assert_eq!(
        (
            p.plan,
            p.behavior,
            p.scoring_cap,
            p.candidate_cap,
            p.send_to_redo,
            p.feature
        ),
        (
            Some("INGEST"),
            Some("NAME"),
            Some(1),
            Some(2),
            Some("Yes"),
            None
        )
    );
    let full = json!({
        "plan": "INGEST", "behavior": "NAME", "scoringCap": 1, "candidateCap": 2,
        "sendToRedo": "No", "feature": "NAME"
    });
    let a = AddGenericThresholdParams::try_from(&full).unwrap();
    assert_eq!(
        (
            a.plan,
            a.behavior,
            a.scoring_cap,
            a.candidate_cap,
            a.send_to_redo,
            a.feature
        ),
        (
            Some("INGEST"),
            Some("NAME"),
            Some(1),
            Some(2),
            Some("No"),
            Some("NAME")
        )
    );
    let s = SetGenericThresholdParams::try_from(&full).unwrap();
    assert_eq!(
        (
            s.plan,
            s.behavior,
            s.feature,
            s.candidate_cap,
            s.scoring_cap,
            s.send_to_redo
        ),
        (
            Some("INGEST"),
            Some("NAME"),
            Some("NAME"),
            Some(2),
            Some(1),
            Some("No")
        )
    );
    for key in ["scoringCap", "candidateCap"] {
        let bad = json!({ key: 1.5 });
        let msg = format!("{key} must be an integer");
        assert_err(
            AddGenericThresholdParams::try_from(&bad),
            SzErrorKind::InvalidInput,
            &msg,
        );
        assert_err(
            SetGenericThresholdParams::try_from(&bad),
            SzErrorKind::InvalidInput,
            &msg,
        );
    }

    let d = DeleteGenericThresholdParams::new("INGEST", "NAME").with_feature("NAME");
    assert_eq!(
        (d.plan, d.behavior, d.feature),
        (Some("INGEST"), Some("NAME"), Some("NAME"))
    );
    let d = DeleteGenericThresholdParams::try_from(&full).unwrap();
    assert_eq!(
        (d.plan, d.behavior, d.feature),
        (Some("INGEST"), Some("NAME"), Some("NAME"))
    );
    let empty = json!({});
    let d = DeleteGenericThresholdParams::try_from(&empty).unwrap();
    assert_eq!((d.plan, d.behavior, d.feature), (None, None, None));
}

// ===========================================================================
// Comparison thresholds (CFG_CFRTN)
// ===========================================================================

#[test]
fn add_comparison_threshold_error_paths() {
    let c = cfg();
    for (params, field) in [
        (
            AddComparisonThresholdParams {
                cfunc_code: None,
                ..AddComparisonThresholdParams::new("CF", "NAME", "X")
            },
            "cfunc_code",
        ),
        (
            AddComparisonThresholdParams {
                ftype_code: None,
                ..AddComparisonThresholdParams::new("CF", "NAME", "X")
            },
            "ftype_code",
        ),
        (
            AddComparisonThresholdParams {
                cfunc_rtnval: None,
                ..AddComparisonThresholdParams::new("CF", "NAME", "X")
            },
            "cfunc_rtnval",
        ),
    ] {
        assert_err(
            add_comparison_threshold(&c, params),
            SzErrorKind::MissingField,
            field,
        );
    }
    assert_kind(
        add_comparison_threshold(
            BAD_JSON,
            AddComparisonThresholdParams::new("CF", "NAME", "X"),
        ),
        SzErrorKind::JsonParse,
    );
    assert_err(
        add_comparison_threshold(&c, AddComparisonThresholdParams::new("NOPE", "NAME", "X")),
        SzErrorKind::NotFound,
        "Comparison function 'NOPE' not found",
    );
    assert_err(
        add_comparison_threshold(&c, AddComparisonThresholdParams::new("CF", "NOPE", "X")),
        SzErrorKind::NotFound,
        "Feature 'NOPE' not found",
    );
    assert_err(
        add_comparison_threshold(
            &without("CFG_CFRTN"),
            AddComparisonThresholdParams::new("CF", "NAME", "X"),
        ),
        SzErrorKind::MissingSection,
        "CFG_CFRTN",
    );
    assert_err(
        add_comparison_threshold(&c, AddComparisonThresholdParams::new("CF", "ALL", "full")),
        SzErrorKind::AlreadyExists,
        "Comparison threshold: CF+ALL+FULL",
    );
    let taken = AddComparisonThresholdParams {
        exec_order: Some(3),
        ..AddComparisonThresholdParams::new("CF", "all", "OTHER")
    };
    assert_err(
        add_comparison_threshold(&c, taken),
        SzErrorKind::AlreadyExists,
        "The specified EXEC_ORDER 3 is already taken",
    );
}

#[test]
fn add_comparison_threshold_tier_reuse_and_allocation() {
    // Per-feature override reuses the all-features tier order.
    let params = AddComparisonThresholdParams {
        exec_order: Some(9),
        same_score: Some(1),
        close_score: Some(2),
        likely_score: Some(3),
        plausible_score: Some(4),
        un_likely_score: Some(5),
        ..AddComparisonThresholdParams::new("CF", "NAME", "full")
    };
    let out = add_comparison_threshold(&cfg(), params).unwrap();
    let row = rows(&out, "CFG_CFRTN").pop().unwrap();
    assert_eq!(
        row,
        json!({"CFRTN_ID": 5, "CFUNC_ID": 7, "FTYPE_ID": 1, "CFUNC_RTNVAL": "FULL",
               "EXEC_ORDER": 3, "SAME_SCORE": 1, "CLOSE_SCORE": 2, "LIKELY_SCORE": 3,
               "PLAUSIBLE_SCORE": 4, "UN_LIKELY_SCORE": 5})
    );
    // A new return value without a tier allocates the next order.
    let out = add_comparison_threshold(
        &cfg(),
        AddComparisonThresholdParams::new("CF", "all", "new"),
    )
    .unwrap();
    assert_eq!(rows(&out, "CFG_CFRTN").pop().unwrap()["EXEC_ORDER"], 4);
    // Real template: a new tier on STR_COMP.
    let out = add_comparison_threshold(
        &template(),
        AddComparisonThresholdParams::new("STR_COMP", "NAME", "NEW_SCORE"),
    )
    .unwrap();
    assert_eq!(
        rows(&out, "CFG_CFRTN").len(),
        rows(&template(), "CFG_CFRTN").len() + 1
    );
}

#[test]
fn add_comparison_threshold_by_id_paths() {
    let add = |config: &str, rtn: &str, ftype: Option<i64>, order: Option<i64>| {
        add_comparison_threshold_by_id(
            config,
            7,
            ftype,
            rtn,
            order,
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
        )
    };
    assert_kind(add(BAD_JSON, "X", None, None), SzErrorKind::JsonParse);
    assert_err(
        add(&without("CFG_CFRTN"), "X", None, None),
        SzErrorKind::MissingSection,
        "CFG_CFRTN",
    );
    assert_err(
        add(&cfg(), "full", None, None),
        SzErrorKind::AlreadyExists,
        "Comparison threshold already exists",
    );
    assert_err(
        add(&cfg(), "X", Some(0), Some(3)),
        SzErrorKind::AlreadyExists,
        "The specified EXEC_ORDER 3 is already taken",
    );
    let out = add(&cfg(), "full", Some(1), None).unwrap();
    let row = rows(&out, "CFG_CFRTN").pop().unwrap();
    assert_eq!(
        (
            row["FTYPE_ID"].clone(),
            row["EXEC_ORDER"].clone(),
            row["UN_LIKELY_SCORE"].clone()
        ),
        (json!(1), json!(3), json!(5))
    );
}

#[test]
fn set_comparison_threshold_by_id_paths() {
    let set = |config: &str, id: i64| {
        set_comparison_threshold_by_id(config, id, Some(1), Some(2), Some(3), Some(4), Some(5))
    };
    assert_kind(set(BAD_JSON, 4), SzErrorKind::JsonParse);
    assert_err(
        set(&without("CFG_CFRTN"), 4),
        SzErrorKind::MissingSection,
        "CFG_CFRTN",
    );
    assert_err(
        set(&cfg(), 99),
        SzErrorKind::NotFound,
        "Comparison threshold ID: 99",
    );
    let out = set(&cfg(), 4).unwrap();
    let row = &rows(&out, "CFG_CFRTN")[0];
    assert_eq!(
        [
            &row["SAME_SCORE"],
            &row["CLOSE_SCORE"],
            &row["LIKELY_SCORE"],
            &row["PLAUSIBLE_SCORE"],
            &row["UN_LIKELY_SCORE"]
        ],
        [&json!(1), &json!(2), &json!(3), &json!(4), &json!(5)]
    );
    // All-None leaves the row untouched.
    let out = set_comparison_threshold_by_id(&cfg(), 4, None, None, None, None, None).unwrap();
    assert_eq!(rows(&out, "CFG_CFRTN"), rows(&cfg(), "CFG_CFRTN"));
}

#[test]
fn delete_comparison_threshold_by_id_paths() {
    assert_kind(
        delete_comparison_threshold_by_id(BAD_JSON, 4),
        SzErrorKind::JsonParse,
    );
    for config in [without("CFG_CFRTN"), cfg()] {
        assert_err(
            delete_comparison_threshold_by_id(&config, 99),
            SzErrorKind::NotFound,
            "Comparison threshold ID: 99",
        );
    }
    let out = delete_comparison_threshold_by_id(&cfg(), 4).unwrap();
    assert!(rows(&out, "CFG_CFRTN").is_empty());
}

#[test]
fn delete_comparison_threshold_paths() {
    assert_kind(
        delete_comparison_threshold(BAD_JSON, "CF", "all", "FULL"),
        SzErrorKind::JsonParse,
    );
    assert_err(
        delete_comparison_threshold(&cfg(), "NOPE", "all", "FULL"),
        SzErrorKind::NotFound,
        "Comparison function 'NOPE' not found",
    );
    assert_err(
        delete_comparison_threshold(&cfg(), "CF", "NOPE", "FULL"),
        SzErrorKind::NotFound,
        "Feature 'NOPE' not found",
    );
    assert_err(
        delete_comparison_threshold(&without("CFG_CFRTN"), "CF", "all", "FULL"),
        SzErrorKind::MissingSection,
        "CFG_CFRTN",
    );
    assert_err(
        delete_comparison_threshold(&cfg(), "CF", "NAME", "FULL"),
        SzErrorKind::NotFound,
        "Comparison threshold for cfunc='CF', ftype='NAME', rtnval='FULL'",
    );
    let out = delete_comparison_threshold(&cfg(), "CF", "ALL", "full").unwrap();
    assert!(rows(&out, "CFG_CFRTN").is_empty());
}

#[test]
fn set_comparison_threshold_paths() {
    let base = || SetComparisonThresholdParams {
        cfunc_code: Some("CF"),
        ftype_code: Some("all"),
        cfunc_rtnval: Some("full"),
        ..Default::default()
    };
    for (params, field) in [
        (
            SetComparisonThresholdParams {
                cfunc_code: None,
                ..base()
            },
            "cfunc_code",
        ),
        (
            SetComparisonThresholdParams {
                ftype_code: None,
                ..base()
            },
            "ftype_code",
        ),
        (
            SetComparisonThresholdParams {
                cfunc_rtnval: None,
                ..base()
            },
            "cfunc_rtnval",
        ),
    ] {
        assert_err(
            set_comparison_threshold(&cfg(), params),
            SzErrorKind::MissingField,
            field,
        );
    }
    assert_kind(
        set_comparison_threshold(BAD_JSON, base()),
        SzErrorKind::JsonParse,
    );
    assert_err(
        set_comparison_threshold(
            &cfg(),
            SetComparisonThresholdParams {
                cfunc_code: Some("NOPE"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Comparison function 'NOPE' not found",
    );
    assert_err(
        set_comparison_threshold(
            &cfg(),
            SetComparisonThresholdParams {
                ftype_code: Some("NOPE"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Feature 'NOPE' not found",
    );
    assert_err(
        set_comparison_threshold(&without("CFG_CFRTN"), base()),
        SzErrorKind::MissingSection,
        "CFG_CFRTN",
    );
    assert_err(
        set_comparison_threshold(
            &cfg(),
            SetComparisonThresholdParams {
                ftype_code: Some("NAME"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Comparison threshold: CF+NAME+full",
    );
    let params = SetComparisonThresholdParams {
        exec_order: Some(8),
        same_score: Some(1),
        close_score: Some(2),
        likely_score: Some(3),
        plausible_score: Some(4),
        un_likely_score: Some(5),
        ..base()
    };
    let out = set_comparison_threshold(&cfg(), params).unwrap();
    assert_eq!(
        rows(&out, "CFG_CFRTN")[0],
        json!({"CFRTN_ID": 4, "CFUNC_ID": 7, "FTYPE_ID": 0, "CFUNC_RTNVAL": "FULL",
               "EXEC_ORDER": 8, "SAME_SCORE": 1, "CLOSE_SCORE": 2, "LIKELY_SCORE": 3,
               "PLAUSIBLE_SCORE": 4, "UN_LIKELY_SCORE": 5})
    );
    // Per-feature row on the real template.
    let out = set_comparison_threshold(
        &template(),
        SetComparisonThresholdParams {
            cfunc_code: Some("STR_COMP"),
            ftype_code: Some("all"),
            cfunc_rtnval: Some("FULL_SCORE"),
            same_score: Some(42),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(rows(&out, "CFG_CFRTN")[0]["SAME_SCORE"], 42);
    let mut v = mini();
    v["G2_CONFIG"]["CFG_CFRTN"][0]["FTYPE_ID"] = json!(1);
    let out = set_comparison_threshold(
        &v.to_string(),
        SetComparisonThresholdParams {
            ftype_code: Some("NAME"),
            same_score: Some(7),
            ..base()
        },
    )
    .unwrap();
    assert_eq!(rows(&out, "CFG_CFRTN")[0]["SAME_SCORE"], 7);
    // Only a non-leading score: the untouched SAME_SCORE keeps its value.
    let out = set_comparison_threshold(
        &cfg(),
        SetComparisonThresholdParams {
            close_score: Some(55),
            ..base()
        },
    )
    .unwrap();
    let row = &rows(&out, "CFG_CFRTN")[0];
    assert_eq!(
        (row["SAME_SCORE"].clone(), row["CLOSE_SCORE"].clone()),
        (json!(100), json!(55))
    );
}

#[test]
fn list_comparison_thresholds_paths() {
    assert_kind(list_comparison_thresholds(BAD_JSON), SzErrorKind::JsonParse);
    for section in ["CFG_CFRTN", "CFG_CFUNC", "CFG_FTYPE"] {
        assert_err(
            list_comparison_thresholds(&without(section)),
            SzErrorKind::MissingSection,
            section,
        );
    }
    let mut v = mini();
    v["G2_CONFIG"]["CFG_CFRTN"] = json!([
        {"CFRTN_ID": 9, "CFUNC_ID": 7, "FTYPE_ID": 1, "CFUNC_RTNVAL": "B"},
        {"CFRTN_ID": 2, "CFUNC_ID": 8, "FTYPE_ID": 5},
        {}
    ]);
    let items = list_comparison_thresholds(&v.to_string()).unwrap();
    assert_eq!(
        items[0],
        json!({"id": 0, "function": "unknown", "returnOrder": 0, "scoreName": "",
               "feature": "all", "sameScore": 0, "closeScore": 0, "likelyScore": 0,
               "plausibleScore": 0, "unlikelyScore": 0})
    );
    assert_eq!(
        (items[1]["function"].clone(), items[1]["feature"].clone()),
        (json!("CF"), json!("NAME"))
    );
    assert_eq!(items[2]["feature"], "unknown");
    let items = list_comparison_thresholds(&template()).unwrap();
    assert_eq!(items.len(), rows(&template(), "CFG_CFRTN").len());
    assert_eq!(items[0]["function"], "STR_COMP");
}

// ===========================================================================
// Generic thresholds (CFG_GENERIC_THRESHOLD)
// ===========================================================================

#[test]
fn generic_threshold_check_serializes_every_variant() {
    let ser = |c: &GenericThresholdCheck| serde_json::to_value(c).unwrap();
    let schema = GENERIC_THRESHOLD_CHECK_SCHEMA;
    assert_eq!(
        ser(&GenericThresholdCheck::Ok),
        json!({"schema": schema, "result": "ok"})
    );
    assert_eq!(
        ser(&GenericThresholdCheck::Duplicate),
        json!({"schema": schema, "result": "duplicate"})
    );
    for (which, tag) in [
        (GenericThresholdRef::Plan, "plan"),
        (GenericThresholdRef::Feature, "feature"),
    ] {
        assert_eq!(
            ser(&GenericThresholdCheck::NotFound {
                which,
                value: "X".into()
            }),
            json!({"schema": schema, "result": "notFound", "which": tag, "value": "X"})
        );
    }
    let invalid = GenericThresholdCheck::Invalid(vec![
        ValidationFailure::new(
            "behavior",
            ValidationReason::UnknownReferenceCode,
            Some("B".into()),
        ),
        ValidationFailure::new("sendToRedo", ValidationReason::OutOfDomain, None),
    ]);
    assert_eq!(
        ser(&invalid),
        json!({"schema": schema, "result": "invalid", "failures": [
            {"field": "behavior", "reasonCode": "UNKNOWN_REFERENCE_CODE", "offendingValue": "B"},
            {"field": "sendToRedo", "reasonCode": "OUT_OF_DOMAIN", "offendingValue": null}
        ]})
    );
}

#[test]
fn validate_generic_threshold_stages() {
    let c = cfg();
    assert_kind(
        validate_generic_threshold(BAD_JSON, "INGEST", "NAME", "Yes", None),
        SzErrorKind::JsonParse,
    );
    assert_eq!(
        validate_generic_threshold(&without("CFG_GPLAN"), "ingest", "NAME", "Yes", None).unwrap(),
        GenericThresholdCheck::NotFound {
            which: GenericThresholdRef::Plan,
            value: "INGEST".into()
        }
    );
    assert_eq!(
        validate_generic_threshold(&without("CFG_FTYPE"), "INGEST", "NAME", "Yes", Some("name"))
            .unwrap(),
        GenericThresholdCheck::NotFound {
            which: GenericThresholdRef::Feature,
            value: "NAME".into()
        }
    );
    assert_eq!(
        validate_generic_threshold(&c, "INGEST", "name", "Yes", Some("all")).unwrap(),
        GenericThresholdCheck::Duplicate
    );
    assert_eq!(
        validate_generic_threshold(
            &without("CFG_GENERIC_THRESHOLD"),
            "INGEST",
            "NAME",
            "Yes",
            None
        )
        .unwrap(),
        GenericThresholdCheck::Ok
    );
    assert_eq!(
        validate_generic_threshold(&c, "INGEST", "F1", "Yes", Some("NAME")).unwrap(),
        GenericThresholdCheck::Ok
    );
    let check = validate_generic_threshold(&c, "INGEST", "BOGUS", "maybe", None).unwrap();
    let GenericThresholdCheck::Invalid(failures) = check else {
        panic!("expected Invalid, got {check:?}");
    };
    let fields: Vec<_> = failures.iter().map(|f| (f.field, f.reason_code)).collect();
    assert_eq!(
        fields,
        vec![
            ("behavior", ValidationReason::UnknownReferenceCode),
            ("sendToRedo", ValidationReason::OutOfDomain)
        ]
    );
}

#[test]
fn add_generic_threshold_paths() {
    let all_none = AddGenericThresholdParams {
        plan: None,
        behavior: None,
        scoring_cap: None,
        candidate_cap: None,
        send_to_redo: None,
        feature: None,
    };
    assert_err(
        add_generic_threshold(&cfg(), all_none),
        SzErrorKind::MissingField,
        "plan, behavior, scoring_cap, candidate_cap, send_to_redo",
    );
    let partial = AddGenericThresholdParams {
        candidate_cap: None,
        ..AddGenericThresholdParams::new("INGEST", "F1", 1, 2, "Yes")
    };
    assert_err(
        add_generic_threshold(&cfg(), partial),
        SzErrorKind::MissingField,
        "candidate_cap",
    );
    let p = || AddGenericThresholdParams::new("ingest", "f1", 5, 6, "no");
    assert_kind(add_generic_threshold(BAD_JSON, p()), SzErrorKind::JsonParse);
    assert_err(
        add_generic_threshold(&without("CFG_GPLAN"), p()),
        SzErrorKind::MissingSection,
        "CFG_GPLAN",
    );
    assert_err(
        add_generic_threshold(
            &cfg(),
            AddGenericThresholdParams::new("NOPE", "F1", 1, 2, "Yes"),
        ),
        SzErrorKind::NotFound,
        "Generic plan: NOPE",
    );
    let with_feature = |f| AddGenericThresholdParams {
        feature: Some(f),
        ..p()
    };
    assert_err(
        add_generic_threshold(&without("CFG_FTYPE"), with_feature("NAME")),
        SzErrorKind::MissingSection,
        "CFG_FTYPE",
    );
    assert_err(
        add_generic_threshold(&cfg(), with_feature("nope")),
        SzErrorKind::NotFound,
        "Feature: NOPE",
    );
    assert_err(
        add_generic_threshold(&without("CFG_GENERIC_THRESHOLD"), p()),
        SzErrorKind::MissingSection,
        "CFG_GENERIC_THRESHOLD",
    );
    assert_err(
        add_generic_threshold(
            &cfg(),
            AddGenericThresholdParams::new("INGEST", "name", 1, 2, "Yes"),
        ),
        SzErrorKind::AlreadyExists,
        "Generic threshold: plan=INGEST, behavior=NAME, feature=ALL",
    );
    let err = add_generic_threshold(
        &cfg(),
        AddGenericThresholdParams::new("INGEST", "BOGUS", 1, 2, "Yes"),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::ValidationErrors);
    assert_eq!(
        err.validation_failures().unwrap(),
        &[ValidationFailure::new(
            "behavior",
            ValidationReason::UnknownReferenceCode,
            Some("BOGUS".into())
        )]
    );
    let out = add_generic_threshold(&cfg(), with_feature("name")).unwrap();
    assert_eq!(
        rows(&out, "CFG_GENERIC_THRESHOLD").pop().unwrap(),
        json!({"GPLAN_ID": 1, "BEHAVIOR": "F1", "FTYPE_ID": 1, "CANDIDATE_CAP": 6,
               "SCORING_CAP": 5, "SEND_TO_REDO": "No"})
    );
}

#[test]
fn delete_generic_threshold_paths() {
    let missing_plan = DeleteGenericThresholdParams {
        plan: None,
        ..DeleteGenericThresholdParams::new("INGEST", "NAME")
    };
    assert_err(
        delete_generic_threshold(&cfg(), missing_plan),
        SzErrorKind::MissingField,
        "plan",
    );
    let missing_behavior = DeleteGenericThresholdParams {
        behavior: None,
        ..DeleteGenericThresholdParams::new("INGEST", "NAME")
    };
    assert_err(
        delete_generic_threshold(&cfg(), missing_behavior),
        SzErrorKind::MissingField,
        "behavior",
    );
    let p = || DeleteGenericThresholdParams::new("ingest", "name");
    assert_kind(
        delete_generic_threshold(BAD_JSON, p()),
        SzErrorKind::JsonParse,
    );
    assert_err(
        delete_generic_threshold(&cfg(), DeleteGenericThresholdParams::new("NOPE", "NAME")),
        SzErrorKind::NotFound,
        "Generic plan 'NOPE' not found",
    );
    assert_err(
        delete_generic_threshold(&without("CFG_FTYPE"), p().with_feature("NAME")),
        SzErrorKind::MissingSection,
        "CFG_FTYPE",
    );
    assert_err(
        delete_generic_threshold(&cfg(), p().with_feature("nope")),
        SzErrorKind::NotFound,
        "Feature: NOPE",
    );
    let not_found = "Generic threshold not found: GPLAN_ID=1, behavior=NAME, feature=ALL";
    assert_err(
        delete_generic_threshold(&without("CFG_GENERIC_THRESHOLD"), p()),
        SzErrorKind::NotFound,
        not_found,
    );
    assert_err(
        delete_generic_threshold(&cfg(), p().with_feature("NAME")),
        SzErrorKind::NotFound,
        "Generic threshold not found: GPLAN_ID=1, behavior=NAME, feature=NAME",
    );
    let out = delete_generic_threshold(&cfg(), p()).unwrap();
    assert!(rows(&out, "CFG_GENERIC_THRESHOLD").is_empty());
    assert_err(
        delete_generic_threshold(&out, p()),
        SzErrorKind::NotFound,
        not_found,
    );
}

#[test]
fn set_generic_threshold_paths() {
    let base = || SetGenericThresholdParams {
        plan: Some("ingest"),
        behavior: Some("name"),
        feature: None,
        candidate_cap: None,
        scoring_cap: None,
        send_to_redo: None,
    };
    assert_err(
        set_generic_threshold(
            &cfg(),
            SetGenericThresholdParams {
                plan: None,
                ..base()
            },
        ),
        SzErrorKind::MissingField,
        "plan",
    );
    assert_err(
        set_generic_threshold(
            &cfg(),
            SetGenericThresholdParams {
                behavior: None,
                ..base()
            },
        ),
        SzErrorKind::MissingField,
        "behavior",
    );
    assert_kind(
        set_generic_threshold(BAD_JSON, base()),
        SzErrorKind::JsonParse,
    );
    assert_err(
        set_generic_threshold(
            &cfg(),
            SetGenericThresholdParams {
                plan: Some("NOPE"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Generic plan 'NOPE' not found",
    );
    assert_err(
        set_generic_threshold(
            &cfg(),
            SetGenericThresholdParams {
                feature: Some("NOPE"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Feature 'NOPE' not found",
    );
    assert_err(
        set_generic_threshold(&without("CFG_GENERIC_THRESHOLD"), base()),
        SzErrorKind::MissingSection,
        "CFG_GENERIC_THRESHOLD",
    );
    assert_err(
        set_generic_threshold(
            &cfg(),
            SetGenericThresholdParams {
                feature: Some("NAME"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        "Generic threshold not found: GPLAN_ID=1, BEHAVIOR=NAME, FTYPE_ID=1",
    );
    let err = set_generic_threshold(
        &cfg(),
        SetGenericThresholdParams {
            send_to_redo: Some("maybe"),
            ..base()
        },
    )
    .unwrap_err();
    assert_eq!(
        err.validation_failures().unwrap(),
        &[ValidationFailure::new(
            "sendToRedo",
            ValidationReason::OutOfDomain,
            Some("maybe".into())
        )]
    );
    let out = set_generic_threshold(
        &cfg(),
        SetGenericThresholdParams {
            candidate_cap: Some(11),
            scoring_cap: Some(12),
            send_to_redo: Some("no"),
            feature: Some("all"),
            ..base()
        },
    )
    .unwrap();
    assert_eq!(
        rows(&out, "CFG_GENERIC_THRESHOLD")[0],
        json!({"GPLAN_ID": 1, "BEHAVIOR": "NAME", "FTYPE_ID": 0, "CANDIDATE_CAP": 11,
               "SCORING_CAP": 12, "SEND_TO_REDO": "No"})
    );
    // No updates requested: row unchanged.
    let out = set_generic_threshold(&cfg(), base()).unwrap();
    assert_eq!(
        rows(&out, "CFG_GENERIC_THRESHOLD"),
        rows(&cfg(), "CFG_GENERIC_THRESHOLD")
    );
}

#[test]
fn list_generic_thresholds_paths() {
    assert_kind(list_generic_thresholds(BAD_JSON), SzErrorKind::JsonParse);
    for section in ["CFG_GENERIC_THRESHOLD", "CFG_GPLAN", "CFG_FTYPE"] {
        assert_err(
            list_generic_thresholds(&without(section)),
            SzErrorKind::MissingSection,
            section,
        );
    }
    let mut v = mini();
    v["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"] = json!([
        {"GPLAN_ID": 1, "BEHAVIOR": "BOGUS", "FTYPE_ID": 5},
        {"GPLAN_ID": 1, "BEHAVIOR": "NAME", "FTYPE_ID": 1, "CANDIDATE_CAP": 3,
         "SCORING_CAP": 4, "SEND_TO_REDO": "No"},
        {"GPLAN_ID": 2}
    ]);
    let items = list_generic_thresholds(&v.to_string()).unwrap();
    assert_eq!(
        items[0],
        json!({"id": 1, "plan": "INGEST", "behavior": "NAME", "feature": "NAME",
               "candidateCap": 3, "scoringCap": 4, "sendToRedo": "No"})
    );
    assert_eq!(
        items[1],
        json!({"id": 1, "plan": "INGEST", "behavior": "BOGUS", "feature": "unknown",
               "candidateCap": 0, "scoringCap": 0, "sendToRedo": ""})
    );
    assert_eq!(
        (items[2]["plan"].clone(), items[2]["feature"].clone()),
        (json!("unknown"), json!("all"))
    );
    let items = list_generic_thresholds(&template()).unwrap();
    assert_eq!(
        items.len(),
        rows(&template(), "CFG_GENERIC_THRESHOLD").len()
    );
}
