use crate::config_rows::FelemRow;
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
        })
    }
}

/// Parameters for setting (updating) an element
#[derive(Debug, Clone)]
pub struct SetElementParams<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
    pub data_type: Option<&'a str>,
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
        })
    }
}

/// Parameters for setting a feature element
#[derive(Debug, Clone, Default)]
pub struct SetFeatureElementParams<'a> {
    /// Feature code (e.g., "NAME", "ADDRESS")
    pub feature_code: Option<&'a str>,

    /// Element code (e.g., "FIRST_NAME", "FULL_NAME")
    pub element_code: Option<&'a str>,

    pub exec_order: Option<i64>,
    pub display_level: Option<i64>,
    pub display_delim: Option<&'a str>,
    pub derived: Option<&'a str>,
}

impl<'a> SetFeatureElementParams<'a> {
    /// Create new params using feature and element codes
    ///
    /// # Example
    /// ```no_run
    /// use sz_configtool_lib::elements::SetFeatureElementParams;
    ///
    /// let params = SetFeatureElementParams::new("NAME", "FIRST_NAME")
    ///     .with_display_level(1);
    /// ```
    pub fn new(feature_code: &'a str, element_code: &'a str) -> Self {
        Self {
            feature_code: Some(feature_code),
            element_code: Some(element_code),
            exec_order: None,
            display_level: None,
            display_delim: None,
            derived: None,
        }
    }

    /// Set execution order
    pub fn with_exec_order(mut self, order: i64) -> Self {
        self.exec_order = Some(order);
        self
    }

    /// Set display level
    pub fn with_display_level(mut self, level: i64) -> Self {
        self.display_level = Some(level);
        self
    }

    /// Set display delimiter
    pub fn with_display_delim(mut self, delim: &'a str) -> Self {
        self.display_delim = Some(delim);
        self
    }

    /// Set derived flag
    pub fn with_derived(mut self, derived: &'a str) -> Self {
        self.derived = Some(derived);
        self
    }
}

