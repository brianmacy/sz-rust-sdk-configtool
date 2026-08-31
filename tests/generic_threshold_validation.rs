//! Generic-threshold validation exercised against the REAL Senzing v4 template
//! (`tests/fixtures/g2config_template.json`), not synthetic config.
//!
//! Covers the v0.7.0 STEP B hardening (#49/#50 + comparison-cap fold-in):
//!  - `add_generic_threshold` rejects a bogus BEHAVIOR code via the canonical
//!    17-code domain (`behavior_domain::BEHAVIOR_CODES`).
//!  - `add_generic_threshold` succeeds for a genuinely new (plan, behavior,
//!    feature) tuple on the shipped template.
//!  - Every distinct BEHAVIOR already shipped in the template passes the new
//!    validator (the tightened check is a no-op on real data).
//!  - The strict `optional_i64` boundary rejects a present-but-wrong-type cap
//!    (`{"candidateCap": "500"}`) instead of silently coercing it to `None`
//!    and turning the update into a no-op.

use serde_json::{Value, json};
use sz_configtool_lib::error::{SzErrorKind, ValidationReason};
use sz_configtool_lib::thresholds::{
    AddGenericThresholdParams, GenericThresholdCheck, GenericThresholdRef,
    SetGenericThresholdParams, add_generic_threshold, set_generic_threshold,
    validate_generic_threshold,
};

fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

/// (a) A bogus behaviour code on an existing plan is now a structured
/// `ValidationErrors` with a single `behavior`/`UnknownReferenceCode` failure
/// echoing the offending (upper-cased) code.
#[test]
fn add_generic_threshold_rejects_bogus_behavior_on_template() {
    let config = template();

    // INGEST is a real plan; "BOGUS" is not one of the canonical behaviour codes.
    let params = AddGenericThresholdParams::new("INGEST", "BOGUS", 20, 10, "No");
    let err = add_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::ValidationErrors);

    let failures = err.validation_failures().expect("structured failures");
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].field, "behavior");
    assert_eq!(
        failures[0].reason_code,
        ValidationReason::UnknownReferenceCode
    );
    assert_eq!(failures[0].offending_value.as_deref(), Some("BOGUS"));
}

/// (a2) BOTH a bad behaviour AND a bad sendToRedo aggregate into TWO failures in
/// canonical order [behavior, sendToRedo] — proving the aggregate survives (the
/// old lossy `join("; ")` is gone).
#[test]
fn add_generic_threshold_aggregates_behavior_and_send_to_redo_on_template() {
    let config = template();

    let params = AddGenericThresholdParams::new("INGEST", "BOGUS", 20, 10, "perhaps");
    let err = add_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::ValidationErrors);

    let failures = err.validation_failures().expect("structured failures");
    assert_eq!(failures.len(), 2, "both fields should fail: {failures:?}");
    // Canonical order: behavior first, sendToRedo second.
    assert_eq!(failures[0].field, "behavior");
    assert_eq!(
        failures[0].reason_code,
        ValidationReason::UnknownReferenceCode
    );
    assert_eq!(failures[1].field, "sendToRedo");
    assert_eq!(failures[1].reason_code, ValidationReason::OutOfDomain);
    assert_eq!(failures[1].offending_value.as_deref(), Some("perhaps"));
}

/// (a3) A bad sendToRedo ONLY (valid behaviour) yields a single
/// `sendToRedo`/`OutOfDomain` failure.
#[test]
fn add_generic_threshold_rejects_bad_send_to_redo_only_on_template() {
    let config = template();

    // Use the known-absent (SEARCH, NAME, NAME-feature) tuple so the duplicate
    // check (which fires before field validation) does not pre-empt it.
    let mut params = AddGenericThresholdParams::new("SEARCH", "NAME", 20, 10, "perhaps");
    params.feature = Some("NAME");
    let err = add_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::ValidationErrors);

    let failures = err.validation_failures().expect("structured failures");
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].field, "sendToRedo");
    assert_eq!(failures[0].reason_code, ValidationReason::OutOfDomain);
}

