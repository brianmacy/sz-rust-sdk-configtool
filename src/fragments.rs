//! Fragment (CFG_ERFRAG) operations
//!
//! Functions for managing entity resolution fragments in the configuration.
//! Fragments define matching criteria used by rules.

use crate::error::{Result, SzConfigError};
use crate::helpers::{self, FieldUpdate};
use serde::Serialize;
use serde_json::{Value, json};

/// Complete CFG_ERFRAG row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted (optional fields serialize as JSON `null`). The Senzing engine's
/// config loader requires every key to be present, so partial rows must never
/// be written.
#[derive(Debug, Clone, Serialize)]
struct ErfragRow {
    #[serde(rename = "ERFRAG_ID")]
    erfrag_id: i64,
    #[serde(rename = "ERFRAG_CODE")]
    erfrag_code: String,
    #[serde(rename = "ERFRAG_DESC")]
    erfrag_desc: Option<String>,
    #[serde(rename = "ERFRAG_SOURCE")]
    erfrag_source: Option<String>,
    #[serde(rename = "ERFRAG_DEPENDS")]
    erfrag_depends: Option<String>,
}

/// Parameters for setting (updating) a fragment.
///
/// Both fields are tri-state [`FieldUpdate`]s so an update can distinguish "leave
/// unchanged" from "clear to null":
///
/// - `source` (`ERFRAG_SOURCE`): `Leave` keeps the stored source and its computed
///   `ERFRAG_DEPENDS`; `Clear` writes `null` for both source and depends (D11);
///   `Set` validates the new source and recomputes `ERFRAG_DEPENDS`.
/// - `description` (`ERFRAG_DESC`): `Leave` keeps the stored description; `Clear`
///   writes `null`; `Set` writes the new description.
#[derive(Debug, Clone, Default)]
pub struct SetFragmentParams<'a> {
    pub source: FieldUpdate<&'a str>,
    pub description: FieldUpdate<&'a str>,
}

