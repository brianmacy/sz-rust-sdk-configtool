use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// Get the next available ID for a config array
///
/// Finds the maximum value of the specified ID field and returns max + 1
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
///
/// # Returns
/// Next available ID value
pub fn get_next_id_from_array(array: &[Value], id_field: &str) -> Result<i64> {
    let max_id = array
        .iter()
        .filter_map(|item| item.get(id_field))
        .filter_map(|v| v.as_i64())
        .max()
        .unwrap_or(0);

    Ok(max_id + 1)
}

/// Get the next available ID for a config section with optional seed value
///
/// Navigates to a config section using a path and finds the next available ID.
/// Useful for user-created items that should start at a specific ID (e.g., 1000).
///
/// # Arguments
/// * `config_data` - Parsed configuration JSON Value
/// * `section_path` - Dot-separated path (e.g., "G2_CONFIG.CFG_SFCALL")
/// * `id_field` - Name of the ID field (e.g., "SFCALL_ID")
/// * `seed_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// Next available ID value, at least seed_value
///
/// # Errors
/// Returns error if section path not found
pub fn get_next_id(
    config_data: &Value,
    section_path: &str,
    id_field: &str,
    seed_value: i64,
) -> Result<i64> {
    // Parse section path (e.g., "G2_CONFIG.CFG_SFCALL")
    let parts: Vec<&str> = section_path.split('.').collect();

    let mut current = config_data;
    for part in &parts {
        current = current.get(part).ok_or_else(|| {
            SzConfigError::MissingSection(format!("Section path '{}' not found", section_path))
        })?;
    }

    // Get max ID from array
    let max_id = if let Some(items) = current.as_array() {
        items
            .iter()
            .filter_map(|item| item.get(id_field).and_then(|v| v.as_i64()))
            .max()
            .unwrap_or(seed_value - 1)
    } else {
        seed_value - 1
    };

    Ok(std::cmp::max(max_id + 1, seed_value))
}

/// Get the next available ID for a config array with minimum value
///
/// Finds the maximum value of the specified ID field and returns max(max_id + 1, min_value)
/// This is useful for user-created items that should start at a high ID (e.g., 1000)
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "FTYPE_ID", "FELEM_ID")
/// * `min_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// Next available ID value, at least min_value
pub fn get_next_id_with_min(array: &[Value], id_field: &str, min_value: i64) -> Result<i64> {
    let max_id = array
        .iter()
        .filter_map(|item| item.get(id_field))
        .filter_map(|v| v.as_i64())
        .max()
        .unwrap_or(min_value - 1);

    Ok(std::cmp::max(max_id + 1, min_value))
}

/// Check if an ID is already taken in a config array
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
/// * `id_value` - ID value to check
///
/// # Returns
/// true if ID is taken, false otherwise
pub fn is_id_taken(array: &[Value], id_field: &str, id_value: i64) -> bool {
    array
        .iter()
        .any(|item| item.get(id_field).and_then(|v| v.as_i64()) == Some(id_value))
}

/// Get the next available ID or use desired ID if specified and available
///
/// Matches Python's getDesiredValueOrNext behavior:
/// - If desired_id is Some and available, returns it
/// - If desired_id is Some but taken, returns error
/// - If desired_id is None, returns next available ID
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
/// * `desired_id` - Optional user-specified ID
/// * `min_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// ID to use (either desired_id or next available)
///
/// # Errors
/// Returns error if desired_id is already taken
pub fn get_desired_or_next_id(
    array: &[Value],
    id_field: &str,
    desired_id: Option<i64>,
    min_value: i64,
) -> Result<i64> {
    if let Some(id) = desired_id {
        if id > 0 {
            if is_id_taken(array, id_field, id) {
                return Err(SzConfigError::AlreadyExists(format!(
                    "The specified ID {} is already taken",
                    id
                )));
            }
            return Ok(id);
        }
    }

    // No desired ID or invalid, get next available
    get_next_id_with_min(array, id_field, min_value)
}

/// Get the next available ID or use desired ID (for config sections)
///
/// Same as get_desired_or_next_id but works with section paths
///
/// # Arguments
/// * `config_data` - Parsed configuration JSON Value
/// * `section_path` - Dot-separated path (e.g., "G2_CONFIG.CFG_SFCALL")
/// * `id_field` - Name of the ID field (e.g., "SFCALL_ID")
/// * `desired_id` - Optional user-specified ID
/// * `seed_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// ID to use (either desired_id or next available)
///
/// # Errors
/// Returns error if section not found or desired_id is already taken
pub fn get_desired_or_next_id_from_section(
    config_data: &Value,
    section_path: &str,
    id_field: &str,
    desired_id: Option<i64>,
    seed_value: i64,
) -> Result<i64> {
    // Parse section path
    let parts: Vec<&str> = section_path.split('.').collect();

    let mut current = config_data;
    for part in &parts {
        current = current.get(part).ok_or_else(|| {
            SzConfigError::MissingSection(format!("Section path '{}' not found", section_path))
        })?;
    }

    let array = current.as_array().ok_or_else(|| {
        SzConfigError::MissingSection(format!("Section '{}' is not an array", section_path))
    })?;

    get_desired_or_next_id(array, id_field, desired_id, seed_value)
}

