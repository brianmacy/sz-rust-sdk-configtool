use crate::config_rows::FbomRow;
use crate::config_rows::FelemRow;
use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Canonical feature-element validation (D25)
// ============================================================================
//
// One source of truth for validating the DISPLAY_LEVEL and DERIVED columns of a
// CFG_FBOM (feature-element) row, shared by [`add_element_to_feature`] and
// [`set_feature_element`] so the two cannot drift. `add_feature`'s own
// element-list builder is intentionally left on its (more lenient, coercing)
// path this wave to avoid changing its established behaviour.

/// Validate a feature-element `DISPLAY_LEVEL`.
///
/// The level is stored as an integer (sz-tools#130); a negative value is
/// rejected as invalid input.
pub(crate) fn validate_display_level(level: i64) -> Result<i64> {
    if level < 0 {
        return Err(SzConfigError::InvalidInput(format!(
            "Invalid DISPLAY_LEVEL value '{level}'. Must be a non-negative integer"
        )));
    }
    Ok(level)
}

/// Validate and canonicalize a feature-element `DERIVED` flag to `"Yes"`/`"No"`.
pub(crate) fn validate_derived(value: &str) -> Result<&'static str> {
    match value.to_uppercase().as_str() {
        "YES" => Ok("Yes"),
        "NO" => Ok("No"),
        _ => Err(SzConfigError::InvalidInput(format!(
            "Invalid DERIVED value '{value}'. Must be 'Yes' or 'No'"
        ))),
    }
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding an element to a feature (a new CFG_FBOM row).
///
/// `display_level` defaults to `1`, `derived` to `"No"`, and `display_delim` to
/// null when omitted. `EXEC_ORDER` is always allocated automatically.
#[derive(Debug, Clone, Default)]
pub struct AddElementToFeatureParams<'a> {
    pub feature_code: &'a str,
    pub element_code: &'a str,
    pub display_level: Option<i64>,
    pub display_delim: Option<&'a str>,
    pub derived: Option<&'a str>,
}

impl<'a> AddElementToFeatureParams<'a> {
    /// Create params for the given feature and element codes.
    pub fn new(feature_code: &'a str, element_code: &'a str) -> Self {
        Self {
            feature_code,
            element_code,
            ..Default::default()
        }
    }

    /// Set the display level (default `1`).
    pub fn with_display_level(mut self, level: i64) -> Self {
        self.display_level = Some(level);
        self
    }

    /// Set the display delimiter (default null).
    pub fn with_display_delim(mut self, delim: &'a str) -> Self {
        self.display_delim = Some(delim);
        self
    }

    /// Set the derived flag (default `"No"`).
    pub fn with_derived(mut self, derived: &'a str) -> Self {
        self.derived = Some(derived);
        self
    }
}

