//! Distinct call management operations
//!
//! Functions for managing CFG_DFCALL (distinct calls) and CFG_DFBOM
//! (distinct bill of materials) configuration sections.

use crate::calls::{CallSelector, derive_bom_exec_order, ensure_call_exists, resolve_call_id};
use crate::config_rows::{DfbomRow, DfcallRow};
use crate::error::{Result, SzConfigError};
use crate::helpers::{
    get_next_id, lookup_dfunc_id, lookup_element_id, lookup_feature_id,
    resolve_dfcall_id_for_feature,
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
    /// Execution order, allocated per `DFCALL_ID` (see the "Execution-order
    /// policy" in [`crate::calls`]). `None` auto-allocates the next order on the
    /// call; `Some(n > 0)` requests that exact order and fails with
    /// `AlreadyExists` if already taken. The written BOM row always carries a
    /// concrete order.
    pub exec_order: Option<i64>,
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

    // Validate element list is not empty
    if params.element_list.is_empty() {
        return Err(SzConfigError::InvalidInput(
            "No elements were found in the elementList".to_string(),
        ));
    }

    // Validate each element is not blank
    for (idx, element_code) in params.element_list.iter().enumerate() {
        if element_code.trim().is_empty() {
            return Err(SzConfigError::InvalidInput(format!(
                "Element cannot be blank in item {} on the element list",
                idx + 1
            )));
        }
    }

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
        return Err(SzConfigError::AlreadyPresent(format!(
            "Distinct call for feature {} already set",
            params.ftype_code
        )));
    }

    // Lookup function ID
    let dfunc_id = lookup_dfunc_id(config, &params.dfunc_code)?;

    // Process element list and create DFBOM records
    let mut dfbom_records = Vec::new();

    for (idx, element_code) in params.element_list.iter().enumerate() {
        // Validate element is not blank (already checked in add_distinct_call, defensive)
        if element_code.trim().is_empty() {
            return Err(SzConfigError::InvalidInput(format!(
                "Element cannot be blank in item {} on the element list",
                idx + 1
            )));
        }

        // Lookup element ID (global lookup - Python allows any element in call)
        let bom_felem_id = lookup_element_id(config, element_code)?;

        // Create DFBOM record via DfbomRow so every key is always present.
        // EXEC_ORDER is 1-based over the element list.
        let bom_row = DfbomRow {
            dfcall_id,
            ftype_id,
            felem_id: bom_felem_id,
            exec_order: idx as i64 + 1,
        };
        dfbom_records.push(serde_json::to_value(&bom_row)?);
    }

    // Create new CFG_DFCALL record via DfcallRow. CFG_DFCALL is exactly
    // DFCALL_ID, FTYPE_ID, DFUNC_ID per the authoritative Senzing v4 schema;
    // FELEM_ID and EXEC_ORDER live on the CFG_DFBOM rows built above.
    let dfcall_row = DfcallRow {
        dfcall_id,
        ftype_id,
        dfunc_id,
    };
    let new_record = serde_json::to_value(&dfcall_row)?;

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
            "Distinct call ID {dfcall_id} does not exist"
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

/// Get a single distinct call, addressed by id or by feature code.
///
/// Pass [`CallSelector::Id`] to look the call up by its `DFCALL_ID`, or
/// [`CallSelector::Feature`] to resolve the (0-or-1) distinct call bound to a
/// feature. The feature path scans `CFG_DFCALL` by `FTYPE_ID` via
/// [`resolve_dfcall_id_for_feature`] rather than treating the feature id as a
/// call id (the historical bug this fixes).
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `selector` - Call id or feature code identifying the call
///
/// # Returns
/// JSON Value representing the distinct call record
///
/// # Errors
/// - `NotFound` if no matching call exists
/// - `InvalidInput` if a feature code matches more than one call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::distinct::get_distinct_call;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_DFCALL": [{"DFCALL_ID": 11, "FTYPE_ID": 3, "DFUNC_ID": 1}]
/// }}"#;
/// let by_id = get_distinct_call(config, CallSelector::Id(11)).unwrap();
/// let by_feature = get_distinct_call(config, CallSelector::Feature("NAME")).unwrap();
/// assert_eq!(by_id, by_feature);
/// ```
pub fn get_distinct_call(config: &str, selector: CallSelector) -> Result<Value> {
    let root: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;
    let dfcall_id = resolve_call_id(config, &root, selector, resolve_dfcall_id_for_feature)?;

    root.get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFCALL"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|c| c.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(dfcall_id))
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Distinct call ID {dfcall_id} does not exist"))
        })
}

