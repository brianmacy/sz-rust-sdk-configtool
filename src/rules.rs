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
    pub fragment: Option<&'a str>,
    pub disqualifier: Option<&'a str>,
    pub tier: Option<i64>,
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
            resolve: json
                .get("resolve")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("RESOLVE").and_then(|v| v.as_str())),
            relate: json
                .get("relate")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("RELATE").and_then(|v| v.as_str())),
            rtype_id: json
                .get("rtypeId")
                .and_then(|v| v.as_i64())
                .or_else(|| json.get("RTYPE_ID").and_then(|v| v.as_i64())),
            fragment: json
                .get("fragment")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("FRAGMENT").and_then(|v| v.as_str())),
            disqualifier: json
                .get("disqualifier")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("DISQUALIFIER").and_then(|v| v.as_str())),
            tier: json
                .get("tier")
                .and_then(|v| v.as_i64())
                .or_else(|| json.get("TIER").and_then(|v| v.as_i64())),
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
/// // ID parameter is required (0 for auto-assign, >0 for specific ID)
/// let (_modified, _rule_id) = rules::add_rule(config, 0, &rule_config).unwrap();
/// ```
pub fn add_rule(config_json: &str, id: i64, rule_config: &Value) -> Result<(String, i64)> {
    let code = rule_config
        .get("ERRULE_CODE")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::MissingField("ERRULE_CODE".to_string()))?;

    // Validate ID not already taken
    let config_data: Value = serde_json::from_str(config_json)?;
    if let Some(errule_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ERRULE"))
        .and_then(|v| v.as_array())
    {
        if errule_array
            .iter()
            .any(|item| item.get("ERRULE_ID").and_then(|v| v.as_i64()) == Some(id))
        {
            return Err(SzConfigError::AlreadyExists(
                "The specified ID is already taken".to_string(),
            ));
        }
    }

    // Create new item with provided config plus ID
    let mut new_item = rule_config.clone();
    if let Some(obj) = new_item.as_object_mut() {
        obj.insert("ERRULE_ID".to_string(), json!(id));
        obj.insert("ERRULE_CODE".to_string(), json!(code.to_uppercase()));
    }

    // Add to config
    let modified_json = helpers::add_to_config_array(config_json, "CFG_ERRULE", new_item)?;

    Ok((modified_json, id))
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
        .ok_or_else(|| SzConfigError::NotFound(format!("Rule not found: {rule_code}")))?;

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
    let item = if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &search_value)?
    {
        item
    } else if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_ID", &search_value)?
    {
        item
    } else {
        return Err(SzConfigError::NotFound(format!(
            "Rule not found: {search_value}"
        )));
    };

    // Transform to lowercase format (matching list_rules for consistency)
    let resolve = item.get("RESOLVE").and_then(|v| v.as_str()).unwrap_or("");
    let tier = if resolve == "Yes" {
        item.get("ERRULE_TIER").and_then(|v| v.as_i64())
    } else {
        None
    };

    Ok(json!({
        "id": item.get("ERRULE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
        "rule": item.get("ERRULE_CODE").and_then(|v| v.as_str()).unwrap_or(""),
        "resolve": resolve,
        "relate": item.get("RELATE").and_then(|v| v.as_str()).unwrap_or(""),
        "rtype_id": item.get("RTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0),
        "fragment": item.get("QUAL_ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
        "disqualifier": item.get("DISQ_ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
        "tier": tier
    }))
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
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST", "RESOLVE": "No"}], "CFG_ERFRAG": []}}"#;
/// let params = rules::SetRuleParams {
///     code: "TEST",
///     resolve: Some("Yes"),
///     relate: Some("No"),
///     rtype_id: None,
///     fragment: None,
///     disqualifier: None,
///     tier: None,
/// };
/// let modified = rules::set_rule(config, params).unwrap();
/// ```
pub fn set_rule(config_json: &str, params: SetRuleParams) -> Result<String> {
    let code = params.code.to_uppercase();

    // Get existing rule to validate and merge updates
    let existing_rule =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &code)?
            .ok_or_else(|| SzConfigError::NotFound(format!("Rule not found: {code}")))?;

    // Validate NEW fragment if being updated (line 4683-4686)
    if let Some(frag) = params.fragment {
        let frag_upper = frag.to_uppercase();
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &frag_upper)?
            .ok_or_else(|| SzConfigError::NotFound(format!("Fragment '{frag_upper}' not found")))?;
    }

    // Validate NEW disqualifier if being updated (line 4688-4692)
    if let Some(disq) = params.disqualifier {
        let disq_upper = disq.to_uppercase();
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &disq_upper)?
            .ok_or_else(|| SzConfigError::NotFound(format!("Fragment '{disq_upper}' not found")))?;
    }

    // Determine final RESOLVE value (from params or existing)
    let resolve_value = params
        .resolve
        .or_else(|| existing_rule.get("RESOLVE").and_then(|v| v.as_str()))
        .unwrap_or("No");

    // CHECK 3: RESOLVE domain validation (line 4694-4697)
    let resolve_upper = resolve_value.to_uppercase();
    if resolve_upper != "YES" && resolve_upper != "NO" {
        return Err(SzConfigError::InvalidInput(
            "resolve value must be in [\"Yes\", \"No\"]".to_string(),
        ));
    }
    let final_resolve = if resolve_upper == "YES" { "Yes" } else { "No" };

    // Determine final RELATE value (from params or existing)
    let relate_value = params
        .relate
        .or_else(|| existing_rule.get("RELATE").and_then(|v| v.as_str()))
        .unwrap_or("No");

    // CHECK 4: RELATE domain validation (line 4699-4702)
    let relate_upper = relate_value.to_uppercase();
    if relate_upper != "YES" && relate_upper != "NO" {
        return Err(SzConfigError::InvalidInput(
            "relate value must be in [\"Yes\", \"No\"]".to_string(),
        ));
    }
    let final_relate = if relate_upper == "YES" { "Yes" } else { "No" };

    // CHECK 5: Can't have both RESOLVE=Yes AND RELATE=Yes (line 4704-4709)
    if final_resolve == "Yes" && final_relate == "Yes" {
        return Err(SzConfigError::InvalidInput(
            "A rule must either resolve or relate, please set the other to No".to_string(),
        ));
    }

    // Determine final RTYPE_ID (from params or existing)
    let mut final_rtype_id = params
        .rtype_id
        .or_else(|| existing_rule.get("RTYPE_ID").and_then(|v| v.as_i64()))
        .unwrap_or(1);

    // AUTO-CORRECT: RESOLVE=Yes forces RTYPE_ID to 1 (Python line 4722-4725)
    // "just do it without making them wonder"
    if final_resolve == "Yes" && final_rtype_id != 1 {
        final_rtype_id = 1;
    }

    // CHECK 8: RELATE=Yes requires RTYPE_ID in [2, 3, 4] (line 4731-4736)
    if final_relate == "Yes" && ![2, 3, 4].contains(&final_rtype_id) {
        return Err(SzConfigError::InvalidInput(
            "Relationship type (RTYPE_ID) must be set to either 2=Possible match or 3=Possibly related".to_string(),
        ));
    }

    // Extract ERRULE_ID from existing rule to preserve it
    let errule_id = existing_rule
        .get("ERRULE_ID")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Build update object from params with validated values
    let mut updated_item = json!({
        "ERRULE_ID": errule_id,
        "ERRULE_CODE": code.clone()
    });

    if let Some(obj) = updated_item.as_object_mut() {
        if params.resolve.is_some() {
            obj.insert("RESOLVE".to_string(), json!(final_resolve));
        }
        if params.relate.is_some() {
            obj.insert("RELATE".to_string(), json!(final_relate));
        }
        // Insert RTYPE_ID if explicitly provided OR if resolve was updated (auto-correction may have occurred)
        if params.rtype_id.is_some() || params.resolve.is_some() {
            obj.insert("RTYPE_ID".to_string(), json!(final_rtype_id));
        }
        if let Some(frag) = params.fragment {
            obj.insert("QUAL_ERFRAG_CODE".to_string(), json!(frag.to_uppercase()));
        }
        if let Some(disq) = params.disqualifier {
            obj.insert("DISQ_ERFRAG_CODE".to_string(), json!(disq.to_uppercase()));
        }
        if let Some(tier) = params.tier {
            obj.insert("ERRULE_TIER".to_string(), json!(tier));
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
