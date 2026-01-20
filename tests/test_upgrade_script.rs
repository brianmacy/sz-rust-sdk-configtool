//! Integration test for command script processor using real upgrade script commands
//!
//! Tests the 3 newly implemented commands that were previously NotImplemented:
//! - deleteComparisonCallElement
//! - addComparisonCallElement
//! - deleteDistinctCallElement

use sz_configtool_lib::command_processor::CommandProcessor;

const TEST_CONFIG_WITH_CALLS: &str = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [
      {
        "FTYPE_ID": 1,
        "FTYPE_CODE": "ADDRESS",
        "FTYPE_DESC": "Address",
        "FCLASS_ID": 1,
        "FTYPE_FREQ": "FM",
        "FTYPE_EXCL": "No",
        "FTYPE_STAB": "No"
      },
      {
        "FTYPE_ID": 2,
        "FTYPE_CODE": "NATIONAL_ID",
        "FTYPE_DESC": "National ID",
        "FCLASS_ID": 2,
        "FTYPE_FREQ": "F1",
        "FTYPE_EXCL": "Yes",
        "FTYPE_STAB": "No"
      }
    ],
    "CFG_FELEM": [
      {"FELEM_ID": 1, "FELEM_CODE": "PLACEKEY", "FELEM_DESC": "Placekey", "DATA_TYPE": "string"},
      {"FELEM_ID": 2, "FELEM_CODE": "ID_TYPE", "FELEM_DESC": "ID Type", "DATA_TYPE": "string"}
    ],
    "CFG_CFUNC": [
      {"CFUNC_ID": 1, "CFUNC_CODE": "ADDR_COMP", "CONNECT_STR": "g2AddrComp"}
    ],
    "CFG_DFUNC": [
      {"DFUNC_ID": 1, "DFUNC_CODE": "ADDR_DIST", "CONNECT_STR": "g2AddrDist"}
    ],
    "CFG_CFCALL": [
      {"CFCALL_ID": 1, "CFUNC_ID": 1, "FTYPE_ID": 1},
      {"CFCALL_ID": 2, "CFUNC_ID": 1, "FTYPE_ID": 2}
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
    "CFG_FCLASS": [
      {"FCLASS_ID": 1, "FCLASS_CODE": "ADDRESS"},
      {"FCLASS_ID": 2, "FCLASS_CODE": "IDENTIFIER"}
    ],
    "CONFIG_BASE_VERSION": {
      "VERSION": "4.0.0",
      "COMPATIBILITY_VERSION": {"CONFIG_VERSION": "10"}
    }
  }
}"#;

#[test]
fn test_delete_comparison_call_element_command() {
    // Test deleteComparisonCallElement command from upgrade script
    let script = r#"
deleteComparisonCallElement {"feature": "ADDRESS", "element": "PLACEKEY"}
"#;

    let mut processor = CommandProcessor::new(TEST_CONFIG_WITH_CALLS.to_string());
    let result = processor.process_script(script);

    assert!(
        result.is_ok(),
        "Failed to process script: {:?}",
        result.err()
    );

    let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

    // Verify PLACEKEY was removed from CFG_CFBOM for ADDRESS feature
    let cfbom = config["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();
    let has_placekey = cfbom
        .iter()
        .any(|bom| bom["FTYPE_ID"].as_i64() == Some(1) && bom["FELEM_ID"].as_i64() == Some(1));

    assert!(
        !has_placekey,
        "PLACEKEY should have been removed from CFG_CFBOM"
    );
}

#[test]
fn test_add_comparison_call_element_command() {
    // Test addComparisonCallElement command from upgrade script
    let script = r#"
addComparisonCallElement {"feature": "NATIONAL_ID", "element": "ID_TYPE"}
"#;

    let mut processor = CommandProcessor::new(TEST_CONFIG_WITH_CALLS.to_string());
    let result = processor.process_script(script);

    assert!(
        result.is_ok(),
        "Failed to process script: {:?}",
        result.err()
    );

    let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

    // Verify ID_TYPE was added to CFG_CFBOM for NATIONAL_ID feature
    let cfbom = config["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();
    let has_id_type = cfbom.iter().any(|bom| {
        bom["CFCALL_ID"].as_i64() == Some(2) // NATIONAL_ID's call
            && bom["FELEM_ID"].as_i64() == Some(2) // ID_TYPE
    });

    assert!(has_id_type, "ID_TYPE should have been added to CFG_CFBOM");
}

#[test]
fn test_delete_distinct_call_element_command() {
    // Test deleteDistinctCallElement command from upgrade script
    let script = r#"
deleteDistinctCallElement {"feature": "ADDRESS", "element": "PLACEKEY"}
"#;

    let mut processor = CommandProcessor::new(TEST_CONFIG_WITH_CALLS.to_string());
    let result = processor.process_script(script);

    assert!(
        result.is_ok(),
        "Failed to process script: {:?}",
        result.err()
    );

    let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

    // Verify PLACEKEY was removed from CFG_DFBOM for ADDRESS feature
    let dfbom = config["G2_CONFIG"]["CFG_DFBOM"].as_array().unwrap();
    let has_placekey = dfbom
        .iter()
        .any(|bom| bom["DFCALL_ID"].as_i64() == Some(1) && bom["FELEM_ID"].as_i64() == Some(1));

    assert!(
        !has_placekey,
        "PLACEKEY should have been removed from CFG_DFBOM"
    );
}

#[test]
fn test_upgrade_script_subset() {
    // Test a realistic subset of commands from the actual upgrade-10-to-11.gtc script
    let script = r#"
# Verify starting version
verifyCompatibilityVersion {"expectedVersion": "10"}

# Delete call element (newly implemented!)
deleteComparisonCallElement {"feature": "ADDRESS", "element": "PLACEKEY"}

# Add call element (newly implemented!)
addComparisonCallElement {"feature": "NATIONAL_ID", "element": "ID_TYPE"}

# Update compatibility version
updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}

save
"#;

    let mut processor = CommandProcessor::new(TEST_CONFIG_WITH_CALLS.to_string());
    let result = processor.process_script(script);

    assert!(
        result.is_ok(),
        "Failed to process upgrade script: {:?}",
        result.err()
    );

    // Verify version was upgraded
    let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert_eq!(
        config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"],
        "11"
    );

    // Verify operations
    let cfbom = config["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();

    // PLACEKEY (felem_id=1) should be deleted from ADDRESS (ftype_id=1)
    let has_placekey = cfbom
        .iter()
        .any(|bom| bom["FTYPE_ID"].as_i64() == Some(1) && bom["FELEM_ID"].as_i64() == Some(1));
    assert!(
        !has_placekey,
        "PLACEKEY should have been removed from ADDRESS"
    );

    // ID_TYPE (felem_id=2) should be added to NATIONAL_ID (ftype_id=2)
    let has_id_type = cfbom
        .iter()
        .any(|bom| bom["CFCALL_ID"].as_i64() == Some(2) && bom["FELEM_ID"].as_i64() == Some(2));
    assert!(has_id_type, "ID_TYPE should have been added to NATIONAL_ID");

    // Verify 4 commands executed
    assert_eq!(processor.get_executed_commands().len(), 4);

    println!("✓ {}", processor.summary());
}