/// List all distinct calls with resolved names
///
/// Returns all distinct calls with feature and function codes resolved, plus the
/// call's `elementList` (element codes assembled from `CFG_DFBOM`, ordered by
/// `EXEC_ORDER`).
///
/// The rows are sorted inside the SDK by `(FTYPE_ID, DFCALL_ID)` — the same key
/// Python uses — so callers never need to re-sort. `execOrder` is retained on the
/// distinct projection (the header row's `EXEC_ORDER`, always `1`).
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names, `execOrder`, and an `elementList`
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::distinct::list_distinct_calls;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
///     "CFG_DFUNC": [{"DFUNC_ID": 1, "DFUNC_CODE": "FELEM_EXP"}],
///     "CFG_DFCALL": [{"DFCALL_ID": 7, "FTYPE_ID": 3, "DFUNC_ID": 1, "EXEC_ORDER": 1}],
///     "CFG_DFBOM": [{"DFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}]
/// }}"#;
/// let calls = list_distinct_calls(config).unwrap();
/// assert_eq!(calls[0]["elementList"], serde_json::json!(["FIRST_NAME"]));
/// ```
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

    let felem_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FELEM"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let dfbom_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFBOM"))
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

    let resolve_felem = |felem_id: i64| -> String {
        felem_array
            .iter()
            .find(|fe| fe.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id))
            .and_then(|fe| fe.get("FELEM_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Assemble a call's elementList from CFG_DFBOM, ordered by EXEC_ORDER.
    let element_list = |dfcall_id: i64| -> Vec<Value> {
        let mut rows: Vec<&Value> = dfbom_array
            .iter()
            .filter(|bom| bom.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(dfcall_id))
            .collect();
        rows.sort_by_key(|bom| bom.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0));
        rows.into_iter()
            .map(|bom| {
                let felem_id = bom.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
                Value::from(resolve_felem(felem_id))
            })
            .collect()
    };

    // Sort the raw rows by (FTYPE_ID, DFCALL_ID) before projection so the numeric
    // sort key is never lost (mirrors Python).
    let mut sorted: Vec<&Value> = dfcall_array.iter().collect();
    sorted.sort_by_key(|item| {
        (
            item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
            item.get("DFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
        )
    });

    // Transform distinct calls
    let items: Vec<Value> = sorted
        .into_iter()
        .map(|item| {
            let dfcall_id = item.get("DFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let dfunc_id = item.get("DFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            json!({
                "id": dfcall_id,
                "feature": resolve_ftype(ftype_id),
                "function": resolve_dfunc(dfunc_id),
                "execOrder": item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(1),
                "elementList": element_list(dfcall_id)
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

    // Check if element already exists. Dup identity is (DFCALL_ID, FTYPE_ID,
    // FELEM_ID) — EXEC_ORDER is NOT part of it (realigned to match the
    // comparison/expression siblings and Python addCallElement, which treats the
    // same element on a call as a duplicate regardless of position).
    if let Some(dbom_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFBOM"))
        .and_then(|v| v.as_array())
    {
        for item in dbom_array {
            if item.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(params.dfcall_id)
                && item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(params.ftype_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(params.felem_id)
            {
                return Err(SzConfigError::AlreadyPresent(
                    "Distinct call element already exists".to_string(),
                ));
            }
        }
    }

    // Allocate EXEC_ORDER per DFCALL_ID (the scope Python addCallElement numbers
    // BOM execution order within). `None` auto-allocates; a supplied order is
    // honoured or rejected if taken. Never left null.
    let exec_order = {
        let empty: Vec<Value> = Vec::new();
        let dfbom_rows = config_data["G2_CONFIG"]["CFG_DFBOM"]
            .as_array()
            .unwrap_or(&empty);
        crate::helpers::get_desired_or_next_order(
            dfbom_rows,
            "EXEC_ORDER",
            &[("DFCALL_ID", params.dfcall_id)],
            params.exec_order,
        )?
    };

    // Create new DBOM record via DfbomRow so every key is always present.
    let row = DfbomRow {
        dfcall_id: params.dfcall_id,
        ftype_id: params.ftype_id,
        felem_id: params.felem_id,
        exec_order,
    };
    let new_record = serde_json::to_value(&row)?;

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

/// Delete a distinct call element, addressed by (call | feature) + element code.
///
/// The caller no longer supplies `EXEC_ORDER`: the target `CFG_DFBOM` row is
/// located by its call id and the element's `FELEM_ID`, and its execution order
/// is derived from that row. A DFBOM record stores the *element's* feature id in
/// `FTYPE_ID`, so that column is not part of the address.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `call` - Call id or feature code identifying the distinct call
/// * `element_code` - Element code to remove from the call
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if the call id does not exist, or the call/element codes don't resolve
/// - `NotOnCall` if the call exists but the element is not one of its BOM rows
/// - `InvalidInput` if a feature code matches more than one call, or the element
///   matches more than one BOM row on the call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::distinct::delete_distinct_call_element;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
///     "CFG_DFCALL": [{"DFCALL_ID": 11, "FTYPE_ID": 3, "DFUNC_ID": 1}],
///     "CFG_DFBOM": [{"DFCALL_ID": 11, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}]
/// }}"#;
/// let out = delete_distinct_call_element(
///     config, CallSelector::Id(11), "FIRST_NAME", None).unwrap();
/// let v: serde_json::Value = serde_json::from_str(&out).unwrap();
/// assert!(v["G2_CONFIG"]["CFG_DFBOM"].as_array().unwrap().is_empty());
/// ```
pub fn delete_distinct_call_element(
    config: &str,
    call: CallSelector,
    element_code: &str,
    element_feature: Option<&str>,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let dfcall_id = resolve_call_id(config, &config_data, call, resolve_dfcall_id_for_feature)?;
    // A non-existent call id is a hard NotFound (Python parity); only a missing
    // element on an *existing* call is the benign NotOnCall from derive below.
    ensure_call_exists(
        &config_data,
        "CFG_DFCALL",
        "DFCALL_ID",
        dfcall_id,
        "Distinct",
    )?;
    let felem_id = lookup_element_id(config, element_code)?;
    // When the element appears under multiple features in this call, the
    // element's feature disambiguates to the correct BOM row (Python parity).
    let element_ftype_id = match element_feature {
        Some(f) => Some(lookup_feature_id(config, f)?),
        None => None,
    };

    // Derive EXEC_ORDER from the located BOM row (also validates existence).
    let exec_order = derive_bom_exec_order(
        &config_data,
        "CFG_DFBOM",
        "DFCALL_ID",
        dfcall_id,
        felem_id,
        element_ftype_id,
        "Distinct",
    )?;

    if let Some(dbom_array) = config_data["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        // Mirror the derive predicate: when a feature disambiguated the target
        // row, constrain the retain by FTYPE too. Otherwise a sibling row sharing
        // (call, felem, exec) under another feature would be over-deleted.
        dbom_array.retain(|item| {
            !(item.get("DFCALL_ID").and_then(|v| v.as_i64()) == Some(dfcall_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(exec_order)
                && element_ftype_id
                    .is_none_or(|ft| item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ft)))
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

#[cfg(test)]
mod tests {
    use super::*;

    // CFG_DFCALL is exactly DFCALL_ID, FTYPE_ID, DFUNC_ID per the Senzing v4
    // schema; FELEM_ID and EXEC_ORDER belong to CFG_DFBOM.
    const DFCALL_KEYS: [&str; 3] = ["DFCALL_ID", "FTYPE_ID", "DFUNC_ID"];
    const DFBOM_KEYS: [&str; 4] = ["DFCALL_ID", "FTYPE_ID", "FELEM_ID", "EXEC_ORDER"];

    fn assert_all_keys(obj: &Value, keys: &[&str]) {
        let map = obj.as_object().unwrap();
        for key in keys {
            assert!(map.contains_key(*key), "{key} key must be present");
        }
    }

    fn base_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_DFCALL": [],
            "CFG_DFBOM": [],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_DFUNC": [{"DFUNC_ID": 7, "DFUNC_CODE": "FELEM_EXP"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}]
        }}"#
        .to_string()
    }

    #[test]
    fn test_add_distinct_call_emits_all_keys() {
        let config = base_config();
        let params = AddDistinctCallParams {
            ftype_code: "NAME".to_string(),
            dfunc_code: "FELEM_EXP".to_string(),
            element_list: vec!["FIRST_NAME".to_string()],
        };

        let (modified, new_record) = add_distinct_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();

        // CFG_DFCALL row: exactly 3 keys (no FELEM_ID, no EXEC_ORDER).
        let dfcall = &value["G2_CONFIG"]["CFG_DFCALL"][0];
        assert_all_keys(dfcall, &DFCALL_KEYS);
        assert_eq!(
            dfcall.as_object().unwrap().len(),
            3,
            "DFCALL is exactly 3 columns"
        );
        assert!(!dfcall.as_object().unwrap().contains_key("FELEM_ID"));
        assert!(!dfcall.as_object().unwrap().contains_key("EXEC_ORDER"));
        assert_eq!(dfcall["FTYPE_ID"], json!(5));
        assert_eq!(dfcall["DFUNC_ID"], json!(7));
        assert_all_keys(&new_record, &DFCALL_KEYS);

        // CFG_DFBOM row: all 4 keys.
        let dfbom = &value["G2_CONFIG"]["CFG_DFBOM"][0];
        assert_all_keys(dfbom, &DFBOM_KEYS);
        assert_eq!(dfbom["FELEM_ID"], json!(11));
        assert_eq!(dfbom["EXEC_ORDER"], json!(1));
    }

    #[test]
    fn test_add_distinct_call_element_emits_all_keys() {
        let config = base_config();
        // A specific free order is honoured (exec_order now Option).
        let params = AddDistinctCallElementParams {
            dfcall_id: 1000,
            ftype_id: 5,
            felem_id: 11,
            exec_order: Some(3),
        };

        let (modified, new_record) = add_distinct_call_element(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let dfbom = &value["G2_CONFIG"]["CFG_DFBOM"][0];

        assert_all_keys(dfbom, &DFBOM_KEYS);
        assert_all_keys(&new_record, &DFBOM_KEYS);
        assert_eq!(dfbom["EXEC_ORDER"], json!(3));
    }

    #[test]
    fn test_add_distinct_call_element_auto_alloc_and_dup_ignores_exec_order() {
        // Call 1000 carries one BOM row at order 1. Auto-alloc gives 2, and the
        // dup check on (DFCALL_ID, FTYPE_ID, FELEM_ID) rejects the same element
        // even when a DIFFERENT exec_order is requested.
        let config = r#"{"G2_CONFIG": {
            "CFG_DFBOM": [
                {"DFCALL_ID": 1000, "FTYPE_ID": 5, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ]
        }}"#;

        // New element -> next order 2.
        let (modified, _) = add_distinct_call_element(
            config,
            AddDistinctCallElementParams {
                dfcall_id: 1000,
                ftype_id: 5,
                felem_id: 12,
                exec_order: None,
            },
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let new_row = value["G2_CONFIG"]["CFG_DFBOM"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["FELEM_ID"].as_i64() == Some(12))
            .unwrap();
        assert_eq!(new_row["EXEC_ORDER"], json!(2));

        // Same (call, ftype, felem) but a different exec_order -> still a dup.
        let err = add_distinct_call_element(
            config,
            AddDistinctCallElementParams {
                dfcall_id: 1000,
                ftype_id: 5,
                felem_id: 11,
                exec_order: Some(99),
            },
        )
        .unwrap_err();
        // A duplicate element on the call is the benign "already present"
        // sub-case (step D), distinct from a taken exec-order (AlreadyExists).
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyPresent);
    }

    // #40 fixtures: DFCALL_ID (11) deliberately differs from FTYPE_ID (3). The
    // historical bug used the feature id directly as the call id; the by-feature
    // path must instead scan CFG_DFCALL and return DFCALL_ID 11.
    fn populated_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_DFCALL": [{"DFCALL_ID": 11, "FTYPE_ID": 3, "DFUNC_ID": 1}],
            "CFG_DFBOM": [
                {"DFCALL_ID": 11, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1},
                {"DFCALL_ID": 11, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2}
            ]
        }}"#
        .to_string()
    }

    #[test]
    fn test_get_distinct_call_by_feature_returns_correct_call() {
        let config = populated_config();
        let by_id = get_distinct_call(&config, CallSelector::Id(11)).unwrap();
        let by_feature = get_distinct_call(&config, CallSelector::Feature("NAME")).unwrap();
        assert_eq!(by_id, by_feature);
        // Regression: the returned call id is the real DFCALL_ID (11), NOT the
        // feature id (3) that the old FTYPE_ID-as-call-id bug would have used.
        assert_eq!(by_feature["DFCALL_ID"], json!(11));
    }

    #[test]
    fn test_delete_distinct_call_element_derives_exec_order() {
        let config = populated_config();
        let out =
            delete_distinct_call_element(&config, CallSelector::Id(11), "LAST_NAME", None).unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        let dfbom = v["G2_CONFIG"]["CFG_DFBOM"].as_array().unwrap();
        assert_eq!(dfbom.len(), 1);
        assert_eq!(dfbom[0]["FELEM_ID"], json!(11));
    }

    #[test]
    fn test_list_distinct_calls_sorted_element_list_and_exec_order() {
        // Stored order differs from (FTYPE_ID, DFCALL_ID); BOM stored out of order.
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 3, "FTYPE_CODE": "NAME"},
                {"FTYPE_ID": 7, "FTYPE_CODE": "ADDRESS"}
            ],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_DFUNC": [{"DFUNC_ID": 1, "DFUNC_CODE": "FELEM_EXP"}],
            "CFG_DFCALL": [
                {"DFCALL_ID": 20, "FTYPE_ID": 7, "DFUNC_ID": 1, "EXEC_ORDER": 1},
                {"DFCALL_ID": 5, "FTYPE_ID": 3, "DFUNC_ID": 1, "EXEC_ORDER": 1}
            ],
            "CFG_DFBOM": [
                {"DFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2},
                {"DFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ]
        }}"#;

        let calls = list_distinct_calls(config).unwrap();
        assert_eq!(calls[0]["feature"], json!("NAME"));
        assert_eq!(calls[0]["id"], json!(5));
        // execOrder ruling retained (D28).
        assert_eq!(calls[0]["execOrder"], json!(1));
        // elementList ordered by EXEC_ORDER despite reversed storage.
        assert_eq!(calls[0]["elementList"], json!(["FIRST_NAME", "LAST_NAME"]));
        assert_eq!(calls[1]["feature"], json!("ADDRESS"));
    }
}
