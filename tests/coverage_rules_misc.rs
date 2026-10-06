//! Public-API coverage for rules, fragments, generic plans, search profiles,
//! config sections, system parameters, settings, versioning, behavior
//! overrides and the generic config-array helpers — success and every error
//! path, asserting the exact error kind (and message where stable).

use serde_json::{Value, json};
use sz_configtool_lib::behavior_overrides as bovr;
use sz_configtool_lib::config_sections as sections;
use sz_configtool_lib::fragments::{self, SetFragmentParams};
use sz_configtool_lib::generic_plans as plans;
use sz_configtool_lib::helpers::{self, FieldUpdate};
use sz_configtool_lib::rules::{self, SetRuleParams};
use sz_configtool_lib::search_profiles::{self as sp, AddSearchProfileParams};
use sz_configtool_lib::{SzConfigError, SzErrorKind, settings, system_params, versioning};

const TEMPLATE: &str = include_str!("fixtures/g2config_template.json");
const BAD_JSON: &str = "{not json";

fn kind<T: std::fmt::Debug>(result: Result<T, SzConfigError>) -> SzErrorKind {
    result.unwrap_err().kind()
}

fn msg<T: std::fmt::Debug>(result: Result<T, SzConfigError>) -> String {
    result.unwrap_err().to_string()
}

fn parsed(config: &str) -> Value {
    serde_json::from_str(config).unwrap()
}

fn section<'a>(config: &'a Value, name: &str) -> &'a Vec<Value> {
    config["G2_CONFIG"][name].as_array().unwrap()
}

fn find<'a>(config: &'a Value, name: &str, key: &str, value: &str) -> Option<&'a Value> {
    section(config, name)
        .iter()
        .find(|r| r[key].as_str() == Some(value))
}

// ============================================================================
// versioning
// ============================================================================

#[test]
fn test_versioning_getters() {
    assert_eq!(versioning::get_version(TEMPLATE).unwrap(), "4.4.0");
    assert_eq!(
        versioning::get_compatibility_version(TEMPLATE).unwrap(),
        "11"
    );
    assert_eq!(
        versioning::verify_compatibility_version(TEMPLATE, "10").unwrap(),
        ("11".to_string(), false)
    );

    assert_eq!(
        kind(versioning::get_version(BAD_JSON)),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(versioning::get_compatibility_version(BAD_JSON)),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(versioning::verify_compatibility_version(BAD_JSON, "11")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(versioning::update_compatibility_version(BAD_JSON, "11")),
        SzErrorKind::JsonParse
    );

    let empty = r#"{"G2_CONFIG": {}}"#;
    assert_eq!(
        msg(versioning::get_version(empty)),
        "VERSION not found in configuration"
    );
    assert_eq!(
        msg(versioning::get_compatibility_version(empty)),
        "COMPATIBILITY_VERSION not found in configuration"
    );
    assert_eq!(
        kind(versioning::verify_compatibility_version(empty, "11")),
        SzErrorKind::NotFound
    );
}

#[test]
fn test_update_compatibility_version_paths() {
    let out = versioning::update_compatibility_version(TEMPLATE, "12").unwrap();
    assert_eq!(versioning::get_compatibility_version(&out).unwrap(), "12");

    let not_object = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {"COMPATIBILITY_VERSION": "11"}}}"#;
    let err = versioning::update_compatibility_version(not_object, "12").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(
        err.to_string(),
        "Invalid configuration: COMPATIBILITY_VERSION is not an object"
    );

    let no_compat = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {}}}"#;
    let err = versioning::update_compatibility_version(no_compat, "12").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "COMPATIBILITY_VERSION not found");

    let no_base = r#"{"G2_CONFIG": {}}"#;
    let err = versioning::update_compatibility_version(no_base, "12").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "CONFIG_BASE_VERSION not found");

    // No G2_CONFIG at all: nothing to update, returned unchanged.
    assert_eq!(
        versioning::update_compatibility_version("{}", "12").unwrap(),
        "{}"
    );
}

// ============================================================================
// system parameters / settings
// ============================================================================