/// Find item in config array by field value
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `field` - Field name to search
/// * `value` - Value to match
///
/// # Returns
/// Reference to matching item, or None if not found
pub fn find_in_array<'a>(array: &'a [Value], field: &str, value: &str) -> Option<&'a Value> {
    array.iter().find(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s == value)
            .unwrap_or(false)
    })
}

/// Get mutable reference to item in config array
///
/// # Arguments
/// * `array` - Mutable array of configuration items
/// * `field` - Field name to search
/// * `value` - Value to match
///
/// # Returns
/// Mutable reference to matching item, or None if not found
pub fn find_in_array_mut<'a>(
    array: &'a mut [Value],
    field: &str,
    value: &str,
) -> Option<&'a mut Value> {
    array.iter_mut().find(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s == value)
            .unwrap_or(false)
    })
}

/// Add item to config array (generic)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `item` - JSON Value to add
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if section doesn't exist
pub fn add_to_config_array(config_json: &str, section: &str, item: Value) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    array.push(item);

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete item from config array by field value
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match for deletion
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if section doesn't exist
/// - `NotFound` if no item matches the criteria
pub fn delete_from_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    let original_len = array.len();
    array.retain(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s != value)
            .unwrap_or(true)
    });

    if array.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "{} '{}' not found",
            section, value
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Find item in config array by field value (returns owned value)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match
///
/// # Returns
/// Cloned item if found, None otherwise
///
/// # Errors
/// - `JsonParse` if config_json is invalid
pub fn find_in_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<Option<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get("G2_CONFIG")
        .and_then(|g| g.get(section))
        .and_then(|v| v.as_array());

    if let Some(arr) = array {
        let item = arr.iter().find(|item| {
            item.get(field)
                .and_then(|v| v.as_str())
                .map(|s| s == value)
                .or_else(|| {
                    // Also try numeric comparison
                    item.get(field)
                        .and_then(|v| v.as_i64())
                        .and_then(|id| value.parse::<i64>().ok().map(|val| id == val))
                })
                .unwrap_or(false)
        });
        Ok(item.cloned())
    } else {
        Ok(None)
    }
}

/// Alias for delete_from_config_array for compatibility
pub fn remove_from_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<String> {
    delete_from_config_array(config_json, section, field, value)
}

/// Update item in config array (complete replacement)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match
/// * `new_item` - Complete new item value (replaces old item entirely)
///
/// # Returns
/// Modified configuration JSON string
pub fn update_in_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
    new_item: Value,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    let item = find_in_array_mut(array, field, value)
        .ok_or_else(|| SzConfigError::NotFound(format!("{} '{}' not found", section, value)))?;

    // Replace the entire item
    *item = new_item;

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// List all items from a config array
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
///
/// # Returns
/// Vector of all items in the section
///
/// # Errors
/// - `JsonParse` if config_json is invalid
pub fn list_from_config_array(config_json: &str, section: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let items = if let Some(g2_config) = config.get("G2_CONFIG") {
        if let Some(array) = g2_config.get(section).and_then(|v| v.as_array()) {
            array.clone()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Lookup feature ID by feature code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code` - Feature code to look up (case-insensitive)
///
/// # Returns
/// Feature ID (FTYPE_ID)
///
/// # Errors
/// Returns error if feature not found or JSON is invalid
pub fn lookup_feature_id(config_json: &str, feature_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("FTYPE_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(feature_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("FTYPE_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature '{}' not found", feature_code)))
}

/// Lookup element ID by element code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `element_code` - Element code to look up (case-insensitive)
///
/// # Returns
/// Element ID (FELEM_ID)
///
/// # Errors
/// Returns error if element not found or JSON is invalid
pub fn lookup_element_id(config_json: &str, element_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FELEM"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|e| {
                    e.get("FELEM_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(element_code))
                        .unwrap_or(false)
                })
                .and_then(|e| e.get("FELEM_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Element '{}' not found", element_code)))
}

/// Lookup standardize function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (SFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_sfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("SFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("SFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Standardize function '{}' not found", func_code))
        })
}

/// Lookup expression function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (EFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_efunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("EFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("EFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Expression function '{}' not found", func_code))
        })
}

/// Lookup comparison function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (CFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_cfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("CFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("CFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Comparison function '{}' not found", func_code))
        })
}

/// Lookup distinct function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (DFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_dfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("DFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("DFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Distinct function '{}' not found", func_code))
        })
}

/// Lookup generic plan ID by plan code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `plan_code` - Plan code to look up (case-insensitive, e.g., "INGEST", "SEARCH")
///
/// # Returns
/// Plan ID (GPLAN_ID)
///
/// # Errors
/// Returns error if plan not found or JSON is invalid
pub fn lookup_gplan_id(config_json: &str, plan_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GPLAN"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|p| {
                    p.get("GPLAN_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(plan_code))
                        .unwrap_or(false)
                })
                .and_then(|p| p.get("GPLAN_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan '{}' not found", plan_code)))
}

/// Internal: Lookup generic plan code by plan ID (for FFI use)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `gplan_id` - Plan ID to look up
///
/// # Returns
/// Plan code (GPLAN_CODE)
///
/// # Errors
/// Returns error if plan not found or JSON is invalid
pub(crate) fn lookup_gplan_code(config_json: &str, gplan_id: i64) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GPLAN"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|p| p.get("GPLAN_ID").and_then(|v| v.as_i64()) == Some(gplan_id))
                .and_then(|p| p.get("GPLAN_CODE"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan ID: {}", gplan_id)))
}