impl<'a> TryFrom<&'a Value> for SetFeatureElementParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let feature_code = json
            .get("featureCode")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("featureCode".to_string()))?;

        let element_code = json
            .get("elementCode")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("elementCode".to_string()))?;

        Ok(Self {
            feature_code: Some(feature_code),
            element_code: Some(element_code),
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
            "Element already exists: {code_upper}"
        )));
    }

    // Get next ID
    let felem_id = helpers::get_next_id_with_min(felem_array, "FELEM_ID", 1000)?;

    // Validate and normalize datatype (Python lines 1974-1981)
    let data_type = if let Some(dt) = params.data_type {
        let dt_lower = dt.to_lowercase();
        match dt_lower.as_str() {
            "string" => "string",
            "number" => "number",
            "date" => "date",
            "datetime" => "datetime",
            "json" => "json",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid DATATYPE value '{dt}'. Must be one of: string, number, date, datetime, json"
                )));
            }
        }
    } else {
        "string" // Default
    };

    // Build a complete row via FelemRow so every CFG_FELEM key is present.
    // FELEM_DESC uses the supplied description or falls back to the code.
    let row = FelemRow {
        felem_id,
        felem_code: code_upper.clone(),
        data_type: data_type.to_string(),
        felem_desc: params
            .description
            .map(str::to_string)
            .unwrap_or_else(|| code_upper.clone()),
    };
    let new_record = serde_json::to_value(&row)?;

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
///
/// # Errors
/// - `NotFound` if element doesn't exist
/// - `InvalidInput` if element is linked to features (Python parity: linkage check)
pub fn delete_element(config_json: &str, felem_code: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let code_upper = felem_code.to_uppercase();

    // Find element to get its ID for linkage check
    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    let element_record = felem_array
        .iter()
        .find(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
        .ok_or_else(|| SzConfigError::NotFound("Element does not exist".to_string()))?;

    let felem_id = element_record["FELEM_ID"]
        .as_i64()
        .ok_or_else(|| SzConfigError::MissingField("FELEM_ID".to_string()))?;

    // Check linkage - prevent deletion if element is used in any features (Python line 2068-2074)
    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    let linked_features: Vec<String> = fbom_array
        .iter()
        .filter(|fbom| fbom["FELEM_ID"].as_i64() == Some(felem_id))
        .filter_map(|fbom| {
            let ftype_id = fbom["FTYPE_ID"].as_i64()?;
            let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"].as_array()?;
            let ftype = ftype_array
                .iter()
                .find(|f| f["FTYPE_ID"].as_i64() == Some(ftype_id))?;
            ftype["FTYPE_CODE"].as_str().map(|s| s.to_string())
        })
        .collect();

    if !linked_features.is_empty() {
        return Err(SzConfigError::InvalidInput(format!(
            "Element linked to the following feature(s): {}",
            linked_features.join(",")
        )));
    }

    // Safe to delete - get mutable array
    let felem_array_mut = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

    if !felem_array_mut
        .iter()
        .any(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
    {
        return Err(SzConfigError::NotFound(format!(
            "Element not found: {code_upper}"
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

    let element = felem_array
        .iter()
        .find(|e| e["FELEM_CODE"].as_str() == Some(code_upper.as_str()))
        .ok_or_else(|| SzConfigError::NotFound(format!("Element not found: {code_upper}")))?;

    // Format to display format with lowercase fields (matching list_elements and Python parity)
    Ok(json!({
        "id": element["FELEM_ID"].as_i64().unwrap_or(0),
        "element": element["FELEM_CODE"].as_str().unwrap_or(""),
        "datatype": element["DATA_TYPE"].as_str().unwrap_or("")
    }))
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
    // In-place update of a complete existing row; all keys preserved.
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
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set feature element (update FBOM record)
///
/// This function updates feature-to-element mappings in CFG_FBOM.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature element parameters (feature_code and element_code required; updates optional)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::elements::{set_feature_element, SetFeatureElementParams};
///
/// let config = r#"{ ... }"#;
/// let params = SetFeatureElementParams::new("NAME", "FIRST_NAME")
///     .with_display_level(1);
/// let updated = set_feature_element(&config, params)?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn set_feature_element(config_json: &str, params: SetFeatureElementParams) -> Result<String> {
    // In-place update of a complete existing CFG_FBOM row; all keys preserved.
    // Resolve codes to IDs
    let feature_code = params
        .feature_code
        .ok_or_else(|| SzConfigError::MissingField("feature_code".to_string()))?;
    let element_code = params
        .element_code
        .ok_or_else(|| SzConfigError::MissingField("element_code".to_string()))?;

    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, element_code)?;

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
                "Feature element mapping not found: FTYPE_ID={ftype_id}, FELEM_ID={felem_id}"
            ))
        })?;

    // Update fields if provided (with validation for Python parity)
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
        // Validate derived domain (Python lines 2192-2198)
        let der_upper = der.to_uppercase();
        let validated_derived = match der_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid DERIVED value '{der}'. Must be 'Yes' or 'No'"
                )));
            }
        };
        fbom["DERIVED"] = json!(validated_derived);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set feature element display level
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code` - Feature code (e.g., "NAME", "ADDRESS")
/// * `element_code` - Element code (e.g., "FIRST_NAME", "FULL_NAME")
/// * `display_level` - Display level value
///
/// # Returns
/// Modified configuration JSON string
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::elements::set_feature_element_display_level;
///
/// let config = r#"{ ... }"#;
/// let updated = set_feature_element_display_level(&config, "NAME", "FIRST_NAME", 1)?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn set_feature_element_display_level(
    config_json: &str,
    feature_code: &str,
    element_code: &str,
    display_level: i64,
) -> Result<String> {
    // Delegates to set_feature_element: in-place CFG_FBOM update, all keys preserved.
    set_feature_element(
        config_json,
        SetFeatureElementParams::new(feature_code, element_code).with_display_level(display_level),
    )
}

/// Set feature element derived flag
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code` - Feature code (e.g., "NAME", "ADDRESS")
/// * `element_code` - Element code (e.g., "FIRST_NAME", "FULL_NAME")
/// * `derived` - Derived flag value ("Yes" or "No")
///
/// # Returns
/// Modified configuration JSON string
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::elements::set_feature_element_derived;
///
/// let config = r#"{ ... }"#;
/// let updated = set_feature_element_derived(&config, "NAME", "FIRST_NAME", "Yes")?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn set_feature_element_derived(
    config_json: &str,
    feature_code: &str,
    element_code: &str,
    derived: &str,
) -> Result<String> {
    // Delegates to set_feature_element: in-place CFG_FBOM update, all keys preserved.
    set_feature_element(
        config_json,
        SetFeatureElementParams::new(feature_code, element_code).with_derived(derived),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG_WITH_FEATURES: &str = r#"{
        "G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 1, "FTYPE_CODE": "NAME"},
                {"FTYPE_ID": 2, "FTYPE_CODE": "ADDRESS"}
            ],
            "CFG_FELEM": [
                {"FELEM_ID": 1, "FELEM_CODE": "FIRST_NAME", "DATA_TYPE": "string"},
                {"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME", "DATA_TYPE": "string"},
                {"FELEM_ID": 3, "FELEM_CODE": "ADDR_LINE1", "DATA_TYPE": "string"}
            ],
            "CFG_FBOM": [
                {"FTYPE_ID": 1, "FELEM_ID": 1, "EXEC_ORDER": 1, "DISPLAY_LEVEL": 0},
                {"FTYPE_ID": 1, "FELEM_ID": 2, "EXEC_ORDER": 2, "DISPLAY_LEVEL": 1},
                {"FTYPE_ID": 2, "FELEM_ID": 3, "EXEC_ORDER": 1, "DISPLAY_LEVEL": 0}
            ]
        }
    }"#;

    #[test]
    fn test_set_feature_element_with_codes() {
        // Test new code-based API
        let params = SetFeatureElementParams::new("NAME", "FIRST_NAME").with_display_level(1);

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(result.is_ok(), "Should succeed with valid codes");

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let fbom = &config["G2_CONFIG"]["CFG_FBOM"][0];
        assert_eq!(fbom["DISPLAY_LEVEL"], 1);
    }

    #[test]
    fn test_set_feature_element_with_codes_all_params() {
        // Test with all optional parameters
        let params = SetFeatureElementParams::new("NAME", "FIRST_NAME")
            .with_display_level(2)
            .with_exec_order(5)
            .with_display_delim("|")
            .with_derived("Yes");

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(result.is_ok());

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let fbom = &config["G2_CONFIG"]["CFG_FBOM"][0];
        assert_eq!(fbom["DISPLAY_LEVEL"], 2);
        assert_eq!(fbom["EXEC_ORDER"], 5);
        assert_eq!(fbom["DISPLAY_DELIM"], "|");
        assert_eq!(fbom["DERIVED"], "Yes");
    }

    #[test]
    fn test_set_feature_element_error_invalid_code() {
        // Test error with invalid feature code
        let params = SetFeatureElementParams::new("INVALID_FEATURE", "FIRST_NAME");

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(result.is_err(), "Should error with invalid feature code");
    }

    #[test]
    fn test_set_feature_element_error_invalid_element_code() {
        // Test error with invalid element code
        let params = SetFeatureElementParams::new("NAME", "INVALID_ELEMENT");

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(result.is_err(), "Should error with invalid element code");
    }

    #[test]
    fn test_set_feature_element_error_mapping_not_found() {
        // Test error when FBOM mapping doesn't exist
        let params = SetFeatureElementParams::new("ADDRESS", "FIRST_NAME");

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(
            result.is_err(),
            "Should error when feature-element mapping doesn't exist"
        );
    }

    #[test]
    fn test_set_feature_element_display_level() {
        // Test code-based convenience function
        let result =
            set_feature_element_display_level(TEST_CONFIG_WITH_FEATURES, "NAME", "FIRST_NAME", 5);
        assert!(result.is_ok());

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let fbom = &config["G2_CONFIG"]["CFG_FBOM"][0];
        assert_eq!(fbom["DISPLAY_LEVEL"], 5);
    }

    #[test]
    fn test_set_feature_element_derived() {
        // Test code-based convenience function
        let result =
            set_feature_element_derived(TEST_CONFIG_WITH_FEATURES, "NAME", "FIRST_NAME", "Yes");
        assert!(result.is_ok());

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let fbom = &config["G2_CONFIG"]["CFG_FBOM"][0];
        assert_eq!(fbom["DERIVED"], "Yes");
    }

    const FELEM_KEYS: [&str; 4] = ["FELEM_ID", "FELEM_CODE", "DATA_TYPE", "FELEM_DESC"];

    #[test]
    fn test_add_element_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {"CFG_FELEM": []}}"#;
        let params = AddElementParams {
            code: "my_elem",
            description: None,
            data_type: None,
        };

        let modified = add_element(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let felem = &value["G2_CONFIG"]["CFG_FELEM"][0];
        let obj = felem.as_object().unwrap();

        assert_eq!(obj.len(), 4, "CFG_FELEM is exactly 4 columns");
        for key in FELEM_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(felem["FELEM_CODE"], json!("MY_ELEM"));
        assert_eq!(felem["DATA_TYPE"], json!("string"));
        // CFG_FELEM has no TOKENIZE/TOKENIZED column in the Senzing v4 schema.
        assert!(!obj.contains_key("TOKENIZE"));
        assert!(!obj.contains_key("TOKENIZED"));
        // FELEM_DESC falls back to the code when no description is supplied.
        assert_eq!(felem["FELEM_DESC"], json!("MY_ELEM"));
    }

    #[test]
    fn test_add_element_with_all_fields() {
        let config = r#"{"G2_CONFIG": {"CFG_FELEM": []}}"#;
        let params = AddElementParams {
            code: "my_elem",
            description: Some("My element"),
            data_type: Some("number"),
        };

        let modified = add_element(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let felem = &value["G2_CONFIG"]["CFG_FELEM"][0];
        let obj = felem.as_object().unwrap();

        for key in FELEM_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert!(!obj.contains_key("TOKENIZE"));
        assert_eq!(felem["DATA_TYPE"], json!("number"));
        assert_eq!(felem["FELEM_DESC"], json!("My element"));
    }

    #[test]
    fn test_set_feature_element_case_insensitive() {
        // Test that codes are case-insensitive (helpers use eq_ignore_ascii_case)
        let params = SetFeatureElementParams::new("name", "first_name").with_display_level(9);

        let result = set_feature_element(TEST_CONFIG_WITH_FEATURES, params);
        assert!(
            result.is_ok(),
            "Should work with lowercase codes (case-insensitive)"
        );

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let fbom = &config["G2_CONFIG"]["CFG_FBOM"][0];
        assert_eq!(fbom["DISPLAY_LEVEL"], 9);
    }
}
