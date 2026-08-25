//! Error sub-case surface (v0.7.0 STEP D / #53 + #54) exercised against the REAL
//! Senzing v4 template (`tests/fixtures/g2config_template.json`), never
//! synthetic-only config. Two past delete-path regressions slipped through
//! synthetic tests, so the sub-case classification AND the "delete removes
//! exactly one row" invariant are proven here on the shipped data.
//!
//! Step D carves two benign single-call sub-cases out of the broader
//! `NotFound` / `AlreadyExists` families:
//!   - [`SzErrorKind::NotOnCall`] — a call-element delete found nothing to
//!     remove (the element is not a BOM row of the call).
//!   - [`SzErrorKind::AlreadyPresent`] — a call/call-element add is a no-op (the
//!     per-feature call is already set, or the element is already on the call).
//!
//! Hard collisions stay put: a taken explicit id and a taken exec-order remain
//! `AlreadyExists`, and a genuinely missing id/lookup remains `NotFound`. The
//! negative guards at the bottom prove those did NOT get reclassified.

use serde_json::Value;
use sz_configtool_lib::calls::CallSelector;
use sz_configtool_lib::calls::comparison::{
    AddComparisonCallElementParams, AddComparisonCallParams, add_comparison_call,
    add_comparison_call_element, delete_comparison_call_element, get_comparison_call,
};
use sz_configtool_lib::calls::distinct::{
    AddDistinctCallElementParams, add_distinct_call_element, delete_distinct_call_element,
};
use sz_configtool_lib::calls::expression::{
    ExpressionCallElementParams, add_expression_call_element, delete_expression_call_element,
};
use sz_configtool_lib::calls::standardize::{
    AddStandardizeCallElementParams, DeleteStandardizeCallElementParams,
    add_standardize_call_element, delete_standardize_call_element,
};
use sz_configtool_lib::error::SzErrorKind;

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
    v["G2_CONFIG"][section]
        .as_array()
        .unwrap_or_else(|| panic!("section {section} missing"))
}

