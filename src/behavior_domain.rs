//! Canonical behaviour-code domain.
//!
//! A feature's *behaviour* is encoded as a short code combining a frequency
//! (`A1`, `F1`, `FF`, `FM`, `FVM`, plus the special `NAME` and `NONE`) with
//! optional `E` (exclusivity) and `S` (stability) suffixes. This module is the
//! single home for that domain:
//!
//! - [`BEHAVIOR_CODES`] — the canonical **ordered** list of the 17 recognised
//!   behaviour codes, used as the sort key for behaviour-ordered listings.
//! - [`parse_behavior_code`] — split a code into `(frequency, exclusivity, stability)`.
//! - [`compute_behavior`] — reconstruct a code from a `CFG_FTYPE` row.
//! - [`behavior_position`] — index of a code within [`BEHAVIOR_CODES`].
//!
//! Historically `parse_behavior_code`/`compute_behavior` existed as two
//! byte-identical private copies (in `features.rs` and `behavior_overrides.rs`).
//! They now live here once and both modules delegate to these functions.

use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// Canonical, ordered list of the 17 recognised behaviour codes.
///
/// The order is the authoritative Senzing display/sort order: the special
/// `NAME` first, then each frequency (`A1`, `F1`, `FF`, `FM`, `FVM`) with its
/// plain / `E` / `ES` variants, and finally `NONE`. This ordering is the sort
/// key for behaviour-ordered listings (e.g. generic thresholds).
///
/// Note that this is the canonical *display* domain: [`parse_behavior_code`]
/// accepts a slightly broader input set (for example a bare `S` suffix), but
/// only the codes listed here participate in ordering.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::behavior_domain::BEHAVIOR_CODES;
///
/// assert_eq!(BEHAVIOR_CODES.len(), 17);
/// assert_eq!(BEHAVIOR_CODES[0], "NAME");
/// assert_eq!(BEHAVIOR_CODES[BEHAVIOR_CODES.len() - 1], "NONE");
/// ```
pub const BEHAVIOR_CODES: &[&str] = &[
    "NAME", "A1", "A1E", "A1ES", "F1", "F1E", "F1ES", "FF", "FFE", "FFES", "FM", "FME", "FMES",
    "FVM", "FVME", "FVMES", "NONE",
];

/// Parse a behaviour code string into `(frequency, exclusivity, stability)`.
///
/// The frequency must be one of `A1`, `F1`, `FF`, `FM`, `FVM`, `NONE`, `NAME`
/// (case-insensitive). An `E` suffix sets exclusivity to `"Yes"` and an `S`
/// suffix sets stability to `"Yes"`; the special codes `NAME` and `NONE` do not
/// take suffixes. Exclusivity and stability default to `"No"`.
///
/// # Arguments
/// * `behavior` - Behaviour code (e.g. `"FM"`, `"F1E"`, `"F1ES"`, `"NAME"`)
///
/// # Returns
/// * `Ok((frequency, exclusivity, stability))` on success
/// * `Err(SzConfigError::InvalidInput)` if the frequency code is not recognised
///
/// # Example
///
/// ```
/// use sz_configtool_lib::behavior_domain::parse_behavior_code;
///
/// assert_eq!(parse_behavior_code("F1ES").unwrap(), ("F1", "Yes", "Yes"));
/// assert_eq!(parse_behavior_code("fm").unwrap(), ("FM", "No", "No"));
/// assert_eq!(parse_behavior_code("NAME").unwrap(), ("NAME", "No", "No"));
/// assert!(parse_behavior_code("BOGUS").is_err());
/// ```
pub fn parse_behavior_code(behavior: &str) -> Result<(&'static str, &'static str, &'static str)> {
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
                "Invalid behavior code '{behavior}'. Valid codes: A1, F1, FF, FM, FVM, NONE, NAME (with optional E/S suffixes)"
            )));
        }
    };

    Ok((frequency, exclusivity, stability))
}

