//! Comprehensive integration test using real upgrade script commands
//!
//! This test processes a larger subset of the actual upgrade-10-to-11.gtc script
//! to validate end-to-end functionality.

use sz_configtool_lib::command_processor::CommandProcessor;

/// Minimal but complete config for testing upgrade commands
const MINIMAL_V10_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": [],
    "CFG_ATTR": [
      {"ATTR_ID": 1, "ATTR_CODE": "DSRC_ACTION", "ATTR_CLASS": "OTHER", "FTYPE_CODE": "DSRC_RECORD", "FELEM_CODE": "DSRC_ACTION", "FELEM_REQ": "No", "INTERNAL": "No"}
    ],
    "CFG_FTYPE": [
      {"FTYPE_ID": 1, "FTYPE_CODE": "ADDRESS", "FTYPE_DESC": "Address", "FCLASS_ID": 1, "FTYPE_FREQ": "FM", "FTYPE_EXCL": "No", "FTYPE_STAB": "No", "ANONYMIZE": "No", "USED_FOR_CAND": "Yes", "PERSIST_HISTORY": "Yes", "SHOW_IN_MATCH_KEY": "No", "VERSION": 1, "RTYPE_ID": 0},
      {"FTYPE_ID": 2, "FTYPE_CODE": "PHONE", "FTYPE_DESC": "Phone", "FCLASS_ID": 1, "FTYPE_FREQ": "FM", "FTYPE_EXCL": "No", "FTYPE_STAB": "No", "ANONYMIZE": "No", "USED_FOR_CAND": "Yes", "PERSIST_HISTORY": "Yes", "SHOW_IN_MATCH_KEY": "No", "VERSION": 1, "RTYPE_ID": 0},
      {"FTYPE_ID": 3, "FTYPE_CODE": "EMAIL", "FTYPE_DESC": "Email", "FCLASS_ID": 1, "FTYPE_FREQ": "FM", "FTYPE_EXCL": "No", "FTYPE_STAB": "No", "ANONYMIZE": "No", "USED_FOR_CAND": "Yes", "PERSIST_HISTORY": "Yes", "SHOW_IN_MATCH_KEY": "No", "VERSION": 1, "RTYPE_ID": 0}
    ],
    "CFG_FELEM": [
      {"FELEM_ID": 1, "FELEM_CODE": "PLACEKEY", "FELEM_DESC": "Placekey", "DATA_TYPE": "string", "TOKENIZE": "No"}
    ],
    "CFG_FCLASS": [
      {"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}
    ],
    "CFG_FBOVR": [],
    "CFG_ERFRAG": [
      {"ERFRAG_ID": 1, "ERFRAG_CODE": "SAME_NAME", "ERFRAG_SOURCE": "./FRAGMENT[./BT_SAME_NAME>0]"},
      {"ERFRAG_ID": 2, "ERFRAG_CODE": "TRUSTED_ID", "ERFRAG_SOURCE": "./FRAGMENT[./TRUSTED_ID>0]"}
    ],
    "CFG_CFUNC": [
      {"CFUNC_ID": 1, "CFUNC_CODE": "ADDR_COMP", "CONNECT_STR": "g2AddrComp"}
    ],
    "CFG_DFUNC": [
      {"DFUNC_ID": 1, "DFUNC_CODE": "ADDR_DIST", "CONNECT_STR": "g2AddrDist"}
    ],
    "CFG_EFUNC": [],
    "CFG_SFUNC": [],
    "CFG_CFCALL": [
      {"CFCALL_ID": 1, "CFUNC_ID": 1, "FTYPE_ID": 1}
    ],
    "CFG_DFCALL": [
      {"DFCALL_ID": 1, "DFUNC_ID": 1, "FTYPE_ID": 1}
    ],
    "CFG_CFBOM": [
      {"CFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 1, "EXEC_ORDER": 1}
    ],
    "CFG_DFBOM": [
      {"DFCALL_ID": 1, "FTYPE_ID": 1, "FELEM_ID": 1, "EXEC_ORDER": 1}
    ],
    "CFG_ERRULE": [],
    "CONFIG_BASE_VERSION": {
      "VERSION": "4.0.0",
      "BUILD_VERSION": "4.0.0.0",
      "BUILD_DATE": "2024-01-01",
      "COMPATIBILITY_VERSION": {
        "CONFIG_VERSION": "10"
      }
    }
  }
}"#;

