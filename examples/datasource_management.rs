//! Data source management example
//!
//! Demonstrates data source operations using the library's actual API.

use serde_json::json;
use sz_configtool_lib::datasources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Data Source Management Example ===\n");

    // Start with empty config
    let mut config = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": []
  }
}"#
    .to_string();

    // CREATE: Add data sources using the API
    println!("1. CREATE - Adding data sources...");

    config = datasources::add_data_source(
        &config,
        "CUSTOMERS",
        datasources::AddDataSourceParams::default(),
    )?;
    println!("  ✓ Added CUSTOMERS");

    config = datasources::add_data_source(
        &config,
        "VENDORS",
        datasources::AddDataSourceParams::default(),
    )?;
    println!("  ✓ Added VENDORS");

    // READ: List all data sources - returns "id" and "dataSource" format
    println!("\n2. READ - Listing all data sources:");
    let sources = datasources::list_data_sources(&config)?;

    for source in &sources {
        println!("  • {} (ID: {})", source["dataSource"], source["id"]);
    }

    // READ: Get specific data source - returns raw format with DSRC_CODE, DSRC_ID, etc.
    println!("\n3. READ - Getting specific data source (VENDORS):");
    let vendor_source = datasources::get_data_source(&config, "VENDORS")?;
    println!("  Code: {}", vendor_source["DSRC_CODE"]);
    println!("  ID: {}", vendor_source["DSRC_ID"]);
    println!("  Description: {}", vendor_source["DSRC_DESC"]);

    // UPDATE: Modify data source
    println!("\n4. UPDATE - Updating CUSTOMERS description:");
    let updates = json!({
        "DSRC_DESC": "Updated: Customer records from Salesforce CRM"
    });

    config = datasources::set_data_source(&config, "CUSTOMERS", &updates)?;

    let updated_source = datasources::get_data_source(&config, "CUSTOMERS")?;
    println!("  New description: {}", updated_source["DSRC_DESC"]);

    // DELETE: Remove a data source
    println!("\n5. DELETE - Removing VENDORS:");
    config = datasources::delete_data_source(&config, "VENDORS")?;
    println!("  ✓ VENDORS deleted");

    // Verify deletion
    println!("\n6. VERIFY - Final data source list:");
    let final_sources = datasources::list_data_sources(&config)?;
    println!("  Total: {} data sources", final_sources.len());

    for source in &final_sources {
        println!("  • {} (ID: {})", source["dataSource"], source["id"]);
    }

    // Error handling example
    println!("\n7. ERROR HANDLING - Attempting to get deleted VENDORS:");
    match datasources::get_data_source(&config, "VENDORS") {
        Ok(_) => println!("  Unexpected: VENDORS still exists!"),
        Err(e) => println!("  Expected error: {}", e),
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
