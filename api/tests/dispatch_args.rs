//! Manifest-driven sweep of the generated dispatcher: for EVERY function and
//! EVERY declared arg, `invoke` rejects that arg when it is mistyped
//! (`INVALID_INPUT`) and, for a non-optional arg, when it is absent
//! (`MISSING_FIELD`). Every function is also called once with every arg
//! well-typed against the REAL template config, so the library call itself
//! runs (success or a library reason code listed in the function's `errors`).

mod common;

use serde_json::{Map, Value, json};
use sz_configtool_api::invoke;

/// A well-typed wire value for `arg` (the dispatcher's type check passes).
fn valid_value(arg: &Value) -> Value {
    match (arg["type"].as_str(), arg["rust_convert"].as_str()) {
        (Some("json"), Some("expression_element_list" | "search_profile_elements")) => json!([]),
        (Some("json"), _) => json!({}),
        (Some("int"), _) => json!(1),
        (Some("bool"), _) => json!(true),
        (Some("str_list"), _) => json!(["X"]),
        (Some("str" | "int_or_str"), _) => json!("X"),
        (other, _) => panic!("unhandled manifest arg type {other:?} in {arg}"),
    }
}

/// A value the dispatcher must reject as `INVALID_INPUT` for `arg`: `null`
/// for a json arg (only tri-state args accept null, and json is never
/// tri-state), an object for every other type.
fn wrong_value(arg: &Value) -> Value {
    match arg["type"].as_str() {
        Some("json") => Value::Null,
        _ => json!({}),
    }
}

fn args_of(f: &Value) -> &Vec<Value> {
    f["args"].as_array().expect("args array")
}

fn name_of(v: &Value) -> &str {
    v["name"].as_str().expect("name string")
}

/// Every arg of `f` well-typed, except `skip` (omitted) and `replace`.
fn all_valid(f: &Value, skip: Option<&str>, replace: Option<(&str, Value)>) -> String {
    let mut map = Map::new();
    for a in args_of(f) {
        let name = name_of(a);
        if Some(name) == skip {
            continue;
        }
        let value = match &replace {
            Some((n, v)) if *n == name => v.clone(),
            _ => valid_value(a),
        };
        map.insert(name.to_string(), value);
    }
    Value::Object(map).to_string()
}

fn reason(func: &str, config: &str, args_json: &str) -> &'static str {
    invoke(func, config, args_json)
        .map(|out| panic!("{func} {args_json}: expected an error, got {out:?}"))
        .unwrap_or_else(|e| e.reason_code())
}

fn functions() -> Vec<Value> {
    assert_eq!(
        common::function_strings("name"),
        sz_configtool_api::FUNCTION_NAMES,
        "the sweep covers exactly the dispatched functions"
    );
    common::manifest()["functions"]
        .as_array()
        .expect("functions array")
        .clone()
}

fn fixture() -> String {
    common::read(&common::project_path("fixture"))
}

#[test]
fn test_every_mistyped_arg_is_invalid_input() {
    let config = fixture();
    for f in functions() {
        let func = name_of(&f);
        for a in args_of(&f) {
            let args_json = all_valid(&f, None, Some((name_of(a), wrong_value(a))));
            assert_eq!(
                reason(func, &config, &args_json),
                "INVALID_INPUT",
                "{func}.{} = {args_json}",
                name_of(a)
            );
        }
    }
}

#[test]
fn test_every_absent_non_optional_arg_is_missing_field() {
    let config = fixture();
    let mut checked = 0;
    for f in functions() {
        let func = name_of(&f);
        // Absent tri-state args mean Leave, and `required: true` args are
        // `optional` on the wire (the library reports their absence).
        let mandatory = args_of(&f)
            .iter()
            .filter(|a| a["optional"] == false && a["tristate"] == false);
        for a in mandatory {
            let args_json = all_valid(&f, Some(name_of(a)), None);
            assert_eq!(
                reason(func, &config, &args_json),
                "MISSING_FIELD",
                "{func} without {}: {args_json}",
                name_of(a)
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "the manifest declares mandatory args");
}

#[test]
fn test_every_function_runs_with_well_typed_args() {
    let config = fixture();
    for f in functions() {
        let func = name_of(&f);
        let args_json = all_valid(&f, None, None);
        if let Err(e) = invoke(func, &config, &args_json) {
            let listed = f["errors"]
                .as_array()
                .expect("errors array")
                .iter()
                .any(|code| code == e.reason_code());
            assert!(
                listed,
                "{func} {args_json}: {} ({e}) is not in its manifest errors",
                e.reason_code()
            );
        }
    }
}

#[test]
fn test_every_function_rejects_unknown_arg_and_non_object_args() {
    let config = fixture();
    for f in functions() {
        let func = name_of(&f);
        let mut args: Value = serde_json::from_str(&all_valid(&f, None, None)).unwrap();
        args["zz_not_an_arg"] = json!(1);
        assert_eq!(
            reason(func, &config, &args.to_string()),
            "INVALID_INPUT",
            "{func}"
        );
        assert_eq!(reason(func, &config, "[]"), "INVALID_INPUT", "{func}");
    }
}
