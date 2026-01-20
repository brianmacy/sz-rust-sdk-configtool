//! Config Section operations
//!
//! Functions for managing top-level configuration sections in G2_CONFIG.
//! These functions allow adding, removing, and querying configuration sections.

use crate::error::{Result, SzConfigError};
use serde_json::{Value, json};

/// Add a new configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to add
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {}}"#;
/// let modified = config_sections::add_config_section(config, "CFG_CUSTOM").unwrap();
/// ```
pub fn add_config_section(config_json: &str, section_name: &str) -> Result<String> {
    let section_name = section_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;

    // Check if section already exists
    if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if g2_config.get(&section_name).is_some() {
            return Err(SzConfigError::AlreadyExists(
                "Configuration section already exists".to_string(),
            ));
        }
    }

    // Add new section as empty array
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(obj) = g2_config.as_object_mut() {
            obj.insert(section_name.clone(), json!([]));
        }
    } else {
        return Err(SzConfigError::NotFound(
            "G2_CONFIG section not found in configuration".to_string(),
        ));
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Remove a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to remove
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_CUSTOM": []}}"#;
/// let modified = config_sections::remove_config_section(config, "CFG_CUSTOM").unwrap();
/// ```
pub fn remove_config_section(config_json: &str, section_name: &str) -> Result<String> {
    let section_name = section_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut removed = false;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(g2_config_obj) = g2_config.as_object_mut() {
            if g2_config_obj.remove(&section_name).is_some() {
                removed = true;
            }
        }
    }

    if !removed {
        return Err(SzConfigError::NotFound(format!(
            "Config section not found: {}",
            section_name
        )));
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Get items from a configuration section with optional filtering
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to get
/// * `filter` - Optional filter string to search in records
///
/// # Returns
///
/// Returns a vector of items from the section
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}]}}"#;
/// let items = config_sections::get_config_section(config, "CFG_ATTR", None).unwrap();
/// assert_eq!(items.len(), 1);
/// ```
pub fn get_config_section(
    config_json: &str,
    section_name: &str,
    filter: Option<&str>,
) -> Result<Vec<Value>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    // Check if section exists
    let section_data = config_data
        .get("G2_CONFIG")
        .and_then(|g2| g2.get(section_name))
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Configuration section '{}' not found",
                section_name
            ))
        })?;

    // Handle empty section
    if section_data.is_null()
        || (section_data.is_array() && section_data.as_array().unwrap().is_empty())
    {
        return Ok(Vec::new());
    }

    // Apply filter if provided
    let output_data = if let Some(filter_str) = filter {
        if let Some(array) = section_data.as_array() {
            array
                .iter()
                .filter(|record| {
                    serde_json::to_string(record)
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&filter_str.to_lowercase())
                })
                .cloned()
                .collect()
        } else {
            // Not an array, just check the single value
            let as_string = serde_json::to_string(section_data)
                .unwrap_or_default()
                .to_lowercase();
            if as_string.contains(&filter_str.to_lowercase()) {
                vec![section_data.clone()]
            } else {
                Vec::new()
            }
        }
    } else {
        // No filter - return all
        if let Some(array) = section_data.as_array() {
            array.clone()
        } else {
            vec![section_data.clone()]
        }
    };

    Ok(output_data)
}

/// List all configuration section names
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns a vector of section names
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [], "CFG_DSRC": []}}"#;
/// let sections = config_sections::list_config_sections(config).unwrap();
/// assert!(sections.contains(&"CFG_ATTR".to_string()));
/// ```
pub fn list_config_sections(config_json: &str) -> Result<Vec<String>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let sections = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(obj) = g2_config.as_object() {
            obj.keys().cloned().collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(sections)
}

/// Add a field to all items in a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Section name
/// * `field_name` - Field name to add
/// * `field_value` - Value for the field
///
/// # Returns
///
/// Returns `(modified_config, item_count)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}]}}"#;
/// let (modified, count) = config_sections::add_config_section_field(
///     config,
///     "CFG_ATTR",
///     "NEW_FIELD",
///     &json!("default")
/// ).unwrap();
/// assert_eq!(count, 1);
/// ```
pub fn add_config_section_field(
    config_json: &str,
    section_name: &str,
    field_name: &str,
    field_value: &Value,
) -> Result<(String, usize)> {
    let section_name = section_name.to_uppercase();
    let field_name = field_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut item_count = 0;

    // Navigate to section and add field to all items in the array
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(section_array) = g2_config
            .get_mut(&section_name)
            .and_then(|v| v.as_array_mut())
        {
            for item in section_array.iter_mut() {
                if let Some(item_obj) = item.as_object_mut() {
                    item_obj.insert(field_name.clone(), field_value.clone());
                    item_count += 1;
                }
            }
        } else {
            return Err(SzConfigError::NotFound(format!(
                "Section not found or not an array: {}",
                section_name
            )));
        }
    }

    Ok((serde_json::to_string(&config_data)?, item_count))
}

/// Remove a field from all items in a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Section name
/// * `field_name` - Field name to remove
///
/// # Returns
///
/// Returns `(modified_config, item_count)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME", "OLD_FIELD": "value"}]}}"#;
/// let (modified, count) = config_sections::remove_config_section_field(
///     config,
///     "CFG_ATTR",
///     "OLD_FIELD"
/// ).unwrap();
/// assert_eq!(count, 1);
/// ```
pub fn remove_config_section_field(
    config_json: &str,
    section_name: &str,
    field_name: &str,
) -> Result<(String, usize)> {
    let section_name = section_name.to_uppercase();
    let field_name = field_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut item_count = 0;

    // Navigate to section and remove field from all items in the array
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(section_array) = g2_config
            .get_mut(&section_name)
            .and_then(|v| v.as_array_mut())
        {
            for item in section_array.iter_mut() {
                if let Some(item_obj) = item.as_object_mut() {
                    if item_obj.remove(&field_name).is_some() {
                        item_count += 1;
                    }
                }
            }
        } else {
            return Err(SzConfigError::NotFound(format!(
                "Section not found or not an array: {}",
                section_name
            )));
        }
    }

    Ok((serde_json::to_string(&config_data)?, item_count))
}