/// (b) A genuinely new (plan, behavior, feature) tuple must be added. The
/// template ships no `(SEARCH, NAME, NAME-feature)` row, so this is a real add.
#[test]
fn add_generic_threshold_succeeds_for_new_tuple_on_template() {
    let config = template();

    let before: Value = serde_json::from_str(&config).unwrap();
    let before_len = before["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .unwrap()
        .len();

    // SEARCH plan + NAME behaviour + NAME feature (FTYPE_ID 1) is absent.
    let mut params = AddGenericThresholdParams::new("SEARCH", "NAME", 15, 25, "yes");
    params.feature = Some("NAME");
    let modified = add_generic_threshold(&config, params).unwrap();

    let after: Value = serde_json::from_str(&modified).unwrap();
    let rows = after["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .unwrap();
    assert_eq!(rows.len(), before_len + 1);

    // The new row carries the NAME feature (FTYPE_ID 1) and canonical redo.
    let new_row = rows
        .iter()
        .find(|r| {
            r["GPLAN_ID"].as_i64() == Some(2)
                && r["BEHAVIOR"].as_str() == Some("NAME")
                && r["FTYPE_ID"].as_i64() == Some(1)
        })
        .expect("newly added SEARCH/NAME/NAME row must exist");
    assert_eq!(new_row["CANDIDATE_CAP"], json!(25));
    assert_eq!(new_row["SCORING_CAP"], json!(15));
    // Lower-case "yes" input canonicalises to title-case "Yes".
    assert_eq!(new_row["SEND_TO_REDO"], json!("Yes"));
}

/// (c) The tightened behaviour validator must be a no-op on shipped data: every
/// distinct BEHAVIOR already in the template is a recognised canonical code.
#[test]
fn every_template_generic_threshold_behavior_is_canonical() {
    let config = template();
    let value: Value = serde_json::from_str(&config).unwrap();
    let rows = value["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .unwrap();

    let mut behaviors: Vec<&str> = rows.iter().filter_map(|r| r["BEHAVIOR"].as_str()).collect();
    behaviors.sort_unstable();
    behaviors.dedup();
    assert!(!behaviors.is_empty(), "template must have threshold rows");

    for b in behaviors {
        assert!(
            sz_configtool_lib::behavior_domain::behavior_position(b).is_some(),
            "shipped BEHAVIOR '{b}' must pass the canonical 17-code validator"
        );
    }
}

/// (d) A present-but-wrong-type cap must now surface as `InvalidInput` at the
/// JSON boundary (`SetGenericThresholdParams::try_from`) rather than being
/// silently dropped to `None` and turning the update into a no-op.
#[test]
fn set_generic_threshold_wrong_typed_cap_errors_on_template() {
    // A string where an integer is required.
    let updates = json!({"candidateCap": "500"});
    let err = SetGenericThresholdParams::try_from(&updates).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);

    // Contrast: a correctly-typed cap parses and actually mutates a real
    // template row (proving the previous case was a genuine rejection, not a
    // blanket failure). INGEST/NAME/all (FTYPE_ID 0) exists in the template.
    let config = template();
    let ok_updates = json!({"candidateCap": 500});
    let mut params = SetGenericThresholdParams::try_from(&ok_updates).unwrap();
    params.plan = Some("INGEST");
    params.behavior = Some("NAME");
    let modified = set_generic_threshold(&config, params).unwrap();

    let after: Value = serde_json::from_str(&modified).unwrap();
    let row = after["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| {
            r["GPLAN_ID"].as_i64() == Some(1)
                && r["BEHAVIOR"].as_str() == Some("NAME")
                && r["FTYPE_ID"].as_i64() == Some(0)
        })
        .expect("INGEST/NAME/all row must exist");
    assert_eq!(row["CANDIDATE_CAP"], json!(500));
}

/// (d2) A boolean cap must be rejected as a scalar `InvalidInput` at the strict
/// `optional_i64` boundary (Python accepts `isinstance(False, int)`; we
/// deliberately do NOT — Ant ruling 17/08/2026). It must NOT become a
/// `ValidationErrors`.
#[test]
fn set_generic_threshold_bool_cap_rejected_as_scalar_invalid_input() {
    let updates = json!({"candidateCap": true});
    let err = SetGenericThresholdParams::try_from(&updates).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);
    assert!(
        err.validation_failures().is_none(),
        "a bad cap is a scalar InvalidInput, never a ValidationErrors aggregate"
    );
}

// ===== SET path parity (Python `do_setGenericThreshold`) =====
//
// Python looks the row up by (plan, behavior, feature) BEFORE running
// `validateGenericThreshold` on the merged record, so an unknown behaviour
// (part of the lookup KEY) surfaces as a row NotFound — it is NOT re-validated
// as a reference code. Only `sendToRedo` is genuinely re-validated, and it
// aggregates into `ValidationErrors` for a uniform surface with ADD.

/// SET with an unknown behaviour hits the row lookup and returns `NotFound`
/// (behaviour is a key, not a re-validated reference code).
#[test]
fn set_generic_threshold_unknown_behavior_is_not_found_on_template() {
    let config = template();
    let params = SetGenericThresholdParams {
        plan: Some("INGEST"),
        behavior: Some("BOGUS"),
        feature: None,
        candidate_cap: Some(5),
        scoring_cap: None,
        send_to_redo: None,
    };
    let err = set_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
}

