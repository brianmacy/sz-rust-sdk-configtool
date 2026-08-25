use crate::behavior_domain::{compute_behavior, parse_behavior_code};
use crate::config_rows::{
    CfbomRow, CfcallRow, DfcallRow, EfbomRow, EfcallRow, FbomRow, FelemRow, FtypeRow, SfcallRow,
};
use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a new feature
#[derive(Debug, Clone, Default)]
pub struct AddFeatureParams<'a> {
    pub feature: &'a str,
    pub element_list: &'a Value,
    pub class: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub candidates: Option<&'a str>,
    pub anonymize: Option<&'a str>,
    pub derived: Option<&'a str>,
    pub history: Option<&'a str>,
    pub matchkey: Option<&'a str>,
    pub standardize: Option<&'a str>,
    pub expression: Option<&'a str>,
    pub comparison: Option<&'a str>,
    pub version: Option<i64>,
    pub rtype_id: Option<i64>,
    /// Caller-supplied `FTYPE_ID`. `None`/non-positive auto-assigns at the
    /// user-range floor of 1000; `Some(id > 0)` is honoured unless already taken.
    pub id: Option<i64>,
}

impl<'a> AddFeatureParams<'a> {
    pub fn new(feature: &'a str, element_list: &'a Value) -> Self {
        Self {
            feature,
            element_list,
            ..Default::default()
        }
    }

    /// Request a specific `FTYPE_ID` for the new feature.
    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}

impl<'a> TryFrom<&'a Value> for AddFeatureParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let feature = json
            .get("feature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;

        let element_list = json
            .get("elementList")
            .ok_or_else(|| SzConfigError::MissingField("elementList".to_string()))?;

        Ok(Self {
            feature,
            element_list,
            class: json.get("class").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            candidates: json.get("candidates").and_then(|v| v.as_str()),
            anonymize: json.get("anonymize").and_then(|v| v.as_str()),
            derived: json.get("derived").and_then(|v| v.as_str()),
            history: json.get("history").and_then(|v| v.as_str()),
            matchkey: json.get("matchKey").and_then(|v| v.as_str()),
            standardize: json
                .get("standardize")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty()),
            expression: json
                .get("expression")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty()),
            comparison: json
                .get("comparison")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty()),
            version: json.get("version").and_then(|v| v.as_i64()),
            rtype_id: json.get("rtypeId").and_then(|v| v.as_i64()),
            id: json.get("id").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for setting/updating a feature
#[derive(Debug, Clone, Default)]
pub struct SetFeatureParams<'a> {
    pub feature: &'a str,
    pub candidates: Option<&'a str>,
    pub anonymize: Option<&'a str>,
    pub derived: Option<&'a str>,
    pub history: Option<&'a str>,
    pub matchkey: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub class: Option<&'a str>,
    pub version: Option<i64>,
    pub rtype_id: Option<i64>,
}

impl<'a> SetFeatureParams<'a> {
    pub fn new(feature: &'a str) -> Self {
        Self {
            feature,
            ..Default::default()
        }
    }
}

impl<'a> TryFrom<&'a Value> for SetFeatureParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let feature = json
            .get("feature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;

        Ok(Self {
            feature,
            candidates: json.get("candidates").and_then(|v| v.as_str()),
            anonymize: json.get("anonymize").and_then(|v| v.as_str()),
            derived: json.get("derived").and_then(|v| v.as_str()),
            history: json.get("history").and_then(|v| v.as_str()),
            matchkey: json.get("matchKey").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            class: json.get("class").and_then(|v| v.as_str()),
            version: json.get("version").and_then(|v| v.as_i64()),
            rtype_id: json.get("rtypeId").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for adding a feature comparison (FBOM)
#[derive(Debug, Clone, Default)]
pub struct AddFeatureComparisonParams<'a> {
    pub feature_code: Option<&'a str>,
    pub element_code: Option<&'a str>,
    /// Execution order of the new `CFG_FBOM` row (whole-table scope; see the
    /// "Execution-order policy" in [`crate::calls`]). `None` auto-allocates the
    /// next order across the whole table; `Some(n > 0)` requests that exact
    /// order and fails with `AlreadyExists` if already taken. It is never left
    /// null on the written row.
    pub exec_order: Option<i64>,
    pub display_level: Option<i64>,
    pub display_delim: Option<&'a str>,
    pub derived: Option<&'a str>,
}

impl<'a> AddFeatureComparisonParams<'a> {
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

    pub fn with_exec_order(mut self, order: i64) -> Self {
        self.exec_order = Some(order);
        self
    }

    pub fn with_display_level(mut self, level: i64) -> Self {
        self.display_level = Some(level);
        self
    }

    pub fn with_display_delim(mut self, delim: &'a str) -> Self {
        self.display_delim = Some(delim);
        self
    }

    pub fn with_derived(mut self, derived: &'a str) -> Self {
        self.derived = Some(derived);
        self
    }
}

impl<'a> TryFrom<&'a Value> for AddFeatureComparisonParams<'a> {
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

/// Parameters for getting a feature comparison
#[derive(Debug, Clone, Default)]
pub struct GetFeatureComparisonParams<'a> {
    pub feature_code: Option<&'a str>,
    pub element_code: Option<&'a str>,
}

impl<'a> GetFeatureComparisonParams<'a> {
    pub fn new(feature_code: &'a str, element_code: &'a str) -> Self {
        Self {
            feature_code: Some(feature_code),
            element_code: Some(element_code),
        }
    }
}

impl<'a> TryFrom<&'a Value> for GetFeatureComparisonParams<'a> {
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
        })
    }
}

/// Parameters for adding a feature distinct call element (CFG_DFCALL)
#[derive(Debug, Clone, Default)]
pub struct AddFeatureDistinctCallElementParams<'a> {
    pub feature_code: Option<&'a str>,
    pub distinct_func_code: Option<&'a str>,
    pub element_code: Option<&'a str>,
    pub exec_order: Option<i64>,
}

impl<'a> AddFeatureDistinctCallElementParams<'a> {
    pub fn new(feature_code: &'a str, distinct_func_code: &'a str) -> Self {
        Self {
            feature_code: Some(feature_code),
            distinct_func_code: Some(distinct_func_code),
            element_code: None,
            exec_order: None,
        }
    }

    pub fn with_element_code(mut self, element_code: &'a str) -> Self {
        self.element_code = Some(element_code);
        self
    }

    pub fn with_exec_order(mut self, order: i64) -> Self {
        self.exec_order = Some(order);
        self
    }
}

// Protected features that cannot be deleted.
//
// This is the ratified (human-approved) locked-feature set that mirrors the
// authoritative Python `locked_feature_list`. Only these codes are protected;
// every other shipped feature (EMAIL, RECORD_TYPE, NATIONAL_ID, TAX_ID,
// ACCT_NUM, ...) is deletable. The previous list also carried codes that do not
// exist as feature codes in the shipped config (DATE_OF_BIRTH, SSN_NUM,
// PASSPORT_NUM, DRIVERS_LICENSE_NUM), which were inert but misleading.
const LOCKED_FEATURES: &[&str] = &[
    "NAME",
    "ADDRESS",
    "PHONE",
    "DOB",
    "REL_LINK",
    "REL_ANCHOR",
    "REL_POINTER",
];

