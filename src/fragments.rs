//! Fragment (CFG_ERFRAG) operations
//!
//! Functions for managing entity resolution fragments in the configuration.
//! Fragments define matching criteria used by rules.

use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

/// Validates a fragment source XPath expression and computes dependencies
///
/// Parses the source string to find all ./FRAGMENT[...] references,
/// validates that referenced fragments exist, and returns their IDs.
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `source_string` - Fragment source XPath expression
///
/// # Returns
///
/// Returns `(dependency_ids, error_message)` tuple
/// - dependency_ids: Vec of fragment IDs as strings (empty if no dependencies)
/// - error_message: Empty string on success, error description on failure
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
/// let source = "./FRAGMENT[./SAME_NAME>0 and ./SAME_STAB>0]";
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": []}}"#;
/// // Note: validate_fragment_source is private, used internally by add_fragment
/// ```
fn validate_fragment_source(config_json: &str, source_string: &str) -> (Vec<String>, String) {
    // Validate JSON parses correctly
    if let Err(e) = serde_json::from_str::<Value>(config_json) {
        return (vec![], format!("Invalid JSON: {}", e));
    }

    let mut dependency_list = Vec::new();
    let mut source = source_string.to_string();

    // Find all FRAGMENT[...] patterns
    while let Some(start_pos) = source.find("FRAGMENT[") {
        // Find the matching closing bracket
        let fragment_start = start_pos;
        if let Some(bracket_pos) = source[fragment_start..].find(']') {
            let fragment_string = &source[fragment_start..fragment_start + bracket_pos + 1];

            // Parse fragment references within FRAGMENT[...]
            let mut current_frag = String::new();
            let mut in_fragment = false;

            for ch in fragment_string.chars() {
                if ch == '/' {
                    // Start or continue parsing fragment name
                    if in_fragment && !current_frag.is_empty() {
                        // End of previous fragment, lookup and validate
                        match helpers::find_in_config_array(
                            config_json,
                            "CFG_ERFRAG",
                            "ERFRAG_CODE",
                            &current_frag,
                        ) {
                            Ok(Some(frag_record)) => {
                                if let Some(frag_id) =
                                    frag_record.get("ERFRAG_ID").and_then(|v| v.as_i64())
                                {
                                    dependency_list.push(frag_id.to_string());
                                }
                            }
                            Ok(None) => {
                                return (
                                    vec![],
                                    format!("Invalid fragment reference: {}", current_frag),
                                );
                            }
                            Err(_) => {
                                return (
                                    vec![],
                                    format!("Invalid fragment reference: {}", current_frag),
                                );
                            }
                        }
                    }
                    current_frag.clear();
                    in_fragment = true;
                } else if in_fragment {
                    // Check for delimiters that end fragment name
                    if "|=><)] ".contains(ch) {
                        if !current_frag.is_empty() {
                            // Lookup fragment
                            match helpers::find_in_config_array(
                                config_json,
                                "CFG_ERFRAG",
                                "ERFRAG_CODE",
                                &current_frag,
                            ) {
                                Ok(Some(frag_record)) => {
                                    if let Some(frag_id) =
                                        frag_record.get("ERFRAG_ID").and_then(|v| v.as_i64())
                                    {
                                        dependency_list.push(frag_id.to_string());
                                    }
                                }
                                Ok(None) => {
                                    return (
                                        vec![],
                                        format!("Invalid fragment reference: {}", current_frag),
                                    );
                                }
                                Err(_) => {
                                    return (
                                        vec![],
                                        format!("Invalid fragment reference: {}", current_frag),
                                    );
                                }
                            }
                            current_frag.clear();
                        }
                        in_fragment = false;
                    } else {
                        current_frag.push(ch);
                    }
                }
            }

            // Remove this FRAGMENT[...] from source to find next one
            source = source.replace(fragment_string, "");
        } else {
            break;
        }
    }

    // Remove duplicates and return
    dependency_list.sort();
    dependency_list.dedup();
    (dependency_list, String::new())
}

