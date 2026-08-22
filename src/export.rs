//! Canonical config-document export renderer.
//!
//! This module provides [`render_config`], the single canonical "render a config
//! document for export" entry point (decision D31). It reproduces the on-disk
//! form the CLI's `exportToFile` (and the shell's historic-config path) produce:
//! a **recursive key sort** — the semantics of Python's `json.dumps(...,
//! sort_keys=True)` — followed by a pretty-print at a caller-chosen indent.
//!
//! The indent is a **required parameter** and is never hardcoded: the CLI
//! deliberately exports at 2 spaces while the Python tooling uses 4, and both
//! must be expressible through one renderer.
//!
//! # Why the explicit sort
//!
//! This crate enables `serde_json`'s `preserve_order` feature, so a parsed
//! object keeps its original key order rather than sorting. Producing a
//! canonical (order-independent) rendering therefore requires rebuilding every
//! object with its keys in sorted order, recursively — which is exactly what
//! this module does before serialising.

use crate::error::{Result, SzConfigError};
use serde_json::{Map, Value};

/// Recursively rebuild a JSON value with every object's keys in sorted order.
///
/// Arrays keep their element order (only *object keys* are sorted, matching
/// Python's `sort_keys=True`); each element is itself recursively sorted.
fn sort_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut sorted = Map::with_capacity(map.len());
            for key in keys {
                sorted.insert(key.clone(), sort_value(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(sort_value).collect()),
        other => other.clone(),
    }
}

/// Render a config document to its canonical export form.
///
/// The input is parsed, every object's keys are sorted recursively (the
/// semantics of Python's `json.dumps(..., sort_keys=True)`), and the result is
/// pretty-printed using `indent` spaces per level. `indent` is required and is
/// applied verbatim — pass `2` for the CLI's on-disk form or `4` for Python
/// parity.
///
/// The rendering is deterministic: two inputs that are semantically equal (equal
/// as parsed JSON, regardless of key order) render to byte-identical output.
///
/// # Arguments
/// * `config_json` - Configuration JSON string
/// * `indent` - Number of spaces per indentation level (may be `0`)
///
/// # Returns
/// * `Ok(String)` - the canonical, key-sorted, pretty-printed rendering
/// * `Err(SzConfigError::JsonParse)` - if `config_json` is not valid JSON
///
/// # Example
/// ```
/// use sz_configtool_lib::export::render_config;
///
/// let a = r#"{"b":1,"a":{"y":2,"x":3}}"#;
/// let rendered = render_config(a, 2)?;
/// // Keys are sorted at every level.
/// assert!(rendered.find("\"a\"").unwrap() < rendered.find("\"b\"").unwrap());
/// assert!(rendered.contains("  \"a\": {"));
///
/// // Deterministic: a reordered-but-equal input renders identically.
/// let b = r#"{"a":{"x":3,"y":2},"b":1}"#;
/// assert_eq!(render_config(a, 2)?, render_config(b, 2)?);
/// # Ok::<(), sz_configtool_lib::error::SzConfigError>(())
/// ```
pub fn render_config(config_json: &str, indent: usize) -> Result<String> {
    let value: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let sorted = sort_value(&value);

    let indent_bytes = vec![b' '; indent];
    let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
    let mut buf = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
    serde::Serialize::serialize(&sorted, &mut serializer)
        .map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    String::from_utf8(buf).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_key_sort_at_indent_2() {
        let input = r#"{"b":1,"a":{"y":2,"x":3},"c":[{"z":1,"m":2}]}"#;
        let out = render_config(input, 2).unwrap();
        let expected = "{\n  \"a\": {\n    \"x\": 3,\n    \"y\": 2\n  },\n  \"b\": 1,\n  \"c\": [\n    {\n      \"m\": 2,\n      \"z\": 1\n    }\n  ]\n}";
        assert_eq!(out, expected);
    }

    #[test]
    fn test_indent_4() {
        let input = r#"{"a":{"x":1}}"#;
        let out = render_config(input, 4).unwrap();
        let expected = "{\n    \"a\": {\n        \"x\": 1\n    }\n}";
        assert_eq!(out, expected);
    }

    #[test]
    fn test_indent_is_not_hardcoded() {
        let input = r#"{"a":1}"#;
        assert_ne!(
            render_config(input, 2).unwrap(),
            render_config(input, 4).unwrap()
        );
    }

    #[test]
    fn test_deterministic_regardless_of_key_order() {
        let a = r#"{"b":1,"a":{"y":2,"x":3}}"#;
        let b = r#"{"a":{"x":3,"y":2},"b":1}"#;
        assert_eq!(render_config(a, 2).unwrap(), render_config(b, 2).unwrap());
        assert_eq!(render_config(a, 4).unwrap(), render_config(b, 4).unwrap());
    }

    #[test]
    fn test_semantic_round_trip() {
        let input =
            r#"{"G2_CONFIG":{"CFG_DSRC":[{"DSRC_ID":1,"DSRC_CODE":"TEST"}],"CFG_FTYPE":[]}}"#;
        let rendered = render_config(input, 2).unwrap();
        let original: Value = serde_json::from_str(input).unwrap();
        let round: Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(original, round);
    }

    #[test]
    fn test_round_trip_on_stock_template() {
        let config = include_str!("../tests/fixtures/g2config_template.json");
        let rendered = render_config(config, 2).unwrap();
        let original: Value = serde_json::from_str(config).unwrap();
        let round: Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(original, round);
        // Rendering the rendered output is a fixed point.
        assert_eq!(rendered, render_config(&rendered, 2).unwrap());
    }
}
