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
}

impl<'a> AddFeatureParams<'a> {
    pub fn new(feature: &'a str, element_list: &'a Value) -> Self {
        Self {
            feature,
            element_list,
            ..Default::default()
        }
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
    pub ftype_id: i64,
    pub felem_id: i64,
    pub exec_order: Option<i64>,
    pub display_level: Option<i64>,
    pub display_delim: Option<&'a str>,
    pub derived: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for AddFeatureComparisonParams<'a> {
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

/// Parameters for getting a feature comparison
#[derive(Debug, Clone)]
pub struct GetFeatureComparisonParams {
    pub ftype_id: i64,
    pub felem_id: i64,
}

impl TryFrom<&Value> for GetFeatureComparisonParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let ftype_id = json
            .get("ftypeId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("ftypeId".to_string()))?;

        let felem_id = json
            .get("felemId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("felemId".to_string()))?;

        Ok(Self { ftype_id, felem_id })
    }
}

/// Parameters for adding a feature distinct call element (CFG_DFCALL)
#[derive(Debug, Clone, Default)]
pub struct AddFeatureDistinctCallElementParams {
    pub ftype_id: i64,
    pub dfunc_id: i64,
    pub felem_id: Option<i64>,
    pub exec_order: Option<i64>,
}

impl AddFeatureDistinctCallElementParams {
    pub fn new(ftype_id: i64, dfunc_id: i64) -> Self {
        Self {
            ftype_id,
            dfunc_id,
            felem_id: None,
            exec_order: None,
        }
    }
}

// Protected features that cannot be deleted
const LOCKED_FEATURES: &[&str] = &[
    "NAME",
    "ADDRESS",
    "PHONE",
    "EMAIL",
    "RECORD_TYPE",
    "DATE_OF_BIRTH",
    "NATIONAL_ID",
    "TAX_ID",
    "ACCT_NUM",
    "SSN_NUM",
    "PASSPORT_NUM",
    "DRIVERS_LICENSE_NUM",
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
pub fn add_feature(
    config_json: &str,
    params: AddFeatureParams,
) -> Result<String> {
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
            "Feature already exists: {}",
            feature_upper
        )));
    }

    // Validate element_list
    let elements = params.element_list
        .as_array()
        .ok_or_else(|| SzConfigError::InvalidInput("elementList must be an array".to_string()))?;

    if elements.is_empty() {
        return Err(SzConfigError::InvalidInput(
            "elementList must contain at least one element".to_string(),
        ));
    }

    // Get defaults from params
    let class = params.class.unwrap_or("OTHER");
    let behavior = params.behavior.unwrap_or("FM");
    let candidates_val = params.candidates.unwrap_or("No");
    let anonymize_val = params.anonymize.unwrap_or("No");
    let derived_val = params.derived.unwrap_or("No");
    let history_val = params.history.unwrap_or("Yes");

    // matchkey default depends on whether comparison is specified
    let matchkey_val = params.matchkey.unwrap_or(if params.comparison.is_some() { "Yes" } else { "No" });

    // Get next FTYPE_ID (seed at 1000 for user-created features)
    let ftype_id = helpers::get_next_id_with_min(ftypes, "FTYPE_ID", 1000)?;

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
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {}", class)))?;

    // Lookup optional functions
    let sfunc_id = if let Some(func_code) = params.standardize {
        lookup_sfunc_id(&config, func_code).unwrap_or(0)
    } else {
        0
    };

    let efunc_id = if let Some(func_code) = params.expression {
        lookup_efunc_id(&config, func_code).unwrap_or(0)
    } else {
        0
    };

    let cfunc_id = if let Some(func_code) = params.comparison {
        lookup_cfunc_id(&config, func_code).unwrap_or(0)
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

    // Create CFG_FTYPE record
    let ftype_record = json!({
        "FTYPE_ID": ftype_id,
        "FTYPE_CODE": feature_upper.clone(),
        "FTYPE_DESC": feature_upper.clone(),
        "FCLASS_ID": fclass_id,
        "FTYPE_FREQ": frequency,
        "FTYPE_EXCL": exclusivity,
        "FTYPE_STAB": stability,
        "ANONYMIZE": anonymize_val,
        "DERIVED": derived_val,
        "USED_FOR_CAND": candidates_val,
        "SHOW_IN_MATCH_KEY": matchkey_val,
        "PERSIST_HISTORY": history_val,
        "VERSION": params.version.unwrap_or(1),
        "RTYPE_ID": params.rtype_id.unwrap_or(0)
    });

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
        let record = json!({
            "SFCALL_ID": id,
            "SFUNC_ID": sfunc_id,
            "EXEC_ORDER": 1,
            "FTYPE_ID": ftype_id,
            "FELEM_ID": -1
        });
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
        let record = json!({
            "EFCALL_ID": id,
            "EFUNC_ID": efunc_id,
            "EXEC_ORDER": 1,
            "FTYPE_ID": ftype_id,
            "FELEM_ID": -1,
            "EFEAT_FTYPE_ID": -1,
            "IS_VIRTUAL": "No"
        });
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
        let record = json!({
            "CFCALL_ID": id,
            "CFUNC_ID": cfunc_id,
            "FTYPE_ID": ftype_id
        });
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
                            "Missing element code in elementList item {}",
                            fbom_order
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
                let disp_level = if let Some(display) = elem_obj
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

                let disp_delim = elem_obj
                    .get("displaydelim")
                    .or_else(|| elem_obj.get("DISPLAYDELIM"))
                    .or_else(|| elem_obj.get("display_delim"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let elem_deriv = elem_obj
                    .get("derived")
                    .or_else(|| elem_obj.get("DERIVED"))
                    .and_then(|v| v.as_str())
                    .map(|s| {
                        if s.eq_ignore_ascii_case("yes") {
                            "Yes"
                        } else {
                            "No"
                        }
                        .to_string()
                    })
                    .unwrap_or_else(|| "No".to_string());

                (code, expr, comp, disp_level, disp_delim, elem_deriv)
            } else {
                return Err(SzConfigError::InvalidInput(format!(
                    "Invalid element in elementList item {}",
                    fbom_order
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
            let new_element = json!({
                "FELEM_ID": new_id,
                "FELEM_CODE": element_code.clone(),
                "FELEM_DESC": element_code.clone(),
                "DATA_TYPE": "string",
                "TOKENIZE": "No"
            });
            if let Some(array) = config["G2_CONFIG"]["CFG_FELEM"].as_array_mut() {
                array.push(new_element);
            }
            new_id
        };

        // Add to EFBOM if expressed
        if efcall_id > 0 && expressed.eq_ignore_ascii_case("yes") {
            let record = json!({
                "EFCALL_ID": efcall_id,
                "EXEC_ORDER": fbom_order,
                "FTYPE_ID": ftype_id,
                "FELEM_ID": felem_id,
                "FELEM_REQ": "Yes"
            });
            if let Some(array) = config["G2_CONFIG"]["CFG_EFBOM"].as_array_mut() {
                array.push(record);
            }
        }

        // Add to CFBOM if compared
        if cfcall_id > 0 && compared.eq_ignore_ascii_case("yes") {
            let record = json!({
                "CFCALL_ID": cfcall_id,
                "EXEC_ORDER": fbom_order,
                "FTYPE_ID": ftype_id,
                "FELEM_ID": felem_id
            });
            if let Some(array) = config["G2_CONFIG"]["CFG_CFBOM"].as_array_mut() {
                array.push(record);
            }
        }

        // Add to FBOM (always)
        let mut fbom_record = json!({
            "FTYPE_ID": ftype_id,
            "FELEM_ID": felem_id,
            "EXEC_ORDER": fbom_order,
            "DISPLAY_LEVEL": display_level,
            "DERIVED": elem_derived
        });

        if let Some(delim) = display_delim {
            fbom_record["DISPLAY_DELIM"] = json!(delim);
        }

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
            return Err(SzConfigError::NotFound(format!("Feature: {}", id)));
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", ftype_id)))?
        .to_string();

    // Check if feature is locked
    if LOCKED_FEATURES
        .iter()
        .any(|&locked| locked.eq_ignore_ascii_case(&feature_code))
    {
        return Err(SzConfigError::InvalidInput(format!(
            "The feature {} cannot be deleted (it is a protected system feature)",
            feature_code
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
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", id)))?
    } else {
        let code_upper = feature_code_or_id.to_uppercase();
        config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|f| f["FTYPE_CODE"].as_str() == Some(code_upper.as_str()))
            })
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", code_upper)))?
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
pub fn set_feature(
    config_json: &str,
    params: SetFeatureParams,
) -> Result<String> {
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", ftype_id)))?;

    // Update fields if provided
    if let Some(val) = params.candidates {
        ftype["USED_FOR_CAND"] = json!(val);
    }
    if let Some(val) = params.anonymize {
        ftype["ANONYMIZE"] = json!(val);
    }
    if let Some(val) = params.derived {
        ftype["DERIVED"] = json!(val);
    }
    if let Some(val) = params.history {
        ftype["PERSIST_HISTORY"] = json!(val);
    }
    if let Some(val) = params.matchkey {
        ftype["SHOW_IN_MATCH_KEY"] = json!(val);
    }
    if let Some(val) = params.version {
        ftype["VERSION"] = json!(val);
    }
    if let Some(val) = params.rtype_id {
        ftype["RTYPE_ID"] = json!(val);
    }

    // Parse and set behavior (FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB)
    if let Some(behavior_code) = params.behavior {
        let (frequency, exclusivity, stability) = parse_behavior_code(behavior_code)?;
        ftype["FTYPE_FREQ"] = json!(frequency);
        ftype["FTYPE_EXCL"] = json!(exclusivity);
        ftype["FTYPE_STAB"] = json!(stability);
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
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {}", class_name)))?;

        ftype["FCLASS_ID"] = json!(fclass_id);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
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

/// Parse a behavior code string into (frequency, exclusivity, stability)
/// Valid frequency codes: A1, F1, FF, FM, FVM, NONE, NAME
/// E suffix means EXCLUSIVITY = "Yes"
/// S suffix means STABILITY = "Yes"
fn parse_behavior_code(behavior: &str) -> Result<(&'static str, &'static str, &'static str)> {
    let mut code = behavior.to_uppercase();
    let mut exclusivity = "No";
    let mut stability = "No";

    // Special cases that don't get E/S parsing
    if code != "NAME" && code != "NONE" {
        if code.contains('E') {
            exclusivity = "Yes";
            code = code.replace('E', "");
        }
        if code.contains('S') {
            stability = "Yes";
            code = code.replace('S', "");
        }
    }

    // Validate frequency code
    let frequency: &'static str = match code.as_str() {
        "A1" => "A1",
        "F1" => "F1",
        "FF" => "FF",
        "FM" => "FM",
        "FVM" => "FVM",
        "NONE" => "NONE",
        "NAME" => "NAME",
        _ => {
            return Err(SzConfigError::InvalidInput(format!(
                "Invalid behavior code '{}'. Valid codes: A1, F1, FF, FM, FVM, NONE, NAME (with optional E/S suffixes)",
                behavior
            )));
        }
    };

    Ok((frequency, exclusivity, stability))
}

fn compute_behavior(ftype: &Value) -> String {
    let freq = ftype["FTYPE_FREQ"].as_str().unwrap_or("");
    let excl = ftype["FTYPE_EXCL"].as_str().unwrap_or("");
    let stab = ftype["FTYPE_STAB"].as_str().unwrap_or("");

    let mut behavior = freq.to_string();
    if excl.to_uppercase() == "Y" || excl == "1" || excl.to_uppercase() == "YES" {
        behavior.push('E');
    }
    if stab.to_uppercase() == "Y" || stab == "1" || stab.to_uppercase() == "YES" {
        behavior.push('S');
    }
    behavior
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", code_upper)))
}

fn lookup_sfunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_SFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["SFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["SFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Standardize function: {}", code_upper)))
}

fn lookup_efunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_EFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["EFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["EFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Expression function: {}", code_upper)))
}

fn lookup_cfunc_id(config: &Value, func_code: &str) -> Result<i64> {
    let code_upper = func_code.to_uppercase();
    config["G2_CONFIG"]["CFG_CFUNC"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|f| f["CFUNC_CODE"].as_str() == Some(code_upper.as_str()))
        })
        .and_then(|f| f["CFUNC_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Comparison function: {}", code_upper)))
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
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    // Check if already exists
    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    if fbom_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(params.ftype_id) && item["FELEM_ID"].as_i64() == Some(params.felem_id)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature comparison: FTYPE_ID={}, FELEM_ID={}",
            params.ftype_id, params.felem_id
        )));
    }

    // Build record
    let mut record = json!({
        "FTYPE_ID": params.ftype_id,
        "FELEM_ID": params.felem_id,
    });

    if let Some(order) = params.exec_order {
        record["EXEC_ORDER"] = json!(order);
    }
    if let Some(level) = params.display_level {
        record["DISPLAY_LEVEL"] = json!(level);
    }
    if let Some(delim) = params.display_delim {
        record["DISPLAY_DELIM"] = json!(delim);
    }
    if let Some(der) = params.derived {
        record["DERIVED"] = json!(der);
    }

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
    ftype_id: i64,
    felem_id: i64,
) -> Result<String> {
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
            "Feature comparison: FTYPE_ID={}, FELEM_ID={}",
            ftype_id, felem_id
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
pub fn get_feature_comparison(config_json: &str, params: GetFeatureComparisonParams) -> Result<Value> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let fbom_array = config["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FBOM".to_string()))?;

    fbom_array
        .iter()
        .find(|item| {
            item["FTYPE_ID"].as_i64() == Some(params.ftype_id)
                && item["FELEM_ID"].as_i64() == Some(params.felem_id)
        })
        .cloned()
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Feature comparison: FTYPE_ID={}, FELEM_ID={}",
                params.ftype_id, params.felem_id
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
    ftype_id: i64,
    felem_id: i64,
) -> Result<String> {
    delete_feature_comparison(config_json, ftype_id, felem_id)
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
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let felem = params.felem_id.unwrap_or(-1);

    // Check if already exists
    let dfcall_array = config["G2_CONFIG"]["CFG_DFCALL"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_DFCALL".to_string()))?;

    if dfcall_array.iter().any(|item| {
        item["FTYPE_ID"].as_i64() == Some(params.ftype_id)
            && item["DFUNC_ID"].as_i64() == Some(params.dfunc_id)
            && item["FELEM_ID"].as_i64() == Some(felem)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Feature distinct call element: FTYPE_ID={}, DFUNC_ID={}, FELEM_ID={}",
            params.ftype_id, params.dfunc_id, felem
        )));
    }

    // Get next DFCALL_ID
    let dfcall_id = helpers::get_next_id_with_min(dfcall_array, "DFCALL_ID", 1000)?;

    // Build record
    let mut record = json!({
        "DFCALL_ID": dfcall_id,
        "FTYPE_ID": params.ftype_id,
        "DFUNC_ID": params.dfunc_id,
        "FELEM_ID": felem,
    });

    if let Some(order) = params.exec_order {
        record["EXEC_ORDER"] = json!(order);
    }

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
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {}", id)))
    } else {
        // Try as code
        let code_upper = fclass_id_or_code.to_uppercase();
        fclass_array
            .iter()
            .find(|item| item["FCLASS_CODE"].as_str() == Some(code_upper.as_str()))
            .cloned()
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature class: {}", code_upper)))
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

    // Navigate to COMPATIBILITY_VERSION
    let compat_version = config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]
        .as_object_mut()
        .ok_or_else(|| SzConfigError::MissingSection("COMPATIBILITY_VERSION".to_string()))?;

    compat_version.insert("FEATURE_VERSION".to_string(), json!(version));

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}
