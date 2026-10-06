//! # sz-configtool-api
//!
//! The dynamic seam over [`sz_configtool_lib`]: [`invoke`] calls any manifest
//! function by name with a JSON object of arguments. It backs the C ABI's
//! `SzConfigTool_invoke` and RPC transports (tRPC). Typed language bindings
//! call the root library directly instead.
//!
//! The dispatcher (`dispatch_gen.rs`) is GENERATED from `api/manifest/*.yaml`
//! by `cargo run -p sz-configtool-codegen`; see `api/manifest/schema.md`.
//!
//! ## Wire convention for `args_json`
//! A JSON object keyed by the manifest arg `name`: key absent = Leave/None,
//! `null` = Clear (tri-state args only; an error elsewhere), value = Set.
//! Unknown keys are rejected.
//!
//! ## Example
//! ```
//! let config = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;
//! let out = sz_configtool_api::invoke("add_data_source", config, r#"{"code": "crm"}"#)?;
//! assert_eq!(out.kind(), "config");
//! let err = sz_configtool_api::invoke("no_such_fn", config, "{}").unwrap_err();
//! assert_eq!(err.reason_code(), "INVALID_INPUT");
//! # Ok::<(), sz_configtool_api::ApiError>(())
//! ```

mod args;
pub mod convert;
#[rustfmt::skip]
mod dispatch_gen;
mod output;

use std::panic::{AssertUnwindSafe, catch_unwind};

use serde_json::Value;
pub use sz_configtool_lib::SzConfigError;
use sz_configtool_lib::error::ValidationFailure;

pub use args::Args;
pub use dispatch_gen::{FUNCTION_NAMES, MANIFEST_JSON};
pub use output::Output;

/// The complete wire error taxonomy: the 13 [`SzConfigError::reason_code`]
/// values plus `INTERNAL`. Mirrors `reason_codes` in `api/manifest/project.yaml`
/// (a drift test keeps them equal).
pub const REASON_CODES: [&str; 14] = [
    "JSON_PARSE",
    "NOT_FOUND",
    "NOT_ON_CALL",
    "NOT_IN_FEATURE",
    "ALREADY_EXISTS",
    "ALREADY_PRESENT",
    "INVALID_INPUT",
    "MISSING_SECTION",
    "INVALID_STRUCTURE",
    "MISSING_FIELD",
    "INVALID_CONFIG",
    "NOT_IMPLEMENTED",
    "VALIDATION_ERRORS",
    "INTERNAL",
];

/// NUL-terminated [`LIBRARY_VERSION`], for the C ABI
/// (`SzConfigTool_getLibraryVersion` returns a pointer into it).
#[doc(hidden)]
pub const LIBRARY_VERSION_NUL: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

/// The library version (e.g. `"4.4.0-1"`): the workspace version shared by
/// `sz_configtool_lib`, this crate, the C ABI library and every binding seam.
/// The single definition every binding's `library_version` accessor returns.
pub const LIBRARY_VERSION: &str = match LIBRARY_VERSION_NUL.as_bytes().split_last() {
    Some((_, version)) => match std::str::from_utf8(version) {
        Ok(v) => v,
        Err(_) => panic!("CARGO_PKG_VERSION is UTF-8"),
    },
    None => panic!("LIBRARY_VERSION_NUL is NUL-terminated"),
};

/// ABI version of the C interface (`SZCONFIGTOOL_ABI_VERSION` in
/// `libSzConfigTool.h`). Bumped only on an incompatible change to an existing
/// C declaration; additions keep the same value. Every binding exposes it as
/// its `abi_version` accessor.
pub const ABI_VERSION: i32 = 2;

/// An `invoke` failure: a library error, or an internal boundary failure.
#[derive(Debug)]
pub enum ApiError {
    /// An error from the root library (or argument validation, which reuses
    /// its kinds: `INVALID_INPUT`, `MISSING_FIELD`).
    Config(SzConfigError),
    /// Result serialization failure or a caught panic (`INTERNAL`).
    Internal(String),
}

impl ApiError {
    /// Stable reason code; one of [`REASON_CODES`].
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::Config(e) => e.reason_code(),
            Self::Internal(_) => "INTERNAL",
        }
    }

    /// The underlying library error, if any (e.g. for validation details).
    pub fn config_error(&self) -> Option<&SzConfigError> {
        match self {
            Self::Config(e) => Some(e),
            Self::Internal(_) => None,
        }
    }

    /// The versioned validation details JSON (see [`validation_details_json`])
    /// when this is a `VALIDATION_ERRORS` failure; `None` otherwise.
    pub fn details_json(&self) -> Option<String> {
        self.config_error()?
            .validation_failures()
            .map(validation_details_json)
    }
}

