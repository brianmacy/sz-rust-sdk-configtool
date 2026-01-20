//! Process real Senzing upgrade script example
//!
//! This example demonstrates processing the actual szcore-configuration-upgrade-10-to-11.gtc
//! script from Senzing. It shows how to:
//! - Load a configuration file
//! - Process an official upgrade script
//! - Validate the results
//! - Save the upgraded configuration

use std::env;
use sz_configtool_lib::command_processor::CommandProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Process Real Senzing Upgrade Script ===\n");

    // Get paths from arguments or use defaults
    let args: Vec<String> = env::args().collect();

    let config_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("g2config_v10.json");

    let script_path = args.get(2)
        .map(|s| s.as_str())
        .unwrap_or("/opt/homebrew/opt/senzing/runtime/er/resources/config/szcore-configuration-upgrade-10-to-11.gtc");

    let output_path = args
        .get(3)
        .map(|s| s.as_str())
        .unwrap_or("g2config_v11.json");

    println!("Configuration:");
    println!("  Input:  {}", config_path);
    println!("  Script: {}", script_path);
    println!("  Output: {}\n", output_path);

    // Load configuration
    println!("Loading configuration...");
    let config = match std::fs::read_to_string(config_path) {
        Ok(c) => {
            println!("  ✓ Loaded {} bytes", c.len());
            c
        }
        Err(e) => {
            eprintln!("  ✗ Failed to load config: {}", e);
            eprintln!(
                "\nUsage: {} [config.json] [upgrade.gtc] [output.json]",
                args[0]
            );
            std::process::exit(1);
        }
    };

    // Verify starting version
    let config_val: serde_json::Value = serde_json::from_str(&config)?;
    let start_version =
        config_val["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"]
            .as_str()
            .unwrap_or("unknown");
    println!("  Starting version: {}\n", start_version);

    // Load upgrade script
    println!("Loading upgrade script...");
    let script = match std::fs::read_to_string(script_path) {
        Ok(s) => {
            let line_count = s.lines().count();
            let command_count = s
                .lines()
                .filter(|l| {
                    let trimmed = l.trim();
                    !trimmed.is_empty() && !trimmed.starts_with('#') && trimmed != "save"
                })
                .count();
            println!(
                "  ✓ Loaded {} lines ({} commands)",
                line_count, command_count
            );
            s
        }
        Err(e) => {
            eprintln!("  ✗ Failed to load script: {}", e);
            eprintln!("\nUpgrade script not found. This example requires:");
            eprintln!("  {}", script_path);
            std::process::exit(1);
        }
    };

    // Process upgrade script
    println!("\nProcessing upgrade script...");
    let mut processor = CommandProcessor::new(config);

    let upgraded_config = match processor.process_script(&script) {
        Ok(cfg) => {
            println!("  ✓ {}", processor.summary());
            cfg
        }
        Err(e) => {
            eprintln!("  ✗ Error: {}", e);
            eprintln!("\nCommands executed before error:");
            for cmd in processor.get_executed_commands() {
                eprintln!("    {}", cmd);
            }
            std::process::exit(1);
        }
    };

    // Verify ending version
    let upgraded_val: serde_json::Value = serde_json::from_str(&upgraded_config)?;
    let end_version =
        upgraded_val["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"]
            .as_str()
            .unwrap_or("unknown");
    println!("  Ending version: {}\n", end_version);

    // Show sample of executed commands
    println!("Sample of executed commands:");
    for cmd in processor.get_executed_commands().iter().take(10) {
        println!("  - {}", cmd);
    }
    if processor.get_executed_commands().len() > 10 {
        println!(
            "  ... and {} more",
            processor.get_executed_commands().len() - 10
        );
    }

    // Save upgraded configuration
    println!("\nSaving upgraded configuration...");
    match std::fs::write(output_path, &upgraded_config) {
        Ok(_) => {
            println!("  ✓ Saved to {}", output_path);
            println!("  Size: {} bytes", upgraded_config.len());
        }
        Err(e) => {
            eprintln!("  ✗ Failed to save: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n=== Upgrade Complete! ===");
    println!(
        "Configuration successfully upgraded from v{} to v{}",
        start_version, end_version
    );

    Ok(())
}
