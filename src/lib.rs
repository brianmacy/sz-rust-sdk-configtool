//! # SzConfigTool Library
//!
//! Pure Rust library for manipulating Senzing configuration JSON documents.
//!
//! This library provides programmatic access to configuration operations
//! without any display logic, making it suitable for automation, migration
//! scripts, and external tools.
//!
//! ## Features
//!
//! - Pure JSON manipulation (no SDK dependencies)
//! - No display logic (no formatting, colors, or output)
//! - Type-safe error handling
//! - Parameters aligned with sz_configtool CLI commands
//!
//! ## Example Usage
//!
//! ```no_run
//! use sz_configtool_lib::{datasources, attributes};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load existing config
//!     let config = std::fs::read_to_string("g2config.json")?;
//!
//!     // Add a data source
//!     let config = datasources::add_data_source(&config, "NEW_SOURCE", None, None, None)?;
//!
//!     // Add an attribute
//!     let (config, _) = attributes::add_attribute(
//!         &config,
//!         "NEW_ATTR",
//!         "ADDRESS",
//!         "ELEMENT",
//!         "OTHER",
//!         None,
//!         None,
//!         None
//!     )?;
//!
//!     // Save modified config
//!     std::fs::write("g2config_modified.json", config)?;
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod helpers;

// Core entity modules
pub mod attributes;
pub mod datasources;
pub mod elements;
pub mod features;
pub mod thresholds;

// Advanced operations modules
pub mod config_sections;
pub mod fragments;
pub mod generic_plans;
pub mod hashes;
pub mod rules;
pub mod system_params;
pub mod versioning;

// Function and call management modules
pub mod calls;
pub mod functions;

// Re-export commonly used types
pub use error::{Result, SzConfigError};

// C FFI module
pub mod ffi;
