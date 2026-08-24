//! Expression function operations for Senzing configuration
//!
//! This module provides functions for managing expression functions (CFG_EFUNC)
//! in the Senzing configuration JSON.

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

/// Complete CFG_EFUNC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct EfuncRow {
    #[serde(rename = "EFUNC_ID")]
    efunc_id: i64,
    #[serde(rename = "EFUNC_CODE")]
    efunc_code: String,
    #[serde(rename = "CONNECT_STR")]
    connect_str: Option<String>,
    #[serde(rename = "EFUNC_DESC")]
    efunc_desc: Option<String>,
    #[serde(rename = "LANGUAGE")]
    language: Option<String>,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an expression function
///
/// `connect_str` is tri-valued to mirror the Python tool: `None` (or an absent
/// `connectStr` key) stores JSON `null`, `Some("")` stores an empty string, and
/// `Some(x)` stores `x`.
#[derive(Debug, Clone, Default)]
pub struct AddExpressionFunctionParams<'a> {
    pub connect_str: Option<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddExpressionFunctionParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self, SzConfigError> {
        Ok(Self {
            // Absent or explicit-null connectStr -> None (stored as JSON null).
            connect_str: json.get("connectStr").and_then(|v| v.as_str()),
            description: json.get("description").and_then(|v| v.as_str()),
            language: json.get("language").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting an expression function
///
/// `connect_str` is a tri-state [`FieldUpdate`]: `Leave` keeps the stored value,
/// `Clear` writes JSON `null`, and `Set(x)` writes `x` (including `Set("")`).
#[derive(Debug, Clone, Default)]
pub struct SetExpressionFunctionParams<'a> {
    pub connect_str: FieldUpdate<&'a str>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
}

/// Add a new expression function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code (will be uppercased)
/// * `params` - Function parameters (all optional; a `None` `connect_str` stores
///   JSON `null`)
///
/// # Returns
/// Result with modified JSON string and the new function record
///
/// # Errors
/// Returns error if function already exists or JSON is invalid
pub fn add_expression_function(
    config_json: &str,
    efunc_code: &str,
    params: AddExpressionFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Check if function already exists
    if find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?.is_some() {
        return Err(SzConfigError::validation(format!(
            "Expression function already exists: {efunc_code}"
        )));
    }

    // Get next EFUNC_ID
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;
    let efunc_id = get_next_id(&config_data, "G2_CONFIG.CFG_EFUNC", "EFUNC_ID", 1)?;

    // Build a complete row via EfuncRow so every CFG_EFUNC key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = EfuncRow {
        efunc_id,
        efunc_code,
        connect_str: params.connect_str.map(str::to_string),
        efunc_desc: params.description.map(str::to_string),
        language: params.language.map(str::to_string),
    };
    let new_record = serde_json::to_value(&row)?;

    // Add to CFG_EFUNC
    let modified_json = add_to_config_array(config_json, "CFG_EFUNC", new_record.clone())?;

    Ok((modified_json, new_record))
}

/// Delete an expression function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn delete_expression_function(
    config_json: &str,
    efunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Find the function
    let function = find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
        })?;

    // Delete from CFG_EFUNC
    let modified_json =
        delete_from_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?;

    Ok((modified_json, function))
}

/// Delete an expression function and everything that depends on it (cascade).
///
/// Composes the piece-wise deletes in the Python order: `CFG_EFBOM` (rows for
/// the function's calls) → `CFG_EFCALL` → `CFG_EFUNC`. Unlike
/// [`delete_expression_function`] (which removes only the `CFG_EFUNC` row), this
/// leaves no dangling references to the deleted function.
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to delete
///
/// # Returns
/// Result with modified JSON string and the deleted function record
///
/// # Errors
/// Returns error if the function is not found or JSON is invalid
///
/// # Example
/// ```
/// use sz_configtool_lib::functions::expression::delete_expression_function_cascade;
///
/// let config = r#"{"G2_CONFIG": {
///     "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "EXP_X"}],
///     "CFG_EFCALL": [],
///     "CFG_EFBOM": []
/// }}"#;
/// let (updated, _removed) = delete_expression_function_cascade(config, "EXP_X")?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn delete_expression_function_cascade(
    config_json: &str,
    efunc_code: &str,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    let function = find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?
        .ok_or_else(|| {
            SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
        })?;
    let efunc_id = function
        .get("EFUNC_ID")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| SzConfigError::MissingField("EFUNC_ID".to_string()))?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let efcall_ids: Vec<i64> = config["G2_CONFIG"]["CFG_EFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|r| r["EFUNC_ID"].as_i64() == Some(efunc_id))
                .filter_map(|r| r["EFCALL_ID"].as_i64())
                .collect()
        })
        .unwrap_or_default();

    if let Some(efbom) = config["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        efbom.retain(|r| match r["EFCALL_ID"].as_i64() {
            Some(id) => !efcall_ids.contains(&id),
            None => true,
        });
    }
    if let Some(efcall) = config["G2_CONFIG"]["CFG_EFCALL"].as_array_mut() {
        efcall.retain(|r| r["EFUNC_ID"].as_i64() != Some(efunc_id));
    }
    let cur =
        serde_json::to_string(&config).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let (final_json, _) = delete_expression_function(&cur, &efunc_code)?;

    Ok((final_json, function))
}

