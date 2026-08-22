//! Config Section operations
//!
//! Functions for managing top-level configuration sections in G2_CONFIG.
//! These functions allow adding, removing, and querying configuration sections.

use crate::error::{Result, SzConfigError};
use crate::filter::to_json_dumps_string;
use serde_json::{Value, json};

/// Outcome counts for [`add_config_section_field`].
///
/// `existed` counts items that already carried the field (and were therefore
/// left untouched); `updated` counts items the field was newly inserted into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AddFieldCounts {
    /// Number of items that already had the field (value preserved, not overwritten).
    pub existed: usize,
    /// Number of items the field was newly added to.
    pub updated: usize,
}

/// Add a new configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to add
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {}}"#;
/// let modified = config_sections::add_config_section(config, "CFG_CUSTOM").unwrap();
/// ```
pub fn add_config_section(config_json: &str, section_name: &str) -> Result<String> {
    let section_name = section_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;

    // Check if section already exists
    if let Some(g2_config) = config_data.get("G2_CONFIG")
        && g2_config.get(&section_name).is_some()
    {
        return Err(SzConfigError::AlreadyExists(
            "Configuration section already exists".to_string(),
        ));
    }

    // Add new section as empty array
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(obj) = g2_config.as_object_mut() {
            obj.insert(section_name.clone(), json!([]));
        }
    } else {
        return Err(SzConfigError::NotFound(
            "G2_CONFIG section not found in configuration".to_string(),
        ));
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Remove a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to remove
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_CUSTOM": []}}"#;
/// let modified = config_sections::remove_config_section(config, "CFG_CUSTOM").unwrap();
/// ```
pub fn remove_config_section(config_json: &str, section_name: &str) -> Result<String> {
    let section_name = section_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut removed = false;

    if let Some(g2_config) = config_data.get_mut("G2_CONFIG")
        && let Some(g2_config_obj) = g2_config.as_object_mut()
        && g2_config_obj.remove(&section_name).is_some()
    {
        removed = true;
    }

    if !removed {
        return Err(SzConfigError::NotFound(format!(
            "Config section not found: {section_name}"
        )));
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// Get items from a configuration section with optional filtering
///
/// # Filter substrate
///
/// When a `filter` is supplied each record is stringified with **`json.dumps`
/// spacing** (`{"K": 1, "J": null}`) — via [`crate::filter::to_json_dumps_string`]
/// — before the case-insensitive substring test. This matches Python's
/// `do_getConfigSection`, which filters on `json.dumps(record).lower()`. The
/// previous implementation used compact `serde_json::to_string`
/// (`{"K":1,"J":null}`), so a filter term spanning a key/value boundary (a `": "`
/// or `", "`) matched under Python but missed here. The deliberate `ensure_ascii`
/// limitation carried by [`crate::filter`] applies (non-ASCII is emitted as
/// UTF-8, not `\uXXXX`).
///
/// # Empty vs. no-match
///
/// This function returns an empty `Vec` both when the section is empty and when a
/// filter matched nothing; it does **not** distinguish the two (its return type
/// is unchanged for backwards compatibility). Use [`config_section_is_empty`] to
/// tell them apart — Python emits different messages ("Configuration section is
/// empty" vs. "Nothing to display").
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to get
/// * `filter` - Optional filter string to search in records
///
/// # Returns
///
/// Returns a vector of items from the section
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}]}}"#;
/// let items = config_sections::get_config_section(config, "CFG_ATTR", None).unwrap();
/// assert_eq!(items.len(), 1);
/// ```
pub fn get_config_section(
    config_json: &str,
    section_name: &str,
    filter: Option<&str>,
) -> Result<Vec<Value>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    // Check if section exists
    let section_data = config_data
        .get("G2_CONFIG")
        .and_then(|g2| g2.get(section_name))
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Configuration section '{section_name}' not found"))
        })?;

    // Handle empty section
    if section_data.is_null()
        || (section_data.is_array() && section_data.as_array().unwrap().is_empty())
    {
        return Ok(Vec::new());
    }

    // Case-insensitive substring test on the json.dumps-spaced rendering (Python
    // parity — see the "Filter substrate" doc section).
    let matches = |record: &Value, needle: &str| -> bool {
        to_json_dumps_string(record)
            .to_lowercase()
            .contains(&needle.to_lowercase())
    };

    // Apply filter if provided
    let output_data = if let Some(filter_str) = filter {
        if let Some(array) = section_data.as_array() {
            array
                .iter()
                .filter(|record| matches(record, filter_str))
                .cloned()
                .collect()
        } else {
            // Not an array, just check the single value
            if matches(section_data, filter_str) {
                vec![section_data.clone()]
            } else {
                Vec::new()
            }
        }
    } else {
        // No filter - return all
        if let Some(array) = section_data.as_array() {
            array.clone()
        } else {
            vec![section_data.clone()]
        }
    };

    Ok(output_data)
}

