//! CFG_SPROFILE (search profile) operations.
//!
//! A search profile ties a generic plan (`CFG_GPLAN`) and a set of per-feature
//! candidate overrides to a named profile code. The engine reads these when
//! deciding how a search request generates candidates.
//!
//! This module owns **all** reads and writes of the `CFG_SPROFILE` section: the
//! CLI passes human-readable codes (`SPROFILE_CODE`, `GPLAN_CODE`, feature
//! `FTYPE_CODE`s) and this module resolves them to the numeric ids stored on
//! disk and builds/parses the `FTYPE_OVERRIDES` mini-format. Callers never
//! construct ids or the mini-format themselves.
//!
//! # Section shape
//!
//! Each `CFG_SPROFILE` row carries:
//! - `SPROFILE_ID` — allocated integer id.
//! - `SPROFILE_CODE` — required, stored uppercased; the profile is addressed by
//!   this code.
//! - `SPROFILE_DESC` — optional description (default `""`).
//! - `GPLAN_ID` — foreign key into `CFG_GPLAN`.
//! - `DEFAULT_USED_FOR_CAND` — `"Normal"` or `"Off"` (canonical casing).
//! - `FTYPE_OVERRIDES` — a mini-format string `"[]"` or
//!   `"[{<ftypeId>,<Y|N>},...]"`: no spaces, comma-separated, no trailing comma,
//!   ascending by ftypeId, each feature at most once. `Y` forces a feature on
//!   for candidate generation, `N` forces it off.
//!
//! `CFG_SPROFILE` is treated as an **optional** section: [`add_search_profile`]
//! creates it on demand, and the read paths tolerate its absence rather than
//! erroring.

use crate::error::{Result, SzConfigError, ValidationFailure, ValidationReason};
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashSet;

/// Reserved (shipped) search profiles that must never be deleted.
///
/// Mirrors the `deleteFeature` `LOCKED_FEATURES` precedent (`features.rs`): the
/// protected set lives SDK-side so every consumer refuses consistently. Checked
/// case-insensitively against the resolved `SPROFILE_CODE`.
const RESERVED_PROFILES: &[&str] = &["INGEST", "SEARCH"];

// ============================================================================
// Row Struct
// ============================================================================

/// Complete `CFG_SPROFILE` row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted. The Senzing engine's config loader requires every key to be present,
/// so partial rows must never be written (same contract as `DsrcRow`).
#[derive(Debug, Clone, Serialize)]
struct SprofileRow {
    #[serde(rename = "SPROFILE_ID")]
    sprofile_id: i64,
    #[serde(rename = "SPROFILE_CODE")]
    sprofile_code: String,
    #[serde(rename = "SPROFILE_DESC")]
    sprofile_desc: String,
    #[serde(rename = "GPLAN_ID")]
    gplan_id: i64,
    #[serde(rename = "DEFAULT_USED_FOR_CAND")]
    default_used_for_cand: String,
    #[serde(rename = "FTYPE_OVERRIDES")]
    ftype_overrides: String,
}

// ============================================================================
// Parameter Struct
// ============================================================================

/// Parameters for [`add_search_profile`].
///
/// All fields are **codes/strings**, never ids: the profile code, the generic
/// plan code, the candidate-generation default, an optional description, and a
/// list of per-feature overrides as `(feature_code, "Yes"|"No")` pairs. Id
/// resolution and mini-format construction happen inside [`add_search_profile`].
#[derive(Debug, Clone, Default)]
pub struct AddSearchProfileParams<'a> {
    /// `SPROFILE_CODE` (required, uppercased internally).
    pub code: &'a str,
    /// `GPLAN_CODE` of the generic plan this profile uses (required; resolved to
    /// `GPLAN_ID`, `NotFound` if it does not exist).
    pub generic_plan: &'a str,
    /// `DEFAULT_USED_FOR_CAND` — `"Normal"` or `"Off"` (case-insensitive).
    /// `None` defaults to `"Normal"`.
    pub candidates: Option<&'a str>,
    /// `SPROFILE_DESC`. `None` defaults to `""`.
    pub description: Option<&'a str>,
    /// Per-feature overrides as `(feature_code, flag)` where `flag` is
    /// `"Yes"`/`"Y"` (force on) or `"No"`/`"N"` (force off), case-insensitive. A
    /// feature may appear at most once.
    pub elements: Vec<(&'a str, &'a str)>,
}

