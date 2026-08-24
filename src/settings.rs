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
/// overwritten silently. No validation is applied to `value` — it is stored
/// verbatim as a JSON string.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `name` - Setting name (upper-cased before storing)
/// * `value` - Setting value (stored as a JSON string, unvalidated)
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
/// let updated = set_setting(config, "my_setting", "42")?;
/// assert!(updated.contains("MY_SETTING"));
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn set_setting(config_json: &str, name: &str, value: &str) -> Result<String> {
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

    settings.insert(name.to_uppercase(), json!(value));

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
    fn test_set_setting_missing_g2_config() {
        let config = r#"{}"#;
        let err = set_setting(config, "foo", "bar").unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::MissingSection);
    }
}
