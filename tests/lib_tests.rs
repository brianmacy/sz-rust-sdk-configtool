//! Integration tests for sz_configtool_lib
//!
//! These tests verify the library functions work correctly with real
//! configuration JSON documents.

use serde_json::json;
use sz_configtool_lib::{datasources, elements, helpers};

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

    // Add data source
    let dsrc_config = json!({
        "DSRC_CODE": "TEST_SOURCE",
        "DSRC_DESC": "Test data source"
    });

    let config = helpers::add_to_config_array(&config, "CFG_DSRC", dsrc_config)
        .expect("Failed to add data source");

    // List data sources
    let sources = datasources::list_data_sources(&config).expect("Failed to list data sources");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0]["DSRC_CODE"], "TEST_SOURCE");

    // Get data source
    let source =
        datasources::get_data_source(&config, "TEST_SOURCE").expect("Failed to get data source");
    assert_eq!(source["DSRC_CODE"], "TEST_SOURCE");

    // Delete data source
    let config = helpers::delete_from_config_array(&config, "CFG_DSRC", "DSRC_CODE", "TEST_SOURCE")
        .expect("Failed to delete data source");

    // Verify deleted
    let sources =
        datasources::list_data_sources(&config).expect("Failed to list data sources after delete");
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_element_workflow() {
    let config = TEST_CONFIG.to_string();

    // Add element
    let elem_config = json!({
        "FELEM_CODE": "TEST_ELEM",
        "FELEM_DESC": "Test Element",
        "DATA_TYPE": "string"
    });

    let config =
        elements::add_element(&config, "TEST_ELEM", &elem_config).expect("Failed to add element");

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
