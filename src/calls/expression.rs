//! Expression call management operations
//!
//! Functions for managing CFG_EFCALL (expression calls) and CFG_EFBOM
//! (expression bill of materials) configuration sections.

use crate::calls::{CallSelector, derive_bom_exec_order, resolve_call_id};
use crate::config_rows::{EfbomRow, EfcallRow};
use crate::error::{Result, SzConfigError};
use crate::helpers::{
    get_next_id, lookup_efunc_id, lookup_element_id, lookup_feature_id,
    resolve_efcall_id_for_feature,
};
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an expression call
#[derive(Debug, Clone)]
pub struct AddExpressionCallParams<'a> {
    pub efunc_code: &'a str,
    pub element_list: Vec<(String, String, Option<String>)>, // (element, required, feature)
    pub ftype_code: Option<&'a str>,
    pub felem_code: Option<&'a str>,
    pub exec_order: Option<i64>,
    pub expression_feature: Option<&'a str>,
    pub is_virtual: &'a str,
}

impl<'a> AddExpressionCallParams<'a> {
    pub fn new(efunc_code: &'a str, element_list: Vec<(String, String, Option<String>)>) -> Self {
        Self {
            efunc_code,
            element_list,
            ftype_code: None,
            felem_code: None,
            exec_order: None,
            expression_feature: None,
            is_virtual: "No",
        }
    }
}

/// Parameters for expression call element operations
#[derive(Debug, Clone)]
pub struct ExpressionCallElementParams {
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: i64,
    pub felem_req: String,
}

impl ExpressionCallElementParams {
    pub fn new(ftype_id: i64, felem_id: i64, exec_order: i64, felem_req: String) -> Self {
        Self {
            ftype_id,
            felem_id,
            exec_order,
            felem_req,
        }
    }
}

/// Parameters for setting (updating) an expression call
#[derive(Debug, Clone, Default)]
pub struct SetExpressionCallParams {
    pub efcall_id: i64,
    pub exec_order: Option<i64>,
}

impl TryFrom<&Value> for SetExpressionCallParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let efcall_id = json
            .get("efcallId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("efcallId".to_string()))?;

        Ok(Self {
            efcall_id,
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
        })
    }
}

