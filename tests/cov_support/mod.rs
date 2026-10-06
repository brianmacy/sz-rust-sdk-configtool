//! Shared helpers for the `coverage_*` integration tests.
//!
//! The real Senzing template fixture is located through the binding manifest's
//! `paths.fixture` key (`api/manifest/project.yaml`) rather than a hardcoded
//! file name, so every test starts from the same configuration the conformance
//! runner uses.

#![allow(dead_code)] // each test binary uses a different subset

use serde_json::Value;
use sz_configtool_lib::SzConfigError;
use sz_configtool_lib::error::SzErrorKind;

/// Workspace-relative path of the project manifest that names the fixture.
const PROJECT_MANIFEST: &str = "api/manifest/project.yaml";

/// Resolve the fixture path from the manifest's `fixture:` key.
fn fixture_path() -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    let manifest = std::fs::read_to_string(format!("{root}/{PROJECT_MANIFEST}"))
        .expect("project manifest must be readable");
    let rel = manifest
        .lines()
        .find_map(|line| line.trim().strip_prefix("fixture:"))
        .map(str::trim)
        .expect("project manifest must declare paths.fixture");
    format!("{root}/{rel}")
}

/// The real Senzing g2config template as a JSON string.
pub fn template() -> String {
    std::fs::read_to_string(fixture_path()).expect("fixture must be readable")
}

/// Parse a config JSON string.
pub fn parse(config: &str) -> Value {
    serde_json::from_str(config).expect("config must be valid JSON")
}

/// Rows of one `G2_CONFIG` section.
pub fn rows(config: &str, section: &str) -> Vec<Value> {
    parse(config)["G2_CONFIG"][section]
        .as_array()
        .cloned()
        .unwrap_or_default()
}

/// Assert `result` failed with `kind` and exactly `message` (the bare payload).
#[track_caller]
pub fn assert_err<T: std::fmt::Debug>(
    result: Result<T, SzConfigError>,
    kind: SzErrorKind,
    message: &str,
) {
    let err = result.expect_err("expected an error");
    assert_eq!(err.kind(), kind, "unexpected kind for {err:?}");
    assert_eq!(err.message(), message, "unexpected message for {err:?}");
}

/// Assert `result` failed with `kind` (used where the message comes from
/// serde_json and is not part of the stable contract).
#[track_caller]
pub fn assert_kind<T: std::fmt::Debug>(result: Result<T, SzConfigError>, kind: SzErrorKind) {
    let err = result.expect_err("expected an error");
    assert_eq!(err.kind(), kind, "unexpected kind for {err:?}");
}

/// A string that is not valid JSON.
pub const BAD_JSON: &str = "{not json";