#[test]
fn test_comprehensive_upgrade_simulation() {
    // Simulate key commands from the real upgrade-10-to-11.gtc script
    let script = r#"
# Verify we're starting from version 10
verifyCompatibilityVersion {"expectedVersion": "10"}

# Remove deprecated fields
removeConfigSectionField {"section": "CFG_DSRC", "field": "CONVERSATIONAL"}
removeConfigSectionField {"section": "CFG_FTYPE", "field": "DERIVATION"}

# Delete fragment that will be replaced
deleteFragment {"fragment": "TRUSTED_ID"}

# Delete attribute
deleteAttribute {"attribute": "DSRC_ACTION"}

# Delete call elements
deleteComparisonCallElement {"feature": "ADDRESS", "element": "PLACEKEY"}
deleteDistinctCallElement {"feature": "ADDRESS", "element": "PLACEKEY"}

# Add new elements for semantic similarity
addElement {"element": "EMBEDDING", "datatype": "string"}
addElement {"element": "ALGORITHM", "datatype": "string"}

# Update existing features
setFeature {"feature": "PHONE", "candidates": "No"}
setFeature {"feature": "EMAIL", "candidates": "No"}

# Add new comparison function
addComparisonFunction {"function": "SEMANTIC_SIMILARITY_COMP", "connectStr": "g2SemanticSimilarityComp", "anonSupport": "Yes"}

# Add attribute
addAttribute {"attribute": "SEMANTIC_EMBEDDING", "class": "IDENTIFIER", "feature": "ADDRESS", "element": "EMBEDDING", "required": "No", "internal": "No"}

# Update compatibility version
updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}

