//! Configuration settings operations.
//!
//! Manages the `G2_CONFIG.SETTINGS` object — a flat map of named settings keyed
//! by an upper-cased setting name. This is distinct from the `CFG_*` array
//! sections and is created lazily the first time a setting is written.

use crate::error::{Result, SzConfigError};
use serde_json::{Value, json};

/// Set (create or overwrite) a named configuration setting.
///
/// The `SETTINGS` object under `G2_CONFIG` is created if absent. The `name` is
/// upper-cased before use, and an existing setting of the same name is
/// overwritten silently. The `value` is stored verbatim as its typed JSON
/// value — an integer stays an integer, a string stays a string — no
/// validation is applied.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `name` - Setting name (upper-cased before storing)
/// * `value` - Setting value, stored verbatim as its typed JSON value
///   (accepts anything convertible into `serde_json::Value`, e.g. an integer,
///   a `&str`, or a pre-built `Value`)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `JsonParse` if `config_json` is invalid
/// - `MissingSection` if the top-level `G2_CONFIG` object is absent
///
/// # Example
/// ```
/// use sz_configtool_lib::settings::set_setting;
///
/// let config = r#"{"G2_CONFIG": {}}"#;
/// // A typed integer is stored as a JSON number, not a quoted string.
/// let updated = set_setting(config, "metaphone_version", 3)?;
/// let parsed: serde_json::Value = serde_json::from_str(&updated)?;
/// assert_eq!(parsed["G2_CONFIG"]["SETTINGS"]["METAPHONE_VERSION"], serde_json::json!(3));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn set_setting(config_json: &str, name: &str, value: impl Into<Value>) -> Result<String> {
    let value = value.into();
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let g2_config = config
        .get_mut("G2_CONFIG")
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| SzConfigError::MissingSection("G2_CONFIG".to_string()))?;

    // Create SETTINGS as an object if absent (or if a non-object placeholder is
    // present, replace it with a fresh object).
    let settings_is_object = g2_config
        .get("SETTINGS")
        .map(|v| v.is_object())
        .unwrap_or(false);
    if !settings_is_object {
        g2_config.insert("SETTINGS".to_string(), json!({}));
    }

    let settings = g2_config
        .get_mut("SETTINGS")
        .and_then(|v| v.as_object_mut())
        .expect("SETTINGS object was just ensured");

    settings.insert(name.to_uppercase(), value);

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_setting_creates_section() {
        let config = r#"{"G2_CONFIG": {}}"#;
        let modified = set_setting(config, "foo", "bar").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(value["G2_CONFIG"]["SETTINGS"]["FOO"], json!("bar"));
    }

    #[test]
    fn test_set_setting_uppercases_name() {
        let config = r#"{"G2_CONFIG": {"SETTINGS": {}}}"#;
        let modified = set_setting(config, "MixedCase", "v").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(value["G2_CONFIG"]["SETTINGS"]["MIXEDCASE"], json!("v"));
        assert!(value["G2_CONFIG"]["SETTINGS"].get("MixedCase").is_none());
    }

    #[test]
    fn test_set_setting_overwrites_silently() {
        let config = r#"{"G2_CONFIG": {"SETTINGS": {"FOO": "old"}}}"#;
        let modified = set_setting(config, "foo", "new").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(value["G2_CONFIG"]["SETTINGS"]["FOO"], json!("new"));
        // Exactly one entry — overwrite, not append.
        assert_eq!(value["G2_CONFIG"]["SETTINGS"].as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_set_setting_stores_typed_integer() {
        let config = r#"{"G2_CONFIG": {}}"#;
        let modified = set_setting(config, "metaphone_version", 3).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        // Stored verbatim as a JSON number, not a quoted string.
        assert_eq!(
            value["G2_CONFIG"]["SETTINGS"]["METAPHONE_VERSION"],
            json!(3)
        );
        assert_ne!(
            value["G2_CONFIG"]["SETTINGS"]["METAPHONE_VERSION"],
            json!("3")
        );
    }

    #[test]
    fn test_set_setting_stores_str_verbatim() {
        // A &str value is still stored as a JSON string.
        let config = r#"{"G2_CONFIG": {}}"#;
        let modified = set_setting(config, "foo", "bar").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(value["G2_CONFIG"]["SETTINGS"]["FOO"], json!("bar"));
    }

    #[test]
    fn test_set_setting_missing_g2_config() {
        let config = r#"{}"#;
        let err = set_setting(config, "foo", "bar").unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::MissingSection);
    }
}
