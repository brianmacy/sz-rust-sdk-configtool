//! Rule (CFG_ERRULE) operations
//!
//! Functions for managing entity resolution rules in the configuration.
//! Rules define matching and relationship logic based on fragments.

use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for setting (updating) a rule
#[derive(Debug, Clone)]
pub struct SetRuleParams<'a> {
    pub code: &'a str,
    pub resolve: Option<&'a str>,
    pub relate: Option<&'a str>,
    pub rtype_id: Option<i64>,
}

impl<'a> TryFrom<&'a Value> for SetRuleParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .or_else(|| json.get("rule").and_then(|v| v.as_str()))
            .ok_or_else(|| SzConfigError::MissingField("code or rule".to_string()))?;

        Ok(Self {
            code,
            resolve: json.get("resolve").and_then(|v| v.as_str())
                .or_else(|| json.get("RESOLVE").and_then(|v| v.as_str())),
            relate: json.get("relate").and_then(|v| v.as_str())
                .or_else(|| json.get("RELATE").and_then(|v| v.as_str())),
            rtype_id: json.get("rtypeId").and_then(|v| v.as_i64())
                .or_else(|| json.get("RTYPE_ID").and_then(|v| v.as_i64())),
        })
    }
}

/// Add a new rule to the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `rule_config` - JSON configuration for the rule (must include ERRULE_CODE)
///
/// # Returns
///
/// Returns `(modified_config, new_rule_id)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": []}}"#;
/// let rule_config = json!({
///     "ERRULE_CODE": "CUSTOM_RULE",
///     "RESOLVE": "Yes",
///     "RELATE": "No",
///     "RTYPE_ID": 1
/// });
/// let (modified, rule_id) = rules::add_rule(config, &rule_config).unwrap();
/// assert_eq!(rule_id, 1);
/// ```
pub fn add_rule(config_json: &str, rule_config: &Value) -> Result<(String, i64)> {
    let code = rule_config
        .get("ERRULE_CODE")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::MissingField("ERRULE_CODE".to_string()))?;

    let config_data: Value = serde_json::from_str(config_json)?;

    // Get next ID
    let next_id = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(array) = g2_config.get("CFG_ERRULE").and_then(|v| v.as_array()) {
            array
                .iter()
                .filter_map(|item| item.get("ERRULE_ID").and_then(|v| v.as_i64()))
                .max()
                .unwrap_or(0)
                + 1
        } else {
            1
        }
    } else {
        return Err(SzConfigError::InvalidConfig(
            "G2_CONFIG not found".to_string(),
        ));
    };

    // Create new item with provided config plus ID
    let mut new_item = rule_config.clone();
    if let Some(obj) = new_item.as_object_mut() {
        obj.insert("ERRULE_ID".to_string(), json!(next_id));
        obj.insert("ERRULE_CODE".to_string(), json!(code.to_uppercase()));
    }

    // Add to config
    let modified_json = helpers::add_to_config_array(config_json, "CFG_ERRULE", new_item)?;

    Ok((modified_json, next_id))
}

/// Delete a rule from the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `rule_code` - Rule code to delete
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST"}]}}"#;
/// let modified = rules::delete_rule(config, "TEST").unwrap();
/// ```
pub fn delete_rule(config_json: &str, rule_code: &str) -> Result<String> {
    let rule_code = rule_code.to_uppercase();

    // Verify rule exists before deletion
    let _ = helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &rule_code)?
        .ok_or_else(|| SzConfigError::NotFound(format!("Rule not found: {}", rule_code)))?;

    // Remove from config
    helpers::remove_from_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &rule_code)
}

/// Get a rule by code or ID
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `code_or_id` - Rule code or ID to search for
///
/// # Returns
///
/// Returns the rule JSON object on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST"}]}}"#;
/// let rule = rules::get_rule(config, "TEST").unwrap();
/// ```
pub fn get_rule(config_json: &str, code_or_id: &str) -> Result<Value> {
    let search_value = code_or_id.to_uppercase();

    // Try to find by CODE first, then by ID
    if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &search_value)?
    {
        Ok(item)
    } else if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_ID", &search_value)?
    {
        Ok(item)
    } else {
        Err(SzConfigError::NotFound(format!(
            "Rule not found: {}",
            search_value
        )))
    }
}

/// List all rules in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns a vector of rule objects in Python sz_configtool format
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST", "RESOLVE": "Yes", "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "", "DISQ_ERFRAG_CODE": "", "ERRULE_TIER": 10}]}}"#;
/// let rules = rules::list_rules(config).unwrap();
/// assert_eq!(rules.len(), 1);
/// ```
pub fn list_rules(config_json: &str) -> Result<Vec<Value>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    // Extract rules and transform to Python format
    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(array) = g2_config.get("CFG_ERRULE").and_then(|v| v.as_array()) {
            array
                .iter()
                .map(|item| {
                    let resolve = item.get("RESOLVE").and_then(|v| v.as_str()).unwrap_or("");
                    let tier = if resolve == "Yes" {
                        item.get("ERRULE_TIER").and_then(|v| v.as_i64())
                    } else {
                        None
                    };

                    json!({
                        "id": item.get("ERRULE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                        "rule": item.get("ERRULE_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                        "resolve": resolve,
                        "relate": item.get("RELATE").and_then(|v| v.as_str()).unwrap_or(""),
                        "rtype_id": item.get("RTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                        "fragment": item.get("QUAL_ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                        "disqualifier": item.get("DISQ_ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                        "tier": tier
                    })
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Update an existing rule in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `rule_code` - Rule code to update
/// * `rule_config` - New configuration for the rule
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST"}]}}"#;
/// let params = rules::SetRuleParams {
///     code: "TEST",
///     resolve: Some("Yes"),
///     relate: Some("No"),
///     rtype_id: None,
/// };
/// let modified = rules::set_rule(config, params).unwrap();
/// ```
pub fn set_rule(config_json: &str, params: SetRuleParams) -> Result<String> {
    let code = params.code.to_uppercase();

    // Build update object from params
    let mut updated_item = json!({
        "ERRULE_CODE": code.clone()
    });

    if let Some(obj) = updated_item.as_object_mut() {
        if let Some(resolve) = params.resolve {
            obj.insert("RESOLVE".to_string(), json!(resolve));
        }
        if let Some(relate) = params.relate {
            obj.insert("RELATE".to_string(), json!(relate));
        }
        if let Some(rtype_id) = params.rtype_id {
            obj.insert("RTYPE_ID".to_string(), json!(rtype_id));
        }
    }

    // Update the item in the config
    helpers::update_in_config_array(
        config_json,
        "CFG_ERRULE",
        "ERRULE_CODE",
        &code,
        updated_item,
    )
}