save
"#;

    let mut processor = CommandProcessor::new(MINIMAL_V10_CONFIG.to_string());
    let result = processor.process_script(script);

    assert!(
        result.is_ok(),
        "Failed to process comprehensive upgrade: {:?}",
        result.err()
    );

    let upgraded_config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

    // Verify version upgrade
    assert_eq!(
        upgraded_config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]
            ["CONFIG_VERSION"],
        "11",
        "Config version should be upgraded to 11"
    );

    // Verify fragment deletion
    let fragments = upgraded_config["G2_CONFIG"]["CFG_ERFRAG"].as_array().unwrap();
    let has_trusted_id = fragments
        .iter()
        .any(|f| f["ERFRAG_CODE"].as_str() == Some("TRUSTED_ID"));
    assert!(!has_trusted_id, "TRUSTED_ID fragment should be deleted");

    // Verify attribute deletion
    let attrs = upgraded_config["G2_CONFIG"]["CFG_ATTR"].as_array().unwrap();
    let has_dsrc_action = attrs
        .iter()
        .any(|a| a["ATTR_CODE"].as_str() == Some("DSRC_ACTION"));
    assert!(!has_dsrc_action, "DSRC_ACTION attribute should be deleted");

    // Verify new elements added
    let elements = upgraded_config["G2_CONFIG"]["CFG_FELEM"].as_array().unwrap();
    let has_embedding = elements
        .iter()
        .any(|e| e["FELEM_CODE"].as_str() == Some("EMBEDDING"));
    let has_algorithm = elements
        .iter()
        .any(|e| e["FELEM_CODE"].as_str() == Some("ALGORITHM"));
    assert!(has_embedding, "EMBEDDING element should be added");
    assert!(has_algorithm, "ALGORITHM element should be added");

    // Verify feature updates
    let features = upgraded_config["G2_CONFIG"]["CFG_FTYPE"].as_array().unwrap();
    let phone = features
        .iter()
        .find(|f| f["FTYPE_CODE"].as_str() == Some("PHONE"))
        .expect("PHONE feature should exist");
    assert_eq!(
        phone["USED_FOR_CAND"], "No",
        "PHONE candidates should be set to No"
    );

    let email = features
        .iter()
        .find(|f| f["FTYPE_CODE"].as_str() == Some("EMAIL"))
        .expect("EMAIL feature should exist");
    assert_eq!(
        email["USED_FOR_CAND"], "No",
        "EMAIL candidates should be set to No"
    );

    // Verify new comparison function added
    let cfuncs = upgraded_config["G2_CONFIG"]["CFG_CFUNC"].as_array().unwrap();
    let has_semantic = cfuncs
        .iter()
        .any(|f| f["CFUNC_CODE"].as_str() == Some("SEMANTIC_SIMILARITY_COMP"));
    assert!(has_semantic, "SEMANTIC_SIMILARITY_COMP function should be added");

    // Verify attribute added
    let semantic_attr = attrs
        .iter()
        .find(|a| a["ATTR_CODE"].as_str() == Some("SEMANTIC_EMBEDDING"));
    assert!(semantic_attr.is_some(), "SEMANTIC_EMBEDDING attribute should be added");

    // Verify call elements were removed
    let cfbom = upgraded_config["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();
    let has_placekey = cfbom.iter().any(|bom| {
        bom["FTYPE_ID"].as_i64() == Some(1) && bom["FELEM_ID"].as_i64() == Some(1)
    });
    assert!(!has_placekey, "PLACEKEY should be removed from CFG_CFBOM");

    // Verify command count
    let executed = processor.get_executed_commands();
    assert_eq!(
        executed.len(),
        14,
        "Should have executed 14 commands (excluding comments and save)"
    );

    println!("✓ Comprehensive upgrade test passed!");
    println!("✓ {}", processor.summary());
}

#[test]
fn test_command_processor_error_handling() {
    // Test that errors are properly reported with line numbers
    let bad_script = r#"
verifyCompatibilityVersion {"expectedVersion": "10"}
deleteAttribute {"attribute": "NONEXISTENT_ATTR"}
"#;

    let mut processor = CommandProcessor::new(MINIMAL_V10_CONFIG.to_string());
    let result = processor.process_script(bad_script);

    assert!(result.is_err(), "Should fail on non-existent attribute");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Line 3"),
        "Error should include line number"
    );
    assert!(
        error_msg.contains("NONEXISTENT_ATTR"),
        "Error should include attribute name"
    );
}

#[test]
fn test_command_processor_dry_run_mode() {
    // Test dry-run mode doesn't modify config
    let script = r#"
deleteAttribute {"attribute": "DSRC_ACTION"}
updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}
"#;

    let original_config = MINIMAL_V10_CONFIG.to_string();
    let mut processor = CommandProcessor::new(original_config.clone()).dry_run(true);
    let result = processor.process_script(script);

    assert!(result.is_ok(), "Dry run should succeed");

    // Config should be unchanged in dry-run mode
    let result_config = result.unwrap();
    let config_val: serde_json::Value = serde_json::from_str(&result_config).unwrap();

    // Version should still be 10
    assert_eq!(
        config_val["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"],
        "10",
        "Version should not change in dry-run mode"
    );

    // Attribute should still exist
    let attrs = config_val["G2_CONFIG"]["CFG_ATTR"].as_array().unwrap();
    let has_attr = attrs
        .iter()
        .any(|a| a["ATTR_CODE"].as_str() == Some("DSRC_ACTION"));
    assert!(has_attr, "Attribute should not be deleted in dry-run mode");

    // But commands should be tracked
    assert_eq!(
        processor.get_executed_commands().len(),
        2,
        "Should track executed commands even in dry-run"
    );
}
