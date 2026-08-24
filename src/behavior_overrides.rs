//! Behavior override operations (CFG_FBOVR)
//!
//! Functions for managing feature behavior overrides based on usage types.
//! Overrides allow different behavior codes for features depending on context
//! (e.g., BUSINESS vs MOBILE usage).

use crate::behavior_domain::{compute_behavior, parse_behavior_code};
use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_FBOVR row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted. The Senzing engine's config loader requires every key to be
/// present, so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
struct FbovrRow {
    #[serde(rename = "FTYPE_ID")]
    ftype_id: i64,
    #[serde(rename = "UTYPE_CODE")]
    utype_code: String,
    #[serde(rename = "FTYPE_FREQ")]
    ftype_freq: String,
    #[serde(rename = "FTYPE_EXCL")]
    ftype_excl: String,
    #[serde(rename = "FTYPE_STAB")]
    ftype_stab: String,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a behavior override
#[derive(Debug, Clone)]
pub struct AddBehaviorOverrideParams<'a> {
    pub feature_code: &'a str,
    pub usage_type: &'a str,
    pub behavior: &'a str,
}

impl<'a> AddBehaviorOverrideParams<'a> {
    pub fn new(feature_code: &'a str, usage_type: &'a str, behavior: &'a str) -> Self {
        Self {
            feature_code,
            usage_type,
            behavior,
        }
    }
}

/// Add a behavior override for a feature based on usage type
///
/// # Arguments
/// * `config_json` - Configuration JSON string
/// * `params` - Override parameters (feature_code, usage_type, behavior)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if feature doesn't exist
/// - `AlreadyExists` if override already exists for this feature+usage combination
/// - `InvalidInput` if behavior code is invalid
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::behavior_overrides::{add_behavior_override, AddBehaviorOverrideParams};
/// let config = r#"{"G2_CONFIG":{"CFG_FTYPE":[...], "CFG_FBOVR":[]}}"#;
/// let updated = add_behavior_override(
///     &config,
///     AddBehaviorOverrideParams::new("PLACEKEY", "BUSINESS", "F1E")
/// )?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn add_behavior_override(
    config_json: &str,
    params: AddBehaviorOverrideParams,
) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Lookup FTYPE_ID from feature code
    let ftype_id = helpers::lookup_feature_id(config_json, params.feature_code)?;

    // Parse behavior code into frequency, exclusivity, stability
    let (frequency, exclusivity, stability) = parse_behavior_code(params.behavior)?;

    let utype_upper = params.usage_type.to_uppercase();

    // Check for existing override for this feature+usage combination
    let fbovr_array = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FBOVR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOVR".to_string()))?;

    if fbovr_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(ftype_id)
            && item["UTYPE_CODE"].as_str() == Some(&utype_upper)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Behavior override already exists for feature {} with usage type {}",
            params.feature_code, utype_upper
        )));
    }

    // Build a complete row via FbovrRow so every CFG_FBOVR key is present.
    let row = FbovrRow {
        ftype_id,
        utype_code: utype_upper,
        ftype_freq: frequency.to_string(),
        ftype_excl: exclusivity.to_string(),
        ftype_stab: stability.to_string(),
    };
    let override_record = serde_json::to_value(&row)?;

    // Add to CFG_FBOVR
    helpers::add_to_config_array(config_json, "CFG_FBOVR", override_record)
}

/// Delete a behavior override for a feature and usage type
///
/// # Arguments
/// * `config_json` - Configuration JSON string
/// * `feature_code` - Feature code
/// * `usage_type` - Usage type code
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if feature or override doesn't exist
pub fn delete_behavior_override(
    config_json: &str,
    feature_code: &str,
    usage_type: &str,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Lookup FTYPE_ID
    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let utype_upper = usage_type.to_uppercase();

    let fbovr_array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_FBOVR"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOVR".to_string()))?;

    let original_len = fbovr_array.len();
    fbovr_array.retain(|item| {
        !(item["FTYPE_ID"].as_i64() == Some(ftype_id)
            && item["UTYPE_CODE"].as_str() == Some(&utype_upper))
    });

    if fbovr_array.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "Behavior override not found for feature {feature_code} with usage type {utype_upper}"
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a specific behavior override
///
/// # Arguments
/// * `config_json` - Configuration JSON string
/// * `feature_code` - Feature code
/// * `usage_type` - Usage type code
///
/// # Returns
/// JSON Value representing the behavior override
pub fn get_behavior_override(
    config_json: &str,
    feature_code: &str,
    usage_type: &str,
) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let utype_upper = usage_type.to_uppercase();

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FBOVR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOVR".to_string()))?
        .iter()
        .find(|item| {
            item["FTYPE_ID"].as_i64() == Some(ftype_id)
                && item["UTYPE_CODE"].as_str() == Some(&utype_upper)
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Behavior override not found for feature {feature_code} with usage type {utype_upper}"
            ))
        })
}

