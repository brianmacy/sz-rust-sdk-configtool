# sz_configtool_lib

[![CI](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/ci.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/ci.yml)
[![Security Audit](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/security.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/security.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Pure Rust library for manipulating Senzing configuration JSON documents.

## Overview

`sz_configtool_lib` provides 147 functions across 30 modules for programmatic manipulation of Senzing configuration documents (g2config.json). The library contains only pure business logic with no display formatting, making it ideal for automation scripts, migration tools, and external integrations.

### ⚠️ Important Note on Usage

**This is an unofficial SDK.** Outside of basic operations like adding data sources, Senzing does not publicly document the meaning and proper usage of most configuration functions and parameters without specific education and guidance. This library enables you to programmatically accomplish configuration tasks once you've received proper guidance on their recommended use for your particular situation.

**Recommendation:** Work with Senzing support or documentation to understand:

- When and why to use specific configuration functions
- Appropriate parameter values for your use case
- Impact of configuration changes on entity resolution behavior

This library provides the "how" (programmatic interface) - you need Senzing guidance for the "what" and "when" (proper configuration practices).

## Features

- ✅ **Code-Based API** - Use intuitive string codes instead of numeric IDs (no manual lookups!)
- ✅ **Pure JSON Manipulation** - No SDK dependencies for core operations
- ✅ **No Display Logic** - Zero dependencies on formatting, colors, or output libraries
- ✅ **Type-Safe Errors** - Comprehensive error handling with `SzConfigError`
- ✅ **Well-Documented** - All public functions have rustdoc comments
- ✅ **Tested** - Comprehensive unit and integration tests
- ✅ **Clean API** - Parameter structs with builder pattern for self-documenting code
- ✅ **Modern Design** - All functions use `(config, params)` pattern

## API Design

This library uses **parameter structs** for a clean, self-documenting API:

```rust
// ✨ Named fields - crystal clear what each parameter does
features::set_feature(&config, SetFeatureParams {
    feature: "NAME",
    candidates: Some("Yes"),
    behavior: Some("NAME"),
    version: Some(2),
    ..Default::default()
})?;
```

**Benefits:**

- 🔍 **Self-documenting** - No need to count parameters or check docs
- 🛡️ **Type-safe** - Compile-time field validation
- 📈 **Extensible** - Add fields without breaking existing code
- 💡 **IDE-friendly** - Auto-completion shows available fields

All parameter structs implement `TryFrom<&Value>` for easy JSON conversion.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sz_configtool_lib = { git = "https://github.com/brianmacy/sz-rust-sdk-configtool", tag = "v0.1.0" }
```

Or from a specific commit:

```toml
[dependencies]
sz_configtool_lib = { git = "https://github.com/brianmacy/sz-rust-sdk-configtool", rev = "abc123" }
```

Once published to crates.io:

```toml
[dependencies]
sz_configtool_lib = "0.1.0"
```

## Quick Start

```rust
use sz_configtool_lib::{datasources, attributes, features};
use sz_configtool_lib::datasources::AddDataSourceParams;
use sz_configtool_lib::attributes::AddAttributeParams;
use sz_configtool_lib::features::{AddFeatureParams, SetFeatureParams};
use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load existing config
    let config = fs::read_to_string("g2config.json")?;

    // Add a data source (with named parameters!)
    let config = datasources::add_data_source(
        &config,
        AddDataSourceParams {
            code: "MY_SOURCE",
            ..Default::default()
        },
    )?;

    // Add an attribute (self-documenting!)
    let (config, _attr) = attributes::add_attribute(
        &config,
        AddAttributeParams {
            attribute: "MY_ATTR",
            feature: "ADDRESS",
            element: "ADDR_LINE1",
            class: "ADDRESS",
            default_value: None,
            internal: Some("No"),
            required: Some("No"),
        },
    )?;

    // Add a feature with element list
    let element_list = json!([
        {"element": "NAME", "expressed": "No"},
        {"element": "ADDRESS", "expressed": "No"}
    ]);

    let config = features::add_feature(
        &config,
        AddFeatureParams {
            feature: "MY_FEATURE",
            element_list: &element_list,
            class: Some("IDENTITY"),
            behavior: Some("FM"),
            candidates: Some("Yes"),
            ..Default::default()
        },
    )?;

    // Update a feature (crystal clear what's changing!)
    let config = features::set_feature(
        &config,
        SetFeatureParams {
            feature: "MY_FEATURE",
            behavior: Some("NAME"),
            version: Some(2),
            ..Default::default()
        },
    )?;

    // Save modified config
    fs::write("modified_config.json", config)?;

    Ok(())
}
```

## Module Organization

### Core Infrastructure

- **`error`** - Custom error types (`SzConfigError`)
- **`helpers`** - Shared utilities (ID generation, array operations, lookups)

### Core Entities (87 functions)

#### Data Management

- **`datasources`** (7 functions) - Add, delete, get, list, set data sources (CFG_DSRC)
- **`attributes`** (8 functions) - Attribute management (CFG_ATTR)

#### Feature Management

- **`features`** (24 functions) - Features with elements, comparisons, distinct calls
- **`feature_types`** (5 functions) - Feature type operations (CFG_FTYPE)
- **`elements`** (8 functions) - Element operations (CFG_FELEM)

#### Configuration

- **`thresholds`** (8 functions) - Comparison and generic thresholds
- **`rules`** (5 functions) - Entity resolution rules (CFG_ERRULE)
- **`fragments`** (5 functions) - Rule fragments (CFG_ERFRAG)
- **`generic_plans`** (4 functions) - Generic plan management (CFG_GPLAN)
- **`hashes`** (4 functions) - Name and SSN hash management

#### System Management

- **`config_sections`** (6 functions) - G2_CONFIG section manipulation
- **`system_params`** (2 functions) - System parameter operations
- **`versioning`** (4 functions) - Version management

### Functions & Calls (60 functions)

#### Function Modules (28 functions)

- **`functions/standardize`** (6) - Standardization functions (CFG_SFUNC)
- **`functions/expression`** (6) - Expression functions (CFG_EFUNC)
- **`functions/comparison`** (7) - Comparison functions (CFG_CFUNC)
- **`functions/distinct`** (6) - Distinct functions (CFG_DFUNC)
- **`functions/matching`** (1) - Matching functions (CFG_RTYPE)
- **`functions/scoring`** (0) - Scoring functions (stubs)
- **`functions/candidate`** (0) - Candidate functions (stubs)
- **`functions/validation`** (0) - Validation functions (stubs)

#### Call Modules (32 functions)

- **`calls/standardize`** (8) - Standardize calls with BOM (CFG_SFCALL, CFG_SBOM)
- **`calls/expression`** (8) - Expression calls with BOM (CFG_EFCALL, CFG_EFBOM)
- **`calls/comparison`** (8) - Comparison calls with BOM (CFG_CFCALL, CFG_CFBOM)
- **`calls/distinct`** (8) - Distinct calls with BOM (CFG_DFCALL, CFG_DFBOM)

## API Examples

### Data Source Operations

```rust
use sz_configtool_lib::datasources::{self, AddDataSourceParams, SetDataSourceParams};

// Add a data source with named parameters
let config = datasources::add_data_source(
    &config,
    AddDataSourceParams {
        code: "CUSTOMERS",
        retention_level: Some("Remember"),
        reliability: Some(2),
        ..Default::default()
    },
)?;

// List all data sources
let sources = datasources::list_data_sources(&config)?;
for source in sources {
    println!("{}: {}", source["dataSource"], source["id"]);
}

// Get specific data source
let source = datasources::get_data_source(&config, "CUSTOMERS")?;

// Update data source
let config = datasources::set_data_source(
    &config,
    SetDataSourceParams {
        code: "CUSTOMERS",
        retention_level: Some("Forget"),
        reliability: Some(3),
        ..Default::default()
    },
)?;

// Delete data source
let config = datasources::delete_data_source(&config, "CUSTOMERS")?;
```

### Feature Operations

```rust
use sz_configtool_lib::features::{self, AddFeatureParams, SetFeatureParams};
use serde_json::json;

// Define element list
let elements = json!([
    {"element": "NAME", "expressed": "No"},
    {"element": "ADDRESS", "expressed": "Yes"},
    {"element": "PHONE", "expressed": "No"}
]);

// Add feature with named parameters
let config = features::add_feature(
    &config,
    AddFeatureParams {
        feature: "PERSON",
        element_list: &elements,
        class: Some("IDENTITY"),
        behavior: Some("FM"),
        candidates: Some("Yes"),
        ..Default::default()
    },
)?;

// List features
let features_list = features::list_features(&config)?;

// Get feature with full element list
let feature = features::get_feature(&config, "PERSON")?;

// Update feature (self-documenting!)
let config = features::set_feature(
    &config,
    SetFeatureParams {
        feature: "PERSON",
        class: Some("IDENTITY"),
        behavior: Some("NAME"),
        version: Some(2),
        ..Default::default()
    },
)?;
```

### Element and Feature Element Operations

```rust
use sz_configtool_lib::elements::{self, SetFeatureElementParams};

// Update feature element using intuitive codes (no ID lookups needed!)
let config = elements::set_feature_element(
    &config,
    SetFeatureElementParams::new("NAME", "FIRST_NAME")
        .with_display_level(1)
        .with_derived("No"),
)?;

// Convenience functions for common operations
let config = elements::set_feature_element_display_level(&config, "ADDRESS", "ADDR_LINE1", 2)?;
let config = elements::set_feature_element_derived(&config, "NAME", "FULL_NAME", "Yes")?;
```

### Threshold Operations

```rust
use sz_configtool_lib::thresholds::{self, AddComparisonThresholdParams, AddGenericThresholdParams};

// Add comparison threshold using function and feature codes (no ID lookups!)
let config = thresholds::add_comparison_threshold(
    &config,
    AddComparisonThresholdParams {
        cfunc_code: "SAME_PHONE",
        ftype_code: "PHONE",
        cfunc_rtnval: "FULL_SCORE".to_string(),
        same_score: Some(85),
        close_score: Some(75),
        likely_score: Some(60),
        plausible_score: Some(45),
        un_likely_score: Some(30),
        ..Default::default()
    },
)?;

// Add generic threshold using plan code (no ID lookup needed!)
let config = thresholds::add_generic_threshold(
    &config,
    AddGenericThresholdParams {
        plan_code: "SEARCH",
        behavior: "NAME",
        scoring_cap: 1000,
        candidate_cap: 1000,
        send_to_redo: "Yes",
        feature: Some("NAME"),
    },
)?;

// List all generic thresholds
let thresholds_list = thresholds::list_generic_thresholds(&config)?;

// Update threshold by name (no ID lookups needed)
use sz_configtool_lib::thresholds::SetGenericThresholdByNameParams;
let config = thresholds::set_generic_threshold_by_name(
    &config,
    SetGenericThresholdByNameParams::new("INGEST", "FM")
        .with_feature("SEMANTIC_VALUE")
        .with_candidate_cap(20)
        .with_scoring_cap(-1),
)?;

// Update comparison threshold by name (no ID lookups needed)
use sz_configtool_lib::thresholds::SetComparisonThresholdByKeyParams;
let config = thresholds::set_comparison_threshold_by_key(
    &config,
    SetComparisonThresholdByKeyParams::new("SEMANTIC_SIMILARITY_COMP", "FULL_SCORE")
        .with_feature("SEMANTIC_VALUE")
        .with_same_score(100)
        .with_close_score(90),
)?;
```

### Command Script Processing

Process Senzing `.gtc` command scripts for automated config upgrades:

```rust
use sz_configtool_lib::command_processor::CommandProcessor;

// Load config
let config = std::fs::read_to_string("g2config_v10.json")?;

// Create processor
let mut processor = CommandProcessor::new(config);

// Process upgrade script
let upgraded = processor.process_file(
    "/path/to/szcore-configuration-upgrade-10-to-11.gtc"
)?;

// Save upgraded config
std::fs::write("g2config_v11.json", upgraded)?;

println!("✓ {}", processor.summary());
// Output: "✓ Executed 90 commands"
```

**Supported commands (27 total):**

- Versioning: `verifyCompatibilityVersion`, `updateCompatibilityVersion`
- Attributes: `addAttribute`, `deleteAttribute`, `setAttribute`
- Features: `addFeature`, `setFeature`, `addBehaviorOverride`
- Elements: `addElement`, `setFeatureElement`
- Fragments: `deleteFragment`, `setFragment`
- Functions: `addExpressionFunction`, `addComparisonFunction`, etc.
- Thresholds: `addComparisonThreshold`, `addGenericThreshold`
- Rules: `addRule`, `setRule`
- System: `setSetting`
- And 10+ more...

See `examples/command_processor.rs` for a complete working example.

### Function and Call Management

```rust
use sz_configtool_lib::functions::standardize;
use sz_configtool_lib::calls::standardize as std_calls;

// Add a standardize function
let (config, func) = standardize::add_standardize_function(
    &config,
    "PARSE_PHONE",
    "parsePhone",
    Some("Parse telephone numbers"),
    Some("eng"),
)?;

// Add a standardize call (links function to feature/element)
let (config, call) = std_calls::add_standardize_call(
    &config,
    "PHONE",      // ftype_code
    "PHONE",      // felem_code
    1,            // exec_order
    1001,         // sfunc_id
)?;

// List all standardize calls with resolved names
let calls = std_calls::list_standardize_calls(&config)?;
```

## Error Handling

All functions return `Result<T, SzConfigError>`:

```rust
use sz_configtool_lib::{SzConfigError, datasources};

match datasources::add_data_source(&config, "TEST") {
    Ok(modified_config) => {
        println!("Success!");
        // Use modified_config
    }
    Err(SzConfigError::AlreadyExists(entity, id)) => {
        eprintln!("{} '{}' already exists", entity, id);
    }
    Err(SzConfigError::NotFound(entity, id)) => {
        eprintln!("{} '{}' not found", entity, id);
    }
    Err(SzConfigError::InvalidInput(msg)) => {
        eprintln!("Invalid input: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### Error Types

- **`JsonParse(String)`** - JSON parsing error
- **`NotFound(String, String)`** - Entity not found (entity_type, identifier)
- **`AlreadyExists(String, String)`** - Entity already exists
- **`InvalidInput(String)`** - Invalid input or parameters
- **`MissingSection(String)`** - Required config section missing
- **`InvalidStructure(String)`** - Invalid config structure

## Function Return Types

Functions follow consistent patterns:

### Modification Operations

```rust
// Add/Set/Delete operations return modified config
fn add_data_source(config_json: &str, code: &str) -> Result<String, SzConfigError>
fn delete_data_source(config_json: &str, code: &str) -> Result<String, SzConfigError>
```

### Add Operations with Record Return

```rust
// Add operations that need to return the created record
fn add_attribute(config_json: &str, ...) -> Result<(String, Value), SzConfigError>
//                                               ^^^^^^^^^^^^^^^^^^^^
//                                               (modified_config, new_record)
```

### Query Operations

```rust
// Get operations return a single record
fn get_data_source(config_json: &str, code: &str) -> Result<Value, SzConfigError>

// List operations return array of records
fn list_data_sources(config_json: &str) -> Result<Vec<Value>, SzConfigError>
```

## Testing

```bash
# Run all library tests
cd sz_configtool_lib
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_data_source

# Check code coverage (with tarpaulin)
cargo tarpaulin --out Html
```

## Documentation

Generate API documentation:

```bash
cd sz_configtool_lib
cargo doc --no-deps --open
```

All public functions include comprehensive rustdoc comments with:

- Function description
- Parameter descriptions
- Return value description
- Error conditions
- Usage examples (where applicable)

## Design Principles

1. **Pure Functions** - All functions are pure: same input → same output
2. **No Side Effects** - Functions don't modify global state or make network calls
3. **No Display Logic** - Zero dependencies on formatting libraries
4. **Minimal Dependencies** - Only serde, serde_json, and anyhow
5. **Error Transparency** - All errors are explicit and type-safe
6. **API Stability** - Function signatures match CLI command parameters

## Performance

- **Fast JSON Parsing** - Uses serde_json with ordered maps
- **Zero-Copy Where Possible** - Minimizes allocations
- **Efficient Lookups** - Helper functions cache commonly accessed data
- **Batch Operations** - Multiple operations can be chained efficiently

Example: Processing 1000 data source additions takes ~50ms on modern hardware.

## Limitations

1. **No SDK Integration** - Library operates only on JSON strings
   - For SzConfigManager operations, use the CLI tool or SDK directly
2. **No Validation Against Schema** - Assumes well-formed g2config.json
   - Validation should be done with SzConfig SDK methods
3. **English-Only Errors** - Error messages are in English
4. **No Async Support** - All operations are synchronous

## C FFI Interface

The library provides a C-compatible Foreign Function Interface (FFI) for use from C, C++, Python (ctypes), and other languages.

### Building the Shared Library

```bash
# Build shared library (.so on Linux, .dylib on macOS, .dll on Windows)
cargo build --lib --release

# Library location
target/release/libsz_configtool_lib.so    # Linux
target/release/libsz_configtool_lib.dylib # macOS
target/release/sz_configtool_lib.dll      # Windows
```

### C Header File

The C header file is located at `include/libSzConfigTool.h` and contains declarations for 98 FFI functions.

### C Example

```c
#include "libSzConfigTool.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Load config from file
    char *config = /* read config file */;

    // Add a data source
    SzConfigTool_result result = SzConfigTool_addDataSource(config, "MY_SOURCE", NULL, NULL, NULL);

    if (result.return_code == 0) {
        printf("Data source added successfully\n");
        // Use result.response (modified config JSON)

        // Always free the response string
        SzConfigTool_free(result.response);
    } else {
        const char *error = SzConfigTool_getLastError();
        fprintf(stderr, "Error: %s\n", error);
    }

    return 0;
}
```

### Building C Applications

```bash
# Compile C program
gcc -o myapp myapp.c -L./target/release -lsz_configtool_lib -I./include

# Run (Linux/macOS)
LD_LIBRARY_PATH=./target/release ./myapp

# Run (macOS alternative)
DYLD_LIBRARY_PATH=./target/release ./myapp
```

### Memory Management

**Critical**: All strings returned by FFI functions are owned by Rust and must be freed using `SzConfigTool_free()`:

```c
SzConfigTool_result result = SzConfigTool_listDataSources(config, "JSON");
if (result.return_code == 0) {
    // Use result.response
    printf("%s\n", result.response);

    // REQUIRED: Free the string
    SzConfigTool_free(result.response);
}
```

### Error Handling

Errors are stored in thread-local storage and retrieved with `SzConfigTool_getLastError()`:

```c
if (result.return_code != 0) {
    const char *error = SzConfigTool_getLastError();
    fprintf(stderr, "Operation failed: %s\n", error);
}
```

### JSON Parameter Marshalling

Complex parameters are passed as JSON strings:

```c
// Updating a function with multiple fields
const char *updates = "{\"CONNECT_STR\": \"new_value\", \"SFUNC_DESC\": \"Updated description\"}";
SzConfigTool_result result = SzConfigTool_setStandardizeFunctionWithJson(
    config,
    "PARSE",
    updates
);
```

### Available FFI Functions

The FFI provides 98 functions covering:

- Data source operations (7 functions)
- Attribute management (8 functions)
- Feature operations (24 functions)
- Element management (8 functions)
- Threshold configuration (6 functions)
- Function management (28 functions)
- Call management (32 functions)
- System configuration (multiple functions)

See `include/libSzConfigTool.h` for the complete function list and documentation.

## Contributing

Contributions are welcome! Please see `docs/CONTRIBUTING.md` for guidelines.

## License

Apache 2.0 License - See LICENSE file for details.

## See Also

- **CLI Tool:** [sz_configtool](https://github.com/brianmacy/sz_configtool_rust) - Interactive command-line tool using this library
- **Python Version:** [sz_configtool](https://github.com/senzing-garage/sz-python-tools) - Original Python implementation
- **Senzing SDK:** [sz-rust-sdk](https://github.com/brianmacy/sz-rust-sdk) - Full Senzing SDK for Rust

## Version History

### v0.1.0 (2025-10-02)

- Initial release
- 147 functions across 30 modules
- Complete coverage of Senzing config operations
- Comprehensive documentation and tests

---

**Status:** ✅ Production Ready
**Build:** ✅ 0 errors, 0 warnings
**Tests:** ✅ All passing
**Documentation:** ✅ Complete
