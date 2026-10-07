//! Typed access to an `args_json` object, implementing the wire convention:
//! key absent = Leave/None, `null` = Clear (tri-state args only), value = Set.

use serde_json::{Map, Value};
use sz_configtool_lib::{FieldUpdate, SzConfigError};

use crate::ApiError;

/// A borrowed, validated `args_json` object.
#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    map: &'a Map<String, Value>,
}

fn invalid(msg: String) -> ApiError {
    ApiError::Config(SzConfigError::InvalidInput(msg))
}

fn wrong_type(name: &str, want: &str) -> ApiError {
    invalid(format!("argument '{name}' must be {want}"))
}

impl<'a> Args<'a> {
    /// Wrap a JSON value, which must be an object.
    pub fn new(value: &'a Value) -> Result<Self, ApiError> {
        value
            .as_object()
            .map(|map| Self { map })
            .ok_or_else(|| invalid("args_json must be a JSON object".to_string()))
    }

    /// Reject keys the function does not declare (catches typos on the wire).
    pub fn check_known(&self, known: &[&str]) -> Result<(), ApiError> {
        match self.map.keys().find(|k| !known.contains(&k.as_str())) {
            Some(k) => Err(invalid(format!("unknown argument '{k}'"))),
            None => Ok(()),
        }
    }

