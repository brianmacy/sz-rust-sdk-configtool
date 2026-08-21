//! Comparison call management operations
//!
//! Functions for managing CFG_CFCALL (comparison calls) and CFG_CFBOM
//! (comparison bill of materials) configuration sections.

use crate::config_rows::{CfbomRow, CfcallRow};
use crate::error::{Result, SzConfigError};
use crate::helpers::{
    find_in_config_array, get_next_id, lookup_cfunc_id, lookup_element_id, lookup_feature_id,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison call
#[derive(Debug, Clone)]
pub struct AddComparisonCallParams {
    pub ftype_code: String,
    pub cfunc_code: String,
    pub element_list: Vec<String>,
}

impl TryFrom<&Value> for AddComparisonCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        Ok(Self {
            ftype_code: json
                .get("ftypeCode")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("ftypeCode".to_string()))?
                .to_string(),
            cfunc_code: json
                .get("cfuncCode")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("cfuncCode".to_string()))?
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

/// Parameters for adding a comparison call element (CBOM record)
#[derive(Debug, Clone)]
pub struct AddComparisonCallElementParams {
    pub cfcall_id: i64,
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
}

impl TryFrom<&Value> for AddComparisonCallElementParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        Ok(Self {
            cfcall_id: json
                .get("cfcallId")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| SzConfigError::MissingField("cfcallId".to_string()))?,
            ftype_id: json
                .get("ftypeId")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| SzConfigError::MissingField("ftypeId".to_string()))?,
            felem_id: json
                .get("felemId")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| SzConfigError::MissingField("felemId".to_string()))?,
            exec_order: json
                .get("execOrder")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| SzConfigError::MissingField("execOrder".to_string()))?,
        })
    }
}

/// Parameters for deleting a comparison call element
#[derive(Debug, Clone)]
pub struct DeleteComparisonCallElementParams {
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
}

/// Parameters for setting (updating) a comparison call
#[derive(Debug, Clone, Default)]
pub struct SetComparisonCallParams {
    pub cfcall_id: i64,
    pub exec_order: Option<i64>,
}

