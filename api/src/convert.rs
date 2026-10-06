//! Hand-written argument converters named by a manifest arg's `rust_convert`.
//!
//! Use one when the root-library parameter type is not a plain wire type
//! (e.g. an enum such as `calls::CallSelector`, a `usize`, or a slice). The
//! generated handler emits `crate::convert::<name>(args, "<arg name>")?`, so
//! each converter has the signature
//!
//! ```text
//! pub fn <name><'a>(args: &Args<'a>, name: &str) -> Result<T, ApiError>
//! ```
//!
//! and must apply the same absent / null / value convention as [`Args`](crate::Args). Add a
//! unit test for every converter, and document its wire shape in the arg's
//! `semantics` in the manifest so every language encodes it identically.
//!
//! No group in the A1 proof slice needs one yet.

// ---------------------------------------------------------------------------
// calls/standardize + calls/expression converters (A2 group calls_*)
// ---------------------------------------------------------------------------

fn convert_invalid(msg: String) -> crate::ApiError {
    crate::ApiError::Config(sz_configtool_lib::SzConfigError::InvalidInput(msg))
}

/// Required `CallSelector`. Wire shape: an integer selects by call id
/// (`CallSelector::Id`), a string selects by feature code
/// (`CallSelector::Feature`). Absent = `MISSING_FIELD`; `null` or any other
/// JSON type = `INVALID_INPUT`.
pub fn call_selector<'a>(
    args: &crate::Args<'a>,
    name: &str,
) -> Result<sz_configtool_lib::calls::CallSelector<'a>, crate::ApiError> {
    use sz_configtool_lib::calls::CallSelector;
    match args.raw(name) {
        None => Err(crate::ApiError::Config(
            sz_configtool_lib::SzConfigError::MissingField(name.to_string()),
        )),
        Some(serde_json::Value::String(s)) => Ok(CallSelector::Feature(s.as_str())),
        Some(v) => v.as_i64().map(CallSelector::Id).ok_or_else(|| {
            convert_invalid(format!(
                "argument '{name}' must be an integer call id or a feature code string"
            ))
        }),
    }
}

/// Required expression-call element list: a JSON array of objects
/// `{"element": str, "required": str, "feature"?: str}` →
/// `Vec<(element, required, Option<feature>)>`. `element` and `required` are
/// required strings; `feature` may be absent (not `null`). Unknown keys, a
/// non-array, or a non-object item = `INVALID_INPUT`; absent = `MISSING_FIELD`.
pub fn expression_element_list(
    args: &crate::Args<'_>,
    name: &str,
) -> Result<Vec<(String, String, Option<String>)>, crate::ApiError> {
    let items = args
        .req_json(name)?
        .as_array()
        .ok_or_else(|| convert_invalid(format!("argument '{name}' must be an array")))?;
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let at = format!("{name}[{i}]");
            let obj = item
                .as_object()
                .ok_or_else(|| convert_invalid(format!("{at} must be an object")))?;
            if let Some(k) = obj
                .keys()
                .find(|k| !["element", "required", "feature"].contains(&k.as_str()))
            {
                return Err(convert_invalid(format!("{at}: unknown key '{k}'")));
            }
            let field = |key: &str| -> Result<Option<String>, crate::ApiError> {
                match obj.get(key) {
                    None => Ok(None),
                    Some(serde_json::Value::String(s)) => Ok(Some(s.clone())),
                    Some(_) => Err(convert_invalid(format!("{at}.{key} must be a string"))),
                }
            };
            let need = |key: &str| -> Result<String, crate::ApiError> {
                field(key)?.ok_or_else(|| {
                    crate::ApiError::Config(sz_configtool_lib::SzConfigError::MissingField(
                        format!("{at}.{key}"),
                    ))
                })
            };
            Ok((need("element")?, need("required")?, field("feature")?))
        })
        .collect()
}

#[cfg(test)]
mod calls_converter_tests {
    use super::*;
    use serde_json::json;
    use sz_configtool_lib::calls::CallSelector;

