//! Comparison function operations for Senzing configuration
//!
//! This module provides functions for managing comparison functions (CFG_CFUNC)
//! in the Senzing configuration JSON. CFG_CFRTN (comparison function return
//! codes / score rows) is owned by `thresholds.rs`.

use crate::error::SzConfigError;
use crate::helpers::{
    FieldUpdate, add_to_config_array, delete_from_config_array, field_or_null,
    find_in_config_array, get_next_id,
};
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_CFUNC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct CfuncRow {
    #[serde(rename = "CFUNC_ID")]
    cfunc_id: i64,
    #[serde(rename = "CFUNC_CODE")]
    cfunc_code: String,
    #[serde(rename = "CONNECT_STR")]
    connect_str: Option<String>,
    #[serde(rename = "ANON_SUPPORT")]
    anon_support: String,
    #[serde(rename = "CFUNC_DESC")]
    cfunc_desc: Option<String>,
    #[serde(rename = "LANGUAGE")]
    language: Option<String>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison function
///
/// `connect_str` is tri-valued to mirror the Python tool: `None` (or an absent
/// `connectStr` key) stores JSON `null`, `Some("")` stores an empty string, and
/// `Some(x)` stores `x`.
#[derive(Debug, Clone, Default)]
pub struct AddComparisonFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddComparisonFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            // Absent or explicit-null connectStr -> None (stored as JSON null).
            connect_str: json.get("connectStr").and_then(|v| v.as_str()),
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
            anon_support: json.get("anonSupport").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting a comparison function
///
/// `connect_str` is a tri-state [`FieldUpdate`]: `Leave` keeps the stored value,
/// `Clear` writes JSON `null`, and `Set(x)` writes `x` (including `Set("")`).
#[derive(Debug, Clone, Default)]
pub struct SetComparisonFunctionParams<'a> {
    pub connect_str: FieldUpdate<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub anon_support: Option<&'a str>,
}

/// Add a new comparison function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (all optional; a `None` `connect_str` stores
///   JSON `null`)
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
            "Comparison function already exists: {cfunc_code}"
        )));
    }

    // Validate ANON_SUPPORT domain (Python parity: ["Yes", "No"], default "No")
    let anon_support = if let Some(val) = params.anon_support {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::validation(format!(
                    "Invalid ANON_SUPPORT value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Get next CFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let cfunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_CFUNC", "CFUNC_ID", 1)?;

    // Build a complete row via CfuncRow so every CFG_CFUNC key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = CfuncRow {
        cfunc_id,
        cfunc_code,
        connect_str: params.connect_str.map(str::to_string),
        anon_support: anon_support.to_string(),
        cfunc_desc: params.description.map(str::to_string),
        language: params.language.map(str::to_string),
    };
    let new_record = serde_json::to_value(&row)?;

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
            SzConfigError::not_found(format!("Comparison function not found: {cfunc_code}"))
        })?;

    // Delete from CFG_CFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?;

    Ok((modified_json, function))
}

