//! Config-document structural validation.
//!
//! This module provides [`validate_config`], a single structural gate that
//! callers (and, in future, the crate's own mutators) can rely on to decide
//! whether a JSON document is a plausible Senzing config document *before*
//! attempting section-by-section operations that would otherwise fail with
//! ad-hoc per-section errors.
//!
//! The check is deliberately **structure-only** (see decision D31): it does not
//! perform any cross-reference validation (foreign keys between sections, id
//! uniqueness, behaviour-code validity, and so on). It answers one question —
//! "is the top-level shape a config document?" — and nothing more.

use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// The recognised `CFG_*` config sections.
///
/// Each of these, **when present** in `G2_CONFIG`, must be a JSON array. Keys
/// not in this list (for example `SETTINGS`, `SYS_OOM`, `CONFIG_BASE_VERSION`)
/// are not `CFG_*` array sections and are left unchecked by [`validate_config`].
///
/// # Example
///
/// ```
/// use sz_configtool_lib::validation::EXPECTED_SECTIONS;
///
/// assert!(EXPECTED_SECTIONS.contains(&"CFG_FTYPE"));
/// assert!(EXPECTED_SECTIONS.contains(&"CFG_DSRC"));
/// ```
pub const EXPECTED_SECTIONS: &[&str] = &[
    "CFG_ATTR",
    "CFG_CFBOM",
    "CFG_CFCALL",
    "CFG_CFRTN",
    "CFG_CFUNC",
    "CFG_DFBOM",
    "CFG_DFCALL",
    "CFG_DFUNC",
    "CFG_DSRC",
    "CFG_DSRC_INTEREST",
    "CFG_EFBOM",
    "CFG_EFCALL",
    "CFG_EFUNC",
    "CFG_ERFRAG",
    "CFG_ERRULE",
    "CFG_FBOM",
    "CFG_FBOVR",
    "CFG_FCLASS",
    "CFG_FELEM",
    "CFG_FTYPE",
    "CFG_GENERIC_THRESHOLD",
    "CFG_GPLAN",
    "CFG_RCLASS",
    "CFG_RTYPE",
    "CFG_SFCALL",
    "CFG_SFUNC",
    "CFG_SPROFILE",
];

/// Validate the top-level structure of a config document.
///
/// This is a **structure-only** gate (decision D31). It confirms, in order:
///
/// 1. `config_json` parses as JSON,
/// 2. a top-level `G2_CONFIG` key is present and is a JSON object,
/// 3. every recognised `CFG_*` section listed in [`EXPECTED_SECTIONS`], **if
///    present**, is a JSON array.
///
/// It does **not** perform any deep or cross-reference validation: it never
/// checks that required sections exist, that ids are unique, or that foreign
/// keys between sections resolve. A document that passes `validate_config` is
/// only guaranteed to have the right *shape* for the crate's navigators, not to
/// be semantically complete.
///
/// # Arguments
/// * `config_json` - Configuration JSON string
///
/// # Returns
/// * `Ok(())` if the document is structurally a config document
/// * `Err(SzConfigError::JsonParse)` if the input is not valid JSON
/// * `Err(SzConfigError::MissingSection)` if `G2_CONFIG` is absent
/// * `Err(SzConfigError::InvalidStructure)` if `G2_CONFIG` is not an object, or
///   a present `CFG_*` section is not an array
///
/// # Example
/// ```
/// use sz_configtool_lib::validation::validate_config;
///
/// // Well-formed (empty but correctly shaped) document.
/// assert!(validate_config(r#"{"G2_CONFIG":{"CFG_DSRC":[]}}"#).is_ok());
///
/// // G2_CONFIG is not an object.
/// assert!(validate_config(r#"{"G2_CONFIG":42}"#).is_err());
///
/// // A CFG_* section is present but not an array.
/// assert!(validate_config(r#"{"G2_CONFIG":{"CFG_DSRC":{}}}"#).is_err());
/// ```
pub fn validate_config(config_json: &str) -> Result<()> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let g2 = config
        .get("G2_CONFIG")
        .ok_or_else(|| SzConfigError::MissingSection("G2_CONFIG".to_string()))?;

    let g2_obj = g2.as_object().ok_or_else(|| {
        SzConfigError::InvalidStructure("G2_CONFIG must be a JSON object".to_string())
    })?;

    for section in EXPECTED_SECTIONS {
        match g2_obj.get(*section) {
            Some(value) if !value.is_array() => {
                return Err(SzConfigError::InvalidStructure(format!(
                    "Config section {section} must be a JSON array"
                )));
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepts_well_formed_config() {
        let config = r#"{"G2_CONFIG":{"CFG_DSRC":[],"CFG_FTYPE":[{"FTYPE_ID":1}]}}"#;
        assert!(validate_config(config).is_ok());
    }

    #[test]
    fn test_accepts_config_with_non_cfg_scalars() {
        // Non-CFG_* keys (SETTINGS, SYS_OOM, CONFIG_BASE_VERSION) are unchecked.
        let config = r#"{"G2_CONFIG":{"CFG_DSRC":[],"CONFIG_BASE_VERSION":{"x":1},"SETTINGS":{}}}"#;
        assert!(validate_config(config).is_ok());
    }

    #[test]
    fn test_rejects_g2_config_not_object() {
        let err = validate_config(r#"{"G2_CONFIG":42}"#).unwrap_err();
        assert!(matches!(err, SzConfigError::InvalidStructure(_)));
    }

    #[test]
    fn test_rejects_missing_g2_config() {
        let err = validate_config(r#"{"SOMETHING_ELSE":{}}"#).unwrap_err();
        assert!(matches!(err, SzConfigError::MissingSection(_)));
    }

    #[test]
    fn test_rejects_non_array_cfg_section() {
        let err = validate_config(r#"{"G2_CONFIG":{"CFG_DSRC":{}}}"#).unwrap_err();
        assert!(matches!(err, SzConfigError::InvalidStructure(_)));
        assert!(err.to_string().contains("CFG_DSRC"));
    }

    #[test]
    fn test_rejects_invalid_json() {
        let err = validate_config("{not json").unwrap_err();
        assert!(matches!(err, SzConfigError::JsonParse(_)));
    }

    #[test]
    fn test_accepts_stock_template() {
        let config = include_str!("../tests/fixtures/g2config_template.json");
        assert!(validate_config(config).is_ok());
    }
}