/// Report whether a configuration section is empty.
///
/// A companion to [`get_config_section`] that lets a caller distinguish an
/// **empty section** from a **filter that matched nothing** — both of which come
/// back from `get_config_section` as an empty `Vec`. Python emits different
/// messages for the two cases ("Configuration section is empty" vs. "Nothing to
/// display"); this accessor supplies the discriminator without a breaking change
/// to `get_config_section`'s return type.
///
/// A section counts as empty when it is JSON `null` or an empty array. A
/// non-array, non-null value (e.g. a scalar or object) counts as non-empty.
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Name of the section to check
///
/// # Returns
///
/// * `Ok(true)` if the section exists and is empty (null or `[]`)
/// * `Ok(false)` if the section exists and holds at least one item
/// * `Err(SzConfigError::NotFound)` if the section does not exist
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections::{config_section_is_empty, get_config_section};
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}], "CFG_EMPTY": []}}"#;
///
/// // Empty section -> is_empty is true.
/// assert!(config_section_is_empty(config, "CFG_EMPTY").unwrap());
///
/// // Populated section that a filter excludes: get returns [] but is_empty is false,
/// // so the caller knows it was "nothing matched", not "section empty".
/// let matched = get_config_section(config, "CFG_ATTR", Some("PHONE")).unwrap();
/// assert!(matched.is_empty());
/// assert!(!config_section_is_empty(config, "CFG_ATTR").unwrap());
/// ```
pub fn config_section_is_empty(config_json: &str, section_name: &str) -> Result<bool> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let section_data = config_data
        .get("G2_CONFIG")
        .and_then(|g2| g2.get(section_name))
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Configuration section '{section_name}' not found"))
        })?;

    Ok(section_data.is_null()
        || section_data
            .as_array()
            .map(|arr| arr.is_empty())
            .unwrap_or(false))
}

/// List all configuration section names
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// # Returns
///
/// Returns a vector of section names
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [], "CFG_DSRC": []}}"#;
/// let sections = config_sections::list_config_sections(config).unwrap();
/// assert!(sections.contains(&"CFG_ATTR".to_string()));
/// ```
pub fn list_config_sections(config_json: &str) -> Result<Vec<String>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    let sections = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(obj) = g2_config.as_object() {
            obj.keys().cloned().collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(sections)
}

/// Add a field to all items in a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Section name
/// * `field_name` - Field name to add
/// * `field_value` - Value for the field
///
/// # Returns
///
/// Returns `(modified_config, AddFieldCounts)` on success. The counts report how
/// many items already had the field (left untouched) versus how many the field
/// was newly added to.
///
/// # Behaviour
///
/// The field is inserted only into items that do **not** already have it, so an
/// existing value is never overwritten.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}]}}"#;
/// let (modified, counts) = config_sections::add_config_section_field(
///     config,
///     "CFG_ATTR",
///     "NEW_FIELD",
///     &json!("default")
/// ).unwrap();
/// assert_eq!(counts.updated, 1);
/// assert_eq!(counts.existed, 0);
/// ```
pub fn add_config_section_field(
    config_json: &str,
    section_name: &str,
    field_name: &str,
    field_value: &Value,
) -> Result<(String, AddFieldCounts)> {
    let section_name = section_name.to_uppercase();
    let field_name = field_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut counts = AddFieldCounts::default();

    // Navigate to section and add the field only to items that lack it, so an
    // existing value is never overwritten.
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(section_array) = g2_config
            .get_mut(&section_name)
            .and_then(|v| v.as_array_mut())
        {
            for item in section_array.iter_mut() {
                if let Some(item_obj) = item.as_object_mut() {
                    if item_obj.contains_key(&field_name) {
                        counts.existed += 1;
                    } else {
                        item_obj.insert(field_name.clone(), field_value.clone());
                        counts.updated += 1;
                    }
                }
            }
        } else {
            return Err(SzConfigError::NotFound(format!(
                "Section not found or not an array: {section_name}"
            )));
        }
    }

    Ok((serde_json::to_string(&config_data)?, counts))
}

