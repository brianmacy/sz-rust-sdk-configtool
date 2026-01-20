//! Behavior override operations (CFG_FBOVR)
//!
//! Functions for managing feature behavior overrides based on usage types.
//! Overrides allow different behavior codes for features depending on context
//! (e.g., BUSINESS vs MOBILE usage).

use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{json, Value};

/// Add a behavior override for a feature based on usage type
///
/// # Arguments
/// * `config_json` - Configuration JSON string
/// * `feature_code` - Feature code (e.g., "PLACEKEY", "PHONE")
/// * `usage_type` - Usage type code (e.g., "BUSINESS", "MOBILE", "ORGANIZATION")
/// * `behavior` - Behavior code (e.g., "F1E", "FM", "NAME")
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
/// use sz_configtool_lib::behavior_overrides;
/// let config = r#"{"G2_CONFIG":{"CFG_FTYPE":[...], "CFG_FBOVR":[]}}"#;
/// let updated = behavior_overrides::add_behavior_override(
///     &config,
///     "PLACEKEY",
///     "BUSINESS",
///     "F1E"
/// )?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn add_behavior_override(
    config_json: &str,
    feature_code: &str,
    usage_type: &str,
    behavior: &str,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Lookup FTYPE_ID from feature code
    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;

    // Parse behavior code into frequency, exclusivity, stability
    let (frequency, exclusivity, stability) = parse_behavior_code(behavior)?;

    let utype_upper = usage_type.to_uppercase();

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
            feature_code, utype_upper
        )));
    }

    // Create override record
    let override_record = json!({
        "FTYPE_ID": ftype_id,
        "UTYPE_CODE": utype_upper,
        "FTYPE_FREQ": frequency,
        "FTYPE_EXCL": exclusivity,
        "FTYPE_STAB": stability
    });

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
            "Behavior override not found for feature {} with usage type {}",
            feature_code, utype_upper
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
                "Behavior override not found for feature {} with usage type {}",
                feature_code, utype_upper
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

/// Parse a behavior code string into (frequency, exclusivity, stability)
///
/// Valid frequency codes: A1, F1, FF, FM, FVM, NONE, NAME
/// E suffix means EXCLUSIVITY = "Yes"
/// S suffix means STABILITY = "Yes"
///
/// # Arguments
/// * `behavior` - Behavior code (e.g., "FM", "F1E", "F1ES", "NAME")
///
/// # Returns
/// Tuple of (frequency, exclusivity, stability)
///
/// # Errors
/// - `InvalidInput` if behavior code is invalid
fn parse_behavior_code(behavior: &str) -> Result<(&'static str, &'static str, &'static str)> {
    let mut code = behavior.to_uppercase();
    let mut exclusivity = "No";
    let mut stability = "No";

    // Special cases that don't get E/S parsing
    if code != "NAME" && code != "NONE" {
        if code.contains('E') {
            exclusivity = "Yes";
            code = code.replace('E', "");
        }
        if code.contains('S') {
            stability = "Yes";
            code = code.replace('S', "");
        }
    }

    // Validate frequency code
    let frequency: &'static str = match code.as_str() {
        "A1" => "A1",
        "F1" => "F1",
        "FF" => "FF",
        "FM" => "FM",
        "FVM" => "FVM",
        "NONE" => "NONE",
        "NAME" => "NAME",
        _ => {
            return Err(SzConfigError::InvalidInput(format!(
                "Invalid behavior code '{}'. Valid codes: A1, F1, FF, FM, FVM, NONE, NAME (with optional E/S suffixes)",
                behavior
            )));
        }
    };

    Ok((frequency, exclusivity, stability))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_add_behavior_override() {
        let config = add_behavior_override(TEST_CONFIG, "TEST_FEATURE", "BUSINESS", "F1E")
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
        let config = add_behavior_override(TEST_CONFIG, "TEST_FEATURE", "BUSINESS", "F1E")
            .expect("Failed to add");

        let config = delete_behavior_override(&config, "TEST_FEATURE", "BUSINESS")
            .expect("Failed to delete");

        let config_val: Value = serde_json::from_str(&config).unwrap();
        let overrides = &config_val["G2_CONFIG"]["CFG_FBOVR"];
        assert_eq!(overrides.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_list_behavior_overrides() {
        let config = add_behavior_override(TEST_CONFIG, "TEST_FEATURE", "BUSINESS", "F1E")
            .expect("Failed to add first");
        let config = add_behavior_override(&config, "TEST_FEATURE", "MOBILE", "FM")
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
    fn test_behavior_override_duplicate() {
        let config = add_behavior_override(TEST_CONFIG, "TEST_FEATURE", "BUSINESS", "F1E")
            .expect("Failed to add first");

        let result = add_behavior_override(&config, "TEST_FEATURE", "BUSINESS", "FM");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("already exists"));
    }
}
