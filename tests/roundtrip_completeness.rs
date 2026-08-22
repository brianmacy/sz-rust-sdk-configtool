//! Round-trip completeness tests.
//!
//! These guard the invariant behind SENZ9117: the Senzing engine's config
//! loader requires every key of every CFG_* row to be present (even when its
//! value is null). A previous bug dropped a null `DISQ_ERFRAG_CODE` from a
//! CFG_ERRULE row on update, so the engine refused to load the saved config.
//!
//! The core check is `assert_no_keys_dropped`: after running a config through an
//! SDK write operation, every row that existed before must still carry all of
//! its original keys. Updates replace a row in place (same index) and adds
//! append, so a positional comparison of the "before" rows is exact.

use serde_json::{Value, json};
use sz_configtool_lib::fragments::SetFragmentParams;
use sz_configtool_lib::helpers::FieldUpdate;
use sz_configtool_lib::{fragments, rules};

/// For every section array under `G2_CONFIG`, assert that each row present in
/// `before` still has all of its keys in `after` at the same index. New rows
/// appended by an add operation are ignored (we only verify nothing was lost).
fn assert_no_keys_dropped(before: &Value, after: &Value) {
    let b = before["G2_CONFIG"].as_object().expect("before G2_CONFIG");
    let a = after["G2_CONFIG"].as_object().expect("after G2_CONFIG");

    for (section, b_val) in b {
        let (Some(b_rows), Some(a_rows)) =
            (b_val.as_array(), a.get(section).and_then(|v| v.as_array()))
        else {
            continue; // not an array section (e.g. scalar or object)
        };

        for (i, b_row) in b_rows.iter().enumerate() {
            let Some(b_obj) = b_row.as_object() else {
                continue;
            };
            let a_obj = a_rows
                .get(i)
                .and_then(|v| v.as_object())
                .unwrap_or_else(|| panic!("{section}[{i}] missing after round-trip"));

            for key in b_obj.keys() {
                assert!(
                    a_obj.contains_key(key),
                    "{section}[{i}] dropped key '{key}' during round-trip"
                );
            }
        }
    }
}

/// A representative slice of a g2config document: complete CFG_ERRULE and
/// CFG_ERFRAG rows, including a rule whose DISQ_ERFRAG_CODE is null (the exact
/// shape that triggered SENZ9117).
fn sample_config() -> Value {
    json!({
        "G2_CONFIG": {
            "CFG_ERRULE": [
                {
                    "ERRULE_ID": 100, "ERRULE_CODE": "SAME_A1", "RESOLVE": "Yes",
                    "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "SAME_A1",
                    "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": 10
                },
                {
                    "ERRULE_ID": 110, "ERRULE_CODE": "SF1_PNAME_CSTAB", "RESOLVE": "Yes",
                    "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "SF1_PNAME",
                    "DISQ_ERFRAG_CODE": "DIST_NAME", "ERRULE_TIER": 20
                }
            ],
            "CFG_ERFRAG": [
                {
                    "ERFRAG_ID": 10, "ERFRAG_CODE": "SAME_A1", "ERFRAG_DESC": "SAME_A1",
                    "ERFRAG_SOURCE": "./FRAGMENT[./SAME_NAME>0]", "ERFRAG_DEPENDS": null
                }
            ]
        }
    })
}

/// Updating a rule must not drop any key from any row anywhere in the config.
#[test]
fn roundtrip_set_rule_drops_no_keys() {
    let before = sample_config();
    let config_str = serde_json::to_string(&before).unwrap();

    // The operation that originally corrupted the config: flip RESOLVE on a rule
    // whose DISQ_ERFRAG_CODE is null.
    let params = rules::SetRuleParams {
        code: "SAME_A1",
        resolve: Some("No"),
        relate: None,
        rtype_id: None,
        fragment: FieldUpdate::Leave,
        disqualifier: FieldUpdate::Leave,
        tier: FieldUpdate::Leave,
    };
    let modified = rules::set_rule(&config_str, params).unwrap();
    let after: Value = serde_json::from_str(&modified).unwrap();

    assert_no_keys_dropped(&before, &after);

    // Explicitly confirm the field that used to vanish is still present as null.
    let rule = &after["G2_CONFIG"]["CFG_ERRULE"][0];
    assert!(rule.as_object().unwrap().contains_key("DISQ_ERFRAG_CODE"));
    assert_eq!(rule["DISQ_ERFRAG_CODE"], Value::Null);
    assert_eq!(rule["RESOLVE"], json!("No"));
}