    /// The raw value: `None` when absent, `Some(Null)` when explicitly null.
    pub fn raw(&self, name: &str) -> Option<&'a Value> {
        self.map.get(name)
    }

    /// Present and non-null, else an error. Null is rejected for non-tri-state args.
    fn present(&self, name: &str) -> Result<Option<&'a Value>, ApiError> {
        match self.map.get(name) {
            Some(Value::Null) => Err(invalid(format!(
                "argument '{name}' is not tri-state and does not accept null"
            ))),
            other => Ok(other),
        }
    }

    fn required(&self, name: &str) -> Result<&'a Value, ApiError> {
        self.present(name)?
            .ok_or_else(|| ApiError::Config(SzConfigError::MissingField(name.to_string())))
    }

    fn as_str(name: &str, v: &'a Value) -> Result<&'a str, ApiError> {
        v.as_str().ok_or_else(|| wrong_type(name, "a string"))
    }

    fn as_int(name: &str, v: &Value) -> Result<i64, ApiError> {
        v.as_i64().ok_or_else(|| wrong_type(name, "an integer"))
    }

    fn as_bool(name: &str, v: &Value) -> Result<bool, ApiError> {
        v.as_bool().ok_or_else(|| wrong_type(name, "a boolean"))
    }

    fn as_str_list(name: &str, v: &Value) -> Result<Vec<String>, ApiError> {
        v.as_array()
            .and_then(|a| a.iter().map(|s| s.as_str().map(str::to_string)).collect())
            .ok_or_else(|| wrong_type(name, "an array of strings"))
    }

    pub fn req_str(&self, name: &str) -> Result<&'a str, ApiError> {
        Self::as_str(name, self.required(name)?)
    }

    pub fn opt_str(&self, name: &str) -> Result<Option<&'a str>, ApiError> {
        self.present(name)?
            .map(|v| Self::as_str(name, v))
            .transpose()
    }

    pub fn tri_str(&self, name: &str) -> Result<FieldUpdate<&'a str>, ApiError> {
        match self.map.get(name) {
            None => Ok(FieldUpdate::Leave),
            Some(Value::Null) => Ok(FieldUpdate::Clear),
            Some(v) => Self::as_str(name, v).map(FieldUpdate::Set),
        }
    }

    pub fn req_int(&self, name: &str) -> Result<i64, ApiError> {
        Self::as_int(name, self.required(name)?)
    }

    pub fn opt_int(&self, name: &str) -> Result<Option<i64>, ApiError> {
        self.present(name)?
            .map(|v| Self::as_int(name, v))
            .transpose()
    }

    pub fn tri_int(&self, name: &str) -> Result<FieldUpdate<i64>, ApiError> {
        match self.map.get(name) {
            None => Ok(FieldUpdate::Leave),
            Some(Value::Null) => Ok(FieldUpdate::Clear),
            Some(v) => Self::as_int(name, v).map(FieldUpdate::Set),
        }
    }

    pub fn req_bool(&self, name: &str) -> Result<bool, ApiError> {
        Self::as_bool(name, self.required(name)?)
    }

    pub fn opt_bool(&self, name: &str) -> Result<Option<bool>, ApiError> {
        self.present(name)?
            .map(|v| Self::as_bool(name, v))
            .transpose()
    }

    pub fn req_json(&self, name: &str) -> Result<&'a Value, ApiError> {
        self.required(name)
    }

    pub fn opt_json(&self, name: &str) -> Result<Option<&'a Value>, ApiError> {
        self.present(name)
    }

    pub fn req_json_owned(&self, name: &str) -> Result<Value, ApiError> {
        self.required(name).cloned()
    }

    pub fn opt_json_owned(&self, name: &str) -> Result<Option<Value>, ApiError> {
        self.present(name).map(|v| v.cloned())
    }

    pub fn req_str_list(&self, name: &str) -> Result<Vec<String>, ApiError> {
        Self::as_str_list(name, self.required(name)?)
    }

    pub fn opt_str_list(&self, name: &str) -> Result<Option<Vec<String>>, ApiError> {
        self.present(name)?
            .map(|v| Self::as_str_list(name, v))
            .transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn reason(r: Result<impl std::fmt::Debug, ApiError>) -> &'static str {
        r.expect_err("expected an error").reason_code()
    }

    #[test]
    fn test_non_object_is_invalid_input() {
        assert_eq!(reason(Args::new(&json!([1]))), "INVALID_INPUT");
        assert_eq!(reason(Args::new(&json!("x"))), "INVALID_INPUT");
    }

    #[test]
    fn test_required_absent_is_missing_field_and_null_is_invalid() {
        let v = json!({"n": null});
        let a = Args::new(&v).unwrap();
        assert_eq!(reason(a.req_str("code")), "MISSING_FIELD");
        assert_eq!(reason(a.req_str("n")), "INVALID_INPUT");
        assert_eq!(reason(a.opt_str("n")), "INVALID_INPUT");
        assert_eq!(reason(a.opt_int("n")), "INVALID_INPUT");
    }

    #[test]
    fn test_wrong_types_are_invalid_input() {
        let v = json!({"s": 1, "i": "1", "f": 1.5, "b": "true", "l": ["a", 2]});
        let a = Args::new(&v).unwrap();
        assert_eq!(reason(a.req_str("s")), "INVALID_INPUT");
        assert_eq!(reason(a.req_int("i")), "INVALID_INPUT");
        assert_eq!(reason(a.req_int("f")), "INVALID_INPUT");
        assert_eq!(reason(a.req_bool("b")), "INVALID_INPUT");
        assert_eq!(reason(a.req_str_list("l")), "INVALID_INPUT");
    }

    #[test]
    fn test_tristate_absent_null_value() {
        let v = json!({"c": null, "s": "x", "i": 7, "ic": null});
        let a = Args::new(&v).unwrap();
        assert_eq!(a.tri_str("absent").unwrap(), FieldUpdate::Leave);
        assert_eq!(a.tri_str("c").unwrap(), FieldUpdate::Clear);
        assert_eq!(a.tri_str("s").unwrap(), FieldUpdate::Set("x"));
        assert_eq!(a.tri_int("i").unwrap(), FieldUpdate::Set(7));
        assert_eq!(a.tri_int("ic").unwrap(), FieldUpdate::Clear);
        assert_eq!(reason(a.tri_int("s")), "INVALID_INPUT");
    }

    #[test]
    fn test_optional_and_values() {
        let v = json!({"s": "x", "i": -3, "b": true, "j": {"k": 1}, "l": ["a", "b"]});
        let a = Args::new(&v).unwrap();
        assert_eq!(a.opt_str("absent").unwrap(), None);
        assert_eq!(a.opt_str("s").unwrap(), Some("x"));
        assert_eq!(a.opt_int("i").unwrap(), Some(-3));
        assert_eq!(a.opt_bool("b").unwrap(), Some(true));
        assert_eq!(a.req_json("j").unwrap(), &json!({"k": 1}));
        assert_eq!(a.opt_json_owned("j").unwrap(), Some(json!({"k": 1})));
        assert_eq!(a.req_json_owned("j").unwrap(), json!({"k": 1}));
        assert_eq!(a.opt_json("absent").unwrap(), None);
        assert_eq!(a.req_str_list("l").unwrap(), vec!["a", "b"]);
        assert_eq!(a.opt_str_list("absent").unwrap(), None);
        assert_eq!(a.raw("absent"), None);
    }

    #[test]
    fn test_check_known_rejects_unknown_keys() {
        let v = json!({"code": "X", "cdoe": "Y"});
        let a = Args::new(&v).unwrap();
        assert!(a.check_known(&["code", "cdoe"]).is_ok());
        assert_eq!(reason(a.check_known(&["code"])), "INVALID_INPUT");
    }
}
