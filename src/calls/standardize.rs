//! Standardize call management operations
//!
//! Functions for managing CFG_SFCALL (standardize calls) and CFG_SBOM
//! (standardize bill of materials) configuration sections.

use crate::error::{Result, SzConfigError};
use crate::helpers::{
    find_in_config_array, get_next_id, lookup_element_id, lookup_feature_id, lookup_sfunc_id,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a standardize call
#[derive(Debug, Clone)]
pub struct AddStandardizeCallParams<'a> {
    pub ftype_code: Option<&'a str>,
    pub felem_code: Option<&'a str>,
    pub exec_order: Option<i64>,
    pub sfunc_code: &'a str,
}

impl<'a> AddStandardizeCallParams<'a> {
    pub fn new(sfunc_code: &'a str) -> Self {
        Self {
            ftype_code: None,
            felem_code: None,
            exec_order: None,
            sfunc_code,
        }
    }
}

/// Parameters for adding a standardize call element
#[derive(Debug, Clone)]
pub struct AddStandardizeCallElementParams {
    pub ftype_id: i64,
    pub sfunc_id: i64,
    pub felem_id: Option<i64>,
    pub exec_order: Option<i64>,
}

/// Parameters for setting (updating) a standardize call
#[derive(Debug, Clone, Default)]
pub struct SetStandardizeCallParams {
    pub sfcall_id: i64,
    pub exec_order: Option<i64>,
}