/// Updating a fragment must not drop any key from any row anywhere in the config.
#[test]
fn roundtrip_set_fragment_drops_no_keys() {
    let before = sample_config();
    let config_str = serde_json::to_string(&before).unwrap();

    // Update only the description; ERFRAG_ID / ERFRAG_SOURCE / ERFRAG_DEPENDS
    // must be carried forward, not dropped by the full-row replace.
    let update = SetFragmentParams {
        source: FieldUpdate::Leave,
        description: FieldUpdate::Set("Updated"),
    };
    let modified = fragments::set_fragment(&config_str, "SAME_A1", update).unwrap();
    let after: Value = serde_json::from_str(&modified).unwrap();

    assert_no_keys_dropped(&before, &after);
    let frag = &after["G2_CONFIG"]["CFG_ERFRAG"][0];
    assert_eq!(frag["ERFRAG_ID"], json!(10));
    assert_eq!(frag["ERFRAG_SOURCE"], json!("./FRAGMENT[./SAME_NAME>0]"));
}

/// Real-config round-trip against a genuine engine template.
///
/// By default this runs against the committed fixture
/// `tests/fixtures/g2config_template.json` (a real Senzing v4 engine template),
/// so it exercises a full config on every CI run. Set
/// `SZ_CONFIG_FIXTURE=/path/to/g2config.json` to point it at your own config
/// instead.
///
/// Reads the ENTIRE config, runs every CFG_ERRULE row back through `set_rule`
/// (a no-op update that still rebuilds the row via the serde struct) and every
/// CFG_ERFRAG row through `set_fragment` (an empty update), then asserts no key
/// was dropped anywhere. This proves the SDK round-trips a genuine engine config
/// without dropping keys. If `SZ_CONFIG_OUT` is set, the re-serialized config is
/// written there so it can be fed to a real Senzing engine to confirm it loads.
#[test]
fn roundtrip_real_config_fixture() {
    let path = std::env::var("SZ_CONFIG_FIXTURE").unwrap_or_else(|_| {
        format!(
            "{}/tests/fixtures/g2config_template.json",
            env!("CARGO_MANIFEST_DIR")
        )
    });

    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"));
    let before: Value = serde_json::from_str(&raw).expect("fixture is not valid JSON");

    let mut config_str = raw.clone();

    // Re-run every rule through set_rule with no field changes: this rebuilds
    // each CFG_ERRULE row via ErruleRow, exercising the full-replace path.
    if let Some(rules_arr) = before["G2_CONFIG"]["CFG_ERRULE"].as_array() {
        for row in rules_arr {
            if let Some(code) = row.get("ERRULE_CODE").and_then(|v| v.as_str()) {
                let params = rules::SetRuleParams {
                    code,
                    resolve: None,
                    relate: None,
                    rtype_id: None,
                    fragment: FieldUpdate::Leave,
                    disqualifier: FieldUpdate::Leave,
                    tier: FieldUpdate::Leave,
                };
                config_str = rules::set_rule(&config_str, params)
                    .unwrap_or_else(|e| panic!("set_rule({code}) failed: {e}"));
            }
        }
    }

    // Re-run every fragment through set_fragment with an empty update.
    if let Some(frags) = before["G2_CONFIG"]["CFG_ERFRAG"].as_array() {
        for row in frags {
            if let Some(code) = row.get("ERFRAG_CODE").and_then(|v| v.as_str()) {
                let empty = SetFragmentParams::default();
                config_str = fragments::set_fragment(&config_str, code, empty)
                    .unwrap_or_else(|e| panic!("set_fragment({code}) failed: {e}"));
            }
        }
    }

    let after: Value = serde_json::from_str(&config_str).unwrap();
    assert_no_keys_dropped(&before, &after);

    if let Ok(out) = std::env::var("SZ_CONFIG_OUT") {
        std::fs::write(&out, serde_json::to_string_pretty(&after).unwrap())
            .unwrap_or_else(|e| panic!("cannot write SZ_CONFIG_OUT '{out}': {e}"));
        eprintln!("round-tripped config written to {out}; load it into the engine to confirm");
    }
}