impl<'a> AddSearchProfileParams<'a> {
    /// Start building params with the required profile and generic-plan codes.
    pub fn new(code: &'a str, generic_plan: &'a str) -> Self {
        Self {
            code,
            generic_plan,
            ..Default::default()
        }
    }

    /// Set the candidate-generation default (`"Normal"` or `"Off"`).
    pub fn with_candidates(mut self, candidates: &'a str) -> Self {
        self.candidates = Some(candidates);
        self
    }

    /// Set the profile description.
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Append a single feature override.
    pub fn with_element(mut self, feature_code: &'a str, flag: &'a str) -> Self {
        self.elements.push((feature_code, flag));
        self
    }

    /// Replace the feature-override list.
    pub fn with_elements(mut self, elements: Vec<(&'a str, &'a str)>) -> Self {
        self.elements = elements;
        self
    }
}

// Note: no `TryFrom<&Value>` marshaller is provided. Every current consumer
// (the CLI) builds `AddSearchProfileParams` via the builder methods above.
// A JSON marshaller is intentionally deferred until a real FFI wrapper needs
// one, so its key vocabulary can be aligned to the confirmed command schema and
// tested against that consumer rather than guessed at here.

// ============================================================================
// Public API
// ============================================================================

/// Add a new search profile to the configuration.
///
/// Resolves the generic-plan code and every feature code to ids, builds the
/// `FTYPE_OVERRIDES` mini-format, allocates `SPROFILE_ID`, and appends a
/// complete `CFG_SPROFILE` row. The `CFG_SPROFILE` section is created if it does
/// not yet exist.
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `params` - Search profile parameters (see [`AddSearchProfileParams`])
///
/// # Returns
/// * `Ok(String)` - Modified configuration JSON on success
///
/// # Errors
/// - `AlreadyExists` if the profile code already exists
/// - `NotFound` if the generic plan or a referenced feature does not exist
/// - `ValidationErrors` if `candidates` is not `Normal`/`Off`, a flag is not
///   `Yes`/`No`, or a feature is listed more than once
/// - `InvalidInput` if the profile code is empty
/// - `JsonParse` if `config_json` is invalid
///
/// # Example
/// ```
/// use sz_configtool_lib::search_profiles::{add_search_profile, AddSearchProfileParams};
///
/// let config = r#"{"G2_CONFIG":{"CFG_GPLAN":[{"GPLAN_ID":2,"GPLAN_CODE":"SEARCH"}],
///     "CFG_FTYPE":[{"FTYPE_ID":1,"FTYPE_CODE":"NAME"}],"CFG_SPROFILE":[]}}"#;
/// let params = AddSearchProfileParams::new("EMBEDDED_SEARCH", "SEARCH")
///     .with_element("NAME", "Yes");
/// let modified = add_search_profile(config, params).unwrap();
/// assert!(modified.contains("EMBEDDED_SEARCH"));
/// ```
pub fn add_search_profile(config_json: &str, params: AddSearchProfileParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // --- Resolution / validation phase (immutable borrow of config) ---
    let code_upper = params.code.trim().to_uppercase();
    if code_upper.is_empty() {
        return Err(SzConfigError::InvalidInput(
            "search profile code is required".to_string(),
        ));
    }

    let candidates = canonical_candidates(params.candidates)?;
    let gplan_id = resolve_gplan_id(&config, params.generic_plan)?;
    let ftype_overrides = build_ftype_overrides(&config, &params.elements)?;
    let description = params.description.unwrap_or("").to_string();

    // Duplicate SPROFILE_CODE (case-insensitive) against any existing section.
    if let Some(arr) = config
        .pointer("/G2_CONFIG/CFG_SPROFILE")
        .and_then(|v| v.as_array())
        && arr.iter().any(|r| {
            r.get("SPROFILE_CODE")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s.eq_ignore_ascii_case(&code_upper))
        })
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Search profile already exists: {code_upper}"
        )));
    }

    // --- Mutation phase: create the section if absent, then push the row. ---
    let g2 = config
        .get_mut("G2_CONFIG")
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| SzConfigError::MissingSection("G2_CONFIG".to_string()))?;
    let sprofiles = g2
        .entry("CFG_SPROFILE")
        .or_insert_with(|| Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| {
            SzConfigError::InvalidStructure("CFG_SPROFILE is not an array".to_string())
        })?;

    // Shipped profiles use low ids (INGEST=1, SEARCH=2); user profiles continue
    // the sequence (seed 1 -> next is max+1), so the first added profile on the
    // stock template gets id 3.
    let next_id = crate::helpers::get_desired_or_next_id(sprofiles, "SPROFILE_ID", None, 1)?;

    let row = SprofileRow {
        sprofile_id: next_id,
        sprofile_code: code_upper,
        sprofile_desc: description,
        gplan_id,
        default_used_for_cand: candidates,
        ftype_overrides,
    };
    sprofiles.push(serde_json::to_value(&row)?);

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single search profile by code (case-insensitive).
///
/// Returns the raw `CFG_SPROFILE` row exactly as stored.
///
/// # Errors
/// - `NotFound` if no profile with that code exists (including when the section
///   is absent)
/// - `JsonParse` if `config_json` is invalid
pub fn get_search_profile(config_json: &str, code: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = code.to_uppercase();
    config
        .pointer("/G2_CONFIG/CFG_SPROFILE")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| {
                    r.get("SPROFILE_CODE")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| s.eq_ignore_ascii_case(&code_upper))
                })
                .cloned()
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Search profile not found: {code_upper}")))
}