/// Remove a field from all items in a configuration section
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `section_name` - Section name
/// * `field_name` - Field name to remove
///
/// # Returns
///
/// Returns `(modified_config, item_count)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::config_sections;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME", "OLD_FIELD": "value"}]}}"#;
/// let (modified, count) = config_sections::remove_config_section_field(
///     config,
///     "CFG_ATTR",
///     "OLD_FIELD"
/// ).unwrap();
/// assert_eq!(count, 1);
/// ```
pub fn remove_config_section_field(
    config_json: &str,
    section_name: &str,
    field_name: &str,
) -> Result<(String, usize)> {
    let section_name = section_name.to_uppercase();
    let field_name = field_name.to_uppercase();
    let mut config_data: Value = serde_json::from_str(config_json)?;
    let mut item_count = 0;

    // Navigate to section and remove field from all items in the array
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(section_array) = g2_config
            .get_mut(&section_name)
            .and_then(|v| v.as_array_mut())
        {
            for item in section_array.iter_mut() {
                if let Some(item_obj) = item.as_object_mut()
                    && item_obj.remove(&field_name).is_some()
                {
                    item_count += 1;
                }
            }
        } else {
            return Err(SzConfigError::NotFound(format!(
                "Section not found or not an array: {section_name}"
            )));
        }
    }

    Ok((serde_json::to_string(&config_data)?, item_count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SzErrorKind;

    #[test]
    fn test_add_config_section_field_added() {
        let config = r#"{"G2_CONFIG": {"CFG_ATTR": [
            {"ATTR_CODE": "NAME"},
            {"ATTR_CODE": "PHONE"}
        ]}}"#;

        let (modified, counts) =
            add_config_section_field(config, "CFG_ATTR", "NEW_FIELD", &json!("default")).unwrap();
        assert_eq!(counts.updated, 2);
        assert_eq!(counts.existed, 0);

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_ATTR"].as_array().unwrap();
        assert_eq!(arr[0]["NEW_FIELD"], json!("default"));
        assert_eq!(arr[1]["NEW_FIELD"], json!("default"));
    }

    #[test]
    fn test_add_config_section_field_already_present_preserves_value() {
        let config = r#"{"G2_CONFIG": {"CFG_ATTR": [
            {"ATTR_CODE": "NAME", "EXISTING": "keep-me"}
        ]}}"#;

        let (modified, counts) =
            add_config_section_field(config, "CFG_ATTR", "EXISTING", &json!("overwrite")).unwrap();
        assert_eq!(counts.updated, 0);
        assert_eq!(counts.existed, 1);

        // The pre-existing value must be preserved, never overwritten.
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(
            value["G2_CONFIG"]["CFG_ATTR"][0]["EXISTING"],
            json!("keep-me")
        );
    }

    #[test]
    fn test_add_config_section_field_mixed() {
        let config = r#"{"G2_CONFIG": {"CFG_ATTR": [
            {"ATTR_CODE": "NAME", "F": "old"},
            {"ATTR_CODE": "PHONE"},
            {"ATTR_CODE": "EMAIL"}
        ]}}"#;

        // Field name is upper-cased before matching, so "f" matches "F".
        let (modified, counts) =
            add_config_section_field(config, "CFG_ATTR", "f", &json!("new")).unwrap();
        assert_eq!(counts.existed, 1);
        assert_eq!(counts.updated, 2);

        let value: Value = serde_json::from_str(&modified).unwrap();
        let arr = value["G2_CONFIG"]["CFG_ATTR"].as_array().unwrap();
        assert_eq!(arr[0]["F"], json!("old")); // preserved
        assert_eq!(arr[1]["F"], json!("new"));
        assert_eq!(arr[2]["F"], json!("new"));
    }

    #[test]
    fn test_add_config_section_field_missing_section_errors() {
        let config = r#"{"G2_CONFIG": {"CFG_ATTR": []}}"#;
        let err = add_config_section_field(config, "CFG_NOPE", "F", &json!("x")).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
    }

    // #36 in-repo repro: a record like {"K":1,"J":null} with a boundary-spanning
    // filter term. Under the old compact substrate the term missed; under the new
    // json.dumps substrate it matches (Python parity).
    #[test]
    fn test_get_config_section_boundary_spanning_filter_matches_json_dumps() {
        let config = r#"{"G2_CONFIG": {"CFG_X": [{"K": 1, "J": null}]}}"#;

        // Boundary-spanning term with the json.dumps ", " spacing between items.
        let matched = get_config_section(config, "CFG_X", Some("1, \"j\"")).unwrap();
        assert_eq!(matched.len(), 1, "json.dumps substrate should match");

        // Key/value boundary (": ") likewise matches.
        let matched = get_config_section(config, "CFG_X", Some("\"k\": 1")).unwrap();
        assert_eq!(matched.len(), 1);

        // A term only present in the compact form (no spaces) must NOT match now.
        let missed = get_config_section(config, "CFG_X", Some("1,\"j\"")).unwrap();
        assert!(
            missed.is_empty(),
            "compact-only term must miss under json.dumps"
        );
    }

    #[test]
    fn test_config_section_is_empty_distinguishes_empty_vs_no_match() {
        let config = r#"{"G2_CONFIG": {"CFG_ATTR": [{"ATTR_CODE": "NAME"}], "CFG_EMPTY": []}}"#;

        // Empty section -> is_empty true.
        assert!(config_section_is_empty(config, "CFG_EMPTY").unwrap());

        // Populated section with a filter that excludes everything: get returns
        // [] but is_empty is false -> caller can tell "nothing matched" apart from
        // "section empty".
        let matched = get_config_section(config, "CFG_ATTR", Some("PHONE")).unwrap();
        assert!(matched.is_empty());
        assert!(!config_section_is_empty(config, "CFG_ATTR").unwrap());

        // Missing section -> NotFound, same as get_config_section.
        let err = config_section_is_empty(config, "CFG_NOPE").unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
    }

    #[test]
    fn test_config_section_is_empty_null_section() {
        let config = r#"{"G2_CONFIG": {"CFG_NULL": null}}"#;
        assert!(config_section_is_empty(config, "CFG_NULL").unwrap());
    }
}
