//! Execution-order auto-allocation (v0.7.0 STEP C / #55) exercised against the
//! REAL Senzing v4 template (`tests/fixtures/g2config_template.json`), never
//! synthetic-only config. Two past regressions slipped through synthetic tests,
//! so the allocation/tier policy is proven here on the shipped data.
//!
//! Covers:
//!  - E1 `features::add_feature_comparison` — whole-table CFG_FBOM allocation.
//!  - E2 `calls::standardize::add_standardize_call_element` — per (FTYPE_ID,
//!    FELEM_ID) allocation.
//!  - E3 `thresholds::add_comparison_threshold` — CFRTN all-features tier REUSE
//!    (load-bearing scoring invariant), next-available, and reject-if-taken.
//!  - D1/D2/D3 comparison/expression/distinct `add_*_call_element` — per-call
//!    allocation (None -> max+1, explicit-free honoured, explicit-taken ->
//!    AlreadyExists) plus the distinct dup-check realignment (a duplicate
//!    element is AlreadyPresent, not AlreadyExists — step D).
//!  - command_processor `addComparisonCallElement` dispatch (per-call order).

use serde_json::Value;
use sz_configtool_lib::calls::comparison::{
    AddComparisonCallElementParams, add_comparison_call_element,
};
use sz_configtool_lib::calls::distinct::{AddDistinctCallElementParams, add_distinct_call_element};
use sz_configtool_lib::calls::expression::{
    ExpressionCallElementParams, add_expression_call_element,
};
use sz_configtool_lib::calls::standardize::{
    AddStandardizeCallElementParams, add_standardize_call_element,
};
use sz_configtool_lib::command_processor::CommandProcessor;
use sz_configtool_lib::error::SzErrorKind;
use sz_configtool_lib::features::{AddFeatureComparisonParams, add_feature_comparison};
use sz_configtool_lib::thresholds::{AddComparisonThresholdParams, add_comparison_threshold};

fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read config fixture '{path}': {e}"))
}

fn parse(config: &str) -> Value {
    serde_json::from_str(config).unwrap()
}

fn rows<'a>(v: &'a Value, section: &str) -> &'a Vec<Value> {
    v["G2_CONFIG"][section].as_array().unwrap()
}

fn feature_id(v: &Value, code: &str) -> i64 {
    rows(v, "CFG_FTYPE")
        .iter()
        .find(|r| r["FTYPE_CODE"].as_str() == Some(code))
        .and_then(|r| r["FTYPE_ID"].as_i64())
        .unwrap_or_else(|| panic!("feature {code} not in template"))
}

fn element_id(v: &Value, code: &str) -> i64 {
    rows(v, "CFG_FELEM")
        .iter()
        .find(|r| r["FELEM_CODE"].as_str() == Some(code))
        .and_then(|r| r["FELEM_ID"].as_i64())
        .unwrap_or_else(|| panic!("element {code} not in template"))
}

// ============================================================================
// E1: add_feature_comparison — whole-table CFG_FBOM allocation
// ============================================================================

#[test]
fn e1_add_feature_comparison_allocates_whole_table_next() {
    let config = template();
    let before = parse(&config);
    let whole_max = rows(&before, "CFG_FBOM")
        .iter()
        .filter_map(|r| r["EXEC_ORDER"].as_i64())
        .max()
        .unwrap();

    // GENDER exists but is not an FBOM element of NAME.
    let params = AddFeatureComparisonParams::new("NAME", "GENDER");
    let modified = add_feature_comparison(&config, params).unwrap();
    let after = parse(&modified);

    let name_id = feature_id(&after, "NAME");
    let gender_id = element_id(&after, "GENDER");
    let new_row = rows(&after, "CFG_FBOM")
        .iter()
        .find(|r| {
            r["FTYPE_ID"].as_i64() == Some(name_id) && r["FELEM_ID"].as_i64() == Some(gender_id)
        })
        .expect("new NAME/GENDER FBOM row");
    // None -> next order across the whole table.
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(whole_max + 1));
}

