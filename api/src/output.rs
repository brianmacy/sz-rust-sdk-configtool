//! The successful result of an `invoke`, and its JSON envelope for the C ABI
//! and RPC transports.

use serde::Serialize;
use serde_json::{Value, json};

use crate::ApiError;

/// A function's successful result, shaped by its manifest `returns`.
///
/// `config` strings are OPAQUE: they are exactly what the root library
/// produced and are never parsed or re-serialized here.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// `returns: config` — the modified configuration.
    Config(String),
    /// `returns: json` — a JSON result (row, list, scalar, ...).
    Json(Value),
    /// `returns: config_and_json` — the modified configuration AND a record
    /// (e.g. the new row). Neither half is ever dropped.
    ConfigAndJson { config: String, record: Value },
    /// `returns: int`.
    Int(i64),
    /// `returns: unit` — success with no value.
    Unit,
}

impl Output {
    /// The manifest `returns` value this output corresponds to.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Config(_) => "config",
            Self::Json(_) => "json",
            Self::ConfigAndJson { .. } => "config_and_json",
            Self::Int(_) => "int",
            Self::Unit => "unit",
        }
    }

    /// The modified configuration, if this output carries one.
    pub fn config(&self) -> Option<&str> {
        match self {
            Self::Config(c) | Self::ConfigAndJson { config: c, .. } => Some(c),
            _ => None,
        }
    }

    /// The non-config result value (`json` result, record, or int), if any.
    pub fn result(&self) -> Option<Value> {
        match self {
            Self::Json(v) | Self::ConfigAndJson { record: v, .. } => Some(v.clone()),
            Self::Int(i) => Some(Value::from(*i)),
            Self::Config(_) | Self::Unit => None,
        }
    }

    /// The wire envelope: `{"kind": ..., "config"?: "<string>", "result"?: ...}`.
    ///
    /// `config` is carried as a JSON *string* holding the exact bytes the
    /// library produced (escaping is not re-serialization), so the caller
    /// recovers it byte-for-byte by decoding that string.
    pub fn to_envelope(&self) -> String {
        let mut env = json!({ "kind": self.kind() });
        if let Some(c) = self.config() {
            env["config"] = Value::String(c.to_string());
        }
        if let Some(r) = self.result() {
            env["result"] = r;
        }
        env.to_string()
    }
}

/// `returns: json` — serialize any library result.
pub(crate) fn json<T: Serialize>(value: T) -> Result<Output, ApiError> {
    serde_json::to_value(value)
        .map(Output::Json)
        .map_err(|e| ApiError::Internal(format!("serializing result: {e}")))
}

/// `returns: config_and_json` — keep both halves.
pub(crate) fn config_and_json<T: Serialize>(config: String, record: T) -> Result<Output, ApiError> {
    serde_json::to_value(record)
        .map(|record| Output::ConfigAndJson { config, record })
        .map_err(|e| ApiError::Internal(format!("serializing record: {e}")))
}

/// `status: not_implemented` — the library call always fails, and its error
/// (`NOT_IMPLEMENTED`) is the result. A success means the manifest status is
/// stale, which is reported as `INTERNAL` rather than shaped by a guess.
// Called by generated dispatch only for `status: not_implemented` functions,
// of which the manifest has none today (schema support is kept); `expect`
// fails the build once one exists, so this attribute cannot go stale.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no status: not_implemented function in the manifest"
    )
)]
pub(crate) fn not_implemented<T>(
    name: &str,
    result: Result<T, sz_configtool_lib::SzConfigError>,
) -> Result<Output, ApiError> {
    result?;
    Err(ApiError::Internal(format!(
        "{name} is marked status: not_implemented in the manifest but succeeded"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use sz_configtool_lib::SzConfigError;

    /// A map with non-string keys: serde_json cannot represent it.
    fn unserializable() -> BTreeMap<(i32, i32), i32> {
        BTreeMap::from([((1, 2), 3)])
    }

    #[test]
    fn test_unserializable_results_are_internal() {
        let err = json(unserializable()).unwrap_err();
        assert_eq!(err.reason_code(), "INTERNAL");
        assert!(err.to_string().contains("serializing result"), "{err}");
        let err = config_and_json("c".into(), unserializable()).unwrap_err();
        assert_eq!(err.reason_code(), "INTERNAL");
        assert!(err.to_string().contains("serializing record"), "{err}");
    }

    #[test]
    fn test_not_implemented_passes_error_and_rejects_success() {
        let lib_err = SzConfigError::NotImplemented("f".into());
        let err = not_implemented::<()>("f", Err(lib_err)).unwrap_err();
        assert_eq!(err.reason_code(), "NOT_IMPLEMENTED");
        let err = not_implemented("f", Ok(1)).unwrap_err();
        assert_eq!(err.reason_code(), "INTERNAL");
        assert!(
            err.to_string()
                .contains("f is marked status: not_implemented"),
            "{err}"
        );
    }

    #[test]
    fn test_envelope_shapes() {
        let cfg = r#"{"G2_CONFIG": {"x": "a\"b"}}"#.to_string();
        let env: Value = serde_json::from_str(&Output::Config(cfg.clone()).to_envelope()).unwrap();
        assert_eq!(env["kind"], "config");
        assert_eq!(env["config"].as_str(), Some(cfg.as_str()));
        assert!(env.get("result").is_none());

        let both = Output::ConfigAndJson {
            config: cfg.clone(),
            record: json!({"ID": 1}),
        };
        let env: Value = serde_json::from_str(&both.to_envelope()).unwrap();
        assert_eq!(env["config"].as_str(), Some(cfg.as_str()));
        assert_eq!(env["result"], json!({"ID": 1}));

        let env: Value = serde_json::from_str(&Output::Int(7).to_envelope()).unwrap();
        assert_eq!(env, json!({"kind": "int", "result": 7}));
        let env: Value = serde_json::from_str(&Output::Unit.to_envelope()).unwrap();
        assert_eq!(env, json!({"kind": "unit"}));
        let env: Value = serde_json::from_str(&Output::Json(json!([1])).to_envelope()).unwrap();
        assert_eq!(env, json!({"kind": "json", "result": [1]}));
    }

    #[test]
    fn test_json_helpers() {
        assert_eq!(json(vec![1, 2]).unwrap(), Output::Json(json!([1, 2])));
        assert_eq!(
            config_and_json("c".into(), 5_i64).unwrap(),
            Output::ConfigAndJson {
                config: "c".into(),
                record: json!(5)
            }
        );
    }
}
