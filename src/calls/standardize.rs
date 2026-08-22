//! Standardize call management operations
//!
//! Functions for managing CFG_SFCALL (standardize calls) configuration rows.
//! Standardize calls and their elements are both represented as CFG_SFCALL
//! rows; there is no separate standardize bill-of-materials table.

use crate::calls::{CallSelector, resolve_call_id};
use crate::config_rows::SfcallRow;
use crate::error::{Result, SzConfigError};
use crate::helpers::{
    get_next_id, lookup_element_id, lookup_feature_id, lookup_sfunc_id,
    resolve_sfcall_id_for_feature,
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
                "Execution order {order} already taken for this feature/element"
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

    // Create new CFG_SFCALL record via SfcallRow so every key is always present.
    let row = SfcallRow {
        sfcall_id,
        ftype_id,
        felem_id,
        sfunc_id,
        exec_order: Some(final_exec_order),
    };
    let new_record = serde_json::to_value(&row)?;

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
            "Standardize call ID {sfcall_id} does not exist"
        )));
    }

    // Delete the standardize call
    if let Some(sfcall_array) = config_data["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.retain(|record| record["SFCALL_ID"].as_i64() != Some(sfcall_id));
    }

    serde_json::to_string(&config_data).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a single standardize call, addressed by id or by feature code.
///
/// Pass [`CallSelector::Id`] to look the call up by its `SFCALL_ID`, or
/// [`CallSelector::Feature`] to resolve the standardize call bound to a feature.
/// The feature path scans `CFG_SFCALL` by `FTYPE_ID` via
/// [`resolve_sfcall_id_for_feature`] rather than treating the feature id as a
/// call id (the historical bug this fixes). Standardize calls can legitimately
/// be many-per-feature, so an ambiguous feature match errors — address by id.
///
/// # Arguments
/// * `config` - Configuration JSON string
/// * `selector` - Call id or feature code identifying the call
///
/// # Returns
/// JSON Value representing the standardize call record
///
/// # Errors
/// - `NotFound` if no matching call exists
/// - `InvalidInput` if a feature code matches more than one call (ambiguous)
///
/// # Example
/// ```
/// use sz_configtool_lib::calls::CallSelector;
/// use sz_configtool_lib::calls::standardize::get_standardize_call;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
///     "CFG_SFCALL": [{"SFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1}]
/// }}"#;
/// let by_id = get_standardize_call(config, CallSelector::Id(5)).unwrap();
/// let by_feature = get_standardize_call(config, CallSelector::Feature("NAME")).unwrap();
/// assert_eq!(by_id, by_feature);
/// ```
pub fn get_standardize_call(config: &str, selector: CallSelector) -> Result<Value> {
    let root: Value =
        serde_json::from_str(config).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;
    let sfcall_id = resolve_call_id(config, &root, selector, resolve_sfcall_id_for_feature)?;

    root.get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFCALL"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|c| c.get("SFCALL_ID").and_then(|v| v.as_i64()) == Some(sfcall_id))
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Standardize call ID {sfcall_id} does not exist"))
        })
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

    // Sort the raw rows by (FTYPE_ID, EXEC_ORDER) before projection so the numeric
    // sort key is never lost (mirrors Python). Standardize calls have no BOM, so
    // there is deliberately no elementList.
    let mut sorted: Vec<&Value> = sfcall_array.iter().collect();
    sorted.sort_by_key(|item| {
        (
            item.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
            item.get("EXEC_ORDER").and_then(|v| v.as_i64()).unwrap_or(0),
        )
    });

    // Transform standardize calls
    let items: Vec<Value> = sorted
        .into_iter()
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