/// Get an expression function by code
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to retrieve
///
/// # Returns
/// Result with the function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn get_expression_function(
    config_json: &str,
    efunc_code: &str,
) -> Result<Value, SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?.ok_or_else(|| {
        SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
    })
}

/// List all expression functions
///
/// # Arguments
/// * `config_json` - The configuration JSON string
///
/// # Returns
/// Result with vector of function records in display format
///
/// # Errors
/// Returns error if JSON is invalid
pub fn list_expression_functions(config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    let config_data: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::json_parse(e.to_string()))?;

    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG")
        && let Some(array) = g2_config.get("CFG_EFUNC")
        && let Some(items) = array.as_array()
    {
        items
            .iter()
            .map(|item| {
                // connectStr and language are stored-nullable; project them
                // null-preserving via field_or_null rather than coercing to "".
                json!({
                    "id": field_or_null(item, "EFUNC_ID"),
                    "function": item.get("EFUNC_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                    "connectStr": field_or_null(item, "CONNECT_STR"),
                    "language": field_or_null(item, "LANGUAGE")
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Set (update) an expression function
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `efunc_code` - Function code to update
/// * `params` - Function parameters to update (all optional)
///
/// # Returns
/// Result with modified JSON string and the updated function record
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn set_expression_function(
    config_json: &str,
    efunc_code: &str,
    params: SetExpressionFunctionParams,
) -> Result<(String, Value), SzConfigError> {
    let efunc_code = efunc_code.to_uppercase();

    // Find existing function
    let mut function = find_in_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?
        .ok_or_else(|| {
        SzConfigError::not_found(format!("Expression function not found: {efunc_code}"))
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
            obj.insert("EFUNC_DESC".to_string(), json!(desc));
        }
        if let Some(lang) = params.language {
            obj.insert("LANGUAGE".to_string(), json!(lang));
        }
    }

    // Delete old and add updated
    let temp_json = delete_from_config_array(config_json, "CFG_EFUNC", "EFUNC_CODE", &efunc_code)?;
    let modified_json = add_to_config_array(&temp_json, "CFG_EFUNC", function.clone())?;

    Ok((modified_json, function))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> String {
        json!({
            "G2_CONFIG": {
                "CFG_EFUNC": [
                    {
                        "EFUNC_ID": 1,
                        "EFUNC_CODE": "EXPR_FEAT",
                        "CONNECT_STR": "g2ExprFeat",
                        "LANGUAGE": "en"
                    }
                ]
            }
        })
        .to_string()
    }

    #[test]
    fn test_add_expression_function() {
        let config = get_test_config();
        let result = add_expression_function(
            &config,
            "custom_expr",
            AddExpressionFunctionParams {
                connect_str: Some("g2CustomExpr"),
                description: Some("Custom expression"),
                language: Some("en"),
            },
        );
        assert!(result.is_ok());
        let (modified, record) = result.unwrap();
        assert!(modified.contains("CUSTOM_EXPR"));
        assert_eq!(record["EFUNC_CODE"], "CUSTOM_EXPR");
    }

    #[test]
    fn test_list_expression_functions() {
        let config = get_test_config();
        let result = list_expression_functions(&config);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["function"], "EXPR_FEAT");
    }

    /// add_expression_function must write a complete CFG_EFUNC row even when the
    /// caller omits optionals — they become null, never dropped.
    #[test]
    fn test_add_expression_function_emits_all_keys() {
        let config = get_test_config();
        let (modified, record) = add_expression_function(
            &config,
            "custom_expr",
            AddExpressionFunctionParams {
                connect_str: Some("g2CustomExpr"),
                description: None,
                language: None,
            },
        )
        .unwrap();

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_EFUNC"].as_array().unwrap();
        let obj = arr.last().unwrap().as_object().unwrap();
        for key in [
            "EFUNC_ID",
            "EFUNC_CODE",
            "CONNECT_STR",
            "EFUNC_DESC",
            "LANGUAGE",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["EFUNC_CODE"], json!("CUSTOM_EXPR"));
        assert_eq!(obj["CONNECT_STR"], json!("g2CustomExpr"));
        assert_eq!(obj["EFUNC_DESC"], Value::Null);
        assert_eq!(obj["LANGUAGE"], Value::Null);
        // Returned record mirrors the written row.
        assert_eq!(record["EFUNC_DESC"], Value::Null);
    }

    /// connect_str tri-value on the add path: None -> null, Some("") -> "",
    /// Some(x) -> x.
    #[test]
    fn test_add_expression_function_connect_str_tri_value() {
        let (m_none, _) = add_expression_function(
            &get_test_config(),
            "e_none",
            AddExpressionFunctionParams {
                connect_str: None,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_none).unwrap();
        let row = v["G2_CONFIG"]["CFG_EFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_empty, _) = add_expression_function(
            &get_test_config(),
            "e_empty",
            AddExpressionFunctionParams {
                connect_str: Some(""),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_empty).unwrap();
        let row = v["G2_CONFIG"]["CFG_EFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(row["CONNECT_STR"], json!(""));
    }

    /// connect_str tri-state on the set path: Leave keeps, Clear nulls, Set writes.
    #[test]
    fn test_set_expression_function_connect_str_tri_state() {
        let base = get_test_config();

        let (m_leave, _) = set_expression_function(
            &base,
            "EXPR_FEAT",
            SetExpressionFunctionParams {
                connect_str: FieldUpdate::Leave,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_leave).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_EFUNC"][0]["CONNECT_STR"],
            json!("g2ExprFeat")
        );

        let (m_clear, _) = set_expression_function(
            &base,
            "EXPR_FEAT",
            SetExpressionFunctionParams {
                connect_str: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_clear).unwrap();
        let row = &v["G2_CONFIG"]["CFG_EFUNC"][0];
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);

        let (m_set, _) = set_expression_function(
            &base,
            "EXPR_FEAT",
            SetExpressionFunctionParams {
                connect_str: FieldUpdate::Set("g2New"),
                ..Default::default()
            },
        )
        .unwrap();
        let v: Value = serde_json::from_str(&m_set).unwrap();
        assert_eq!(
            v["G2_CONFIG"]["CFG_EFUNC"][0]["CONNECT_STR"],
            json!("g2New")
        );
    }

    /// #38.4: the cascade empties CFG_EFBOM and CFG_EFCALL for the function and
    /// then removes the function, leaving no orphans.
    #[test]
    fn test_delete_expression_function_cascade_empties_all() {
        let config = json!({
            "G2_CONFIG": {
                "CFG_EFUNC": [
                    {"EFUNC_ID": 1, "EFUNC_CODE": "EXP_X"},
                    {"EFUNC_ID": 2, "EFUNC_CODE": "EXP_KEEP"}
                ],
                "CFG_EFCALL": [
                    {"EFCALL_ID": 10, "FTYPE_ID": 3, "EFUNC_ID": 1},
                    {"EFCALL_ID": 11, "FTYPE_ID": 3, "EFUNC_ID": 2}
                ],
                "CFG_EFBOM": [
                    {"EFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 5, "EXEC_ORDER": 1},
                    {"EFCALL_ID": 11, "FTYPE_ID": 3, "FELEM_ID": 5, "EXEC_ORDER": 1}
                ]
            }
        })
        .to_string();

        let (modified, removed) = delete_expression_function_cascade(&config, "exp_x").unwrap();
        assert_eq!(removed["EFUNC_CODE"], "EXP_X");
        let v: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &v["G2_CONFIG"];

        assert_eq!(g2["CFG_EFUNC"].as_array().unwrap().len(), 1);
        assert!(
            g2["CFG_EFCALL"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["EFUNC_ID"] != 1)
        );
        assert!(
            g2["CFG_EFBOM"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["EFCALL_ID"] != 10)
        );
        assert_eq!(g2["CFG_EFCALL"].as_array().unwrap().len(), 1);
        assert_eq!(g2["CFG_EFBOM"].as_array().unwrap().len(), 1);
    }
}
