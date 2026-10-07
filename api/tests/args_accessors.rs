//! The public `Args` accessors and `convert` helpers that no current manifest
//! arg type reaches through `invoke` (bool args, optional string lists) or
//! whose error sub-cases the dispatcher sweep does not select.

use serde_json::{Value, json};
use sz_configtool_api::{ApiError, Args, convert};

fn reason<T: std::fmt::Debug>(r: Result<T, ApiError>) -> &'static str {
    r.expect_err("expected an error").reason_code()
}

fn args(v: &Value) -> Args<'_> {
    Args::new(v).expect("an object")
}

#[test]
fn test_req_bool_absent_null_wrong_and_value() {
    let v = json!({"b": false, "n": null, "s": "true"});
    let a = args(&v);
    assert!(!a.req_bool("b").unwrap());
    assert_eq!(reason(a.req_bool("absent")), "MISSING_FIELD");
    assert_eq!(reason(a.req_bool("n")), "INVALID_INPUT");
    assert_eq!(reason(a.req_bool("s")), "INVALID_INPUT");
}

#[test]
fn test_opt_bool_absent_null_wrong_and_value() {
    let v = json!({"b": true, "n": null, "i": 1});
    let a = args(&v);
    assert_eq!(a.opt_bool("b").unwrap(), Some(true));
    assert_eq!(a.opt_bool("absent").unwrap(), None);
    assert_eq!(reason(a.opt_bool("n")), "INVALID_INPUT");
    assert_eq!(reason(a.opt_bool("i")), "INVALID_INPUT");
}

#[test]
fn test_opt_str_list_absent_null_wrong_and_value() {
    let v = json!({"l": ["A", "B"], "n": null, "mixed": ["A", 1], "s": "A"});
    let a = args(&v);
    assert_eq!(
        a.opt_str_list("l").unwrap(),
        Some(vec!["A".to_string(), "B".to_string()])
    );
    assert_eq!(a.opt_str_list("absent").unwrap(), None);
    assert_eq!(reason(a.opt_str_list("n")), "INVALID_INPUT");
    assert_eq!(reason(a.opt_str_list("mixed")), "INVALID_INPUT");
    assert_eq!(reason(a.opt_str_list("s")), "INVALID_INPUT");
}

#[test]
fn test_expression_element_list_rejects_non_string_required_keys() {
    let v = json!({
        "element": [{"element": 1, "required": "Yes"}],
        "required": [{"element": "A", "required": true}],
    });
    let a = args(&v);
    for key in ["element", "required"] {
        let err = convert::expression_element_list(&a, key).unwrap_err();
        assert_eq!(err.reason_code(), "INVALID_INPUT", "{key}");
        assert!(
            err.to_string()
                .contains(&format!("{key}[0].{key} must be a string")),
            "{key}: {err}"
        );
    }
}
