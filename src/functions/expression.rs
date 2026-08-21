//! Expression function operations for Senzing configuration
//!
//! This module provides functions for managing expression functions (CFG_EFUNC)
//! in the Senzing configuration JSON.

use crate::error::SzConfigError;
use crate::helpers::{
    add_to_config_array, delete_from_config_array, find_in_config_array, get_next_id,
};
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_EFUNC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct EfuncRow {
    #[serde(rename = "EFUNC_ID")]
    efunc_id: i64,
    #[serde(rename = "EFUNC_CODE")]
    efunc_code: String,
    #[serde(rename = "CONNECT_STR")]
    connect_str: String,
    #[serde(rename = "EFUNC_DESC")]
    efunc_desc: Option<String>,
    #[serde(rename = "LANGUAGE")]
    language: Option<String>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an expression function
#[derive(Debug, Clone, Default)]
pub struct AddExpressionFunctionParams<'a> {
    pub connect_str: &'a str,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddExpressionFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            connect_str: json
                .get("connectStr")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("connectStr".to_string()))?,
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting an expression function
#[derive(Debug, Clone, Default)]
pub struct SetExpressionFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

/// Add a new expression function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (connect_str required, others optional)
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_expression_function(
    config_json: &str,
    efunc_code: &str,
    params: AddExpressionFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Expression function already exists: {efunc_code}"
        )));
    }

    // Get next EFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let efunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_EFUNC", "EFUNC_ID", 1)?;

    // Build a complete row via EfuncRow so every CFG_EFUNC key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = EfuncRow {
        efunc_id,
        efunc_code,
        connect_str: params.connect_str.to_string(),
        efunc_desc: params.description.map(str::to_string),
        language: params.language.map(str::to_string),
    };
    let new_record = serde_json::to_value(&row)?;

    // Add to CFG_EFUNC
    let modified_json = add_to_config_array(config_json, "CFG_EFUNC", new_record.clone())?;

    Ok((modified_json, new_record))
}

/// Delete an expression function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn delete_expression_function(
    config_json: &str,
    efunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Find the function
    let function = find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
        })?;

    // Delete from CFG_EFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?;

    Ok((modified_json, function))
}

/// Get an expression function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to retrieve
///
/// # Returns
/// Result with the function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn get_expression_function(
    config_json: &str,
    efunc_code: &str,
) -> Result<Value, SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?.ok_or_else(|| {
        SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
    })
}

/// List all expression functions
///
/// # Arguments
/// * `config_json` - The configuration JSON string
///
/// # Returns
/// Result with vector of function records in display format
///
/// # Errors
/// Returns error if JSON is invalid
pub fn list_expression_functions(config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_EFUNC")
        && let Some(items) = array.as_array()
    {
        items
            .iter()
            .map(|item| {
                json!({
                    "id": item.get("EFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("EFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "connectStr": item.get("CONNECT_STR").and_then(|v| v.as_str()).unwrap_or(""),
                    "language": item.get("LANGUAGE").and_then(|v| v.as_str()).unwrap_or("")
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Set (update) an expression function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to update
/// * `params` - Function parameters to update (all optional)
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_expression_function(
    config_json: &str,
    efunc_code: &str,
    params: SetExpressionFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
    })?;

    // In-place update of a complete existing row; all keys preserved.
    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        if let Some(conn) = params.connect_str {
            obj.insert("CONNECT_STR".to_string(), json!(conn));
        }
        if let Some(desc) = params.description {
            obj.insert("EFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
    }

    // Delete old and add updated
    let temp_json = delete_from_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?;
    let modified_json = add_to_config_array(&temp_json, "CFG_EFUNC", function.clone())?;

    Ok((modified_json, function))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_EFUNC": [
                    {
                        "EFUNC_ID": 1,
                        "EFUNC_CODE": "EXPR_FEAT",
                        "CONNECT_STR": "g2ExprFeat",
                        "LANGUAGE": "en"
                    }
                ]
            }
        })
        .to_string()
    }

    #[test]
    fn test_add_expression_function() {
        let config = get_test_config();
        let result = add_expression_function(
            &config,
            "custom_expr",
            AddExpressionFunctionParams {
                connect_str: "g2CustomExpr",
                description: Some("Custom expression"),
                language: Some("en"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_EXPR"));
        assert_eq!(record["EFUNC_CODE"], "CUSTOM_EXPR");
    }

    #[test]
    fn test_list_expression_functions() {
        let config = get_test_config();
        let result = list_expression_functions(&config);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["function"], "EXPR_FEAT");
    }

    /// add_expression_function must write a complete CFG_EFUNC row even when the
    /// caller omits optionals — they become null, never dropped.
    #[test]
    fn test_add_expression_function_emits_all_keys() {
        let config = get_test_config();
        let (modified, record) = add_expression_function(
            &config,
            "custom_expr",
            AddExpressionFunctionParams {
                connect_str: "g2CustomExpr",
                description: None,
                language: None,
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_EFUNC"].as_array().unwrap();
        let obj = arr.last().unwrap().as_object().unwrap();
        for key in [
            "EFUNC_ID",
            "EFUNC_CODE",
            "CONNECT_STR",
            "EFUNC_DESC",
            "LANGUAGE",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["EFUNC_CODE"], json!("CUSTOM_EXPR"));
        assert_eq!(obj["CONNECT_STR"], json!("g2CustomExpr"));
        assert_eq!(obj["EFUNC_DESC"], Value::Null);
        assert_eq!(obj["LANGUAGE"], Value::Null);
        // Returned record mirrors the written row.
        assert_eq!(record["EFUNC_DESC"], Value::Null);
    }
}
