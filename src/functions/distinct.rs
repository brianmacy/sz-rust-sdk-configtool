//! Distinct function operations for Senzing configuration
//!
//! This module provides functions for managing distinct functions (CFG_DFUNC)
//! in the Senzing configuration JSON.

use crate::error::SzConfigError;
use crate::helpers::{
    FieldUpdate, add_to_config_array, delete_from_config_array, field_or_null,
    find_in_config_array, get_next_id,
};
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_DFUNC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct DfuncRow {
    #[serde(rename = "DFUNC_ID")]
    dfunc_id: i64,
    #[serde(rename = "DFUNC_CODE")]
    dfunc_code: String,
    #[serde(rename = "DFUNC_DESC")]
    dfunc_desc: Option<String>,
    #[serde(rename = "CONNECT_STR")]
    connect_str: Option<String>,
    #[serde(rename = "ANON_SUPPORT")]
    anon_support: String,
    #[serde(rename = "LANGUAGE")]
    language: Option<String>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a distinct function
///
/// `connect_str` is tri-valued to mirror the Python tool: `None` (or an absent
/// `connectStr` key) stores JSON `null`, `Some("")` stores an empty string, and
/// `Some(x)` stores `x`. A blank `connect_str` is accepted (Python parity).
#[derive(Debug, Clone, Default)]
pub struct AddDistinctFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddDistinctFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            // Absent or explicit-null connectStr -> None (stored as JSON null).
            connect_str: json.get("connectStr").and_then(|v| v.as_str()),
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
            anon_support: json.get("anonSupport").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting a distinct function
///
/// `connect_str` is a tri-state [`FieldUpdate`]: `Leave` keeps the stored value,
/// `Clear` writes JSON `null`, and `Set(x)` writes `x` (including `Set("")`).
#[derive(Debug, Clone, Default)]
pub struct SetDistinctFunctionParams<'a> {
    pub connect_str: FieldUpdate<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

/// Add a new distinct function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `dfunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (all optional; a `None` `connect_str` stores
///   JSON `null`, and a blank `connect_str` is accepted)
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_distinct_function(
    config_json: &str,
    dfunc_code: &str,
    params: AddDistinctFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let dfunc_code = dfunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Distinct function already exists: {dfunc_code}"
        )));
    }

    // Validate ANON_SUPPORT domain (["Yes", "No"], default "No"), mirroring
    // add_comparison_function.
    let anon_support = if let Some(val) = params.anon_support {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::validation(format!(
                    "Invalid ANON_SUPPORT value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Get next DFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let dfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_DFUNC", "DFUNC_ID", 1)?;

    // Build a complete row via DfuncRow so every CFG_DFUNC key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = DfuncRow {
        dfunc_id,
        dfunc_code,
        dfunc_desc: params.description.map(str::to_string),
        connect_str: params.connect_str.map(str::to_string),
        anon_support: anon_support.to_string(),
        language: params.language.map(str::to_string),
    };
    let new_record = serde_json::to_value(&row)?;

    // Add to CFG_DFUNC
    let modified_json = add_to_config_array(config_json, "CFG_DFUNC", new_record.clone())?;

    Ok((modified_json, new_record))
}

/// Delete a distinct function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `dfunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn delete_distinct_function(
    config_json: &str,
    dfunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let dfunc_code = dfunc_code.to_uppercase();

    // Find the function
    let function = find_in_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Distinct function not found: {dfunc_code}"))
        })?;

    // Delete from CFG_DFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?;

    Ok((modified_json, function))
}

/// Get a distinct function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `dfunc_code` - Function code to retrieve
///
/// # Returns
/// Result with the function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn get_distinct_function(config_json: &str, dfunc_code: &str) -> Result<Value, SzConfigError> {
    let dfunc_code = dfunc_code.to_uppercase();

    find_in_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?.ok_or_else(|| {
        SzConfigError::not_found(format!("Distinct function not found: {dfunc_code}"))
    })
}