impl TryFrom<&Value> for SetComparisonCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let cfcall_id = json
            .get("cfcallId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("cfcallId".to_string()))?;

        Ok(Self {
            cfcall_id,
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for setting a comparison call element
#[derive(Debug, Clone)]
pub struct SetComparisonCallElementParams {
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
    pub updates: Value,
}

/// Add a new comparison call with element list
///
/// Creates a new comparison call linking a function to a feature
/// with associated elements (CBOM records).
/// Note: Only one comparison call is allowed per feature.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Comparison call parameters (ftype_code, cfunc_code, element_list required)
///
/// # Returns
/// Tuple of (modified_config, new_cfcall_record)
///
/// # Errors
/// - `Duplicate` if a comparison call already exists for this feature
/// - `NotFound` if function/feature/element codes don't exist
pub fn add_comparison_call(
    config: &str,
    params: AddComparisonCallParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Get next CFCALL_ID (seed at 1000 for user-created calls)
    let cfcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_CFCALL", "CFCALL_ID", 1000)?;

    // Lookup feature ID
    let ftype_id = lookup_feature_id(config, &params.ftype_code)?;

    // Check if comparison call already exists for this feature (only one allowed per feature)
    let call_exists = config_data["G2_CONFIG"]["CFG_CFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
        })
        .unwrap_or(false);

    if call_exists {
        return Err(SzConfigError::AlreadyExists(format!(
            "Comparison call for feature {} already set",
            params.ftype_code
        )));
    }

    // Lookup function ID
    let cfunc_id = lookup_cfunc_id(config, &params.cfunc_code)?;

    // Validate element list is not empty
    if params.element_list.is_empty() {
        return Err(SzConfigError::InvalidInput(
            "No elements were found in the elementList".to_string(),
        ));
    }

    // Process element list and create CFBOM records
    let mut cfbom_records = Vec::new();

    for (idx, element_code) in params.element_list.iter().enumerate() {
        // Validate element is not blank
        if element_code.trim().is_empty() {
            return Err(SzConfigError::InvalidInput(format!(
                "Element cannot be blank in item {} on the element list",
                idx + 1
            )));
        }

        // Lookup element ID (global lookup - Python allows any element in call)
        let bom_felem_id = lookup_element_id(config, element_code)?;

        // Create CFBOM record via CfbomRow so every key is always present.
        // EXEC_ORDER is 1-based over the element list.
        let bom_row = CfbomRow {
            cfcall_id,
            ftype_id,
            felem_id: bom_felem_id,
            exec_order: idx as i64 + 1,
        };
        cfbom_records.push(serde_json::to_value(&bom_row)?);
    }

    // Create new CFG_CFCALL record via CfcallRow so every key is always present.
    let cfcall_row = CfcallRow {
        cfcall_id,
        ftype_id,
        cfunc_id,
    };
    let new_record = serde_json::to_value(&cfcall_row)?;

    // Add to config
    if let Some(cfcall_array) = config_data["G2_CONFIG"]["CFG_CFCALL"].as_array_mut() {
        cfcall_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_CFCALL".to_string()));
    }

    if let Some(cfbom_array) = config_data["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cfbom_array.extend(cfbom_records);
    } else {
        return Err(SzConfigError::MissingSection("CFG_CFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a comparison call by ID
///
/// Also deletes associated CFBOM records.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `cfcall_id` - Comparison call ID to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn delete_comparison_call(config: &str, cfcall_id: i64) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the call exists
    let call_exists = config_data["G2_CONFIG"]["CFG_CFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .any(|call| call["CFCALL_ID"].as_i64() == Some(cfcall_id))
        })
        .unwrap_or(false);

    if !call_exists {
        return Err(SzConfigError::NotFound(format!(
            "Comparison call ID {cfcall_id} does not exist"
        )));
    }

    // Delete the comparison call
    if let Some(cfcall_array) = config_data["G2_CONFIG"]["CFG_CFCALL"].as_array_mut() {
        cfcall_array.retain(|record| record["CFCALL_ID"].as_i64() != Some(cfcall_id));
    }

    // Delete associated CFBOM records
    if let Some(cfbom_array) = config_data["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cfbom_array.retain(|record| record["CFCALL_ID"].as_i64() != Some(cfcall_id));
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single comparison call by ID
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `cfcall_id` - Comparison call ID
///
/// # Returns
/// JSON Value representing the comparison call record
///
/// # Errors
/// - `NotFound` if call ID doesn't exist
pub fn get_comparison_call(config: &str, cfcall_id: i64) -> Result<Value> {
    find_in_config_array(config, "CFG_CFCALL", "CFCALL_ID", &cfcall_id.to_string())?
        .ok_or_else(|| SzConfigError::NotFound(format!("Comparison call ID {cfcall_id}")))
}

/// List all comparison calls with resolved names
///
/// Returns all comparison calls with feature and function codes resolved.
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names
pub fn list_comparison_calls(config: &str) -> Result<Vec<Value>> {
    let config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let empty_array = vec![];
    let cfcall_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFCALL"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let ftype_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let cfunc_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFUNC"))
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

    let resolve_cfunc = |cfunc_id: i64| -> String {
        cfunc_array
            .iter()
            .find(|cf| cf.get("CFUNC_ID").and_then(|v| v.as_i64()) == Some(cfunc_id))
            .and_then(|cf| cf.get("CFUNC_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Transform comparison calls
    let items: Vec<Value> = cfcall_array
        .iter()
        .map(|item| {
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let cfunc_id = item.get("CFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            json!({
                "id": item.get("CFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "feature": resolve_ftype(ftype_id),
                "function": resolve_cfunc(cfunc_id)
            })
        })
        .collect();

    Ok(items)
}

/// Update a comparison call (stub - not implemented in Python)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Comparison call parameters (cfcall_id required, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_comparison_call(config: &str, _params: SetComparisonCallParams) -> Result<String> {
    // This is a stub - the Python version doesn't implement this
    Ok(config.to_string())
}

/// Add a comparison call element (CBOM record)
///
/// Creates a new comparison bill of materials entry.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Element parameters (cfcall_id, ftype_id, felem_id, exec_order required)
///
/// # Returns
/// Tuple of (modified_config, new_cbom_record)
pub fn add_comparison_call_element(
    config: &str,
    params: AddComparisonCallElementParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate ftype_id is a valid feature ID (not -1 sentinel value)
    if params.ftype_id < 0 {
        return Err(SzConfigError::InvalidInput(format!(
            "{} is not a valid feature ID",
            params.ftype_id
        )));
    }

    // Check if element already exists
    // Python duplicate check (line 2941): checks [call_id_field, "FTYPE_ID", "FELEM_ID"] - 3 fields only
    // EXEC_ORDER excluded from check because same element at different positions is still a duplicate
    if let Some(cbom_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFBOM"))
        .and_then(|v| v.as_array())
    {
        for item in cbom_array {
            if item.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(params.cfcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
            // && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order)
            // ↑ Commented out: Python excludes EXEC_ORDER from duplicate check (sz_configtool line 2941)
            // Reason: Same element in same call is duplicate regardless of position/order
            {
                return Err(SzConfigError::AlreadyExists(
                    "Feature/element already exists for call".to_string(),
                ));
            }
        }
    }

    // Create new CBOM record via CfbomRow (use params.exec_order directly - no
    // auto-assignment) so every key is always present.
    let row = CfbomRow {
        cfcall_id: params.cfcall_id,
        ftype_id: params.ftype_id,
        felem_id: params.felem_id,
        exec_order: params.exec_order,
    };
    let new_record = serde_json::to_value(&row)?;

    // Add to CFG_CFBOM
    if let Some(cbom_array) = config_data["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cbom_array.push(new_record.clone());
    } else {
        return Err(SzConfigError::MissingSection("CFG_CFBOM".to_string()));
    }

    let modified_config =
        serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    Ok((modified_config, new_record))
}

/// Delete a comparison call element
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `cfcall_id` - Comparison call ID
/// * `params` - Element parameters (ftype_id, felem_id, exec_order)
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_comparison_call_element(
    config: &str,
    cfcall_id: i64,
    params: DeleteComparisonCallElementParams,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate that the element exists
    let element_exists = config_data["G2_CONFIG"]["CFG_CFBOM"]
        .as_array()
        .map(|arr| {
            arr.iter().any(|item| {
                item.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(cfcall_id)
                    && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                    && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
                    && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order)
            })
        })
        .unwrap_or(false);

    if !element_exists {
        return Err(SzConfigError::NotFound(
            "Comparison call element not found".to_string(),
        ));
    }

    // Delete the element
    if let Some(cbom_array) = config_data["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cbom_array.retain(|item| {
            !(item.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(cfcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(params.exec_order))
        });
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Update a comparison call element (stub - not typically used)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `cfcall_id` - Comparison call ID
/// * `params` - Element parameters including updates
///
/// # Returns
/// Modified configuration JSON string
pub fn set_comparison_call_element(
    config: &str,
    _cfcall_id: i64,
    _params: SetComparisonCallElementParams,
) -> Result<String> {
    // This is a stub - not commonly used
    Ok(config.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CFCALL_KEYS: [&str; 3] = ["CFCALL_ID", "FTYPE_ID", "CFUNC_ID"];
    const CFBOM_KEYS: [&str; 4] = ["CFCALL_ID", "FTYPE_ID", "FELEM_ID", "EXEC_ORDER"];

    fn assert_all_keys(obj: &Value, keys: &[&str]) {
        let map = obj.as_object().unwrap();
        for key in keys {
            assert!(map.contains_key(*key), "{key} key must be present");
        }
    }

    fn base_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_CFCALL": [],
            "CFG_CFBOM": [],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_CFUNC": [{"CFUNC_ID": 7, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}]
        }}"#
        .to_string()
    }

    #[test]
    fn test_add_comparison_call_emits_all_keys() {
        let config = base_config();
        let params = AddComparisonCallParams {
            ftype_code: "NAME".to_string(),
            cfunc_code: "GNR_COMP".to_string(),
            element_list: vec!["FIRST_NAME".to_string()],
        };

        let (modified, new_record) = add_comparison_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();

        // CFG_CFCALL row: exactly the 3 keys, no EXEC_ORDER (flagged discrepancy).
        let cfcall = &value["G2_CONFIG"]["CFG_CFCALL"][0];
        assert_all_keys(cfcall, &CFCALL_KEYS);
        assert!(!cfcall.as_object().unwrap().contains_key("EXEC_ORDER"));
        assert_eq!(cfcall["FTYPE_ID"], json!(5));
        assert_eq!(cfcall["CFUNC_ID"], json!(7));
        assert_all_keys(&new_record, &CFCALL_KEYS);

        // CFG_CFBOM row: all 4 keys.
        let cfbom = &value["G2_CONFIG"]["CFG_CFBOM"][0];
        assert_all_keys(cfbom, &CFBOM_KEYS);
        assert_eq!(cfbom["FELEM_ID"], json!(11));
        assert_eq!(cfbom["EXEC_ORDER"], json!(1));
    }

    #[test]
    fn test_add_comparison_call_element_emits_all_keys() {
        let config = base_config();
        let params = AddComparisonCallElementParams {
            cfcall_id: 1000,
            ftype_id: 5,
            felem_id: 11,
            exec_order: 2,
        };

        let (modified, new_record) = add_comparison_call_element(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let cfbom = &value["G2_CONFIG"]["CFG_CFBOM"][0];

        assert_all_keys(cfbom, &CFBOM_KEYS);
        assert_all_keys(&new_record, &CFBOM_KEYS);
        assert_eq!(cfbom["EXEC_ORDER"], json!(2));
    }
}
