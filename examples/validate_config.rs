//! Config validation example
//!
//! Demonstrates validating Senzing configuration JSON structure.
//! Checks for required sections and basic structural integrity.

use serde_json::Value;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Senzing Config Validator ===\n");

    let args: Vec<String> = env::args().collect();
    let config_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("g2config.json");

    println!("Loading: {}", config_path);
    let config_json = match std::fs::read_to_string(config_path) {
        Ok(c) => {
            println!("  ✓ File loaded ({} bytes)\n", c.len());
            c
        }
        Err(e) => {
            eprintln!("  ✗ Failed to load: {}", e);
            eprintln!("\nUsage: {} [config.json]", args[0]);
            std::process::exit(1);
        }
    };

    // Parse JSON
    println!("Parsing JSON...");
    let config: Value = match serde_json::from_str(&config_json) {
        Ok(v) => {
            println!("  ✓ Valid JSON\n");
            v
        }
        Err(e) => {
            eprintln!("  ✗ Invalid JSON: {}", e);
            std::process::exit(1);
        }
    };

    // Validate structure
    println!("Validating structure...");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check G2_CONFIG exists
    if config.get("G2_CONFIG").is_none() {
        errors.push("Missing required section: G2_CONFIG".to_string());
    } else {
        let g2_config = &config["G2_CONFIG"];

        // Required sections
        let required_sections = vec![
            "CFG_ATTR",
            "CFG_DSRC",
            "CFG_FTYPE",
            "CFG_FELEM",
            "CFG_FCLASS",
            "CONFIG_BASE_VERSION",
        ];

        for section in required_sections {
            if g2_config.get(section).is_none() {
                errors.push(format!("Missing required section: {}", section));
            }
        }

        // Recommended sections
        let recommended_sections = vec![
            "CFG_ERFRAG",
            "CFG_ERRULE",
            "CFG_FBOM",
            "CFG_CFUNC",
            "CFG_EFUNC",
            "CFG_SFUNC",
        ];

        for section in recommended_sections {
            if g2_config.get(section).is_none() {
                warnings.push(format!("Missing recommended section: {}", section));
            }
        }

        // Check version info
        if let Some(base_version) = g2_config.get("CONFIG_BASE_VERSION") {
            if base_version.get("VERSION").is_none() {
                warnings.push("Missing VERSION in CONFIG_BASE_VERSION".to_string());
            }
            if base_version.get("COMPATIBILITY_VERSION").is_none() {
                warnings.push("Missing COMPATIBILITY_VERSION".to_string());
            } else {
                let compat = &base_version["COMPATIBILITY_VERSION"];
                if let Some(config_version) = compat.get("CONFIG_VERSION").and_then(|v| v.as_str()) {
                    println!("  Config version: {}", config_version);
                } else {
                    warnings.push("Missing CONFIG_VERSION".to_string());
                }
            }
        }

        // Count entities
        println!("  Data sources: {}", count_array(g2_config, "CFG_DSRC"));
        println!("  Attributes: {}", count_array(g2_config, "CFG_ATTR"));
        println!("  Features: {}", count_array(g2_config, "CFG_FTYPE"));
        println!("  Elements: {}", count_array(g2_config, "CFG_FELEM"));
        println!("  Fragments: {}", count_array(g2_config, "CFG_ERFRAG"));
        println!("  Rules: {}", count_array(g2_config, "CFG_ERRULE"));
    }

    println!();

    // Report results
    if !errors.is_empty() {
        println!("❌ ERRORS ({}):", errors.len());
        for error in &errors {
            println!("  - {}", error);
        }
        println!();
    }

    if !warnings.is_empty() {
        println!("⚠️  WARNINGS ({}):", warnings.len());
        for warning in &warnings {
            println!("  - {}", warning);
        }
        println!();
    }

    if errors.is_empty() && warnings.is_empty() {
        println!("✅ Configuration is valid!");
        println!("\n=== Validation Complete ===");
        Ok(())
    } else if errors.is_empty() {
        println!("✅ Configuration is structurally valid");
        println!("⚠️  {} warnings", warnings.len());
        println!("\n=== Validation Complete ===");
        Ok(())
    } else {
        println!("❌ Configuration has {} errors", errors.len());
        println!("\n=== Validation Failed ===");
        std::process::exit(1);
    }
}

fn count_array(config: &Value, section: &str) -> usize {
    config
        .get(section)
        .and_then(|v| v.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0)
}
