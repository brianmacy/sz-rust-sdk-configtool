use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde::Serialize;
use serde_json::{Value, json};

/// Canonical set of valid attribute classes (`ATTR_CLASS`).
///
/// An attribute's class must be one of these values. This is the single source
/// of truth used by [`add_attribute`] to validate the `class` parameter.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::attributes::ATTRIBUTE_CLASSES;
///
/// assert!(ATTRIBUTE_CLASSES.contains(&"IDENTIFIER"));
/// assert!(!ATTRIBUTE_CLASSES.contains(&"NOT_A_CLASS"));
/// ```
pub const ATTRIBUTE_CLASSES: &[&str] = &[
    "NAME",
    "ATTRIBUTE",
    "IDENTIFIER",
    "ADDRESS",
    "PHONE",
    "RELATIONSHIP",
    "OTHER",
];

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_ATTR row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (the nullable `DEFAULT_VALUE` serializes as JSON `null` rather than
/// being dropped). The Senzing engine's config loader requires every key to be
/// present, so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
struct AttrRow {
    #[serde(rename = "ATTR_ID")]
    attr_id: i64,
    #[serde(rename = "ATTR_CODE")]
    attr_code: String,
    #[serde(rename = "ATTR_CLASS")]
    attr_class: String,
    #[serde(rename = "FTYPE_CODE")]
    ftype_code: String,
    #[serde(rename = "FELEM_CODE")]
    felem_code: String,
    #[serde(rename = "FELEM_REQ")]
    felem_req: String,
    #[serde(rename = "DEFAULT_VALUE")]
    default_value: Option<String>,
    #[serde(rename = "INTERNAL")]
    internal: String,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an attribute
///
/// `id` is a caller-supplied `ATTR_ID`. Leave it `None` (or pass a non-positive
/// value) to auto-assign the next free id (seeded at the user-range floor of
/// 1000); pass `Some(id > 0)` to request that exact id — [`add_attribute`] then
/// fails with `AlreadyExists` if it is already taken.
#[derive(Debug, Clone)]
pub struct AddAttributeParams<'a> {
    pub attribute: &'a str,
    pub feature: &'a str,
    pub element: &'a str,
    pub class: &'a str,
    pub default_value: Option<&'a str>,
    pub internal: Option<&'a str>,
    pub required: Option<&'a str>,
    pub id: Option<i64>,
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
            id: json.get("id").and_then(|v| v.as_i64()),
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
pub fn add_attribute(config_json: &str, params: AddAttributeParams) -> Result<(String, Value)> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Validate attribute class (matches Python line 173-181)
    if !ATTRIBUTE_CLASSES.contains(&params.class) {
        return Err(SzConfigError::InvalidInput(format!(
            "Invalid attribute class '{}'. Must be one of: {}",
            params.class,
            ATTRIBUTE_CLASSES.join(", ")
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
            "Attribute: {attribute_upper}"
        )));
    }

    // Validate feature exists (Python parity)
    let _ftype_id = helpers::lookup_feature_id(config_json, &feature_upper)?;

    // Validate element exists (Python parity)
    let _felem_id = helpers::lookup_element_id(config_json, &element_upper)?;

    // Validate REQUIRED domain (Python parity: ["Yes", "No", "Any", "Desired"])
    let required = if let Some(req) = params.required {
        let req_upper = req.to_uppercase();
        match req_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            "ANY" => "Any",
            "DESIRED" => "Desired",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid REQUIRED value '{req}'. Must be one of: Yes, No, Any, Desired"
                )));
            }
        }
    } else {
        "No"
    };

    // Validate INTERNAL domain (Python parity: ["Yes", "No"])
    let internal = if let Some(int) = params.internal {
        let int_upper = int.to_uppercase();
        match int_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid INTERNAL value '{int}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Get next ATTR_ID. Caller-supplied id (#37): None / non-positive ->
    // auto-assign at the user-range floor of 1000; a specific id > 0 is honoured
    // unless already taken (get_desired_or_next_id returns AlreadyExists).
    let next_attr_id = helpers::get_desired_or_next_id(attrs, "ATTR_ID", params.id, 1000)?;

    // Build a complete row via AttrRow so every CFG_ATTR key is present
    // (the nullable DEFAULT_VALUE serializes as null) matching Python lines
    // 2342-2350. FTYPE_CODE/FELEM_CODE use the actual codes, not Null.
    let row = AttrRow {
        attr_id: next_attr_id,
        attr_code: attribute_upper.clone(),
        attr_class: params.class.to_string(),
        ftype_code: feature_upper,
        felem_code: element_upper,
        felem_req: required.to_string(),
        default_value: params.default_value.map(str::to_string),
        internal: internal.to_string(),
    };
    let new_attribute = serde_json::to_value(&row)?;

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
        .ok_or_else(|| SzConfigError::NotFound(format!("Attribute not found: {code_upper}")))
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
    // In-place update of a complete existing row; all keys preserved.
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Attribute not found: {code_upper}")))?;

    // Update fields if provided (with domain validation)
    if let Some(val) = params.internal {
        // Validate INTERNAL domain (Python parity: ["Yes", "No"])
        let val_upper = val.to_uppercase();
        let validated = match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid INTERNAL value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        };
        attr["INTERNAL"] = json!(validated);
    }
    if let Some(val) = params.required {
        // Validate REQUIRED domain (Python parity: ["Yes", "No", "Any", "Desired"])
        let val_upper = val.to_uppercase();
        let validated = match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            "ANY" => "Any",
            "DESIRED" => "Desired",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid REQUIRED value '{val}'. Must be one of: Yes, No, Any, Desired"
                )));
            }
        };
        attr["FELEM_REQ"] = json!(validated);
    }
    if let Some(val) = params.default_value {
        attr["DEFAULT_VALUE"] = json!(val);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTR_KEYS: [&str; 8] = [
        "ATTR_ID",
        "ATTR_CODE",
        "ATTR_CLASS",
        "FTYPE_CODE",
        "FELEM_CODE",
        "FELEM_REQ",
        "DEFAULT_VALUE",
        "INTERNAL",
    ];

    const TEST_CONFIG: &str = r#"{
        "G2_CONFIG": {
            "CFG_ATTR": [],
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "FULL_NAME"}]
        }
    }"#;

    fn assert_all_keys(attr: &Value) {
        let obj = attr.as_object().unwrap();
        for key in ATTR_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
    }

    #[test]
    fn test_add_attribute_emits_all_keys() {
        let params = AddAttributeParams {
            attribute: "my_attr",
            feature: "NAME",
            element: "FULL_NAME",
            class: "NAME",
            default_value: None,
            internal: None,
            required: None,
            id: None,
        };

        let (modified, returned) = add_attribute(TEST_CONFIG, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let attr = &value["G2_CONFIG"]["CFG_ATTR"][0];

        assert_all_keys(attr);
        // The returned Value must carry every key too (used for display).
        assert_all_keys(&returned);

        assert_eq!(attr["ATTR_CODE"], json!("MY_ATTR"));
        assert_eq!(attr["ATTR_CLASS"], json!("NAME"));
        assert_eq!(attr["FTYPE_CODE"], json!("NAME"));
        assert_eq!(attr["FELEM_CODE"], json!("FULL_NAME"));
        assert_eq!(attr["FELEM_REQ"], json!("No"));
        assert_eq!(attr["INTERNAL"], json!("No"));
        // Nullable default surfaces as null (present, not dropped).
        assert_eq!(attr["DEFAULT_VALUE"], Value::Null);
    }

    #[test]
    fn test_add_attribute_with_default_value() {
        let params = AddAttributeParams {
            attribute: "my_attr",
            feature: "NAME",
            element: "FULL_NAME",
            class: "NAME",
            default_value: Some("DFLT"),
            internal: Some("Yes"),
            required: Some("Desired"),
            id: None,
        };

        let (modified, _returned) = add_attribute(TEST_CONFIG, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let attr = &value["G2_CONFIG"]["CFG_ATTR"][0];

        assert_all_keys(attr);
        assert_eq!(attr["DEFAULT_VALUE"], json!("DFLT"));
        assert_eq!(attr["INTERNAL"], json!("Yes"));
        assert_eq!(attr["FELEM_REQ"], json!("Desired"));
    }

    fn add_params(id: Option<i64>) -> AddAttributeParams<'static> {
        AddAttributeParams {
            attribute: "my_attr",
            feature: "NAME",
            element: "FULL_NAME",
            class: "NAME",
            default_value: None,
            internal: None,
            required: None,
            id,
        }
    }

    #[test]
    fn test_add_attribute_auto_id_seeds_1000() {
        // D18: attributes now seed at ATTR_ID 1000, not max+1, on a stock config.
        let config = r#"{
            "G2_CONFIG": {
                "CFG_ATTR": [{"ATTR_ID": 5, "ATTR_CODE": "EXISTING"}],
                "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
                "CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "FULL_NAME"}]
            }
        }"#;
        let (modified, _) = add_attribute(config, add_params(None)).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let attr = value["G2_CONFIG"]["CFG_ATTR"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(attr["ATTR_ID"], json!(1000));
    }

    #[test]
    fn test_add_attribute_specific_id_honoured() {
        let (modified, _) = add_attribute(TEST_CONFIG, add_params(Some(2500))).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let attr = &value["G2_CONFIG"]["CFG_ATTR"][0];
        assert_eq!(attr["ATTR_ID"], json!(2500));
    }

    #[test]
    fn test_add_attribute_taken_id_rejected() {
        let config = r#"{
            "G2_CONFIG": {
                "CFG_ATTR": [{"ATTR_ID": 2500, "ATTR_CODE": "OTHER"}],
                "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
                "CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "FULL_NAME"}]
            }
        }"#;
        let err = add_attribute(config, add_params(Some(2500))).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }
}
