//! Matching function operations for Senzing configuration
//!
//! This module provides functions for managing matching functions in CFG_RTYPE
//! in the Senzing configuration JSON.
//!
//! Note: These are placeholder functions that will be implemented when the
//! CLI command implementations are completed.

use crate::error::SzConfigError;
use serde_json::Value;

/// Add a new matching function (placeholder)
pub fn add_matching_function(
    _config_json: &str,
    _rtype_code: &str,
    _matching_func: &str,
) -> Result<(String, Value), SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}

/// Delete a matching function (placeholder)
pub fn delete_matching_function(
    _config_json: &str,
    _rtype_code: &str,
) -> Result<(String, Value), SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}

/// Get a matching function (placeholder)
pub fn get_matching_function(
    _config_json: &str,
    _rtype_code: &str,
) -> Result<Value, SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}

/// List all matching functions (placeholder)
pub fn list_matching_functions(_config_json: &str) -> Result<Vec<Value>, SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}

/// Set (update) a matching function (placeholder)
pub fn set_matching_function(
    _config_json: &str,
    _rtype_code: &str,
    _matching_func: Option<&str>,
) -> Result<(String, Value), SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}

/// Remove a matching function (placeholder)
pub fn remove_matching_function(
    _config_json: &str,
    _rtype_code: &str,
) -> Result<(String, Value), SzConfigError> {
    Err(SzConfigError::not_implemented(
        "Matching functions are not yet fully implemented",
    ))
}