fn felem_code(v: &Value, id: i64) -> String {
    v["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["FELEM_ID"].as_i64() == Some(id))
        .and_then(|r| r["FELEM_CODE"].as_str())
        .unwrap_or_else(|| panic!("no FELEM_CODE for id {id}"))
        .to_string()
}

/// A BOM row `(call_id, ftype_id, felem_id)` whose `(call_id, felem_id)` pair is
/// unique across the whole `bom_section` — so deleting it by `(call, element)`
/// with no feature disambiguator resolves to exactly one row (never ambiguous).
fn unique_bom_target(v: &Value, bom_section: &str, id_field: &str) -> (i64, i64, i64) {
    let bom = rows(v, bom_section);
    for row in bom {
        let call_id = row[id_field].as_i64().unwrap();
        let felem_id = row["FELEM_ID"].as_i64().unwrap();
        let ftype_id = row["FTYPE_ID"].as_i64().unwrap();
        let matches = bom
            .iter()
            .filter(|r| {
                r[id_field].as_i64() == Some(call_id) && r["FELEM_ID"].as_i64() == Some(felem_id)
            })
            .count();
        if matches == 1 {
            return (call_id, ftype_id, felem_id);
        }
    }
    panic!("no unique (call, felem) BOM row in {bom_section}");
}

/// A real feature (FTYPE_ID > 0) that has at least one `CFG_FBOM` member.
fn a_feature_with_fbom(v: &Value) -> (i64, String) {
    let fbom = rows(v, "CFG_FBOM");
    for f in v["G2_CONFIG"]["CFG_FTYPE"].as_array().unwrap() {
        let ftype_id = f["FTYPE_ID"].as_i64().unwrap();
        if ftype_id > 0
            && fbom
                .iter()
                .any(|r| r["FTYPE_ID"].as_i64() == Some(ftype_id))
        {
            return (ftype_id, f["FTYPE_CODE"].as_str().unwrap().to_string());
        }
    }
    panic!("no feature with a CFG_FBOM member");
}

/// A `felem_id` that exists in `CFG_FELEM` but is NOT a `CFG_FBOM` member of
/// `ftype_id` — i.e. "not an element of that feature", for the NotInFeature path.
fn felem_not_in_feature(v: &Value, ftype_id: i64) -> i64 {
    let members: Vec<i64> = rows(v, "CFG_FBOM")
        .iter()
        .filter(|r| r["FTYPE_ID"].as_i64() == Some(ftype_id))
        .filter_map(|r| r["FELEM_ID"].as_i64())
        .collect();
    v["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|r| r["FELEM_ID"].as_i64())
        .find(|id| *id >= 0 && !members.contains(id))
        .expect("a FELEM not in the chosen feature")
}

fn first_call_id(v: &Value, section: &str, id_field: &str) -> i64 {
    rows(v, section)[0][id_field].as_i64().unwrap()
}

/// A `felem_id` that exists in `CFG_FELEM` but is NOT a BOM row of `call_id`
/// within `bom_section` — the "not on this call" element used to drive delete.
fn felem_not_on_call(v: &Value, bom_section: &str, id_field: &str, call_id: i64) -> i64 {
    let bom = rows(v, bom_section);
    let on_call: Vec<i64> = bom
        .iter()
        .filter(|r| r[id_field].as_i64() == Some(call_id))
        .filter_map(|r| r["FELEM_ID"].as_i64())
        .collect();
    v["G2_CONFIG"]["CFG_FELEM"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|r| r["FELEM_ID"].as_i64())
        .find(|id| *id >= 0 && !on_call.contains(id))
        .expect("a FELEM not on the chosen call")
}

// ============================================================================
// NotOnCall: delete of an element that is NOT on the call, for the three
// BOM-backed families (comparison/expression/distinct). Standardize has no
// NotOnCall concept — see the missing-call NotFound test.
// ============================================================================

#[test]
fn delete_element_not_on_call_is_not_on_call_bom_families() {
    let config = template();
    let v = parse(&config);

    // --- comparison ---
    let (call_id, _, _) = unique_bom_target(&v, "CFG_CFBOM", "CFCALL_ID");
    let missing = felem_not_on_call(&v, "CFG_CFBOM", "CFCALL_ID", call_id);
    let code = felem_code(&v, missing);
    let err = delete_comparison_call_element(&config, CallSelector::Id(call_id), &code, None)
        .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotOnCall, "comparison");

    // --- expression ---
    let (call_id, _, _) = unique_bom_target(&v, "CFG_EFBOM", "EFCALL_ID");
    let missing = felem_not_on_call(&v, "CFG_EFBOM", "EFCALL_ID", call_id);
    let code = felem_code(&v, missing);
    let err = delete_expression_call_element(&config, CallSelector::Id(call_id), &code, None)
        .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotOnCall, "expression");

    // --- distinct ---
    let (call_id, _, _) = unique_bom_target(&v, "CFG_DFBOM", "DFCALL_ID");
    let missing = felem_not_on_call(&v, "CFG_DFBOM", "DFCALL_ID", call_id);
    let code = felem_code(&v, missing);
    let err =
        delete_distinct_call_element(&config, CallSelector::Id(call_id), &code, None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotOnCall, "distinct");

    // Standardize is deliberately NOT here: it has no BOM/two-tier split (the
    // SFCALL row IS the element) and no benign "not on call" concept — any miss
    // is a hard NotFound (see the missing-call test below).
}

// ============================================================================
// NotFound (NOT NotOnCall): delete against a call id that does not exist at all
// is a hard error, distinct from the benign "element not on an existing call".
// (The by-id delete path does not validate call existence via the resolver, so
// this guards the ensure_call_exists check that draws that line — Python's
// prepCallElement errors on a missing call record before touching the BOM.)
// ============================================================================

#[test]
fn delete_element_missing_call_id_is_not_found_all_families() {
    let config = template();
    let v = parse(&config);
    // A real element code so the failure is the call id, not the element lookup.
    let (_, _, felem_id) = unique_bom_target(&v, "CFG_CFBOM", "CFCALL_ID");
    let code = felem_code(&v, felem_id);
    const GHOST: i64 = 9_999_999;

    let err =
        delete_comparison_call_element(&config, CallSelector::Id(GHOST), &code, None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound, "comparison");

    let err =
        delete_expression_call_element(&config, CallSelector::Id(GHOST), &code, None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound, "expression");

    let err =
        delete_distinct_call_element(&config, CallSelector::Id(GHOST), &code, None).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound, "distinct");

    // Standardize: a delete that matches no SFCALL row (here a non-existent
    // element on a real (ftype, sfunc)) is a hard NotFound — Python's nearest
    // parity, deleteStandardizeCall, errors on a miss; there is no benign
    // "not on call" for the BOM-less standardize family.
    let sfcall = &rows(&v, "CFG_SFCALL")[0];
    let err = delete_standardize_call_element(
        &config,
        DeleteStandardizeCallElementParams {
            ftype_id: sfcall["FTYPE_ID"].as_i64().unwrap(),
            sfunc_id: sfcall["SFUNC_ID"].as_i64().unwrap(),
            felem_id: Some(GHOST),
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound, "standardize");
}