#[test]
fn e1_add_feature_comparison_rejects_taken_whole_table_order() {
    let config = template();
    // Order 1 is certainly used somewhere in the whole CFG_FBOM table, so a
    // whole-table honour of 1 must be rejected.
    let params = AddFeatureComparisonParams::new("NAME", "GENDER").with_exec_order(1);
    let err = add_feature_comparison(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

// ============================================================================
// E2: add_standardize_call_element — per (FTYPE_ID, FELEM_ID) allocation
// ============================================================================

#[test]
fn e2_add_standardize_call_element_allocates_per_feature_element() {
    let config = template();
    let before = parse(&config);

    // Scope (FTYPE_ID = NAME, FELEM_ID = -1) carries standardize calls in the
    // template; compute its current max and a free SFUNC_ID for that scope.
    let name_id = feature_id(&before, "NAME");
    let scope_orders: Vec<i64> = rows(&before, "CFG_SFCALL")
        .iter()
        .filter(|r| r["FTYPE_ID"].as_i64() == Some(name_id) && r["FELEM_ID"].as_i64() == Some(-1))
        .filter_map(|r| r["EXEC_ORDER"].as_i64())
        .collect();
    let scope_max = *scope_orders.iter().max().unwrap();
    let used_sfuncs: Vec<i64> = rows(&before, "CFG_SFCALL")
        .iter()
        .filter(|r| r["FTYPE_ID"].as_i64() == Some(name_id) && r["FELEM_ID"].as_i64() == Some(-1))
        .filter_map(|r| r["SFUNC_ID"].as_i64())
        .collect();
    let free_sfunc = rows(&before, "CFG_SFUNC")
        .iter()
        .filter_map(|r| r["SFUNC_ID"].as_i64())
        .find(|id| !used_sfuncs.contains(id))
        .expect("a SFUNC_ID free for the NAME/-1 scope");

    let params = AddStandardizeCallElementParams {
        ftype_id: name_id,
        sfunc_id: free_sfunc,
        felem_id: None, // -> -1
        exec_order: None,
    };
    let (modified, _) = add_standardize_call_element(&config, params).unwrap();
    let after = parse(&modified);

    let new_row = rows(&after, "CFG_SFCALL")
        .iter()
        .find(|r| {
            r["FTYPE_ID"].as_i64() == Some(name_id)
                && r["FELEM_ID"].as_i64() == Some(-1)
                && r["SFUNC_ID"].as_i64() == Some(free_sfunc)
        })
        .expect("new standardize call element");
    // Per-scope next order.
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(scope_max + 1));
}

// ============================================================================
// E3: add_comparison_threshold — CFRTN tier reuse / next / reject
// ============================================================================

#[test]
fn e3_comparison_threshold_reuses_all_features_tier_order() {
    let config = template();
    let before = parse(&config);

    // ID_COMP (cfunc 9) ships a FULL_SCORE all-features (FTYPE_ID=0) tier row.
    let tier_order = rows(&before, "CFG_CFRTN")
        .iter()
        .find(|r| {
            r["CFUNC_ID"].as_i64() == Some(9)
                && r["FTYPE_ID"].as_i64() == Some(0)
                && r["CFUNC_RTNVAL"].as_str() == Some("FULL_SCORE")
        })
        .and_then(|r| r["EXEC_ORDER"].as_i64())
        .expect("ID_COMP FULL_SCORE tier row in template");

    // NAME has no ID_COMP/FULL_SCORE row yet. Request a bogus order 99 — tier
    // reuse must win and the new row must carry the tier's order, NOT 99.
    let mut params = AddComparisonThresholdParams::new("ID_COMP", "NAME", "FULL_SCORE");
    params.exec_order = Some(99);
    let modified = add_comparison_threshold(&config, params).unwrap();
    let after = parse(&modified);

    let name_id = feature_id(&after, "NAME");
    let new_row = rows(&after, "CFG_CFRTN")
        .iter()
        .find(|r| {
            r["CFUNC_ID"].as_i64() == Some(9)
                && r["FTYPE_ID"].as_i64() == Some(name_id)
                && r["CFUNC_RTNVAL"].as_str() == Some("FULL_SCORE")
        })
        .expect("new ID_COMP/NAME/FULL_SCORE row");
    assert_eq!(
        new_row["EXEC_ORDER"].as_i64(),
        Some(tier_order),
        "tier reuse must take precedence over the requested order"
    );
}

#[test]
fn e3_comparison_threshold_next_available_when_no_tier() {
    let config = template();
    let before = parse(&config);

    // A brand-new return value for ID_COMP at the all-features level: no tier
    // row exists, so the next free order within (CFUNC_ID 9, FTYPE_ID 0) is used.
    let scope_max = rows(&before, "CFG_CFRTN")
        .iter()
        .filter(|r| r["CFUNC_ID"].as_i64() == Some(9) && r["FTYPE_ID"].as_i64() == Some(0))
        .filter_map(|r| r["EXEC_ORDER"].as_i64())
        .max()
        .unwrap();

    let params = AddComparisonThresholdParams::new("ID_COMP", "all", "NEW_RTNVAL_X");
    let modified = add_comparison_threshold(&config, params).unwrap();
    let after = parse(&modified);

    let new_row = rows(&after, "CFG_CFRTN")
        .iter()
        .find(|r| {
            r["CFUNC_ID"].as_i64() == Some(9)
                && r["FTYPE_ID"].as_i64() == Some(0)
                && r["CFUNC_RTNVAL"].as_str() == Some("NEW_RTNVAL_X")
        })
        .expect("new ID_COMP/all/NEW_RTNVAL_X row");
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(scope_max + 1));
}

