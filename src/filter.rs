//! Substring-filter substrate for list/record matching.
//!
//! Several listing operations filter records by testing whether a
//! user-supplied term occurs as a substring of the record's *stringified*
//! form. The result of such a test depends entirely on **how the record is
//! stringified** — a term like `"K": 1` (with a space after the colon) matches
//! under a spaced JSON rendering but misses under a compact one.
//!
//! [`FilterSubstrate`] names the four renderings this substrate supports, and
//! [`matches_filter`] performs the substring test under a chosen rendering.
//!
//! ## Deliberate limitation: `ensure_ascii` is *not* reproduced
//!
//! Python's `json.dumps` defaults to `ensure_ascii=True`, escaping every
//! non-ASCII character to a `\uXXXX` sequence. [`to_json_dumps_string`] here
//! deliberately does **not** reproduce that: it emits UTF-8 text directly (as
//! `serde_json` does). For records containing non-ASCII text, a filter term may
//! therefore match here but not under real CPython `json.dumps`, and vice
//! versa. This mirrors the original tool's behaviour and is intentional; do not
//! "fix" it without a corresponding decision.

use serde_json::Value;

/// The record-stringification strategy used for substring filtering.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::filter::FilterSubstrate;
///
/// // Default matches the safe, spaced JSON rendering.
/// assert_eq!(FilterSubstrate::default(), FilterSubstrate::JsonDumps);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FilterSubstrate {
    /// `serde_json` compact form: no spaces (`{"K":1,"J":null}`).
    Compact,
    /// Python `json.dumps` default spacing: `", "` between items, `": "`
    /// between key and value (`{"K": 1, "J": null}`), but **without**
    /// `ensure_ascii` escaping (see the module note).
    #[default]
    JsonDumps,
    /// CPython `repr` of the equivalent Python object: `None`/`True`/`False`,
    /// single-quoted strings, `", "`/`": "` spacing (`{'K': 1, 'J': None}`).
    PythonRepr,
    /// Values-only, space-joined, Python-`str`-rendered form used by
    /// `do_listComparisonThresholds`: keys are dropped and each value is rendered
    /// as `str(v)` (bare unquoted strings, `None`/`True`/`False`) then joined with
    /// a single space (`{"K": "a", "J": null}` -> `a None`). See
    /// [`to_values_join_string`].
    ValuesJoin,
}

/// Render a JSON value the way Python's `json.dumps` does by default.
///
/// Containers use `", "` between items and `": "` between an object's key and
/// value; scalars are rendered as standard JSON. Non-ASCII text is emitted
/// directly as UTF-8 — the `ensure_ascii` escaping of real `json.dumps` is
/// **not** reproduced (see the module-level note).
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::filter::to_json_dumps_string;
///
/// let v = json!({"K": 1, "J": null});
/// assert_eq!(to_json_dumps_string(&v), r#"{"K": 1, "J": null}"#);
/// ```
pub fn to_json_dumps_string(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let inner: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}: {}",
                        serde_json::to_string(k).unwrap_or_else(|_| String::from("\"\"")),
                        to_json_dumps_string(v)
                    )
                })
                .collect();
            format!("{{{}}}", inner.join(", "))
        }
        Value::Array(arr) => {
            let inner: Vec<String> = arr.iter().map(to_json_dumps_string).collect();
            format!("[{}]", inner.join(", "))
        }
        // Scalars (null/bool/number/string) render identically to JSON; serde
        // keeps non-ASCII as UTF-8, which is exactly the limitation we want.
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// Render a string the way CPython's `repr` would.
///
/// Uses single quotes by default, switching to double quotes when the string
/// contains a single quote but no double quote (matching CPython). Backslash
/// and the common control characters (`\n`, `\r`, `\t`) are escaped. Non-ASCII
/// printable characters are emitted directly (the `\uXXXX` / `\xXX` escaping of
/// unprintable code points is not reproduced — this is an approximation aimed
/// at the common cases used in filtering).
fn python_repr_str(s: &str) -> String {
    let has_single = s.contains('\'');
    let has_double = s.contains('"');
    // CPython prefers single quotes, but uses double quotes to avoid escaping
    // when the string has a single quote and no double quote.
    let use_double = has_single && !has_double;
    let quote = if use_double { '"' } else { '\'' };

    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

/// Render a JSON value the way CPython's `repr` renders the equivalent object.
///
/// `null`/`true`/`false` become `None`/`True`/`False`; strings are
/// single-quoted (see [`python_repr_str`]); containers use `", "`/`": "`
/// spacing. Nesting is handled recursively.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::filter::to_python_repr_string;
///
/// let v = json!({"K": 1, "J": null, "L": true});
/// assert_eq!(to_python_repr_string(&v), "{'K': 1, 'J': None, 'L': True}");
/// ```
pub fn to_python_repr_string(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => python_repr_str(s),
        Value::Array(arr) => {
            let inner: Vec<String> = arr.iter().map(to_python_repr_string).collect();
            format!("[{}]", inner.join(", "))
        }
        Value::Object(map) => {
            let inner: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", python_repr_str(k), to_python_repr_string(v)))
                .collect();
            format!("{{{}}}", inner.join(", "))
        }
    }
}