impl<'a> TryFrom<&'a Value> for SetFragmentParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        // Tri-state per field: an absent key -> Leave, an explicit JSON null ->
        // Clear, a string value -> Set. Both the uppercase CFG_ERFRAG column
        // names and lowercase aliases are accepted.
        Ok(Self {
            source: helpers::field_update_str(json, &["ERFRAG_SOURCE", "source"]),
            description: helpers::field_update_str(json, &["ERFRAG_DESC", "description"]),
        })
    }
}

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
        return (vec![], format!("Invalid JSON: {e}"));
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
                                    format!("Invalid fragment reference: {current_frag}"),
                                );
                            }
                            Err(_) => {
                                return (
                                    vec![],
                                    format!("Invalid fragment reference: {current_frag}"),
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
                                        format!("Invalid fragment reference: {current_frag}"),
                                    );
                                }
                                Err(_) => {
                                    return (
                                        vec![],
                                        format!("Invalid fragment reference: {current_frag}"),
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

    // Check if fragment already exists (Python line 4520-4522)
    let code_upper = code.to_uppercase();
    if helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &code_upper)?
        .is_some()
    {
        return Err(SzConfigError::AlreadyExists(
            "Fragment already exists".to_string(),
        ));
    }

    // Validate source and compute dependencies
    let (dependency_list, error_message) = validate_fragment_source(config_json, source);
    if !error_message.is_empty() {
        return Err(SzConfigError::InvalidInput(error_message));
    }

    let config_data: Value = serde_json::from_str(config_json)?;

    // Caller-supplied ERFRAG_ID (#37/D19): previously the add path computed
    // max+1 unconditionally and *ignored* any ERFRAG_ID present in the input
    // Value. Now an explicit ERFRAG_ID (> 0) is honoured — rejected with
    // AlreadyExists if taken — while None/absent/non-positive auto-assigns the
    // next id (unseeded max+1, floor 1, preserving the historical numbering).
    let desired_id = fragment_config.get("ERFRAG_ID").and_then(|v| v.as_i64());
    let empty: Vec<Value> = Vec::new();
    let erfrag_array = config_data
        .get("G2_CONFIG")
        .ok_or_else(|| SzConfigError::InvalidConfig("G2_CONFIG not found".to_string()))?
        .get("CFG_ERFRAG")
        .and_then(|v| v.as_array())
        .unwrap_or(&empty);
    let next_id = helpers::get_desired_or_next_id(erfrag_array, "ERFRAG_ID", desired_id, 1)?;

    // Build a complete row via ErfragRow so every CFG_ERFRAG key is present
    // (optional fields serialize as null).
    let row = ErfragRow {
        erfrag_id: next_id,
        erfrag_code: code_upper.clone(),
        erfrag_desc: Some(code_upper.clone()),
        erfrag_source: Some(source.to_string()),
        erfrag_depends: if dependency_list.is_empty() {
            None
        } else {
            Some(dependency_list.join(","))
        },
    };
    let new_item = serde_json::to_value(&row)?;

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
        .ok_or_else(|| SzConfigError::NotFound(format!("Fragment not found: {frag_code}")))?;

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
    let item = if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &search_value)?
    {
        item
    } else if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_ID", &search_value)?
    {
        item
    } else {
        return Err(SzConfigError::NotFound(format!(
            "Fragment not found: {search_value}"
        )));
    };

    // Transform to lowercase format (matching list_fragments for consistency).
    // ERFRAG_SOURCE and ERFRAG_DEPENDS are stored-nullable, so they are projected
    // null-preserving (stored null stays null, stored "" stays "", absent ->
    // null) via helpers::field_or_null rather than coerced to "".
    Ok(json!({
        "id": helpers::field_or_null(&item, "ERFRAG_ID"),
        "fragment": item.get("ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
        "source": helpers::field_or_null(&item, "ERFRAG_SOURCE"),
        "depends": helpers::field_or_null(&item, "ERFRAG_DEPENDS")
    }))
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
                    // ERFRAG_SOURCE and ERFRAG_DEPENDS null-preserved (see get_fragment).
                    json!({
                        "id": helpers::field_or_null(item, "ERFRAG_ID"),
                        "fragment": item.get("ERFRAG_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                        "source": helpers::field_or_null(item, "ERFRAG_SOURCE"),
                        "depends": helpers::field_or_null(item, "ERFRAG_DEPENDS")
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
/// * `params` - Tri-state update for `ERFRAG_SOURCE` / `ERFRAG_DESC`
///   ([`SetFragmentParams`])
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::fragments::{self, SetFragmentParams};
/// use sz_configtool_lib::helpers::FieldUpdate;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST"}]}}"#;
/// let params = SetFragmentParams {
///     source: FieldUpdate::Set("NAME+DOB"),
///     description: FieldUpdate::Leave,
/// };
/// let modified = fragments::set_fragment(config, "TEST", params).unwrap();
/// ```
pub fn set_fragment(
    config_json: &str,
    fragment_code: &str,
    params: SetFragmentParams,
) -> Result<String> {
    let code = fragment_code.to_uppercase();

    // Fetch the existing row so fields not part of the update (ERFRAG_ID, and any
    // Leave field) are carried forward rather than dropped by the full-row
    // replace.
    let existing = helpers::find_in_config_array(config_json, "CFG_ERFRAG", "ERFRAG_CODE", &code)?
        .ok_or_else(|| SzConfigError::NotFound(format!("Fragment not found: {code}")))?;

    let erfrag_id = existing
        .get("ERFRAG_ID")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // SOURCE tri-state:
    // - Leave: keep the existing SOURCE and DEPENDS.
    // - Clear: write null for SOURCE and, per D11, also clear DEPENDS to null.
    // - Set: validate the new SOURCE and recompute DEPENDS.
    let (erfrag_source, erfrag_depends) = match params.source {
        FieldUpdate::Leave => (
            helpers::field_as_string(&existing, "ERFRAG_SOURCE"),
            helpers::field_as_string(&existing, "ERFRAG_DEPENDS"),
        ),
        FieldUpdate::Clear => (None, None),
        FieldUpdate::Set(new_source) => {
            let (dependency_list, error_message) =
                validate_fragment_source(config_json, new_source);
            if !error_message.is_empty() {
                return Err(SzConfigError::InvalidInput(error_message));
            }
            let depends = if dependency_list.is_empty() {
                None
            } else {
                Some(dependency_list.join(","))
            };
            (Some(new_source.to_string()), depends)
        }
    };

    // ERFRAG_DESC tri-state: Leave preserves, Clear nulls, Set writes.
    let erfrag_desc = match params.description {
        FieldUpdate::Leave => helpers::field_as_string(&existing, "ERFRAG_DESC"),
        FieldUpdate::Clear => None,
        FieldUpdate::Set(desc) => Some(desc.to_string()),
    };

    let row = ErfragRow {
        erfrag_id,
        erfrag_code: code.clone(),
        erfrag_desc,
        erfrag_source,
        erfrag_depends,
    };
    let updated_item = serde_json::to_value(&row)?;

    // Update the item in the config
    helpers::update_in_config_array(
        config_json,
        "CFG_ERFRAG",
        "ERFRAG_CODE",
        &code,
        updated_item,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERFRAG_KEYS: [&str; 5] = [
        "ERFRAG_ID",
        "ERFRAG_CODE",
        "ERFRAG_DESC",
        "ERFRAG_SOURCE",
        "ERFRAG_DEPENDS",
    ];

    fn assert_all_keys(rule: &Value) {
        let obj = rule.as_object().unwrap();
        for key in ERFRAG_KEYS {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
    }

    #[test]
    fn test_add_fragment_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": []}}"#;
        let frag_config = json!({"ERFRAG_CODE": "custom_frag", "ERFRAG_SOURCE": "NAME+DOB"});

        let (modified, _id) = add_fragment(config, &frag_config).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = &value["G2_CONFIG"]["CFG_ERFRAG"][0];

        assert_all_keys(frag);
        assert_eq!(frag["ERFRAG_CODE"], json!("CUSTOM_FRAG"));
        assert_eq!(frag["ERFRAG_SOURCE"], json!("NAME+DOB"));
        // No FRAGMENT[...] references -> no dependencies -> null (present).
        assert_eq!(frag["ERFRAG_DEPENDS"], Value::Null);
    }

    /// #37/D19: a caller-supplied ERFRAG_ID is now honoured (previously ignored),
    /// and a taken id is rejected.
    #[test]
    fn test_add_fragment_caller_supplied_id() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 3, "ERFRAG_CODE": "EXISTING", "ERFRAG_SOURCE": "NAME"}
        ]}}"#;

        // Honour an explicit id rather than computing max+1 (=4).
        let frag_config = json!({
            "ERFRAG_CODE": "CUSTOM_FRAG",
            "ERFRAG_SOURCE": "NAME+DOB",
            "ERFRAG_ID": 50
        });
        let (modified, id) = add_fragment(config, &frag_config).unwrap();
        assert_eq!(id, 50);
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = value["G2_CONFIG"]["CFG_ERFRAG"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert_eq!(frag["ERFRAG_ID"], json!(50));

        // A taken id is rejected.
        let frag_config = json!({
            "ERFRAG_CODE": "ANOTHER",
            "ERFRAG_SOURCE": "NAME",
            "ERFRAG_ID": 3
        });
        let err = add_fragment(config, &frag_config).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);

        // Absent id still auto-assigns max+1.
        let frag_config = json!({"ERFRAG_CODE": "AUTO", "ERFRAG_SOURCE": "NAME"});
        let (_m, id) = add_fragment(config, &frag_config).unwrap();
        assert_eq!(id, 4);
    }

    /// #33: get_fragment / list_fragments null-preserve ERFRAG_SOURCE and
    /// ERFRAG_DEPENDS (stored null -> null, stored "" -> "", absent -> null)
    /// rather than coercing them to "".
    #[test]
    fn test_fragments_source_depends_null_preserved() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 1, "ERFRAG_CODE": "A", "ERFRAG_SOURCE": null, "ERFRAG_DEPENDS": ""},
            {"ERFRAG_ID": 2, "ERFRAG_CODE": "B", "ERFRAG_SOURCE": "NAME"}
        ]}}"#;

        let list = list_fragments(config).unwrap();
        // Row A: stored null stays null; stored "" stays "".
        assert_eq!(list[0]["source"], Value::Null);
        assert_eq!(list[0]["depends"], json!(""));
        // Row B: source present; depends absent -> null.
        assert_eq!(list[1]["source"], json!("NAME"));
        assert_eq!(list[1]["depends"], Value::Null);

        // get_fragment agrees.
        let a = get_fragment(config, "A").unwrap();
        assert_eq!(a["source"], Value::Null);
        assert_eq!(a["depends"], json!(""));
    }

    /// Regression: updating one field must not drop the row's other keys.
    /// Previously set_fragment replaced the whole row with the partial update,
    /// discarding ERFRAG_ID / ERFRAG_DESC and leaving the engine loader with an
    /// incomplete CFG_ERFRAG row.
    #[test]
    fn test_set_fragment_preserves_unupdated_fields() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 7, "ERFRAG_CODE": "TEST", "ERFRAG_DESC": "TEST",
             "ERFRAG_SOURCE": "NAME+DOB", "ERFRAG_DEPENDS": null}
        ]}}"#;

        // Update only the description; SOURCE is not part of the update.
        let params = SetFragmentParams {
            source: FieldUpdate::Leave,
            description: FieldUpdate::Set("Updated desc"),
        };
        let modified = set_fragment(config, "TEST", params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = &value["G2_CONFIG"]["CFG_ERFRAG"][0];

        assert_all_keys(frag);
        assert_eq!(frag["ERFRAG_ID"], json!(7)); // preserved, not dropped
        assert_eq!(frag["ERFRAG_DESC"], json!("Updated desc"));
        assert_eq!(frag["ERFRAG_SOURCE"], json!("NAME+DOB")); // preserved
        assert_eq!(frag["ERFRAG_DEPENDS"], Value::Null); // preserved (present)
    }

    #[test]
    fn test_set_fragment_all_keys_present_on_source_update() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 1, "ERFRAG_CODE": "TEST"}
        ]}}"#;

        let params = SetFragmentParams {
            source: FieldUpdate::Set("NAME+DOB"),
            description: FieldUpdate::Leave,
        };
        let modified = set_fragment(config, "TEST", params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = &value["G2_CONFIG"]["CFG_ERFRAG"][0];

        assert_all_keys(frag);
        assert_eq!(frag["ERFRAG_ID"], json!(1));
        assert_eq!(frag["ERFRAG_SOURCE"], json!("NAME+DOB"));
    }

    /// D11: clearing ERFRAG_SOURCE also clears ERFRAG_DEPENDS to null; both keys
    /// remain present.
    #[test]
    fn test_set_fragment_source_clear_clears_depends() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 3, "ERFRAG_CODE": "TEST", "ERFRAG_DESC": "TEST",
             "ERFRAG_SOURCE": "./FRAGMENT[./SAME_NAME>0]", "ERFRAG_DEPENDS": "1,2"}
        ]}}"#;

        let params = SetFragmentParams {
            source: FieldUpdate::Clear,
            description: FieldUpdate::Leave,
        };
        let modified = set_fragment(config, "TEST", params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = &value["G2_CONFIG"]["CFG_ERFRAG"][0];

        assert_all_keys(frag);
        assert_eq!(frag["ERFRAG_SOURCE"], Value::Null);
        assert_eq!(frag["ERFRAG_DEPENDS"], Value::Null);
        // Untouched fields preserved.
        assert_eq!(frag["ERFRAG_DESC"], json!("TEST"));
    }

    /// The description can be cleared independently of the source.
    #[test]
    fn test_set_fragment_description_clear() {
        let config = r#"{"G2_CONFIG": {"CFG_ERFRAG": [
            {"ERFRAG_ID": 4, "ERFRAG_CODE": "TEST", "ERFRAG_DESC": "TEST",
             "ERFRAG_SOURCE": "NAME", "ERFRAG_DEPENDS": null}
        ]}}"#;

        let params = SetFragmentParams {
            source: FieldUpdate::Leave,
            description: FieldUpdate::Clear,
        };
        let modified = set_fragment(config, "TEST", params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let frag = &value["G2_CONFIG"]["CFG_ERFRAG"][0];
        assert_eq!(frag["ERFRAG_DESC"], Value::Null);
        // Source untouched (Leave).
        assert_eq!(frag["ERFRAG_SOURCE"], json!("NAME"));
    }
}