/// List search profiles as a display-oriented projection.
///
/// Each entry resolves ids to codes for readability and includes both a
/// structured `overrides` list and the raw `overridesRaw` mini-format string:
///
/// ```json
/// {
///   "id": 2,
///   "profile": "SEARCH",
///   "description": "Standard Search",
///   "genericPlan": "SEARCH",
///   "candidates": "Normal",
///   "overrides": [{"feature": "NAME", "flag": "Yes"}],
///   "overridesRaw": "[{1,Y}]"
/// }
/// ```
///
/// Results are sorted by `id`. An optional `filter` keeps only rows whose raw
/// JSON contains the (case-insensitive) substring, matching
/// [`list_generic_plans`](crate::generic_plans::list_generic_plans). A missing
/// `CFG_SPROFILE` section yields an empty list rather than an error.
///
/// # Errors
/// - `JsonParse` if `config_json` is invalid
pub fn list_search_profiles(config_json: &str, filter: Option<&str>) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let rows: Vec<Value> = config
        .pointer("/G2_CONFIG/CFG_SPROFILE")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut result: Vec<Value> = rows
        .into_iter()
        .filter(|row| match filter {
            Some(f) => row.to_string().to_lowercase().contains(&f.to_lowercase()),
            None => true,
        })
        .map(|row| project_profile(&config, &row))
        .collect();

    result.sort_by_key(|item| item.get("id").and_then(|v| v.as_i64()).unwrap_or(0));
    Ok(result)
}

