//! Standardize function operations for Senzing configuration
//!
//! This module provides functions for managing standardize functions (CFG_SFUNC)
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

/// Complete CFG_SFUNC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct SfuncRow {
    #[serde(rename = "SFUNC_ID")]
    sfunc_id: i64,
    #[serde(rename = "SFUNC_CODE")]
    sfunc_code: String,
    #[serde(rename = "CONNECT_STR")]
    connect_str: Option<String>,
    #[serde(rename = "SFUNC_DESC")]
    sfunc_desc: Option<String>,
    #[serde(rename = "LANGUAGE")]
    language: Option<String>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a standardize function
///
/// `connect_str` is tri-valued to mirror the Python tool: `None` (or an absent
/// `connectStr` key) stores JSON `null`, `Some("")` stores an empty string, and
/// `Some(x)` stores `x`.
#[derive(Debug, Clone, Default)]
pub struct AddStandardizeFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddStandardizeFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            // Absent or explicit-null connectStr -> None (stored as JSON null).
            connect_str: json.get("connectStr").and_then(|v| v.as_str()),
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting a standardize function
///
/// `connect_str` is a tri-state [`FieldUpdate`]: `Leave` keeps the stored value,
/// `Clear` writes JSON `null`, and `Set(x)` writes `x` (including `Set("")`).
#[derive(Debug, Clone, Default)]
pub struct SetStandardizeFunctionParams<'a> {
    pub connect_str: FieldUpdate<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

/// Add a new standardize function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (all optional; a `None` `connect_str` stores
///   JSON `null`)
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_standardize_function(
    config_json: &str,
    sfunc_code: &str,
    params: AddStandardizeFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Standardize function already exists: {sfunc_code}"
        )));
    }

    // Get next SFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let sfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_SFUNC", "SFUNC_ID", 1)?;

    // Build a complete row via SfuncRow so every CFG_SFUNC key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = SfuncRow {
        sfunc_id,
        sfunc_code,
        connect_str: params.connect_str.map(str::to_string),
        sfunc_desc: params.description.map(str::to_string),
        language: params.language.map(str::to_string),
    };
    let new_record = serde_json::to_value(&row)?;

    // Add to CFG_SFUNC
    let modified_json = add_to_config_array(config_json, "CFG_SFUNC", new_record.clone())?;

    Ok((modified_json, new_record))
}

/// Delete a standardize function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn delete_standardize_function(
    config_json: &str,
    sfunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    // Find the function
    let function = find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Standardize function not found: {sfunc_code}"))
        })?;

    // Delete from CFG_SFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?;

    Ok((modified_json, function))
}

/// Get a standardize function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code to retrieve
///
/// # Returns
/// Result with the function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn get_standardize_function(
    config_json: &str,
    sfunc_code: &str,
) -> Result<Value, SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?.ok_or_else(|| {
        SzConfigError::not_found(format!("Standardize function not found: {sfunc_code}"))
    })
}