    fn reason<T: std::fmt::Debug>(r: Result<T, crate::ApiError>) -> &'static str {
        r.expect_err("expected an error").reason_code()
    }

    #[test]
    fn test_call_selector_shapes() {
        let v = json!({"i": 7, "s": "NAME", "n": null, "f": 1.5, "b": true});
        let a = crate::Args::new(&v).unwrap();
        assert_eq!(call_selector(&a, "i").unwrap(), CallSelector::Id(7));
        assert_eq!(
            call_selector(&a, "s").unwrap(),
            CallSelector::Feature("NAME")
        );
        assert_eq!(reason(call_selector(&a, "absent")), "MISSING_FIELD");
        assert_eq!(reason(call_selector(&a, "n")), "INVALID_INPUT");
        assert_eq!(reason(call_selector(&a, "f")), "INVALID_INPUT");
        assert_eq!(reason(call_selector(&a, "b")), "INVALID_INPUT");
    }

    #[test]
    fn test_expression_element_list() {
        let v = json!({
            "ok": [{"element": "A", "required": "Yes"}, {"element": "B", "required": "No", "feature": "F"}],
            "empty": [],
            "obj": {"element": "A"},
            "item": ["A", "Yes"],
            "miss": [{"element": "A"}],
            "extra": [{"element": "A", "required": "Yes", "x": 1}],
            "badtype": [{"element": "A", "required": "Yes", "feature": null}]
        });
        let a = crate::Args::new(&v).unwrap();
        assert_eq!(
            expression_element_list(&a, "ok").unwrap(),
            vec![
                ("A".to_string(), "Yes".to_string(), None),
                ("B".to_string(), "No".to_string(), Some("F".to_string()))
            ]
        );
        assert!(expression_element_list(&a, "empty").unwrap().is_empty());
        assert_eq!(
            reason(expression_element_list(&a, "absent")),
            "MISSING_FIELD"
        );
        assert_eq!(reason(expression_element_list(&a, "obj")), "INVALID_INPUT");
        assert_eq!(reason(expression_element_list(&a, "item")), "INVALID_INPUT");
        assert_eq!(reason(expression_element_list(&a, "miss")), "MISSING_FIELD");
        assert_eq!(
            reason(expression_element_list(&a, "extra")),
            "INVALID_INPUT"
        );
        assert_eq!(
            reason(expression_element_list(&a, "badtype")),
            "INVALID_INPUT"
        );
    }
}

/// Required `str` arg as an owned `String`, for params-struct fields typed
/// `String` (e.g. `AddComparisonCallParams::ftype_code`). Same absent / null /
/// type rules as [`Args::req_str`].
pub fn owned_str(args: &crate::Args<'_>, name: &str) -> Result<String, crate::ApiError> {
    args.req_str(name).map(str::to_string)
}

#[cfg(test)]
mod owned_str_tests {
    use super::*;
    use crate::Args;
    use serde_json::json;

    #[test]
    fn test_owned_str_value_absent_null_wrong_type() {
        let v = json!({"s": "NAME", "n": null, "i": 1});
        let a = Args::new(&v).unwrap();
        assert_eq!(owned_str(&a, "s").unwrap(), "NAME".to_string());
        assert_eq!(
            owned_str(&a, "absent").unwrap_err().reason_code(),
            "MISSING_FIELD"
        );
        assert_eq!(
            owned_str(&a, "n").unwrap_err().reason_code(),
            "INVALID_INPUT"
        );
        assert_eq!(
            owned_str(&a, "i").unwrap_err().reason_code(),
            "INVALID_INPUT"
        );
    }
}

// ---------------------------------------------------------------------------
// featelem (features / elements) additions
// ---------------------------------------------------------------------------

/// A REQUIRED `str` arg passed to a params-struct field typed `Option<&str>`.
///
/// Some root params structs (`features::AddFeatureComparisonParams`,
/// `GetFeatureComparisonParams`, `AddFeatureDistinctCallElementParams`,
/// `elements::SetFeatureElementParams`) type mandatory codes as
/// `Option<&str>` and fail with `MISSING_FIELD` on `None`. On the wire the arg
/// is a plain required string: absent → `MISSING_FIELD`, `null` / non-string →
/// `INVALID_INPUT`, value → `Some(value)` (never `None`).
pub fn required_str_as_some<'a>(
    args: &crate::Args<'a>,
    name: &str,
) -> Result<Option<&'a str>, crate::ApiError> {
    args.req_str(name).map(Some)
}

#[cfg(test)]
mod featelem_convert_tests {
    use super::*;
    use crate::{ApiError, Args};
    use serde_json::json;

    #[test]
    fn test_required_str_as_some() {
        let v = json!({"code": "NAME", "n": null, "i": 3});
        let a = Args::new(&v).unwrap();
        assert_eq!(required_str_as_some(&a, "code").unwrap(), Some("NAME"));
        let reason = |r: Result<Option<&str>, ApiError>| r.unwrap_err().reason_code();
        assert_eq!(reason(required_str_as_some(&a, "absent")), "MISSING_FIELD");
        assert_eq!(reason(required_str_as_some(&a, "n")), "INVALID_INPUT");
        assert_eq!(reason(required_str_as_some(&a, "i")), "INVALID_INPUT");
    }
}

// ---------------------------------------------------------------------------
// A2 misc-group converters (export, search_profiles).
// ---------------------------------------------------------------------------

