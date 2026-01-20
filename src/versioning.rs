//! Version operations
//!
//! Functions for managing configuration version information.
//! Includes VERSION and COMPATIBILITY_VERSION fields.

use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// Get the configuration version
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns the VERSION string on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::versioning;
///
/// let config = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {"VERSION": "4.0.0"}}}"#;
/// let version = versioning::get_version(config).unwrap();
/// assert_eq!(version, "4.0.0");
/// ```
pub fn get_version(config_json: &str) -> Result<String> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let version = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CONFIG_BASE_VERSION"))
        .and_then(|bv| bv.get("VERSION"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::NotFound("VERSION not found in configuration".to_string()))?;

    Ok(version.to_string())
}

/// Get the compatibility version
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns the COMPATIBILITY_VERSION CONFIG_VERSION string on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::versioning;
///
/// let config = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {"COMPATIBILITY_VERSION": {"CONFIG_VERSION": "11"}}}}"#;
/// let version = versioning::get_compatibility_version(config).unwrap();
/// assert_eq!(version, "11");
/// ```
pub fn get_compatibility_version(config_json: &str) -> Result<String> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let version = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CONFIG_BASE_VERSION"))
        .and_then(|bv| bv.get("COMPATIBILITY_VERSION"))
        .and_then(|cv| cv.get("CONFIG_VERSION"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            SzConfigError::NotFound("COMPATIBILITY_VERSION not found in configuration".to_string())
        })?;

    Ok(version.to_string())
}

/// Update the compatibility version
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `new_version` - New version string
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::versioning;
///
/// let config = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {"COMPATIBILITY_VERSION": {"CONFIG_VERSION": "10"}}}}"#;
/// let modified = versioning::update_compatibility_version(config, "11").unwrap();
/// ```
pub fn update_compatibility_version(config_json: &str, new_version: &str) -> Result<String> {
    let mut config_data: Value = serde_json::from_str(config_json)?;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(base_version) = g2_config.get_mut("CONFIG_BASE_VERSION") {
            if let Some(compat_version) = base_version.get_mut("COMPATIBILITY_VERSION") {
                if let Some(compat_obj) = compat_version.as_object_mut() {
                    compat_obj.insert("CONFIG_VERSION".to_string(), serde_json::json!(new_version));
                } else {
                    return Err(SzConfigError::InvalidConfig(
                        "COMPATIBILITY_VERSION is not an object".to_string(),
                    ));
                }
            } else {
                return Err(SzConfigError::NotFound(
                    "COMPATIBILITY_VERSION not found".to_string(),
                ));
            }
        } else {
            return Err(SzConfigError::NotFound(
                "CONFIG_BASE_VERSION not found".to_string(),
            ));
        }
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Verify the compatibility version matches expected value
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `expected_version` - Expected version string
///
/// # Returns
///
/// Returns `(current_version, matches)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::versioning;
///
/// let config = r#"{"G2_CONFIG": {"CONFIG_BASE_VERSION": {"COMPATIBILITY_VERSION": {"CONFIG_VERSION": "11"}}}}"#;
/// let (current, matches) = versioning::verify_compatibility_version(config, "11").unwrap();
/// assert!(matches);
/// ```
pub fn verify_compatibility_version(
    config_json: &str,
    expected_version: &str,
) -> Result<(String, bool)> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let current_version = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CONFIG_BASE_VERSION"))
        .and_then(|bv| bv.get("COMPATIBILITY_VERSION"))
        .and_then(|cv| cv.get("CONFIG_VERSION"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::NotFound("CONFIG_VERSION not found".to_string()))?;

    let matches = current_version == expected_version;

    Ok((current_version.to_string(), matches))
}
