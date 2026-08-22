//! Substring-filter substrate for list/record matching.
//!
//! Several listing operations filter records by testing whether a
//! user-supplied term occurs as a substring of the record's *stringified*
//! form. The result of such a test depends entirely on **how the record is
//! stringified** — a term like `"K": 1` (with a space after the colon) matches
//! under a spaced JSON rendering but misses under a compact one.
//!
//! [`FilterSubstrate`] names the three renderings this substrate supports, and
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

/// Test whether `filter` occurs as a substring of `record` rendered under the
/// given [`FilterSubstrate`].
///
/// The match is case-sensitive, mirroring Python's `in` operator on strings.
/// An empty filter always matches.
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
    };
    haystack.contains(filter)
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
    fn test_matches_filter_empty_always_matches() {
        let record = json!({"a": 1});
        assert!(matches_filter(&record, "", FilterSubstrate::Compact));
        assert!(matches_filter(&record, "", FilterSubstrate::JsonDumps));
        assert!(matches_filter(&record, "", FilterSubstrate::PythonRepr));
    }

    #[test]
    fn test_ensure_ascii_not_reproduced() {
        // Deliberate limitation: non-ASCII is emitted as UTF-8, not \uXXXX.
        let v = json!({"name": "café"});
        assert_eq!(to_json_dumps_string(&v), r#"{"name": "café"}"#);
        assert!(!to_json_dumps_string(&v).contains("\\u"));
    }
}