// ============================================================================
// NotInFeature (#58): when a call-element delete is addressed WITH an element
// feature, an element that is not a member of that feature is a hard error
// (Python "X is not an element of FEATURE"), distinct from the benign NotOnCall
// used when the element IS a feature member but simply not on the call.
// ============================================================================

#[test]
fn delete_element_not_in_feature_is_not_in_feature_all_families() {
    let config = template();
    let v = parse(&config);
    let (ftype_id, fcode) = a_feature_with_fbom(&v);
    let ecode = felem_code(&v, felem_not_in_feature(&v, ftype_id));

    // The element feature is supplied but the element is not one of its members,
    // so resolution fails before the call BOM is even consulted.
    let err = delete_comparison_call_element(
        &config,
        CallSelector::Id(first_call_id(&v, "CFG_CFCALL", "CFCALL_ID")),
        &ecode,
        Some(&fcode),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotInFeature, "comparison");
    assert_eq!(err.reason_code(), "NOT_IN_FEATURE");
    assert!(
        err.to_string().contains("is not an element of"),
        "message: {err}"
    );

    let err = delete_expression_call_element(
        &config,
        CallSelector::Id(first_call_id(&v, "CFG_EFCALL", "EFCALL_ID")),
        &ecode,
        Some(&fcode),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotInFeature, "expression");

    let err = delete_distinct_call_element(
        &config,
        CallSelector::Id(first_call_id(&v, "CFG_DFCALL", "DFCALL_ID")),
        &ecode,
        Some(&fcode),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotInFeature, "distinct");
}

// ============================================================================
// Delete-path regression guard: an element that IS on the call removes EXACTLY
// one BOM row (the two past regressions were over/under-deletion here).
// ============================================================================

#[test]
fn delete_element_on_call_removes_exactly_one_row_all_families() {
    let config = template();
    let v = parse(&config);

    // --- comparison ---
    let before = rows(&v, "CFG_CFBOM").len();
    let (call_id, _, felem_id) = unique_bom_target(&v, "CFG_CFBOM", "CFCALL_ID");
    let code = felem_code(&v, felem_id);
    let out =
        delete_comparison_call_element(&config, CallSelector::Id(call_id), &code, None).unwrap();
    assert_eq!(
        rows(&parse(&out), "CFG_CFBOM").len(),
        before - 1,
        "comparison must drop exactly one CFBOM row"
    );

    // --- expression ---
    let before = rows(&v, "CFG_EFBOM").len();
    let (call_id, _, felem_id) = unique_bom_target(&v, "CFG_EFBOM", "EFCALL_ID");
    let code = felem_code(&v, felem_id);
    let out =
        delete_expression_call_element(&config, CallSelector::Id(call_id), &code, None).unwrap();
    assert_eq!(
        rows(&parse(&out), "CFG_EFBOM").len(),
        before - 1,
        "expression must drop exactly one EFBOM row"
    );

    // --- distinct ---
    let before = rows(&v, "CFG_DFBOM").len();
    let (call_id, _, felem_id) = unique_bom_target(&v, "CFG_DFBOM", "DFCALL_ID");
    let code = felem_code(&v, felem_id);
    let out =
        delete_distinct_call_element(&config, CallSelector::Id(call_id), &code, None).unwrap();
    assert_eq!(
        rows(&parse(&out), "CFG_DFBOM").len(),
        before - 1,
        "distinct must drop exactly one DFBOM row"
    );

    // --- standardize: delete a real (ftype, sfunc, felem) triple, drop one row ---
    let before = rows(&v, "CFG_SFCALL").len();
    let sfcall = &rows(&v, "CFG_SFCALL")[0];
    let felem = sfcall["FELEM_ID"].as_i64().unwrap();
    let out = delete_standardize_call_element(
        &config,
        DeleteStandardizeCallElementParams {
            ftype_id: sfcall["FTYPE_ID"].as_i64().unwrap(),
            sfunc_id: sfcall["SFUNC_ID"].as_i64().unwrap(),
            felem_id: if felem == -1 { None } else { Some(felem) },
        },
    )
    .unwrap();
    assert_eq!(
        rows(&parse(&out), "CFG_SFCALL").len(),
        before - 1,
        "standardize must drop exactly one SFCALL row"
    );
}

// ============================================================================
// AlreadyPresent: adding an element already on the call, all four families.
// ============================================================================