/// Delete a search profile by code or id.
///
/// `search_value` is resolved against `SPROFILE_CODE` (case-insensitive) first,
/// then against `SPROFILE_ID`, so both `"EMBEDDED_SEARCH"` and `"3"` address the
/// same row (mirroring `delete_feature`'s code-then-id resolution).
///
/// The shipped profiles in `RESERVED_PROFILES` (`INGEST`, `SEARCH`) are
/// protected and cannot be deleted, matching the `deleteFeature`
/// `LOCKED_FEATURES` precedent. As in `delete_feature`, existence is resolved
/// **before** the protected check, so a truly-absent value reports `NotFound`
/// (not "protected").
///
/// # Errors
/// - `NotFound` if no profile matches the code or id
/// - `InvalidInput` if the resolved profile is reserved (`INGEST`/`SEARCH`)
/// - `JsonParse` if `config_json` is invalid
///
/// # Example
/// ```
/// use sz_configtool_lib::search_profiles::{add_search_profile, delete_search_profile,
///     AddSearchProfileParams};
///
/// let config = r#"{"G2_CONFIG":{"CFG_GPLAN":[{"GPLAN_ID":2,"GPLAN_CODE":"SEARCH"}],
///     "CFG_FTYPE":[],"CFG_SPROFILE":[]}}"#;
/// let config = add_search_profile(config, AddSearchProfileParams::new("EMBEDDED", "SEARCH")).unwrap();
/// let config = delete_search_profile(&config, "EMBEDDED").unwrap();
/// assert!(!config.contains("EMBEDDED"));
/// ```
pub fn delete_search_profile(config_json: &str, search_value: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Resolve the target row: SPROFILE_CODE (case-insensitive), then SPROFILE_ID.
    let as_id = search_value.trim().parse::<i64>().ok();
    let matched: Option<(i64, String)> = config
        .pointer("/G2_CONFIG/CFG_SPROFILE")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| {
                    let code_match = r
                        .get("SPROFILE_CODE")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| s.eq_ignore_ascii_case(search_value));
                    let id_match =
                        as_id.is_some() && r.get("SPROFILE_ID").and_then(|v| v.as_i64()) == as_id;
                    code_match || id_match
                })
                .map(|r| {
                    let id = r.get("SPROFILE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
                    let code = r
                        .get("SPROFILE_CODE")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    (id, code)
                })
        });

    // Existence first (matches delete_feature): absent -> NotFound, even for a
    // reserved code.
    let (id, code) = matched.ok_or_else(|| {
        SzConfigError::NotFound(format!("Search profile not found: {search_value}"))
    })?;

    // Protected guard, on the resolved canonical code.
    if RESERVED_PROFILES
        .iter()
        .any(|&reserved| reserved.eq_ignore_ascii_case(&code))
    {
        return Err(SzConfigError::InvalidInput(format!(
            "The search profile {code} cannot be deleted (it is a protected system profile)"
        )));
    }

    // Remove the row by its (unique) id.
    if let Some(arr) = config
        .pointer_mut("/G2_CONFIG/CFG_SPROFILE")
        .and_then(|v| v.as_array_mut())
    {
        arr.retain(|r| r.get("SPROFILE_ID").and_then(|v| v.as_i64()) != Some(id));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Canonicalise `DEFAULT_USED_FOR_CAND` to `"Normal"`/`"Off"`, defaulting to
/// `"Normal"` when absent/blank. Any other value is an out-of-domain failure.
fn canonical_candidates(value: Option<&str>) -> Result<String> {
    match value.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok("Normal".to_string()),
        Some(s) => match s.to_uppercase().as_str() {
            "NORMAL" => Ok("Normal".to_string()),
            "OFF" => Ok("Off".to_string()),
            _ => Err(SzConfigError::ValidationErrors(vec![
                ValidationFailure::new(
                    "candidates",
                    ValidationReason::OutOfDomain,
                    Some(s.to_string()),
                ),
            ])),
        },
    }
}

/// Canonicalise an override flag to `'Y'`/`'N'`. Out-of-domain otherwise.
fn canonical_flag(flag: &str) -> Result<char> {
    match flag.trim().to_uppercase().as_str() {
        "YES" | "Y" => Ok('Y'),
        "NO" | "N" => Ok('N'),
        other => Err(SzConfigError::ValidationErrors(vec![
            ValidationFailure::new(
                "overrides",
                ValidationReason::OutOfDomain,
                Some(other.to_string()),
            ),
        ])),
    }
}

/// Build the `FTYPE_OVERRIDES` mini-format from `(feature_code, flag)` pairs.
///
/// Resolves each feature to its `FTYPE_ID`, rejects a feature listed twice, and
/// emits `"[{id,YN},...]"` sorted ascending by id (or `"[]"` when empty).
fn build_ftype_overrides(config: &Value, elements: &[(&str, &str)]) -> Result<String> {
    if elements.is_empty() {
        return Ok("[]".to_string());
    }

    let mut seen: HashSet<i64> = HashSet::with_capacity(elements.len());
    let mut pairs: Vec<(i64, char)> = Vec::with_capacity(elements.len());
    for (feature, flag) in elements {
        let ftype_id = resolve_ftype_id(config, feature)?;
        if !seen.insert(ftype_id) {
            return Err(SzConfigError::ValidationErrors(vec![
                ValidationFailure::new(
                    "overrides",
                    ValidationReason::Duplicate,
                    Some((*feature).to_string()),
                ),
            ]));
        }
        pairs.push((ftype_id, canonical_flag(flag)?));
    }

    pairs.sort_by_key(|(id, _)| *id);
    let body = pairs
        .iter()
        .map(|(id, yn)| format!("{{{id},{yn}}}"))
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!("[{body}]"))
}

