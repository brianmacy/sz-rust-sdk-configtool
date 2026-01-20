//! Distinct function operations for Senzing configuration
//!
//! This module provides functions for managing distinct functions (CFG_DFUNC)
//! in the Senzing configuration JSON.

use crate::error::SzConfigError;
use crate::helpers::{
    add_to_config_array, delete_from_config_array, find_in_config_array, get_next_id,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a distinct function
#[derive(Debug, Clone, Default)]
pub struct AddDistinctFunctionParams<'a> {
    pub connect_str: &'a str,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddDistinctFunctionParams<'a> {
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

/// Parameters for setting a distinct function
#[derive(Debug, Clone, Default)]
pub struct SetDistinctFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

/// Add a new distinct function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `dfunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (connect_str required, others optional)
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
            "Distinct function already exists: {}",
            dfunc_code
        )));
    }

    // Get next DFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let dfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_DFUNC", "DFUNC_ID", 1)?;

    // Create new function record
    let mut new_record = json!({
        "DFUNC_ID": dfunc_id,
        "DFUNC_CODE": dfunc_code,
        "CONNECT_STR": params.connect_str,
    });

    // Add optional fields
    if let Some(obj) = new_record.as_object_mut() {
        if let Some(desc) = params.description {
            obj.insert("DFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
    }

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
            SzConfigError::not_found(format!("Distinct function not found: {}", dfunc_code))
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
        SzConfigError::not_found(format!("Distinct function not found: {}", dfunc_code))
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
                json!({
                    "id": item.get("DFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("DFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
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
        SzConfigError::not_found(format!("Distinct function not found: {}", dfunc_code))
    })?;

    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        if let Some(conn) = params.connect_str {
            obj.insert("CONNECT_STR".to_string(), json!(conn));
        }
        if let Some(desc) = params.description {
            obj.insert("DFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
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
                connect_str: "g2CustomDist",
                description: Some("Custom distinct"),
                language: Some("en"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_DIST"));
        assert_eq!(record["DFUNC_CODE"], "CUSTOM_DIST");
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
}