impl<'a> TryFrom<&'a Value> for AddElementToFeatureParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let feature_code = json
            .get("featureCode")
            .or_else(|| json.get("feature"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("featureCode".to_string()))?;
        let element_code = json
            .get("elementCode")
            .or_else(|| json.get("element"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("elementCode".to_string()))?;

        Ok(Self {
            feature_code,
            element_code,
            display_level: json.get("displayLevel").and_then(|v| v.as_i64()),
            display_delim: json.get("displayDelim").and_then(|v| v.as_str()),
            derived: json.get("derived").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for adding an element
///
/// `id` is a caller-supplied `FELEM_ID`. Leave it `None` (or pass a non-positive
/// value) to auto-assign the next free id (seeded at the user-range floor of
/// 1000); pass `Some(id > 0)` to request that exact id — [`add_element`] then
/// fails with `AlreadyExists` if it is already taken.
#[derive(Debug, Clone)]
pub struct AddElementParams<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
    pub data_type: Option<&'a str>,
    pub id: Option<i64>,
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
            id: json.get("id").and_then(|v| v.as_i64()),
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

    // Get next FELEM_ID. Caller-supplied id (#37): None / non-positive ->
    // auto-assign at the user-range floor of 1000; a specific id > 0 is honoured
    // unless already taken (get_desired_or_next_id returns AlreadyExists).
    let felem_id = helpers::get_desired_or_next_id(felem_array, "FELEM_ID", params.id, 1000)?;

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

    // Update fields if provided, using the canonical validators (D25) shared
    // with add_element_to_feature.
    if let Some(order) = params.exec_order {
        fbom["EXEC_ORDER"] = json!(order);
    }
    if let Some(level) = params.display_level {
        fbom["DISPLAY_LEVEL"] = json!(validate_display_level(level)?);
    }
    if let Some(delim) = params.display_delim {
        fbom["DISPLAY_DELIM"] = json!(delim);
    }
    if let Some(der) = params.derived {
        fbom["DERIVED"] = json!(validate_derived(der)?);
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

/// Add an element to a feature (append a new CFG_FBOM row).
///
/// Resolves the feature and element codes to their ids, rejects a duplicate
/// `(FTYPE_ID, FELEM_ID)` mapping with `AlreadyExists`, allocates a fresh
/// `EXEC_ORDER` from the whole CFG_FBOM table, and writes a complete
/// [`FbomRow`](crate::config_rows) (every key present). `DISPLAY_LEVEL` and
/// `DERIVED` are checked with the canonical validators (D25).
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature/element codes and optional display/derived overrides
///
/// # Errors
/// - `NotFound` if the feature or element code does not exist
/// - `AlreadyExists` if the feature already maps that element
/// - `InvalidInput` if `display_level` or `derived` is invalid
///
/// # Example
/// ```
/// use sz_configtool_lib::elements::{add_element_to_feature, AddElementToFeatureParams};
///
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
///     "CFG_FBOM": []
/// }}"#;
/// let params = AddElementToFeatureParams::new("NAME", "FULL_NAME");
/// let updated = add_element_to_feature(config, params)?;
/// assert!(updated.contains("FULL_NAME") || updated.contains("\"FELEM_ID\":2"));
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn add_element_to_feature(
    config_json: &str,
    params: AddElementToFeatureParams,
) -> Result<String> {
    let ftype_id = helpers::lookup_feature_id(config_json, params.feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, params.element_code)?;

    // Validate the display/derived inputs before mutating anything.
    let display_level = validate_display_level(params.display_level.unwrap_or(1))?;
    let derived = validate_derived(params.derived.unwrap_or("No"))?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    // Duplicate (FTYPE_ID, FELEM_ID) detection.
    if fbom_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(ftype_id) && item["FELEM_ID"].as_i64() == Some(felem_id)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature element mapping already exists: FTYPE_ID={ftype_id}, FELEM_ID={felem_id}"
        )));
    }

    // Whole-table EXEC_ORDER allocation (max over the entire CFG_FBOM + 1).
    let exec_order = helpers::get_next_id_from_array(fbom_array, "EXEC_ORDER")?;

    let row = FbomRow {
        ftype_id,
        felem_id,
        exec_order: Some(exec_order),
        display_level: Some(display_level),
        display_delim: params.display_delim.map(str::to_string),
        derived: Some(derived.to_string()),
    };
    fbom_array.push(serde_json::to_value(&row)?);

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete a single feature-element mapping (one CFG_FBOM row).
///
/// The inverse of [`add_element_to_feature`]: removes the row matching both the
/// resolved feature id and element id. Returns `NotFound` if the mapping is
/// absent.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code` - Feature code (case-insensitive)
/// * `element_code` - Element code (case-insensitive)
///
/// # Errors
/// - `NotFound` if the feature, element, or the mapping does not exist
///
/// # Example
/// ```
/// use sz_configtool_lib::elements::delete_element_from_feature;
///
/// let config = r#"{"G2_CONFIG": {
///     "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
///     "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
///     "CFG_FBOM": [{"FTYPE_ID": 1, "FELEM_ID": 2, "EXEC_ORDER": 1, "DISPLAY_LEVEL": 1}]
/// }}"#;
/// let updated = delete_element_from_feature(config, "NAME", "FULL_NAME")?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn delete_element_from_feature(
    config_json: &str,
    feature_code: &str,
    element_code: &str,
) -> Result<String> {
    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, element_code)?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    let original_len = fbom_array.len();
    fbom_array.retain(|item| {
        !(item["FTYPE_ID"].as_i64() == Some(ftype_id)
            && item["FELEM_ID"].as_i64() == Some(felem_id))
    });

    if fbom_array.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "Feature element mapping not found: FTYPE_ID={ftype_id}, FELEM_ID={felem_id}"
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
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
            id: None,
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
            id: None,
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

    #[test]
    fn test_add_element_auto_id_seeds_1000() {
        let config = r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_ID": 5, "FELEM_CODE": "X"}]}}"#;
        let modified = add_element(
            config,
            AddElementParams {
                code: "my_elem",
                description: None,
                data_type: None,
                id: None,
            },
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let felem = value["G2_CONFIG"]["CFG_FELEM"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(felem["FELEM_ID"], json!(1000));
    }

    #[test]
    fn test_add_element_specific_id_and_taken() {
        let config = r#"{"G2_CONFIG": {"CFG_FELEM": []}}"#;
        let modified = add_element(
            config,
            AddElementParams {
                code: "my_elem",
                description: None,
                data_type: None,
                id: Some(2500),
            },
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(value["G2_CONFIG"]["CFG_FELEM"][0]["FELEM_ID"], json!(2500));

        // Requesting the now-taken id fails.
        let err = add_element(
            &modified,
            AddElementParams {
                code: "other",
                description: None,
                data_type: None,
                id: Some(2500),
            },
        )
        .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_add_element_to_feature_emits_all_keys_and_exec_order() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
            "CFG_FBOM": [{"FTYPE_ID": 9, "FELEM_ID": 9, "EXEC_ORDER": 7, "DISPLAY_LEVEL": 1,
                          "DISPLAY_DELIM": null, "DERIVED": "No"}]
        }}"#;
        let modified = add_element_to_feature(
            config,
            AddElementToFeatureParams::new("name", "full_name").with_display_level(3),
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = value["G2_CONFIG"]["CFG_FBOM"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        let obj = row.as_object().unwrap();
        for key in [
            "FTYPE_ID",
            "FELEM_ID",
            "EXEC_ORDER",
            "DISPLAY_LEVEL",
            "DISPLAY_DELIM",
            "DERIVED",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(row["FTYPE_ID"], json!(1));
        assert_eq!(row["FELEM_ID"], json!(2));
        // Whole-table EXEC_ORDER allocation: max(7) + 1.
        assert_eq!(row["EXEC_ORDER"], json!(8));
        assert_eq!(row["DISPLAY_LEVEL"], json!(3));
        assert_eq!(row["DERIVED"], json!("No"));
        assert_eq!(row["DISPLAY_DELIM"], Value::Null);
    }

    #[test]
    fn test_add_element_to_feature_duplicate_rejected() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
            "CFG_FBOM": [{"FTYPE_ID": 1, "FELEM_ID": 2, "EXEC_ORDER": 1, "DISPLAY_LEVEL": 1}]
        }}"#;
        let err =
            add_element_to_feature(config, AddElementToFeatureParams::new("NAME", "FULL_NAME"))
                .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_add_element_to_feature_invalid_display_level() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
            "CFG_FBOM": []
        }}"#;
        let err = add_element_to_feature(
            config,
            AddElementToFeatureParams::new("NAME", "FULL_NAME").with_display_level(-1),
        )
        .unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
    }

    #[test]
    fn test_delete_element_from_feature_round_trip() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
            "CFG_FBOM": []
        }}"#;
        let added =
            add_element_to_feature(config, AddElementToFeatureParams::new("NAME", "FULL_NAME"))
                .unwrap();
        let v: Value = serde_json::from_str(&added).unwrap();
        assert_eq!(v["G2_CONFIG"]["CFG_FBOM"].as_array().unwrap().len(), 1);

        let removed = delete_element_from_feature(&added, "name", "full_name").unwrap();
        let v: Value = serde_json::from_str(&removed).unwrap();
        assert_eq!(v["G2_CONFIG"]["CFG_FBOM"].as_array().unwrap().len(), 0);

        // Deleting again -> NotFound.
        let err = delete_element_from_feature(&removed, "NAME", "FULL_NAME").unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
    }
}
