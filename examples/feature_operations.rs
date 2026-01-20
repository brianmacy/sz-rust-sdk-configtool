//! Feature operations example
//!
//! Demonstrates working with features using the library's actual API.

use serde_json::json;
use sz_configtool_lib::{elements, features};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Feature Operations Example ===\n");

    // Start with config that has some elements
    let mut config = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [],
    "CFG_FELEM": [
      {
        "FELEM_ID": 1,
        "FELEM_CODE": "NAME",
        "FELEM_DESC": "Name",
        "DATA_TYPE": "string"
      },
      {
        "FELEM_ID": 2,
        "FELEM_CODE": "ADDRESS",
        "FELEM_DESC": "Address",
        "DATA_TYPE": "string"
      },
      {
        "FELEM_ID": 3,
        "FELEM_CODE": "PHONE",
        "FELEM_DESC": "Phone Number",
        "DATA_TYPE": "string"
      }
    ]
  }
}"#
    .to_string();

    println!("Initial elements:");
    let elems = elements::list_elements(&config)?;
    for elem in &elems {
        println!("  • {}: {}", elem["FELEM_CODE"], elem["FELEM_DESC"]);
    }

    // Add a new element
    println!("\n1. Adding new element 'EMAIL'...");
    let email_config = json!({
        "FELEM_CODE": "EMAIL",
        "FELEM_DESC": "Email Address",
        "DATA_TYPE": "string"
    });

    config = elements::add_element(&config, "EMAIL", &email_config)?;
    println!("  ✓ Added element");

    // Create a feature with element list
    println!("\n2. Creating feature 'PERSON' with elements...");

    let element_list = json!([
        {"element": "NAME", "expressed": "No"},
        {"element": "ADDRESS", "expressed": "No"},
        {"element": "PHONE", "expressed": "Yes"}
    ]);

    config = features::add_feature(
        &config,
        "PERSON",
        &element_list,
        Some("IDENTITY"),
        Some("NAME"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;

    println!("  ✓ Created feature PERSON");

    // List all features
    println!("\n3. Listing all features:");
    let features_list = features::list_features(&config)?;

    for feature in &features_list {
        println!(
            "  • {} (ID: {}): {} - {}",
            feature["FTYPE_CODE"],
            feature["FTYPE_ID"],
            feature.get("FCLASS").and_then(|v| v.as_str()).unwrap_or(""),
            feature
                .get("FTYPE_FREQ")
                .and_then(|v| v.as_str())
                .unwrap_or("")
        );
    }

    // Get specific feature
    println!("\n4. Getting details for feature 'PERSON':");
    let person_feature = features::get_feature(&config, "PERSON")?;
    println!("  Code: {}", person_feature["FTYPE_CODE"]);
    println!("  ID: {}", person_feature["FTYPE_ID"]);

    // Delete a feature
    println!("\n5. Deleting PERSON feature:");
    config = features::delete_feature(&config, "PERSON")?;
    println!("  ✓ Feature deleted");

    // Final feature count
    let final_features = features::list_features(&config)?;
    println!(
        "\n6. Final state: {} features remaining",
        final_features.len()
    );

    println!("\n=== Example Complete ===");

    Ok(())
}
