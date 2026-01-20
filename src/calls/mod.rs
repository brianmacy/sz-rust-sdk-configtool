//! Call management operations
//!
//! This module provides functions for managing the four types of calls in
//! Senzing configuration:
//!
//! - **Standardize calls** (CFG_SFCALL/CFG_SBOM) - Data standardization operations
//! - **Expression calls** (CFG_EFCALL/CFG_EFBOM) - Feature expression operations
//! - **Comparison calls** (CFG_CFCALL/CFG_CFBOM) - Feature comparison operations
//! - **Distinct calls** (CFG_DFCALL/CFG_DFBOM) - Feature distinctness operations
//!
//! Each call type links functions to features/elements with execution order and
//! maintains associated bill of materials (BOM) records for element relationships.

pub mod comparison;
pub mod distinct;
pub mod expression;
pub mod standardize;

// Re-export commonly used functions for convenience
pub use standardize::{
    add_standardize_call, add_standardize_call_element, delete_standardize_call,
    delete_standardize_call_element, get_standardize_call, list_standardize_calls,
    set_standardize_call, set_standardize_call_element,
};

pub use expression::{
    add_expression_call, add_expression_call_element, delete_expression_call,
    delete_expression_call_element, get_expression_call, list_expression_calls,
    set_expression_call, set_expression_call_element,
};

pub use comparison::{
    add_comparison_call, add_comparison_call_element, delete_comparison_call,
    delete_comparison_call_element, get_comparison_call, list_comparison_calls,
    set_comparison_call, set_comparison_call_element,
};

pub use distinct::{
    add_distinct_call, add_distinct_call_element, delete_distinct_call,
    delete_distinct_call_element, get_distinct_call, list_distinct_calls, set_distinct_call,
    set_distinct_call_element,
};
