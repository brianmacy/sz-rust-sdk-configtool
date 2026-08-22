//! Integration tests for sz_configtool_lib
//!
//! These tests verify the library functions work correctly with real
//! configuration JSON documents.

use serde_json::{Value, json};
use sz_configtool_lib::{datasources, elements, features, helpers};

const TEST_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": [],
    "CFG_ATTR": [],
    "CFG_FTYPE": [],
    "CFG_FELEM": []
  }
}"#;

#[test]
fn test_data_source_workflow() {
    let config = TEST_CONFIG.to_string();

    // Add data source using the proper API function
    let config = datasources::add_data_source(
        &config,
        datasources::AddDataSourceParams {
            code: "TEST_SOURCE",
            ..Default::default()
        },
    )
    .expect("Failed to add data source");

    // List data sources - returns transformed format with "id" and "dataSource" fields
    let sources = datasources::list_data_sources(&config).expect("Failed to list data sources");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0]["dataSource"], "TEST_SOURCE");

    // Get data source - returns raw format with DSRC_CODE
    let source =
        datasources::get_data_source(&config, "TEST_SOURCE").expect("Failed to get data source");
    assert_eq!(source["DSRC_CODE"], "TEST_SOURCE");

    // #37/D18: a fresh data source now seeds at DSRC_ID 1000 (was max+1), so it is
    // well clear of the system range (<= 2) and IS deletable.
    assert_eq!(source["DSRC_ID"], 1000);
    let config = datasources::delete_data_source(&config, "TEST_SOURCE")
        .expect("A user data source (id >= 1000) must be deletable");
    let sources = datasources::list_data_sources(&config).expect("Failed to list data sources");
    assert_eq!(sources.len(), 0);

    // A genuine system data source (DSRC_ID <= 2) is still protected.
    let sys_config = r#"{"G2_CONFIG": {"CFG_DSRC": [{"DSRC_ID": 1, "DSRC_CODE": "TEST"}]}}"#;
    let delete_result = datasources::delete_data_source(sys_config, "TEST");
    assert!(
        delete_result.is_err(),
        "Should fail to delete a system datasource (id <= 2)"
    );
    let err_msg = delete_result.unwrap_err().to_string();
    assert!(
        err_msg.contains("cannot be deleted") || err_msg.contains("system"),
        "Error should indicate system datasource protection, got: {err_msg}"
    );
}

#[test]
fn test_element_workflow() {
    let config = TEST_CONFIG.to_string();

    // Add element
    let add_params = elements::AddElementParams {
        code: "TEST_ELEM",
        description: Some("Test Element"),
        data_type: Some("string"),
        id: None,
    };

    let config = elements::add_element(&config, add_params).expect("Failed to add element");

    // List elements
    let elems = elements::list_elements(&config).expect("Failed to list elements");
    assert_eq!(elems.len(), 1);

    // Delete element
    let config = helpers::delete_from_config_array(&config, "CFG_FELEM", "FELEM_CODE", "TEST_ELEM")
        .expect("Failed to delete element");

    // Verify deleted
    let elems = elements::list_elements(&config).expect("Failed to list elements after delete");
    assert_eq!(elems.len(), 0);
}

#[test]
fn test_error_not_found() {
    let config = TEST_CONFIG.to_string();

    // Try to get non-existent data source
    let result = datasources::get_data_source(&config, "NONEXISTENT");
    assert!(result.is_err());
}

#[test]
fn test_json_parsing() {
    // Test with valid JSON
    let config = TEST_CONFIG.to_string();
    let result = datasources::list_data_sources(&config);
    assert!(result.is_ok());

    // Test with invalid JSON
    let invalid_config = "not valid json";
    let result = datasources::list_data_sources(invalid_config);
    assert!(result.is_err());
}

#[test]
fn test_chained_operations() {
    let config = TEST_CONFIG.to_string();

    // Chain multiple operations
    let dsrc1 = json!({"DSRC_CODE": "SOURCE1", "DSRC_DESC": "First source"});
    let config =
        helpers::add_to_config_array(&config, "CFG_DSRC", dsrc1).expect("Failed to add SOURCE1");

    let dsrc2 = json!({"DSRC_CODE": "SOURCE2", "DSRC_DESC": "Second source"});
    let config =
        helpers::add_to_config_array(&config, "CFG_DSRC", dsrc2).expect("Failed to add SOURCE2");

    let dsrc3 = json!({"DSRC_CODE": "SOURCE3", "DSRC_DESC": "Third source"});
    let config =
        helpers::add_to_config_array(&config, "CFG_DSRC", dsrc3).expect("Failed to add SOURCE3");

    // Verify all three exist
    let sources = datasources::list_data_sources(&config).expect("Failed to list data sources");
    assert_eq!(sources.len(), 3);

    // Delete middle one
    let config = helpers::delete_from_config_array(&config, "CFG_DSRC", "DSRC_CODE", "SOURCE2")
        .expect("Failed to delete SOURCE2");

    // Verify two remain
    let sources =
        datasources::list_data_sources(&config).expect("Failed to list data sources after delete");
    assert_eq!(sources.len(), 2);
}