/// List all distinct functions
///
/// # Arguments
/// * `config_json` - The configuration JSON string
///
/// # Returns
/// Result with vector of function records in display format
///
/// # Errors
/// Returns error if JSON is invalid
pub fn list_distinct_functions(config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_DFUNC")
        && let Some(items) = array.as_array()
    {
        items
            .iter()
            .map(|item| {
                // Stored-nullable columns (and the id, per D10) are projected
                // null-preserving via field_or_null rather than coerced.
                json!({
                    "id": field_or_null(item, "DFUNC_ID"),
                    "function": item.get("DFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "connectStr": field_or_null(item, "CONNECT_STR"),
                    "anonSupport": field_or_null(item, "ANON_SUPPORT"),
                    "language": field_or_null(item, "LANGUAGE")
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Set (update) a distinct function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `dfunc_code` - Function code to update
/// * `params` - Function parameters to update (all optional)
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_distinct_function(
    config_json: &str,
    dfunc_code: &str,
    params: SetDistinctFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let dfunc_code = dfunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Distinct function not found: {dfunc_code}"))
    })?;

    // In-place update of a complete existing row; all keys preserved.
    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        match params.connect_str {
            FieldUpdate::Leave => {}
            FieldUpdate::Clear => {
                obj.insert("CONNECT_STR".to_string(), Value::Null);
            }
            FieldUpdate::Set(conn) => {
                obj.insert("CONNECT_STR".to_string(), json!(conn));
            }
        }
        if let Some(desc) = params.description {
            obj.insert("DFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
        if let Some(anon) = params.anon_support {
            obj.insert("ANON_SUPPORT".to_string(), json!(anon));
        }
    }

    // Delete old and add updated
    let temp_json = delete_from_config_array(config_json, "CFG_DFUNC", "DFUNC_CODE", &dfunc_code)?;
    let modified_json = add_to_config_array(&temp_json, "CFG_DFUNC", function.clone())?;

    Ok((modified_json, function))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_DFUNC": [
                    {
                        "DFUNC_ID": 1,
                        "DFUNC_CODE": "DIST_NAME",
                        "CONNECT_STR": "g2DistName",
                        "LANGUAGE": "en"
                    }
                ]
            }
        })
        .to_string()
    }

    #[test]
    fn test_add_distinct_function() {
        let config = get_test_config();
        let result = add_distinct_function(
            &config,
            "custom_dist",
            AddDistinctFunctionParams {
                connect_str: Some("g2CustomDist"),
                description: Some("Custom distinct"),
                language: Some("en"),
                anon_support: Some("Yes"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_DIST"));
        assert_eq!(record["DFUNC_CODE"], "CUSTOM_DIST");
        assert_eq!(record["ANON_SUPPORT"], "Yes");
    }

    #[test]
    fn test_list_distinct_functions() {
        let config = get_test_config();
        let result = list_distinct_functions(&config);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["function"], "DIST_NAME");
    }

    /// add_distinct_function must write a complete CFG_DFUNC row even when the
    /// caller omits optionals — they become null, never dropped. ANON_SUPPORT
    /// defaults to "No".
    #[test]
    fn test_add_distinct_function_emits_all_keys() {
        let config = get_test_config();
        let (modified, record) = add_distinct_function(
            &config,
            "custom_dist",
            AddDistinctFunctionParams {
                connect_str: Some("g2CustomDist"),
                description: None,
                language: None,
                anon_support: None,
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_DFUNC"].as_array().unwrap();
        let obj = arr.last().unwrap().as_object().unwrap();
        for key in [
            "DFUNC_ID",
            "DFUNC_CODE",
            "DFUNC_DESC",
            "CONNECT_STR",
            "ANON_SUPPORT",
            "LANGUAGE",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["DFUNC_CODE"], json!("CUSTOM_DIST"));
        assert_eq!(obj["CONNECT_STR"], json!("g2CustomDist"));
        assert_eq!(obj["ANON_SUPPORT"], json!("No"));
        assert_eq!(obj["DFUNC_DESC"], Value::Null);
        assert_eq!(obj["LANGUAGE"], Value::Null);
        assert_eq!(record["DFUNC_DESC"], Value::Null);
        assert_eq!(record["ANON_SUPPORT"], json!("No"));
    }

    /// A blank connect_str is now accepted (the "CONNECTSTR cannot be blank"
    /// rejection was removed for Python parity) and stored as an empty string.
    #[test]
    fn test_add_distinct_function_accepts_blank_connect_str() {
        let (modified, _record) = add_distinct_function(
            &get_test_config(),
            "custom_blank",
            AddDistinctFunctionParams {
                connect_str: Some(""),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&modified).unwrap();
        let row = v["G2_CONFIG"]["CFG_DFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!(""));
    }

    /// connect_str tri-value on the add path: None -> null, Some("") -> "",
    /// Some(x) -> x.
    #[test]
    fn test_add_distinct_function_connect_str_tri_value() {
        let (m_none, _) = add_distinct_function(
            &get_test_config(),
            "d_none",
            AddDistinctFunctionParams {
                connect_str: None,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_none).unwrap();
        let row = v["G2_CONFIG"]["CFG_DFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_val, _) = add_distinct_function(
            &get_test_config(),
            "d_val",
            AddDistinctFunctionParams {
                connect_str: Some("g2X"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_val).unwrap();
        let row = v["G2_CONFIG"]["CFG_DFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!("g2X"));
    }

    /// connect_str tri-state on the set path: Leave keeps, Clear nulls, Set writes.
    #[test]
    fn test_set_distinct_function_connect_str_tri_state() {
        let base = get_test_config();

        let (m_leave, _) = set_distinct_function(
            &base,
            "DIST_NAME",
            SetDistinctFunctionParams {
                connect_str: FieldUpdate::Leave,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_leave).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_DFUNC"][0]["CONNECT_STR"],
            json!("g2DistName")
        );

        let (m_clear, _) = set_distinct_function(
            &base,
            "DIST_NAME",
            SetDistinctFunctionParams {
                connect_str: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_clear).unwrap();
        let row = &v["G2_CONFIG"]["CFG_DFUNC"][0];
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_set, _) = set_distinct_function(
            &base,
            "DIST_NAME",
            SetDistinctFunctionParams {
                connect_str: FieldUpdate::Set("g2New"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_set).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_DFUNC"][0]["CONNECT_STR"],
            json!("g2New")
        );
    }
}