/// Add a new expression call with element list
///
/// Creates a new expression call linking a function to a feature or element
/// with an execution order and associated elements (EBOM records).
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Expression call parameters
///
/// # Returns
/// Tuple of (modified_config, new_efcall_record)
///
/// # Errors
/// - `InvalidParameter` if both ftype_code and felem_code are specified or both missing
/// - `Duplicate` if exec_order is already taken for the feature/element
/// - `NotFound` if function/feature/element codes don't exist
pub fn add_expression_call(
    config: &str,
    params: AddExpressionCallParams,
) -> Result<(String, Value)> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Get next EFCALL_ID (seed at 1000 for user-created calls)
    let efcall_id = get_next_id(&config_data, "G2_CONFIG.CFG_EFCALL", "EFCALL_ID", 1000)?;

    // Lookup function ID
    let efunc_id = lookup_efunc_id(config, params.efunc_code)?;

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

    // Determine exec_order
    let final_exec_order = if let Some(order) = params.exec_order {
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
                "Execution order {order} already taken for this feature/element"
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
    let efeat_ftype_id = if let Some(expr_feat) = params
        .expression_feature
        .filter(|f| !f.eq_ignore_ascii_case("N/A"))
    {
        lookup_feature_id(config, expr_feat)?
    } else {
        -1
    };

    // Process element list and create EFBOM records
    let mut efbom_records = Vec::new();

    for (idx, (element_code, required, feature_opt)) in params.element_list.into_iter().enumerate()
    {
        // EXEC_ORDER is 1-based over the element list.
        let bom_exec_order = idx as i64 + 1;

        // Keep feature name for error messages (clone before consuming)
        let _bom_feature_name_for_errors = feature_opt.clone();

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

        // Lookup element ID (always global lookup - feature field is just metadata for EFBOM)
        let bom_felem_id = lookup_element_id(config, &element_code)?;

        // Create EFBOM record via EfbomRow so every key is always present.
        let bom_row = EfbomRow {
            efcall_id,
            ftype_id: bom_ftype_id,
            felem_id: bom_felem_id,
            exec_order: bom_exec_order,
            felem_req: required,
        };
        efbom_records.push(serde_json::to_value(&bom_row)?);
    }

    // Create new CFG_EFCALL record via EfcallRow so every key is always present.
    let efcall_row = EfcallRow {
        efcall_id,
        ftype_id,
        felem_id,
        efunc_id,
        exec_order: final_exec_order,
        efeat_ftype_id,
        is_virtual: params.is_virtual.to_string(),
    };
    let new_record = serde_json::to_value(&efcall_row)?;

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
            "Expression call ID {efcall_id} does not exist"
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

/// Get a single expression call, addressed by id or by feature code.
///
/// Pass [`CallSelector::Id`] to look the call up by its `EFCALL_ID`, or
/// [`CallSelector::Feature`] to resolve the expression call bound to a feature.
/// Expression calls are genuinely many-per-feature (`EXEC_ORDER` exists for
/// exactly this reason), so a feature code that matches more than one call
/// errors clearly via [`resolve_efcall_id_for_feature`] rather than silently
/// picking one — address such calls by id.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `selector` - Call id or feature code identifying the call
///
/// # Returns
/// JSON Value representing the expression call record
///
/// # Errors
/// - `NotFound` if no matching call exists
/// - `InvalidInput` if a feature code matches more than one call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::expression::get_expression_call;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_EFCALL": [{"EFCALL_ID": 9, "FTYPE_ID": 3, "FELEM_ID": -1, "EFUNC_ID": 1,
///                     "EXEC_ORDER": 1, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"}]
/// }}"#;
/// let by_id = get_expression_call(config, CallSelector::Id(9)).unwrap();
/// let by_feature = get_expression_call(config, CallSelector::Feature("NAME")).unwrap();
/// assert_eq!(by_id, by_feature);
/// ```
pub fn get_expression_call(config: &str, selector: CallSelector) -> Result<Value> {
    let root: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;
    let efcall_id = resolve_call_id(config, &root, selector, resolve_efcall_id_for_feature)?;

    root.get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFCALL"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|c| c.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id))
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Expression call ID {efcall_id} does not exist"))
        })
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

    let efbom_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFBOM"))
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

    // Assemble a call's elementList from CFG_EFBOM, ordered by EXEC_ORDER.
    let element_list = |efcall_id: i64| -> Vec<Value> {
        let mut rows: Vec<&Value> = efbom_array
            .iter()
            .filter(|bom| bom.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id))
            .collect();
        rows.sort_by_key(|bom| bom.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0));
        rows.into_iter()
            .map(|bom| {
                let felem_id = bom.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
                Value::from(resolve_felem(felem_id))
            })
            .collect()
    };

    // Sort the raw rows by (FTYPE_ID, FELEM_ID, EXEC_ORDER) before projection so
    // the numeric sort key is never lost (mirrors Python).
    let mut sorted: Vec<&Value> = efcall_array.iter().collect();
    sorted.sort_by_key(|item| {
        (
            item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
            item.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0),
            item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0),
        )
    });

    // Transform expression calls
    let items: Vec<Value> = sorted
        .into_iter()
        .map(|item| {
            let efcall_id = item.get("EFCALL_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let ftype_id = item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let felem_id = item.get("FELEM_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let efunc_id = item.get("EFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0);

            let efeat_ftype_id = item.get("EFEAT_FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(-1);

            json!({
                "id": efcall_id,
                "feature": resolve_ftype(ftype_id),
                "element": resolve_felem(felem_id),
                "execOrder": item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0),
                "function": resolve_efunc(efunc_id),
                "isVirtual": item.get("IS_VIRTUAL").and_then(|v| v.as_str()).unwrap_or("No"),
                "expressionFeature": if efeat_ftype_id <= 0 { "n/a".to_string() } else { resolve_ftype(efeat_ftype_id) },
                "elementList": element_list(efcall_id)
            })
        })
        .collect();

    Ok(items)
}

/// Update an expression call (stub - not implemented in Python)
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `params` - Expression call parameters (efcall_id required, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_expression_call(config: &str, _params: SetExpressionCallParams) -> Result<String> {
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
/// * `params` - Expression call element parameters
///
/// # Returns
/// Tuple of (modified_config, new_ebom_record)
pub fn add_expression_call_element(
    config: &str,
    efcall_id: i64,
    params: ExpressionCallElementParams,
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
    if let Some(ebom_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFBOM"))
        .and_then(|v| v.as_array())
    {
        for item in ebom_array {
            if item.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id)
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

    // Create new EBOM record via EfbomRow (use params.exec_order directly - no
    // auto-assignment) so every key is always present.
    let row = EfbomRow {
        efcall_id,
        ftype_id: params.ftype_id,
        felem_id: params.felem_id,
        exec_order: params.exec_order,
        felem_req: params.felem_req,
    };
    let new_record = serde_json::to_value(&row)?;

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

/// Delete an expression call element, addressed by (call | feature) + element code.
///
/// The caller no longer supplies `EXEC_ORDER`: the target `CFG_EFBOM` row is
/// located by its call id and the element's `FELEM_ID`, and its execution order
/// is derived from that row. An EFBOM record stores the *element's* feature id in
/// `FTYPE_ID`, so that column is not part of the address.
///
/// Because expression calls are many-per-feature, addressing by
/// [`CallSelector::Feature`] errors when the feature has more than one
/// expression call — use [`CallSelector::Id`] to name the specific call.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `call` - Call id or feature code identifying the expression call
/// * `element_code` - Element code to remove from the call
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if the element is not on the call (or the call/element codes don't resolve)
/// - `InvalidInput` if a feature code matches more than one call, or the element
///   matches more than one BOM row on the call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::expression::delete_expression_call_element;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
///     "CFG_EFBOM": [{"EFCALL_ID": 9, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1, "FELEM_REQ": "No"}]
/// }}"#;
/// let out = delete_expression_call_element(
///     config, CallSelector::Id(9), "FIRST_NAME").unwrap();
/// let v: serde_json::Value = serde_json::from_str(&out).unwrap();
/// assert!(v["G2_CONFIG"]["CFG_EFBOM"].as_array().unwrap().is_empty());
/// ```
pub fn delete_expression_call_element(
    config: &str,
    call: CallSelector,
    element_code: &str,
) -> Result<String> {
    let mut config_data: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let efcall_id = resolve_call_id(config, &config_data, call, resolve_efcall_id_for_feature)?;
    let felem_id = lookup_element_id(config, element_code)?;

    // Derive EXEC_ORDER from the located BOM row (also validates existence).
    let exec_order = derive_bom_exec_order(
        &config_data,
        "CFG_EFBOM",
        "EFCALL_ID",
        efcall_id,
        felem_id,
        "Expression",
    )?;

    if let Some(ebom_array) = config_data["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        ebom_array.retain(|item| {
            !(item.get("EFCALL_ID").and_then(|v| v.as_i64()) == Some(efcall_id)
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
/// * `params` - Expression call element parameters (efcall_id, ftype_id, felem_id, exec_order, updates)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_expression_call_element(
    config: &str,
    _params: ExpressionCallElementParams,
) -> Result<String> {
    // This is a stub - not commonly used
    Ok(config.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EFCALL_KEYS: [&str; 7] = [
        "EFCALL_ID",
        "FTYPE_ID",
        "FELEM_ID",
        "EFUNC_ID",
        "EXEC_ORDER",
        "EFEAT_FTYPE_ID",
        "IS_VIRTUAL",
    ];
    const EFBOM_KEYS: [&str; 5] = [
        "EFCALL_ID",
        "FTYPE_ID",
        "FELEM_ID",
        "EXEC_ORDER",
        "FELEM_REQ",
    ];

    fn assert_all_keys(obj: &Value, keys: &[&str]) {
        let map = obj.as_object().unwrap();
        for key in keys {
            assert!(map.contains_key(*key), "{key} key must be present");
        }
    }

    fn base_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_EFCALL": [],
            "CFG_EFBOM": [],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_EFUNC": [{"EFUNC_ID": 7, "EFUNC_CODE": "EXPRESS_BOM"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}]
        }}"#
        .to_string()
    }

    #[test]
    fn test_add_expression_call_emits_all_keys() {
        let config = base_config();
        let params = AddExpressionCallParams::new(
            "EXPRESS_BOM",
            vec![("FIRST_NAME".to_string(), "No".to_string(), None)],
        );
        let params = AddExpressionCallParams {
            ftype_code: Some("NAME"),
            ..params
        };

        let (modified, new_record) = add_expression_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();

        // CFG_EFCALL row: all 7 keys.
        let efcall = &value["G2_CONFIG"]["CFG_EFCALL"][0];
        assert_all_keys(efcall, &EFCALL_KEYS);
        assert_all_keys(&new_record, &EFCALL_KEYS);
        assert_eq!(efcall["FTYPE_ID"], json!(5));
        assert_eq!(efcall["EFUNC_ID"], json!(7));
        assert_eq!(efcall["FELEM_ID"], json!(-1));
        assert_eq!(efcall["EFEAT_FTYPE_ID"], json!(-1));
        assert_eq!(efcall["IS_VIRTUAL"], json!("No"));

        // CFG_EFBOM row: all 5 keys.
        let efbom = &value["G2_CONFIG"]["CFG_EFBOM"][0];
        assert_all_keys(efbom, &EFBOM_KEYS);
        assert_eq!(efbom["FELEM_ID"], json!(11));
        assert_eq!(efbom["FELEM_REQ"], json!("No"));
        assert_eq!(efbom["EXEC_ORDER"], json!(1));
    }

    #[test]
    fn test_add_expression_call_element_emits_all_keys() {
        let config = base_config();
        let params = ExpressionCallElementParams::new(5, 11, 2, "Yes".to_string());

        let (modified, new_record) = add_expression_call_element(&config, 1000, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let efbom = &value["G2_CONFIG"]["CFG_EFBOM"][0];

        assert_all_keys(efbom, &EFBOM_KEYS);
        assert_all_keys(&new_record, &EFBOM_KEYS);
        assert_eq!(efbom["FELEM_REQ"], json!("Yes"));
        assert_eq!(efbom["EXEC_ORDER"], json!(2));
    }

    // #40 fixtures: two expression calls bound to the same feature (legitimate —
    // expression is many-per-feature) plus a BOM row per call.
    fn populated_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}],
            "CFG_EFCALL": [
                {"EFCALL_ID": 9, "FTYPE_ID": 3, "FELEM_ID": -1, "EFUNC_ID": 1,
                 "EXEC_ORDER": 1, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"},
                {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": -1, "EFUNC_ID": 2,
                 "EXEC_ORDER": 2, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"}
            ],
            "CFG_EFBOM": [
                {"EFCALL_ID": 9, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1, "FELEM_REQ": "No"},
                {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1, "FELEM_REQ": "No"}
            ]
        }}"#
        .to_string()
    }

    #[test]
    fn test_get_expression_call_by_id() {
        let config = populated_config();
        let by_id = get_expression_call(&config, CallSelector::Id(10)).unwrap();
        assert_eq!(by_id["EFCALL_ID"], json!(10));
    }

    #[test]
    fn test_get_expression_call_ambiguous_feature_errors() {
        let config = populated_config();
        // The feature has two expression calls -> by-feature must error clearly,
        // not silently pick one.
        let err = get_expression_call(&config, CallSelector::Feature("NAME"));
        assert_eq!(
            err.unwrap_err().kind(),
            crate::error::SzErrorKind::InvalidInput
        );
    }

    #[test]
    fn test_delete_expression_call_element_ambiguous_feature_errors() {
        let config = populated_config();
        let err =
            delete_expression_call_element(&config, CallSelector::Feature("NAME"), "FIRST_NAME");
        assert_eq!(
            err.unwrap_err().kind(),
            crate::error::SzErrorKind::InvalidInput
        );
    }

    #[test]
    fn test_delete_expression_call_element_by_id_derives_exec_order() {
        let config = populated_config();
        // Addressing the specific call by id disambiguates; exec_order derived.
        let out =
            delete_expression_call_element(&config, CallSelector::Id(9), "FIRST_NAME").unwrap();
        let v: Value = serde_json::from_str(&out).unwrap();
        let efbom = v["G2_CONFIG"]["CFG_EFBOM"].as_array().unwrap();
        assert_eq!(efbom.len(), 1);
        assert_eq!(efbom[0]["EFCALL_ID"], json!(10));
    }

    #[test]
    fn test_list_expression_calls_sorted_with_element_list() {
        // Stored order differs from (FTYPE_ID, FELEM_ID, EXEC_ORDER); BOM out of order.
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "EXP_X"}],
            "CFG_EFCALL": [
                {"EFCALL_ID": 30, "FTYPE_ID": 3, "FELEM_ID": 12, "EFUNC_ID": 1, "EXEC_ORDER": 1, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"},
                {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 11, "EFUNC_ID": 1, "EXEC_ORDER": 1, "EFEAT_FTYPE_ID": -1, "IS_VIRTUAL": "No"}
            ],
            "CFG_EFBOM": [
                {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2, "FELEM_REQ": "Yes"},
                {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1, "FELEM_REQ": "Yes"}
            ]
        }}"#;

        let calls = list_expression_calls(config).unwrap();
        // Sorted by (FTYPE_ID, FELEM_ID, EXEC_ORDER): FELEM_ID 11 (id 10) before 12 (id 30).
        assert_eq!(calls[0]["id"], json!(10));
        assert_eq!(calls[0]["element"], json!("FIRST_NAME"));
        assert_eq!(calls[1]["id"], json!(30));
        // elementList ordered by EXEC_ORDER despite reversed storage.
        assert_eq!(calls[0]["elementList"], json!(["FIRST_NAME", "LAST_NAME"]));
    }
}
