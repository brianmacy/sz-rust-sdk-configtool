//! Comparison call management operations
//!
//! Functions for managing CFG_CFCALL (comparison calls) and CFG_CFBOM
//! (comparison bill of materials) configuration sections.

use crate::calls::{
    CallSelector, derive_bom_exec_order, ensure_call_exists, resolve_call_id,
    resolve_feature_element_id,
};
use crate::config_rows::{CfbomRow, CfcallRow};
use crate::error::{Result, SzConfigError};
use crate::helpers::{
    get_desired_or_next_id_from_section, lookup_cfunc_id, lookup_element_id, lookup_feature_id,
    resolve_cfcall_id_for_feature,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison call
///
/// `id` is a caller-supplied `CFCALL_ID`. Leave it `None` (or pass a
/// non-positive value) to auto-assign the next free id (seeded at the user-range
/// floor of 1000); pass `Some(id > 0)` to request that exact id —
/// [`add_comparison_call`] then fails with `AlreadyExists` if it is already taken.
#[derive(Debug, Clone, Default)]
pub struct AddComparisonCallParams {
    pub ftype_code: String,
    pub cfunc_code: String,
    pub element_list: Vec<String>,
    pub id: Option<i64>,
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
            id: json.get("id").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for adding a comparison call element (CBOM record)
#[derive(Debug, Clone)]
pub struct AddComparisonCallElementParams {
    pub cfcall_id: i64,
    pub ftype_id: i64,
    pub felem_id: i64,
    /// Execution order, allocated per `CFCALL_ID` (see the "Execution-order
    /// policy" in [`crate::calls`]). `None` auto-allocates the next order on the
    /// call; `Some(n > 0)` requests that exact order and fails with
    /// `AlreadyExists` if already taken. The written BOM row always carries a
    /// concrete order.
    pub exec_order: Option<i64>,
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
            // execOrder is optional: absent -> auto-allocate per call.
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
        })
    }
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

    // Get next CFCALL_ID (seed at 1000 for user-created calls). Caller-supplied
    // id (#37): None/non-positive auto-assigns; a specific id > 0 is honoured
    // unless already taken (returns AlreadyExists).
    let cfcall_id = get_desired_or_next_id_from_section(
        &config_data,
        "G2_CONFIG.CFG_CFCALL",
        "CFCALL_ID",
        params.id,
        1000,
    )?;

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
        return Err(SzConfigError::AlreadyPresent(format!(
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

/// Get a single comparison call, addressed by id or by feature code.
///
/// Pass [`CallSelector::Id`] to look the call up by its `CFCALL_ID`, or
/// [`CallSelector::Feature`] to resolve the (0-or-1) comparison call bound to a
/// feature. The feature path scans `CFG_CFCALL` by `FTYPE_ID` via
/// [`resolve_cfcall_id_for_feature`] rather than treating the feature id as a
/// call id.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `selector` - Call id or feature code identifying the call
///
/// # Returns
/// JSON Value representing the comparison call record
///
/// # Errors
/// - `NotFound` if no matching call exists
/// - `InvalidInput` if a feature code matches more than one call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::comparison::get_comparison_call;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}]
/// }}"#;
/// let by_id = get_comparison_call(config, CallSelector::Id(7)).unwrap();
/// let by_feature = get_comparison_call(config, CallSelector::Feature("NAME")).unwrap();
/// assert_eq!(by_id, by_feature);
/// ```
pub fn get_comparison_call(config: &str, selector: CallSelector) -> Result<Value> {
    let root: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;
    let cfcall_id = resolve_call_id(config, &root, selector, resolve_cfcall_id_for_feature)?;

    root.get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFCALL"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|c| c.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(cfcall_id))
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Comparison call ID {cfcall_id} does not exist"))
        })
}

/// List all comparison calls with resolved names
///
/// Returns all comparison calls with feature and function codes resolved, plus
/// the call's `elementList` (element codes assembled from `CFG_CFBOM`, ordered by
/// `EXEC_ORDER`).
///
/// The rows are sorted inside the SDK by `(FTYPE_ID, CFCALL_ID)` — the same key
/// Python uses — so callers never need to re-sort. The sort runs on the raw rows
/// before id→code projection, so the numeric key is never lost.
///
/// # Arguments
/// * `config` - Configuration JSON string
///
/// # Returns
/// Vector of JSON Values with resolved names and an `elementList`
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::comparison::list_comparison_calls;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
///     "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "GNR_COMP"}],
///     "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
///     "CFG_CFBOM": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}]
/// }}"#;
/// let calls = list_comparison_calls(config).unwrap();
/// assert_eq!(calls[0]["elementList"], serde_json::json!(["FIRST_NAME"]));
/// ```
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

    let felem_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FELEM"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty_array);

    let cfbom_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFBOM"))
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

    let resolve_felem = |felem_id: i64| -> String {
        felem_array
            .iter()
            .find(|fe| fe.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id))
            .and_then(|fe| fe.get("FELEM_CODE"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };

    // Assemble a call's elementList from CFG_CFBOM, ordered by EXEC_ORDER.
    let element_list = |cfcall_id: i64| -> Vec<Value> {
        let mut rows: Vec<&Value> = cfbom_array
            .iter()
            .filter(|bom| bom.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(cfcall_id))
            .collect();
        rows.sort_by_key(|bom| bom.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0));
        rows.into_iter()
            .map(|bom| {
                let felem_id = bom.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
                Value::from(resolve_felem(felem_id))
            })
            .collect()
    };

    // Sort the raw rows by (FTYPE_ID, CFCALL_ID) before projection so the numeric
    // sort key is never lost (mirrors Python and list_comparison_thresholds).
    let mut sorted: Vec<&Value> = cfcall_array.iter().collect();
    sorted.sort_by_key(|item| {
        (
            item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
            item.get("CFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0),
        )
    });

    // Transform comparison calls
    let items: Vec<Value> = sorted
        .into_iter()
        .map(|item| {
            let cfcall_id = item.get("CFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let cfunc_id = item.get("CFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            json!({
                "id": cfcall_id,
                "feature": resolve_ftype(ftype_id),
                "function": resolve_cfunc(cfunc_id),
                "elementList": element_list(cfcall_id)
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
                return Err(SzConfigError::AlreadyPresent(
                    "Feature/element already exists for call".to_string(),
                ));
            }
        }
    }

    // Allocate EXEC_ORDER per CFCALL_ID (the scope Python addCallElement numbers
    // BOM execution order within: getDesiredValueOrNext(bom_table,
    // [call_id_field, "EXEC_ORDER"], ...)). `None` auto-allocates; a supplied
    // order is honoured or rejected if taken. Never left null.
    let exec_order = {
        let empty: Vec<Value> = Vec::new();
        let cfbom_rows = config_data["G2_CONFIG"]["CFG_CFBOM"]
            .as_array()
            .unwrap_or(&empty);
        crate::helpers::get_desired_or_next_order(
            cfbom_rows,
            "EXEC_ORDER",
            &[("CFCALL_ID", params.cfcall_id)],
            params.exec_order,
        )?
    };

    // Create new CBOM record via CfbomRow so every key is always present.
    let row = CfbomRow {
        cfcall_id: params.cfcall_id,
        ftype_id: params.ftype_id,
        felem_id: params.felem_id,
        exec_order,
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

/// Delete a comparison call element, addressed by (call | feature) + element code.
///
/// The caller no longer supplies `EXEC_ORDER`: the target `CFG_CFBOM` row is
/// located by its call id and the element's `FELEM_ID`, and its execution order
/// is derived from that row. A CFBOM record stores the *element's* feature id in
/// `FTYPE_ID`, so that column is not part of the address.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `call` - Call id or feature code identifying the comparison call
/// * `element_code` - Element code (e.g. `"FIRST_NAME"`) to remove from the call
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
/// use sz_configtool_lib::calls::comparison::delete_comparison_call_element;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
///     "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
///     "CFG_CFBOM": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}]
/// }}"#;
/// let out = delete_comparison_call_element(
///     config, CallSelector::Feature("NAME"), "FIRST_NAME", None).unwrap();
/// let v: serde_json::Value = serde_json::from_str(&out).unwrap();
/// assert!(v["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap().is_empty());
/// ```
pub fn delete_comparison_call_element(
    config: &str,
    call: CallSelector,
    element_code: &str,
    element_feature: Option<&str>,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfcall_id = resolve_call_id(config, &config_data, call, resolve_cfcall_id_for_feature)?;
    // A non-existent call id is a hard NotFound (Python parity); only a missing
    // element on an *existing* call is the benign NotOnCall from derive below.
    ensure_call_exists(
        &config_data,
        "CFG_CFCALL",
        "CFCALL_ID",
        cfcall_id,
        "Comparison",
    )?;
    // When an element feature is given it (a) disambiguates the target BOM row
    // and (b) requires the element to be a member of that feature — a non-member
    // is a hard NotInFeature, not the benign NotOnCall (Python parity). Resolve
    // the feature first, then the element scoped to it.
    let (felem_id, element_ftype_id) = match element_feature {
        Some(f) => {
            let ftype_id = lookup_feature_id(config, f)?;
            (
                resolve_feature_element_id(&config_data, config, ftype_id, element_code, f)?,
                Some(ftype_id),
            )
        }
        None => (lookup_element_id(config, element_code)?, None),
    };

    // Derive EXEC_ORDER from the located BOM row (also validates existence).
    let exec_order = derive_bom_exec_order(
        &config_data,
        "CFG_CFBOM",
        "CFCALL_ID",
        cfcall_id,
        felem_id,
        element_ftype_id,
        "Comparison",
    )?;

    if let Some(cbom_array) = config_data["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        // Mirror the derive predicate: when a feature disambiguated the target
        // row, constrain the retain by FTYPE too. Otherwise a sibling row sharing
        // (call, felem, exec) under another feature would be over-deleted.
        cbom_array.retain(|item| {
            !(item.get("CFCALL_ID").and_then(|v| v.as_i64()) == Some(cfcall_id)
                && item.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
                && item.get("EXEC_ORDER").and_then(|v| v.as_i64()) == Some(exec_order)
                && element_ftype_id
                    .is_none_or(|ft| item.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ft)))
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
            id: None,
        };

        let (modified, new_record) = add_comparison_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();

        // CFG_CFCALL row: exactly the 3 keys, no EXEC_ORDER (flagged discrepancy).
        let cfcall = &value["G2_CONFIG"]["CFG_CFCALL"][0];
        assert_all_keys(cfcall, &CFCALL_KEYS);
        assert!(!cfcall.as_object().unwrap().contains_key("EXEC_ORDER"));
        assert_eq!(cfcall["FTYPE_ID"], json!(5));
        assert_eq!(cfcall["CFUNC_ID"], json!(7));
        // Auto-assigned CFCALL_ID seeds at 1000.
        assert_eq!(cfcall["CFCALL_ID"], json!(1000));
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
        // A specific free order is honoured (exec_order now Option).
        let params = AddComparisonCallElementParams {
            cfcall_id: 1000,
            ftype_id: 5,
            felem_id: 11,
            exec_order: Some(2),
        };

        let (modified, new_record) = add_comparison_call_element(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let cfbom = &value["G2_CONFIG"]["CFG_CFBOM"][0];

        assert_all_keys(cfbom, &CFBOM_KEYS);
        assert_all_keys(&new_record, &CFBOM_KEYS);
        assert_eq!(cfbom["EXEC_ORDER"], json!(2));
    }

    #[test]
    fn test_add_comparison_call_element_auto_allocates_and_rejects_taken() {
        // A call carrying one BOM row at order 1; a second element auto-allocates
        // to 2, an explicit free order is honoured, and a taken order is rejected.
        let config = r#"{"G2_CONFIG": {
            "CFG_CFBOM": [
                {"CFCALL_ID": 1000, "FTYPE_ID": 5, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ],
            "CFG_CFCALL": [{"CFCALL_ID": 1000, "FTYPE_ID": 5, "CFUNC_ID": 7}],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_CFUNC": [{"CFUNC_ID": 7, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FELEM": [{"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}]
        }}"#;

        // None -> max (1) + 1 = 2.
        let (modified, _) = add_comparison_call_element(
            config,
            AddComparisonCallElementParams {
                cfcall_id: 1000,
                ftype_id: 5,
                felem_id: 12,
                exec_order: None,
            },
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let new_row = value["G2_CONFIG"]["CFG_CFBOM"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["FELEM_ID"].as_i64() == Some(12))
            .unwrap();
        assert_eq!(new_row["EXEC_ORDER"], json!(2));

        // A taken order on the same call -> AlreadyExists.
        let err = add_comparison_call_element(
            config,
            AddComparisonCallElementParams {
                cfcall_id: 1000,
                ftype_id: 5,
                felem_id: 12,
                exec_order: Some(1),
            },
        )
        .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_add_comparison_call_specific_and_taken_id() {
        let config = base_config();
        // Specific free id honoured.
        let params = AddComparisonCallParams {
            ftype_code: "NAME".to_string(),
            cfunc_code: "GNR_COMP".to_string(),
            element_list: vec!["FIRST_NAME".to_string()],
            id: Some(2500),
        };
        let (modified, _) = add_comparison_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(
            value["G2_CONFIG"]["CFG_CFCALL"][0]["CFCALL_ID"],
            json!(2500)
        );

        // A taken id is rejected (config already carries CFCALL_ID 2500).
        let cfg = json!({"G2_CONFIG": {
            "CFG_CFCALL": [{"CFCALL_ID": 2500, "FTYPE_ID": 9, "CFUNC_ID": 7}],
            "CFG_CFBOM": [],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_CFUNC": [{"CFUNC_ID": 7, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}]
        }})
        .to_string();
        let params = AddComparisonCallParams {
            ftype_code: "NAME".to_string(),
            cfunc_code: "GNR_COMP".to_string(),
            element_list: vec!["FIRST_NAME".to_string()],
            id: Some(2500),
        };
        let err = add_comparison_call(&cfg, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    // #40 fixtures: a comparison call whose CFCALL_ID deliberately differs from
    // its FTYPE_ID, so id-vs-feature addressing is distinguishable.
    fn populated_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
            "CFG_CFBOM": [
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1},
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2}
            ]
        }}"#
        .to_string()
    }

    #[test]
    fn test_get_comparison_call_by_id_and_feature_match() {
        let config = populated_config();
        let by_id = get_comparison_call(&config, CallSelector::Id(7)).unwrap();
        let by_feature = get_comparison_call(&config, CallSelector::Feature("NAME")).unwrap();
        assert_eq!(by_id, by_feature);
        assert_eq!(by_id["CFCALL_ID"], json!(7));
    }

    #[test]
    fn test_get_comparison_call_missing() {
        let config = populated_config();
        assert!(get_comparison_call(&config, CallSelector::Id(999)).is_err());
        assert!(get_comparison_call(&config, CallSelector::Feature("PHONE")).is_err());
    }

    // #42: get-by-id not-found now carries the canonical
    // "{X} call ID {id} does not exist" wording, matching delete and the
    // other call families.
    #[test]
    fn test_get_comparison_call_not_found_message() {
        let config = populated_config();
        let err = get_comparison_call(&config, CallSelector::Id(999)).unwrap_err();
        assert_eq!(err.to_string(), "Comparison call ID 999 does not exist");
    }

    #[test]
    fn test_delete_comparison_call_element_derives_exec_order() {
        let config = populated_config();
        // No exec_order supplied; the SDK derives it and removes the right row.
        let out = delete_comparison_call_element(
            &config,
            CallSelector::Feature("NAME"),
            "FIRST_NAME",
            None,
        )
        .unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        let cfbom = v["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();
        assert_eq!(cfbom.len(), 1);
        assert_eq!(cfbom[0]["FELEM_ID"], json!(12));
    }

    #[test]
    fn test_delete_comparison_call_element_missing_call_is_not_found() {
        // A call id that does not exist at all is a hard NotFound (Python's
        // prepCallElement errors on the missing call record before touching the
        // BOM), NOT the benign NotOnCall.
        let config = populated_config();
        let err =
            delete_comparison_call_element(&config, CallSelector::Id(999), "FIRST_NAME", None)
                .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "Comparison call ID 999 does not exist");
    }

    #[test]
    fn test_delete_comparison_call_element_in_feature_not_on_call_is_not_on_call() {
        // #58: element feature supplied, MIDDLE_NAME IS a member of NAME's FBOM
        // but is not on call 7 -> the benign NotOnCall, NOT NotInFeature.
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 3, "FTYPE_CODE": "NAME"},
                {"FTYPE_ID": 4, "FTYPE_CODE": "ADDRESS"}
            ],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 13, "FELEM_CODE": "MIDDLE_NAME"},
                {"FELEM_ID": 20, "FELEM_CODE": "STREET"}
            ],
            "CFG_FBOM": [
                {"FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1},
                {"FTYPE_ID": 3, "FELEM_ID": 13, "EXEC_ORDER": 2},
                {"FTYPE_ID": 4, "FELEM_ID": 20, "EXEC_ORDER": 1}
            ],
            "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
            "CFG_CFBOM": [
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ]
        }}"#;
        let err = delete_comparison_call_element(
            config,
            CallSelector::Id(7),
            "MIDDLE_NAME",
            Some("NAME"),
        )
        .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotOnCall);

        // The same element under a real feature it is NOT a member of (ADDRESS
        // has no MIDDLE_NAME) -> the hard NotInFeature, with Python's wording.
        let err = delete_comparison_call_element(
            config,
            CallSelector::Id(7),
            "MIDDLE_NAME",
            Some("ADDRESS"),
        )
        .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotInFeature);
        assert_eq!(err.to_string(), "MIDDLE_NAME is not an element of ADDRESS");
    }

    #[test]
    fn test_delete_comparison_call_element_not_on_existing_call_is_not_on_call() {
        // The call exists, but this element is not among its BOM rows -> the
        // benign NotOnCall sub-case (step D). MIDDLE_NAME exists as an element
        // but is not on call 7.
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 13, "FELEM_CODE": "MIDDLE_NAME"}
            ],
            "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
            "CFG_CFBOM": [
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ]
        }}"#;
        let err = delete_comparison_call_element(config, CallSelector::Id(7), "MIDDLE_NAME", None)
            .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotOnCall);
    }

    // #41: stored order deliberately differs from the sorted (FTYPE_ID, CFCALL_ID)
    // order, and the CFBOM rows are stored out of EXEC_ORDER, so the test proves
    // both the SDK-owned sort and the EXEC_ORDER-ordered elementList assembly.
    fn unsorted_list_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 3, "FTYPE_CODE": "NAME"},
                {"FTYPE_ID": 7, "FTYPE_CODE": "ADDRESS"}
            ],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_CFCALL": [
                {"CFCALL_ID": 20, "FTYPE_ID": 7, "CFUNC_ID": 1},
                {"CFCALL_ID": 5, "FTYPE_ID": 3, "CFUNC_ID": 1}
            ],
            "CFG_CFBOM": [
                {"CFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2},
                {"CFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1}
            ]
        }}"#
        .to_string()
    }

    #[test]
    fn test_list_comparison_calls_sorted_with_element_list() {
        let calls = list_comparison_calls(&unsorted_list_config()).unwrap();
        // Sorted by (FTYPE_ID, CFCALL_ID): NAME (ftype 3) before ADDRESS (ftype 7).
        assert_eq!(calls[0]["feature"], json!("NAME"));
        assert_eq!(calls[0]["id"], json!(5));
        assert_eq!(calls[1]["feature"], json!("ADDRESS"));
        // elementList ordered by EXEC_ORDER despite reversed storage.
        assert_eq!(calls[0]["elementList"], json!(["FIRST_NAME", "LAST_NAME"]));
        // The ADDRESS call has no BOM rows -> empty elementList.
        assert_eq!(calls[1]["elementList"], json!([]));
    }
}
