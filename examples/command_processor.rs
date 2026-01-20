//! Command script processor example
//!
//! Demonstrates processing .gtc command scripts to transform configuration JSON.
//! This example shows how to process Senzing upgrade scripts programmatically.

use sz_configtool_lib::command_processor::CommandProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Command Script Processor Example ===\n");

    // Start with a basic config
    let initial_config = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": [],
    "CFG_ATTR": [],
    "CFG_FTYPE": [
      {
        "FTYPE_ID": 1,
        "FTYPE_CODE": "NAME",
        "FTYPE_DESC": "Name",
        "FCLASS_ID": 1,
        "FTYPE_FREQ": "FM",
        "FTYPE_EXCL": "No",
        "FTYPE_STAB": "No",
        "USED_FOR_CAND": "No",
        "VERSION": 1
      }
    ],
    "CFG_FELEM": [
      {
        "FELEM_ID": 1,
        "FELEM_CODE": "FULL_NAME",
        "FELEM_DESC": "Full Name",
        "DATA_TYPE": "string"
      }
    ],
    "CFG_FCLASS": [
      {"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"},
      {"FCLASS_ID": 2, "FCLASS_CODE": "IDENTITY"}
    ],
    "CFG_FBOVR": [],
    "CFG_ERFRAG": [],
    "CONFIG_BASE_VERSION": {
      "VERSION": "4.0.0",
      "COMPATIBILITY_VERSION": {
        "CONFIG_VERSION": "10"
      }
    }
  }
}"#;

    println!("Initial config version: 10\n");

    // Sample command script demonstrating various command types
    let script = r#"
# Verify we're starting from version 10
verifyCompatibilityVersion {"expectedVersion": "10"}

# Add new data source
addAttribute {"attribute": "TEST_ATTR", "class": "OTHER", "feature": "NAME", "element": "FULL_NAME", "required": "No", "internal": "No"}

# Add new elements for semantic similarity
addElement {"element": "EMBEDDING", "datatype": "string"}
addElement {"element": "ALGORITHM", "datatype": "string"}

# Update existing feature - enable for candidates and set behavior
setFeature {"feature": "NAME", "candidates": "Yes", "behavior": "NAME", "version": 2}

# Add behavior override for business usage
addBehaviorOverride {"feature": "NAME", "usageType": "BUSINESS", "behavior": "F1E"}

# Update compatibility version
updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}

save
"#;

    println!("Processing command script...\n");

    // Create processor and execute script
    let mut processor = CommandProcessor::new(initial_config.to_string());
    let upgraded_config = processor.process_script(script)?;

    println!("✓ {}\n", processor.summary());

    // Show executed commands
    println!("Commands executed:");
    for cmd in processor.get_executed_commands() {
        println!("  - {}", cmd);
    }

    // Verify results
    let config: serde_json::Value = serde_json::from_str(&upgraded_config)?;

    println!("\nResults:");
    println!(
        "  Config version: {}",
        config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"]
    );

    let name_feature = &config["G2_CONFIG"]["CFG_FTYPE"][0];
    println!("  NAME feature version: {}", name_feature["VERSION"]);
    println!("  NAME used for candidates: {}", name_feature["USED_FOR_CAND"]);

    let overrides = config["G2_CONFIG"]["CFG_FBOVR"].as_array().unwrap();
    println!("  Behavior overrides: {}", overrides.len());
    if !overrides.is_empty() {
        println!("    - Feature: NAME, Usage: BUSINESS, Behavior: F1E");
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