/// Add a new fragment to the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `fragment_config` - JSON configuration for the fragment (must include ERFRAG_CODE)
///
/// # Returns
///
/// Returns `(modified_config, new_fragment_id)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": []}}"#;
/// let frag_config = json!({
///     "ERFRAG_CODE": "CUSTOM_FRAG",
///     "ERFRAG_SOURCE": "NAME+ADDRESS"
/// });
/// let (modified, frag_id) = fragments::add_fragment(config, &frag_config).unwrap();
/// assert_eq!(frag_id, 1);
/// ```
pub fn add_fragment(config_json: &str, fragment_config: &Value) -> Result<(String, i64)> {
    let code = fragment_config
        .get("ERFRAG_CODE")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::MissingField("ERFRAG_CODE".to_string()))?;

    let source = fragment_config
        .get("ERFRAG_SOURCE")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::MissingField("ERFRAG_SOURCE".to_string()))?;

    // Validate source and compute dependencies
    let (dependency_list, error_message) = validate_fragment_source(config_json, source);
    if !error_message.is_empty() {
        return Err(SzConfigError::InvalidInput(error_message));
    }

    let config_data: Value = serde_json::from_str(config_json)?;

    // Get next ID
    let next_id = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(array) = g2_config.get("CFG_ERFRAG").and_then(|v| v.as_array()) {
            array
                .iter()
                .filter_map(|item| item.get("ERFRAG_ID").and_then(|v| v.as_i64()))
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

    // Create new item with provided config plus ID and computed dependencies
    let mut new_item = fragment_config.clone();
    if let Some(obj) = new_item.as_object_mut() {
        obj.insert("ERFRAG_ID".to_string(), json!(next_id));
        obj.insert("ERFRAG_CODE".to_string(), json!(code.to_uppercase()));
        obj.insert("ERFRAG_DESC".to_string(), json!(code.to_uppercase()));

        // Set ERFRAG_DEPENDS to comma-separated list or null
        if dependency_list.is_empty() {
            obj.insert("ERFRAG_DEPENDS".to_string(), Value::Null);
        } else {
            obj.insert(
                "ERFRAG_DEPENDS".to_string(),
                json!(dependency_list.join(",")),
            );
        }
    }

    // Add to config
    let modified_json = helpers::add_to_config_array(config_json, "CFG_ERFRAG", new_item)?;

    Ok((modified_json, next_id))
}

/// Delete a fragment from the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `fragment_code` - Fragment code to delete
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST"}]}}"#;
/// let modified = fragments::delete_fragment(config, "TEST").unwrap();
/// ```
pub fn delete_fragment(config_json: &str, fragment_code: &str) -> Result<String> {
    let frag_code = fragment_code.to_uppercase();

    // Verify fragment exists before deletion
    let _ = helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &frag_code)?
        .ok_or_else(|| SzConfigError::NotFound(format!("Fragment not found: {}", frag_code)))?;

    // Remove from config
    helpers::remove_from_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &frag_code)
}

/// Get a fragment by code or ID
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `code_or_id` - Fragment code or ID to search for
///
/// # Returns
///
/// Returns the fragment JSON object on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST"}]}}"#;
/// let fragment = fragments::get_fragment(config, "TEST").unwrap();
/// ```
pub fn get_fragment(config_json: &str, code_or_id: &str) -> Result<Value> {
    let search_value = code_or_id.to_uppercase();

    // Try to find by CODE first, then by ID
    if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &search_value)?
    {
        Ok(item)
    } else if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_ID", &search_value)?
    {
        Ok(item)
    } else {
        Err(SzConfigError::NotFound(format!(
            "Fragment not found: {}",
            search_value
        )))
    }
}

/// List all fragments in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns a vector of fragment objects in Python sz_configtool format
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST", "ERFRAG_SOURCE": "NAME", "ERFRAG_DEPENDS": ""}]}}"#;
/// let fragments = fragments::list_fragments(config).unwrap();
/// assert_eq!(fragments.len(), 1);
/// ```
pub fn list_fragments(config_json: &str) -> Result<Vec<Value>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    // Extract fragments and transform to Python format
    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(array) = g2_config.get("CFG_ERFRAG").and_then(|v| v.as_array()) {
            array
                .iter()
                .map(|item| {
                    json!({
                        "id": item.get("ERFRAG_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                        "fragment": item.get("ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                        "source": item.get("ERFRAG_SOURCE").and_then(|v| v.as_str()).unwrap_or(""),
                        "depends": item.get("ERFRAG_DEPENDS").and_then(|v| v.as_str()).unwrap_or("")
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

/// Update an existing fragment in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `fragment_code` - Fragment code to update
/// * `fragment_config` - New configuration for the fragment
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST"}]}}"#;
/// let new_config = json!({"ERFRAG_SOURCE": "NAME+DOB"});
/// let modified = fragments::set_fragment(config, "TEST", &new_config).unwrap();
/// ```
pub fn set_fragment(
    config_json: &str,
    fragment_code: &str,
    fragment_config: &Value,
) -> Result<String> {
    let code = fragment_code.to_uppercase();

    // Validate and compute dependencies if SOURCE is being updated
    let mut updated_item = fragment_config.clone();
    if let Some(new_source) = fragment_config
        .get("ERFRAG_SOURCE")
        .and_then(|v| v.as_str())
    {
        let (dependency_list, error_message) = validate_fragment_source(config_json, new_source);
        if !error_message.is_empty() {
            return Err(SzConfigError::InvalidInput(error_message));
        }

        // Update ERFRAG_DEPENDS in the update
        if let Some(obj) = updated_item.as_object_mut() {
            if dependency_list.is_empty() {
                obj.insert("ERFRAG_DEPENDS".to_string(), Value::Null);
            } else {
                obj.insert(
                    "ERFRAG_DEPENDS".to_string(),
                    json!(dependency_list.join(",")),
                );
            }
        }
    }

    // Ensure the code field matches
    if let Some(obj) = updated_item.as_object_mut() {
        obj.insert("ERFRAG_CODE".to_string(), json!(code.clone()));
    }

    // Update the item in the config
    helpers::update_in_config_array(
        config_json,
        "CFG_ERFRAG",
        "ERFRAG_CODE",
        &code,
        updated_item,
    )
}
