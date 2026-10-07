//! Runs every conformance case (api/manifest/conformance/*.yaml, via the
//! generated conformance.json) through `invoke` against the REAL template
//! config. Every language binding reruns the same cases with the same
//! matching rules (documented in api/manifest/schema.md).

mod common;

use std::collections::BTreeSet;

use serde_json::Value;
use sz_configtool_api::{Output, invoke};

/// `expected` is a subset of `actual`: objects by key (recursively), arrays
/// element-wise with equal length, scalars by equality.
fn subset(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Object(e), Value::Object(a)) => e
            .iter()
            .all(|(k, ev)| a.get(k).is_some_and(|av| subset(ev, av))),
        (Value::Array(e), Value::Array(a)) => {
            e.len() == a.len() && e.iter().zip(a).all(|(ev, av)| subset(ev, av))
        }
        _ => expected == actual,
    }
}

fn check_success(expect: &Value, out: &Output) -> Result<(), String> {
    if let Some(kind) = expect["kind"].as_str()
        && kind != out.kind()
    {
        return Err(format!("kind {} != expected {kind}", out.kind()));
    }
    let result = out.result().unwrap_or(Value::Null);
    if let Some(want) = expect.get("result")
        && !subset(want, &result)
    {
        return Err(format!("result {result} does not contain {want}"));
    }
    let inspects_array = ["len", "contains", "excludes"]
        .iter()
        .any(|k| expect.get(*k).is_some());
    if !inspects_array {
        return Ok(());
    }
    let items = result
        .as_array()
        .ok_or_else(|| format!("len/contains/excludes need an array result, got {result}"))?;
    if let Some(len) = expect["len"].as_u64()
        && items.len() as u64 != len
    {
        return Err(format!("len {} != expected {len}", items.len()));
    }
    for want in expect["contains"].as_array().into_iter().flatten() {
        if !items.iter().any(|item| subset(want, item)) {
            return Err(format!("no element matches {want}"));
        }
    }
    for unwanted in expect["excludes"].as_array().into_iter().flatten() {
        if items.iter().any(|item| subset(unwanted, item)) {
            return Err(format!("an element matches excluded {unwanted}"));
        }
    }
    Ok(())
}

/// Run one step; on success with a config, advance `config`.
fn run_step(step: &Value, config: &mut String) -> Result<(), String> {
    let func = step["fn"].as_str().ok_or("step without fn")?;
    let input = step["config_literal"]
        .as_str()
        .unwrap_or(config)
        .to_string();
    let outcome = invoke(func, &input, &step["args"].to_string());
    let expect = &step["expect"];
    match (expect["error"].as_str(), outcome) {
        (Some(code), Err(e)) if e.reason_code() == code => Ok(()),
        (Some(code), Err(e)) => Err(format!("{func}: error {} ({e}) != {code}", e.reason_code())),
        (Some(code), Ok(out)) => Err(format!(
            "{func}: succeeded ({}) but expected {code}",
            out.kind()
        )),
        (None, Err(e)) => Err(format!("{func}: failed {} ({e})", e.reason_code())),
        (None, Ok(out)) => {
            check_success(expect, &out).map_err(|m| format!("{func}: {m}"))?;
            if let Some(c) = out.config() {
                *config = c.to_string();
            }
            Ok(())
        }
    }
}

fn conformance() -> Value {
    let text = common::read(&common::project_path("conformance_json_out"));
    serde_json::from_str(&text).expect("conformance.json parses")
}

#[test]
fn test_conformance_cases_pass_against_real_fixture() {
    let suite = conformance();
    let fixture_rel = suite["fixture"].as_str().expect("fixture path");
    let fixture = common::read(&common::workspace_root().join(fixture_rel));
    let cases = suite["cases"].as_array().expect("cases");
    assert!(!cases.is_empty(), "no conformance cases");
    let mut failures = Vec::new();
    for case in cases {
        let id = format!("{}/{}", case["group"].as_str().unwrap_or("?"), case["name"]);
        let mut config = fixture.clone();
        for (i, step) in case["steps"].as_array().into_iter().flatten().enumerate() {
            if let Err(m) = run_step(step, &mut config) {
                failures.push(format!("{id} step {i}: {m}"));
                break;
            }
        }
    }
    assert!(
        failures.is_empty(),
        "conformance failures:\n{}",
        failures.join("\n")
    );
}

#[test]
fn test_every_manifest_function_has_a_conformance_case() {
    let exercised: BTreeSet<String> = conformance()["cases"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|c| c["steps"].as_array().cloned().unwrap_or_default())
        .filter_map(|s| s["fn"].as_str().map(str::to_string))
        .collect();
    let missing: Vec<String> = common::function_strings("name")
        .into_iter()
        .filter(|n| !exercised.contains(n))
        .collect();
    assert!(
        missing.is_empty(),
        "functions without a conformance case: {missing:?}"
    );
}

#[test]
fn test_array_checks_fail_on_non_array_results() {
    use serde_json::json;
    // An object/int result must never satisfy `len`/`contains`/`excludes`
    // vacuously (it used to be treated as `[]`).
    let object = Output::Json(json!({"a": 1}));
    for expect in [
        json!({"len": 0}),
        json!({"excludes": [{"a": 1}]}),
        json!({"contains": [{"a": 1}]}),
    ] {
        assert!(check_success(&expect, &object).is_err(), "{expect}");
        assert!(check_success(&expect, &Output::Int(3)).is_err(), "{expect}");
    }
    let array = Output::Json(json!([{"a": 1}]));
    assert!(check_success(&json!({"len": 1, "contains": [{"a": 1}]}), &array).is_ok());
    assert!(check_success(&json!({"excludes": [{"a": 2}]}), &array).is_ok());
    assert!(check_success(&json!({"len": 0}), &array).is_err());
}

#[test]
fn test_subset_rules() {
    use serde_json::json;
    assert!(subset(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
    assert!(!subset(&json!({"a": 1}), &json!({"a": 2})));
    assert!(!subset(&json!({"c": null}), &json!({})));
    assert!(subset(&json!({"c": null}), &json!({"c": null})));
    assert!(subset(&json!([{"a": 1}]), &json!([{"a": 1, "b": 2}])));
    assert!(!subset(&json!([{"a": 1}]), &json!([{"a": 1}, {"a": 2}])));
}
