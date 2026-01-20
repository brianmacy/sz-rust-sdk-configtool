//! Tests for set_feature() with extended parameters (behavior, class, rtype_id)
//!
//! These tests verify the parameters that were documented in the Python tool
//! but missing from the initial Rust implementation.

use serde_json::json;
use sz_configtool_lib::features;

const TEST_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [
      {
        "FTYPE_ID": 1,
        "FTYPE_CODE": "TEST_FEATURE",
        "FTYPE_DESC": "Test Feature",
        "FCLASS_ID": 1,
        "FTYPE_FREQ": "FM",
        "FTYPE_EXCL": "No",
        "FTYPE_STAB": "No",
        "ANONYMIZE": "No",
        "DERIVED": "No",
        "USED_FOR_CAND": "No",
        "SHOW_IN_MATCH_KEY": "No",
        "PERSIST_HISTORY": "Yes",
        "VERSION": 1,
        "RTYPE_ID": 0
      }
    ],
    "CFG_FCLASS": [
      {
        "FCLASS_ID": 1,
        "FCLASS_CODE": "OTHER"
      },
      {
        "FCLASS_ID": 2,
        "FCLASS_CODE": "IDENTITY"
      }
    ]
  }
}"#;

#[test]
fn test_set_feature_behavior() {
    // Test changing behavior from FM to NAME
    let config = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None,    // candidates
        None,    // anonymize
        None,    // derived
        None,    // history
        None,    // matchkey
        Some("NAME"), // behavior
        None,    // class
        None,    // version
        None,    // rtype_id
    )
    .expect("Failed to set feature behavior");

    let feature = features::get_feature(&config, "TEST_FEATURE")
        .expect("Failed to get feature");

    assert_eq!(feature["behavior"], "NAME");
    // Verify internal fields
    let config_val: serde_json::Value = serde_json::from_str(&config).unwrap();
    let ftype = &config_val["G2_CONFIG"]["CFG_FTYPE"][0];
    assert_eq!(ftype["FTYPE_FREQ"], "NAME");
}

#[test]
fn test_set_feature_behavior_with_modifiers() {
    // Test behavior with exclusivity and stability (F1ES)
    let config = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None, None, None, None, None,
        Some("F1ES"), // behavior: F1 + Exclusive + Stable
        None, None, None,
    )
    .expect("Failed to set feature behavior F1ES");

    let config_val: serde_json::Value = serde_json::from_str(&config).unwrap();
    let ftype = &config_val["G2_CONFIG"]["CFG_FTYPE"][0];
    assert_eq!(ftype["FTYPE_FREQ"], "F1");
    assert_eq!(ftype["FTYPE_EXCL"], "Yes");
    assert_eq!(ftype["FTYPE_STAB"], "Yes");
}

#[test]
fn test_set_feature_class() {
    // Test changing class from OTHER to IDENTITY
    let config = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None, None, None, None, None,
        None,              // behavior
        Some("IDENTITY"),  // class
        None, None,
    )
    .expect("Failed to set feature class");

    let config_val: serde_json::Value = serde_json::from_str(&config).unwrap();
    let ftype = &config_val["G2_CONFIG"]["CFG_FTYPE"][0];
    assert_eq!(ftype["FCLASS_ID"], 2);

    // Verify via get_feature
    let feature = features::get_feature(&config, "TEST_FEATURE")
        .expect("Failed to get feature");
    assert_eq!(feature["class"], "IDENTITY");
}

#[test]
fn test_set_feature_rtype_id() {
    // Test setting RTYPE_ID
    let config = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None, None, None, None, None,
        None,   // behavior
        None,   // class
        None,   // version
        Some(5), // rtype_id
    )
    .expect("Failed to set feature rtype_id");

    let config_val: serde_json::Value = serde_json::from_str(&config).unwrap();
    let ftype = &config_val["G2_CONFIG"]["CFG_FTYPE"][0];
    assert_eq!(ftype["RTYPE_ID"], 5);
}

#[test]
fn test_set_feature_multiple_params() {
    // Test setting multiple parameters at once (real-world use case)
    let config = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        Some("Yes"),      // candidates
        None,             // anonymize
        None,             // derived
        None,             // history
        None,             // matchkey
        Some("NAME"),     // behavior - critical for embeddings!
        Some("IDENTITY"), // class
        Some(2),          // version
        None,             // rtype_id
    )
    .expect("Failed to set multiple feature parameters");

    let config_val: serde_json::Value = serde_json::from_str(&config).unwrap();
    let ftype = &config_val["G2_CONFIG"]["CFG_FTYPE"][0];

    // Verify all changes
    assert_eq!(ftype["USED_FOR_CAND"], "Yes");
    assert_eq!(ftype["FTYPE_FREQ"], "NAME");
    assert_eq!(ftype["FCLASS_ID"], 2);
    assert_eq!(ftype["VERSION"], 2);
}

#[test]
fn test_set_feature_invalid_class() {
    // Test error handling for non-existent class
    let result = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None, None, None, None, None,
        None,
        Some("NONEXISTENT_CLASS"),
        None, None,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Feature class"));
}

#[test]
fn test_set_feature_invalid_behavior() {
    // Test error handling for invalid behavior code
    let result = features::set_feature(
        TEST_CONFIG,
        "TEST_FEATURE",
        None, None, None, None, None,
        Some("INVALID_BEHAVIOR"),
        None, None, None,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("behavior"));
}