impl TryFrom<&Value> for SetStandardizeCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let sfcall_id = json
            .get("sfcallId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("sfcallId".to_string()))?;

        Ok(Self {
            sfcall_id,
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for deleting a standardize call element
#[derive(Debug, Clone)]
pub struct DeleteStandardizeCallElementParams {
    pub ftype_id: i64,
    pub sfunc_id: i64,
    pub felem_id: Option<i64>,
}

/// Parameters for setting a standardize call element
#[derive(Debug, Clone)]
pub struct SetStandardizeCallElementParams {
    pub ftype_id: i64,
    pub sfunc_id: i64,
    pub felem_id: Option<i64>,
    pub updates: Value,
}

/// Add a new standardize call
///
/// Creates a new standardize call linking a function to a feature or element
/// with an execution order.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Call parameters (ftype_code, felem_code, exec_order, sfunc_code)
///
/// # Returns
/// Tuple of (modified_config, new_sfcall_record)
///
/// # Errors
/// - `InvalidParameter` if both ftype_code and felem_code are specified or both missing
/// - `Duplicate` if exec_order is already taken for the feature/element
/// - `NotFound` if function/feature/element codes don't exist
pub fn add_standardize_call(
    config: &str,
    params: AddStandardizeCallParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Get next SFCALL_ID (seed at 1000 for user-created calls)
    let sfcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_SFCALL", "SFCALL_ID", 1000)?;

    // Lookup function ID
    let sfunc_id = lookup_sfunc_id(config, params.sfunc_code)?;

    // Determine FTYPE_ID and FELEM_ID (-1 means not specified)
    let mut ftype_id: i64 = -1;
    let mut felem_id: i64 = -1;

    if let Some(feature) = params.ftype_code.filter(|f| !f.eq_ignore_ascii_case("ALL")) {
        ftype_id = lookup_feature_id(config, feature)?;
    }

    if let Some(element) = params.felem_code.filter(|e| !e.eq_ignore_ascii_case("N/A")) {
        felem_id = lookup_element_id(config, element)?;
    }

    // Validate: exactly one of (feature, element) must be specified
    if (ftype_id > 0 && felem_id > 0) || (ftype_id < 0 && felem_id < 0) {
        return Err(SzConfigError::InvalidInput(
            "Either a feature or an element must be specified, but not both".to_string(),
        ));
    }

    // Determine exec_order: use provided value or get next available for this feature/element
    let final_exec_order = if let Some(order) = params.exec_order {
        // Check if this exec_order is already taken for this feature/element
        let order_taken = config_data["G2_CONFIG"]["CFG_SFCALL"]
            .as_array()
            .map(|arr| {
                arr.iter().any(|call| {
                    call["FTYPE_ID"].as_i64() == Some(ftype_id)
                        && call["FELEM_ID"].as_i64() == Some(felem_id)
                        && call["EXEC_ORDER"].as_i64() == Some(order)
                })
            })
            .unwrap_or(false);

        if order_taken {
            return Err(SzConfigError::AlreadyExists(format!(
                "Execution order {} already taken for this feature/element",
                order
            )));
        }
        order
    } else {
        // Get next available exec_order for this feature/element combination
        config_data["G2_CONFIG"]["CFG_SFCALL"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter(|call| {
                        call["FTYPE_ID"].as_i64() == Some(ftype_id)
                            && call["FELEM_ID"].as_i64() == Some(felem_id)
                    })
                    .filter_map(|call| call["EXEC_ORDER"].as_i64())
                    .max()
                    .map(|max| max + 1)
                    .unwrap_or(1)
            })
            .unwrap_or(1)
    };

    // Create new CFG_SFCALL record
    let new_record = json!({
        "SFCALL_ID": sfcall_id,
        "FTYPE_ID": ftype_id,
        "FELEM_ID": felem_id,
        "SFUNC_ID": sfunc_id,
        "EXEC_ORDER": final_exec_order
    });

    // Add to config
    if let Some(sfcall_array) = config_data["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_SFCALL".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a standardize call by ID
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `sfcall_id` - Standardize call ID to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn delete_standardize_call(config: &str, sfcall_id: i64) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the call exists
    let call_exists = config_data["G2_CONFIG"]["CFG_SFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["SFCALL_ID"].as_i64() == Some(sfcall_id))
        })
        .unwrap_or(false);

    if !call_exists {
        return Err(SzConfigError::NotFound(format!(
            "Standardize call ID {}",
            sfcall_id
        )));
    }

    // Delete the standardize call
    if let Some(sfcall_array) = config_data["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.retain(|record| record["SFCALL_ID"].as_i64() != Some(sfcall_id));
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single standardize call by ID
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `sfcall_id` - Standardize call ID
///
/// # Returns
/// JSON Value representing the standardize call record
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn get_standardize_call(config: &str, sfcall_id: i64) -> Result<Value> {
    find_in_config_array(config, "CFG_SFCALL", "SFCALL_ID", &sfcall_id.to_string())?
        .ok_or_else(|| SzConfigError::NotFound(format!("Standardize call ID {}", sfcall_id)))
}

/// List all standardize calls with resolved names
///
/// Returns all standardize calls with feature, element, and function codes resolved.
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names (id, feature, element, execOrder, function)
pub fn list_standardize_calls(config: &str) -> Result<Vec<Value>> {
    let config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let empty_array = vec![];
    let sfcall_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFCALL"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let ftype_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let felem_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FELEM"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let sfunc_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFUNC"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    // Helper functions for ID resolution
    let resolve_ftype = |ftype_id: i64| -> String {
        if ftype_id <= 0 {
            "all".to_string()
        } else {
            ftype_array
                .iter()
                .find(|ft| ft.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id))
                .and_then(|ft| ft.get("FTYPE_CODE"))
                .and_then(|v| v.as_str())
                .unwrap_or("all")
                .to_string()
        }
    };

    let resolve_felem = |felem_id: i64| -> String {
        if felem_id <= 0 {
            "n/a".to_string()
        } else {
            felem_array
                .iter()
                .find(|fe| fe.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id))
                .and_then(|fe| fe.get("FELEM_CODE"))
                .and_then(|v| v.as_str())
                .unwrap_or("n/a")
                .to_string()
        }
    };

    let resolve_sfunc = |sfunc_id: i64| -> String {
        sfunc_array
            .iter()
            .find(|sf| sf.get("SFUNC_ID").and_then(|v| v.as_i64()) == Some(sfunc_id))
            .and_then(|sf| sf.get("SFUNC_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Transform standardize calls
    let items: Vec<Value> = sfcall_array
        .iter()
        .map(|item| {
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let felem_id = item.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let sfunc_id = item.get("SFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            json!({
                "id": item.get("SFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "feature": resolve_ftype(ftype_id),
                "element": resolve_felem(felem_id),
                "execOrder": item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0),
                "function": resolve_sfunc(sfunc_id)
            })
        })
        .collect();

    Ok(items)
}

/// Update a standardize call (stub - not implemented in Python)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Standardize call parameters (sfcall_id required, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_standardize_call(config: &str, _params: SetStandardizeCallParams) -> Result<String> {
    // This is a stub - the Python version doesn't implement this
    Ok(config.to_string())
}

/// Add a standardize call element (SBOM record)
///
/// Creates a new standardize bill of materials entry.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters (ftype_id, sfunc_id, felem_id, exec_order)
///
/// # Returns
/// Tuple of (modified_config, new_sbom_record)
pub fn add_standardize_call_element(
    config: &str,
    params: AddStandardizeCallElementParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let final_felem_id = params.felem_id.unwrap_or(-1);

    // Check if call element already exists
    if let Some(sfcall_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFCALL"))
        .and_then(|v| v.as_array())
    {
        for item in sfcall_array {
            if item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("SFUNC_ID").and_then(|v| v.as_i64()) == Some(params.sfunc_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(final_felem_id)
            {
                return Err(SzConfigError::AlreadyExists(
                    "Standardize call element already exists".to_string(),
                ));
            }
        }
    }

    // Get next SFCALL_ID
    let sfcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_SFCALL", "SFCALL_ID", 1000)?;

    // Create new call element record
    let mut new_record = json!({
        "SFCALL_ID": sfcall_id,
        "FTYPE_ID": params.ftype_id,
        "FELEM_ID": final_felem_id,
        "SFUNC_ID": params.sfunc_id,
    });

    // Add optional exec_order if provided
    if let Some(order) = params.exec_order {
        if let Some(obj) = new_record.as_object_mut() {
            obj.insert("EXEC_ORDER".to_string(), json!(order));
        }
    }

    // Add to CFG_SFCALL
    if let Some(sfcall_array) = config_data["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_SFCALL".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a standardize call element
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters (ftype_id, sfunc_id, felem_id)
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_standardize_call_element(
    config: &str,
    params: DeleteStandardizeCallElementParams,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let final_felem_id = params.felem_id.unwrap_or(-1);

    // Validate that the element exists
    let element_exists = config_data["G2_CONFIG"]["CFG_SFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter().any(|item| {
                item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                    && item.get("SFUNC_ID").and_then(|v| v.as_i64()) == Some(params.sfunc_id)
                    && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(final_felem_id)
            })
        })
        .unwrap_or(false);

    if !element_exists {
        return Err(SzConfigError::NotFound(
            "Standardize call element not found".to_string(),
        ));
    }

    // Delete the element
    if let Some(sfcall_array) = config_data["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.retain(|item| {
            !(item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("SFUNC_ID").and_then(|v| v.as_i64()) == Some(params.sfunc_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(final_felem_id))
        });
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Update a standardize call element (stub - not typically used)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters including updates
///
/// # Returns
/// Modified configuration JSON string
pub fn set_standardize_call_element(
    config: &str,
    _params: SetStandardizeCallElementParams,
) -> Result<String> {
    // This is a stub - not commonly used
    Ok(config.to_string())
}