/// List all standardize functions
///
/// # Arguments
/// * `config_json` - The configuration JSON string
///
/// # Returns
/// Result with vector of function records in display format
///
/// # Errors
/// Returns error if JSON is invalid
pub fn list_standardize_functions(config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_SFUNC")
        && let Some(items) = array.as_array()
    {
        items
            .iter()
            .map(|item| {
                // connectStr and language are stored-nullable; project them
                // null-preserving via field_or_null rather than coercing to "".
                json!({
                    "id": item.get("SFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("SFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "connectStr": field_or_null(item, "CONNECT_STR"),
                    "language": field_or_null(item, "LANGUAGE")
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Set (update) a standardize function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code to update
/// * `params` - Function parameters to update (all optional)
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_standardize_function(
    config_json: &str,
    sfunc_code: &str,
    params: SetStandardizeFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Standardize function not found: {sfunc_code}"))
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
            obj.insert("SFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
    }

    // Delete old and add updated
    let temp_json = delete_from_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?;
    let modified_json = add_to_config_array(&temp_json, "CFG_SFUNC", function.clone())?;

    Ok((modified_json, function))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_SFUNC": [
                    {
                        "SFUNC_ID": 1,
                        "SFUNC_CODE": "PARSE_NAME",
                        "CONNECT_STR": "g2ParseName",
                        "LANGUAGE": "en"
                    }
                ]
            }
        })
        .to_string()
    }

    #[test]
    fn test_add_standardize_function() {
        let config = get_test_config();
        let result = add_standardize_function(
            &config,
            "custom_parse",
            AddStandardizeFunctionParams {
                connect_str: Some("g2CustomParse"),
                description: Some("Custom parser"),
                language: Some("en"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_PARSE"));
        assert_eq!(record["SFUNC_CODE"], "CUSTOM_PARSE");
    }

    #[test]
    fn test_list_standardize_functions() {
        let config = get_test_config();
        let result = list_standardize_functions(&config);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["function"], "PARSE_NAME");
    }

    #[test]
    fn test_get_standardize_function() {
        let config = get_test_config();
        let result = get_standardize_function(&config, "PARSE_NAME");
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func["SFUNC_CODE"], "PARSE_NAME");
    }

    #[test]
    fn test_delete_standardize_function() {
        let config = get_test_config();
        let result = delete_standardize_function(&config, "PARSE_NAME");
        assert!(result.is_ok());
        let (modified, _) = result.unwrap();
        assert!(!modified.contains("PARSE_NAME"));
    }

    /// add_standardize_function must write a complete CFG_SFUNC row even when the
    /// caller omits optionals — they become null, never dropped.
    #[test]
    fn test_add_standardize_function_emits_all_keys() {
        let config = get_test_config();
        let (modified, record) = add_standardize_function(
            &config,
            "custom_parse",
            AddStandardizeFunctionParams {
                connect_str: Some("g2CustomParse"),
                description: None,
                language: None,
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_SFUNC"].as_array().unwrap();
        let obj = arr.last().unwrap().as_object().unwrap();
        for key in [
            "SFUNC_ID",
            "SFUNC_CODE",
            "CONNECT_STR",
            "SFUNC_DESC",
            "LANGUAGE",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["SFUNC_CODE"], json!("CUSTOM_PARSE"));
        assert_eq!(obj["CONNECT_STR"], json!("g2CustomParse"));
        assert_eq!(obj["SFUNC_DESC"], Value::Null);
        assert_eq!(obj["LANGUAGE"], Value::Null);
        assert_eq!(record["SFUNC_DESC"], Value::Null);
    }

    /// connect_str tri-value on the add path: None -> null, Some("") -> "",
    /// Some(x) -> x.
    #[test]
    fn test_add_standardize_function_connect_str_tri_value() {
        let (m_none, _) = add_standardize_function(
            &get_test_config(),
            "s_none",
            AddStandardizeFunctionParams {
                connect_str: None,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_none).unwrap();
        let row = v["G2_CONFIG"]["CFG_SFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_empty, _) = add_standardize_function(
            &get_test_config(),
            "s_empty",
            AddStandardizeFunctionParams {
                connect_str: Some(""),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_empty).unwrap();
        let row = v["G2_CONFIG"]["CFG_SFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!(""));
    }

    /// connect_str tri-state on the set path: Leave keeps, Clear nulls, Set writes.
    #[test]
    fn test_set_standardize_function_connect_str_tri_state() {
        let base = get_test_config();

        let (m_leave, _) = set_standardize_function(
            &base,
            "PARSE_NAME",
            SetStandardizeFunctionParams {
                connect_str: FieldUpdate::Leave,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_leave).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_SFUNC"][0]["CONNECT_STR"],
            json!("g2ParseName")
        );

        let (m_clear, _) = set_standardize_function(
            &base,
            "PARSE_NAME",
            SetStandardizeFunctionParams {
                connect_str: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_clear).unwrap();
        let row = &v["G2_CONFIG"]["CFG_SFUNC"][0];
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_set, _) = set_standardize_function(
            &base,
            "PARSE_NAME",
            SetStandardizeFunctionParams {
                connect_str: FieldUpdate::Set("g2New"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_set).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_SFUNC"][0]["CONNECT_STR"],
            json!("g2New")
        );
    }
}