/// Reconstruct a behaviour code from a `CFG_FTYPE` row.
///
/// Reads `FTYPE_FREQ`, `FTYPE_EXCL` and `FTYPE_STAB` from the row and appends
/// `E` and/or `S` when exclusivity / stability are truthy. A value is truthy
/// when it is `"Y"`, `"1"` or `"YES"` (case-insensitive).
///
/// # Arguments
/// * `ftype` - A `CFG_FTYPE` row as a JSON object
///
/// # Returns
/// The behaviour code string (e.g. `"F1ES"`).
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::behavior_domain::compute_behavior;
///
/// let ftype = json!({"FTYPE_FREQ": "F1", "FTYPE_EXCL": "Yes", "FTYPE_STAB": "No"});
/// assert_eq!(compute_behavior(&ftype), "F1E");
/// ```
pub fn compute_behavior(ftype: &Value) -> String {
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

/// Return the position of a behaviour code within [`BEHAVIOR_CODES`].
///
/// The comparison is case-insensitive. Returns `None` if `behavior` is not one
/// of the canonical ordered codes (even if it is otherwise parseable, such as a
/// bare `S`-suffixed code). Use this as a sort key for behaviour-ordered lists.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::behavior_domain::behavior_position;
///
/// assert_eq!(behavior_position("NAME"), Some(0));
/// assert!(behavior_position("F1E").unwrap() < behavior_position("FF").unwrap());
/// assert_eq!(behavior_position("nonsense"), None);
/// ```
pub fn behavior_position(behavior: &str) -> Option<usize> {
    BEHAVIOR_CODES
        .iter()
        .position(|c| c.eq_ignore_ascii_case(behavior))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_behavior_codes_ordered_and_sized() {
        assert_eq!(BEHAVIOR_CODES.len(), 17);
        assert_eq!(BEHAVIOR_CODES[0], "NAME");
        assert_eq!(*BEHAVIOR_CODES.last().unwrap(), "NONE");
        // Strictly increasing positions (no duplicates, monotonic).
        for (i, code) in BEHAVIOR_CODES.iter().enumerate() {
            assert_eq!(behavior_position(code), Some(i));
        }
    }

    #[test]
    fn test_parse_behavior_code_frequencies() {
        assert_eq!(parse_behavior_code("A1").unwrap(), ("A1", "No", "No"));
        assert_eq!(parse_behavior_code("F1").unwrap(), ("F1", "No", "No"));
        assert_eq!(parse_behavior_code("FF").unwrap(), ("FF", "No", "No"));
        assert_eq!(parse_behavior_code("FM").unwrap(), ("FM", "No", "No"));
        assert_eq!(parse_behavior_code("FVM").unwrap(), ("FVM", "No", "No"));
    }

    #[test]
    fn test_parse_behavior_code_suffixes() {
        assert_eq!(parse_behavior_code("F1E").unwrap(), ("F1", "Yes", "No"));
        assert_eq!(parse_behavior_code("F1S").unwrap(), ("F1", "No", "Yes"));
        assert_eq!(parse_behavior_code("F1ES").unwrap(), ("F1", "Yes", "Yes"));
        // Order of suffix letters does not matter.
        assert_eq!(parse_behavior_code("F1SE").unwrap(), ("F1", "Yes", "Yes"));
    }

    #[test]
    fn test_parse_behavior_code_lowercase() {
        assert_eq!(parse_behavior_code("f1es").unwrap(), ("F1", "Yes", "Yes"));
        assert_eq!(parse_behavior_code("fvm").unwrap(), ("FVM", "No", "No"));
    }

    #[test]
    fn test_parse_behavior_code_name_and_none() {
        assert_eq!(parse_behavior_code("NAME").unwrap(), ("NAME", "No", "No"));
        assert_eq!(parse_behavior_code("name").unwrap(), ("NAME", "No", "No"));
        assert_eq!(parse_behavior_code("NONE").unwrap(), ("NONE", "No", "No"));
        // NAME/NONE do not undergo E/S stripping.
        assert_eq!(parse_behavior_code("NONE").unwrap().1, "No");
    }

    #[test]
    fn test_parse_behavior_code_invalid() {
        let err = parse_behavior_code("BOGUS").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid behavior code 'BOGUS'"), "got: {msg}");
    }

    #[test]
    fn test_compute_behavior_variants() {
        assert_eq!(
            compute_behavior(&json!({"FTYPE_FREQ": "F1", "FTYPE_EXCL": "No", "FTYPE_STAB": "No"})),
            "F1"
        );
        assert_eq!(
            compute_behavior(&json!({"FTYPE_FREQ": "F1", "FTYPE_EXCL": "Yes", "FTYPE_STAB": "No"})),
            "F1E"
        );
        assert_eq!(
            compute_behavior(
                &json!({"FTYPE_FREQ": "FVM", "FTYPE_EXCL": "Yes", "FTYPE_STAB": "Yes"})
            ),
            "FVMES"
        );
        // Truthy variants: Y / 1 / YES (case-insensitive).
        assert_eq!(
            compute_behavior(&json!({"FTYPE_FREQ": "A1", "FTYPE_EXCL": "y", "FTYPE_STAB": "1"})),
            "A1ES"
        );
    }

    /// The two former private copies of these functions (features.rs and
    /// behavior_overrides.rs) were byte-identical. This asserts the single
    /// shared copy round-trips every canonical code, standing in for the
    /// "collapsed copies still agree" guarantee.
    #[test]
    fn test_compute_behavior_roundtrips_full_domain() {
        for code in BEHAVIOR_CODES {
            let (freq, excl, stab) = parse_behavior_code(code).unwrap();
            let ftype = json!({
                "FTYPE_FREQ": freq,
                "FTYPE_EXCL": excl,
                "FTYPE_STAB": stab,
            });
            // NAME/NONE compute back to themselves (no suffixes emitted).
            assert_eq!(
                &compute_behavior(&ftype),
                code,
                "round-trip failed for {code}"
            );
        }
    }
}