#[test]
fn test_list_system_parameters() {
    // Template: the RCLASS_ID=2 row carries a non-integer BREAK_RES ("No").
    assert!(
        system_params::list_system_parameters(TEMPLATE)
            .unwrap()
            .is_empty()
    );

    let config = r#"{"G2_CONFIG": {"CFG_RTYPE": [
        {"RCLASS_ID": 1, "BREAK_RES": 0},
        {"RCLASS_ID": 2, "BREAK_RES": 1},
        {"RCLASS_ID": 2, "BREAK_RES": 5}
    ]}}"#;
    let params = system_params::list_system_parameters(config).unwrap();
    assert_eq!(params.len(), 1);
    assert_eq!(params["relationshipsBreakMatches"], "1");

    assert!(
        system_params::list_system_parameters("{}")
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        kind(system_params::list_system_parameters(BAD_JSON)),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_set_system_parameter() {
    for name in ["relationshipsBreakMatches", "relationships_break_matches"] {
        let out = system_params::set_system_parameter(TEMPLATE, name, &json!(1)).unwrap();
        let config = parsed(&out);
        let rows = section(&config, "CFG_RTYPE");
        assert_eq!(
            rows.iter().find(|r| r["RCLASS_ID"] == 2).unwrap()["BREAK_RES"],
            1
        );
        // Rows of other classes are untouched.
        assert_eq!(rows[0]["BREAK_RES"], "No");
    }

    let err = system_params::set_system_parameter(TEMPLATE, "bogus", &json!(1)).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(
        err.to_string(),
        "Invalid configuration: Unknown system parameter: BOGUS"
    );

    let not_found = "Failed to set system parameter: RELATIONSHIPSBREAKMATCHES";
    for config in [
        r#"{"G2_CONFIG": {"CFG_RTYPE": [{"RCLASS_ID": 1}]}}"#,
        r#"{"G2_CONFIG": {}}"#,
    ] {
        let err =
            system_params::set_system_parameter(config, "relationshipsBreakMatches", &json!(1))
                .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), not_found);
    }

    assert_eq!(
        kind(system_params::set_system_parameter(
            BAD_JSON,
            "relationshipsBreakMatches",
            &json!(1)
        )),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_set_setting_bad_json() {
    assert_eq!(
        kind(settings::set_setting(BAD_JSON, "foo", 1)),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// config sections
// ============================================================================

#[test]
fn test_add_and_remove_config_section() {
    let out = sections::add_config_section(TEMPLATE, "cfg_custom").unwrap();
    assert_eq!(parsed(&out)["G2_CONFIG"]["CFG_CUSTOM"], json!([]));

    let err = sections::add_config_section(TEMPLATE, "CFG_ATTR").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
    assert_eq!(err.to_string(), "Configuration section already exists");

    let err = sections::add_config_section("{}", "CFG_X").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(
        err.to_string(),
        "G2_CONFIG section not found in configuration"
    );

    // A non-object G2_CONFIG is left untouched rather than erroring.
    assert_eq!(
        sections::add_config_section(r#"{"G2_CONFIG":[]}"#, "CFG_X").unwrap(),
        r#"{"G2_CONFIG":[]}"#
    );

    let out = sections::remove_config_section(&out, "cfg_custom").unwrap();
    assert!(parsed(&out)["G2_CONFIG"].get("CFG_CUSTOM").is_none());

    for config in [TEMPLATE, r#"{"G2_CONFIG":[]}"#, "{}"] {
        let err = sections::remove_config_section(config, "CFG_NOPE").unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "Config section not found: CFG_NOPE");
    }

    assert_eq!(
        kind(sections::add_config_section(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sections::remove_config_section(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_get_config_section_variants() {
    let all = sections::get_config_section(TEMPLATE, "CFG_GPLAN", None).unwrap();
    assert_eq!(all.len(), 2);

    let config = r#"{"G2_CONFIG": {"SETTINGS": {"METAPHONE": 3}}}"#;
    // Non-array section, no filter: returned as a single item.
    assert_eq!(
        sections::get_config_section(config, "SETTINGS", None).unwrap(),
        vec![json!({"METAPHONE": 3})]
    );
    // Non-array section, filter matches / misses.
    assert_eq!(
        sections::get_config_section(config, "SETTINGS", Some("metaphone"))
            .unwrap()
            .len(),
        1
    );
    assert!(
        sections::get_config_section(config, "SETTINGS", Some("nope"))
            .unwrap()
            .is_empty()
    );

    let err = sections::get_config_section(config, "CFG_NOPE", None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(
        err.to_string(),
        "Configuration section 'CFG_NOPE' not found"
    );

    assert_eq!(
        kind(sections::get_config_section(BAD_JSON, "X", None)),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sections::config_section_is_empty(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_list_config_sections_variants() {
    let names = sections::list_config_sections(TEMPLATE).unwrap();
    assert!(names.contains(&"CFG_ERRULE".to_string()));
    assert!(
        sections::list_config_sections(r#"{"G2_CONFIG": 1}"#)
            .unwrap()
            .is_empty()
    );
    assert!(sections::list_config_sections("{}").unwrap().is_empty());
    assert_eq!(
        kind(sections::list_config_sections(BAD_JSON)),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_config_section_field_edge_cases() {
    // Non-object items are skipped by both add and remove.
    let config = r#"{"G2_CONFIG": {"CFG_X": [{"A": 1}, 7, {"B": 2}]}}"#;
    let (out, counts) =
        sections::add_config_section_field(config, "CFG_X", "a", &json!(0)).unwrap();
    assert_eq!((counts.existed, counts.updated), (1, 1));
    assert_eq!(parsed(&out)["G2_CONFIG"]["CFG_X"][1], 7);

    let (out, removed) = sections::remove_config_section_field(&out, "CFG_X", "a").unwrap();
    assert_eq!(removed, 2);
    assert_eq!(parsed(&out)["G2_CONFIG"]["CFG_X"], json!([{}, 7, {"B": 2}]));

    let err = sections::remove_config_section_field(config, "CFG_NOPE", "A").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(
        err.to_string(),
        "Section not found or not an array: CFG_NOPE"
    );

    // No G2_CONFIG: nothing to touch, zero counts.
    let (out, counts) = sections::add_config_section_field("{}", "CFG_X", "A", &json!(0)).unwrap();
    assert_eq!((out.as_str(), counts.existed, counts.updated), ("{}", 0, 0));
    assert_eq!(
        sections::remove_config_section_field("{}", "CFG_X", "A").unwrap(),
        ("{}".to_string(), 0)
    );

    assert_eq!(
        kind(sections::add_config_section_field(
            BAD_JSON,
            "X",
            "A",
            &json!(0)
        )),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sections::remove_config_section_field(BAD_JSON, "X", "A")),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// fragments
// ============================================================================

#[test]
fn test_set_fragment_params_try_from() {
    let v = json!({});
    let p = SetFragmentParams::try_from(&v).unwrap();
    assert_eq!(
        (p.source, p.description),
        (FieldUpdate::Leave, FieldUpdate::Leave)
    );

    let v = json!({"ERFRAG_SOURCE": null, "description": "d"});
    let p = SetFragmentParams::try_from(&v).unwrap();
    assert_eq!(
        (p.source, p.description),
        (FieldUpdate::Clear, FieldUpdate::Set("d"))
    );

    let v = json!({"source": "s", "ERFRAG_DESC": null});
    let p = SetFragmentParams::try_from(&v).unwrap();
    assert_eq!(
        (p.source, p.description),
        (FieldUpdate::Set("s"), FieldUpdate::Clear)
    );
}

fn frag(code: &str, source: &str) -> Value {
    json!({"ERFRAG_CODE": code, "ERFRAG_SOURCE": source})
}

#[test]
fn test_add_fragment_dependency_parsing() {
    // SAME_NAME is ERFRAG_ID 11 and CLOSE_NAME is 12 in the template.
    let cases = [
        (
            "./FRAGMENT[./SAME_NAME>0 and ./CLOSE_NAME>0]",
            Some("11,12"),
        ),
        // '/' directly terminating a reference.
        ("FRAGMENT[/SAME_NAME/CLOSE_NAME>0]", Some("11,12")),
        // A delimiter with no pending name.
        ("FRAGMENT[/>0]", None),
        // Unterminated FRAGMENT[ is ignored.
        ("FRAGMENT[/SAME_NAME", None),
        // Duplicate references are de-duplicated.
        ("FRAGMENT[./SAME_NAME>0 or ./SAME_NAME>1]", Some("11")),
    ];
    for (source, depends) in cases {
        let (out, id) = fragments::add_fragment(TEMPLATE, &frag("MY_FRAG", source)).unwrap();
        let config = parsed(&out);
        let row = find(&config, "CFG_ERFRAG", "ERFRAG_CODE", "MY_FRAG").unwrap();
        assert_eq!(row["ERFRAG_ID"], id, "{source}");
        assert_eq!(row["ERFRAG_DEPENDS"], json!(depends), "{source}");
    }

    for source in ["FRAGMENT[/NOPE/SAME_NAME>0]", "FRAGMENT[./NOPE>0]"] {
        let err = fragments::add_fragment(TEMPLATE, &frag("MY_FRAG", source)).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput, "{source}");
        assert_eq!(
            err.to_string(),
            "Invalid input: Invalid fragment reference: NOPE"
        );
    }
}

#[test]
fn test_add_fragment_reference_without_id_is_not_a_dependency() {
    let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_CODE": "NOID"}]}}"#;
    for source in ["FRAGMENT[/NOID/NOID>0]", "FRAGMENT[./NOID>0]"] {
        let (out, id) = fragments::add_fragment(config, &frag("NEW", source)).unwrap();
        assert_eq!(id, 1);
        let config = parsed(&out);
        let row = find(&config, "CFG_ERFRAG", "ERFRAG_CODE", "NEW").unwrap();
        assert!(row["ERFRAG_DEPENDS"].is_null(), "{source}");
    }
}

#[test]
fn test_add_fragment_errors() {
    let err = fragments::add_fragment(TEMPLATE, &json!({"ERFRAG_CODE": "X"})).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingField);
    assert_eq!(err.to_string(), "Missing required field: ERFRAG_SOURCE");

    assert_eq!(
        kind(fragments::add_fragment(BAD_JSON, &frag("X", "S"))),
        SzErrorKind::JsonParse
    );

    let err = fragments::add_fragment(TEMPLATE, &frag("same_name", "S")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
    assert_eq!(err.to_string(), "Fragment already exists");

    let err = fragments::add_fragment("{}", &frag("X", "S")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(
        err.to_string(),
        "Invalid configuration: G2_CONFIG not found"
    );

    let mut taken = frag("X", "S");
    taken["ERFRAG_ID"] = json!(11);
    assert_eq!(
        kind(fragments::add_fragment(TEMPLATE, &taken)),
        SzErrorKind::AlreadyExists
    );

    let err = fragments::add_fragment(r#"{"G2_CONFIG": {}}"#, &frag("X", "S")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: CFG_ERFRAG");
}

#[test]
fn test_fragment_get_delete_list_set() {
    let by_id = fragments::get_fragment(TEMPLATE, "11").unwrap();
    assert_eq!(by_id["fragment"], "SAME_NAME");

    let err = fragments::get_fragment(TEMPLATE, "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Fragment not found: NOPE");

    let err = fragments::delete_fragment(TEMPLATE, "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Fragment not found: NOPE");

    assert!(
        fragments::list_fragments(r#"{"G2_CONFIG": {}}"#)
            .unwrap()
            .is_empty()
    );
    assert!(fragments::list_fragments("{}").unwrap().is_empty());

    let set = |source| SetFragmentParams {
        source,
        description: FieldUpdate::Leave,
    };
    let out = fragments::set_fragment(
        TEMPLATE,
        "same_name",
        set(FieldUpdate::Set("./FRAGMENT[./CLOSE_NAME>0]")),
    )
    .unwrap();
    let config = parsed(&out);
    let row = find(&config, "CFG_ERFRAG", "ERFRAG_CODE", "SAME_NAME").unwrap();
    assert_eq!(row["ERFRAG_DEPENDS"], "12");

    let err = fragments::set_fragment(
        TEMPLATE,
        "SAME_NAME",
        set(FieldUpdate::Set("./FRAGMENT[./NOPE>0]")),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);

    let err = fragments::set_fragment(TEMPLATE, "nope", set(FieldUpdate::Leave)).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Fragment not found: NOPE");

    assert_eq!(
        kind(fragments::get_fragment(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(fragments::delete_fragment(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(fragments::list_fragments(BAD_JSON)),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(fragments::set_fragment(
            BAD_JSON,
            "X",
            set(FieldUpdate::Leave)
        )),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// rules
// ============================================================================

#[test]
fn test_set_rule_params_try_from() {
    let v = json!({
        "code": "R",
        "resolve": "Yes",
        "relate": "No",
        "rtypeId": 1,
        "fragment": "F",
        "disqualifier": null,
        "tier": 5
    });
    let p = SetRuleParams::try_from(&v).unwrap();
    assert_eq!(
        (p.code, p.resolve, p.relate, p.rtype_id),
        ("R", Some("Yes"), Some("No"), Some(1))
    );
    assert_eq!(
        (p.fragment, p.disqualifier, p.tier),
        (
            FieldUpdate::Set("F"),
            FieldUpdate::Clear,
            FieldUpdate::Set(5)
        )
    );

    let v = json!({"rule": "R2", "RESOLVE": "No", "RELATE": "Yes", "RTYPE_ID": 2, "TIER": null});
    let p = SetRuleParams::try_from(&v).unwrap();
    assert_eq!(
        (p.code, p.resolve, p.relate, p.rtype_id),
        ("R2", Some("No"), Some("Yes"), Some(2))
    );
    assert_eq!(
        (p.fragment, p.disqualifier, p.tier),
        (FieldUpdate::Leave, FieldUpdate::Leave, FieldUpdate::Clear)
    );

    let v = json!({});
    let err = SetRuleParams::try_from(&v).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingField);
    assert_eq!(err.to_string(), "Missing required field: code or rule");
}

fn relate_rule(code: &str) -> Value {
    json!({
        "ERRULE_CODE": code,
        "RESOLVE": "No",
        "RELATE": "Yes",
        "RTYPE_ID": 2,
        "QUAL_ERFRAG_CODE": "SAME_NAME"
    })
}

#[test]
fn test_add_rule_errors() {
    assert_eq!(
        kind(rules::add_rule(BAD_JSON, 0, &relate_rule("R"))),
        SzErrorKind::JsonParse
    );

    let no_rules =
        r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "SAME_NAME"}]}}"#;
    let err = rules::add_rule(no_rules, 0, &relate_rule("R")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: CFG_ERRULE");
}

#[test]
fn test_delete_get_list_rules() {
    let out = rules::delete_rule(TEMPLATE, "same_a1").unwrap();
    assert!(find(&parsed(&out), "CFG_ERRULE", "ERRULE_CODE", "SAME_A1").is_none());

    let err = rules::delete_rule(TEMPLATE, "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Rule not found: NOPE");

    // SAME_A1 is ERRULE_ID 100.
    assert_eq!(rules::get_rule(TEMPLATE, "100").unwrap()["rule"], "SAME_A1");

    let err = rules::get_rule(TEMPLATE, "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Rule not found: NOPE");

    let (out, _) = rules::add_rule(TEMPLATE, 0, &relate_rule("MY_RELATE")).unwrap();
    let rule = rules::get_rule(&out, "MY_RELATE").unwrap();
    assert!(rule["tier"].is_null());

    assert!(
        rules::list_rules(r#"{"G2_CONFIG": {}}"#)
            .unwrap()
            .is_empty()
    );
    assert!(rules::list_rules("{}").unwrap().is_empty());

    assert_eq!(
        kind(rules::delete_rule(BAD_JSON, "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(kind(rules::get_rule(BAD_JSON, "X")), SzErrorKind::JsonParse);
    assert_eq!(kind(rules::list_rules(BAD_JSON)), SzErrorKind::JsonParse);
}

fn set_params(code: &str) -> SetRuleParams<'_> {
    SetRuleParams {
        code,
        resolve: None,
        relate: None,
        rtype_id: None,
        fragment: FieldUpdate::Leave,
        disqualifier: FieldUpdate::Leave,
        tier: FieldUpdate::Leave,
    }
}

#[test]
fn test_set_rule_paths() {
    // SF1_SNAME_CFF_CSTAB carries a disqualifier (DIFF_EXCL) and RTYPE_ID 1.
    let code = "SF1_SNAME_CFF_CSTAB";
    let params = SetRuleParams {
        fragment: FieldUpdate::Clear,
        disqualifier: FieldUpdate::Clear,
        ..set_params(code)
    };
    let out = rules::set_rule(TEMPLATE, params).unwrap();
    let config = parsed(&out);
    let row = find(&config, "CFG_ERRULE", "ERRULE_CODE", code).unwrap();
    assert!(row["QUAL_ERFRAG_CODE"].is_null());
    assert!(row["DISQ_ERFRAG_CODE"].is_null());
    assert_eq!(row["RTYPE_ID"], 1);

    let params = SetRuleParams {
        disqualifier: FieldUpdate::Set("same_name"),
        ..set_params(code)
    };
    let out = rules::set_rule(TEMPLATE, params).unwrap();
    let config = parsed(&out);
    let row = find(&config, "CFG_ERRULE", "ERRULE_CODE", code).unwrap();
    assert_eq!(row["DISQ_ERFRAG_CODE"], "SAME_NAME");

    let params = SetRuleParams {
        disqualifier: FieldUpdate::Set("NOPE"),
        ..set_params(code)
    };
    assert_eq!(
        kind(rules::set_rule(TEMPLATE, params)),
        SzErrorKind::NotFound
    );

    let params = SetRuleParams {
        resolve: Some("Maybe"),
        ..set_params(code)
    };
    let err = rules::set_rule(TEMPLATE, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "Invalid input: resolve value must be in [\"Yes\", \"No\"]"
    );

    // Set a fragment and clear the tier on a relate-only rule.
    let (with_relate, _) = rules::add_rule(TEMPLATE, 0, &relate_rule("MY_RELATE")).unwrap();
    let params = SetRuleParams {
        fragment: FieldUpdate::Set("close_name"),
        tier: FieldUpdate::Clear,
        ..set_params("MY_RELATE")
    };
    let out = rules::set_rule(&with_relate, params).unwrap();
    let config = parsed(&out);
    let row = find(&config, "CFG_ERRULE", "ERRULE_CODE", "MY_RELATE").unwrap();
    assert_eq!(row["QUAL_ERFRAG_CODE"], "CLOSE_NAME");
    assert!(row["ERRULE_TIER"].is_null());

    let err = rules::set_rule(TEMPLATE, set_params("nope")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Rule not found: NOPE");

    assert_eq!(
        kind(rules::set_rule(BAD_JSON, set_params("X"))),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// generic plans
// ============================================================================

#[test]
fn test_clone_generic_plan_copies_thresholds() {
    let config = parsed(TEMPLATE);
    let ingest = section(&config, "CFG_GENERIC_THRESHOLD")
        .iter()
        .filter(|r| r["GPLAN_ID"] == 1)
        .count();
    assert!(ingest > 0);

    let (out, id) = plans::clone_generic_plan(TEMPLATE, "ingest", "copy", Some("Copy")).unwrap();
    assert_eq!(id, 3);
    let config = parsed(&out);
    let copied = section(&config, "CFG_GENERIC_THRESHOLD")
        .iter()
        .filter(|r| r["GPLAN_ID"] == 3)
        .count();
    assert_eq!(copied, ingest);
    assert_eq!(
        find(&config, "CFG_GPLAN", "GPLAN_CODE", "COPY").unwrap()["GPLAN_DESC"],
        "Copy"
    );
}

#[test]
fn test_clone_generic_plan_without_threshold_section() {
    let config = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "A"}]}}"#;
    let (out, id) = plans::clone_generic_plan(config, "A", "B", None).unwrap();
    assert_eq!(id, 2);
    let config = parsed(&out);
    let row = find(&config, "CFG_GPLAN", "GPLAN_CODE", "B").unwrap();
    // The description defaults to the new code.
    assert_eq!(row["GPLAN_DESC"], "B");
    assert!(config["G2_CONFIG"].get("CFG_GENERIC_THRESHOLD").is_none());
}

#[test]
fn test_clone_generic_plan_errors() {
    assert_eq!(
        kind(plans::clone_generic_plan(BAD_JSON, "A", "B", None)),
        SzErrorKind::JsonParse
    );

    let err = plans::clone_generic_plan(TEMPLATE, "nope", "B", None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Source generic plan not found: NOPE");

    let no_id = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_CODE": "A"}]}}"#;
    let err = plans::clone_generic_plan(no_id, "A", "B", None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(
        err.to_string(),
        "Invalid configuration: Invalid GPLAN_ID in source plan"
    );

    let err = plans::clone_generic_plan(TEMPLATE, "INGEST", "search", None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
    assert_eq!(err.to_string(), "Generic plan already exists: SEARCH");
}

#[test]
fn test_delete_generic_plan() {
    let (cloned, id) = plans::clone_generic_plan(TEMPLATE, "INGEST", "COPY", None).unwrap();
    let out = plans::delete_generic_plan(&cloned, "copy").unwrap();
    let config = parsed(&out);
    assert!(find(&config, "CFG_GPLAN", "GPLAN_CODE", "COPY").is_none());
    assert!(
        section(&config, "CFG_GENERIC_THRESHOLD")
            .iter()
            .all(|r| r["GPLAN_ID"] != id)
    );

    // No threshold section: only the plan row is removed.
    let plan_only = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 3, "GPLAN_CODE": "P"}]}}"#;
    let out = plans::delete_generic_plan(plan_only, "P").unwrap();
    assert_eq!(out, r#"{"G2_CONFIG":{"CFG_GPLAN":[]}}"#);

    let err = plans::delete_generic_plan(TEMPLATE, "search").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "Invalid input: The SEARCH plan cannot be deleted"
    );

    let err = plans::delete_generic_plan(TEMPLATE, "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Generic plan not found: NOPE");

    let no_id = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_CODE": "P"}]}}"#;
    let err = plans::delete_generic_plan(no_id, "P").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(err.to_string(), "Invalid configuration: Invalid GPLAN_ID");

    assert_eq!(
        kind(plans::delete_generic_plan(BAD_JSON, "P")),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_list_generic_plans() {
    let all = plans::list_generic_plans(TEMPLATE, None).unwrap();
    assert_eq!(
        all.iter().map(|p| p["plan"].clone()).collect::<Vec<_>>(),
        vec![json!("INGEST"), json!("SEARCH")]
    );

    let filtered = plans::list_generic_plans(TEMPLATE, Some("search")).unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0]["id"], 2);

    assert_eq!(
        kind(plans::list_generic_plans(BAD_JSON, None)),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_set_generic_plan_paths() {
    let (_, id, created) = plans::set_generic_plan(TEMPLATE, "new_plan", "New").unwrap();
    assert_eq!((id, created), (3, true));

    let err = plans::set_generic_plan(r#"{"G2_CONFIG": {}}"#, "P", "D").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: CFG_GPLAN");

    assert_eq!(
        kind(plans::set_generic_plan(BAD_JSON, "P", "D")),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// search profiles
// ============================================================================

#[test]
fn test_add_search_profile_paths() {
    let params = AddSearchProfileParams::new("mine", "SEARCH")
        .with_candidates("normal")
        .with_elements(vec![("NAME", "Yes"), ("DOB", "n")]);
    let out = sp::add_search_profile(TEMPLATE, params).unwrap();
    let row = sp::get_search_profile(&out, "MINE").unwrap();
    assert_eq!(row["DEFAULT_USED_FOR_CAND"], "Normal");
    assert_eq!(row["FTYPE_OVERRIDES"], "[{1,Y},{2,N}]");

    let not_array = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 2, "GPLAN_CODE": "SEARCH"}],
        "CFG_SPROFILE": {}}}"#;
    let err =
        sp::add_search_profile(not_array, AddSearchProfileParams::new("P", "SEARCH")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidStructure);
    assert_eq!(
        err.to_string(),
        "Invalid config structure: CFG_SPROFILE is not an array"
    );

    let bad = || AddSearchProfileParams::new("P", "SEARCH");
    assert_eq!(
        kind(sp::add_search_profile(BAD_JSON, bad())),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sp::get_search_profile(BAD_JSON, "P")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sp::list_search_profiles(BAD_JSON, None)),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(sp::delete_search_profile(BAD_JSON, "P")),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_list_search_profiles_unresolved_and_malformed_rows() {
    let config = r#"{"G2_CONFIG": {
        "CFG_GPLAN": [{"GPLAN_ID": 2, "GPLAN_CODE": "SEARCH"}],
        "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
        "CFG_SPROFILE": [
            {"SPROFILE_ID": 1, "SPROFILE_CODE": "EMPTY", "GPLAN_ID": 2, "FTYPE_OVERRIDES": "[]"},
            {"SPROFILE_ID": 2, "SPROFILE_CODE": "ODD", "GPLAN_ID": 77,
             "FTYPE_OVERRIDES": "[{1,Z},{55,Y},{x,Y},{9}]"},
            {"SPROFILE_ID": 3, "SPROFILE_CODE": "UNBRACKETED", "GPLAN_ID": 2,
             "FTYPE_OVERRIDES": "{1,Y}"}
        ]
    }}"#;
    let rows = sp::list_search_profiles(config, None).unwrap();

    assert_eq!(rows[0]["genericPlan"], "SEARCH");
    assert_eq!(rows[0]["overrides"], json!([]));
    // A value missing its brackets yields no recoverable pairs.
    assert_eq!(rows[2]["overrides"], json!([]));
    assert_eq!(rows[2]["overridesRaw"], "{1,Y}");

    // Unknown plan id and unknown feature id fall back to the numeric id; an
    // unknown flag renders "?"; unparseable/short pairs are dropped.
    assert_eq!(rows[1]["genericPlan"], "77");
    assert_eq!(
        rows[1]["overrides"],
        json!([
            {"feature": "NAME", "flag": "?"},
            {"feature": "55", "flag": "Yes"}
        ])
    );
}

// ============================================================================
// behavior overrides
// ============================================================================

#[test]
fn test_add_behavior_override_errors() {
    let p = |feature, behavior| bovr::AddBehaviorOverrideParams::new(feature, "TEST", behavior);

    assert_eq!(
        kind(bovr::add_behavior_override(BAD_JSON, p("NAME", "F1"))),
        SzErrorKind::JsonParse
    );

    let err = bovr::add_behavior_override(TEMPLATE, p("NOPE", "F1")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Feature 'NOPE' not found");

    assert_eq!(
        kind(bovr::add_behavior_override(TEMPLATE, p("NAME", "BOGUS"))),
        SzErrorKind::InvalidInput
    );

    let no_section = r#"{"G2_CONFIG": {"CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}]}}"#;
    let err = bovr::add_behavior_override(no_section, p("NAME", "F1")).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: CFG_FBOVR");
}

#[test]
fn test_delete_behavior_override_errors() {
    // ADDRESS (FTYPE_ID 5) has a BUSINESS override in the template.
    let out = bovr::delete_behavior_override(TEMPLATE, "ADDRESS", "business").unwrap();
    assert_eq!(
        kind(bovr::get_behavior_override(&out, "ADDRESS", "BUSINESS")),
        SzErrorKind::NotFound
    );

    assert_eq!(
        kind(bovr::delete_behavior_override(BAD_JSON, "NAME", "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(bovr::delete_behavior_override(TEMPLATE, "NOPE", "X")),
        SzErrorKind::NotFound
    );

    let no_section = r#"{"G2_CONFIG": {"CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}]}}"#;
    assert_eq!(
        kind(bovr::delete_behavior_override(no_section, "NAME", "X")),
        SzErrorKind::MissingSection
    );

    let err = bovr::delete_behavior_override(TEMPLATE, "NAME", "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(
        err.to_string(),
        "Behavior override not found for feature NAME with usage type NOPE"
    );
}

#[test]
fn test_get_and_list_behavior_overrides() {
    let row = bovr::get_behavior_override(TEMPLATE, "address", "business").unwrap();
    assert_eq!(row["FTYPE_FREQ"], "FF");

    let err = bovr::get_behavior_override(TEMPLATE, "NAME", "nope").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(
        err.to_string(),
        "Behavior override not found for feature NAME with usage type NOPE"
    );

    assert_eq!(
        kind(bovr::get_behavior_override(BAD_JSON, "NAME", "X")),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        kind(bovr::get_behavior_override(TEMPLATE, "NOPE", "X")),
        SzErrorKind::NotFound
    );

    let no_section = r#"{"G2_CONFIG": {"CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}]}}"#;
    assert_eq!(
        kind(bovr::get_behavior_override(no_section, "NAME", "X")),
        SzErrorKind::MissingSection
    );
    assert_eq!(
        kind(bovr::list_behavior_overrides(no_section)),
        SzErrorKind::MissingSection
    );
    assert_eq!(
        kind(bovr::list_behavior_overrides(BAD_JSON)),
        SzErrorKind::JsonParse
    );
}

#[test]
fn test_list_behavior_overrides_resolved_edges() {
    // A CFG_FTYPE row without an id/code is skipped for resolution.
    let config = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [{"FTYPE_CODE": "NOID"}],
        "CFG_FBOVR": [{"FTYPE_ID": 4, "UTYPE_CODE": "U", "FTYPE_FREQ": "F1",
                       "FTYPE_EXCL": "No", "FTYPE_STAB": "No"}]
    }}"#;
    let rows = bovr::list_behavior_overrides_resolved(config).unwrap();
    assert_eq!(rows[0]["feature"], "4");

    // No CFG_FTYPE at all: same id fallback.
    let config = r#"{"G2_CONFIG": {"CFG_FBOVR": [{"FTYPE_ID": 4, "UTYPE_CODE": "U"}]}}"#;
    let rows = bovr::list_behavior_overrides_resolved(config).unwrap();
    assert_eq!(rows[0]["feature"], "4");

    let err = bovr::list_behavior_overrides_resolved("{}").unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: G2_CONFIG");

    let err = bovr::list_behavior_overrides_resolved(r#"{"G2_CONFIG": {}}"#).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(err.to_string(), "Missing config section: CFG_FBOVR");

    assert_eq!(
        kind(bovr::list_behavior_overrides_resolved(BAD_JSON)),
        SzErrorKind::JsonParse
    );
}

// ============================================================================
// helpers
// ============================================================================

#[test]
fn test_field_update_parsers() {
    let v = json!({"null": null, "str": "s", "int": 7, "bool": true});

    assert_eq!(
        helpers::field_update_str(&v, &["absent"]),
        FieldUpdate::Leave
    );
    assert_eq!(helpers::field_update_str(&v, &["null"]), FieldUpdate::Clear);
    assert_eq!(
        helpers::field_update_str(&v, &["int", "str"]),
        FieldUpdate::Set("s")
    );
    assert_eq!(helpers::field_update_str(&v, &["bool"]), FieldUpdate::Leave);

    assert_eq!(
        helpers::field_update_i64(&v, &["absent"]),
        FieldUpdate::Leave
    );
    assert_eq!(helpers::field_update_i64(&v, &["null"]), FieldUpdate::Clear);
    assert_eq!(
        helpers::field_update_i64(&v, &["str", "int"]),
        FieldUpdate::Set(7)
    );
    assert_eq!(helpers::field_update_i64(&v, &["bool"]), FieldUpdate::Leave);
}

#[test]
fn test_section_path_id_helpers() {
    let config = json!({"G2_CONFIG": {"CFG_X": [{"ID": 4}], "OBJ": {}}});

    let err = helpers::get_next_id(&config, "G2_CONFIG.CFG_NOPE", "ID", 1).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(
        err.to_string(),
        "Missing config section: Section path 'G2_CONFIG.CFG_NOPE' not found"
    );
    // A non-array section yields the seed.
    assert_eq!(
        helpers::get_next_id(&config, "G2_CONFIG.OBJ", "ID", 10).unwrap(),
        10
    );

    let err =
        helpers::get_desired_or_next_id_from_section(&config, "G2_CONFIG.NOPE", "ID", None, 1)
            .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    let err = helpers::get_desired_or_next_id_from_section(&config, "G2_CONFIG.OBJ", "ID", None, 1)
        .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::MissingSection);
    assert_eq!(
        err.to_string(),
        "Missing config section: Section 'G2_CONFIG.OBJ' is not an array"
    );
    assert_eq!(
        helpers::get_desired_or_next_id_from_section(&config, "G2_CONFIG.CFG_X", "ID", None, 1)
            .unwrap(),
        5
    );
}

#[test]
fn test_find_in_array() {
    let arr = vec![json!({"CODE": 1}), json!({"CODE": "A"})];
    assert_eq!(helpers::find_in_array(&arr, "CODE", "A"), Some(&arr[1]));
    assert_eq!(helpers::find_in_array(&arr, "CODE", "B"), None);
}

#[test]
fn test_config_array_helpers_errors() {
    let empty = r#"{"G2_CONFIG": {}}"#;
    let missing = "Missing config section: CFG_X";

    assert_eq!(
        kind(helpers::add_to_config_array(BAD_JSON, "CFG_X", json!({}))),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        msg(helpers::add_to_config_array(empty, "CFG_X", json!({}))),
        missing
    );

    assert_eq!(
        kind(helpers::delete_from_config_array(
            BAD_JSON, "CFG_X", "K", "V"
        )),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        msg(helpers::delete_from_config_array(empty, "CFG_X", "K", "V")),
        missing
    );

    assert_eq!(
        kind(helpers::update_in_config_array(
            BAD_JSON,
            "CFG_X",
            "K",
            "V",
            json!({})
        )),
        SzErrorKind::JsonParse
    );
    assert_eq!(
        msg(helpers::update_in_config_array(
            empty,
            "CFG_X",
            "K",
            "V",
            json!({})
        )),
        missing
    );
    let one = r#"{"G2_CONFIG": {"CFG_X": [{"K": "A"}]}}"#;
    let err = helpers::update_in_config_array(one, "CFG_X", "K", "V", json!({})).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "CFG_X 'V' not found");
}

#[test]
fn test_find_and_list_config_array() {
    let config = r#"{"G2_CONFIG": {"CFG_X": [{"ID": 3, "K": "A"}]}}"#;
    // Numeric comparison against an integer field.
    assert_eq!(
        helpers::find_in_config_array(config, "CFG_X", "ID", "3").unwrap(),
        Some(json!({"ID": 3, "K": "A"}))
    );
    assert_eq!(
        helpers::find_in_config_array(config, "CFG_X", "ID", "x").unwrap(),
        None
    );
    assert_eq!(
        helpers::find_in_config_array(config, "CFG_NOPE", "ID", "3").unwrap(),
        None
    );
    assert_eq!(
        kind(helpers::find_in_config_array(BAD_JSON, "CFG_X", "ID", "3")),
        SzErrorKind::JsonParse
    );

    assert_eq!(
        helpers::list_from_config_array(config, "CFG_X").unwrap(),
        vec![json!({"ID": 3, "K": "A"})]
    );
    assert!(
        helpers::list_from_config_array(config, "CFG_NOPE")
            .unwrap()
            .is_empty()
    );
    assert!(
        helpers::list_from_config_array("{}", "CFG_X")
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        kind(helpers::list_from_config_array(BAD_JSON, "CFG_X")),
        SzErrorKind::JsonParse
    );
}

type Lookup = fn(&str, &str) -> Result<i64, SzConfigError>;

#[test]
fn test_lookup_helpers() {
    let lookups: [(Lookup, &str, &str); 7] = [
        (
            helpers::lookup_feature_id,
            "NAME",
            "Feature 'NOPE' not found",
        ),
        (
            helpers::lookup_element_id,
            "FULL_NAME",
            "Element 'NOPE' not found",
        ),
        (
            helpers::lookup_sfunc_id,
            "PARSE_NAME",
            "Standardize function 'NOPE' not found",
        ),
        (
            helpers::lookup_efunc_id,
            "EXPRESS_BOM",
            "Expression function 'NOPE' not found",
        ),
        (
            helpers::lookup_cfunc_id,
            "STR_COMP",
            "Comparison function 'NOPE' not found",
        ),
        (
            helpers::lookup_dfunc_id,
            "FELEM_STRICT_SUBSET",
            "Distinct function 'NOPE' not found",
        ),
        (
            helpers::lookup_gplan_id,
            "SEARCH",
            "Generic plan 'NOPE' not found",
        ),
    ];
    for (lookup, known, not_found) in lookups {
        assert!(lookup(TEMPLATE, known).unwrap() > 0, "{known}");
        let err = lookup(TEMPLATE, "NOPE").unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), not_found);
        assert_eq!(kind(lookup(BAD_JSON, known)), SzErrorKind::JsonParse);
    }

    assert_eq!(helpers::lookup_gplan_code(TEMPLATE, 2).unwrap(), "SEARCH");
    let err = helpers::lookup_gplan_code(TEMPLATE, 99).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.to_string(), "Generic plan ID: 99");
    assert_eq!(
        kind(helpers::lookup_gplan_code(BAD_JSON, 2)),
        SzErrorKind::JsonParse
    );
}
