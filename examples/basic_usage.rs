//! Basic usage example for sz_configtool_lib
//!
//! This example demonstrates basic operations using the library's actual API.

use sz_configtool_lib::datasources;

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

    // Add a data source using the library's add_data_source API
    println!("Adding data source 'CUSTOMERS'...");

    let config = datasources::add_data_source(config, "CUSTOMERS", None, None, None)?;

    println!("✓ Data source added\n");

    // List all data sources - returns transformed format with "id" and "dataSource" fields
    println!("Listing all data sources:");
    let sources = datasources::list_data_sources(&config)?;

    for source in &sources {
        println!("  - {} (ID: {})", source["dataSource"], source["id"]);
    }

    // Get a specific data source - returns raw format with DSRC_CODE, DSRC_ID, etc.
    println!("\nGetting data source details:");
    let customer = datasources::get_data_source(&config, "CUSTOMERS")?;
    println!("  Code: {}", customer["DSRC_CODE"]);
    println!("  Description: {}", customer["DSRC_DESC"]);

    println!("\nFinal configuration:");
    println!("{}", config);

    println!("\n=== Example Complete ===");

    Ok(())
}