#[test]
fn e3_comparison_threshold_rejects_taken_explicit_order() {
    let config = template();
    let before = parse(&config);
    // A used order within (CFUNC_ID 9, FTYPE_ID 0).
    let taken = rows(&before, "CFG_CFRTN")
        .iter()
        .find(|r| r["CFUNC_ID"].as_i64() == Some(9) && r["FTYPE_ID"].as_i64() == Some(0))
        .and_then(|r| r["EXEC_ORDER"].as_i64())
        .unwrap();

    // New return value (no tier), explicit already-taken order -> AlreadyExists.
    let mut params = AddComparisonThresholdParams::new("ID_COMP", "all", "NEW_RTNVAL_Y");
    params.exec_order = Some(taken);
    let err = add_comparison_threshold(&config, params).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

// ============================================================================
// D1/D2/D3: BOM call-element allocation (per call)
// ============================================================================

/// Return (call_id, ftype_id, max_order, a free element id) for the first call
/// in `call_section` and its BOM in `bom_section`.
fn first_call_probe(
    v: &Value,
    call_section: &str,
    call_id_field: &str,
    bom_section: &str,
) -> (i64, i64, i64, i64) {
    let call = &rows(v, call_section)[0];
    let call_id = call[call_id_field].as_i64().unwrap();
    let ftype_id = call["FTYPE_ID"].as_i64().unwrap_or(1);
    let bom: Vec<&Value> = rows(v, bom_section)
        .iter()
        .filter(|b| b[call_id_field].as_i64() == Some(call_id))
        .collect();
    let max_order = bom
        .iter()
        .filter_map(|b| b["EXEC_ORDER"].as_i64())
        .max()
        .unwrap();
    let used_felems: Vec<i64> = bom.iter().filter_map(|b| b["FELEM_ID"].as_i64()).collect();
    let free_felem = rows(v, "CFG_FELEM")
        .iter()
        .filter_map(|r| r["FELEM_ID"].as_i64())
        .find(|id| !used_felems.contains(id))
        .expect("a free element for the call");
    (call_id, ftype_id, max_order, free_felem)
}

#[test]
fn d1_comparison_call_element_allocates_per_call() {
    let config = template();
    let before = parse(&config);
    let (call_id, ftype_id, max_order, free_felem) =
        first_call_probe(&before, "CFG_CFCALL", "CFCALL_ID", "CFG_CFBOM");

    // None -> max+1.
    let (modified, _) = add_comparison_call_element(
        &config,
        AddComparisonCallElementParams {
            cfcall_id: call_id,
            ftype_id,
            felem_id: free_felem,
            exec_order: None,
        },
    )
    .unwrap();
    let after = parse(&modified);
    let new_row = rows(&after, "CFG_CFBOM")
        .iter()
        .find(|b| {
            b["CFCALL_ID"].as_i64() == Some(call_id) && b["FELEM_ID"].as_i64() == Some(free_felem)
        })
        .unwrap();
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(max_order + 1));

    // Explicit free order honoured.
    let (modified2, _) = add_comparison_call_element(
        &config,
        AddComparisonCallElementParams {
            cfcall_id: call_id,
            ftype_id,
            felem_id: free_felem,
            exec_order: Some(max_order + 5),
        },
    )
    .unwrap();
    let after2 = parse(&modified2);
    let honoured = rows(&after2, "CFG_CFBOM")
        .iter()
        .find(|b| {
            b["CFCALL_ID"].as_i64() == Some(call_id) && b["FELEM_ID"].as_i64() == Some(free_felem)
        })
        .unwrap();
    assert_eq!(honoured["EXEC_ORDER"].as_i64(), Some(max_order + 5));

    // Explicit taken order rejected (order 1 exists on the call).
    let err = add_comparison_call_element(
        &config,
        AddComparisonCallElementParams {
            cfcall_id: call_id,
            ftype_id,
            felem_id: free_felem,
            exec_order: Some(1),
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

#[test]
fn d2_expression_call_element_allocates_per_call() {
    let config = template();
    let before = parse(&config);
    let (call_id, ftype_id, max_order, free_felem) =
        first_call_probe(&before, "CFG_EFCALL", "EFCALL_ID", "CFG_EFBOM");

    let (modified, _) = add_expression_call_element(
        &config,
        call_id,
        ExpressionCallElementParams::new(ftype_id, free_felem, None, "No".to_string()),
    )
    .unwrap();
    let after = parse(&modified);
    let new_row = rows(&after, "CFG_EFBOM")
        .iter()
        .find(|b| {
            b["EFCALL_ID"].as_i64() == Some(call_id) && b["FELEM_ID"].as_i64() == Some(free_felem)
        })
        .unwrap();
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(max_order + 1));

    // Explicit taken order rejected.
    let err = add_expression_call_element(
        &config,
        call_id,
        ExpressionCallElementParams::new(ftype_id, free_felem, Some(1), "No".to_string()),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

#[test]
fn d3_distinct_call_element_allocates_per_call_and_dup_check_realigned() {
    let config = template();
    let before = parse(&config);
    let (call_id, ftype_id, max_order, free_felem) =
        first_call_probe(&before, "CFG_DFCALL", "DFCALL_ID", "CFG_DFBOM");

    // None -> max+1.
    let (modified, _) = add_distinct_call_element(
        &config,
        AddDistinctCallElementParams {
            dfcall_id: call_id,
            ftype_id,
            felem_id: free_felem,
            exec_order: None,
        },
    )
    .unwrap();
    let after = parse(&modified);
    let new_row = rows(&after, "CFG_DFBOM")
        .iter()
        .find(|b| {
            b["DFCALL_ID"].as_i64() == Some(call_id) && b["FELEM_ID"].as_i64() == Some(free_felem)
        })
        .unwrap();
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(max_order + 1));

    // Dup-check realignment: an element already on the call is a duplicate even
    // when a different exec_order is requested (EXEC_ORDER no longer part of the
    // dup identity). Reuse an element that IS already on the call.
    let existing_felem = rows(&before, "CFG_DFBOM")
        .iter()
        .find(|b| b["DFCALL_ID"].as_i64() == Some(call_id))
        .and_then(|b| b["FELEM_ID"].as_i64())
        .unwrap();
    let err = add_distinct_call_element(
        &config,
        AddDistinctCallElementParams {
            dfcall_id: call_id,
            ftype_id,
            felem_id: existing_felem,
            exec_order: Some(9999), // different order, still a dup
        },
    )
    .unwrap_err();
    // Step D: a duplicate element on the call is the benign "already present"
    // sub-case, distinct from a taken exec-order (which stays AlreadyExists).
    assert_eq!(err.kind(), SzErrorKind::AlreadyPresent);
}

// ============================================================================
// command_processor addComparisonCallElement dispatch (per-call order)
// ============================================================================

#[test]
fn command_processor_add_comparison_call_element_allocates_per_call() {
    let config = template();
    let before = parse(&config);

    // NATIONAL_ID's comparison call and its current max BOM order.
    let nat_id = feature_id(&before, "NATIONAL_ID");
    let cfcall_id = rows(&before, "CFG_CFCALL")
        .iter()
        .find(|c| c["FTYPE_ID"].as_i64() == Some(nat_id))
        .and_then(|c| c["CFCALL_ID"].as_i64())
        .expect("NATIONAL_ID comparison call");
    let max_order = rows(&before, "CFG_CFBOM")
        .iter()
        .filter(|b| b["CFCALL_ID"].as_i64() == Some(cfcall_id))
        .filter_map(|b| b["EXEC_ORDER"].as_i64())
        .max()
        .unwrap();

    // GENDER exists but is not on NATIONAL_ID's comparison call.
    let script = r#"addComparisonCallElement {"feature": "NATIONAL_ID", "element": "GENDER"}"#;
    let mut processor = CommandProcessor::new(config);
    let result = processor.process_script(script).expect("dispatch ok");
    let after = parse(&result);

    let gender_id = element_id(&after, "GENDER");
    let new_row = rows(&after, "CFG_CFBOM")
        .iter()
        .find(|b| {
            b["CFCALL_ID"].as_i64() == Some(cfcall_id) && b["FELEM_ID"].as_i64() == Some(gender_id)
        })
        .expect("GENDER added to NATIONAL_ID comparison call");
    // Per-call allocation -> max+1 (the old per-(call,ftype) calc would also give
    // max+1 here since all rows share the feature, but this pins the new path).
    assert_eq!(new_row["EXEC_ORDER"].as_i64(), Some(max_order + 1));
}
