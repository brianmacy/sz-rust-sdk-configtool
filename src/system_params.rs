//! System Parameter operations
//!
//! Functions for managing system parameters in the configuration.
//! Currently supports the relationshipsBreakMatches parameter (BREAK_RES in RCLASS_ID=2).

use crate::error::{Result, SzConfigError};
use serde_json::Value;
use std::collections::HashMap;

/// List all system parameters
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns a HashMap of parameter names to values
///
/// # Example
///
/// ```
/// use sz_configtool_lib::system_params;
///
/// let config = r#"{"G2_CONFIG": {"CFG_RTYPE": [{"RCLASS_ID": 2, "BREAK_RES": 1}]}}"#;
/// let params = system_params::list_system_parameters(config).unwrap();
/// assert!(params.contains_key("relationshipsBreakMatches"));
/// ```
pub fn list_system_parameters(config_json: &str) -> Result<HashMap<String, String>> {
    let config_data: Value = serde_json::from_str(config_json)?;
    let mut params = HashMap::new();

    // Find RCLASS_ID == 2 in CFG_RTYPE to get BREAK_RES value
    if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(rtype_array) = g2_config.get("CFG_RTYPE").and_then(|v| v.as_array()) {
            for rtype in rtype_array {
                if let Some(rclass_id) = rtype.get("RCLASS_ID").and_then(|v| v.as_i64()) {
                    if rclass_id == 2 {
                        if let Some(break_res) = rtype.get("BREAK_RES").and_then(|v| v.as_i64()) {
                            params.insert(
                                "relationshipsBreakMatches".to_string(),
                                break_res.to_string(),
                            );
                        }
                        break;
                    }
                }
            }
        }
    }

    Ok(params)
}

/// Set a system parameter
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `parameter_name` - Parameter name (e.g., "relationshipsBreakMatches")
/// * `parameter_value` - Parameter value
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::system_params;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_RTYPE": [{"RCLASS_ID": 2, "BREAK_RES": 0}]}}"#;
/// let modified = system_params::set_system_parameter(config, "relationshipsBreakMatches", &json!(1)).unwrap();
/// ```
pub fn set_system_parameter(
    config_json: &str,
    parameter_name: &str,
    parameter_value: &Value,
) -> Result<String> {
    let parameter_name = parameter_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut found = false;

    // Currently, the main system parameter is "relationshipsBreakMatches" which maps to BREAK_RES in RCLASS_ID=2
    if parameter_name == "RELATIONSHIPSBREAKMATCHES"
        || parameter_name == "RELATIONSHIPS_BREAK_MATCHES"
    {
        if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
            if let Some(rtype_array) = g2_config
                .get_mut("CFG_RTYPE")
                .and_then(|v| v.as_array_mut())
            {
                for rtype in rtype_array.iter_mut() {
                    if let Some(rclass_id) = rtype.get("RCLASS_ID").and_then(|v| v.as_i64()) {
                        if rclass_id == 2 {
                            if let Some(rtype_obj) = rtype.as_object_mut() {
                                rtype_obj.insert("BREAK_RES".to_string(), parameter_value.clone());
                                found = true;
                            }
                            break;
                        }
                    }
                }
            }
        }
    } else {
        return Err(SzConfigError::InvalidConfig(format!(
            "Unknown system parameter: {}",
            parameter_name
        )));
    }

    if !found {
        return Err(SzConfigError::NotFound(format!(
            "Failed to set system parameter: {}",
            parameter_name
        )));
    }

    Ok(serde_json::to_string(&config_data)?)
}
