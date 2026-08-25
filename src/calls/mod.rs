//! Call management operations
//!
//! This module provides functions for managing the four types of calls in
//! Senzing configuration:
//!
//! - **Standardize calls** (CFG_SFCALL) - Data standardization operations
//! - **Expression calls** (CFG_EFCALL/CFG_EFBOM) - Feature expression operations
//! - **Comparison calls** (CFG_CFCALL/CFG_CFBOM) - Feature comparison operations
//! - **Distinct calls** (CFG_DFCALL/CFG_DFBOM) - Feature distinctness operations
//!
//! Each call type links functions to features/elements with execution order and
//! maintains associated bill of materials (BOM) records for element relationships.
//!
//! # Execution-order policy
//!
//! Every add path that writes an `EXEC_ORDER` resolves it through one shared
//! helper, [`crate::helpers::get_desired_or_next_order`], so the behaviour is
//! uniform across the SDK. The three intents are:
//!
//! - **Auto-allocate** (`exec_order: None`): the next free order *within the
//!   row's scope* is used (max in scope + 1, seeded at `0` so an empty scope
//!   starts at `1`).
//! - **Honour** (`Some(n)`, `n > 0`, free in scope): the requested order is used
//!   verbatim.
//! - **Reject** (`Some(n)`, `n > 0`, already taken in scope): the call fails with
//!   `AlreadyExists` rather than silently reallocating (the SDK-wide
//!   reject-if-taken policy). A non-positive value falls through to
//!   auto-allocation.
//!
//! An order is always resolved to a concrete value — it is never written as
//! `null`. The *scope* (the senior key fields that partition the order space)
//! differs per family:
//!
//! | Path | Section | Scope |
//! |------|---------|-------|
//! | `add_standardize_call` / `add_standardize_call_element` | `CFG_SFCALL` | `(FTYPE_ID, FELEM_ID)` |
//! | `add_expression_call` | `CFG_EFCALL` | `(FTYPE_ID, FELEM_ID)` |
//! | comparison / expression / distinct `add_*_call_element` | `CFG_?FBOM` | `(call id)` |
//! | `features::add_feature_comparison` | `CFG_FBOM` | whole table |
//! | `thresholds::add_comparison_threshold` | `CFG_CFRTN` | `(CFUNC_ID, FTYPE_ID=0)`, after tier reuse |
//!
//! The comparison-threshold path additionally reuses an existing all-features
//! (`FTYPE_ID = 0`) return-value tier's order before falling back to the policy
//! above — see [`crate::thresholds`]. The `add_feature` bulk builder and the
//! positional BOM loops assign `EXEC_ORDER` by 1-based list position and are
//! deliberately outside this policy.

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

/// Derive the `EXEC_ORDER` of the single BOM row addressed by
/// (call, element[, element feature]).
///
/// A BOM record (`CFG_CFBOM` / `CFG_DFBOM` / `CFG_EFBOM`) stores the *element's*
/// feature id in its `FTYPE_ID` (not the call's). One call can carry the same
/// element (`FELEM_ID`) under several element-features — the stock config does
/// exactly this (e.g. `EFCALL_ID 97` / `TOKENIZED_NM` under both
/// `GROUP_ASSOCIATION` and `EMPLOYER`). When `element_ftype_id` is supplied it is
/// added to the match so that collision resolves to the feature-matched row,
/// mirroring Python's `(call_id, FTYPE_ID, FELEM_ID)` addressing. When it is
/// `None` the row is located by (call id, element id) alone.
///
/// Returns the located row's `EXEC_ORDER`. Errors with `NotFound` when no row
/// matches and `InvalidInput` when more than one does (an ambiguous
/// `(call, element)` — the caller should pass the element's feature).
pub(crate) fn derive_bom_exec_order(
    root: &Value,
    section: &str,
    call_id_field: &str,
    call_id: i64,
    felem_id: i64,
    element_ftype_id: Option<i64>,
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
                && element_ftype_id
                    .is_none_or(|ft| r.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ft))
        })
        .filter_map(|r| r.get("EXEC_ORDER").and_then(|v| v.as_i64()))
        .collect();

    match orders.as_slice() {
        [] => Err(SzConfigError::NotOnCall(format!(
            "{label} call element not found"
        ))),
        [order] => Ok(*order),
        many => Err(SzConfigError::InvalidInput(format!(
            "Ambiguous {label} call element: element matches {} rows across features; specify the element's feature to disambiguate",
            many.len()
        ))),
    }
}

/// Assert that a call with `call_id` exists in `section`, returning `NotFound`
/// otherwise.
///
/// This distinguishes the two "no BOM row" situations that
/// [`derive_bom_exec_order`] alone cannot: a *missing element on an existing
/// call* (the benign [`SzConfigError::NotOnCall`]) versus a *call that does not
/// exist at all* (a hard [`SzConfigError::NotFound`], matching Python's
/// `prepCallElement`, which errors on a missing call record before ever looking
/// at the BOM). Delete-by-`CallSelector::Id` skips the resolver's existence
/// check, so the call-family delete paths call this first.
pub(crate) fn ensure_call_exists(
    root: &Value,
    section: &str,
    id_field: &str,
    call_id: i64,
    label: &str,
) -> Result<()> {
    let exists = root
        .get("G2_CONFIG")
        .and_then(|g| g.get(section))
        .and_then(|v| v.as_array())
        .is_some_and(|rows| {
            rows.iter()
                .any(|r| r.get(id_field).and_then(|v| v.as_i64()) == Some(call_id))
        });
    if exists {
        Ok(())
    } else {
        Err(SzConfigError::NotFound(format!(
            "{label} call ID {call_id} does not exist"
        )))
    }
}

/// Resolve an element to its `FELEM_ID`, requiring it to be a member of
/// `feature_code`'s definition (`CFG_FBOM` for `ftype_id`).
///
/// This mirrors Python `prepCallElement`'s feature-scoped branch: when a
/// call-element op names an element *feature*, the element must first be an
/// element of that feature (`lookupFeatureElement`). If it is not — whether the
/// element code does not exist at all, or exists but is not in this feature's
/// `CFG_FBOM` — that is a hard [`SzConfigError::NotInFeature`]
/// (`"{element} is not an element of {feature}"`), distinct from the benign
/// [`SzConfigError::NotOnCall`] returned when the element *is* a feature member
/// but simply not on this particular call.
///
/// (The feature-less path — no element feature given — keeps a plain global
/// [`crate::helpers::lookup_element_id`] and cannot produce this error, matching
/// Python's `ftype_id < 0` branch.)
pub(crate) fn resolve_feature_element_id(
    root: &Value,
    config: &str,
    ftype_id: i64,
    element_code: &str,
    feature_code: &str,
) -> Result<i64> {
    let not_in_feature = || {
        SzConfigError::NotInFeature(format!(
            "{element_code} is not an element of {feature_code}"
        ))
    };
    // The element must exist globally AND be a CFG_FBOM member of the feature.
    let felem_id =
        crate::helpers::lookup_element_id(config, element_code).map_err(|_| not_in_feature())?;
    let in_feature = root
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FBOM"))
        .and_then(|v| v.as_array())
        .is_some_and(|rows| {
            rows.iter().any(|r| {
                r.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id)
                    && r.get("FELEM_ID").and_then(|v| v.as_i64()) == Some(felem_id)
            })
        });
    if in_feature {
        Ok(felem_id)
    } else {
        Err(not_in_feature())
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
