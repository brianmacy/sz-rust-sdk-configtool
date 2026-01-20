//! Standardize function operations for Senzing configuration
//!
//! This module provides functions for managing standardize functions (CFG_SFUNC)
//! in the Senzing configuration JSON.

use crate::error::SzConfigError;
use crate::helpers::{
    add_to_config_array, delete_from_config_array, find_in_config_array, get_next_id,
};
use serde_json::{Value, json};

/// Add a new standardize function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code (will be uppercased)
/// * `connect_str` - Connection string
/// * `sfunc_desc` - Optional description
/// * `language` - Optional language code
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_standardize_function(
    config_json: &str,
    sfunc_code: &str,
    connect_str: &str,
    sfunc_desc: Option<&str>,
    language: Option<&str>,
) -> Result<(String, Value), SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Standardize function already exists: {}",
            sfunc_code
        )));
    }

    // Get next SFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let sfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_SFUNC", "SFUNC_ID", 1)?;

    // Create new function record
    let mut new_record = json!({
        "SFUNC_ID": sfunc_id,
        "SFUNC_CODE": sfunc_code,
        "CONNECT_STR": connect_str,
    });

    // Add optional fields
    if let Some(obj) = new_record.as_object_mut() {
        if let Some(desc) = sfunc_desc {
            obj.insert("SFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
    }

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
            SzConfigError::not_found(format!("Standardize function not found: {}", sfunc_code))
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
        SzConfigError::not_found(format!("Standardize function not found: {}", sfunc_code))
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
                json!({
                    "id": item.get("SFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("SFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
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

/// Set (update) a standardize function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `sfunc_code` - Function code to update
/// * `connect_str` - Optional new connection string
/// * `sfunc_desc` - Optional new description
/// * `language` - Optional new language code
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_standardize_function(
    config_json: &str,
    sfunc_code: &str,
    connect_str: Option<&str>,
    sfunc_desc: Option<&str>,
    language: Option<&str>,
) -> Result<(String, Value), SzConfigError> {
    let sfunc_code = sfunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_SFUNC", "SFUNC_CODE", &sfunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Standardize function not found: {}", sfunc_code))
    })?;

    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        if let Some(conn) = connect_str {
            obj.insert("CONNECT_STR".to_string(), json!(conn));
        }
        if let Some(desc) = sfunc_desc {
            obj.insert("SFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = language {
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
            "g2CustomParse",
            Some("Custom parser"),
            Some("en"),
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
}
