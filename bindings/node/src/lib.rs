//! # sz-configtool-node
//!
//! The Node.js native seam (napi-rs v3) over [`sz_configtool_api::invoke`].
//! It exposes exactly ONE function to JavaScript, `invoke(name, config,
//! argsJson)`; the typed TypeScript wrappers in `ts/generated/` are generated
//! from `api/manifest/*.yaml` and all call it (see `bindings/CONTRACT.md`).
//!
//! * Success returns `{ kind, config?, result? }`: `config` is the library's
//!   configuration string, byte-exact (never parsed here); `result` is the
//!   JSON TEXT of the result value (the TS layer decides whether to parse).
//! * Failure throws a JS `Error` carrying `reasonCode` (one of the 14 wire
//!   reason codes), `kind` (the same reason code: the reason code IS the
//!   error kind in every binding) and, for `VALIDATION_ERRORS`, `details` (JSON text,
//!   `sz-configtool.validation-errors/v1`).
//! * A Rust panic anywhere in the call is caught and reported as `INTERNAL`.

use std::panic::{AssertUnwindSafe, catch_unwind};

use napi::bindgen_prelude::JsObjectValue;
use napi::{Env, Error, Result, Status};
use napi_derive::napi;
use sz_configtool_api::{ApiError, Output};

/// The success envelope handed to JavaScript.
#[napi(object)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeOutput {
    /// The manifest `returns` value: `config`, `json`, `config_and_json`,
    /// `int` or `unit`.
    pub kind: String,
    /// The modified configuration (`config`, `config_and_json`), byte-exact.
    pub config: Option<String>,
    /// JSON text of the result (`json`, `config_and_json` record, `int`).
    pub result: Option<String>,
}

/// A failure, before it becomes a JS error object.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Failure {
    reason_code: &'static str,
    message: String,
    details: Option<String>,
}

fn to_output(out: &Output) -> InvokeOutput {
    InvokeOutput {
        kind: out.kind().to_string(),
        config: out.config().map(str::to_string),
        result: out.result().map(|v| v.to_string()),
    }
}

fn failure(e: &ApiError) -> Failure {
    Failure {
        reason_code: e.reason_code(),
        message: e.to_string(),
        details: e.details_json(),
    }
}

fn internal(message: String) -> Failure {
    Failure {
        reason_code: "INTERNAL",
        message,
        details: None,
    }
}

/// Run `call`, turning a panic into an `INTERNAL` failure.
fn guarded<F>(call: F) -> std::result::Result<InvokeOutput, Failure>
where
    F: FnOnce() -> std::result::Result<Output, ApiError>,
{
    match catch_unwind(AssertUnwindSafe(call)) {
        Ok(Ok(out)) => Ok(to_output(&out)),
        Ok(Err(e)) => Err(failure(&e)),
        Err(_) => Err(internal("panic in native invoke".to_string())),
    }
}

/// Build the JS error object, throw it, and return the matching Rust error.
fn throw_failure(env: &Env, f: &Failure) -> Error {
    // napi_create_error / napi_set_named_property / napi_throw fail only with
    // an invalid env or handle scope; such a failure is returned as is.
    let props = [
        Some(("code", f.reason_code)),
        Some(("reasonCode", f.reason_code)),
        Some(("kind", f.reason_code)),
        f.details.as_deref().map(|d| ("details", d)),
    ];
    env.create_error(Error::from_reason(f.message.clone()))
        .and_then(|mut obj| {
            props
                .into_iter()
                .flatten()
                .try_for_each(|(key, value)| obj.set_named_property(key, value))
                .and_then(|()| env.throw(obj))
        })
        .err()
        .unwrap_or_else(|| Error::from_status(Status::PendingException))
}

/// Call manifest function `name` on `config` with `argsJson` (a JSON object
/// keyed by snake_case arg names; absent/`undefined` means `{}`).
#[napi]
pub fn invoke(
    env: Env,
    name: String,
    config: String,
    args_json: Option<String>,
) -> Result<InvokeOutput> {
    let args = args_json.as_deref().unwrap_or("{}");
    guarded(|| sz_configtool_api::invoke(&name, &config, args)).map_err(|f| throw_failure(&env, &f))
}