/// Resolve a `GPLAN_CODE` to its `GPLAN_ID` against an already-parsed config.
fn resolve_gplan_id(config: &Value, code: &str) -> Result<i64> {
    config
        .pointer("/G2_CONFIG/CFG_GPLAN")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| {
                    r.get("GPLAN_CODE")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| s.eq_ignore_ascii_case(code))
                })
                .and_then(|r| r.get("GPLAN_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan '{code}' not found")))
}

/// Resolve an `FTYPE_CODE` to its `FTYPE_ID` against an already-parsed config.
fn resolve_ftype_id(config: &Value, code: &str) -> Result<i64> {
    config
        .pointer("/G2_CONFIG/CFG_FTYPE")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| {
                    r.get("FTYPE_CODE")
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| s.eq_ignore_ascii_case(code))
                })
                .and_then(|r| r.get("FTYPE_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature '{code}' not found")))
}

/// Reverse of [`resolve_gplan_id`]: `GPLAN_ID` -> `GPLAN_CODE` for display.
fn resolve_gplan_code(config: &Value, id: i64) -> Option<String> {
    config
        .pointer("/G2_CONFIG/CFG_GPLAN")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| r.get("GPLAN_ID").and_then(|v| v.as_i64()) == Some(id))
                .and_then(|r| r.get("GPLAN_CODE"))
                .and_then(|v| v.as_str())
                .map(String::from)
        })
}

/// Reverse of [`resolve_ftype_id`]: `FTYPE_ID` -> `FTYPE_CODE` for display.
fn resolve_ftype_code(config: &Value, id: i64) -> Option<String> {
    config
        .pointer("/G2_CONFIG/CFG_FTYPE")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|r| r.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(id))
                .and_then(|r| r.get("FTYPE_CODE"))
                .and_then(|v| v.as_str())
                .map(String::from)
        })
}

/// Parse the `FTYPE_OVERRIDES` mini-format into `(ftypeId, flag)` pairs.
///
/// Best-effort and display-oriented: this reads a string this module itself
/// wrote, so a malformed value (e.g. hand-edited config) yields the pairs it can
/// recover rather than an error — the raw string is always available separately
/// via `overridesRaw`.
fn parse_overrides(raw: &str) -> Vec<(i64, char)> {
    let trimmed = raw.trim();
    let inner = match trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        Some(i) => i.trim(),
        None => return Vec::new(),
    };
    if inner.is_empty() {
        return Vec::new();
    }

    // Flatten on the structural characters, leaving a [id, flag, id, flag, ...]
    // token stream, then pair up.
    let tokens: Vec<&str> = inner
        .split(['{', '}', ','])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    // Pair up [id, flag, ...]. `chunks` (not `chunks_exact`) keeps this on the
    // MSRV without the `as_chunks` clippy lint; a short trailing chunk from a
    // malformed value is dropped by the length-guarded `get`s.
    tokens
        .chunks(2)
        .filter_map(|pair| {
            let id = pair.first()?.parse::<i64>().ok()?;
            let yn = pair.get(1)?.chars().next()?.to_ascii_uppercase();
            Some((id, yn))
        })
        .collect()
}

