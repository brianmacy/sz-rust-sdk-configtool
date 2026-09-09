//! Integration tests for `CFG_SPROFILE` (search profile) operations, exercised
//! against the real Senzing v4 engine template `tests/fixtures/g2config_template.json`
//! (not synthetic JSON — the write/validate paths must be proven against stock
//! config).

use serde_json::Value;
use sz_configtool_lib::error::SzErrorKind;
use sz_configtool_lib::search_profiles::{
    AddSearchProfileParams, add_search_profile, get_search_profile, list_search_profiles,
};

fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

fn sprofiles(config_json: &str) -> Vec<Value> {
    let v: Value = serde_json::from_str(config_json).expect("valid JSON");
    v["G2_CONFIG"]["CFG_SPROFILE"]
        .as_array()
        .expect("CFG_SPROFILE array")
        .clone()
}

/// Precondition: the shipped template carries a CFG_SPROFILE section with the
/// SEARCH profile (id 2) and a resolvable generic plan. If this ever changes,
/// the tests below need revisiting.
#[test]
fn template_precondition_has_search_profile_section() {
    let config = template();
    let rows = sprofiles(&config);
    assert!(
        rows.iter().any(|r| r["SPROFILE_CODE"] == "SEARCH"),
        "template should ship a SEARCH profile"
    );
    // The SEARCH profile's GPLAN_ID must resolve for the projection test.
    let listed = list_search_profiles(&config, None).unwrap();
    let search = listed.iter().find(|p| p["profile"] == "SEARCH").unwrap();
    assert!(
        search["genericPlan"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "SEARCH profile's generic plan should resolve to a code"
    );
}

#[test]
fn add_profile_to_template_allocates_next_id() {
    let config = template();
    let before = sprofiles(&config).len();

    // Use feature/plan codes that exist in the stock template.
    let params = AddSearchProfileParams::new("EMBEDDED_SEARCH", "SEARCH")
        .with_description("Embedded search profile")
        .with_element("NAME", "Yes");
    let out = add_search_profile(&config, params).unwrap();

    let rows = sprofiles(&out);
    assert_eq!(rows.len(), before + 1);
    let added = rows
        .iter()
        .find(|r| r["SPROFILE_CODE"] == "EMBEDDED_SEARCH")
        .expect("added profile present");
    // Existing max id is 2 (SEARCH), so the new one is 3.
    assert_eq!(added["SPROFILE_ID"], 3);
    assert_eq!(added["DEFAULT_USED_FOR_CAND"], "Normal");
    // NAME resolves to FTYPE_ID 1 in the template.
    assert_eq!(added["FTYPE_OVERRIDES"], "[{1,Y}]");

    // Round-trips through the get/list read paths.
    let got = get_search_profile(&out, "embedded_search").unwrap();
    assert_eq!(got["SPROFILE_ID"], 3);

    let listed = list_search_profiles(&out, Some("embedded")).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["genericPlan"], "SEARCH");
    assert_eq!(listed[0]["overrides"][0]["feature"], "NAME");
    assert_eq!(listed[0]["overrides"][0]["flag"], "Yes");
}

#[test]
fn duplicate_profile_code_rejected_on_template() {
    let config = template();
    let err =
        add_search_profile(&config, AddSearchProfileParams::new("SEARCH", "SEARCH")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

#[test]
fn unknown_feature_not_found_on_template() {
    let config = template();
    let params =
        AddSearchProfileParams::new("P", "SEARCH").with_element("NOT_A_REAL_FEATURE", "Yes");
    let err = add_search_profile(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
}

#[test]
fn create_if_missing_path_on_section_removed_template() {
    // Remove CFG_SPROFILE from the real template to exercise the create-if-missing
    // branch (the stock template ships the section, so this is the only way to
    // cover it against real config).
    let mut config: Value = serde_json::from_str(&template()).unwrap();
    config["G2_CONFIG"]
        .as_object_mut()
        .unwrap()
        .remove("CFG_SPROFILE");
    let stripped = serde_json::to_string(&config).unwrap();

    // Read paths tolerate the missing section.
    assert!(list_search_profiles(&stripped, None).unwrap().is_empty());
    assert_eq!(
        get_search_profile(&stripped, "SEARCH").unwrap_err().kind(),
        SzErrorKind::NotFound
    );

    // Add creates the section and seeds the first id at 1.
    let out =
        add_search_profile(&stripped, AddSearchProfileParams::new("FIRST", "SEARCH")).unwrap();
    let rows = sprofiles(&out);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["SPROFILE_ID"], 1);
    assert_eq!(rows[0]["SPROFILE_CODE"], "FIRST");
}
