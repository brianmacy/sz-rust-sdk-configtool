use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an element
#[derive(Debug, Clone)]
pub struct AddElementParams<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
    pub data_type: Option<&'a str>,
    pub tokenized: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddElementParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("code".to_string()))?;

        Ok(Self {
            code,
            description: json.get("description").and_then(|v| v.as_str()),
            data_type: json.get("dataType").and_then(|v| v.as_str()),
            tokenized: json.get("tokenized").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting (updating) an element
#[derive(Debug, Clone)]
pub struct SetElementParams<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
    pub data_type: Option<&'a str>,
    pub tokenized: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for SetElementParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("code".to_string()))?;

        Ok(Self {
            code,
            description: json.get("description").and_then(|v| v.as_str()),
            data_type: json.get("dataType").and_then(|v| v.as_str()),
            tokenized: json.get("tokenized").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting a feature element
#[derive(Debug, Clone, Default)]
pub struct SetFeatureElementParams<'a> {
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: Option<i64>,
    pub display_level: Option<i64>,
    pub display_delim: Option<&'a str>,
    pub derived: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for SetFeatureElementParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let ftype_id = json
            .get("ftypeId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("ftypeId".to_string()))?;

        let felem_id = json
            .get("felemId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("felemId".to_string()))?;

        Ok(Self {
            ftype_id,
            felem_id,
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
            display_level: json.get("displayLevel").and_then(|v| v.as_i64()),
            display_delim: json.get("displayDelim").and_then(|v| v.as_str()),
            derived: json.get("derived").and_then(|v| v.as_str()),
        })
    }
}

/// Add a new element (CFG_FELEM record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Element parameters (code required, others optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_element(config_json: &str, params: AddElementParams) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = params.code.to_uppercase();

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

    // Build record from params
    let mut new_record = json!({
        "FELEM_ID": felem_id,
        "FELEM_CODE": code_upper.clone(),
    });

    if let Some(obj) = new_record.as_object_mut() {
        if let Some(desc) = params.description {
            obj.insert("FELEM_DESC".to_string(), json!(desc));
        } else {
            obj.insert("FELEM_DESC".to_string(), json!(code_upper));
        }
        if let Some(dt) = params.data_type {
            obj.insert("DATA_TYPE".to_string(), json!(dt));
        }
        if let Some(tok) = params.tokenized {
            obj.insert("TOKENIZED".to_string(), json!(tok));
        }
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
/// * `params` - Element parameters (code required to identify, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_element(config_json: &str, params: SetElementParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = params.code.to_uppercase();

    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    // Find and update the element
    let felem = felem_array
        .iter_mut()
        .find(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
        .ok_or_else(|| SzConfigError::NotFound(format!("Element: {}", code_upper.clone())))?;

    // Update fields from params
    if let Some(dest_obj) = felem.as_object_mut() {
        if let Some(desc) = params.description {
            dest_obj.insert("FELEM_DESC".to_string(), json!(desc));
        }
        if let Some(dt) = params.data_type {
            dest_obj.insert("DATA_TYPE".to_string(), json!(dt));
        }
        if let Some(tok) = params.tokenized {
            dest_obj.insert("TOKENIZED".to_string(), json!(tok));
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set feature element (update FBOM record)
///
/// This function updates feature-to-element mappings in CFG_FBOM.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature element parameters (ftype_id, felem_id required; updates optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_feature_element(
    config_json: &str,
    params: SetFeatureElementParams,
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
            item["FTYPE_ID"].as_i64() == Some(params.ftype_id)
                && item["FELEM_ID"].as_i64() == Some(params.felem_id)
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Feature element mapping not found: FTYPE_ID={}, FELEM_ID={}",
                params.ftype_id, params.felem_id
            ))
        })?;

    // Update fields if provided
    if let Some(order) = params.exec_order {
        fbom["EXEC_ORDER"] = json!(order);
    }
    if let Some(level) = params.display_level {
        fbom["DISPLAY_LEVEL"] = json!(level);
    }
    if let Some(delim) = params.display_delim {
        fbom["DISPLAY_DELIM"] = json!(delim);
    }
    if let Some(der) = params.derived {
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
        SetFeatureElementParams {
            ftype_id,
            felem_id,
            display_level: Some(display_level),
            ..Default::default()
        },
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
        SetFeatureElementParams {
            ftype_id,
            felem_id,
            derived: Some(derived),
            ..Default::default()
        },
    )
}
