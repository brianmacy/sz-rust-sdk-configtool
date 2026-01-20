//! Expression call management operations
//!
//! Functions for managing CFG_EFCALL (expression calls) and CFG_EFBOM
//! (expression bill of materials) configuration sections.

use crate::error::{Result, SzConfigError};
use crate::helpers::{
    find_in_config_array, get_next_id, lookup_efunc_id, lookup_element_id, lookup_feature_id,
};
use serde_json::{Value, json};

/// Add a new expression call with element list
///
/// Creates a new expression call linking a function to a feature or element
/// with an execution order and associated elements (EBOM records).
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `ftype_code` - Feature type code (use "ALL" for all features, or None)
/// * `felem_code` - Feature element code (use "N/A" for no element, or None)
/// * `exec_order` - Optional execution order (if None, next available is used)
/// * `efunc_code` - Expression function code
/// * `element_list` - Vector of (element_code, required, feature_code) tuples
/// * `expression_feature` - Optional expression feature code
/// * `is_virtual` - Virtual flag ("Yes", "No", "Any", "Desired")
///
/// # Returns
/// Tuple of (modified_config, new_efcall_record)
///
/// # Errors
/// - `InvalidParameter` if both ftype_code and felem_code are specified or both missing
/// - `Duplicate` if exec_order is already taken for the feature/element
/// - `NotFound` if function/feature/element codes don't exist
#[allow(clippy::too_many_arguments)]
pub fn add_expression_call(
    config: &str,
    ftype_code: Option<&str>,
    felem_code: Option<&str>,
    exec_order: Option<i64>,
    efunc_code: &str,
    element_list: Vec<(String, String, Option<String>)>, // (element, required, feature)
    expression_feature: Option<&str>,
    is_virtual: &str,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Get next EFCALL_ID (seed at 1000 for user-created calls)
    let efcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_EFCALL", "EFCALL_ID", 1000)?;

    // Lookup function ID
    let efunc_id = lookup_efunc_id(config, efunc_code)?;

    // Determine FTYPE_ID and FELEM_ID (-1 means not specified)
    let mut ftype_id: i64 = -1;
    let mut felem_id: i64 = -1;

    if let Some(feature) = ftype_code.filter(|f| !f.eq_ignore_ascii_case("ALL")) {
        ftype_id = lookup_feature_id(config, feature)?;
    }

    if let Some(element) = felem_code.filter(|e| !e.eq_ignore_ascii_case("N/A")) {
        felem_id = lookup_element_id(config, element)?;
    }

    // Validate: exactly one of (feature, element) must be specified
    if (ftype_id > 0 && felem_id > 0) || (ftype_id < 0 && felem_id < 0) {
        return Err(SzConfigError::InvalidInput(
            "Either a feature or an element must be specified, but not both".to_string(),
        ));
    }

    // Determine exec_order
    let final_exec_order = if let Some(order) = exec_order {
        // Check if this exec_order is already taken for this feature/element
        let order_taken = config_data["G2_CONFIG"]["CFG_EFCALL"]
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
        config_data["G2_CONFIG"]["CFG_EFCALL"]
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

    // Lookup expression feature ID if specified
    let efeat_ftype_id =
        if let Some(expr_feat) = expression_feature.filter(|f| !f.eq_ignore_ascii_case("N/A")) {
            lookup_feature_id(config, expr_feat)?
        } else {
            -1
        };

    // Process element list and create EFBOM records
    let mut efbom_records = Vec::new();
    let mut bom_exec_order = 0;

    for (element_code, required, feature_opt) in element_list {
        bom_exec_order += 1;

        // Determine BOM FTYPE_ID
        let bom_ftype_id =
            if let Some(bom_feature) = feature_opt.filter(|f| !f.eq_ignore_ascii_case("PARENT")) {
                if bom_feature.eq_ignore_ascii_case("parent") {
                    0 // Special value for parent feature link
                } else {
                    lookup_feature_id(config, &bom_feature)?
                }
            } else {
                -1
            };

        // Lookup element ID
        let bom_felem_id = if bom_ftype_id > 0 {
            // Lookup element within specific feature
            config_data["G2_CONFIG"]["CFG_FBOM"]
                .as_array()
                .and_then(|arr| {
                    arr.iter()
                        .find(|fbom| {
                            fbom["FTYPE_ID"].as_i64() == Some(bom_ftype_id)
                                && fbom["FELEM_CODE"]
                                    .as_str()
                                    .map(|s| s.eq_ignore_ascii_case(&element_code))
                                    .unwrap_or(false)
                        })
                        .and_then(|fbom| fbom["FELEM_ID"].as_i64())
                })
                .ok_or_else(|| {
                    SzConfigError::NotFound(format!(
                        "Element '{}' not found in feature",
                        element_code
                    ))
                })?
        } else {
            // Lookup element globally
            lookup_element_id(config, &element_code)?
        };

        // Create EFBOM record
        efbom_records.push(json!({
            "EFCALL_ID": efcall_id,
            "FTYPE_ID": bom_ftype_id,
            "FELEM_ID": bom_felem_id,
            "EXEC_ORDER": bom_exec_order,
            "FELEM_REQ": required
        }));
    }

    // Create new CFG_EFCALL record
    let new_record = json!({
        "EFCALL_ID": efcall_id,
        "FTYPE_ID": ftype_id,
        "FELEM_ID": felem_id,
        "EFUNC_ID": efunc_id,
        "EXEC_ORDER": final_exec_order,
        "EFEAT_FTYPE_ID": efeat_ftype_id,
        "IS_VIRTUAL": is_virtual
    });

    // Add to config
    if let Some(efcall_array) = config_data["G2_CONFIG"]["CFG_EFCALL"].as_array_mut() {
        efcall_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_EFCALL".to_string()));
    }

    if let Some(efbom_array) = config_data["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        efbom_array.extend(efbom_records);
    } else {
        return Err(SzConfigError::MissingSection("CFG_EFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete an expression call by ID
///
/// Also deletes associated EFBOM records.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn delete_expression_call(config: &str, efcall_id: i64) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the call exists
    let call_exists = config_data["G2_CONFIG"]["CFG_EFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["EFCALL_ID"].as_i64() == Some(efcall_id))
        })
        .unwrap_or(false);

    if !call_exists {
        return Err(SzConfigError::NotFound(format!(
            "Expression call ID {}",
            efcall_id
        )));
    }

    // Delete the expression call
    if let Some(efcall_array) = config_data["G2_CONFIG"]["CFG_EFCALL"].as_array_mut() {
        efcall_array.retain(|record| record["EFCALL_ID"].as_i64() != Some(efcall_id));
    }

    // Delete associated EFBOM records
    if let Some(efbom_array) = config_data["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        efbom_array.retain(|record| record["EFCALL_ID"].as_i64() != Some(efcall_id));
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single expression call by ID
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID
///
/// # Returns
/// JSON Value representing the expression call record
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn get_expression_call(config: &str, efcall_id: i64) -> Result<Value> {
    find_in_config_array(config, "CFG_EFCALL", "EFCALL_ID", &efcall_id.to_string())?
        .ok_or_else(|| SzConfigError::NotFound(format!("Expression call ID {}", efcall_id)))
}

/// List all expression calls with resolved names
///
/// Returns all expression calls with feature, element, and function codes resolved.
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names
pub fn list_expression_calls(config: &str) -> Result<Vec<Value>> {
    let config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let empty_array = vec![];
    let efcall_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFCALL"))
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

    let efunc_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFUNC"))
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

    let resolve_efunc = |efunc_id: i64| -> String {
        efunc_array
            .iter()
            .find(|ef| ef.get("EFUNC_ID").and_then(|v| v.as_i64()) == Some(efunc_id))
            .and_then(|ef| ef.get("EFUNC_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Transform expression calls
    let items: Vec<Value> = efcall_array
        .iter()
        .map(|item| {
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let felem_id = item.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let efunc_id = item.get("EFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            let efeat_ftype_id = item.get("EFEAT_FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(-1);

            json!({
                "id": item.get("EFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "feature": resolve_ftype(ftype_id),
                "element": resolve_felem(felem_id),
                "execOrder": item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0),
                "function": resolve_efunc(efunc_id),
                "isVirtual": item.get("IS_VIRTUAL").and_then(|v| v.as_str()).unwrap_or("No"),
                "expressionFeature": if efeat_ftype_id <= 0 { "n/a".to_string() } else { resolve_ftype(efeat_ftype_id) }
            })
        })
        .collect();

    Ok(items)
}

/// Update an expression call (stub - not implemented in Python)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID to update
/// * `updates` - JSON Value with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_expression_call(config: &str, _efcall_id: i64, _updates: Value) -> Result<String> {
    // This is a stub - the Python version doesn't implement this
    Ok(config.to_string())
}

/// Add an expression call element (EBOM record)
///
/// Creates a new expression bill of materials entry.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Feature element ID
/// * `exec_order` - Execution order
/// * `felem_req` - Element required flag ("Yes" or "No")
///
/// # Returns
/// Tuple of (modified_config, new_ebom_record)
pub fn add_expression_call_element(
    config: &str,
    efcall_id: i64,
    ftype_id: i64,
    felem_id: i64,
    exec_order: i64,
    felem_req: &str,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Check if element already exists
    if let Some(ebom_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFBOM"))
        .and_then(|v| v.as_array())
    {
        for item in ebom_array {
            if item.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(exec_order)
            {
                return Err(SzConfigError::AlreadyExists(
                    "Expression call element already exists".to_string(),
                ));
            }
        }
    }

    // Create new EBOM record
    let new_record = json!({
        "EFCALL_ID": efcall_id,
        "FTYPE_ID": ftype_id,
        "FELEM_ID": felem_id,
        "EXEC_ORDER": exec_order,
        "FELEM_REQ": felem_req
    });

    // Add to CFG_EFBOM
    if let Some(ebom_array) = config_data["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        ebom_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_EFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete an expression call element
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Feature element ID
/// * `exec_order` - Execution order
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_expression_call_element(
    config: &str,
    efcall_id: i64,
    ftype_id: i64,
    felem_id: i64,
    exec_order: i64,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the element exists
    let element_exists = config_data["G2_CONFIG"]["CFG_EFBOM"]
        .as_array()
        .map(|arr| {
            arr.iter().any(|item| {
                item.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id)
                    && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id)
                    && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
                    && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(exec_order)
            })
        })
        .unwrap_or(false);

    if !element_exists {
        return Err(SzConfigError::NotFound(
            "Expression call element not found".to_string(),
        ));
    }

    // Delete the element
    if let Some(ebom_array) = config_data["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        ebom_array.retain(|item| {
            !(item.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(exec_order))
        });
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Update an expression call element (stub - not typically used)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `efcall_id` - Expression call ID
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Feature element ID
/// * `exec_order` - Execution order
/// * `updates` - JSON Value with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_expression_call_element(
    config: &str,
    _efcall_id: i64,
    _ftype_id: i64,
    _felem_id: i64,
    _exec_order: i64,
    _updates: Value,
) -> Result<String> {
    // This is a stub - not commonly used
    Ok(config.to_string())
}