/// Project a raw `CFG_SPROFILE` row to the display shape returned by
/// [`list_search_profiles`].
fn project_profile(config: &Value, row: &Value) -> Value {
    let gplan_id = row.get("GPLAN_ID").and_then(|v| v.as_i64());
    let generic_plan = gplan_id
        .and_then(|id| resolve_gplan_code(config, id))
        .or_else(|| gplan_id.map(|id| id.to_string()))
        .unwrap_or_default();

    let raw = row
        .get("FTYPE_OVERRIDES")
        .and_then(|v| v.as_str())
        .unwrap_or("[]");

    let overrides: Vec<Value> = parse_overrides(raw)
        .into_iter()
        .map(|(id, yn)| {
            let feature = resolve_ftype_code(config, id).unwrap_or_else(|| id.to_string());
            let flag = match yn {
                'Y' => "Yes",
                'N' => "No",
                _ => "?",
            };
            json!({ "feature": feature, "flag": flag })
        })
        .collect();

    json!({
        "id": row.get("SPROFILE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
        "profile": row.get("SPROFILE_CODE").and_then(|v| v.as_str()).unwrap_or(""),
        "description": row.get("SPROFILE_DESC").and_then(|v| v.as_str()).unwrap_or(""),
        "genericPlan": generic_plan,
        "candidates": row.get("DEFAULT_USED_FOR_CAND").and_then(|v| v.as_str()).unwrap_or("Normal"),
        "overrides": overrides,
        "overridesRaw": raw,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SzErrorKind;

    /// Minimal synthetic config with the sections the module touches.
    fn base_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_GPLAN": [
                    {"GPLAN_ID": 1, "GPLAN_CODE": "INGEST", "GPLAN_DESC": "Ingest"},
                    {"GPLAN_ID": 2, "GPLAN_CODE": "SEARCH", "GPLAN_DESC": "Search"}
                ],
                "CFG_FTYPE": [
                    {"FTYPE_ID": 1, "FTYPE_CODE": "NAME"},
                    {"FTYPE_ID": 99, "FTYPE_CODE": "ADDRESS"},
                    {"FTYPE_ID": 102, "FTYPE_CODE": "PHONE"}
                ],
                "CFG_SPROFILE": [
                    {"SPROFILE_ID": 2, "SPROFILE_CODE": "SEARCH", "SPROFILE_DESC": "Standard Search",
                     "GPLAN_ID": 2, "DEFAULT_USED_FOR_CAND": "Normal", "FTYPE_OVERRIDES": "[]"}
                ]
            }
        })
        .to_string()
    }

    fn sprofiles(config_json: &str) -> Vec<Value> {
        let v: Value = serde_json::from_str(config_json).unwrap();
        v["G2_CONFIG"]["CFG_SPROFILE"].as_array().unwrap().clone()
    }

    #[test]
    fn add_basic_profile_allocates_next_id_and_uppercases() {
        let params = AddSearchProfileParams::new("embedded_search", "SEARCH")
            .with_description("Embedded")
            .with_candidates("off");
        let out = add_search_profile(&base_config(), params).unwrap();
        let rows = sprofiles(&out);
        let added = rows.iter().find(|r| r["SPROFILE_ID"] == 3).unwrap();
        assert_eq!(added["SPROFILE_CODE"], "EMBEDDED_SEARCH");
        assert_eq!(added["SPROFILE_DESC"], "Embedded");
        assert_eq!(added["GPLAN_ID"], 2);
        assert_eq!(added["DEFAULT_USED_FOR_CAND"], "Off");
        assert_eq!(added["FTYPE_OVERRIDES"], "[]");
    }

    #[test]
    fn add_defaults_candidates_normal_and_desc_empty() {
        let out =
            add_search_profile(&base_config(), AddSearchProfileParams::new("P", "SEARCH")).unwrap();
        let rows = sprofiles(&out);
        let added = rows.iter().find(|r| r["SPROFILE_CODE"] == "P").unwrap();
        assert_eq!(added["DEFAULT_USED_FOR_CAND"], "Normal");
        assert_eq!(added["SPROFILE_DESC"], "");
    }

    #[test]
    fn overrides_built_sorted_ascending_with_codes_resolved() {
        // Deliberately out of order; expect ascending by ftype_id.
        let params = AddSearchProfileParams::new("P", "SEARCH")
            .with_element("PHONE", "No") // 102
            .with_element("NAME", "yes") // 1
            .with_element("ADDRESS", "Y"); // 99
        let out = add_search_profile(&base_config(), params).unwrap();
        let rows = sprofiles(&out);
        let added = rows.iter().find(|r| r["SPROFILE_CODE"] == "P").unwrap();
        assert_eq!(added["FTYPE_OVERRIDES"], "[{1,Y},{99,Y},{102,N}]");
    }

    #[test]
    fn add_creates_section_when_missing() {
        let no_section = json!({
            "G2_CONFIG": {
                "CFG_GPLAN": [{"GPLAN_ID": 2, "GPLAN_CODE": "SEARCH"}],
                "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}]
            }
        })
        .to_string();
        let out = add_search_profile(&no_section, AddSearchProfileParams::new("FIRST", "SEARCH"))
            .unwrap();
        let rows = sprofiles(&out);
        assert_eq!(rows.len(), 1);
        // Empty section -> first id is the seed (1).
        assert_eq!(rows[0]["SPROFILE_ID"], 1);
        assert_eq!(rows[0]["SPROFILE_CODE"], "FIRST");
    }

    #[test]
    fn duplicate_code_rejected_case_insensitively() {
        let err = add_search_profile(
            &base_config(),
            AddSearchProfileParams::new("search", "SEARCH"),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
    }

    #[test]
    fn unknown_generic_plan_is_not_found() {
        let err = add_search_profile(&base_config(), AddSearchProfileParams::new("P", "NOPE"))
            .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
    }

    #[test]
    fn unknown_feature_is_not_found() {
        let params = AddSearchProfileParams::new("P", "SEARCH").with_element("BOGUS", "Yes");
        let err = add_search_profile(&base_config(), params).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
    }

    #[test]
    fn bad_candidates_is_out_of_domain_validation_error() {
        let params = AddSearchProfileParams::new("P", "SEARCH").with_candidates("Maybe");
        let err = add_search_profile(&base_config(), params).unwrap_err();
        let failures = err.validation_failures().expect("ValidationErrors");
        assert_eq!(failures[0].field, "candidates");
        assert_eq!(failures[0].reason_code, ValidationReason::OutOfDomain);
        assert_eq!(failures[0].offending_value.as_deref(), Some("Maybe"));
    }

    #[test]
    fn bad_flag_is_out_of_domain_validation_error() {
        let params = AddSearchProfileParams::new("P", "SEARCH").with_element("NAME", "perhaps");
        let err = add_search_profile(&base_config(), params).unwrap_err();
        let failures = err.validation_failures().expect("ValidationErrors");
        assert_eq!(failures[0].field, "overrides");
        assert_eq!(failures[0].reason_code, ValidationReason::OutOfDomain);
    }

    #[test]
    fn duplicate_feature_is_duplicate_validation_error() {
        let params = AddSearchProfileParams::new("P", "SEARCH")
            .with_element("NAME", "Yes")
            .with_element("name", "No");
        let err = add_search_profile(&base_config(), params).unwrap_err();
        let failures = err.validation_failures().expect("ValidationErrors");
        assert_eq!(failures[0].field, "overrides");
        assert_eq!(failures[0].reason_code, ValidationReason::Duplicate);
    }

    #[test]
    fn empty_code_is_invalid_input() {
        let err = add_search_profile(&base_config(), AddSearchProfileParams::new("  ", "SEARCH"))
            .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);
    }

    #[test]
    fn get_profile_case_insensitive_and_not_found() {
        let config = base_config();
        let got = get_search_profile(&config, "search").unwrap();
        assert_eq!(got["SPROFILE_ID"], 2);
        assert_eq!(
            get_search_profile(&config, "nope").unwrap_err().kind(),
            SzErrorKind::NotFound
        );
    }

    #[test]
    fn get_profile_not_found_when_section_absent() {
        let no_section = json!({"G2_CONFIG": {}}).to_string();
        assert_eq!(
            get_search_profile(&no_section, "SEARCH")
                .unwrap_err()
                .kind(),
            SzErrorKind::NotFound
        );
    }

    #[test]
    fn list_projects_resolved_codes_and_overrides() {
        let params = AddSearchProfileParams::new("EMB", "SEARCH")
            .with_element("NAME", "Yes")
            .with_element("ADDRESS", "No");
        let out = add_search_profile(&base_config(), params).unwrap();

        let listed = list_search_profiles(&out, None).unwrap();
        assert_eq!(listed.len(), 2);
        // Sorted by id: SEARCH(2) then EMB(3).
        assert_eq!(listed[0]["profile"], "SEARCH");
        let emb = &listed[1];
        assert_eq!(emb["profile"], "EMB");
        assert_eq!(emb["genericPlan"], "SEARCH");
        assert_eq!(emb["overridesRaw"], "[{1,Y},{99,N}]");
        assert_eq!(emb["overrides"][0]["feature"], "NAME");
        assert_eq!(emb["overrides"][0]["flag"], "Yes");
        assert_eq!(emb["overrides"][1]["feature"], "ADDRESS");
        assert_eq!(emb["overrides"][1]["flag"], "No");
    }

    #[test]
    fn list_filter_is_case_insensitive_substring() {
        let config = base_config();
        assert_eq!(
            list_search_profiles(&config, Some("standard"))
                .unwrap()
                .len(),
            1
        );
        assert_eq!(list_search_profiles(&config, Some("zzz")).unwrap().len(), 0);
    }

    #[test]
    fn list_empty_when_section_absent() {
        let no_section = json!({"G2_CONFIG": {}}).to_string();
        assert!(list_search_profiles(&no_section, None).unwrap().is_empty());
    }

    /// base_config plus a reserved INGEST (id 1) and a user profile (id 3), for
    /// exercising the delete paths.
    fn config_for_delete() -> String {
        let out = add_search_profile(
            &base_config(),
            AddSearchProfileParams::new("EMBEDDED_SEARCH", "SEARCH"),
        )
        .unwrap();
        // Inject a reserved INGEST row (the base fixture only ships SEARCH).
        let mut v: Value = serde_json::from_str(&out).unwrap();
        v["G2_CONFIG"]["CFG_SPROFILE"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "SPROFILE_ID": 1, "SPROFILE_CODE": "INGEST", "SPROFILE_DESC": "Ingest",
                "GPLAN_ID": 1, "DEFAULT_USED_FOR_CAND": "Normal", "FTYPE_OVERRIDES": "[]"
            }));
        v.to_string()
    }

    #[test]
    fn delete_user_profile_by_code_succeeds() {
        let out = delete_search_profile(&config_for_delete(), "embedded_search").unwrap();
        let rows = sprofiles(&out);
        assert!(!rows.iter().any(|r| r["SPROFILE_CODE"] == "EMBEDDED_SEARCH"));
        // Reserved rows untouched.
        assert!(rows.iter().any(|r| r["SPROFILE_CODE"] == "SEARCH"));
        assert!(rows.iter().any(|r| r["SPROFILE_CODE"] == "INGEST"));
    }

    #[test]
    fn delete_user_profile_by_id_succeeds() {
        // EMBEDDED_SEARCH is allocated id 3 on the base fixture (max was 2).
        let out = delete_search_profile(&config_for_delete(), "3").unwrap();
        assert!(!sprofiles(&out).iter().any(|r| r["SPROFILE_ID"] == 3));
    }

    #[test]
    fn delete_reserved_search_refused() {
        let err = delete_search_profile(&config_for_delete(), "SEARCH").unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);
        // .message() is the bare sentence the CLI surfaces (no "Invalid input:" prefix).
        assert_eq!(
            err.message(),
            "The search profile SEARCH cannot be deleted (it is a protected system profile)"
        );
    }

    #[test]
    fn delete_reserved_ingest_refused_case_insensitive() {
        let err = delete_search_profile(&config_for_delete(), "ingest").unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);
        // Message uses the canonical stored code, not the input casing.
        assert!(
            err.message()
                .contains("The search profile INGEST cannot be deleted")
        );
    }

    #[test]
    fn delete_unknown_code_or_id_not_found() {
        let config = config_for_delete();
        assert_eq!(
            delete_search_profile(&config, "NOPE").unwrap_err().kind(),
            SzErrorKind::NotFound
        );
        assert_eq!(
            delete_search_profile(&config, "999").unwrap_err().kind(),
            SzErrorKind::NotFound
        );
    }
}
