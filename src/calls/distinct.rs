//! Distinct call management operations
//!
//! Functions for managing CFG_DFCALL (distinct calls) and CFG_DFBOM
//! (distinct bill of materials) configuration sections.

use crate::error::{Result, SzConfigError};
use crate::helpers::{
    find_in_config_array, get_next_id, lookup_dfunc_id, lookup_element_id, lookup_feature_id,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a distinct call
#[derive(Debug, Clone)]
pub struct AddDistinctCallParams {
    pub ftype_code: String,
    pub dfunc_code: String,
    pub element_list: Vec<String>,
}

impl TryFrom<&Value> for AddDistinctCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        Ok(Self {
            ftype_code: json
                .get("ftypeCode")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("ftypeCode".to_string()))?
                .to_string(),
            dfunc_code: json
                .get("dfuncCode")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("dfuncCode".to_string()))?
                .to_string(),
            element_list: json
                .get("elementList")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

/// Parameters for adding a distinct call element
#[derive(Debug, Clone)]
pub struct AddDistinctCallElementParams {
    pub dfcall_id: i64,
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
}

/// Parameters for deleting a distinct call element
#[derive(Debug, Clone)]
pub struct DeleteDistinctCallElementParams {
    pub dfcall_id: i64,
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
}

/// Parameters for setting (updating) a distinct call
#[derive(Debug, Clone, Default)]
pub struct SetDistinctCallParams {
    pub dfcall_id: i64,
    pub exec_order: Option<i64>,
}

impl TryFrom<&Value> for SetDistinctCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let dfcall_id = json
            .get("dfcallId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("dfcallId".to_string()))?;

        Ok(Self {
            dfcall_id,
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for setting a distinct call element
#[derive(Debug, Clone)]
pub struct SetDistinctCallElementParams {
    pub dfcall_id: i64,
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
    pub updates: Value,
}

/// Add a new distinct call with element list
///
/// Creates a new distinct call linking a function to a feature
/// with associated elements (DBOM records).
/// Note: Only one distinct call is allowed per feature.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Distinct call parameters (ftype_code, dfunc_code, element_list required)
///
/// # Returns
/// Tuple of (modified_config, new_dfcall_record)
///
/// # Errors
/// - `Duplicate` if a distinct call already exists for this feature
/// - `NotFound` if function/feature/element codes don't exist
pub fn add_distinct_call(config: &str, params: AddDistinctCallParams) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Get next DFCALL_ID (seed at 1000 for user-created calls)
    let dfcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_DFCALL", "DFCALL_ID", 1000)?;

    // Lookup feature ID
    let ftype_id = lookup_feature_id(config, &params.ftype_code)?;

    // Check if distinct call already exists for this feature (only one allowed per feature)
    let call_exists = config_data["G2_CONFIG"]["CFG_DFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
        })
        .unwrap_or(false);

    if call_exists {
        return Err(SzConfigError::AlreadyExists(format!(
            "Distinct call for feature {} already set",
            params.ftype_code
        )));
    }

    // Lookup function ID
    let dfunc_id = lookup_dfunc_id(config, &params.dfunc_code)?;

    // Process element list and create DFBOM records
    let mut dfbom_records = Vec::new();
    let mut exec_order = 0;

    for element_code in params.element_list {
        exec_order += 1;

        // Lookup element ID (must belong to the feature)
        let bom_felem_id = config_data["G2_CONFIG"]["CFG_FBOM"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|fbom| {
                        fbom["FTYPE_ID"].as_i64() == Some(ftype_id)
                            && fbom["FELEM_CODE"]
                                .as_str()
                                .map(|s| s.eq_ignore_ascii_case(&element_code))
                                .unwrap_or(false)
                    })
                    .and_then(|fbom| fbom["FELEM_ID"].as_i64())
            })
            .or_else(|| {
                // Fallback: lookup element globally
                lookup_element_id(config, &element_code).ok()
            })
            .ok_or_else(|| {
                SzConfigError::NotFound(format!(
                    "Element '{}' not found in feature '{}'",
                    element_code, params.ftype_code
                ))
            })?;

        // Create DFBOM record
        dfbom_records.push(json!({
            "DFCALL_ID": dfcall_id,
            "FTYPE_ID": ftype_id,
            "FELEM_ID": bom_felem_id,
            "EXEC_ORDER": exec_order
        }));
    }

    // Create new CFG_DFCALL record (EXEC_ORDER is always 1 for distinct calls)
    let new_record = json!({
        "DFCALL_ID": dfcall_id,
        "FTYPE_ID": ftype_id,
        "DFUNC_ID": dfunc_id,
        "EXEC_ORDER": 1
    });

    // Add to config
    if let Some(dfcall_array) = config_data["G2_CONFIG"]["CFG_DFCALL"].as_array_mut() {
        dfcall_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_DFCALL".to_string()));
    }

    if let Some(dfbom_array) = config_data["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        dfbom_array.extend(dfbom_records);
    } else {
        return Err(SzConfigError::MissingSection("CFG_DFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a distinct call by ID
///
/// Also deletes associated DFBOM records.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `dfcall_id` - Distinct call ID to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn delete_distinct_call(config: &str, dfcall_id: i64) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the call exists
    let call_exists = config_data["G2_CONFIG"]["CFG_DFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["DFCALL_ID"].as_i64() == Some(dfcall_id))
        })
        .unwrap_or(false);

    if !call_exists {
        return Err(SzConfigError::NotFound(format!(
            "Distinct call ID {}",
            dfcall_id
        )));
    }

    // Delete the distinct call
    if let Some(dfcall_array) = config_data["G2_CONFIG"]["CFG_DFCALL"].as_array_mut() {
        dfcall_array.retain(|record| record["DFCALL_ID"].as_i64() != Some(dfcall_id));
    }

    // Delete associated DFBOM records
    if let Some(dfbom_array) = config_data["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        dfbom_array.retain(|record| record["DFCALL_ID"].as_i64() != Some(dfcall_id));
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single distinct call by ID
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `dfcall_id` - Distinct call ID
///
/// # Returns
/// JSON Value representing the distinct call record
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn get_distinct_call(config: &str, dfcall_id: i64) -> Result<Value> {
    find_in_config_array(config, "CFG_DFCALL", "DFCALL_ID", &dfcall_id.to_string())?
        .ok_or_else(|| SzConfigError::NotFound(format!("Distinct call ID {}", dfcall_id)))
}

/// List all distinct calls with resolved names
///
/// Returns all distinct calls with feature and function codes resolved.
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names
pub fn list_distinct_calls(config: &str) -> Result<Vec<Value>> {
    let config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let empty_array = vec![];
    let dfcall_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFCALL"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let ftype_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let dfunc_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFUNC"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    // Helper functions for ID resolution
    let resolve_ftype = |ftype_id: i64| -> String {
        ftype_array
            .iter()
            .find(|ft| ft.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id))
            .and_then(|ft| ft.get("FTYPE_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    let resolve_dfunc = |dfunc_id: i64| -> String {
        dfunc_array
            .iter()
            .find(|df| df.get("DFUNC_ID").and_then(|v| v.as_i64()) == Some(dfunc_id))
            .and_then(|df| df.get("DFUNC_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Transform distinct calls
    let items: Vec<Value> = dfcall_array
        .iter()
        .map(|item| {
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let dfunc_id = item.get("DFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            json!({
                "id": item.get("DFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "feature": resolve_ftype(ftype_id),
                "function": resolve_dfunc(dfunc_id),
                "execOrder": item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(1)
            })
        })
        .collect();

    Ok(items)
}

/// Update a distinct call (stub - not implemented in Python)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Distinct call parameters (dfcall_id required, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_distinct_call(config: &str, _params: SetDistinctCallParams) -> Result<String> {
    // This is a stub - the Python version doesn't implement this
    Ok(config.to_string())
}

/// Add a distinct call element (DBOM record)
///
/// Creates a new distinct bill of materials entry.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters (dfcall_id, ftype_id, felem_id, exec_order)
///
/// # Returns
/// Tuple of (modified_config, new_dbom_record)
pub fn add_distinct_call_element(
    config: &str,
    params: AddDistinctCallElementParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Check if element already exists
    if let Some(dbom_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFBOM"))
        .and_then(|v| v.as_array())
    {
        for item in dbom_array {
            if item.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(params.dfcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order)
            {
                return Err(SzConfigError::AlreadyExists(
                    "Distinct call element already exists".to_string(),
                ));
            }
        }
    }

    // Create new DBOM record
    let new_record = json!({
        "DFCALL_ID": params.dfcall_id,
        "FTYPE_ID": params.ftype_id,
        "FELEM_ID": params.felem_id,
        "EXEC_ORDER": params.exec_order
    });

    // Add to CFG_DFBOM
    if let Some(dbom_array) = config_data["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        dbom_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_DFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a distinct call element
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters (dfcall_id, ftype_id, felem_id, exec_order)
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_distinct_call_element(
    config: &str,
    params: DeleteDistinctCallElementParams,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the element exists
    let element_exists = config_data["G2_CONFIG"]["CFG_DFBOM"]
        .as_array()
        .map(|arr| {
            arr.iter().any(|item| {
                item.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(params.dfcall_id)
                    && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                    && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
                    && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order)
            })
        })
        .unwrap_or(false);

    if !element_exists {
        return Err(SzConfigError::NotFound(
            "Distinct call element not found".to_string(),
        ));
    }

    // Delete the element
    if let Some(dbom_array) = config_data["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        dbom_array.retain(|item| {
            !(item.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(params.dfcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order))
        });
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Update a distinct call element (stub - not typically used)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters including updates
///
/// # Returns
/// Modified configuration JSON string
pub fn set_distinct_call_element(
    config: &str,
    _params: SetDistinctCallElementParams,
) -> Result<String> {
    // This is a stub - not commonly used
    Ok(config.to_string())
}
