use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

/// Add a new element (CFG_FELEM record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `felem_code` - Element code
/// * `felem_config` - JSON object with element fields (FELEM_DESC, DATA_TYPE, etc.)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_element(config_json: &str, felem_code: &str, felem_config: &Value) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = felem_code.to_uppercase();

    // Check if already exists
    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    if felem_array
        .iter()
        .any(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Element already exists: {}",
            code_upper
        )));
    }

    // Get next ID
    let felem_id = helpers::get_next_id_with_min(felem_array, "FELEM_ID", 1000)?;

    // Build record with provided config plus ID and CODE
    let mut new_record = felem_config.clone();
    if let Some(obj) = new_record.as_object_mut() {
        obj.insert("FELEM_ID".to_string(), json!(felem_id));
        obj.insert("FELEM_CODE".to_string(), json!(code_upper));
    }

    helpers::add_to_config_array(config_json, "CFG_FELEM", new_record)
}

/// Delete an element (CFG_FELEM record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `felem_code` - Element code
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_element(config_json: &str, felem_code: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = felem_code.to_uppercase();

    // Verify exists
    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    if !felem_array
        .iter()
        .any(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
    {
        return Err(SzConfigError::NotFound(format!(
            "Element not found: {}",
            code_upper
        )));
    }

    // Remove from array
    if let Some(array) = config["G2_CONFIG"]["CFG_FELEM"].as_array_mut() {
        array.retain(|e| e["FELEM_CODE"].as_str() != Some(code_upper.as_str()));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a specific element by code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `felem_code` - Element code
///
/// # Returns
/// JSON Value representing the element
pub fn get_element(config_json: &str, felem_code: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = felem_code.to_uppercase();

    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    felem_array
        .iter()
        .find(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
        .cloned()
        .ok_or_else(|| SzConfigError::NotFound(format!("Element not found: {}", code_upper)))
}

/// List all elements
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing elements with id, element, and datatype fields, sorted by FELEM_ID
pub fn list_elements(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    let mut result: Vec<Value> = felem_array
        .iter()
        .map(|item| {
            json!({
                "id": item["FELEM_ID"].as_i64().unwrap_or(0),
                "element": item["FELEM_CODE"].as_str().unwrap_or(""),
                "datatype": item["DATA_TYPE"].as_str().unwrap_or("")
            })
        })
        .collect();

    // Sort by element code (alphabetic) like Python
    result.sort_by_key(|e| e["element"].as_str().unwrap_or("").to_string());

    Ok(result)
}

/// Set (update) an element's properties
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `felem_code` - Element code
/// * `felem_config` - JSON object with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_element(config_json: &str, felem_code: &str, felem_config: &Value) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = felem_code.to_uppercase();

    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    // Find and update the element
    let felem = felem_array
        .iter_mut()
        .find(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
        .ok_or_else(|| SzConfigError::NotFound(format!("Element: {}", code_upper.clone())))?;

    // Merge config fields into existing record
    if let Some(src_obj) = felem_config.as_object() {
        if let Some(dest_obj) = felem.as_object_mut() {
            for (key, value) in src_obj {
                // Don't allow changing the CODE
                if key != "FELEM_CODE" && key != "felem_code" {
                    dest_obj.insert(key.clone(), value.clone());
                }
            }
        }
    }

    // Ensure CODE is preserved
    if let Some(obj) = felem.as_object_mut() {
        obj.insert("FELEM_CODE".to_string(), json!(code_upper));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set feature element (update FBOM record)
///
/// This function updates feature-to-element mappings in CFG_FBOM.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Element ID
/// * `exec_order` - Optional execution order
/// * `display_level` - Optional display level
/// * `display_delim` - Optional display delimiter
/// * `derived` - Optional derived flag
///
/// # Returns
/// Modified configuration JSON string
pub fn set_feature_element(
    config_json: &str,
    ftype_id: i64,
    felem_id: i64,
    exec_order: Option<i64>,
    display_level: Option<i64>,
    display_delim: Option<&str>,
    derived: Option<&str>,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    // Find the FBOM record
    let fbom = fbom_array
        .iter_mut()
        .find(|item| {
            item["FTYPE_ID"].as_i64() == Some(ftype_id)
                && item["FELEM_ID"].as_i64() == Some(felem_id)
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Feature element mapping not found: FTYPE_ID={}, FELEM_ID={}",
                ftype_id, felem_id
            ))
        })?;

    // Update fields if provided
    if let Some(order) = exec_order {
        fbom["EXEC_ORDER"] = json!(order);
    }
    if let Some(level) = display_level {
        fbom["DISPLAY_LEVEL"] = json!(level);
    }
    if let Some(delim) = display_delim {
        fbom["DISPLAY_DELIM"] = json!(delim);
    }
    if let Some(der) = derived {
        fbom["DERIVED"] = json!(der);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set feature element display level
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Element ID
/// * `display_level` - Display level value
///
/// # Returns
/// Modified configuration JSON string
pub fn set_feature_element_display_level(
    config_json: &str,
    ftype_id: i64,
    felem_id: i64,
    display_level: i64,
) -> Result<String> {
    set_feature_element(
        config_json,
        ftype_id,
        felem_id,
        None,
        Some(display_level),
        None,
        None,
    )
}

/// Set feature element derived flag
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Element ID
/// * `derived` - Derived flag value ("Yes" or "No")
///
/// # Returns
/// Modified configuration JSON string
pub fn set_feature_element_derived(
    config_json: &str,
    ftype_id: i64,
    felem_id: i64,
    derived: &str,
) -> Result<String> {
    set_feature_element(
        config_json,
        ftype_id,
        felem_id,
        None,
        None,
        None,
        Some(derived),
    )
}
