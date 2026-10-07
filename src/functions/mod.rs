//! Function management operations for Senzing configuration
//!
//! This module provides functions for managing various function types in the
//! Senzing configuration JSON, including:
//! - Standardize functions (CFG_SFUNC)
//! - Expression functions (CFG_EFUNC)
//! - Comparison functions (CFG_CFUNC)
//! - Distinct functions (CFG_DFUNC)

pub mod comparison;
pub mod distinct;
pub mod expression;
pub mod standardize;

// Re-export commonly used functions
pub use standardize::{
    add_standardize_function, delete_standardize_function, get_standardize_function,
    list_standardize_functions, set_standardize_function,
};

pub use expression::{
    add_expression_function, delete_expression_function, get_expression_function,
    list_expression_functions, set_expression_function,
};

pub use comparison::{
    add_comparison_function, delete_comparison_function, get_comparison_function,
    list_comparison_functions, set_comparison_function,
};

pub use distinct::{
    add_distinct_function, delete_distinct_function, get_distinct_function,
    list_distinct_functions, set_distinct_function,
};

/// The update shared by the `set_*_function` operations: find the `section`
/// row whose `code_field` is `code` (`not_found` when absent), let `update`
/// edit a copy, then drop every row with that code and append the edited row
/// (the historical delete-then-add, which moves the row to the end).
///
/// Errors: unparsable config -> `JsonParse`; no such row -> `not_found()`;
/// a row matched only numerically (its code stored as a number) cannot be
/// dropped by code -> `NotFound("<section> '<code>' not found")`.
pub(crate) fn replace_row_at_end(
    config_json: &str,
    section: &str,
    code_field: &str,
    code: &str,
    not_found: impl FnOnce() -> crate::error::SzConfigError,
    update: impl FnOnce(&mut serde_json::Map<String, serde_json::Value>),
) -> crate::error::Result<(String, serde_json::Value)> {
    use serde_json::Value;
    let mut config: Value = serde_json::from_str(config_json)
        .map_err(|e| crate::error::SzConfigError::JsonParse(e.to_string()))?;
    // A row matched by a field is an object (`get` on anything else is None).
    let mut row = crate::helpers::find_in_section(&config, section, code_field, code)
        .and_then(Value::as_object)
        .cloned()
        .ok_or_else(not_found)?;
    update(&mut row);

    let rows = crate::helpers::verified_section_mut(&mut config, section);
    let before = rows.len();
    rows.retain(|r| r.get(code_field).and_then(Value::as_str) != Some(code));
    if rows.len() == before {
        return Err(crate::error::SzConfigError::NotFound(format!(
            "{section} '{code}' not found"
        )));
    }
    let row = Value::Object(row);
    rows.push(row.clone());
    Ok((config.to_string(), row))
}
