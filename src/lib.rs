//! # SzConfigTool Library
//!
//! Pure Rust library for manipulating Senzing configuration JSON documents.
//!
//! This library provides programmatic access to configuration operations
//! without any display logic, making it suitable for automation, migration
//! scripts, and external tools.
//!
//! ## ⚠️ Important: Unofficial SDK - Requires Senzing Guidance
//!
//! **This is an unofficial SDK.** Senzing does not publicly document the meaning, usage, or
//! recommended practices for most configuration functions and parameters beyond basic operations
//! (like adding data sources).
//!
//! **Before using this library**, you should have received specific guidance from Senzing support
//! or documentation about:
//! - When and why to use particular configuration functions
//! - Appropriate parameter values for your specific use case
//! - Impact of configuration changes on entity resolution behavior
//!
//! This library provides the programmatic interface ("how") - proper usage requires
//! Senzing-provided guidance on configuration best practices ("what" and "when").
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
//!     let config = datasources::add_data_source(
//!         &config,
//!         datasources::AddDataSourceParams {
//!             code: "NEW_SOURCE",
//!             ..Default::default()
//!         },
//!     )?;
//!
//!     // Add an attribute
//!     let (config, _) = attributes::add_attribute(
//!         &config,
//!         attributes::AddAttributeParams {
//!             attribute: "NEW_ATTR",
//!             feature: "ADDRESS",
//!             element: "ELEMENT",
//!             class: "OTHER",
//!             default_value: None,
//!             internal: None,
//!             required: None,
//!         },
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

// Shared crate-internal row structs (one per CFG_* section).
pub(crate) mod config_rows;

// Shared domain/substrate modules
pub mod behavior_domain;
pub mod filter;

// Core entity modules
pub mod attributes;
pub mod behavior_overrides;
pub mod datasources;
pub mod elements;
pub mod features;
pub mod thresholds;

// Advanced operations modules
pub mod command_processor;
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
pub use error::{Result, SzConfigError, SzErrorKind};

// Re-export shared domain items so consumers can use them without the
// fully-qualified module path.
pub use attributes::ATTRIBUTE_CLASSES;
pub use behavior_domain::{
    BEHAVIOR_CODES, behavior_position, compute_behavior, parse_behavior_code,
};
pub use filter::{FilterSubstrate, matches_filter, to_json_dumps_string, to_python_repr_string};
pub use helpers::FieldUpdate;
pub use helpers::{
    resolve_cfcall_id_for_feature, resolve_dfcall_id_for_feature, resolve_efcall_id_for_feature,
    resolve_sfcall_id_for_feature,
};

// C FFI module
pub mod ffi;