/// SET with a bad `sendToRedo` on a real row aggregates into `ValidationErrors`.
#[test]
fn set_generic_threshold_bad_send_to_redo_is_validation_errors_on_template() {
    let config = template();
    // INGEST/NAME/all (FTYPE_ID 0) exists in the template.
    let params = SetGenericThresholdParams {
        plan: Some("INGEST"),
        behavior: Some("NAME"),
        feature: None,
        candidate_cap: None,
        scoring_cap: None,
        send_to_redo: Some("perhaps"),
    };
    let err = set_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::ValidationErrors);
    let failures = err.validation_failures().unwrap();
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].field, "sendToRedo");
    assert_eq!(failures[0].reason_code, ValidationReason::OutOfDomain);
}

/// SET ordering: a missing row (behaviour is the lookup KEY) wins over a bad
/// `sendToRedo`. Pins that the row lookup runs BEFORE `sendToRedo` validation —
/// matching Python's lookup-by-(plan,behavior,feature) before
/// `validateGenericThreshold`. A regression reordering these would return
/// `ValidationErrors` instead of `NotFound`.
#[test]
fn set_generic_threshold_missing_row_wins_over_bad_send_to_redo_on_template() {
    let config = template();
    let params = SetGenericThresholdParams {
        plan: Some("INGEST"),
        behavior: Some("BOGUS"), // no such (plan, behavior, feature) row
        feature: None,
        candidate_cap: None,
        scoring_cap: None,
        send_to_redo: Some("perhaps"), // also invalid, but must be suppressed
    };
    let err = set_generic_threshold(&config, params).unwrap_err();
    assert_eq!(
        err.kind(),
        SzErrorKind::NotFound,
        "missing row must win over bad sendToRedo (row lookup precedes redo validation)"
    );
    assert!(
        err.validation_failures().is_none(),
        "must not surface sendToRedo as a validation failure when the row is missing"
    );
}

/// SET happy path still mutates a real template row.
#[test]
fn set_generic_threshold_happy_path_mutates_on_template() {
    let config = template();
    let params = SetGenericThresholdParams {
        plan: Some("INGEST"),
        behavior: Some("NAME"),
        feature: None,
        candidate_cap: None,
        scoring_cap: None,
        send_to_redo: Some("no"),
    };
    let modified = set_generic_threshold(&config, params).unwrap();
    let after: Value = serde_json::from_str(&modified).unwrap();
    let row = after["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| {
            r["GPLAN_ID"].as_i64() == Some(1)
                && r["BEHAVIOR"].as_str() == Some("NAME")
                && r["FTYPE_ID"].as_i64() == Some(0)
        })
        .expect("INGEST/NAME/all row must exist");
    // Lower-case "no" canonicalises to title-case "No".
    assert_eq!(row["SEND_TO_REDO"], json!("No"));
}

// ===== validate_generic_threshold (the CLI orchestration surface) =====

/// Plan lookup is fatal-first -> NotFound{Plan}.
#[test]
fn validate_generic_threshold_not_found_plan_on_template() {
    let config = template();
    let check = validate_generic_threshold(&config, "NOPLAN", "NAME", "Yes", None).unwrap();
    assert_eq!(
        check,
        GenericThresholdCheck::NotFound {
            which: GenericThresholdRef::Plan,
            value: "NOPLAN".to_string(),
        }
    );
}

/// Feature lookup is fatal-first -> NotFound{Feature}.
#[test]
fn validate_generic_threshold_not_found_feature_on_template() {
    let config = template();
    let check =
        validate_generic_threshold(&config, "INGEST", "NAME", "Yes", Some("NOFEATURE")).unwrap();
    assert_eq!(
        check,
        GenericThresholdCheck::NotFound {
            which: GenericThresholdRef::Feature,
            value: "NOFEATURE".to_string(),
        }
    );
}

/// An existing (plan, behavior, feature) tuple -> Duplicate (warning-success),
/// which must NEVER be an error.
#[test]
fn validate_generic_threshold_duplicate_on_template() {
    let config = template();
    // INGEST/NAME/all exists in the template.
    let check = validate_generic_threshold(&config, "INGEST", "NAME", "Yes", None).unwrap();
    assert_eq!(check, GenericThresholdCheck::Duplicate);
}

/// A bad behaviour on a non-duplicate tuple -> Invalid with the aggregated
/// failures.
#[test]
fn validate_generic_threshold_invalid_on_template() {
    let config = template();
    let check = validate_generic_threshold(&config, "INGEST", "BOGUS", "perhaps", None).unwrap();
    match check {
        GenericThresholdCheck::Invalid(failures) => {
            assert_eq!(failures.len(), 2);
            assert_eq!(failures[0].field, "behavior");
            assert_eq!(failures[1].field, "sendToRedo");
        }
        other => panic!("expected Invalid, got {other:?}"),
    }
}

/// A genuinely new, valid tuple -> Ok. The template ships no
/// (SEARCH, NAME, NAME-feature) row.
#[test]
fn validate_generic_threshold_ok_on_template() {
    let config = template();
    let check = validate_generic_threshold(&config, "SEARCH", "NAME", "Yes", Some("NAME")).unwrap();
    assert_eq!(check, GenericThresholdCheck::Ok);
}
