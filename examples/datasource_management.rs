//! Data source management example
//!
//! Demonstrates data source operations using the library's actual API.

use serde_json::json;
use sz_configtool_lib::{datasources, helpers};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Data Source Management Example ===\n");

    // Start with empty config
    let mut config = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": []
  }
}"#
    .to_string();

    // CREATE: Add data sources
    println!("1. CREATE - Adding data sources...");

    let dsrc1 = json!({
        "DSRC_CODE": "CUSTOMERS",
        "DSRC_DESC": "Customer records from CRM"
    });

    config = helpers::add_to_config_array(&config, "CFG_DSRC", dsrc1)?;
    println!("  ✓ Added CUSTOMERS");

    let dsrc2 = json!({
        "DSRC_CODE": "VENDORS",
        "DSRC_DESC": "Vendor records from ERP"
    });

    config = helpers::add_to_config_array(&config, "CFG_DSRC", dsrc2)?;
    println!("  ✓ Added VENDORS");

    // READ: List all data sources
    println!("\n2. READ - Listing all data sources:");
    let sources = datasources::list_data_sources(&config)?;

    for source in &sources {
        println!(
            "  • {} (ID: {}): {}",
            source["DSRC_CODE"], source["DSRC_ID"], source["DSRC_DESC"]
        );
    }

    // READ: Get specific data source
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
    config = helpers::delete_from_config_array(&config, "CFG_DSRC", "DSRC_CODE", "VENDORS")?;
    println!("  ✓ VENDORS deleted");

    // Verify deletion
    println!("\n6. VERIFY - Final data source list:");
    let final_sources = datasources::list_data_sources(&config)?;
    println!("  Total: {} data sources", final_sources.len());

    for source in &final_sources {
        println!(
            "  • {} (ID: {}): {}",
            source["DSRC_CODE"], source["DSRC_ID"], source["DSRC_DESC"]
        );
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
