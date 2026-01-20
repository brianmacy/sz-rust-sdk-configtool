//! Hash operations for SYS_OOM section
//!
//! Functions for managing NAME_HASH and SSN_LAST4_HASH arrays in the SYS_OOM section.
//! These hashes control special matching behavior for common names and SSN last 4 digits.

use crate::error::{Result, SzConfigError};
use serde_json::{Value, json};

/// Add a name to the NAME_HASH array
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `name` - Name value to add
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::hashes;
///
/// let config = r#"{"G2_CONFIG": {"SYS_OOM": {"NAME_HASH": []}}}"#;
/// let modified = hashes::add_to_name_hash(config, "JOHN").unwrap();
/// ```
pub fn add_to_name_hash(config_json: &str, name: &str) -> Result<String> {
    let mut config_data: Value = serde_json::from_str(config_json)?;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(sys_oom) = g2_config.get_mut("SYS_OOM") {
            if let Some(sys_oom_obj) = sys_oom.as_object_mut() {
                // Get or create NAME_HASH array
                let name_hash = sys_oom_obj.entry("NAME_HASH").or_insert(json!([]));

                if let Some(name_hash_arr) = name_hash.as_array_mut() {
                    // Check if already exists
                    if name_hash_arr.iter().any(|v| v.as_str() == Some(name)) {
                        return Err(SzConfigError::AlreadyExists(format!(
                            "Name already in hash: {}",
                            name
                        )));
                    }

                    // Add to hash
                    name_hash_arr.push(json!(name));
                } else {
                    return Err(SzConfigError::InvalidConfig(
                        "NAME_HASH is not an array".to_string(),
                    ));
                }
            }
        } else {
            return Err(SzConfigError::NotFound(
                "SYS_OOM section not found".to_string(),
            ));
        }
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Delete a name from the NAME_HASH array
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `name` - Name value to remove
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::hashes;
///
/// let config = r#"{"G2_CONFIG": {"SYS_OOM": {"NAME_HASH": ["JOHN"]}}}"#;
/// let modified = hashes::delete_from_name_hash(config, "JOHN").unwrap();
/// ```
pub fn delete_from_name_hash(config_json: &str, name: &str) -> Result<String> {
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut found = false;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(sys_oom) = g2_config.get_mut("SYS_OOM") {
            if let Some(sys_oom_obj) = sys_oom.as_object_mut() {
                if let Some(name_hash) = sys_oom_obj.get_mut("NAME_HASH") {
                    if let Some(name_hash_arr) = name_hash.as_array_mut() {
                        let before_len = name_hash_arr.len();
                        name_hash_arr.retain(|v| v.as_str() != Some(name));
                        found = name_hash_arr.len() < before_len;
                    }
                }
            }
        }
    }

    if !found {
        return Err(SzConfigError::NotFound(format!(
            "Name not found in hash: {}",
            name
        )));
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Add a name to the SSN_LAST4_HASH array
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `name` - Name value to add
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::hashes;
///
/// let config = r#"{"G2_CONFIG": {"SYS_OOM": {"SSN_LAST4_HASH": []}}}"#;
/// let modified = hashes::add_to_ssn_last4_hash(config, "SMITH").unwrap();
/// ```
pub fn add_to_ssn_last4_hash(config_json: &str, name: &str) -> Result<String> {
    let mut config_data: Value = serde_json::from_str(config_json)?;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(sys_oom) = g2_config.get_mut("SYS_OOM") {
            if let Some(sys_oom_obj) = sys_oom.as_object_mut() {
                // Get or create SSN_LAST4_HASH array
                let ssn_last4_hash = sys_oom_obj.entry("SSN_LAST4_HASH").or_insert(json!([]));

                if let Some(ssn_last4_hash_arr) = ssn_last4_hash.as_array_mut() {
                    // Check if already exists
                    if ssn_last4_hash_arr.iter().any(|v| v.as_str() == Some(name)) {
                        return Err(SzConfigError::AlreadyExists(format!(
                            "Name already in SSN_LAST4_HASH: {}",
                            name
                        )));
                    }

                    // Add to hash
                    ssn_last4_hash_arr.push(json!(name));
                } else {
                    return Err(SzConfigError::InvalidConfig(
                        "SSN_LAST4_HASH is not an array".to_string(),
                    ));
                }
            }
        } else {
            return Err(SzConfigError::NotFound(
                "SYS_OOM section not found".to_string(),
            ));
        }
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Delete a name from the SSN_LAST4_HASH array
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `name` - Name value to remove
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::hashes;
///
/// let config = r#"{"G2_CONFIG": {"SYS_OOM": {"SSN_LAST4_HASH": ["SMITH"]}}}"#;
/// let modified = hashes::delete_from_ssn_last4_hash(config, "SMITH").unwrap();
/// ```
pub fn delete_from_ssn_last4_hash(config_json: &str, name: &str) -> Result<String> {
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut removed = false;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(sys_oom) = g2_config.get_mut("SYS_OOM") {
            if let Some(sys_oom_obj) = sys_oom.as_object_mut() {
                if let Some(ssn_last4_hash) = sys_oom_obj.get_mut("SSN_LAST4_HASH") {
                    if let Some(ssn_last4_hash_arr) = ssn_last4_hash.as_array_mut() {
                        // Find and remove the name
                        if let Some(pos) = ssn_last4_hash_arr
                            .iter()
                            .position(|v| v.as_str() == Some(name))
                        {
                            ssn_last4_hash_arr.remove(pos);
                            removed = true;
                        }
                    } else {
                        return Err(SzConfigError::InvalidConfig(
                            "SSN_LAST4_HASH is not an array".to_string(),
                        ));
                    }
                } else {
                    return Err(SzConfigError::NotFound(
                        "SSN_LAST4_HASH not found in SYS_OOM".to_string(),
                    ));
                }
            }
        } else {
            return Err(SzConfigError::NotFound(
                "SYS_OOM section not found".to_string(),
            ));
        }
    }

    if !removed {
        return Err(SzConfigError::NotFound(format!(
            "Name not found in SSN_LAST4_HASH: {}",
            name
        )));
    }

    Ok(serde_json::to_string(&config_data)?)
}
