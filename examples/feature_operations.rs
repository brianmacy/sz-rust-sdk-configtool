//! Feature operations example
//!
//! Demonstrates working with elements using the library's actual API.
//! Feature creation requires a more complete config with CFG_FCLASS, so this
//! example focuses on element operations which are simpler.

use sz_configtool_lib::elements;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Element Operations Example ===\n");

    // Start with config that has some elements
    let mut config = r#"{
  "G2_CONFIG": {
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

    // List initial elements
    println!("Initial elements:");
    let elems = elements::list_elements(&config)?;
    for elem in &elems {
        println!(
            "  - {} (ID: {}): {}",
            elem["element"], elem["id"], elem["datatype"]
        );
    }

    // Add a new element
    println!("\n1. Adding new element 'EMAIL'...");
    let add_params = elements::AddElementParams {
        code: "EMAIL",
        description: Some("Email Address"),
        data_type: Some("string"),
        tokenized: None,
    };

    config = elements::add_element(&config, add_params)?;
    println!("  ✓ Added element");

    // List elements after addition
    println!("\n2. Elements after adding EMAIL:");
    let elems = elements::list_elements(&config)?;
    for elem in &elems {
        println!(
            "  - {} (ID: {}): {}",
            elem["element"], elem["id"], elem["datatype"]
        );
    }

    // Get a specific element
    println!("\n3. Getting EMAIL element details:");
    let email = elements::get_element(&config, "EMAIL")?;
    println!("  Code: {}", email["FELEM_CODE"]);
    println!("  ID: {}", email["FELEM_ID"]);
    println!("  Description: {}", email["FELEM_DESC"]);
    println!("  Data Type: {}", email["DATA_TYPE"]);

    // Delete an element
    println!("\n4. Deleting PHONE element:");
    config = elements::delete_element(&config, "PHONE")?;
    println!("  ✓ Element deleted");

    // Final element list
    println!("\n5. Final elements:");
    let final_elems = elements::list_elements(&config)?;
    println!("  Total: {} elements", final_elems.len());
    for elem in &final_elems {
        println!(
            "  - {} (ID: {}): {}",
            elem["element"], elem["id"], elem["datatype"]
        );
    }

    // Error handling example
    println!("\n6. ERROR HANDLING - Attempting to get deleted PHONE:");
    match elements::get_element(&config, "PHONE") {
        Ok(_) => println!("  Unexpected: PHONE still exists!"),
        Err(e) => println!("  Expected error: {}", e),
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
