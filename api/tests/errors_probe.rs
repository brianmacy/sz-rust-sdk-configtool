//! Probes every manifest function through `invoke` with systematic argument
//! sets (empty args object, only-required args, all args) built from each
//! arg's wire type, against the REAL fixture and a few degenerate configs.
//! Every observed reason code must be listed in the function's `errors[]`,
//! except the universal wire errors (schema.md, Errors). This catches
//! incomplete `errors[]` lists that hand-written conformance cases miss.

mod common;

use std::collections::BTreeSet;

use serde_json::{Map, Value, json};
use sz_configtool_api::invoke;

/// Typed sample values for one arg, one "style" per index.
fn samples(arg: &Value) -> Vec<Value> {
    let ty = arg["type"].as_str().expect("arg type");
    if arg["rust_convert"] == "req_usize" {
        // A width/size (render_config `indent`), not an id: 999_999 hits no
        // error path, it only renders ~10^6 spaces per level, which made that
        // one function ~60% of this test's run time. -1 still covers the
        // negative -> INVALID_INPUT path.
        return vec![json!(9), json!(0), json!(1), json!(-1), json!(2)];
    }
    match ty {
        "str" => vec![
            json!("ZZ_PROBE"),
            json!(""),
            json!("GENDER"),
            json!("1"),
            json!("ZZ_PROBE2"),
        ],
        "int" => vec![json!(999_999), json!(0), json!(1), json!(-1), json!(2)],
        "bool" => vec![
            json!(true),
            json!(false),
            json!(true),
            json!(false),
            json!(true),
        ],
        "json" => vec![
            json!([]),
            json!({}),
            json!([{}]),
            json!("x"),
            json!(["GENDER"]),
        ],
        "str_list" => vec![
            json!(["ZZ_PROBE"]),
            json!([]),
            json!(["GENDER"]),
            json!([""]),
            json!(["NAME"]),
        ],
        "int_or_str" => vec![
            json!(999_999),
            json!(""),
            json!("GENDER"),
            json!(-1),
            json!(1),
        ],
        other => panic!("unknown arg type {other}"),
    }
}

/// Wire-required: absent is a universal MISSING_FIELD from `Args`.
fn wire_required(arg: &Value) -> bool {
    !arg["optional"].as_bool().unwrap_or(false) && !arg["tristate"].as_bool().unwrap_or(false)
}

/// Distinct argument objects to try for one function (duplicates, e.g. the
/// only-required and all-args sets of a function without optional args, or
/// every set of a function without args, are probed once).
fn arg_sets(f: &Value) -> Vec<Map<String, Value>> {
    let args = f["args"].as_array().cloned().unwrap_or_default();
    let mut sets = vec![Map::new()];
    for style in 0..5 {
        for only_required in [true, false] {
            let mut m = Map::new();
            for a in &args {
                let required = wire_required(a) || a["required"].as_bool().unwrap_or(false);
                if only_required && !required {
                    continue;
                }
                let name = a["name"].as_str().expect("arg name").to_string();
                m.insert(name, samples(a)[style].clone());
            }
            if !sets.contains(&m) {
                sets.push(m);
            }
        }
    }
    sets
}

fn configs() -> Vec<String> {
    let suite: Value =
        serde_json::from_str(&common::read(&common::project_path("conformance_json_out")))
            .expect("conformance.json parses");
    let fixture_rel = suite["fixture"].as_str().expect("fixture path");
    let fixture = common::read(&common::workspace_root().join(fixture_rel));
    vec![
        malformed_ids(&fixture),
        fixture,
        r#"{"G2_CONFIG": {}}"#.to_string(),
        "{}".to_string(),
        "not json".to_string(),
    ]
}

/// The fixture with every integer `*_ID` field turned into a string: a
/// structurally malformed config (INVALID_STRUCTURE / MISSING_FIELD paths).
fn malformed_ids(fixture: &str) -> String {
    fn walk(v: &mut Value) {
        match v {
            Value::Object(m) => {
                for (k, child) in m.iter_mut() {
                    if k.ends_with("_ID") && child.is_i64() {
                        *child = Value::String(child.to_string());
                    } else {
                        walk(child);
                    }
                }
            }
            Value::Array(a) => a.iter_mut().for_each(walk),
            _ => {}
        }
    }
    let mut v: Value = serde_json::from_str(fixture).expect("fixture parses");
    walk(&mut v);
    v.to_string()
}

/// Problems found probing one function `f` (empty when its list is complete).
fn probe(f: &Value, configs: &[String]) -> Vec<String> {
    let name = f["name"].as_str().expect("function name");
    let listed: BTreeSet<&str> = f["errors"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let has_wire_required = f["args"]
        .as_array()
        .into_iter()
        .flatten()
        .any(wire_required);
    let mut problems = BTreeSet::new();
    for set in arg_sets(f) {
        let args_text = Value::Object(set.clone()).to_string();
        for config in configs {
            let Err(e) = invoke(name, config, &args_text) else {
                continue;
            };
            let code = e.reason_code();
            let universal = code == "MISSING_FIELD" && set.is_empty() && has_wire_required;
            if !universal && !listed.contains(code) {
                let shown: String = config.chars().take(24).collect();
                problems.insert(format!(
                    "{name}: {code} not in errors[] (args {args_text}, config {shown:?}): {e}"
                ));
            }
        }
    }
    problems.into_iter().collect()
}

#[test]
fn test_observed_reason_codes_are_listed_in_errors() {
    let configs = configs();
    let manifest = common::manifest();
    let functions = manifest["functions"].as_array().expect("functions");
    assert_eq!(
        functions.len(),
        common::function_strings("name").len(),
        "every manifest function is probed"
    );
    let problems: Vec<String> = functions.iter().flat_map(|f| probe(f, &configs)).collect();
    assert!(
        problems.is_empty(),
        "{} incomplete errors[] observations:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn test_probe_flags_an_unlisted_code() {
    // A synthetic entry with an empty errors[] must be reported.
    let f = json!({"name": "get_data_source", "errors": [],
                   "args": [{"name": "code", "type": "str"}]});
    let problems = probe(&f, &configs());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("NOT_FOUND not in errors[]")),
        "{problems:?}"
    );
    assert!(
        !problems.iter().any(|p| p.contains("MISSING_FIELD")),
        "a missing wire-required arg is universal: {problems:?}"
    );
}