/// Schema id of the structured validation details payload.
pub const VALIDATION_DETAILS_SCHEMA: &str = "sz-configtool.validation-errors/v1";

/// The versioned, namespaced validation details payload shared by every
/// binding (C `SzConfigTool_getLastErrorDetails`, Python, Java, Node):
/// `{"schema": VALIDATION_DETAILS_SCHEMA, "failures": [{"field",
/// "reasonCode", "offendingValue"}...]}`.
pub fn validation_details_json(failures: &[ValidationFailure]) -> String {
    let arr: Vec<Value> = failures
        .iter()
        .map(|f| {
            serde_json::json!({
                "field": f.field,
                "reasonCode": f.reason_code.as_str(),
                "offendingValue": f.offending_value,
            })
        })
        .collect();
    serde_json::json!({
        "schema": VALIDATION_DETAILS_SCHEMA,
        "failures": arr,
    })
    .to_string()
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(e) => write!(f, "{e}"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<SzConfigError> for ApiError {
    fn from(e: SzConfigError) -> Self {
        Self::Config(e)
    }
}

fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "non-string panic payload".to_string())
}

/// Call manifest function `name` on `config` with `args_json` (a JSON object).
///
/// Errors: unknown `name`, unparsable/non-object `args_json`, unknown or
/// mistyped args -> `INVALID_INPUT`; a missing required arg -> `MISSING_FIELD`;
/// library errors keep their own reason code; a panic -> `INTERNAL`.
pub fn invoke(name: &str, config: &str, args_json: &str) -> Result<Output, ApiError> {
    let handler = dispatch_gen::lookup(name).ok_or_else(|| {
        ApiError::Config(SzConfigError::InvalidInput(format!(
            "unknown function '{name}'"
        )))
    })?;
    let value: Value = serde_json::from_str(args_json).map_err(|e| {
        ApiError::Config(SzConfigError::InvalidInput(format!(
            "args_json is not valid JSON: {e}"
        )))
    })?;
    let args = Args::new(&value)?;
    guarded(name, || handler(config, &args))
}

