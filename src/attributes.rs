use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an attribute
#[derive(Debug, Clone)]
pub struct AddAttributeParams<'a> {
    pub attribute: &'a str,
    pub feature: &'a str,
    pub element: &'a str,
    pub class: &'a str,
    pub default_value: Option<&'a str>,
    pub internal: Option<&'a str>,
    pub required: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddAttributeParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            attribute: json
                .get("attribute")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("attribute".to_string()))?,
            feature: json
                .get("feature")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?,
            element: json
                .get("element")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("element".to_string()))?,
            class: json
                .get("class")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("class".to_string()))?,
            default_value: json.get("default").and_then(|v| v.as_str()),
            internal: json.get("internal").and_then(|v| v.as_str()),
            required: json.get("required").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting an attribute
#[derive(Debug, Clone, Default)]
pub struct SetAttributeParams<'a> {
    pub attribute: &'a str,
    pub internal: Option<&'a str>,
    pub required: Option<&'a str>,
    pub default_value: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for SetAttributeParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            attribute: json
                .get("attribute")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("attribute".to_string()))?,
            internal: json.get("internal").and_then(|v| v.as_str()),
            required: json.get("required").and_then(|v| v.as_str()),
            default_value: json.get("default").and_then(|v| v.as_str()),
        })
    }
}

/// Add a new attribute to the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Attribute parameters (attribute, feature, element, class required; others optional)
///
/// # Returns
/// Tuple of (modified_json, new_attribute_value) - returns both the modified config
/// and the newly created attribute for display purposes
///
/// # Errors
/// - `AlreadyExists` if attribute code already exists
/// - `InvalidInput` if attribute class is invalid
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if required sections don't exist
pub fn add_attribute(
    config_json: &str,
    params: AddAttributeParams,
) -> Result<(String, Value)> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate attribute class (matches Python line 173-181)
    let valid_classes = [
        "NAME",
        "ATTRIBUTE",
        "IDENTIFIER",
        "ADDRESS",
        "PHONE",
        "RELATIONSHIP",
        "OTHER",
    ];
    if !valid_classes.contains(&params.class) {
        return Err(SzConfigError::InvalidInput(format!(
            "Invalid attribute class '{}'. Must be one of: {}",
            params.class,
            valid_classes.join(", ")
        )));
    }

    let attribute_upper = params.attribute.to_uppercase();
    let feature_upper = params.feature.to_uppercase();
    let element_upper = params.element.to_uppercase();

    // Check if attribute already exists
    let attrs = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ATTR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_ATTR".to_string()))?;

    if attrs
        .iter()
        .any(|attr| attr["ATTR_CODE"].as_str() == Some(&attribute_upper))
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Attribute: {}",
            attribute_upper
        )));
    }

    // Get next ATTR_ID
    let next_attr_id = helpers::get_next_id_from_array(attrs, "ATTR_ID")?;

    // Create CFG_ATTR entry (matching Python lines 2342-2350)
    let new_attribute = json!({
        "ATTR_ID": next_attr_id,
        "ATTR_CODE": attribute_upper.clone(),
        "ATTR_CLASS": params.class,
        "FTYPE_CODE": feature_upper,  // Use actual feature code, not Null
        "FELEM_CODE": element_upper,  // Use actual element code, not Null
        "FELEM_REQ": params.required.unwrap_or("No"),
        "DEFAULT_VALUE": params.default_value.map(|v| json!(v)).unwrap_or(Value::Null),
        "INTERNAL": params.internal.unwrap_or("No")
    });

    // Add to CFG_ATTR only (Python does not create FBOM in addAttribute)
    let modified_json =
        helpers::add_to_config_array(config_json, "CFG_ATTR", new_attribute.clone())?;

    Ok((modified_json, new_attribute))
}

/// Delete an attribute from the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Attribute code to delete
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if attribute doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_ATTR section doesn't exist
pub fn delete_attribute(config_json: &str, code: &str) -> Result<String> {
    helpers::delete_from_config_array(config_json, "CFG_ATTR", "ATTR_CODE", &code.to_uppercase())
}

/// Get a specific attribute by code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Attribute code to retrieve
///
/// # Returns
/// JSON Value representing the attribute
///
/// # Errors
/// - `NotFound` if attribute doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_ATTR section doesn't exist
pub fn get_attribute(config_json: &str, code: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = code.to_uppercase();
    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ATTR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_ATTR".to_string()))?
        .iter()
        .find(|attr| attr["ATTR_CODE"].as_str() == Some(&code_upper))
        .cloned()
        .ok_or_else(|| SzConfigError::NotFound(format!("Attribute not found: {}", code_upper)))
}

/// List all attributes
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing attributes in Python format
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_ATTR section doesn't exist
pub fn list_attributes(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let attrs = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ATTR"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_ATTR".to_string()))?;

    Ok(attrs
        .iter()
        .map(|item| {
            json!({
                "id": item.get("ATTR_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "attribute": item.get("ATTR_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                "class": item.get("ATTR_CLASS").and_then(|v| v.as_str()).unwrap_or(""),
                "feature": item.get("FTYPE_CODE").cloned().unwrap_or(Value::Null),
                "element": item.get("FELEM_CODE").cloned().unwrap_or(Value::Null),
                "required": item.get("FELEM_REQ").and_then(|v| v.as_str()).unwrap_or(""),
                "default": item.get("DEFAULT_VALUE").cloned().unwrap_or(Value::Null),
                "internal": item.get("INTERNAL").and_then(|v| v.as_str()).unwrap_or("")
            })
        })
        .collect())
}

/// Set (update) an attribute's properties
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `code` - Attribute code to update
/// * `updates` - JSON Value with fields to update
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if attribute doesn't exist
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if CFG_ATTR section doesn't exist
pub fn set_attribute(config_json: &str, params: SetAttributeParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = params.attribute.to_uppercase();
    let attrs = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut("CFG_ATTR"))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_ATTR".to_string()))?;

    let attr = attrs
        .iter_mut()
        .find(|a| a["ATTR_CODE"].as_str() == Some(&code_upper))
        .ok_or_else(|| SzConfigError::NotFound(format!("Attribute not found: {}", code_upper)))?;

    // Update fields if provided
    if let Some(val) = params.internal {
        attr["INTERNAL"] = json!(val);
    }
    if let Some(val) = params.required {
        attr["FELEM_REQ"] = json!(val);
    }
    if let Some(val) = params.default_value {
        attr["DEFAULT_VALUE"] = json!(val);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}