fn a2_invalid(msg: String) -> crate::ApiError {
    crate::ApiError::Config(sz_configtool_lib::SzConfigError::InvalidInput(msg))
}

/// Required `int` arg -> `usize` (e.g. `export::render_config`'s `indent`).
///
/// Wire shape: a JSON integer `>= 0`. A negative value is `INVALID_INPUT`
/// (absent is `MISSING_FIELD`, `null` is `INVALID_INPUT`, as for any required arg).
pub fn req_usize(args: &crate::Args<'_>, name: &str) -> Result<usize, crate::ApiError> {
    let n = args.req_int(name)?;
    usize::try_from(n)
        .map_err(|_| a2_invalid(format!("argument '{name}' must be a non-negative integer")))
}

/// Optional `json` arg -> `Vec<(feature_code, flag)>` for
/// `search_profiles::AddSearchProfileParams::elements`.
///
/// Wire shape: an array of `{"feature": "<FTYPE_CODE>", "flag": "<Yes|No|Y|N>"}`
/// objects (the same shape `list_search_profiles` reports in `overrides`).
/// Absent = empty list; `null`, a non-array, an element that is not an object,
/// a non-string `feature` or `flag`, or an unknown key is `INVALID_INPUT`; a
/// missing `feature` or `flag` key is `MISSING_FIELD` (same as
/// [`expression_element_list`] and a missing required arg).
pub fn search_profile_elements<'a>(
    args: &crate::Args<'a>,
    name: &str,
) -> Result<Vec<(&'a str, &'a str)>, crate::ApiError> {
    let Some(value) = args.opt_json(name)? else {
        return Ok(Vec::new());
    };
    let items = value
        .as_array()
        .ok_or_else(|| a2_invalid(format!("argument '{name}' must be an array")))?;
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let obj = item
                .as_object()
                .ok_or_else(|| a2_invalid(format!("{name}[{i}] must be an object")))?;
            if let Some(k) = obj.keys().find(|k| *k != "feature" && *k != "flag") {
                return Err(a2_invalid(format!("{name}[{i}]: unknown key '{k}'")));
            }
            let field = |key: &str| match obj.get(key) {
                None => Err(crate::ApiError::Config(
                    sz_configtool_lib::SzConfigError::MissingField(format!("{name}[{i}].{key}")),
                )),
                Some(v) => v
                    .as_str()
                    .ok_or_else(|| a2_invalid(format!("{name}[{i}].{key} must be a string"))),
            };
            Ok((field("feature")?, field("flag")?))
        })
        .collect()
}

#[cfg(test)]
mod a2_misc_converter_tests {
    use super::*;
    use serde_json::json;

    fn reason(e: &crate::ApiError) -> &'static str {
        e.reason_code()
    }

    #[test]
    fn test_req_usize() {
        let v = json!({"indent": 4, "neg": -1, "s": "x"});
        let args = crate::Args::new(&v).unwrap();
        assert_eq!(req_usize(&args, "indent").unwrap(), 4);
        assert_eq!(
            reason(&req_usize(&args, "neg").unwrap_err()),
            "INVALID_INPUT"
        );
        assert_eq!(reason(&req_usize(&args, "s").unwrap_err()), "INVALID_INPUT");
        assert_eq!(
            reason(&req_usize(&args, "absent").unwrap_err()),
            "MISSING_FIELD"
        );
    }

    #[test]
    fn test_search_profile_elements() {
        let v = json!({
            "ok": [{"feature": "NAME", "flag": "Yes"}, {"flag": "N", "feature": "DOB"}],
            "null": null,
            "obj": {},
            "scalar_item": ["NAME"],
            "missing_flag": [{"feature": "NAME"}],
            "missing_feature": [{"flag": "Y"}],
            "bad_type": [{"feature": "NAME", "flag": 1}],
            "extra_key": [{"feature": "NAME", "flag": "Y", "x": 1}],
        });
        let args = crate::Args::new(&v).unwrap();
        assert_eq!(
            search_profile_elements(&args, "ok").unwrap(),
            vec![("NAME", "Yes"), ("DOB", "N")]
        );
        assert!(search_profile_elements(&args, "absent").unwrap().is_empty());
        for bad in ["null", "obj", "scalar_item", "extra_key", "bad_type"] {
            let e = search_profile_elements(&args, bad).unwrap_err();
            assert_eq!(reason(&e), "INVALID_INPUT", "{bad}");
        }
        // A missing required item key is MISSING_FIELD, like a missing
        // required arg and like `expression_element_list`.
        for missing in ["missing_flag", "missing_feature"] {
            let e = search_profile_elements(&args, missing).unwrap_err();
            assert_eq!(reason(&e), "MISSING_FIELD", "{missing}");
        }
    }
}
