use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

/// Add a new data source to the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Unique data source code (e.g., "TEST_DS")
/// * `retention_level` - Optional retention level ("Remember" or "Forget", default: "Remember")
/// * `conversational` - Optional conversational flag ("Yes" or "No", default: "No")
/// * `reliability` - Optional reliability score (default: 1)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `AlreadyExists` if data source code already exists
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn add_data_source(
    config_json: &str,
    code: &str,
    retention_level: Option<&str>,
    conversational: Option<&str>,
    reliability: Option<i64>,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let dsrcs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_DSRC"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    // Check for duplicates
    let code_upper = code.to_uppercase();
    if dsrcs
        .iter()
        .any(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Data source already exists: {}",
            code_upper
        )));
    }

    let next_id = helpers::get_next_id_from_array(dsrcs, "DSRC_ID")?;

    // Use parameters or defaults (matching Python behavior)
    let retention = retention_level.unwrap_or("Remember");
    let conversational_flag = conversational.unwrap_or("No");
    let reliability_score = reliability.unwrap_or(1);

    dsrcs.push(json!({
        "DSRC_ID": next_id,
        "DSRC_CODE": code_upper.clone(),
        "DSRC_DESC": code_upper,  // Python uses code as description, not formatted string
        "DSRC_RELY": reliability_score,
        "RETENTION_LEVEL": retention,
        "CONVERSATIONAL": conversational_flag,
    }));

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete a data source from the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Data source code to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if data source doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn delete_data_source(config_json: &str, code: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let dsrcs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_DSRC"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    let code_upper = code.to_uppercase();
    let original_len = dsrcs.len();
    dsrcs.retain(|d| d["DSRC_CODE"].as_str() != Some(&code_upper));

    if dsrcs.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "Data source not found: {}",
            code_upper
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a specific data source by code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Data source code to retrieve
///
/// # Returns
/// JSON Value representing the data source
///
/// # Errors
/// - `NotFound` if data source doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn get_data_source(config_json: &str, code: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = code.to_uppercase();
    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DSRC"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?
        .iter()
        .find(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
        .cloned()
        .ok_or_else(|| SzConfigError::NotFound(format!("Data source not found: {}", code_upper)))
}

/// List all data sources
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing data sources in Python format
/// (with "id" and "dataSource" fields)
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn list_data_sources(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let dsrcs = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DSRC"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    Ok(dsrcs
        .iter()
        .map(|item| {
            json!({
                "id": item.get("DSRC_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "dataSource": item.get("DSRC_CODE").and_then(|v| v.as_str()).unwrap_or("")
            })
        })
        .collect())
}

/// Set (update) a data source's properties
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Data source code to update
/// * `updates` - JSON Value with fields to update
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if data source doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn set_data_source(config_json: &str, code: &str, updates: &Value) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = code.to_uppercase();
    let dsrcs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_DSRC"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    let dsrc = dsrcs
        .iter_mut()
        .find(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
        .ok_or_else(|| SzConfigError::NotFound(format!("Data source not found: {}", code_upper)))?;

    // Merge updates into existing record
    if let Some(updates_obj) = updates.as_object() {
        if let Some(dsrc_obj) = dsrc.as_object_mut() {
            for (key, value) in updates_obj {
                dsrc_obj.insert(key.clone(), value.clone());
            }
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}