/// Add a new feature to the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature parameters (feature, element_list required; others optional)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::features::{add_feature, AddFeatureParams};
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG":{"CFG_FTYPE":[],...}}"#;
/// let elements = json!([{"element": "NAME"}]);
/// let result = add_feature(config, AddFeatureParams {
///     feature: "PERSON",
///     element_list: &elements,
///     class: Some("IDENTITY"),
///     behavior: Some("FM"),
///     ..Default::default()
/// })?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn add_feature(config_json: &str, params: AddFeatureParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let feature_upper = params.feature.to_uppercase();

    // Check if feature already exists
    let ftypes = config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    if ftypes
        .iter()
        .any(|f| f["FTYPE_CODE"].as_str() == Some(&feature_upper))
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature already exists: {feature_upper}"
        )));
    }

    // Validate element_list
    let elements = params
        .element_list
        .as_array()
        .ok_or_else(|| SzConfigError::InvalidInput("elementList must be an array".to_string()))?;

    if elements.is_empty() {
        return Err(SzConfigError::InvalidInput(
            "elementList must contain at least one element".to_string(),
        ));
    }

    // Validate and normalize domain values (Python parity lines 1432-1461)
    let class = params.class.unwrap_or("OTHER");
    let behavior = params.behavior.unwrap_or("FM");

    // Validate CANDIDATES domain (Python lines 1432-1437)
    let candidates_val = if let Some(val) = params.candidates {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid CANDIDATES value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Validate ANONYMIZE domain (Python lines 1439-1444)
    let anonymize_val = if let Some(val) = params.anonymize {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid ANONYMIZE value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Validate DERIVED domain (Python lines 1446-1449)
    let derived_val = if let Some(val) = params.derived {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid DERIVED value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "No"
    };

    // Validate HISTORY domain (Python lines 1451-1454)
    let history_val = if let Some(val) = params.history {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid HISTORY value '{val}'. Must be 'Yes' or 'No'"
                )));
            }
        }
    } else {
        "Yes"
    };

    // Validate MATCHKEY domain (Python lines 1456-1461)
    let matchkey_default = if params.comparison.is_some() {
        "Yes"
    } else {
        "No"
    };
    let matchkey_val = if let Some(val) = params.matchkey {
        let val_upper = val.to_uppercase();
        match val_upper.as_str() {
            "YES" => "Yes",
            "NO" => "No",
            "CONFIRM" => "Confirm",
            "DENIAL" => "Denial",
            _ => {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid MATCHKEY value '{val}'. Must be one of: Yes, No, Confirm, Denial"
                )));
            }
        }
    } else {
        matchkey_default
    };

    // Get next FTYPE_ID (seed at 1000 for user-created features). Caller-supplied
    // id (#37): None/non-positive auto-assigns; a specific id > 0 is honoured
    // unless already taken (get_desired_or_next_id returns AlreadyExists).
    let ftype_id = helpers::get_desired_or_next_id(ftypes, "FTYPE_ID", params.id, 1000)?;

    // Parse behavior code (like Python's parseFeatureBehavior)
    // Valid frequency codes: A1, F1, FF, FM, FVM, NONE, NAME
    // E suffix means EXCLUSIVITY = "Yes"
    // S suffix means STABILITY = "Yes"
    let behavior_upper = behavior.to_uppercase();
    let (frequency, exclusivity, stability) = parse_behavior_code(&behavior_upper)?;

    // Lookup feature class
    let fclass_array = config["G2_CONFIG"]["CFG_FCLASS"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FCLASS".to_string()))?;

    let fclass_id = fclass_array
        .iter()
        .find(|c| {
            c["FCLASS_CODE"]
                .as_str()
                .map(|s| s.eq_ignore_ascii_case(class))
                .unwrap_or(false)
        })
        .and_then(|c| c["FCLASS_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {class}")))?;

    // Lookup optional functions (validate they exist if provided)
    let sfunc_id = if let Some(func_code) = params.standardize {
        helpers::lookup_sfunc_id(config_json, func_code)?
    } else {
        0
    };

    let efunc_id = if let Some(func_code) = params.expression {
        helpers::lookup_efunc_id(config_json, func_code)?
    } else {
        0
    };

    let cfunc_id = if let Some(func_code) = params.comparison {
        helpers::lookup_cfunc_id(config_json, func_code)?
    } else {
        0
    };

    // Validate that elements are marked expressed/compared if functions are specified
    if efunc_id > 0 || cfunc_id > 0 {
        let mut expressed_cnt = 0;
        let mut compared_cnt = 0;

        for element_item in elements {
            if let Some(obj) = element_item.as_object() {
                if obj
                    .get("expressed")
                    .or_else(|| obj.get("EXPRESSED"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.eq_ignore_ascii_case("yes"))
                    .unwrap_or(false)
                {
                    expressed_cnt += 1;
                }
                if obj
                    .get("compared")
                    .or_else(|| obj.get("COMPARED"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.eq_ignore_ascii_case("yes"))
                    .unwrap_or(false)
                {
                    compared_cnt += 1;
                }
            }
        }

        if efunc_id > 0 && expressed_cnt == 0 {
            return Err(SzConfigError::InvalidInput(
                "No elements marked \"expressed\" for expression routine".to_string(),
            ));
        }
        if cfunc_id > 0 && compared_cnt == 0 {
            return Err(SzConfigError::InvalidInput(
                "No elements marked \"compared\" for comparison routine".to_string(),
            ));
        }
    }

    // Create CFG_FTYPE record via FtypeRow so every key is always present.
    let ftype_row = FtypeRow {
        ftype_id,
        ftype_code: feature_upper.clone(),
        ftype_desc: feature_upper.clone(),
        fclass_id,
        ftype_freq: frequency.to_string(),
        ftype_excl: exclusivity.to_string(),
        ftype_stab: stability.to_string(),
        anonymize: anonymize_val.to_string(),
        derived: derived_val.to_string(),
        used_for_cand: candidates_val.to_string(),
        show_in_match_key: matchkey_val.to_string(),
        persist_history: history_val.to_string(),
        version: params.version.unwrap_or(1),
        rtype_id: params.rtype_id.unwrap_or(0),
    };
    let ftype_record = serde_json::to_value(&ftype_row)?;

    // Add to CFG_FTYPE
    if let Some(ftype_array) = config["G2_CONFIG"]["CFG_FTYPE"].as_array_mut() {
        ftype_array.push(ftype_record);
    }

    // Add standardize call if function specified
    if sfunc_id > 0 {
        let sfcall_array = config["G2_CONFIG"]["CFG_SFCALL"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_SFCALL".to_string()))?;
        let id = helpers::get_next_id_with_min(sfcall_array, "SFCALL_ID", 1000)?;
        let record = serde_json::to_value(&SfcallRow {
            sfcall_id: id,
            sfunc_id,
            exec_order: Some(1),
            ftype_id,
            felem_id: -1,
        })?;
        if let Some(array) = config["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
            array.push(record);
        }
    }

    // Add expression call if function specified
    let efcall_id = if efunc_id > 0 {
        let efcall_array = config["G2_CONFIG"]["CFG_EFCALL"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_EFCALL".to_string()))?;
        let id = helpers::get_next_id_with_min(efcall_array, "EFCALL_ID", 1000)?;
        let record = serde_json::to_value(&EfcallRow {
            efcall_id: id,
            efunc_id,
            exec_order: 1,
            ftype_id,
            felem_id: -1,
            efeat_ftype_id: -1,
            is_virtual: "No".to_string(),
        })?;
        if let Some(array) = config["G2_CONFIG"]["CFG_EFCALL"].as_array_mut() {
            array.push(record);
        }
        id
    } else {
        0
    };

    // Add comparison call if function specified
    let cfcall_id = if cfunc_id > 0 {
        let cfcall_array = config["G2_CONFIG"]["CFG_CFCALL"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_CFCALL".to_string()))?;
        let id = helpers::get_next_id_with_min(cfcall_array, "CFCALL_ID", 1000)?;
        // CFG_CFCALL is exactly CFCALL_ID, FTYPE_ID, CFUNC_ID (no EXEC_ORDER column).
        let record = serde_json::to_value(&CfcallRow {
            cfcall_id: id,
            cfunc_id,
            ftype_id,
        })?;
        if let Some(array) = config["G2_CONFIG"]["CFG_CFCALL"].as_array_mut() {
            array.push(record);
        }
        id
    } else {
        0
    };

    // Process element list
    let mut fbom_order = 0;
    for element_item in elements {
        fbom_order += 1;

        // Parse element (can be string or object)
        let (element_code, expressed, compared, display_level, display_delim, elem_derived) =
            if let Some(elem_str) = element_item.as_str() {
                (
                    elem_str.to_uppercase(),
                    "No".to_string(),
                    "No".to_string(),
                    1,
                    None,
                    "No".to_string(),
                )
            } else if let Some(elem_obj) = element_item.as_object() {
                let code = elem_obj
                    .get("element")
                    .or_else(|| elem_obj.get("ELEMENT"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        SzConfigError::InvalidInput(format!(
                            "Missing element code in elementList item {fbom_order}"
                        ))
                    })?
                    .to_uppercase();

                let expr = elem_obj
                    .get("expressed")
                    .or_else(|| elem_obj.get("EXPRESSED"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("No")
                    .to_uppercase();

                let comp = elem_obj
                    .get("compared")
                    .or_else(|| elem_obj.get("COMPARED"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("No")
                    .to_uppercase();

                // Handle display (backwards compatibility)
                let disp_level_raw = if let Some(display) = elem_obj
                    .get("display")
                    .or_else(|| elem_obj.get("DISPLAY"))
                    .and_then(|v| v.as_str())
                {
                    if display.eq_ignore_ascii_case("yes") {
                        1
                    } else {
                        0
                    }
                } else {
                    elem_obj
                        .get("displaylevel")
                        .or_else(|| elem_obj.get("DISPLAYLEVEL"))
                        .or_else(|| elem_obj.get("display_level"))
                        .and_then(|v| v.as_i64())
                        .unwrap_or(1)
                };
                // Unify onto the shared strict validator (D25): a negative
                // DISPLAY_LEVEL is now rejected rather than stored verbatim,
                // matching set_feature_element / add_element_to_feature.
                let disp_level = crate::elements::validate_display_level(disp_level_raw)?;

                let disp_delim = elem_obj
                    .get("displaydelim")
                    .or_else(|| elem_obj.get("DISPLAYDELIM"))
                    .or_else(|| elem_obj.get("display_delim"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                // Unify onto the shared strict validator (D25): an unknown
                // DERIVED value is now rejected rather than silently coerced to
                // "No", matching set_feature_element / add_element_to_feature.
                let elem_deriv = match elem_obj
                    .get("derived")
                    .or_else(|| elem_obj.get("DERIVED"))
                    .and_then(|v| v.as_str())
                {
                    Some(s) => crate::elements::validate_derived(s)?.to_string(),
                    None => "No".to_string(),
                };

                (code, expr, comp, disp_level, disp_delim, elem_deriv)
            } else {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid element in elementList item {fbom_order}"
                )));
            };

        // Get or create element
        let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_FELEM".to_string()))?;

        let felem_id = if let Some(felem) = felem_array
            .iter()
            .find(|e| e["FELEM_CODE"].as_str() == Some(element_code.as_str()))
        {
            felem["FELEM_ID"]
                .as_i64()
                .ok_or_else(|| SzConfigError::InvalidStructure("Invalid FELEM_ID".to_string()))?
        } else {
            // Create new element
            let new_id = helpers::get_next_id_with_min(felem_array, "FELEM_ID", 1000)?;
            let new_element = serde_json::to_value(&FelemRow {
                felem_id: new_id,
                felem_code: element_code.clone(),
                felem_desc: element_code.clone(),
                data_type: "string".to_string(),
            })?;
            if let Some(array) = config["G2_CONFIG"]["CFG_FELEM"].as_array_mut() {
                array.push(new_element);
            }
            new_id
        };

        // Add to EFBOM if expressed
        if efcall_id > 0 && expressed.eq_ignore_ascii_case("yes") {
            let record = serde_json::to_value(&EfbomRow {
                efcall_id,
                exec_order: fbom_order,
                ftype_id,
                felem_id,
                felem_req: "Yes".to_string(),
            })?;
            if let Some(array) = config["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
                array.push(record);
            }
        }

        // Add to CFBOM if compared
        if cfcall_id > 0 && compared.eq_ignore_ascii_case("yes") {
            let record = serde_json::to_value(&CfbomRow {
                cfcall_id,
                exec_order: fbom_order,
                ftype_id,
                felem_id,
            })?;
            if let Some(array) = config["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
                array.push(record);
            }
        }

        // Add to FBOM (always). DISPLAY_DELIM is nullable (seed-then-null).
        let fbom_record = serde_json::to_value(&FbomRow {
            ftype_id,
            felem_id,
            exec_order: Some(fbom_order),
            display_level: Some(display_level),
            display_delim,
            derived: Some(elem_derived),
        })?;

        if let Some(array) = config["G2_CONFIG"]["CFG_FBOM"].as_array_mut() {
            array.push(fbom_record);
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete a feature from the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code_or_id` - Feature code or numeric ID
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `NotFound` if feature doesn't exist
/// - `InvalidInput` if trying to delete a protected feature
pub fn delete_feature(config_json: &str, feature_code_or_id: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Try to parse as ID first, then as code
    let ftype_id = if let Ok(id) = feature_code_or_id.trim().parse::<i64>() {
        // Validate ID exists
        let ftypes = config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

        if !ftypes.iter().any(|f| f["FTYPE_ID"].as_i64() == Some(id)) {
            return Err(SzConfigError::NotFound(format!("Feature: {id}")));
        }
        id
    } else {
        lookup_feature_id(&config, feature_code_or_id)?
    };

    // Get feature code for validation
    let feature_code = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["FTYPE_ID"].as_i64() == Some(ftype_id))
                .and_then(|f| f["FTYPE_CODE"].as_str())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {ftype_id}")))?
        .to_string();

    // Check if feature is locked
    if LOCKED_FEATURES
        .iter()
        .any(|&locked| locked.eq_ignore_ascii_case(&feature_code))
    {
        return Err(SzConfigError::InvalidInput(format!(
            "The feature {feature_code} cannot be deleted (it is a protected system feature)"
        )));
    }

    // Delete FBOM records
    if let Some(fbom_array) = config["G2_CONFIG"]["CFG_FBOM"].as_array_mut() {
        fbom_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    // Delete CFG_ATTR records
    if let Some(attr_array) = config["G2_CONFIG"]["CFG_ATTR"].as_array_mut() {
        attr_array.retain(|record| {
            record["FTYPE_CODE"]
                .as_str()
                .map(|s| !s.eq_ignore_ascii_case(&feature_code))
                .unwrap_or(true)
        });
    }

    // Delete standardize calls
    if let Some(sfcall_array) = config["G2_CONFIG"]["CFG_SFCALL"].as_array_mut() {
        sfcall_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    // Delete expression calls and their BOM records
    let efcall_ids: Vec<i64> = config["G2_CONFIG"]["CFG_EFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
                .filter_map(|call| call["EFCALL_ID"].as_i64())
                .collect()
        })
        .unwrap_or_default();

    if let Some(efbom_array) = config["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
        efbom_array.retain(|record| {
            record["EFCALL_ID"]
                .as_i64()
                .map(|id| !efcall_ids.contains(&id))
                .unwrap_or(true)
        });
    }

    if let Some(efcall_array) = config["G2_CONFIG"]["CFG_EFCALL"].as_array_mut() {
        efcall_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    // Delete comparison calls and their BOM records
    let cfcall_ids: Vec<i64> = config["G2_CONFIG"]["CFG_CFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
                .filter_map(|call| call["CFCALL_ID"].as_i64())
                .collect()
        })
        .unwrap_or_default();

    if let Some(cfbom_array) = config["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
        cfbom_array.retain(|record| {
            record["CFCALL_ID"]
                .as_i64()
                .map(|id| !cfcall_ids.contains(&id))
                .unwrap_or(true)
        });
    }

    if let Some(cfcall_array) = config["G2_CONFIG"]["CFG_CFCALL"].as_array_mut() {
        cfcall_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    // Delete distinct calls and their BOM records
    let dfcall_ids: Vec<i64> = config["G2_CONFIG"]["CFG_DFCALL"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
                .filter_map(|call| call["DFCALL_ID"].as_i64())
                .collect()
        })
        .unwrap_or_default();

    if let Some(dfbom_array) = config["G2_CONFIG"]["CFG_DFBOM"].as_array_mut() {
        dfbom_array.retain(|record| {
            record["DFCALL_ID"]
                .as_i64()
                .map(|id| !dfcall_ids.contains(&id))
                .unwrap_or(true)
        });
    }

    if let Some(dfcall_array) = config["G2_CONFIG"]["CFG_DFCALL"].as_array_mut() {
        dfcall_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    // Finally, delete the feature itself
    if let Some(ftype_array) = config["G2_CONFIG"]["CFG_FTYPE"].as_array_mut() {
        ftype_array.retain(|record| record["FTYPE_ID"].as_i64() != Some(ftype_id));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a specific feature by code or ID
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code_or_id` - Feature code or numeric ID
///
/// # Returns
/// JSON Value representing the complete feature with elementList
pub fn get_feature(config_json: &str, feature_code_or_id: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Try to parse as ID first, then as code
    let ftype = if let Ok(id) = feature_code_or_id.trim().parse::<i64>() {
        config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .and_then(|arr| arr.iter().find(|f| f["FTYPE_ID"].as_i64() == Some(id)))
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {id}")))?
    } else {
        let code_upper = feature_code_or_id.to_uppercase();
        config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|f| f["FTYPE_CODE"].as_str() == Some(code_upper.as_str()))
            })
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {code_upper}")))?
    };

    build_feature_json(&config, ftype)
}

/// List all features in the configuration
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing features with elementList
pub fn list_features(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    let mut result: Vec<Value> = ftype_array
        .iter()
        .map(|ftype| build_feature_json(&config, ftype))
        .collect::<Result<Vec<_>>>()?;

    // Sort by FTYPE_ID
    result.sort_by_key(|item| item["id"].as_i64().unwrap_or(0));

    Ok(result)
}

/// Set (update) a feature's properties
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature parameters (feature required, updates optional)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::features::{set_feature, SetFeatureParams};
///
/// let config = r#"{"G2_CONFIG":{"CFG_FTYPE":[...]}}"#;
/// let result = set_feature(config, SetFeatureParams {
///     feature: "NAME",
///     candidates: Some("Yes"),
///     behavior: Some("NAME"),
///     version: Some(2),
///     ..Default::default()
/// })?;
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn set_feature(config_json: &str, params: SetFeatureParams) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Try to parse as ID first, then as code
    let ftype_id = if let Ok(id) = params.feature.trim().parse::<i64>() {
        id
    } else {
        lookup_feature_id(&config, params.feature)?
    };

    let ftypes = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    let ftype = ftypes
        .iter_mut()
        .find(|f| f["FTYPE_ID"].as_i64() == Some(ftype_id))
        .ok_or_else(|| SzConfigError::NotFound("Feature not found".to_string()))?;

    // In-place update of a complete existing CFG_FTYPE row; all keys preserved.
    // Not routed through FtypeRow (which would require reconstructing every field).
    // Track if any changes made (for "No changes detected")
    let mut changes_made = false;

    // Update fields if provided with validation and normalization
    if let Some(val) = params.candidates {
        // Validate and normalize CANDIDATES domain (Python line 1702-1707)
        let normalized = validate_and_normalize_domain(val, &["Yes", "No"], "CANDIDATES")?;
        if ftype["USED_FOR_CAND"].as_str() != Some(normalized.as_str()) {
            ftype["USED_FOR_CAND"] = json!(normalized);
            changes_made = true;
        }
    }
    if let Some(val) = params.anonymize
        && ftype["ANONYMIZE"].as_str() != Some(val)
    {
        ftype["ANONYMIZE"] = json!(val);
        changes_made = true;
    }
    if let Some(val) = params.derived
        && ftype["DERIVED"].as_str() != Some(val)
    {
        ftype["DERIVED"] = json!(val);
        changes_made = true;
    }
    if let Some(val) = params.history
        && ftype["PERSIST_HISTORY"].as_str() != Some(val)
    {
        ftype["PERSIST_HISTORY"] = json!(val);
        changes_made = true;
    }
    if let Some(val) = params.matchkey {
        // Validate and normalize MATCHKEY domain (Python line 1754-1758)
        let normalized =
            validate_and_normalize_domain(val, &["Yes", "No", "Confirm", "Denial"], "MATCHKEY")?;
        if ftype["SHOW_IN_MATCH_KEY"].as_str() != Some(normalized.as_str()) {
            ftype["SHOW_IN_MATCH_KEY"] = json!(normalized);
            changes_made = true;
        }
    }
    if let Some(val) = params.version
        && ftype["VERSION"].as_i64() != Some(val)
    {
        ftype["VERSION"] = json!(val);
        changes_made = true;
    }
    if let Some(val) = params.rtype_id
        && ftype["RTYPE_ID"].as_i64() != Some(val)
    {
        ftype["RTYPE_ID"] = json!(val);
        changes_made = true;
    }

    // Parse and set behavior (FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB)
    if let Some(behavior_code) = params.behavior {
        let (frequency, exclusivity, stability) = parse_behavior_code(behavior_code)?;
        let freq_changed = ftype["FTYPE_FREQ"].as_str() != Some(frequency);
        let excl_changed = ftype["FTYPE_EXCL"].as_str() != Some(exclusivity);
        let stab_changed = ftype["FTYPE_STAB"].as_str() != Some(stability);
        if freq_changed || excl_changed || stab_changed {
            ftype["FTYPE_FREQ"] = json!(frequency);
            ftype["FTYPE_EXCL"] = json!(exclusivity);
            ftype["FTYPE_STAB"] = json!(stability);
            changes_made = true;
        }
    }

    // Lookup and set class (FCLASS_ID) - must do before modifying ftype
    if let Some(class_name) = params.class {
        // Parse config again to avoid borrow conflict
        let config_for_lookup: Value = serde_json::from_str(config_json)
            .map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

        let fclass_array = config_for_lookup["G2_CONFIG"]["CFG_FCLASS"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_FCLASS".to_string()))?;

        let fclass_id = fclass_array
            .iter()
            .find(|c| {
                c["FCLASS_CODE"]
                    .as_str()
                    .map(|s| s.eq_ignore_ascii_case(class_name))
                    .unwrap_or(false)
            })
            .and_then(|c| c["FCLASS_ID"].as_i64())
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {class_name}")))?;

        if ftype["FCLASS_ID"].as_i64() != Some(fclass_id) {
            ftype["FCLASS_ID"] = json!(fclass_id);
            changes_made = true;
        }
    }

    // Check if any changes were made (Python lines 1824-1825)
    if !changes_made {
        return Err(SzConfigError::InvalidInput(
            "No changes detected".to_string(),
        ));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Validate value is in domain and normalize to proper case
fn validate_and_normalize_domain(value: &str, domain: &[&str], field_name: &str) -> Result<String> {
    let value_upper = value.to_uppercase();
    for valid_value in domain {
        if valid_value.to_uppercase() == value_upper {
            return Ok(valid_value.to_string());
        }
    }
    Err(SzConfigError::InvalidInput(format!(
        "{field_name} value must be in {domain:?}"
    )))
}

// Helper functions

/// Build complete feature JSON with elementList for display
pub fn build_feature_json(config: &Value, ftype: &Value) -> Result<Value> {
    let empty_array = vec![];

    let fclass_array = config["G2_CONFIG"]["CFG_FCLASS"]
        .as_array()
        .unwrap_or(&empty_array);
    let sfcall_array = config["G2_CONFIG"]["CFG_SFCALL"]
        .as_array()
        .unwrap_or(&empty_array);
    let efcall_array = config["G2_CONFIG"]["CFG_EFCALL"]
        .as_array()
        .unwrap_or(&empty_array);
    let cfcall_array = config["G2_CONFIG"]["CFG_CFCALL"]
        .as_array()
        .unwrap_or(&empty_array);
    let sfunc_array = config["G2_CONFIG"]["CFG_SFUNC"]
        .as_array()
        .unwrap_or(&empty_array);
    let efunc_array = config["G2_CONFIG"]["CFG_EFUNC"]
        .as_array()
        .unwrap_or(&empty_array);
    let cfunc_array = config["G2_CONFIG"]["CFG_CFUNC"]
        .as_array()
        .unwrap_or(&empty_array);
    let felem_array = config["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .unwrap_or(&empty_array);
    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .unwrap_or(&empty_array);
    let efbom_array = config["G2_CONFIG"]["CFG_EFBOM"]
        .as_array()
        .unwrap_or(&empty_array);
    let cfbom_array = config["G2_CONFIG"]["CFG_CFBOM"]
        .as_array()
        .unwrap_or(&empty_array);

    let ftype_id = ftype["FTYPE_ID"].as_i64().unwrap_or(0);
    let fclass_id = ftype["FCLASS_ID"].as_i64().unwrap_or(0);

    // Resolve class name
    let class_name = fclass_array
        .iter()
        .find(|fc| fc["FCLASS_ID"].as_i64() == Some(fclass_id))
        .and_then(|fc| fc["FCLASS_CODE"].as_str())
        .unwrap_or("OTHER")
        .to_string();

    // Compute behavior
    let behavior = compute_behavior(ftype);

    // Find standardize function
    let standardize = sfcall_array
        .iter()
        .filter(|sc| sc["FTYPE_ID"].as_i64() == Some(ftype_id))
        .min_by_key(|sc| sc["EXEC_ORDER"].as_i64().unwrap_or(0))
        .and_then(|sc| sc["SFUNC_ID"].as_i64())
        .and_then(|sfunc_id| {
            sfunc_array
                .iter()
                .find(|sf| sf["SFUNC_ID"].as_i64() == Some(sfunc_id))
        })
        .and_then(|sf| sf["SFUNC_CODE"].as_str())
        .unwrap_or("")
        .to_string();

    // Find expression function
    let efcall = efcall_array
        .iter()
        .filter(|ec| ec["FTYPE_ID"].as_i64() == Some(ftype_id))
        .min_by_key(|ec| ec["EXEC_ORDER"].as_i64().unwrap_or(0));

    let expression = efcall
        .and_then(|ec| ec["EFUNC_ID"].as_i64())
        .and_then(|efunc_id| {
            efunc_array
                .iter()
                .find(|ef| ef["EFUNC_ID"].as_i64() == Some(efunc_id))
        })
        .and_then(|ef| ef["EFUNC_CODE"].as_str())
        .unwrap_or("")
        .to_string();

    // Find comparison function
    let cfcall = cfcall_array
        .iter()
        .filter(|cc| cc["FTYPE_ID"].as_i64() == Some(ftype_id))
        .min_by_key(|cc| cc["CFCALL_ID"].as_i64().unwrap_or(0));

    let comparison = cfcall
        .and_then(|cc| cc["CFUNC_ID"].as_i64())
        .and_then(|cfunc_id| {
            cfunc_array
                .iter()
                .find(|cf| cf["CFUNC_ID"].as_i64() == Some(cfunc_id))
        })
        .and_then(|cf| cf["CFUNC_CODE"].as_str())
        .unwrap_or("")
        .to_string();

    // Build elementList
    let mut element_list: Vec<(i64, Value)> = fbom_array
        .iter()
        .filter(|fbom| fbom["FTYPE_ID"].as_i64() == Some(ftype_id))
        .map(|fbom| {
            let felem_id = fbom["FELEM_ID"].as_i64().unwrap_or(0);
            let exec_order = fbom["EXEC_ORDER"].as_i64().unwrap_or(0);

            let element_code = felem_array
                .iter()
                .find(|fe| fe["FELEM_ID"].as_i64() == Some(felem_id))
                .and_then(|fe| fe["FELEM_CODE"].as_str())
                .unwrap_or("")
                .to_string();

            let expressed = efcall
                .and_then(|ec| ec["EFCALL_ID"].as_i64())
                .map(|efcall_id| {
                    efbom_array.iter().any(|efbom| {
                        efbom["EFCALL_ID"].as_i64() == Some(efcall_id)
                            && efbom["FTYPE_ID"].as_i64() == Some(ftype_id)
                            && efbom["FELEM_ID"].as_i64() == Some(felem_id)
                    })
                })
                .unwrap_or(false);

            let compared = cfcall
                .and_then(|cc| cc["CFCALL_ID"].as_i64())
                .map(|cfcall_id| {
                    cfbom_array.iter().any(|cfbom| {
                        cfbom["CFCALL_ID"].as_i64() == Some(cfcall_id)
                            && cfbom["FTYPE_ID"].as_i64() == Some(ftype_id)
                            && cfbom["FELEM_ID"].as_i64() == Some(felem_id)
                    })
                })
                .unwrap_or(false);

            let derived = fbom["DERIVED"].as_str().unwrap_or("No");
            let display_level = fbom["DISPLAY_LEVEL"].as_i64().unwrap_or(1);
            let display = if display_level == 0 { "No" } else { "Yes" };

            (
                exec_order,
                json!({
                    "element": element_code,
                    "expressed": if expressed { "Yes" } else { "No" },
                    "compared": if compared { "Yes" } else { "No" },
                    "derived": derived,
                    "display": display
                }),
            )
        })
        .collect();

    element_list.sort_by_key(|(order, _)| *order);
    let element_list: Vec<Value> = element_list.into_iter().map(|(_, v)| v).collect();

    Ok(json!({
        "id": ftype_id,
        "feature": ftype["FTYPE_CODE"].as_str().unwrap_or(""),
        "class": class_name,
        "behavior": behavior,
        "anonymize": ftype["ANONYMIZE"].as_str().unwrap_or(""),
        "candidates": ftype["USED_FOR_CAND"].as_str().unwrap_or(""),
        "standardize": standardize,
        "expression": expression,
        "comparison": comparison,
        "matchKey": ftype["SHOW_IN_MATCH_KEY"].as_str().unwrap_or(""),
        "version": ftype["VERSION"].as_i64().unwrap_or(0),
        "elementList": element_list
    }))
}

fn lookup_feature_id(config: &Value, feature_code: &str) -> Result<i64> {
    let code_upper = feature_code.to_uppercase();
    config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["FTYPE_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["FTYPE_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound("Feature not found".to_string()))
}

#[allow(dead_code)]
fn lookup_sfunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_SFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["SFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["SFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Standardize function: {code_upper}")))
}

#[allow(dead_code)]
fn lookup_efunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_EFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["EFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["EFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Expression function: {code_upper}")))
}

#[allow(dead_code)]
fn lookup_cfunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_CFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["CFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["CFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Comparison function: {code_upper}")))
}

/// Add a feature comparison element (FBOM record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature comparison parameters (ftype_id, felem_id required; others optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_feature_comparison(
    config_json: &str,
    params: AddFeatureComparisonParams,
) -> Result<String> {
    let feature_code = params
        .feature_code
        .ok_or_else(|| SzConfigError::MissingField("feature_code".to_string()))?;
    let element_code = params
        .element_code
        .ok_or_else(|| SzConfigError::MissingField("element_code".to_string()))?;

    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, element_code)?;

    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Check if already exists
    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    if fbom_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(ftype_id) && item["FELEM_ID"].as_i64() == Some(felem_id)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature comparison: {:?}+{:?}",
            params.feature_code, params.element_code
        )));
    }

    // Allocate EXEC_ORDER over the whole CFG_FBOM table (empty scope), honouring
    // a caller-supplied value or rejecting it if taken (mirrors Python
    // do_addElementToFeature: getDesiredValueOrNext("CFG_FBOM", "EXEC_ORDER", ...)).
    // An order is always resolved to a concrete value here, never left null.
    let exec_order =
        helpers::get_desired_or_next_order(fbom_array, "EXEC_ORDER", &[], params.exec_order)?;

    // Build record via FbomRow so every CFG_FBOM key is present; the remaining
    // optional fields serialize as null (seed-then-null preserved).
    let record = serde_json::to_value(&FbomRow {
        ftype_id,
        felem_id,
        exec_order: Some(exec_order),
        display_level: params.display_level,
        display_delim: params.display_delim.map(str::to_string),
        derived: params.derived.map(str::to_string),
    })?;

    helpers::add_to_config_array(config_json, "CFG_FBOM", record)
}

/// Delete a feature comparison element (FBOM record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Feature element ID
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_feature_comparison(
    config_json: &str,
    feature_code: &str,
    element_code: &str,
) -> Result<String> {
    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, element_code)?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let mut found = false;

    if let Some(fbom_array) = config["G2_CONFIG"]["CFG_FBOM"].as_array_mut() {
        fbom_array.retain(|item| {
            let matches = item["FTYPE_ID"].as_i64() == Some(ftype_id)
                && item["FELEM_ID"].as_i64() == Some(felem_id);
            if matches {
                found = true;
            }
            !matches
        });
    }

    if !found {
        return Err(SzConfigError::NotFound(format!(
            "Feature comparison: FTYPE_ID={ftype_id}, FELEM_ID={felem_id}"
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Get a specific feature comparison element
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature comparison parameters (ftype_id and felem_id)
///
/// # Returns
/// JSON Value representing the feature comparison
pub fn get_feature_comparison(
    config_json: &str,
    params: GetFeatureComparisonParams,
) -> Result<Value> {
    let feature_code = params
        .feature_code
        .ok_or_else(|| SzConfigError::MissingField("feature_code".to_string()))?;
    let element_code = params
        .element_code
        .ok_or_else(|| SzConfigError::MissingField("element_code".to_string()))?;

    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let felem_id = helpers::lookup_element_id(config_json, element_code)?;

    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    fbom_array
        .iter()
        .find(|item| {
            item["FTYPE_ID"].as_i64() == Some(ftype_id)
                && item["FELEM_ID"].as_i64() == Some(felem_id)
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Feature comparison: {:?}+{:?}",
                params.feature_code, params.element_code
            ))
        })
}

/// List all feature comparison elements (FBOM records)
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing feature comparisons, sorted by FTYPE_ID and EXEC_ORDER
pub fn list_feature_comparisons(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    let mut result: Vec<Value> = fbom_array.to_vec();

    // Sort by FTYPE_ID and EXEC_ORDER
    result.sort_by(|a, b| {
        let a_ftype = a["FTYPE_ID"].as_i64().unwrap_or(0);
        let b_ftype = b["FTYPE_ID"].as_i64().unwrap_or(0);
        let a_exec = a["EXEC_ORDER"].as_i64().unwrap_or(0);
        let b_exec = b["EXEC_ORDER"].as_i64().unwrap_or(0);
        (a_ftype, a_exec).cmp(&(b_ftype, b_exec))
    });

    Ok(result)
}

/// Add a feature comparison element (same as add_feature_comparison, for compatibility)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Feature comparison parameters (ftype_id, felem_id required; others optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_feature_comparison_element(
    config_json: &str,
    params: AddFeatureComparisonParams,
) -> Result<String> {
    add_feature_comparison(config_json, params)
}

/// Delete a feature comparison element (same as delete_feature_comparison, for compatibility)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `felem_id` - Feature element ID
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_feature_comparison_element(
    config_json: &str,
    feature_code: &str,
    element_code: &str,
) -> Result<String> {
    delete_feature_comparison(config_json, feature_code, element_code)
}

/// Add a feature distinct call element (CFG_DFCALL record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `ftype_id` - Feature type ID
/// * `dfunc_id` - Distinct function ID
/// * `felem_id` - Optional feature element ID (default: -1)
/// * `exec_order` - Optional execution order
///
/// # Returns
/// Modified configuration JSON string
pub fn add_feature_distinct_call_element(
    config_json: &str,
    params: AddFeatureDistinctCallElementParams,
) -> Result<String> {
    let feature_code = params
        .feature_code
        .ok_or_else(|| SzConfigError::MissingField("feature_code".to_string()))?;
    let distinct_func_code = params
        .distinct_func_code
        .ok_or_else(|| SzConfigError::MissingField("distinct_func_code".to_string()))?;

    let ftype_id = helpers::lookup_feature_id(config_json, feature_code)?;
    let dfunc_id = helpers::lookup_dfunc_id(config_json, distinct_func_code)?;
    // Validate the element code if supplied (a DFCALL identifies a feature/function
    // pair; per the authoritative schema its row carries no FELEM_ID or EXEC_ORDER).
    if let Some(code) = params.element_code {
        helpers::lookup_element_id(config_json, code)?;
    }

    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Check if already exists (identity is FTYPE_ID + DFUNC_ID)
    let dfcall_array = config["G2_CONFIG"]["CFG_DFCALL"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DFCALL".to_string()))?;

    if dfcall_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(ftype_id) && item["DFUNC_ID"].as_i64() == Some(dfunc_id)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature distinct call element: {:?}+{:?}",
            params.feature_code, params.distinct_func_code
        )));
    }

    // Get next DFCALL_ID
    let dfcall_id = helpers::get_next_id_with_min(dfcall_array, "DFCALL_ID", 1000)?;

    // Build record via DfcallRow. CFG_DFCALL is exactly DFCALL_ID, FTYPE_ID,
    // DFUNC_ID per the authoritative Senzing v4 schema.
    let record = serde_json::to_value(&DfcallRow {
        dfcall_id,
        ftype_id,
        dfunc_id,
    })?;

    helpers::add_to_config_array(config_json, "CFG_DFCALL", record)
}

/// List all feature classes (CFG_FCLASS records)
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values representing feature classes
pub fn list_feature_classes(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fclass_array = config["G2_CONFIG"]["CFG_FCLASS"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FCLASS".to_string()))?;

    let mut result: Vec<Value> = fclass_array.to_vec();

    // Sort by FCLASS_ID
    result.sort_by_key(|item| item["FCLASS_ID"].as_i64().unwrap_or(0));

    Ok(result)
}

/// Get a specific feature class by ID or code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `fclass_id_or_code` - Feature class ID (numeric) or code (string)
///
/// # Returns
/// JSON Value representing the feature class
pub fn get_feature_class(config_json: &str, fclass_id_or_code: &str) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fclass_array = config["G2_CONFIG"]["CFG_FCLASS"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FCLASS".to_string()))?;

    // Try to parse as ID first
    if let Ok(id) = fclass_id_or_code.trim().parse::<i64>() {
        fclass_array
            .iter()
            .find(|item| item["FCLASS_ID"].as_i64() == Some(id))
            .cloned()
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {id}")))
    } else {
        // Try as code
        let code_upper = fclass_id_or_code.to_uppercase();
        fclass_array
            .iter()
            .find(|item| item["FCLASS_CODE"].as_str() == Some(code_upper.as_str()))
            .cloned()
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {code_upper}")))
    }
}

/// Update the feature version in compatibility settings
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `version` - New feature version string
///
/// # Returns
/// Modified configuration JSON string
pub fn update_feature_version(config_json: &str, version: &str) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // In-place update of a single scalar field (not a CFG_* section row builder);
    // all other keys preserved. Left as-is per spec.
    // Navigate to COMPATIBILITY_VERSION
    let compat_version = config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]
        .as_object_mut()
        .ok_or_else(|| SzConfigError::MissingSection("COMPATIBILITY_VERSION".to_string()))?;

    compat_version.insert("FEATURE_VERSION".to_string(), json!(version));

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FTYPE_KEYS: [&str; 14] = [
        "FTYPE_ID",
        "FTYPE_CODE",
        "FTYPE_DESC",
        "FCLASS_ID",
        "FTYPE_FREQ",
        "FTYPE_EXCL",
        "FTYPE_STAB",
        "ANONYMIZE",
        "DERIVED",
        "USED_FOR_CAND",
        "SHOW_IN_MATCH_KEY",
        "PERSIST_HISTORY",
        "VERSION",
        "RTYPE_ID",
    ];
    const FBOM_KEYS: [&str; 6] = [
        "FTYPE_ID",
        "FELEM_ID",
        "EXEC_ORDER",
        "DISPLAY_LEVEL",
        "DISPLAY_DELIM",
        "DERIVED",
    ];
    const FELEM_KEYS: [&str; 4] = ["FELEM_ID", "FELEM_CODE", "FELEM_DESC", "DATA_TYPE"];

    fn assert_all_keys(obj: &Value, keys: &[&str]) {
        let map = obj.as_object().unwrap();
        for key in keys {
            assert!(map.contains_key(*key), "{key} key must be present");
        }
    }

    /// add_feature must write a complete CFG_FTYPE row, a complete CFG_FELEM row
    /// for the newly-created element, and a complete CFG_FBOM row.
    #[test]
    fn test_add_feature_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [],
            "CFG_FCLASS": [{"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}],
            "CFG_FELEM": [],
            "CFG_FBOM": []
        }}"#;

        let elements = json!([{"element": "MYELEM"}]);
        let params = AddFeatureParams {
            feature: "MYFEAT",
            element_list: &elements,
            ..Default::default()
        };
        let modified = add_feature(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &value["G2_CONFIG"];

        let ftype = &g2["CFG_FTYPE"][0];
        assert_all_keys(ftype, &FTYPE_KEYS);
        assert_eq!(ftype["FTYPE_CODE"], json!("MYFEAT"));
        assert_eq!(ftype["FTYPE_FREQ"], json!("FM")); // default behavior
        assert_eq!(ftype["PERSIST_HISTORY"], json!("Yes")); // default history
        assert_eq!(ftype["VERSION"], json!(1));
        assert_eq!(ftype["RTYPE_ID"], json!(0));

        // A new element was created for MYELEM.
        let felem = &g2["CFG_FELEM"][0];
        assert_all_keys(felem, &FELEM_KEYS);
        assert_eq!(felem["FELEM_CODE"], json!("MYELEM"));
        assert_eq!(felem["DATA_TYPE"], json!("string"));
        // CFG_FELEM has no TOKENIZE column in the Senzing v4 schema.
        assert!(
            !felem.as_object().unwrap().contains_key("TOKENIZE"),
            "CFG_FELEM must not carry a TOKENIZE key"
        );

        // FBOM row: DISPLAY_DELIM present as null (not supplied).
        let fbom = &g2["CFG_FBOM"][0];
        assert_all_keys(fbom, &FBOM_KEYS);
        assert_eq!(fbom["DISPLAY_DELIM"], Value::Null);
        assert_eq!(fbom["EXEC_ORDER"], json!(1));
        assert_eq!(fbom["DISPLAY_LEVEL"], json!(1));
        assert_eq!(fbom["DERIVED"], json!("No"));
    }

    // D25: add_feature's per-element elementList handling is now unified onto the
    // shared strict validators (elements::validate_display_level / validate_derived),
    // matching set_feature_element and add_element_to_feature.
    #[test]
    fn test_add_feature_rejects_negative_display_level_in_element() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [], "CFG_FCLASS": [{"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}],
            "CFG_FELEM": [], "CFG_FBOM": []
        }}"#;
        let elements = json!([{"element": "MYELEM", "displaylevel": -1}]);
        let params = AddFeatureParams {
            feature: "MYFEAT",
            element_list: &elements,
            ..Default::default()
        };
        let err = add_feature(config, params).unwrap_err();
        assert!(
            matches!(err, SzConfigError::InvalidInput(ref m) if m.contains("DISPLAY_LEVEL")),
            "expected DISPLAY_LEVEL InvalidInput, got: {err:?}"
        );
    }

    #[test]
    fn test_add_feature_rejects_invalid_derived_in_element() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [], "CFG_FCLASS": [{"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}],
            "CFG_FELEM": [], "CFG_FBOM": []
        }}"#;
        let elements = json!([{"element": "MYELEM", "derived": "maybe"}]);
        let params = AddFeatureParams {
            feature: "MYFEAT",
            element_list: &elements,
            ..Default::default()
        };
        let err = add_feature(config, params).unwrap_err();
        assert!(
            matches!(err, SzConfigError::InvalidInput(ref m) if m.contains("DERIVED")),
            "expected DERIVED InvalidInput, got: {err:?}"
        );
    }

    #[test]
    fn test_add_feature_caller_supplied_id() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1000, "FTYPE_CODE": "TAKEN"}],
            "CFG_FCLASS": [{"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}],
            "CFG_FELEM": [],
            "CFG_FBOM": []
        }}"#;
        let elements = json!([{"element": "MYELEM"}]);

        // Specific free id honoured.
        let params = AddFeatureParams::new("MYFEAT", &elements).with_id(2500);
        let modified = add_feature(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let ftype = value["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(ftype["FTYPE_ID"], json!(2500));

        // Taken id rejected.
        let params = AddFeatureParams::new("MYFEAT", &elements).with_id(1000);
        let err = add_feature(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);

        // Auto-assign lands above the existing max (1000) -> 1001.
        let params = AddFeatureParams::new("MYFEAT", &elements);
        let modified = add_feature(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let ftype = value["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(ftype["FTYPE_ID"], json!(1001));
    }

    /// add_feature with standardize/expression/comparison functions must write
    /// complete CFG_SFCALL, CFG_EFCALL, CFG_CFCALL, CFG_EFBOM and CFG_CFBOM rows.
    /// CFG_CFCALL has no EXEC_ORDER column in the Senzing v4 schema.
    #[test]
    fn test_add_feature_calls_emit_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [],
            "CFG_FCLASS": [{"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}],
            "CFG_FELEM": [],
            "CFG_FBOM": [],
            "CFG_SFCALL": [],
            "CFG_EFCALL": [],
            "CFG_CFCALL": [],
            "CFG_EFBOM": [],
            "CFG_CFBOM": [],
            "CFG_SFUNC": [{"SFUNC_ID": 1, "SFUNC_CODE": "MYSTD"}],
            "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "MYEXP"}],
            "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "MYCMP"}]
        }}"#;

        let elements = json!([{"element": "MYELEM", "expressed": "Yes", "compared": "Yes"}]);
        let params = AddFeatureParams {
            feature: "MYFEAT",
            element_list: &elements,
            standardize: Some("MYSTD"),
            expression: Some("MYEXP"),
            comparison: Some("MYCMP"),
            ..Default::default()
        };
        let modified = add_feature(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &value["G2_CONFIG"];

        assert_all_keys(
            &g2["CFG_SFCALL"][0],
            &[
                "SFCALL_ID",
                "SFUNC_ID",
                "EXEC_ORDER",
                "FTYPE_ID",
                "FELEM_ID",
            ],
        );
        assert_all_keys(
            &g2["CFG_EFCALL"][0],
            &[
                "EFCALL_ID",
                "EFUNC_ID",
                "EXEC_ORDER",
                "FTYPE_ID",
                "FELEM_ID",
                "EFEAT_FTYPE_ID",
                "IS_VIRTUAL",
            ],
        );

        // CFG_CFCALL has no EXEC_ORDER column in the schema.
        let cfcall = &g2["CFG_CFCALL"][0];
        assert_all_keys(cfcall, &["CFCALL_ID", "CFUNC_ID", "FTYPE_ID"]);
        assert!(
            !cfcall.as_object().unwrap().contains_key("EXEC_ORDER"),
            "CFG_CFCALL must NOT carry EXEC_ORDER"
        );

        assert_all_keys(
            &g2["CFG_EFBOM"][0],
            &[
                "EFCALL_ID",
                "EXEC_ORDER",
                "FTYPE_ID",
                "FELEM_ID",
                "FELEM_REQ",
            ],
        );
        assert_all_keys(
            &g2["CFG_CFBOM"][0],
            &["CFCALL_ID", "EXEC_ORDER", "FTYPE_ID", "FELEM_ID"],
        );
    }

    /// add_feature_comparison must write a complete CFG_FBOM row: every key
    /// present, EXEC_ORDER now auto-allocated (whole-table scope, never null),
    /// and the remaining unset optionals present as null.
    #[test]
    fn test_add_feature_comparison_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "PERSON"}],
            "CFG_FELEM": [{"FELEM_ID": 5, "FELEM_CODE": "MYELEM"}],
            "CFG_FBOM": []
        }}"#;

        let params = AddFeatureComparisonParams::new("PERSON", "MYELEM");
        let modified = add_feature_comparison(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let fbom = &value["G2_CONFIG"]["CFG_FBOM"][0];

        assert_all_keys(fbom, &FBOM_KEYS);
        assert_eq!(fbom["FTYPE_ID"], json!(1));
        assert_eq!(fbom["FELEM_ID"], json!(5));
        // EXEC_ORDER auto-allocated over an empty table -> 1 (never null).
        assert_eq!(fbom["EXEC_ORDER"], json!(1));
        // The remaining unset optionals are present as null.
        assert_eq!(fbom["DISPLAY_LEVEL"], Value::Null);
        assert_eq!(fbom["DISPLAY_DELIM"], Value::Null);
        assert_eq!(fbom["DERIVED"], Value::Null);
    }

    /// add_feature_comparison honours a free EXEC_ORDER and rejects a taken one
    /// (whole-table scope), mirroring Python do_addElementToFeature.
    #[test]
    fn test_add_feature_comparison_honours_and_rejects_exec_order() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "PERSON"}],
            "CFG_FELEM": [
                {"FELEM_ID": 5, "FELEM_CODE": "MYELEM"},
                {"FELEM_ID": 6, "FELEM_CODE": "OTHERELEM"}
            ],
            "CFG_FBOM": [
                {"FTYPE_ID": 1, "FELEM_ID": 5, "EXEC_ORDER": 3,
                 "DISPLAY_LEVEL": 1, "DISPLAY_DELIM": null, "DERIVED": "No"}
            ]
        }}"#;

        // A free order is honoured.
        let params = AddFeatureComparisonParams::new("PERSON", "OTHERELEM").with_exec_order(7);
        let modified = add_feature_comparison(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let new_row = value["G2_CONFIG"]["CFG_FBOM"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["FELEM_ID"].as_i64() == Some(6))
            .unwrap();
        assert_eq!(new_row["EXEC_ORDER"], json!(7));

        // A taken order (3, used whole-table) is rejected.
        let params = AddFeatureComparisonParams::new("PERSON", "OTHERELEM").with_exec_order(3);
        let err = add_feature_comparison(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    /// add_feature_distinct_call_element must write a CFG_DFCALL row with exactly
    /// the three authoritative columns: DFCALL_ID, FTYPE_ID, DFUNC_ID. FELEM_ID
    /// and EXEC_ORDER belong to CFG_DFBOM, not the DFCALL header row.
    #[test]
    fn test_add_feature_distinct_call_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "PERSON"}],
            "CFG_DFUNC": [{"DFUNC_ID": 2, "DFUNC_CODE": "MYDIST"}],
            "CFG_DFCALL": []
        }}"#;

        let params = AddFeatureDistinctCallElementParams::new("PERSON", "MYDIST");
        let modified = add_feature_distinct_call_element(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let dfcall = &value["G2_CONFIG"]["CFG_DFCALL"][0];

        assert_all_keys(dfcall, &["DFCALL_ID", "FTYPE_ID", "DFUNC_ID"]);
        assert_eq!(
            dfcall.as_object().unwrap().len(),
            3,
            "DFCALL is exactly 3 columns"
        );
        assert_eq!(dfcall["FTYPE_ID"], json!(1));
        assert_eq!(dfcall["DFUNC_ID"], json!(2));
        assert!(!dfcall.as_object().unwrap().contains_key("FELEM_ID"));
        assert!(!dfcall.as_object().unwrap().contains_key("EXEC_ORDER"));
    }

    // ------------------------------------------------------------------
    // LOCKED_FEATURES (ratified protected set) — #35
    // ------------------------------------------------------------------

    /// A real feature that is NO LONGER in the protected set (EMAIL) must be
    /// deletable, and its dependent rows must cascade away.
    #[test]
    fn test_delete_feature_email_now_deletable_and_cascades() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [
                {"FTYPE_ID": 5, "FTYPE_CODE": "EMAIL"},
                {"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}
            ],
            "CFG_FBOM": [
                {"FTYPE_ID": 5, "FELEM_ID": 1},
                {"FTYPE_ID": 3, "FELEM_ID": 2}
            ],
            "CFG_ATTR": [
                {"ATTR_CODE": "EMAIL", "FTYPE_CODE": "EMAIL"}
            ],
            "CFG_CFCALL": [
                {"CFCALL_ID": 50, "FTYPE_ID": 5, "CFUNC_ID": 1}
            ],
            "CFG_CFBOM": [
                {"CFCALL_ID": 50, "FELEM_ID": 1}
            ]
        }}"#;

        let modified = delete_feature(config, "EMAIL").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &value["G2_CONFIG"];

        // The EMAIL feature is gone; NAME survives.
        let ftypes = g2["CFG_FTYPE"].as_array().unwrap();
        assert_eq!(ftypes.len(), 1);
        assert_eq!(ftypes[0]["FTYPE_CODE"], json!("NAME"));

        // Cascade: EMAIL's FBOM/ATTR/CFCALL/CFBOM rows removed, NAME's kept.
        let fbom = g2["CFG_FBOM"].as_array().unwrap();
        assert_eq!(fbom.len(), 1);
        assert_eq!(fbom[0]["FTYPE_ID"], json!(3));
        assert!(g2["CFG_ATTR"].as_array().unwrap().is_empty());
        assert!(g2["CFG_CFCALL"].as_array().unwrap().is_empty());
        assert!(g2["CFG_CFBOM"].as_array().unwrap().is_empty());
    }

    /// Every code in the ratified protected set must be blocked from deletion,
    /// case-insensitively.
    #[test]
    fn test_delete_feature_ratified_codes_blocked() {
        for code in [
            "NAME",
            "ADDRESS",
            "PHONE",
            "DOB",
            "REL_LINK",
            "REL_ANCHOR",
            "REL_POINTER",
        ] {
            let config = format!(
                r#"{{"G2_CONFIG": {{"CFG_FTYPE": [{{"FTYPE_ID": 1, "FTYPE_CODE": "{code}"}}]}}}}"#
            );
            // Exact case.
            let err = delete_feature(&config, code).unwrap_err();
            assert_eq!(
                err.kind(),
                crate::error::SzErrorKind::InvalidInput,
                "{code} must be protected"
            );
            // Case-insensitive.
            let err_lower = delete_feature(&config, &code.to_lowercase()).unwrap_err();
            assert_eq!(err_lower.kind(), crate::error::SzErrorKind::InvalidInput);
        }
    }

    /// A former inert entry (a bogus DATE_OF_BIRTH feature) is no longer falsely
    /// protected: deleting it succeeds rather than being blocked.
    #[test]
    fn test_delete_feature_former_inert_entry_not_protected() {
        let config = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 7, "FTYPE_CODE": "DATE_OF_BIRTH"}]
        }}"#;

        // DATE_OF_BIRTH was in the old list but is not a real protected feature;
        // it must now be deletable.
        let modified = delete_feature(config, "DATE_OF_BIRTH").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert!(
            value["G2_CONFIG"]["CFG_FTYPE"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}
