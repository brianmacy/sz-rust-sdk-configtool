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

use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// Selects a call either by its numeric call id or by the feature code it is
/// bound to.
///
/// The `get_*_call` and `delete_*_call_element` entry points accept this so a
/// caller can pass whichever it already holds — a raw `*CALL_ID` or the feature
/// code the user typed — without a pre-resolution round trip. When a feature
/// code is given the SDK scans the relevant `CFG_*CALL` section by `FTYPE_ID`
/// (see the `helpers::resolve_*call_id_for_feature` resolvers), which owns the
/// per-family multiplicity policy and errors on an ambiguous match rather than
/// silently picking the wrong call.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::calls::CallSelector;
///
/// let by_id = CallSelector::Id(1001);
/// let by_feature = CallSelector::Feature("NAME");
/// assert_ne!(by_id, by_feature);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallSelector<'a> {
    /// Address the call directly by its `*CALL_ID`.
    Id(i64),
    /// Address the call by the feature code it is bound to.
    Feature(&'a str),
}

/// Resolve a [`CallSelector`] to a concrete call id.
///
/// For [`CallSelector::Id`] the id is returned as-is. For
/// [`CallSelector::Feature`] the feature code is resolved to its `FTYPE_ID`
/// (via [`crate::helpers::lookup_feature_id`]) and then to the bound call id via
/// `resolver` (one of the `helpers::resolve_*call_id_for_feature` functions),
/// which errors on an ambiguous or missing match.
///
/// `root` is the already-parsed configuration; `config` is the same document as
/// a string (needed by the string-based feature lookup helper).
pub(crate) fn resolve_call_id(
    config: &str,
    root: &Value,
    selector: CallSelector,
    resolver: fn(&Value, i64) -> Result<i64>,
) -> Result<i64> {
    match selector {
        CallSelector::Id(id) => Ok(id),
        CallSelector::Feature(code) => {
            let ftype_id = crate::helpers::lookup_feature_id(config, code)?;
            resolver(root, ftype_id)
        }
    }
}

/// Derive the `EXEC_ORDER` of the single BOM row addressed by (call, element).
///
/// A BOM record (`CFG_CFBOM` / `CFG_DFBOM` / `CFG_EFBOM`) stores the *element's*
/// feature id in its `FTYPE_ID`, not the call's, so `FTYPE_ID` is deliberately
/// **not** part of the address here — the row is located by (call id, element
/// id) alone. Returns the located row's `EXEC_ORDER`, so callers no longer need
/// to supply it.
///
/// Errors with `NotFound` when no row matches and `InvalidInput` when more than
/// one does (the element cannot be uniquely addressed without more information).
pub(crate) fn derive_bom_exec_order(
    root: &Value,
    section: &str,
    call_id_field: &str,
    call_id: i64,
    felem_id: i64,
    label: &str,
) -> Result<i64> {
    let empty: Vec<Value> = Vec::new();
    let rows = root
        .get("G2_CONFIG")
        .and_then(|g| g.get(section))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty);

    let orders: Vec<i64> = rows
        .iter()
        .filter(|r| {
            r.get(call_id_field).and_then(|v| v.as_i64()) == Some(call_id)
                && r.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
        })
        .filter_map(|r| r.get("EXEC_ORDER").and_then(|v| v.as_i64()))
        .collect();

    match orders.as_slice() {
        [] => Err(SzConfigError::NotFound(format!(
            "{label} call element not found"
        ))),
        [order] => Ok(*order),
        many => Err(SzConfigError::InvalidInput(format!(
            "Ambiguous {label} call element: element matches {} rows; cannot derive execution order",
            many.len()
        ))),
    }
}

// `CallSelector` is exported at the module root so callers can write
// `calls::CallSelector` regardless of which call family they are addressing.

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
