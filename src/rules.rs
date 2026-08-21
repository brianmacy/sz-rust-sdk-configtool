//! Rule (CFG_ERRULE) operations
//!
//! Functions for managing entity resolution rules in the configuration.
//! Rules define matching and relationship logic based on fragments.

use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_ERRULE row.
///
/// This struct is the single source of truth for the on-disk shape of a rule.
/// It derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted — optional fields serialize as JSON `null` rather than being dropped.
/// The Senzing engine's config loader requires every key to be present (a
/// missing key yields SENZ9117), so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
struct ErruleRow {
    #[serde(rename = "ERRULE_ID")]
    errule_id: i64,
    #[serde(rename = "ERRULE_CODE")]
    errule_code: String,
    #[serde(rename = "RESOLVE")]
    resolve: String,
    #[serde(rename = "RELATE")]
    relate: String,
    #[serde(rename = "RTYPE_ID")]
    rtype_id: i64,
    #[serde(rename = "QUAL_ERFRAG_CODE")]
    qual_erfrag_code: Option<String>,
    #[serde(rename = "DISQ_ERFRAG_CODE")]
    disq_erfrag_code: Option<String>,
    #[serde(rename = "ERRULE_TIER")]
    errule_tier: Option<i64>,
}

impl ErruleRow {
    /// Build a complete row from a caller-provided JSON object plus a resolved
    /// id and code. Missing scalar fields fall back to Senzing defaults and
    /// missing optional fields become `None` (serialized as `null`), so the
    /// resulting row always carries every CFG_ERRULE key.
    fn from_config(id: i64, code: String, cfg: &Value) -> Self {
        Self {
            errule_id: id,
            errule_code: code,
            resolve: cfg
                .get("RESOLVE")
                .and_then(|v| v.as_str())
                .unwrap_or("No")
                .to_string(),
            relate: cfg
                .get("RELATE")
                .and_then(|v| v.as_str())
                .unwrap_or("No")
                .to_string(),
            rtype_id: cfg.get("RTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(1),
            qual_erfrag_code: helpers::field_as_string(cfg, "QUAL_ERFRAG_CODE"),
            disq_erfrag_code: helpers::field_as_string(cfg, "DISQ_ERFRAG_CODE"),
            errule_tier: cfg.get("ERRULE_TIER").and_then(|v| v.as_i64()),
        }
    }
}

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
        && errule_array
            .iter()
            .any(|item| item.get("ERRULE_ID").and_then(|v| v.as_i64()) == Some(id))
    {
        return Err(SzConfigError::AlreadyExists(
            "The specified ID is already taken".to_string(),
        ));
    }

    // Build a complete row via ErruleRow so every CFG_ERRULE key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = ErruleRow::from_config(id, code.to_uppercase(), rule_config);
    let new_item = serde_json::to_value(&row)?;

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

    // Build the complete row. Optional fields not supplied in `params` fall back
    // to the existing value (which may itself be null). Because ErruleRow always
    // serializes every key (None -> null), the resulting object can never be
    // missing a key the engine config loader requires.
    let row = ErruleRow {
        errule_id,
        errule_code: code.clone(),
        resolve: final_resolve.to_string(),
        relate: final_relate.to_string(),
        rtype_id: final_rtype_id,
        qual_erfrag_code: params
            .fragment
            .map(str::to_uppercase)
            .or_else(|| helpers::field_as_string(&existing_rule, "QUAL_ERFRAG_CODE")),
        disq_erfrag_code: params
            .disqualifier
            .map(str::to_uppercase)
            .or_else(|| helpers::field_as_string(&existing_rule, "DISQ_ERFRAG_CODE")),
        errule_tier: params
            .tier
            .or_else(|| existing_rule.get("ERRULE_TIER").and_then(|v| v.as_i64())),
    };
    let updated_item = serde_json::to_value(&row)?;

    // Update the item in the config
    helpers::update_in_config_array(
        config_json,
        "CFG_ERRULE",
        "ERRULE_CODE",
        &code,
        updated_item,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: updating a rule must preserve all existing keys, including
    /// null-valued ones like DISQ_ERFRAG_CODE. The engine config loader rejects
    /// a CFG_ERRULE row missing that key (SENZ9117), so set_rule must never drop
    /// it when the disqualifier is not part of the update.
    #[test]
    fn test_set_rule_preserves_null_disqualifier() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 100, "ERRULE_CODE": "SAME_A1", "RESOLVE": "Yes",
             "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "SAME_A1",
             "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": 10}
        ], "CFG_ERFRAG": []}}"#;

        let params = SetRuleParams {
            code: "SAME_A1",
            resolve: Some("No"),
            relate: None,
            rtype_id: None,
            fragment: None,
            disqualifier: None,
            tier: None,
        };

        let modified = set_rule(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rule = &value["G2_CONFIG"]["CFG_ERRULE"][0];
        let obj = rule.as_object().unwrap();

        // Every CFG_ERRULE key must always be present, even when null.
        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }

        // The updated field applied; unprovided fields preserved (incl. null).
        assert_eq!(rule["RESOLVE"], json!("No"));
        assert_eq!(rule["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(rule["RELATE"], json!("No"));
        assert_eq!(rule["QUAL_ERFRAG_CODE"], json!("SAME_A1"));
        assert_eq!(rule["ERRULE_TIER"], json!(10));
    }

    /// A brand-new-style update that provides no optional fields at all must
    /// still emit every key (as null where nothing exists), so the engine
    /// config loader never sees a missing CFG_ERRULE key.
    #[test]
    fn test_set_rule_emits_all_keys_when_optionals_absent() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 42, "ERRULE_CODE": "MINIMAL", "RESOLVE": "No"}
        ], "CFG_ERFRAG": []}}"#;

        let params = SetRuleParams {
            code: "MINIMAL",
            resolve: Some("Yes"),
            relate: None,
            rtype_id: None,
            fragment: None,
            disqualifier: None,
            tier: None,
        };

        let modified = set_rule(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let obj = value["G2_CONFIG"]["CFG_ERRULE"][0].as_object().unwrap();

        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        // Nothing existed for these optionals, so they surface as null.
        assert_eq!(obj["QUAL_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["ERRULE_TIER"], Value::Null);
    }

    /// add_rule must write a complete row even when the caller supplies only a
    /// subset of fields — the omitted optionals become null, never dropped.
    #[test]
    fn test_add_rule_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": []}}"#;
        let rule_config = json!({
            "ERRULE_CODE": "custom_rule",
            "RESOLVE": "Yes",
            "RELATE": "No",
            "RTYPE_ID": 1
        });

        let (modified, _id) = add_rule(config, 0, &rule_config).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let obj = value["G2_CONFIG"]["CFG_ERRULE"][0].as_object().unwrap();

        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["ERRULE_CODE"], json!("CUSTOM_RULE"));
        assert_eq!(obj["QUAL_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["ERRULE_TIER"], Value::Null);
    }
}
