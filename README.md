# sz_configtool_lib

[![CI](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/ci.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/ci.yml)
[![Security Audit](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/security.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-configtool/actions/workflows/security.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)

Pure Rust library for manipulating Senzing configuration JSON documents.

## Overview

`sz_configtool_lib` provides 203 public functions across 35 modules (146 configuration functions in the binding manifest) for programmatic manipulation of Senzing configuration documents (g2config.json). The library contains only pure business logic with no display formatting, making it ideal for automation scripts, migration tools, and external integrations.

### ⚠️ Important Note on Usage

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

Outside of basic operations like adding data sources, the meaning and proper usage of most configuration functions and parameters require specific education and guidance from Senzing. This library enables you to programmatically accomplish configuration tasks once you've received proper guidance on their recommended use for your particular situation.

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

### Execution-order policy

Add paths that write an `EXEC_ORDER` (call elements, feature comparisons and
comparison thresholds) resolve it uniformly:

- `exec_order: None` — **auto-allocate** the next free order within the row's
  scope (max in scope + 1).
- `exec_order: Some(n)` (`n > 0`, free) — **honour** the requested order.
- `exec_order: Some(n)` (`n > 0`, taken) — **reject** with `AlreadyExists`
  (never silently reallocated).

An order is always written as a concrete value, never `null`. The scope is the
call for BOM elements, `(feature, element)` for standardize/expression calls,
and the whole table for feature comparisons. Comparison thresholds additionally
**reuse** an existing all-features return-value tier's order first, so
per-feature overrides stay on the same scoring tier as the base row. See the
`calls` module docs for the full table.

## Pre-built packages

Binaries for the C ABI and every language binding are attached to each
[GitHub Release](https://github.com/brianmacy/sz-rust-sdk-configtool/releases)
— and only there (nothing is published to crates.io, PyPI, Maven Central,
NuGet or npm). Full asset list and contents:
[`packaging/README.md`](packaging/README.md).

**Supported platforms** (pre-built binaries):

| Platform | Requirement |
|---|---|
| Linux x86_64 / arm64 (`linux-x64`, `linux-arm64`) | glibc >= 2.34: RHEL 9+, Amazon Linux 2023, Ubuntu 22.04+, Debian 12+ |
| macOS arm64 (`macos-arm64`) | macOS 15+ |
| Windows x64 (`windows-x64`) | MSVC build, static VC++ runtime (no VC++ Redistributable needed) |

RHEL 8 (glibc 2.28) is **not** supported by these binaries (the floor matches
Senzing's own anylinux builds). RHEL 8 users need to build from source with
the Rust library (`cargo build -p sz-configtool-ffi --release` on the target
system, or depend on `sz_configtool_lib` directly).

| Language | Asset (`<v>` = version, `<os>-<arch>` = platform) |
|---|---|
| C (header + shared/static library) | `sz-configtool-<v>-<os>-<arch>.tar.gz` (`.zip` on Windows) |
| C++ (header-only + CMake package) | `sz-configtool-cpp-<v>-<os>-<arch>.tar.gz` / `.zip` |
| Python | `sz_configtool-<v>-cp310-abi3-<platform>.whl` (`pip install` it; PEP 440 version, e.g. `4.4.0.post1` for `4.4.0-1`) |
| Java | `sz-configtool-<v>.jar` (natives for all platforms inside) |
| .NET | `Sz.ConfigTool.<v>.nupkg` (natives for all platforms inside) |
| Node.js | `sz-configtool-node-<v>-<os>-<arch>.tgz` (`npm install` it); tRPC router `sz-configtool-trpc-<v>.tgz` |
| SBOMs (CycloneDX) | `sz-configtool[-jni\|-node\|-python]-<v>-<os>-<arch>.cdx.json` |

Verify a download against the release's `SHA256SUMS` and its GitHub build
provenance attestation:

```bash
sha256sum -c --ignore-missing SHA256SUMS            # macOS: shasum -a 256 -c --ignore-missing SHA256SUMS
gh attestation verify sz-configtool-<v>-linux-x64.tar.gz --repo brianmacy/sz-rust-sdk-configtool
# offline, with the bundle attached to the release:
gh attestation verify sz-configtool-<v>-linux-x64.tar.gz --repo brianmacy/sz-rust-sdk-configtool \
  --bundle sz-configtool-<v>.intoto.jsonl
```

The binaries are **not code-signed** (no Authenticode on Windows, no macOS
codesign/notarization); verify them as above. On macOS, a file downloaded with
a browser carries the quarantine attribute and Gatekeeper may refuse to load
the library: remove it after verifying, e.g.
`xattr -d com.apple.quarantine libSzConfigTool.dylib` (or `xattr -dr
com.apple.quarantine <extracted-dir>`).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sz_configtool_lib = { git = "https://github.com/brianmacy/sz-rust-sdk-configtool", tag = "v4.4.0-1" }
```

Or from a specific commit:

```toml
[dependencies]
sz_configtool_lib = { git = "https://github.com/brianmacy/sz-rust-sdk-configtool", rev = "abc123" }
```

The crate is not published to crates.io; use a git dependency with a release
tag (above). See [Versioning](#versioning) for what the version means.

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

Counts are public free functions (`pub fn`) per module; the crate has
203 in 35 public modules (23 top-level, 4 under `calls`, 8 under
`functions`). The binding manifest (`api/manifest/`) covers the configuration
functions among them: 146 functions in 28 groups (126 implemented, 20
`NOT_IMPLEMENTED` placeholders).

### Core Infrastructure

- **`error`** - Error types (`SzConfigError`, `SzErrorKind`, validation failures)
- **`helpers`** (28) - Shared utilities (ID generation, array operations, lookups)
- **`filter`** (4), **`behavior_domain`** (3) - List filtering and behavior-code helpers
- **`command_processor`** - Replays `sz_configtool` command scripts (`CommandProcessor` methods)

### Core Entities

- **`datasources`** (5) - Data sources (CFG_DSRC)
- **`attributes`** (5) - Attributes (CFG_ATTR)
- **`features`** (16) - Features (CFG_FTYPE, CFG_FBOM) with elements, comparisons, distinct calls
- **`elements`** (10) - Elements (CFG_FELEM)
- **`behavior_overrides`** (5) - Per-usage-type behavior overrides (CFG_FBOVR)

### Configuration

- **`thresholds`** (14) - Comparison and generic thresholds (CFG_CFRTN, CFG_GENERIC_THRESHOLD)
- **`rules`** (5) - Entity resolution rules (CFG_ERRULE)
- **`fragments`** (5) - Rule fragments (CFG_ERFRAG)
- **`generic_plans`** (4) - Generic plans (CFG_GPLAN)
- **`search_profiles`** (4) - Search profile add/get/list/delete (CFG_SPROFILE); `INGEST`/`SEARCH` are delete-protected
- **`settings`** (1), **`validation`** (1), **`export`** (1) - Settings, config validation, export

### System Management

- **`config_sections`** (7) - G2_CONFIG section manipulation
- **`system_params`** (2) - System parameters
- **`versioning`** (4) - Version management

### Function Modules

- **`functions/standardize`** (6) - Standardization functions (CFG_SFUNC)
- **`functions/expression`** (6) - Expression functions (CFG_EFUNC)
- **`functions/comparison`** (6) - Comparison functions (CFG_CFUNC)
- **`functions/distinct`** (5) - Distinct functions (CFG_DFUNC)
- **`functions/matching`** (6) - Matching functions (CFG_RTYPE; placeholders)
- **`functions/scoring`** (6) - Scoring functions (placeholders)
- **`functions/candidate`** (6) - Candidate functions (placeholders)
- **`functions/validation`** (6) - Validation functions (placeholders)

### Call Modules

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
# Run the Rust library tests (default workspace member)
cargo test

# Run everything, including the C FFI crate and the C ABI tests
cargo test --workspace

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_data_source

# Coverage of every component (Rust + all bindings) with the 100% gate
packaging/install-tools.sh <target> && packaging/install-tools.sh <target> coverage
packaging/coverage.sh
```

Coverage policy, tools and the reviewed exclusions: `coverage/policy.yaml` and
[`packaging/README.md`](packaging/README.md#coverage).

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

The C ABI lives in a separate workspace crate, [`ffi/`](ffi/) (package
`sz-configtool-ffi`), so the Rust library `sz_configtool_lib` stays a plain
`lib` crate whose dependents never inherit C symbols. It is usable from C, C++,
Python (ctypes) and anything else that can call C.

### Building the Library

```bash
cargo build -p sz-configtool-ffi --release

# Outputs (Senzing-style names)
target/release/libSzConfigTool.so      # Linux shared
target/release/libSzConfigTool.dylib   # macOS shared
target/release/SzConfigTool.dll        # Windows shared (+ SzConfigTool.dll.lib import lib)
target/release/libSzConfigTool.a       # static archive (Linux/macOS)
```

Only `SzConfigTool_*` symbols are exported (149 functions). The header is
[`ffi/include/libSzConfigTool.h`](ffi/include/libSzConfigTool.h); define
`SZCONFIGTOOL_STATIC` before including it when linking the static archive.

### C Example

A complete, tested example is [`ffi/examples/c_ffi_example.c`](ffi/examples/c_ffi_example.c):

```c
#include "libSzConfigTool.h"
#include <stdio.h>

int main(void) {
    const char *config = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";
    SzConfigTool_result result = SzConfigTool_addDataSource(config, "MY_SOURCE");
    if (result.returnCode != 0) {
        fprintf(stderr, "Error: %s\n", SzConfigTool_getLastError());
        return 1;
    }
    printf("%s\n", result.response);   /* modified config JSON */
    SzConfigTool_free(result.response);  /* never free() */
    return 0;
}
```

```bash
cc -o myapp myapp.c -Iffi/include -Ltarget/release -lSzConfigTool \
   -Wl,-rpath,"$PWD/target/release"
./myapp
```

### Memory Management

Every non-NULL `response` is allocated by the library and must be released with
`SzConfigTool_free()`. Strings returned by `SzConfigTool_getLastError*` and
`SzConfigTool_getLibraryVersion` are owned by the library and must not be freed.

### Error Handling

Each call records its outcome in a **per-thread** last-error slot (cleared on
success). `SzConfigTool_getLastError()`, `SzConfigTool_getLastErrorCode()`,
`SzConfigTool_getLastErrorReasonCode()` and `SzConfigTool_getLastErrorDetails()`
return NUL-terminated strings that stay valid until the next `SzConfigTool_*`
call on the same thread; other threads never see or clobber them.

Return codes: `0` success, `-1` null/invalid argument, `-2` library error,
`-3` and below argument-JSON parse/serialization failures. A Rust panic never
crosses the C boundary: it is caught and reported as `-2` with an
`internal panic in <function>: ...` message.

```c
if (result.returnCode != 0) {
    fprintf(stderr, "failed (%lld): %s\n",
            (long long)SzConfigTool_getLastErrorCode(), SzConfigTool_getLastError());
}
```

### Versioning

`SzConfigTool_getLibraryVersion()` returns the crate version (e.g. `"4.4.0-1"`).
`SzConfigTool_getAbiVersion()` returns an integer to compare with the header's
`SZCONFIGTOOL_ABI_VERSION`; it changes only for incompatible changes to existing
declarations. Every language binding exposes the same two values (Python
`library_version()` / `abi_version()`, Java `SzConfigToolVersion`, C#
`SzConfigTool.LibraryVersion` / `AbiVersion`, C++ `LibraryVersion()` / `AbiVersion()`,
TS `libraryVersion()` / `abiVersion()`; see `bindings/CONTRACT.md`).

### JSON Parameter Marshalling

Complex parameters are passed as JSON strings. The `*WithJson` setters are
tri-state per field where noted: an absent key leaves the value, JSON `null`
clears it, a value sets it.

```c
const char *updates = "{\"CONNECT_STR\": null, \"SFUNC_DESC\": \"Updated\"}";
SzConfigTool_result result =
    SzConfigTool_setStandardizeFunctionWithJson(config, "PARSE", updates);
```

### Testing the C ABI

```bash
cargo test -p sz-configtool-ffi   # unit tests, header/export sync, and C programs
```

`ffi/tests/c_abi.rs` builds the library, compiles `ffi/tests/c/test_basic.c` and
the C example with `$CC` (default `cc`), and runs them (Unix). The
`ffi/tests/c/Makefile` / `CMakeLists.txt` build the same test by hand against
`target/release`. `ffi/tests/header_sync.rs` fails if the header and the
exported functions drift apart (names, parameter or return types).

## Workspace and Language Bindings

| Path | Crate / project | Role |
|---|---|---|
| `src/` | `sz_configtool_lib` | the pure Rust library (default workspace member) |
| `ffi/` | `sz-configtool-ffi` | C ABI `libSzConfigTool` (typed exports + `SzConfigTool_invoke`) |
| `api/` | `sz-configtool-api` | `invoke(name, config, args_json)` dispatcher + the manifest (`api/manifest/*.yaml`) |
| `tools/codegen/` | `sz-configtool-codegen` | generates the dispatcher, JSON manifests and every binding's typed wrappers |
| `bindings/python` | pyo3 seam + package | distribution `sz-configtool`, import `sz_configtool` |
| `bindings/jni`, `bindings/java` | JNI seam + Maven project | `io.github.brianmacy.szconfigtool` |
| `bindings/csharp` | .NET (P/Invoke over `SzConfigTool_invoke`) | `Sz.ConfigTool` |
| `bindings/cpp` | header-only C++20 over `SzConfigTool_invoke` | `szconfigtool.hpp` |
| `bindings/node` (+ `trpc/`) | napi-rs seam + TypeScript, tRPC router | npm package |

**Architecture.** Generated typed wrappers (one stateless function per manifest
function) call ONE native seam, `invoke(name, config, args_json)`: the Rust
seams (pyo3, JNI, napi) depend on `sz-configtool-api` directly; C# and C++ call
`SzConfigTool_invoke` in `libSzConfigTool`. Configs are opaque strings; `json`
results are JSON text; errors carry the reason code as their `kind`. The
shared contract is [`bindings/CONTRACT.md`](bindings/CONTRACT.md).

**Manifest workflow.** Edit `api/manifest/<group>.yaml` (and
`api/manifest/conformance/<group>.yaml`), then:

```bash
cargo run -p sz-configtool-codegen            # regenerate dispatcher + all bindings
cargo run -p sz-configtool-codegen -- --check # CI: fail when generated files are stale
cargo test -p sz-configtool-api -p sz-configtool-codegen
```

Field reference: [`api/manifest/schema.md`](api/manifest/schema.md). Output
locations live in `api/manifest/project.yaml` (`paths`, `bindings`).

**Build and test each binding** (details in each binding's README):

| Binding | Commands |
|---|---|
| Rust (all crates) | `cargo test --workspace` |
| Python | `cd bindings/python && python3 -m venv .venv && .venv/bin/pip install maturin pytest ruff && .venv/bin/maturin build --release && .venv/bin/pip install ../../target/wheels/sz_configtool-*.whl && .venv/bin/pytest` |
| Java | `cargo build -p sz-configtool-jni --release && (cd bindings/java && mvn test)` |
| C# | `cargo build -p sz-configtool-ffi --release && dotnet test bindings/csharp/Sz.ConfigTool.sln` |
| C++ | `cargo build -p sz-configtool-ffi --release && cmake -S bindings/cpp -B bindings/cpp/build -G Ninja && cmake --build bindings/cpp/build && ctest --test-dir bindings/cpp/build` (ASan/UBSan: add `-DSZCONFIGTOOL_ENABLE_SANITIZERS=ON`) |
| Node | `cd bindings/node && npm ci && npm run build && npm test`; tRPC: `cd trpc && npm ci && npm run build && npm test` |

## Contributing

Contributions are welcome! Please see `docs/CONTRIBUTING.md` for guidelines.

## License

Apache 2.0 License - See LICENSE file for details.

## See Also

- **CLI Tool:** [sz_configtool](https://github.com/brianmacy/sz_configtool_rust) - Interactive command-line tool using this library
- **Python Version:** [sz_configtool](https://github.com/senzing-garage/sz-python-tools) - Original Python implementation
- **Senzing SDK:** [sz-rust-sdk](https://github.com/brianmacy/sz-rust-sdk) - Full Senzing SDK for Rust

## Versioning

Release versions are `X.Y.Z-N` (first release: `4.4.0-1`, tag `v4.4.0-1`),
mirroring Senzing's own package versions (e.g. `4.4.2-26272`):

- `X.Y` (major.minor) is the Senzing line whose configuration template this
  release is tested against (`4.4` = Senzing 4.4).
- `Z` (patch) and `-N` (a release counter on that line) are this project's.
- Within `4.x` the Rust API is additive only (no removals or signature
  changes); the C ABI has its own `SZCONFIGTOOL_ABI_VERSION`.
- Release candidates use `X.Y.Z-rc.N` (also `-alpha.N` / `-beta.N`); no other
  forms are accepted (`packaging/gates/check-versions.sh`).

**Ordering caveat.** SemVer ecosystems (Cargo, npm, NuGet) treat `-1` as a
prerelease that sorts BEFORE `4.4.0`; Python (`4.4.0.post1`, PEP 440
post-release) and Maven sort it AFTER `4.4.0`. Releases are distributed only
through GitHub Releases and consumed with exact pins (git tag, file path,
exact version), so this matters only to range resolvers (e.g. `^4.4.0` in npm
does not match `4.4.0-1`): pin the exact version.

Release history: [CHANGELOG.md](CHANGELOG.md).

---

**Status:** ✅ Production Ready
**Build:** ✅ 0 errors, 0 warnings
**Tests:** ✅ All passing
**Documentation:** ✅ Complete
