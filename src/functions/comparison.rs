//! Comparison function operations for Senzing configuration
//!
//! This module provides functions for managing comparison functions (CFG_CFUNC)
//! and comparison function return codes (CFG_CFRTN) in the Senzing configuration JSON.

use crate::error::SzConfigError;
use crate::helpers::{
    add_to_config_array, delete_from_config_array, find_in_config_array, get_next_id,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison function
#[derive(Debug, Clone, Default)]
pub struct AddComparisonFunctionParams<'a> {
    pub connect_str: &'a str,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddComparisonFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            connect_str: json
                .get("connectStr")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("connectStr".to_string()))?,
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
            anon_support: json.get("anonSupport").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting a comparison function
#[derive(Debug, Clone, Default)]
pub struct SetComparisonFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

/// Add a new comparison function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (connect_str required, others optional)
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_comparison_function(
    config_json: &str,
    cfunc_code: &str,
    params: AddComparisonFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Comparison function already exists: {}",
            cfunc_code
        )));
    }

    // Get next CFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let cfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_CFUNC", "CFUNC_ID", 1)?;

    // Create new function record
    let mut new_record = json!({
        "CFUNC_ID": cfunc_id,
        "CFUNC_CODE": cfunc_code,
        "CONNECT_STR": params.connect_str,
    });

    // Add optional fields
    if let Some(obj) = new_record.as_object_mut() {
        if let Some(desc) = params.description {
            obj.insert("CFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
        if let Some(anon) = params.anon_support {
            obj.insert("ANON_SUPPORT".to_string(), json!(anon));
        }
    }

    // Add to CFG_CFUNC
    let modified_json = add_to_config_array(config_json, "CFG_CFUNC", new_record.clone())?;

    Ok((modified_json, new_record))
}

/// Delete a comparison function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn delete_comparison_function(
    config_json: &str,
    cfunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();

    // Find the function
    let function = find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Comparison function not found: {}", cfunc_code))
        })?;

    // Delete from CFG_CFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?;

    Ok((modified_json, function))
}

/// Get a comparison function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code to retrieve
///
/// # Returns
/// Result with the function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn get_comparison_function(
    config_json: &str,
    cfunc_code: &str,
) -> Result<Value, SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();

    find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?.ok_or_else(|| {
        SzConfigError::not_found(format!("Comparison function not found: {}", cfunc_code))
    })
}

/// List all comparison functions
///
/// # Arguments
/// * `config_json` - The configuration JSON string
///
/// # Returns
/// Result with vector of function records in display format
///
/// # Errors
/// Returns error if JSON is invalid
pub fn list_comparison_functions(config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_CFUNC")
        && let Some(items) = array.as_array()
    {
        items
            .iter()
            .map(|item| {
                json!({
                    "id": item.get("CFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("CFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "connectStr": item.get("CONNECT_STR").and_then(|v| v.as_str()).unwrap_or(""),
                    "language": item.get("LANGUAGE").and_then(|v| v.as_str()).unwrap_or(""),
                    "anonSupport": item.get("ANON_SUPPORT").and_then(|v| v.as_str()).unwrap_or("")
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Set (update) a comparison function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code to update
/// * `params` - Function parameters to update (all optional)
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_comparison_function(
    config_json: &str,
    cfunc_code: &str,
    params: SetComparisonFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Comparison function not found: {}", cfunc_code))
    })?;

    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        if let Some(conn) = params.connect_str {
            obj.insert("CONNECT_STR".to_string(), json!(conn));
        }
        if let Some(desc) = params.description {
            obj.insert("CFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
        if let Some(anon) = params.anon_support {
            obj.insert("ANON_SUPPORT".to_string(), json!(anon));
        }
    }

    // Delete old and add updated
    let temp_json = delete_from_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?;
    let modified_json = add_to_config_array(&temp_json, "CFG_CFUNC", function.clone())?;

    Ok((modified_json, function))
}

/// Add a comparison function return code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code (will be uppercased)
/// * `cfrtn_code` - Return code (will be uppercased)
/// * `cfrtn_desc` - Optional description
///
/// # Returns
/// Result with modified JSON string and the new return code record
///
/// # Errors
/// Returns error if function not found, return code exists, or JSON is invalid
pub fn add_comparison_func_return_code(
    config_json: &str,
    cfunc_code: &str,
    cfrtn_code: &str,
    cfrtn_desc: Option<&str>,
) -> Result<(String, Value), SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();
    let cfrtn_code = cfrtn_code.to_uppercase();

    // Find the function
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let cfunc = find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Comparison function not found: {}", cfunc_code))
        })?;

    let cfunc_id = cfunc
        .get("CFUNC_ID")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| SzConfigError::validation("CFUNC_ID not found"))?;

    // Check if return code already exists for this function
    if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_CFRTN")
        && let Some(items) = array.as_array()
        && items.iter().any(|item| {
            item.get("CFUNC_ID").and_then(|v| v.as_i64()) == Some(cfunc_id)
                && item.get("CFRTN_CODE").and_then(|v| v.as_str()) == Some(&cfrtn_code)
        })
    {
        return Err(SzConfigError::validation(format!(
            "Return code {} already exists for function {}",
            cfrtn_code, cfunc_code
        )));
    }

    // Get next CFRTN_ID
    let cfrtn_id = get_next_id(&config_data, "G2_CONFIG.CFG_CFRTN", "CFRTN_ID", 1)?;

    // Create new return code record
    let mut new_record = json!({
        "CFRTN_ID": cfrtn_id,
        "CFUNC_ID": cfunc_id,
        "CFRTN_CODE": cfrtn_code,
    });

    // Add optional description
    if let Some(desc) = cfrtn_desc {
        if let Some(obj) = new_record.as_object_mut() {
            obj.insert("CFRTN_DESC".to_string(), json!(desc));
        }
    }

    // Add to CFG_CFRTN
    let modified_json = add_to_config_array(config_json, "CFG_CFRTN", new_record.clone())?;

    Ok((modified_json, new_record))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_CFUNC": [
                    {
                        "CFUNC_ID": 1,
                        "CFUNC_CODE": "CMP_NAME",
                        "CONNECT_STR": "g2CmpName",
                        "LANGUAGE": "en"
                    }
                ],
                "CFG_CFRTN": []
            }
        })
        .to_string()
    }

    #[test]
    fn test_add_comparison_function() {
        let config = get_test_config();
        let result = add_comparison_function(
            &config,
            "custom_cmp",
            AddComparisonFunctionParams {
                connect_str: "g2CustomCmp",
                description: Some("Custom compare"),
                language: Some("en"),
                anon_support: Some("Yes"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_CMP"));
        assert_eq!(record["CFUNC_CODE"], "CUSTOM_CMP");
    }

    #[test]
    fn test_list_comparison_functions() {
        let config = get_test_config();
        let result = list_comparison_functions(&config);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["function"], "CMP_NAME");
    }
}