/// Render a JSON value the way Python's `str()` builtin does.
///
/// This is the per-value rendering used by `do_listComparisonThresholds`, which
/// filters over `str(v)` for each dict *value* (not the whole dict). It differs
/// from [`to_python_repr_string`] only for scalars: `str("hi")` is the **bare**
/// `hi` (no quotes), whereas `repr("hi")` is `'hi'`. Containers are rendered
/// identically to `repr` (Python's `str()` of a list/dict delegates to `repr` of
/// its members), so they delegate to [`to_python_repr_string`] here.
///
/// - `null` -> `None`
/// - `true`/`false` -> `True`/`False`
/// - numbers -> their decimal form
/// - strings -> the string itself, unquoted
/// - arrays/objects -> as [`to_python_repr_string`]
fn python_str_string(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => to_python_repr_string(value),
    }
}

/// Render a JSON value as the values-only, space-joined string that
/// `do_listComparisonThresholds` filters against.
///
/// For an object, its **values** (keys dropped) are each rendered via Python
/// `str()` semantics (see [`python_str_string`]) and joined with a single space,
/// reproducing `" ".join(str(v) for v in record.values())`. Any non-object value
/// renders as a single `str()`-rendered value.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::filter::to_values_join_string;
///
/// let v = json!({"id": 2, "function": "GNR_COMP", "feature": "all", "note": null});
/// assert_eq!(to_values_join_string(&v), "2 GNR_COMP all None");
/// ```
pub fn to_values_join_string(value: &Value) -> String {
    match value {
        Value::Object(map) => map
            .values()
            .map(python_str_string)
            .collect::<Vec<_>>()
            .join(" "),
        other => python_str_string(other),
    }
}