/// Run `call`, converting a panic into `INTERNAL` ("panic in {name}: ...") so
/// a library defect never unwinds into a binding.
fn guarded(
    name: &str,
    call: impl FnOnce() -> Result<Output, ApiError>,
) -> Result<Output, ApiError> {
    catch_unwind(AssertUnwindSafe(call)).unwrap_or_else(|payload| {
        Err(ApiError::Internal(format!(
            "panic in {name}: {}",
            panic_text(payload.as_ref())
        )))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CFG: &str = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;

    #[test]
    fn test_unknown_function_is_invalid_input() {
        let err = invoke("nope", CFG, "{}").unwrap_err();
        assert_eq!(err.reason_code(), "INVALID_INPUT");
        assert!(err.to_string().contains("unknown function 'nope'"));
    }

    #[test]
    fn test_removed_hash_functions_are_unknown() {
        // Removed in ABI 2: SYS_OOM is an array in real configs and G2's CLI
        // retired the commands.
        for name in [
            "add_to_name_hash",
            "delete_from_name_hash",
            "add_to_ssn_last4_hash",
            "delete_from_ssn_last4_hash",
        ] {
            let err = invoke(name, CFG, r#"{"name":"X"}"#).unwrap_err();
            assert_eq!(err.reason_code(), "INVALID_INPUT", "{name}");
            assert!(err.to_string().contains("unknown function"), "{name}");
        }
    }

    #[test]
    fn test_bad_args_json() {
        for bad in ["", "not json", "[]", "\"x\"", "null"] {
            let err = invoke("list_data_sources", CFG, bad).unwrap_err();
            assert_eq!(err.reason_code(), "INVALID_INPUT", "args {bad:?}");
        }
    }

    #[test]
    fn test_arg_validation_reasons() {
        let unknown = invoke("add_data_source", CFG, r#"{"code":"A","cdoe":"B"}"#).unwrap_err();
        assert_eq!(unknown.reason_code(), "INVALID_INPUT");
        let missing = invoke("add_data_source", CFG, "{}").unwrap_err();
        assert_eq!(missing.reason_code(), "MISSING_FIELD");
        let mistyped = invoke("add_data_source", CFG, r#"{"code":1}"#).unwrap_err();
        assert_eq!(mistyped.reason_code(), "INVALID_INPUT");
        let null = invoke("add_data_source", CFG, r#"{"code":"A","id":null}"#).unwrap_err();
        assert_eq!(null.reason_code(), "INVALID_INPUT");
    }

    #[test]
    fn test_config_is_passed_through_opaquely() {
        let direct = sz_configtool_lib::datasources::add_data_source(
            CFG,
            sz_configtool_lib::datasources::AddDataSourceParams {
                code: "crm",
                ..Default::default()
            },
        )
        .unwrap();
        let out = invoke("add_data_source", CFG, r#"{"code":"crm"}"#).unwrap();
        assert_eq!(out, Output::Config(direct));
    }

    #[test]
    fn test_library_error_keeps_reason_and_source() {
        let err = invoke("get_data_source", CFG, r#"{"code":"X"}"#).unwrap_err();
        assert_eq!(err.reason_code(), "NOT_FOUND");
        assert_eq!(
            err.config_error().map(SzConfigError::reason_code),
            Some("NOT_FOUND")
        );
        let internal = ApiError::Internal("boom".into());
        assert_eq!(internal.reason_code(), "INTERNAL");
        assert!(internal.config_error().is_none());
        assert_eq!(internal.to_string(), "internal error: boom");
    }

    #[test]
    fn test_validation_details_json_shape() {
        use sz_configtool_lib::error::ValidationReason;
        let failures = [
            ValidationFailure::new(
                "behavior",
                ValidationReason::OutOfDomain,
                Some("XX".to_string()),
            ),
            ValidationFailure::new("sendToRedo", ValidationReason::OutOfDomain, None),
        ];
        let text = validation_details_json(&failures);
        let first_reason = ValidationReason::OutOfDomain.as_str();
        assert_eq!(
            text,
            format!(
                "{{\"schema\":\"sz-configtool.validation-errors/v1\",\"failures\":[\
                 {{\"field\":\"behavior\",\"reasonCode\":\"{first_reason}\",\"offendingValue\":\"XX\"}},\
                 {{\"field\":\"sendToRedo\",\"reasonCode\":\"{first_reason}\",\"offendingValue\":null}}]}}"
            )
        );
        assert_eq!(
            validation_details_json(&[]),
            r#"{"schema":"sz-configtool.validation-errors/v1","failures":[]}"#
        );
        let err = ApiError::Config(SzConfigError::ValidationErrors(failures.to_vec()));
        assert_eq!(err.details_json(), Some(text));
        assert_eq!(
            ApiError::Config(SzConfigError::NotFound("x".into())).details_json(),
            None
        );
        assert_eq!(ApiError::Internal("x".into()).details_json(), None);
    }

    #[test]
    fn test_versions_are_the_single_source() {
        // The workspace version (every crate inherits [workspace.package]).
        let cargo_toml = include_str!("../../Cargo.toml");
        let workspace_version = cargo_toml
            .lines()
            .skip_while(|l| l.trim() != "[workspace.package]")
            .find_map(|l| l.strip_prefix("version = "))
            .map(|v| v.trim_matches('"'))
            .expect("[workspace.package] version");
        assert_eq!(LIBRARY_VERSION, workspace_version);
        assert_eq!(LIBRARY_VERSION_NUL, format!("{workspace_version}\0"));
        assert_eq!(ABI_VERSION, 2);
    }

    #[test]
    fn test_guarded_converts_a_panic_to_internal() {
        let err = guarded("boom_fn", || panic!("kaboom")).unwrap_err();
        assert_eq!(err.reason_code(), "INTERNAL");
        assert_eq!(err.to_string(), "internal error: panic in boom_fn: kaboom");
        assert!(guarded("ok_fn", || invoke("list_data_sources", CFG, "{}")).is_ok());
    }

    #[test]
    fn test_panic_text_variants() {
        assert_eq!(panic_text(&"a"), "a");
        assert_eq!(panic_text(&"b".to_string()), "b");
        assert_eq!(panic_text(&5_i32), "non-string panic payload");
    }

    #[test]
    fn test_reason_codes_cover_every_library_kind() {
        let samples = [
            SzConfigError::JsonParse(String::new()),
            SzConfigError::NotFound(String::new()),
            SzConfigError::NotOnCall(String::new()),
            SzConfigError::NotInFeature(String::new()),
            SzConfigError::AlreadyExists(String::new()),
            SzConfigError::AlreadyPresent(String::new()),
            SzConfigError::InvalidInput(String::new()),
            SzConfigError::MissingSection(String::new()),
            SzConfigError::InvalidStructure(String::new()),
            SzConfigError::MissingField(String::new()),
            SzConfigError::InvalidConfig(String::new()),
            SzConfigError::NotImplemented(String::new()),
            SzConfigError::ValidationErrors(Vec::new()),
        ];
        let lib: Vec<&str> = samples.iter().map(SzConfigError::reason_code).collect();
        assert_eq!(&REASON_CODES[..13], lib.as_slice());
        assert_eq!(REASON_CODES[13], "INTERNAL");
    }
}
