//! Function management operations for Senzing configuration
//!
//! This module provides functions for managing various function types in the
//! Senzing configuration JSON, including:
//! - Standardize functions (CFG_SFUNC)
//! - Expression functions (CFG_EFUNC)
//! - Comparison functions (CFG_CFUNC) and return codes (CFG_CFRTN)
//! - Distinct functions (CFG_DFUNC)
//! - Matching functions (CFG_RTYPE - placeholder)
//! - Scoring functions (CFG_RTYPE - placeholder)
//! - Candidate functions (CFG_RTYPE - placeholder)
//! - Validation functions (CFG_ATTR - placeholder)

pub mod candidate;
pub mod comparison;
pub mod distinct;
pub mod expression;
pub mod matching;
pub mod scoring;
pub mod standardize;
pub mod validation;

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
    add_comparison_func_return_code, add_comparison_function, delete_comparison_function,
    get_comparison_function, list_comparison_functions, set_comparison_function,
};

pub use distinct::{
    add_distinct_function, delete_distinct_function, get_distinct_function,
    list_distinct_functions, set_distinct_function,
};

pub use matching::{
    add_matching_function, delete_matching_function, get_matching_function,
    list_matching_functions, remove_matching_function, set_matching_function,
};

pub use scoring::{
    add_scoring_function, delete_scoring_function, get_scoring_function, list_scoring_functions,
    remove_scoring_function, set_scoring_function,
};

pub use candidate::{
    add_candidate_function, delete_candidate_function, get_candidate_function,
    list_candidate_functions, remove_candidate_function, set_candidate_function,
};

pub use validation::{
    add_validation_function, delete_validation_function, get_validation_function,
    list_validation_functions, remove_validation_function, set_validation_function,
};