/// Add a standardize call element (CFG_SFCALL record)
///
/// Creates a new standardize call element as a CFG_SFCALL row.
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

    // Create new call element record via SfcallRow so every key is always
    // present (EXEC_ORDER null when not specified - seed-then-null pattern).
    let row = SfcallRow {
        sfcall_id,
        ftype_id: params.ftype_id,
        felem_id: final_felem_id,
        sfunc_id: params.sfunc_id,
        exec_order: params.exec_order,
    };
    let new_record = serde_json::to_value(&row)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    const SFCALL_KEYS: [&str; 5] = [
        "SFCALL_ID",
        "FTYPE_ID",
        "FELEM_ID",
        "SFUNC_ID",
        "EXEC_ORDER",
    ];

    fn assert_all_keys(obj: &Value, keys: &[&str]) {
        let map = obj.as_object().unwrap();
        for key in keys {
            assert!(map.contains_key(*key), "{key} key must be present");
        }
    }

    fn base_config() -> String {
        r#"{"G2_CONFIG": {
            "CFG_SFCALL": [],
            "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "NAME"}],
            "CFG_SFUNC": [{"SFUNC_ID": 7, "SFUNC_CODE": "PARSE_NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"}]
        }}"#
        .to_string()
    }

    #[test]
    fn test_add_standardize_call_emits_all_keys() {
        let config = base_config();
        let params = AddStandardizeCallParams {
            ftype_code: Some("NAME"),
            felem_code: None,
            exec_order: None,
            sfunc_code: "PARSE_NAME",
        };

        let (modified, new_record) = add_standardize_call(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let sfcall = &value["G2_CONFIG"]["CFG_SFCALL"][0];

        assert_all_keys(sfcall, &SFCALL_KEYS);
        assert_all_keys(&new_record, &SFCALL_KEYS);
        assert_eq!(sfcall["FTYPE_ID"], json!(5));
        assert_eq!(sfcall["SFUNC_ID"], json!(7));
        assert_eq!(sfcall["FELEM_ID"], json!(-1));
        // add_standardize_call always assigns a concrete EXEC_ORDER.
        assert_eq!(sfcall["EXEC_ORDER"], json!(1));
    }

    #[test]
    fn test_add_standardize_call_element_emits_all_keys_exec_order_present() {
        let config = base_config();
        let params = AddStandardizeCallElementParams {
            ftype_id: 5,
            sfunc_id: 7,
            felem_id: Some(11),
            exec_order: Some(4),
        };

        let (modified, new_record) = add_standardize_call_element(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let sfcall = &value["G2_CONFIG"]["CFG_SFCALL"][0];

        assert_all_keys(sfcall, &SFCALL_KEYS);
        assert_all_keys(&new_record, &SFCALL_KEYS);
        assert_eq!(sfcall["FELEM_ID"], json!(11));
        assert_eq!(sfcall["EXEC_ORDER"], json!(4));
    }

    #[test]
    fn test_add_standardize_call_element_emits_exec_order_null_when_absent() {
        let config = base_config();
        let params = AddStandardizeCallElementParams {
            ftype_id: 5,
            sfunc_id: 7,
            felem_id: None,
            exec_order: None,
        };

        let (modified, _new_record) = add_standardize_call_element(&config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let sfcall = &value["G2_CONFIG"]["CFG_SFCALL"][0];

        // EXEC_ORDER key present but null (seed-then-null preserved); FELEM_ID -1.
        assert_all_keys(sfcall, &SFCALL_KEYS);
        assert_eq!(sfcall["EXEC_ORDER"], Value::Null);
        assert_eq!(sfcall["FELEM_ID"], json!(-1));
    }

    // #40 regression fixtures: SFCALL_ID (5) deliberately differs from FTYPE_ID
    // (3). getStandardizeCall by feature must scan CFG_SFCALL and return the
    // call with SFCALL_ID 5, not the (wrong) row the FTYPE_ID-as-call-id bug hit.
    #[test]
    fn test_get_standardize_call_by_feature_fixes_ftype_as_id_bug() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_SFCALL": [
                {"SFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1}
            ]
        }}"#;
        let by_id = get_standardize_call(config, CallSelector::Id(5)).unwrap();
        let by_feature = get_standardize_call(config, CallSelector::Feature("NAME")).unwrap();
        assert_eq!(by_id, by_feature);
        assert_eq!(by_feature["SFCALL_ID"], json!(5));
    }

    #[test]
    fn test_get_standardize_call_ambiguous_feature_errors() {
        // Two standardize calls for one feature -> by-feature is ambiguous.
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_SFCALL": [
                {"SFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1},
                {"SFCALL_ID": 6, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 2, "EXEC_ORDER": 2}
            ]
        }}"#;
        let err = get_standardize_call(config, CallSelector::Feature("NAME"));
        assert_eq!(
            err.unwrap_err().kind(),
            crate::error::SzErrorKind::InvalidInput
        );
        // Addressing the specific call by id still works.
        assert!(get_standardize_call(config, CallSelector::Id(6)).is_ok());
    }

    #[test]
    fn test_list_standardize_calls_sorted_no_element_list() {
        // Stored order differs from (FTYPE_ID, EXEC_ORDER).
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 3, "FTYPE_CODE": "NAME"},
                {"FTYPE_ID": 7, "FTYPE_CODE": "ADDRESS"}
            ],
            "CFG_FELEM": [],
            "CFG_SFUNC": [{"SFUNC_ID": 1, "SFUNC_CODE": "PARSE_NAME"}],
            "CFG_SFCALL": [
                {"SFCALL_ID": 20, "FTYPE_ID": 7, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1},
                {"SFCALL_ID": 8, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 2},
                {"SFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1}
            ]
        }}"#;

        let calls = list_standardize_calls(config).unwrap();
        // (FTYPE_ID, EXEC_ORDER): (3,1) id 5, (3,2) id 8, (7,1) id 20.
        assert_eq!(calls[0]["id"], json!(5));
        assert_eq!(calls[1]["id"], json!(8));
        assert_eq!(calls[2]["id"], json!(20));
        // Standardize calls carry no BOM, so no elementList key.
        assert!(!calls[0].as_object().unwrap().contains_key("elementList"));
    }

    // #42: the not-found message must carry the canonical
    // "{X} call ID {id} does not exist" wording, matching the comparison/
    // distinct/expression siblings (previously it was truncated).
    #[test]
    fn test_delete_standardize_call_not_found_message() {
        let config = r#"{"G2_CONFIG": {"CFG_SFCALL": []}}"#;
        let err = delete_standardize_call(config, 99).unwrap_err();
        assert_eq!(err.to_string(), "Standardize call ID 99 does not exist");
    }

    #[test]
    fn test_get_standardize_call_not_found_message() {
        let config = r#"{"G2_CONFIG": {"CFG_SFCALL": []}}"#;
        let err = get_standardize_call(config, CallSelector::Id(99)).unwrap_err();
        assert_eq!(err.to_string(), "Standardize call ID 99 does not exist");
    }
}
