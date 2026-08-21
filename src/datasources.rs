use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_DSRC row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted. The Senzing engine's config loader requires every key to be
/// present, so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
struct DsrcRow {
    #[serde(rename = "DSRC_ID")]
    dsrc_id: i64,
    #[serde(rename = "DSRC_CODE")]
    dsrc_code: String,
    #[serde(rename = "DSRC_DESC")]
    dsrc_desc: String,
    #[serde(rename = "RETENTION_LEVEL")]
    retention_level: String,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a data source
#[derive(Debug, Clone, Default)]
pub struct AddDataSourceParams<'a> {
    pub code: &'a str,
    pub retention_level: Option<&'a str>,
}

/// Parameters for setting (updating) a data source
#[derive(Debug, Clone, Default)]
pub struct SetDataSourceParams<'a> {
    pub code: &'a str,
    pub retention_level: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddDataSourceParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("code".to_string()))?;

        Ok(Self {
            code,
            retention_level: json.get("retentionLevel").and_then(|v| v.as_str()),
        })
    }
}

impl<'a> TryFrom<&'a Value> for SetDataSourceParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("code".to_string()))?;

        Ok(Self {
            code,
            retention_level: json.get("retentionLevel").and_then(|v| v.as_str()),
        })
    }
}

/// Add a new data source to the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Data source parameters (code required, others optional)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `AlreadyExists` if data source code already exists
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn add_data_source(config_json: &str, params: AddDataSourceParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let dsrcs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_DSRC"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    // Check for duplicates
    let code_upper = params.code.to_uppercase();
    if dsrcs
        .iter()
        .any(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Data source already exists: {code_upper}"
        )));
    }

    let next_id = helpers::get_next_id_from_array(dsrcs, "DSRC_ID")?;

    // Validate and use parameters or defaults (case-insensitive with normalization)
    let retention = if let Some(level) = params.retention_level {
        // Validate retentionLevel domain (case-insensitive)
        let level_upper = level.to_uppercase();
        match level_upper.as_str() {
            "REMEMBER" => "Remember",
            "FORGET" => "Forget",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid RETENTIONLEVEL value '{level}'. Must be 'Remember' or 'Forget'"
                )));
            }
        }
    } else {
        "Remember"
    };

    // Build a complete row via DsrcRow so every CFG_DSRC key is present.
    // Python uses the code as the description (not a formatted string).
    let row = DsrcRow {
        dsrc_id: next_id,
        dsrc_code: code_upper.clone(),
        dsrc_desc: code_upper,
        retention_level: retention.to_string(),
    };
    dsrcs.push(serde_json::to_value(&row)?);

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
/// - `InvalidInput` if attempting to delete system datasource (ID <= 2)
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

    // Check if datasource exists and get its ID for protection check
    let dsrc_to_delete = dsrcs
        .iter()
        .find(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
        .ok_or_else(|| SzConfigError::NotFound(format!("Data source not found: {code_upper}")))?;

    // Protect system datasources (Python parity: if dsrc_record["DSRC_ID"] <= 2)
    if let Some(dsrc_id) = dsrc_to_delete.get("DSRC_ID").and_then(|v| v.as_i64())
        && dsrc_id <= 2
    {
        return Err(SzConfigError::InvalidInput(format!(
            "The {code_upper} data source cannot be deleted"
        )));
    }

    // Safe to delete
    let original_len = dsrcs.len();
    dsrcs.retain(|d| d["DSRC_CODE"].as_str() != Some(&code_upper));

    if dsrcs.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "Data source not found: {code_upper}"
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Data source not found: {code_upper}")))
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
/// * `params` - Data source parameters (code required, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if data source doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_DSRC section doesn't exist
pub fn set_data_source(config_json: &str, params: SetDataSourceParams) -> Result<String> {
    // In-place update of a complete existing row; all keys preserved.
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = params.code.to_uppercase();
    let dsrcs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_DSRC"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DSRC".to_string()))?;

    let dsrc = dsrcs
        .iter_mut()
        .find(|d| d["DSRC_CODE"].as_str() == Some(&code_upper))
        .ok_or_else(|| SzConfigError::NotFound(format!("Data source not found: {code_upper}")))?;

    // Update fields if provided
    if let Some(dsrc_obj) = dsrc.as_object_mut()
        && let Some(retention) = params.retention_level
    {
        dsrc_obj.insert("RETENTION_LEVEL".to_string(), json!(retention));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DSRC_KEYS: [&str; 4] = ["DSRC_ID", "DSRC_CODE", "DSRC_DESC", "RETENTION_LEVEL"];

    const TEST_CONFIG: &str = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;

    #[test]
    fn test_add_data_source_emits_all_keys() {
        let params = AddDataSourceParams {
            code: "customers",
            retention_level: None,
        };

        let modified = add_data_source(TEST_CONFIG, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let dsrc = &value["G2_CONFIG"]["CFG_DSRC"][0];
        let obj = dsrc.as_object().unwrap();

        for key in DSRC_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(dsrc["DSRC_CODE"], json!("CUSTOMERS"));
        assert_eq!(dsrc["DSRC_DESC"], json!("CUSTOMERS"));
        assert_eq!(dsrc["RETENTION_LEVEL"], json!("Remember"));
    }

    #[test]
    fn test_add_data_source_with_retention() {
        let params = AddDataSourceParams {
            code: "watchlist",
            retention_level: Some("forget"),
        };

        let modified = add_data_source(TEST_CONFIG, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let dsrc = &value["G2_CONFIG"]["CFG_DSRC"][0];
        let obj = dsrc.as_object().unwrap();

        for key in DSRC_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(dsrc["RETENTION_LEVEL"], json!("Forget"));
    }
}