/// Config with all sections needed by add_feature
const FEATURE_TEST_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [],
    "CFG_FELEM": [],
    "CFG_FCLASS": [
      {"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}
    ],
    "CFG_FBOM": [],
    "CFG_SFCALL": [],
    "CFG_EFCALL": [],
    "CFG_CFCALL": [],
    "CFG_EFBOM": [],
    "CFG_CFBOM": [],
    "CFG_ATTR": [],
    "CFG_DSRC": []
  }
}"#;

#[test]
fn test_add_feature_fbom_display_delim_present_when_none() {
    // Regression: add_feature must always include DISPLAY_DELIM in CFG_FBOM,
    // even when no display_delim is provided. Senzing engine requires the field
    // to exist (as null) rather than be omitted entirely.
    let elements = json!([{"element": "ELEM1"}]);
    let config = features::add_feature(
        FEATURE_TEST_CONFIG,
        features::AddFeatureParams {
            feature: "TEST_FEAT",
            element_list: &elements,
            ..Default::default()
        },
    )
    .expect("Failed to add feature");

    let parsed: Value = serde_json::from_str(&config).unwrap();
    let fbom_array = parsed["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .expect("CFG_FBOM should be an array");
    assert_eq!(fbom_array.len(), 1);

    let fbom = &fbom_array[0];
    // The field must exist (not be absent from the object)
    assert!(
        fbom.as_object().unwrap().contains_key("DISPLAY_DELIM"),
        "DISPLAY_DELIM field must be present in CFG_FBOM even when not specified"
    );
    // When not specified, it should be null
    assert!(
        fbom["DISPLAY_DELIM"].is_null(),
        "DISPLAY_DELIM should be null when not specified, got: {}",
        fbom["DISPLAY_DELIM"]
    );
}

#[test]
fn test_add_feature_fbom_display_delim_present_when_set() {
    // Verify DISPLAY_DELIM is set correctly when explicitly provided
    let elements = json!([{"element": "ELEM1", "display_delim": " "}]);
    let config = features::add_feature(
        FEATURE_TEST_CONFIG,
        features::AddFeatureParams {
            feature: "TEST_FEAT",
            element_list: &elements,
            ..Default::default()
        },
    )
    .expect("Failed to add feature");

    let parsed: Value = serde_json::from_str(&config).unwrap();
    let fbom = &parsed["G2_CONFIG"]["CFG_FBOM"][0];
    assert_eq!(
        fbom["DISPLAY_DELIM"], " ",
        "DISPLAY_DELIM should be the provided value"
    );
}

#[test]
fn test_add_feature_fbom_display_delim_multiple_elements() {
    // Test with multiple elements: one with display_delim, one without
    let elements = json!([
        {"element": "FIRST_NAME", "display_delim": " "},
        {"element": "LAST_NAME"}
    ]);
    let config = features::add_feature(
        FEATURE_TEST_CONFIG,
        features::AddFeatureParams {
            feature: "FULL_NAME",
            element_list: &elements,
            ..Default::default()
        },
    )
    .expect("Failed to add feature");

    let parsed: Value = serde_json::from_str(&config).unwrap();
    let fbom_array = parsed["G2_CONFIG"]["CFG_FBOM"]
        .as_array()
        .expect("CFG_FBOM should be an array");
    assert_eq!(fbom_array.len(), 2);

    // First element: has explicit display_delim
    assert_eq!(fbom_array[0]["DISPLAY_DELIM"], " ");

    // Second element: no display_delim provided - must still have the field as null
    let second = fbom_array[1].as_object().unwrap();
    assert!(
        second.contains_key("DISPLAY_DELIM"),
        "DISPLAY_DELIM must be present for all FBOM records"
    );
    assert!(
        fbom_array[1]["DISPLAY_DELIM"].is_null(),
        "DISPLAY_DELIM should be null for element without display_delim"
    );
}
