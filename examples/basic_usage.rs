//! Basic usage example for sz_configtool_lib
//!
//! This example demonstrates basic operations using the library's actual API.

use serde_json::json;
use sz_configtool_lib::{datasources, helpers};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== sz_configtool_lib Basic Usage Example ===\n");

    // Start with minimal config
    let config = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": []
  }
}"#;

    println!("Initial configuration:");
    println!("{}\n", config);

    // Add a data source using the library API
    println!("Adding data source 'CUSTOMERS'...");

    let dsrc_config = json!({
        "DSRC_CODE": "CUSTOMERS",
        "DSRC_DESC": "Customer records"
    });

    let config = helpers::add_to_config_array(config, "CFG_DSRC", dsrc_config)?;

    println!("✓ Data source added\n");

    // List all data sources
    println!("Listing all data sources:");
    let sources = datasources::list_data_sources(&config)?;

    for source in &sources {
        println!(
            "  - {}: {} (ID: {})",
            source["DSRC_CODE"], source["DSRC_DESC"], source["DSRC_ID"]
        );
    }

    println!("\nFinal configuration:");
    println!("{}", config);

    println!("\n=== Example Complete ===");

    Ok(())
}