#[test]
fn add_duplicate_element_is_already_present_all_families() {
    let config = template();
    let v = parse(&config);

    // --- comparison ---
    let (call_id, ftype_id, felem_id) = unique_bom_target(&v, "CFG_CFBOM", "CFCALL_ID");
    let err = add_comparison_call_element(
        &config,
        AddComparisonCallElementParams {
            cfcall_id: call_id,
            ftype_id,
            felem_id,
            exec_order: None,
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyPresent, "comparison");

    // --- expression ---
    let (call_id, ftype_id, felem_id) = unique_bom_target(&v, "CFG_EFBOM", "EFCALL_ID");
    let err = add_expression_call_element(
        &config,
        call_id,
        ExpressionCallElementParams::new(ftype_id, felem_id, None, "No".to_string()),
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyPresent, "expression");

    // --- distinct ---
    let (call_id, ftype_id, felem_id) = unique_bom_target(&v, "CFG_DFBOM", "DFCALL_ID");
    let err = add_distinct_call_element(
        &config,
        AddDistinctCallElementParams {
            dfcall_id: call_id,
            ftype_id,
            felem_id,
            exec_order: None,
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyPresent, "distinct");

    // --- standardize ---
    let sfcall = &rows(&v, "CFG_SFCALL")[0];
    let felem = sfcall["FELEM_ID"].as_i64().unwrap();
    let err = add_standardize_call_element(
        &config,
        AddStandardizeCallElementParams {
            ftype_id: sfcall["FTYPE_ID"].as_i64().unwrap(),
            sfunc_id: sfcall["SFUNC_ID"].as_i64().unwrap(),
            felem_id: if felem == -1 { None } else { Some(felem) },
            exec_order: None,
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyPresent, "standardize");
}

// ============================================================================
// AlreadyPresent: a per-feature call that is already set (comparison/distinct).
// ============================================================================

#[test]
fn add_call_for_feature_already_set_is_already_present() {
    let config = template();
    let v = parse(&config);

    // NAME (FTYPE_ID 1) already has a comparison call in the template. Adding a
    // second comparison call for the same feature is the benign "already set"
    // no-op -> AlreadyPresent (not AlreadyExists).
    assert!(
        rows(&v, "CFG_CFCALL")
            .iter()
            .any(|c| c["FTYPE_ID"].as_i64() == Some(1)),
        "precondition: NAME has a comparison call"
    );
    let err = add_comparison_call(
        &config,
        AddComparisonCallParams {
            ftype_code: "NAME".to_string(),
            cfunc_code: "GNR_COMP".to_string(),
            element_list: vec!["NAME_FULL".to_string()],
            id: None,
        },
    )
    .unwrap_err();
    assert_eq!(
        err.kind(),
        SzErrorKind::AlreadyPresent,
        "comparison already-set"
    );
}

// ============================================================================
// Negative guards: hard collisions were NOT reclassified by step D.
// ============================================================================

#[test]
fn taken_explicit_id_is_still_already_exists() {
    let config = template();
    let v = parse(&config);
    // An explicit, already-used CFCALL_ID is a hard collision. The id check runs
    // before the per-feature "already set" check, so this is AlreadyExists even
    // for a feature that also already has a call.
    let taken = rows(&v, "CFG_CFCALL")[0]["CFCALL_ID"].as_i64().unwrap();
    let err = add_comparison_call(
        &config,
        AddComparisonCallParams {
            ftype_code: "NAME".to_string(),
            cfunc_code: "GNR_COMP".to_string(),
            element_list: vec!["NAME_FULL".to_string()],
            id: Some(taken),
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}

#[test]
fn missing_call_id_is_still_not_found() {
    let config = template();
    // Addressing a call by an id that does not exist is a genuine lookup miss ->
    // NotFound (NOT the NotOnCall delete sub-case).
    let err = get_comparison_call(&config, CallSelector::Id(9_999_999)).unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::NotFound);
}

#[test]
fn taken_exec_order_is_still_already_exists() {
    let config = template();
    let v = parse(&config);

    // Pick a comparison call, a taken EXEC_ORDER on it, and a FREE element (not
    // yet on the call). The dup check passes (free element), so allocation runs
    // and rejects the taken order -> AlreadyExists, unchanged by step D.
    let (call_id, ftype_id, _) = unique_bom_target(&v, "CFG_CFBOM", "CFCALL_ID");
    let taken_order = rows(&v, "CFG_CFBOM")
        .iter()
        .find(|r| r["CFCALL_ID"].as_i64() == Some(call_id))
        .and_then(|r| r["EXEC_ORDER"].as_i64())
        .unwrap();
    let free_felem = felem_not_on_call(&v, "CFG_CFBOM", "CFCALL_ID", call_id);
    let err = add_comparison_call_element(
        &config,
        AddComparisonCallElementParams {
            cfcall_id: call_id,
            ftype_id,
            felem_id: free_felem,
            exec_order: Some(taken_order),
        },
    )
    .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::AlreadyExists);
}