/// The library version: [`sz_configtool_api::LIBRARY_VERSION`] (single source).
#[napi]
pub fn library_version() -> &'static str {
    sz_configtool_api::LIBRARY_VERSION
}

/// The C ABI version: [`sz_configtool_api::ABI_VERSION`] (single source).
#[napi]
pub fn abi_version() -> i32 {
    sz_configtool_api::ABI_VERSION
}

/// The wire error taxonomy (the 14 reason codes), for the TS layer's tests.
#[napi]
pub fn reason_codes() -> Vec<String> {
    sz_configtool_api::REASON_CODES
        .iter()
        .map(|c| c.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use sz_configtool_api::SzConfigError;

    const CFG: &str = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;

    #[test]
    fn test_success_envelope_keeps_config_bytes() {
        let out = guarded(|| sz_configtool_api::invoke("add_data_source", CFG, r#"{"code":"x"}"#))
            .unwrap();
        assert_eq!(out.kind, "config");
        assert!(out.config.unwrap().contains("\"DSRC_CODE\":\"X\""));
        assert!(out.result.is_none());
    }

    #[test]
    fn test_json_result_is_text() {
        let out = guarded(|| sz_configtool_api::invoke("list_data_sources", CFG, "{}")).unwrap();
        assert_eq!(out.kind, "json");
        assert_eq!(out.result.as_deref(), Some("[]"));
        assert!(out.config.is_none());
    }

    #[test]
    fn test_library_error_maps_reason_and_kind() {
        let f = guarded(|| sz_configtool_api::invoke("get_data_source", CFG, r#"{"code":"Q"}"#))
            .unwrap_err();
        assert_eq!(f.reason_code, "NOT_FOUND");
        assert!(f.details.is_none());
    }

    #[test]
    fn test_panic_becomes_internal() {
        let f = guarded(|| panic!("boom")).unwrap_err();
        assert_eq!(f.reason_code, "INTERNAL");
    }

    #[test]
    fn test_internal_api_error_kind() {
        let f = guarded(|| Err(ApiError::Internal("x".into()))).unwrap_err();
        assert_eq!(f, internal("internal error: x".to_string()));
    }

    /// A real VALIDATION_ERRORS from the library (no hand-built failure list).
    fn real_validation_error() -> ApiError {
        // The fixture path comes from the manifest (project.yaml paths.fixture).
        let manifest: Value = serde_json::from_str(sz_configtool_api::MANIFEST_JSON).unwrap();
        let fixture = manifest["paths"]["fixture"].as_str().unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let template = std::fs::read_to_string(root.join(fixture)).unwrap();
        let err = sz_configtool_api::invoke(
            "set_generic_threshold",
            &template,
            r#"{"plan":"INGEST","behavior":"NAME","send_to_redo":"sometimes"}"#,
        )
        .unwrap_err();
        assert_eq!(err.reason_code(), "VALIDATION_ERRORS", "{err}");
        err
    }

    #[test]
    fn test_validation_details_shape() {
        let f = failure(&real_validation_error());
        assert_eq!(f.reason_code, "VALIDATION_ERRORS");
        let d: Value = serde_json::from_str(f.details.as_deref().unwrap()).unwrap();
        assert_eq!(d["schema"], sz_configtool_api::VALIDATION_DETAILS_SCHEMA);
        assert_eq!(
            d["failures"],
            json!([{"field": "sendToRedo", "reasonCode": d["failures"][0]["reasonCode"], "offendingValue": "sometimes"}])
        );
        assert!(d["failures"][0]["reasonCode"].is_string());
        let not_found = failure(&ApiError::Config(SzConfigError::NotFound("x".into())));
        assert!(not_found.details.is_none());
    }

    #[test]
    fn test_versions_are_the_single_source() {
        assert_eq!(library_version(), sz_configtool_api::LIBRARY_VERSION);
        assert_eq!(abi_version(), sz_configtool_api::ABI_VERSION);
    }

    #[test]
    fn test_reason_codes_are_the_taxonomy() {
        assert_eq!(reason_codes().len(), 14);
    }
}