/// List all behavior overrides
///
/// # Arguments
/// * `config_json` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values representing behavior overrides, sorted by FTYPE_ID
pub fn list_behavior_overrides(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbovr_array = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FBOVR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOVR".to_string()))?;

    let mut result: Vec<Value> = fbovr_array.to_vec();

    // Sort by FTYPE_ID
    result.sort_by_key(|item| item["FTYPE_ID"].as_i64().unwrap_or(0));

    Ok(result)
}

/// Display-shaped, fully resolved behaviour override row.
///
/// Unlike the raw `CFG_FBOVR` rows returned by [`list_behavior_overrides`],
/// this carries the human-readable feature code (resolved from `FTYPE_ID`) and
/// the composed behaviour code (via [`compute_behavior`]) rather than the raw
/// `FTYPE_FREQ`/`FTYPE_EXCL`/`FTYPE_STAB` triple. It serialises to the display
/// JSON shape `{ "feature", "usageType", "behavior" }`.
#[derive(Debug, Clone, Serialize)]
struct BehaviorOverrideDisplay {
    feature: String,
    #[serde(rename = "usageType")]
    usage_type: String,
    behavior: String,
}

/// List all behavior overrides in a display-ready, resolved shape.
///
/// This is the richer counterpart to [`list_behavior_overrides`]. Each returned
/// value has the shape `{ "feature", "usageType", "behavior" }`, where:
///
/// - `feature` is the feature code resolved from the row's `FTYPE_ID`,
/// - `usageType` is the row's `UTYPE_CODE`,
/// - `behavior` is the composed behaviour code (frequency plus `E`/`S`
///   suffixes) produced by [`compute_behavior`].
///
/// Rows are sorted by `(FTYPE_ID, UTYPE_CODE)` — the numeric feature id first,
/// then the usage-type code as a tiebreak — which is the ordering the CLI's
/// `listBehaviorOverrides` display expects and which [`list_behavior_overrides`]
/// (sorted by `FTYPE_ID` only) cannot provide.
///
/// A row whose `FTYPE_ID` has no matching `CFG_FTYPE` entry falls back to the
/// numeric id rendered as a string for its `feature` field, so malformed
/// configs still list rather than error.
///
/// # Arguments
/// * `config_json` - Configuration JSON string
///
/// # Returns
/// Vector of resolved override values, sorted by `(FTYPE_ID, UTYPE_CODE)`.
///
/// # Errors
/// - `JsonParse` if `config_json` is not valid JSON
/// - `MissingSection` if `CFG_FBOVR` is absent
///
/// # Example
/// ```
/// use sz_configtool_lib::behavior_overrides::list_behavior_overrides_resolved;
/// let config = r#"{"G2_CONFIG":{
///     "CFG_FTYPE":[{"FTYPE_ID":3,"FTYPE_CODE":"ADDRESS"}],
///     "CFG_FBOVR":[{"FTYPE_ID":3,"UTYPE_CODE":"HOME",
///                   "FTYPE_FREQ":"FF","FTYPE_EXCL":"Yes","FTYPE_STAB":"No"}]
/// }}"#;
/// let rows = list_behavior_overrides_resolved(config)?;
/// assert_eq!(rows[0]["feature"], "ADDRESS");
/// assert_eq!(rows[0]["usageType"], "HOME");
/// assert_eq!(rows[0]["behavior"], "FFE");
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn list_behavior_overrides_resolved(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let g2 = config
        .get("G2_CONFIG")
        .ok_or_else(|| SzConfigError::MissingSection("G2_CONFIG".to_string()))?;

    // Build an FTYPE_ID -> FTYPE_CODE resolution map from CFG_FTYPE.
    let mut ftype_codes: HashMap<i64, String> = HashMap::new();
    if let Some(ftypes) = g2.get("CFG_FTYPE").and_then(|v| v.as_array()) {
        for ftype in ftypes {
            if let (Some(id), Some(code)) = (
                ftype.get("FTYPE_ID").and_then(|v| v.as_i64()),
                ftype.get("FTYPE_CODE").and_then(|v| v.as_str()),
            ) {
                ftype_codes.insert(id, code.to_string());
            }
        }
    }

    let fbovr_array = g2
        .get("CFG_FBOVR")
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOVR".to_string()))?;

    // Collect (ftype_id, utype_code, display) so we can sort by the raw keys
    // before projecting away FTYPE_ID.
    let mut rows: Vec<(i64, String, BehaviorOverrideDisplay)> =
        Vec::with_capacity(fbovr_array.len());
    for item in fbovr_array {
        let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
        let utype_code = item
            .get("UTYPE_CODE")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let feature = ftype_codes
            .get(&ftype_id)
            .cloned()
            .unwrap_or_else(|| ftype_id.to_string());
        let display = BehaviorOverrideDisplay {
            feature,
            usage_type: utype_code.clone(),
            behavior: compute_behavior(item),
        };
        rows.push((ftype_id, utype_code, display));
    }

    // Sort by (FTYPE_ID, UTYPE_CODE).
    rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    rows.into_iter()
        .map(|(_, _, display)| {
            serde_json::to_value(&display).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const TEST_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [
      {
        "FTYPE_ID": 1,
        "FTYPE_CODE": "TEST_FEATURE",
        "FTYPE_DESC": "Test Feature"
      }
    ],
    "CFG_FBOVR": []
  }
}"#;

    const FBOVR_KEYS: [&str; 5] = [
        "FTYPE_ID",
        "UTYPE_CODE",
        "FTYPE_FREQ",
        "FTYPE_EXCL",
        "FTYPE_STAB",
    ];

    #[test]
    fn test_add_behavior_override_emits_all_keys() {
        let config = add_behavior_override(
            TEST_CONFIG,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "F1E"),
        )
        .expect("Failed to add behavior override");

        let config_val: Value = serde_json::from_str(&config).unwrap();
        let override_rec = &config_val["G2_CONFIG"]["CFG_FBOVR"][0];
        let obj = override_rec.as_object().unwrap();

        for key in FBOVR_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(override_rec["FTYPE_ID"], json!(1));
        assert_eq!(override_rec["UTYPE_CODE"], json!("BUSINESS"));
        assert_eq!(override_rec["FTYPE_FREQ"], json!("F1"));
        assert_eq!(override_rec["FTYPE_EXCL"], json!("Yes"));
        assert_eq!(override_rec["FTYPE_STAB"], json!("No"));
    }

    #[test]
    fn test_add_behavior_override() {
        let config = add_behavior_override(
            TEST_CONFIG,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "F1E"),
        )
        .expect("Failed to add behavior override");

        let config_val: Value = serde_json::from_str(&config).unwrap();
        let overrides = &config_val["G2_CONFIG"]["CFG_FBOVR"];

        assert_eq!(overrides.as_array().unwrap().len(), 1);

        let override_rec = &overrides[0];
        assert_eq!(override_rec["FTYPE_ID"], 1);
        assert_eq!(override_rec["UTYPE_CODE"], "BUSINESS");
        assert_eq!(override_rec["FTYPE_FREQ"], "F1");
        assert_eq!(override_rec["FTYPE_EXCL"], "Yes");
        assert_eq!(override_rec["FTYPE_STAB"], "No");
    }

    #[test]
    fn test_delete_behavior_override() {
        let config = add_behavior_override(
            TEST_CONFIG,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "F1E"),
        )
        .expect("Failed to add");

        let config = delete_behavior_override(&config, "TEST_FEATURE", "BUSINESS")
            .expect("Failed to delete");

        let config_val: Value = serde_json::from_str(&config).unwrap();
        let overrides = &config_val["G2_CONFIG"]["CFG_FBOVR"];
        assert_eq!(overrides.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_list_behavior_overrides() {
        let config = add_behavior_override(
            TEST_CONFIG,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "F1E"),
        )
        .expect("Failed to add first");
        let config = add_behavior_override(
            &config,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "MOBILE", "FM"),
        )
        .expect("Failed to add second");

        let overrides = list_behavior_overrides(&config).expect("Failed to list");
        assert_eq!(overrides.len(), 2);
        assert_eq!(overrides[0]["UTYPE_CODE"], "BUSINESS");
        assert_eq!(overrides[1]["UTYPE_CODE"], "MOBILE");
    }

    #[test]
    fn test_parse_behavior_code_simple() {
        let (freq, excl, stab) = parse_behavior_code("FM").unwrap();
        assert_eq!(freq, "FM");
        assert_eq!(excl, "No");
        assert_eq!(stab, "No");
    }

    #[test]
    fn test_parse_behavior_code_with_modifiers() {
        let (freq, excl, stab) = parse_behavior_code("F1ES").unwrap();
        assert_eq!(freq, "F1");
        assert_eq!(excl, "Yes");
        assert_eq!(stab, "Yes");
    }

    #[test]
    fn test_parse_behavior_code_name() {
        let (freq, excl, stab) = parse_behavior_code("NAME").unwrap();
        assert_eq!(freq, "NAME");
        assert_eq!(excl, "No");
        assert_eq!(stab, "No");
    }

    #[test]
    fn test_list_behavior_overrides_resolved_projection_and_sort() {
        // Fixture deliberately ordered so a FTYPE_ID-only sort differs from the
        // required (FTYPE_ID, UTYPE_CODE) sort: feature 3 has two usage types
        // stored MOBILE-before-BUSINESS, and feature 1 is stored last.
        let config = json!({
            "G2_CONFIG": {
                "CFG_FTYPE": [
                    {"FTYPE_ID": 3, "FTYPE_CODE": "ADDRESS"},
                    {"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}
                ],
                "CFG_FBOVR": [
                    {"FTYPE_ID": 3, "UTYPE_CODE": "MOBILE",
                     "FTYPE_FREQ": "FF", "FTYPE_EXCL": "Yes", "FTYPE_STAB": "No"},
                    {"FTYPE_ID": 3, "UTYPE_CODE": "BUSINESS",
                     "FTYPE_FREQ": "F1", "FTYPE_EXCL": "No", "FTYPE_STAB": "Yes"},
                    {"FTYPE_ID": 1, "UTYPE_CODE": "PRIMARY",
                     "FTYPE_FREQ": "NAME", "FTYPE_EXCL": "No", "FTYPE_STAB": "No"}
                ]
            }
        })
        .to_string();

        let rows = list_behavior_overrides_resolved(&config).expect("resolved list");
        assert_eq!(rows.len(), 3);

        // Expected order: (1,PRIMARY), (3,BUSINESS), (3,MOBILE).
        assert_eq!(rows[0]["feature"], "NAME");
        assert_eq!(rows[0]["usageType"], "PRIMARY");
        assert_eq!(rows[0]["behavior"], "NAME");

        assert_eq!(rows[1]["feature"], "ADDRESS");
        assert_eq!(rows[1]["usageType"], "BUSINESS");
        assert_eq!(rows[1]["behavior"], "F1S");

        assert_eq!(rows[2]["feature"], "ADDRESS");
        assert_eq!(rows[2]["usageType"], "MOBILE");
        assert_eq!(rows[2]["behavior"], "FFE");

        // Projection is exactly the display shape: feature, usageType, behavior.
        for row in &rows {
            let obj = row.as_object().unwrap();
            assert_eq!(obj.len(), 3);
            assert!(obj.contains_key("feature"));
            assert!(obj.contains_key("usageType"));
            assert!(obj.contains_key("behavior"));
        }
    }

    #[test]
    fn test_list_behavior_overrides_resolved_unknown_ftype_falls_back_to_id() {
        let config = json!({
            "G2_CONFIG": {
                "CFG_FTYPE": [],
                "CFG_FBOVR": [
                    {"FTYPE_ID": 99, "UTYPE_CODE": "X",
                     "FTYPE_FREQ": "F1", "FTYPE_EXCL": "No", "FTYPE_STAB": "No"}
                ]
            }
        })
        .to_string();

        let rows = list_behavior_overrides_resolved(&config).expect("resolved list");
        assert_eq!(rows[0]["feature"], "99");
        assert_eq!(rows[0]["behavior"], "F1");
    }

    #[test]
    fn test_behavior_override_duplicate() {
        let config = add_behavior_override(
            TEST_CONFIG,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "F1E"),
        )
        .expect("Failed to add first");

        let result = add_behavior_override(
            &config,
            AddBehaviorOverrideParams::new("TEST_FEATURE", "BUSINESS", "FM"),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