/// Reverse-resolve a feature code from an `FTYPE_ID` (`0` -> `"all"`).
fn ftype_code_for_id(config: &Value, ftype_id: i64) -> Option<String> {
    if ftype_id == 0 {
        return Some("all".to_string());
    }
    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| f.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id))
                .and_then(|f| f.get("FTYPE_CODE"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
}

/// Delete a comparison function and everything that depends on it (cascade).
///
/// Composes the existing piece-wise deletes in the Python order:
/// `CFG_CFBOM` (rows for the function's calls) → `CFG_CFCALL` → `CFG_CFRTN` →
/// `CFG_CFUNC`. The `CFG_CFRTN` step reuses the Wave-2 three-key
/// [`delete_comparison_threshold`](crate::thresholds::delete_comparison_threshold)
/// for each well-formed return row, then sweeps any orphan/null rows that remain
/// for the function so no dependent rows are left behind.
///
/// Unlike [`delete_comparison_function`] (which removes only the `CFG_CFUNC`
/// row), this leaves the configuration with no dangling references to the
/// deleted function.
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `cfunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if the function is not found or JSON is invalid
///
/// # Example
/// ```
/// use sz_configtool_lib::functions::comparison::delete_comparison_function_cascade;
///
/// let config = r#"{"G2_CONFIG": {
///     "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "CMP_X"}],
///     "CFG_CFCALL": [],
///     "CFG_CFBOM": [],
///     "CFG_CFRTN": [],
///     "CFG_FTYPE": []
/// }}"#;
/// let (updated, _removed) = delete_comparison_function_cascade(config, "CMP_X")?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn delete_comparison_function_cascade(
    config_json: &str,
    cfunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let cfunc_code = cfunc_code.to_uppercase();

    let function = find_in_config_array(config_json, "CFG_CFUNC", "CFUNC_CODE", &cfunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Comparison function not found: {cfunc_code}"))
        })?;
    let cfunc_id = function
        .get("CFUNC_ID")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| SzConfigError::MissingField("CFUNC_ID".to_string()))?;

    // Steps 1 & 2: remove CFG_CFBOM (by the CFCALL_IDs bound to this function),
    // then CFG_CFCALL.
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let cfcall_ids: Vec<i64> = config["G2_CONFIG"]["CFG_CFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|r| r["CFUNC_ID"].as_i64() == Some(cfunc_id))
                .filter_map(|r| r["CFCALL_ID"].as_i64())
                .collect()
        })
        .unwrap_or_default();

    if let Some(cfbom) = config["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cfbom.retain(|r| match r["CFCALL_ID"].as_i64() {
            Some(id) => !cfcall_ids.contains(&id),
            None => true,
        });
    }
    if let Some(cfcall) = config["G2_CONFIG"]["CFG_CFCALL"].as_array_mut() {
        cfcall.retain(|r| r["CFUNC_ID"].as_i64() != Some(cfunc_id));
    }
    let mut cur =
        serde_json::to_string(&config).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    // Step 3: CFG_CFRTN. Reuse the Wave-2 three-key delete for well-formed rows.
    let snapshot: Value =
        serde_json::from_str(&cur).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let cfrtn_rows: Vec<Value> = snapshot["G2_CONFIG"]["CFG_CFRTN"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter(|r| r["CFUNC_ID"].as_i64() == Some(cfunc_id))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    let mut seen: std::collections::HashSet<(i64, String)> = std::collections::HashSet::new();
    for row in &cfrtn_rows {
        if let (Some(fid), Some(rt)) = (row["FTYPE_ID"].as_i64(), row["CFUNC_RTNVAL"].as_str())
            && let Some(code) = ftype_code_for_id(&snapshot, fid)
            && seen.insert((fid, rt.to_uppercase()))
        {
            cur = crate::thresholds::delete_comparison_threshold(&cur, &cfunc_code, &code, rt)?;
        }
    }

    // Sweep any remaining CFG_CFRTN rows for this function (orphan FTYPE_ID or
    // null CFUNC_RTNVAL rows the three-key delete cannot address).
    let mut swept: Value =
        serde_json::from_str(&cur).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    if let Some(arr) = swept["G2_CONFIG"]["CFG_CFRTN"].as_array_mut() {
        arr.retain(|r| r["CFUNC_ID"].as_i64() != Some(cfunc_id));
    }
    cur = serde_json::to_string(&swept).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    // Step 4: CFG_CFUNC itself.
    let (final_json, _) = delete_comparison_function(&cur, &cfunc_code)?;

    Ok((final_json, function))
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
        SzConfigError::not_found(format!("Comparison function not found: {cfunc_code}"))
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
                // Stored-nullable columns are projected null-preserving; the
                // previously-missing `description` (CFUNC_DESC) is now emitted.
                // Field order: id, function, description, connectStr, anonSupport,
                // language.
                json!({
                    "id": item.get("CFUNC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                    "function": item.get("CFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "description": field_or_null(item, "CFUNC_DESC"),
                    "connectStr": field_or_null(item, "CONNECT_STR"),
                    "anonSupport": field_or_null(item, "ANON_SUPPORT"),
                    "language": field_or_null(item, "LANGUAGE")
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
        SzConfigError::not_found(format!("Comparison function not found: {cfunc_code}"))
    })?;

    // In-place update of a complete existing row; all keys preserved.
    // Update fields if provided
    if let Some(obj) = function.as_object_mut() {
        match params.connect_str {
            FieldUpdate::Leave => {}
            FieldUpdate::Clear => {
                obj.insert("CONNECT_STR".to_string(), Value::Null);
            }
            FieldUpdate::Set(conn) => {
                obj.insert("CONNECT_STR".to_string(), json!(conn));
            }
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
                connect_str: Some("g2CustomCmp"),
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

    /// #33: the list carries `description` (from CFUNC_DESC) in the exact key
    /// order id, function, description, connectStr, anonSupport, language; and
    /// stored-nullable columns are null-preserved (stored null -> null, stored ""
    /// -> "", absent -> null).
    #[test]
    fn test_list_comparison_functions_description_and_order() {
        let config = json!({
            "G2_CONFIG": {"CFG_CFUNC": [{
                "CFUNC_ID": 1,
                "CFUNC_CODE": "CMP_NAME",
                "CFUNC_DESC": "Compare names",
                "CONNECT_STR": "",
                "ANON_SUPPORT": null
                // LANGUAGE absent -> null
            }]}
        })
        .to_string();

        let items = list_comparison_functions(&config).unwrap();
        let keys: Vec<&str> = items[0]
            .as_object()
            .unwrap()
            .keys()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(
            keys,
            vec![
                "id",
                "function",
                "description",
                "connectStr",
                "anonSupport",
                "language"
            ]
        );
        assert_eq!(items[0]["description"], json!("Compare names"));
        assert_eq!(items[0]["connectStr"], json!("")); // stored "" preserved
        assert_eq!(items[0]["anonSupport"], Value::Null); // stored null preserved
        assert_eq!(items[0]["language"], Value::Null); // absent -> null
    }

    /// add_comparison_function must write a complete CFG_CFUNC row even when the
    /// caller omits optionals — they become null, never dropped. ANON_SUPPORT
    /// defaults to "No".
    #[test]
    fn test_add_comparison_function_emits_all_keys() {
        let config = get_test_config();
        let (modified, record) = add_comparison_function(
            &config,
            "custom_cmp",
            AddComparisonFunctionParams {
                connect_str: Some("g2CustomCmp"),
                description: None,
                language: None,
                anon_support: None,
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_CFUNC"].as_array().unwrap();
        let obj = arr.last().unwrap().as_object().unwrap();
        for key in [
            "CFUNC_ID",
            "CFUNC_CODE",
            "CONNECT_STR",
            "ANON_SUPPORT",
            "CFUNC_DESC",
            "LANGUAGE",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["CFUNC_CODE"], json!("CUSTOM_CMP"));
        assert_eq!(obj["CONNECT_STR"], json!("g2CustomCmp"));
        assert_eq!(obj["ANON_SUPPORT"], json!("No"));
        assert_eq!(obj["CFUNC_DESC"], Value::Null);
        assert_eq!(obj["LANGUAGE"], Value::Null);
        assert_eq!(record["ANON_SUPPORT"], json!("No"));
    }

    /// connect_str tri-value on the add path: None -> null, Some("") -> "",
    /// Some(x) -> x, with every key still emitted.
    #[test]
    fn test_add_comparison_function_connect_str_tri_value() {
        // None -> stored JSON null.
        let (m_none, _) = add_comparison_function(
            &get_test_config(),
            "cmp_none",
            AddComparisonFunctionParams {
                connect_str: None,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_none).unwrap();
        let row = v["G2_CONFIG"]["CFG_CFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        // Some("") -> stored empty string (distinct from null).
        let (m_empty, _) = add_comparison_function(
            &get_test_config(),
            "cmp_empty",
            AddComparisonFunctionParams {
                connect_str: Some(""),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_empty).unwrap();
        let row = v["G2_CONFIG"]["CFG_CFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!(""));

        // Some(x) -> stored x.
        let (m_val, _) = add_comparison_function(
            &get_test_config(),
            "cmp_val",
            AddComparisonFunctionParams {
                connect_str: Some("g2X"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_val).unwrap();
        let row = v["G2_CONFIG"]["CFG_CFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!("g2X"));
    }

    /// #38.4: the cascade empties every dependent table (CFBOM, CFCALL, CFRTN)
    /// for the function and then the function row, leaving no orphans.
    #[test]
    fn test_delete_comparison_function_cascade_empties_all() {
        let config = json!({
            "G2_CONFIG": {
                "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
                "CFG_CFUNC": [
                    {"CFUNC_ID": 1, "CFUNC_CODE": "CMP_X"},
                    {"CFUNC_ID": 2, "CFUNC_CODE": "CMP_KEEP"}
                ],
                "CFG_CFCALL": [
                    {"CFCALL_ID": 10, "FTYPE_ID": 3, "CFUNC_ID": 1},
                    {"CFCALL_ID": 11, "FTYPE_ID": 3, "CFUNC_ID": 2}
                ],
                "CFG_CFBOM": [
                    {"CFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 5, "EXEC_ORDER": 1},
                    {"CFCALL_ID": 11, "FTYPE_ID": 3, "FELEM_ID": 5, "EXEC_ORDER": 1}
                ],
                "CFG_CFRTN": [
                    {"CFRTN_ID": 100, "CFUNC_ID": 1, "FTYPE_ID": 3, "CFUNC_RTNVAL": "SAME"},
                    {"CFRTN_ID": 101, "CFUNC_ID": 1, "FTYPE_ID": 0, "CFUNC_RTNVAL": "CLOSE"},
                    {"CFRTN_ID": 102, "CFUNC_ID": 1, "FTYPE_ID": 999, "CFUNC_RTNVAL": null},
                    {"CFRTN_ID": 103, "CFUNC_ID": 2, "FTYPE_ID": 3, "CFUNC_RTNVAL": "SAME"}
                ]
            }
        })
        .to_string();

        let (modified, removed) = delete_comparison_function_cascade(&config, "cmp_x").unwrap();
        assert_eq!(removed["CFUNC_CODE"], "CMP_X");
        let v: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &v["G2_CONFIG"];

        // Function CMP_X gone, CMP_KEEP retained.
        let cfuncs = g2["CFG_CFUNC"].as_array().unwrap();
        assert_eq!(cfuncs.len(), 1);
        assert_eq!(cfuncs[0]["CFUNC_CODE"], "CMP_KEEP");

        // No CFCALL / CFBOM / CFRTN rows reference CFUNC_ID 1 or CFCALL_ID 10.
        assert!(
            g2["CFG_CFCALL"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["CFUNC_ID"] != 1)
        );
        assert!(
            g2["CFG_CFBOM"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["CFCALL_ID"] != 10)
        );
        assert!(
            g2["CFG_CFRTN"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["CFUNC_ID"] != 1)
        );

        // The unrelated function's rows survive.
        assert_eq!(g2["CFG_CFCALL"].as_array().unwrap().len(), 1);
        assert_eq!(g2["CFG_CFBOM"].as_array().unwrap().len(), 1);
        assert_eq!(g2["CFG_CFRTN"].as_array().unwrap().len(), 1);
    }

    /// connect_str tri-state on the set path: Leave keeps, Clear nulls, Set writes.
    #[test]
    fn test_set_comparison_function_connect_str_tri_state() {
        let base = get_test_config();

        // Leave: existing CONNECT_STR ("g2CmpName") preserved.
        let (m_leave, _) = set_comparison_function(
            &base,
            "CMP_NAME",
            SetComparisonFunctionParams {
                connect_str: FieldUpdate::Leave,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_leave).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_CFUNC"][0]["CONNECT_STR"],
            json!("g2CmpName")
        );

        // Clear: CONNECT_STR -> null (key still present).
        let (m_clear, _) = set_comparison_function(
            &base,
            "CMP_NAME",
            SetComparisonFunctionParams {
                connect_str: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_clear).unwrap();
        let row = &v["G2_CONFIG"]["CFG_CFUNC"][0];
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        // Set: CONNECT_STR replaced.
        let (m_set, _) = set_comparison_function(
            &base,
            "CMP_NAME",
            SetComparisonFunctionParams {
                connect_str: FieldUpdate::Set("g2New"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_set).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_CFUNC"][0]["CONNECT_STR"],
            json!("g2New")
        );
    }
}