/// Test whether `filter` occurs as a substring of `record` rendered under the
/// given [`FilterSubstrate`].
///
/// The match is **case-insensitive**: both the rendered record and the filter
/// term are case-folded before the substring test. This mirrors every real
/// `sz_configtool` list-filter site, which tests `arg.lower() in
/// str(record).lower()` — not the bare `in` operator. An empty filter always
/// matches.
///
/// ## Substrate and parity
///
/// The Python tool stringifies with `str(record)`, whose dict form is the
/// CPython `repr` (single quotes, `None`/`True`/`False`) — i.e.
/// [`FilterSubstrate::PythonRepr`]. Callers reproducing the tool's filtering
/// exactly should pass `PythonRepr`; the other substrates are offered for
/// callers filtering against a different rendering of their own choosing.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::filter::{matches_filter, FilterSubstrate};
///
/// let record = json!({"K": 1, "J": null});
///
/// // A boundary-spanning term matches under the spaced rendering...
/// assert!(matches_filter(&record, "\"K\": 1", FilterSubstrate::JsonDumps));
/// // ...but misses under the compact one (which has no space).
/// assert!(!matches_filter(&record, "\"K\": 1", FilterSubstrate::Compact));
/// ```
pub fn matches_filter(record: &Value, filter: &str, substrate: FilterSubstrate) -> bool {
    let haystack = match substrate {
        FilterSubstrate::Compact => serde_json::to_string(record).unwrap_or_default(),
        FilterSubstrate::JsonDumps => to_json_dumps_string(record),
        FilterSubstrate::PythonRepr => to_python_repr_string(record),
        FilterSubstrate::ValuesJoin => to_values_join_string(record),
    };
    // Case-fold both sides, matching the tool's `arg.lower() in str(...).lower()`.
    // An empty filter still matches (`contains("")` is always true).
    haystack.to_lowercase().contains(&filter.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_dumps_spacing() {
        let v = json!({"K": 1, "J": null});
        assert_eq!(to_json_dumps_string(&v), r#"{"K": 1, "J": null}"#);
        // Compact (serde) has no spaces — this is the contrast the bug hinges on.
        assert_eq!(serde_json::to_string(&v).unwrap(), r#"{"K":1,"J":null}"#);
    }

    #[test]
    fn test_json_dumps_nested_and_arrays() {
        let v = json!({"a": [1, 2, {"b": "c"}], "d": true});
        assert_eq!(
            to_json_dumps_string(&v),
            r#"{"a": [1, 2, {"b": "c"}], "d": true}"#
        );
    }

    #[test]
    fn test_python_repr_scalars() {
        assert_eq!(to_python_repr_string(&Value::Null), "None");
        assert_eq!(to_python_repr_string(&json!(true)), "True");
        assert_eq!(to_python_repr_string(&json!(false)), "False");
        assert_eq!(to_python_repr_string(&json!(42)), "42");
        assert_eq!(to_python_repr_string(&json!("hi")), "'hi'");
    }

    #[test]
    fn test_python_repr_nested() {
        let v = json!({"K": 1, "J": null, "nested": {"x": [true, false, null]}});
        assert_eq!(
            to_python_repr_string(&v),
            "{'K': 1, 'J': None, 'nested': {'x': [True, False, None]}}"
        );
    }

    #[test]
    fn test_python_repr_quote_selection() {
        // Default single quotes.
        assert_eq!(to_python_repr_string(&json!("plain")), "'plain'");
        // Contains a single quote but no double quote -> switch to double quotes.
        assert_eq!(to_python_repr_string(&json!("it's")), "\"it's\"");
        // Contains a double quote -> keep single quotes.
        assert_eq!(to_python_repr_string(&json!("say \"hi\"")), "'say \"hi\"'");
        // Contains both -> single quotes with the single quote escaped.
        assert_eq!(
            to_python_repr_string(&json!("it's \"x\"")),
            "'it\\'s \"x\"'"
        );
    }

    #[test]
    fn test_matches_filter_boundary_spanning() {
        // The #36 repro: {"K":1,"J":null} with a boundary-spanning filter.
        let record = json!({"K": 1, "J": null});

        // A term that spans the item boundary (value, next-key) only matches
        // under the spaced json.dumps rendering.
        assert!(matches_filter(
            &record,
            "1, \"J\"",
            FilterSubstrate::JsonDumps
        ));
        assert!(!matches_filter(
            &record,
            "1, \"J\"",
            FilterSubstrate::Compact
        ));

        // The key/value boundary likewise.
        assert!(matches_filter(
            &record,
            "\"K\": 1",
            FilterSubstrate::JsonDumps
        ));
        assert!(!matches_filter(
            &record,
            "\"K\": 1",
            FilterSubstrate::Compact
        ));
    }

    #[test]
    fn test_matches_filter_python_repr_none() {
        let record = json!({"K": 1, "J": null});
        // null renders as None under repr, as null under json.dumps.
        assert!(matches_filter(&record, "None", FilterSubstrate::PythonRepr));
        assert!(!matches_filter(&record, "None", FilterSubstrate::JsonDumps));
        assert!(matches_filter(&record, "null", FilterSubstrate::JsonDumps));
    }

    #[test]
    fn test_matches_filter_is_case_insensitive() {
        // The tool filters with `arg.lower() in str(record).lower()`, so a
        // differently-cased term must still match (e.g. `listRules none`
        // matching a `None`/`null` in the record).
        let record = json!({"RULE": "RESOLVE", "J": null});

        // Term case differs from the record in every substrate.
        assert!(matches_filter(
            &record,
            "resolve",
            FilterSubstrate::JsonDumps
        ));
        assert!(matches_filter(
            &record,
            "RESOLVE",
            FilterSubstrate::JsonDumps
        ));
        assert!(matches_filter(&record, "ReSoLvE", FilterSubstrate::Compact));
        // `none` matches the repr rendering of `null` (None) case-insensitively.
        assert!(matches_filter(&record, "none", FilterSubstrate::PythonRepr));
        // Non-substring still misses regardless of case.
        assert!(!matches_filter(
            &record,
            "candidate",
            FilterSubstrate::JsonDumps
        ));
    }

    #[test]
    fn test_matches_filter_empty_always_matches() {
        let record = json!({"a": 1});
        assert!(matches_filter(&record, "", FilterSubstrate::Compact));
        assert!(matches_filter(&record, "", FilterSubstrate::JsonDumps));
        assert!(matches_filter(&record, "", FilterSubstrate::PythonRepr));
    }

    #[test]
    fn test_values_join_bare_strings_and_none() {
        // Values-only, space-joined; strings are BARE (no quotes), null -> None.
        let v = json!({"id": 2, "function": "GNR_COMP", "feature": "all", "note": null});
        assert_eq!(to_values_join_string(&v), "2 GNR_COMP all None");
        // Contrast: repr would quote the strings and keep keys.
        assert_eq!(
            to_python_repr_string(&v),
            "{'id': 2, 'function': 'GNR_COMP', 'feature': 'all', 'note': None}"
        );
    }

    #[test]
    fn test_values_join_bools_and_numbers() {
        let v = json!({"a": true, "b": false, "c": 100, "d": "x"});
        assert_eq!(to_values_join_string(&v), "True False 100 x");
    }

    #[test]
    fn test_values_join_non_object() {
        // A non-object renders as a single str()-rendered value.
        assert_eq!(to_values_join_string(&json!("hi")), "hi");
        assert_eq!(to_values_join_string(&Value::Null), "None");
        assert_eq!(to_values_join_string(&json!(42)), "42");
        // Nested containers delegate to repr.
        assert_eq!(
            to_values_join_string(&json!([1, "a", null])),
            "[1, 'a', None]"
        );
    }

    #[test]
    fn test_matches_filter_values_join_boundary() {
        // The space-join boundary between two values is matchable, and the
        // keys are absent so a key-only term misses.
        let v = json!({"function": "GNR_COMP", "feature": "all"});
        // Boundary-spanning term across the single space between values.
        assert!(matches_filter(
            &v,
            "gnr_comp all",
            FilterSubstrate::ValuesJoin
        ));
        // The key "function" is dropped from the values-join rendering.
        assert!(!matches_filter(&v, "function", FilterSubstrate::ValuesJoin));
        // But it IS present under json.dumps (keys retained).
        assert!(matches_filter(&v, "function", FilterSubstrate::JsonDumps));
    }

    #[test]
    fn test_matches_filter_values_join_case_insensitive() {
        let v = json!({"function": "GNR_COMP", "feature": "all"});
        assert!(matches_filter(&v, "GNR_COMP", FilterSubstrate::ValuesJoin));
        assert!(matches_filter(&v, "gnr_comp", FilterSubstrate::ValuesJoin));
        assert!(!matches_filter(&v, "str_comp", FilterSubstrate::ValuesJoin));
    }

    #[test]
    fn test_values_join_against_real_template_cfrtn() {
        // Exercise ValuesJoin against the real Senzing v4 template: build a
        // comparison-threshold object from actual CFG_CFRTN rows the way Python's
        // do_listComparisonThresholds does, and assert a real CFUNC_CODE matches.
        let path = format!(
            "{}/tests/fixtures/g2config_template.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read template fixture '{path}': {e}"));
        let config: Value = serde_json::from_str(&raw).expect("template is not valid JSON");
        let g2 = &config["G2_CONFIG"];

        // Build CFUNC_ID -> CFUNC_CODE and FTYPE_ID -> FTYPE_CODE lookups.
        let cfunc_by_id: std::collections::HashMap<i64, &str> = g2["CFG_CFUNC"]
            .as_array()
            .expect("CFG_CFUNC array")
            .iter()
            .map(|r| {
                (
                    r["CFUNC_ID"].as_i64().unwrap(),
                    r["CFUNC_CODE"].as_str().unwrap(),
                )
            })
            .collect();
        let ftype_by_id: std::collections::HashMap<i64, &str> = g2["CFG_FTYPE"]
            .as_array()
            .expect("CFG_FTYPE array")
            .iter()
            .map(|r| {
                (
                    r["FTYPE_ID"].as_i64().unwrap(),
                    r["FTYPE_CODE"].as_str().unwrap(),
                )
            })
            .collect();

        // Find a CFRTN row whose function is GNR_COMP (a real code in the template).
        let cfrtn_rows = g2["CFG_CFRTN"].as_array().expect("CFG_CFRTN array");
        let mut gnr_matches = 0usize;
        for row in cfrtn_rows {
            let cfunc_id = row["CFUNC_ID"].as_i64().unwrap();
            let cfunc_code = cfunc_by_id[&cfunc_id];
            let ftype_id = row.get("FTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(0);
            let feature = if ftype_id != 0 {
                ftype_by_id[&ftype_id]
            } else {
                "all"
            };
            // Mirror format_comparison_threshold_json's field order.
            let cfrtn_json = json!({
                "id": row["CFRTN_ID"],
                "function": cfunc_code,
                "returnOrder": row["EXEC_ORDER"],
                "scoreName": row["CFUNC_RTNVAL"],
                "feature": feature,
                "sameScore": row["SAME_SCORE"],
                "closeScore": row["CLOSE_SCORE"],
                "likelyScore": row["LIKELY_SCORE"],
                "plausibleScore": row["PLAUSIBLE_SCORE"],
                "unlikelyScore": row["UN_LIKELY_SCORE"],
            });

            if matches_filter(&cfrtn_json, "GNR_COMP", FilterSubstrate::ValuesJoin) {
                gnr_matches += 1;
                // The rendered form must be values-only and space-joined.
                let rendered = to_values_join_string(&cfrtn_json);
                assert!(rendered.contains("GNR_COMP"));
                assert!(!rendered.contains("function")); // key dropped
                assert!(!rendered.contains('{')); // not a dict rendering
            }
        }
        assert!(
            gnr_matches > 0,
            "expected at least one GNR_COMP threshold in the real template"
        );
    }

    #[test]
    fn test_ensure_ascii_not_reproduced() {
        // Deliberate limitation: non-ASCII is emitted as UTF-8, not \uXXXX.
        let v = json!({"name": "café"});
        assert_eq!(to_json_dumps_string(&v), r#"{"name": "café"}"#);
        assert!(!to_json_dumps_string(&v).contains("\\u"));
    }
}
