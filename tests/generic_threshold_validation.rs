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
use sz_configtool_lib::error::SzErrorKind;
use sz_configtool_lib::thresholds::{
    AddGenericThresholdParams, SetGenericThresholdParams, add_generic_threshold,
    set_generic_threshold,
};

fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

/// (a) A bogus behaviour code on an existing plan must be rejected as
/// `InvalidInput` — the tightened validity block, not a silent add.
#[test]
fn add_generic_threshold_rejects_bogus_behavior_on_template() {
    let config = template();

    // INGEST is a real plan; "BOGUS" is not one of the canonical 17 codes.
    let params = AddGenericThresholdParams::new("INGEST", "BOGUS", 20, 10, "No");
    let err = add_generic_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);
    assert!(
        err.to_string().contains("BOGUS"),
        "error should name the offending code: {err}"
    );
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
