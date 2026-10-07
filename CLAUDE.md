# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the sz-rust-sdk-configtool codebase.

## Project Overview

This is a pure Rust library for manipulating Senzing configuration JSON documents (g2config.json). It provides 169 public functions across 31 modules (126 configuration functions in the binding manifest, 24 groups) for programmatic configuration management without any display logic or CLI dependencies.

### ⚠️ Important Context

**This is an unofficial SDK.** Senzing does not publicly document the meaning, usage, or recommended practices for most configuration functions and parameters beyond basic operations (like adding data sources). Users of this library should have received specific guidance from Senzing support or documentation about:

- When and why to use particular configuration functions
- Appropriate parameter values for their specific use case
- Impact of configuration changes on entity resolution behavior

This library provides the programmatic interface ("how") - proper usage requires Senzing-provided guidance on configuration best practices ("what" and "when").

**Key Characteristics**:

- **Pure Library**: No CLI code, no interactive features, no display logic
- **JSON Manipulation**: All operations are in-memory JSON transformations
- **No SDK Dependencies**: Does not depend on sz-rust-sdk for core operations
- **Minimal Dependencies**: Only serde, serde_json, and anyhow (the root library;
  binding crates add pyo3 / jni / napi only in their own workspace members)
- **C FFI Support**: A separate workspace crate (`ffi/`, package `sz-configtool-ffi`) exports 124 C-compatible `SzConfigTool_*` functions as libSzConfigTool; the root crate itself has no C symbols

## Architecture

### Core Design Principles

1. **Code-Based API**: Public functions use human-readable string codes (e.g., "NAME", "PHONE") instead of numeric IDs
2. **In-Memory JSON Operations**: All functions operate on JSON strings and return modified JSON strings
3. **Pure Functions**: Functions are side-effect free (except for error handling)
4. **Type-Safe Errors**: Custom `SzConfigError` enum for all error conditions
5. **Parameter Alignment**: Function signatures match sz_configtool CLI commands for consistency
6. **No Display Logic**: Zero dependencies on formatting, colors, tables, or output libraries

### API Design Philosophy

**Use Codes, Not IDs:**

- Public APIs accept string codes (e.g., `feature_code: "NAME"`, `element_code: "FIRST_NAME"`)
- Internal ID lookups happen automatically via `helpers::lookup_*_id()` functions
- This eliminates the need for users to manually lookup foreign keys
- Makes code self-documenting and easier to read

**Builder Pattern:**

- Parameter structs provide `new()` constructors and `.with_*()` builder methods
- Example: `SetFeatureElementParams::new("NAME", "FIRST_NAME").with_display_level(1)`
- All optional parameters use `Option<T>` types

**Internal Helper Functions:**

- Truly internal functions should be `pub(crate)`
- Functions needed only by the sibling FFI crate (e.g. ID-based access) are
  `#[doc(hidden)] pub` with a comment saying so, since `pub(crate)` cannot cross
  crates. Example: `delete_comparison_threshold_by_id()`
- Do not add other `#[doc(hidden)]` items

**Response shape convention** (per-function shapes: each manifest entry's `notes`):

- `get_*` and record-returning `add_*` return the stored row (on-disk keys —
  what you would `set`). Grandfathered summary gets, pinned by conformance:
  `get_feature`, `get_element`, `get_fragment`, `get_rule`. `add_fragment` /
  `add_rule` return the assigned id; `add_config_section_field` returns counts.
- `list_*` return a code-resolved SUMMARY (ids resolved to codes) in a
  deterministic order, not the raw rows. Raw rows: `get_config_section("CFG_...")`
  (section name is case-sensitive), or a raw/resolved pair
  (`list_behavior_overrides` raw vs `list_behavior_overrides_resolved`).
  `list_feature_comparisons` and `list_feature_classes` also return raw rows.
- Known limitation: `list_expression_calls`, `list_comparison_calls` and
  `list_distinct_calls` give `elementList` as bare element codes (ordered by
  BOM `EXEC_ORDER`), omitting stored BOM columns (`FTYPE_ID`, `EXEC_ORDER`,
  and for expression `FELEM_REQ`). The engine reads these columns; use
  `get_config_section("CFG_EFBOM" | "CFG_CFBOM" | "CFG_DFBOM")` for the raw
  rows (G2's `sz_configtool` CLI reads the raw rows for its call views).
- No `describe_*`/view layer in the library or bindings; further presentation
  belongs to the CLI.

### Workspace Layout

- `.` — `sz_configtool_lib`, the pure Rust library (crate-type `lib`; default
  workspace member, so plain `cargo test` covers only it)
- `ffi/` — `sz-configtool-ffi`, the C ABI (`[lib] name = "SzConfigTool"`,
  crate-type `cdylib` + `staticlib`):
  - `ffi/src/lib.rs` — the 124 `extern "C"` exports
  - `ffi/include/libSzConfigTool.h` — the C header
  - `ffi/tests/header_sync.rs` — header vs export drift check
  - `ffi/tests/c_abi.rs` — builds and runs `ffi/tests/c/test_basic.c` and
    `ffi/examples/c_ffi_example.c` against the built library (Unix)
- `api/` — `sz-configtool-api`: `invoke(name, config, args_json)` (generated
  `src/dispatch_gen.rs`), arg converters (`src/convert.rs`), the shared
  `validation_details_json`; `api/manifest/*.yaml` is the hand-maintained
  description of every function (+ `conformance/`, `generated/*.json`)
- `tools/codegen/` — `sz-configtool-codegen`: validates the manifest and writes
  every generated file; per-language generators in `src/lang/<lang>.rs`; all
  output locations come from `api/manifest/project.yaml` (`paths`, `bindings`)
- `bindings/` — generated typed wrappers over ONE native seam
  (`bindings/CONTRACT.md`): `python` (pyo3; distribution `sz-configtool`,
  import `sz_configtool`), `jni` + `java`, `node` (napi-rs,
  + `trpc/`) depend on `sz-configtool-api`; `csharp` and `cpp` call
  `SzConfigTool_invoke` in `libSzConfigTool`

### Binding Manifest Workflow

1. Edit `api/manifest/<group>.yaml` and `api/manifest/conformance/<group>.yaml`
   (field reference: `api/manifest/schema.md`; how-to: `api/manifest/README.md`).
2. `cargo run -p sz-configtool-codegen` — regenerates the dispatcher, JSON
   manifests and all five bindings' wrappers (never hand-edit generated files).
3. `cargo test -p sz-configtool-api -p sz-configtool-codegen`, then each
   binding's suite (below). `-- --check` fails when anything is stale.

A new root-library `pub fn` must be mapped in the manifest or excluded with a
reason (the drift test enforces complete coverage).

### Module Organization

```
src/
├── lib.rs              # Root module, re-exports
├── error.rs            # SzConfigError types
├── helpers.rs          # Core utilities (ID generation, array operations)
├── attributes.rs       # CFG_ATTR operations (5 functions)
├── datasources.rs      # CFG_DSRC operations (5 functions)
├── elements.rs         # CFG_FELEM operations (10 functions)
├── features.rs         # Feature operations (16 functions)
├── thresholds.rs       # Threshold operations (12 functions)
├── config_sections.rs  # G2_CONFIG section operations
├── fragments.rs        # CFG_ERFRAG operations
├── generic_plans.rs    # CFG_GPLAN operations
├── rules.rs            # CFG_ERRULE operations
├── search_profiles.rs  # CFG_SPROFILE operations (4 functions; INGEST/SEARCH delete-protected)
├── system_params.rs    # System parameters
├── versioning.rs       # Version management
├── calls/              # Call management (24 functions)
│   ├── mod.rs
│   ├── standardize.rs  # CFG_SFCALL, CFG_SBOM
│   ├── expression.rs   # CFG_EFCALL, CFG_EFBOM
│   ├── comparison.rs   # CFG_CFCALL, CFG_CFBOM
│   └── distinct.rs     # CFG_DFCALL, CFG_DFBOM
└── functions/          # Function management (23 functions)
    ├── mod.rs
    ├── standardize.rs  # CFG_SFUNC
    ├── expression.rs   # CFG_EFUNC
    ├── comparison.rs   # CFG_CFUNC
    └── distinct.rs     # CFG_DFUNC
```

## Development Standards

### Code Quality Requirements

- **Rust Edition**: 2024
- **Rust Version**: 1.88+ (MSRV, checked in CI; toolchain pinned in `rust-toolchain.toml`)
- **Clippy**: Must pass with `--workspace --all-targets --all-features -- -D warnings`
- **Formatting**: Run `cargo fmt` before committing
- **Security**: Must pass `cargo deny check`
- **Tests**: All tests must pass with `cargo test --workspace`

### Function Signature Pattern

All public functions follow this pattern:

````rust
/// Brief description of what the function does.
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `param1` - Description of parameter
/// * `param2` - Optional parameter (use None to skip)
///
/// # Returns
/// * `Ok(String)` - Modified configuration JSON on success
/// * `Err(SzConfigError)` - Error with descriptive message
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::module_name::function_name;
/// let config = r#"{ ... }"#;
/// let modified = function_name(&config, "value")?;
/// ```
pub fn function_name(
    config_json: &str,
    param1: &str,
    param2: Option<&str>,
) -> Result<String> {
    // Implementation
}
````

### Error Handling

Use the `SzConfigError` enum defined in `src/error.rs`:

```rust
pub enum SzConfigError {
    JsonParse(String),
    NotFound(String, String),      // (entity_type, identifier)
    AlreadyExists(String, String), // (entity_type, identifier)
    InvalidInput(String),
    MissingField(String),
    DependencyExists(String),
    InternalError(String),
}
```

### Testing Requirements

1. **Unit Tests**: Each module should have inline unit tests

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_function_name() {
           let config = r#"{ ... }"#;
           let result = function_name(&config, "value");
           assert!(result.is_ok());
       }
   }
   ```

2. **Integration Tests**: Place in `tests/` directory
3. **Doc Tests**: Include working examples in documentation
4. **Example Programs**: Create examples in `examples/` directory

## C FFI Guidelines

The FFI crate (`ffi/src/lib.rs`) provides C-compatible wrappers for library functions.

### FFI Design Patterns

1. **Return Structure**: All FFI functions return `SzConfigTool_result`:

   ```rust
   #[repr(C)]
   pub struct SzConfigTool_result {
       pub response: *mut c_char, // caller frees with SzConfigTool_free
       pub returnCode: i64,       // 0 = success, negative = error
   }
   ```

2. **Memory Management**: Rust allocates, C must free using `SzConfigTool_free()`

3. **Error Handling**: The last error (message, code, reason code, details) is
   stored per thread in a `thread_local!` `RefCell`, as NUL-terminated
   `CString`s. Pointers returned by `SzConfigTool_getLastError*` are valid until
   the next `SzConfigTool_*` call on the same thread.

4. **Panic Safety**: Every `extern "C"` body is wrapped in
   `ffi_guard("SzConfigTool_name", || { ... })`, which uses `catch_unwind` to
   turn a panic into returnCode -2 (NULL/-2 for other return types) with an
   `internal panic in ...` last error. A unit test fails if any export is not
   wrapped. Never set `panic = "abort"`.

5. **JSON Marshalling**: Complex parameters passed as JSON strings

6. **Helper Macros**: `handle_result!` converts a `Result<String>` into an
   `SzConfigTool_result`; `ffi_required_str!` reads a required C string arg:
   ```rust
   #[unsafe(no_mangle)]
   pub extern "C" fn SzConfigTool_functionName(
       config_json: *const c_char,
       param: *const c_char,
   ) -> SzConfigTool_result {
       ffi_guard("SzConfigTool_functionName", || {
           let config = ffi_required_str!(config_json, "config_json");
           let param = ffi_required_str!(param, "param");
           handle_result!(sz_configtool_lib::module::function_name(config, param))
       })
   }
   ```

7. **Versioning**: `SzConfigTool_getLibraryVersion()` returns the workspace
   version; `SzConfigTool_getAbiVersion()` returns `SZCONFIGTOOL_ABI_VERSION`
   (one definition, `sz_configtool_api::ABI_VERSION`, mirrored by the header's
   `#define`; bump both only for incompatible changes: a removed or changed
   declaration — e.g. 2 removed the SSN_LAST4 hash exports).
8. **Return codes**: new exports use only -1 (NULL / invalid UTF-8) and -2
   via `handle_result!` / `set_error_from` (reason code set). The header's
   "Return codes" section lists the legacy irregular families;
   `ffi/tests/return_codes.rs` fails when they drift.

### FFI Implementation Checklist

When adding new FFI functions:

- [ ] Verify Rust function signature first
- [ ] Wrap the body in `ffi_guard("<exact fn name>", || { ... })`
- [ ] Handle NULL pointers for optional parameters
- [ ] Declare it in `ffi/include/libSzConfigTool.h` with `SZCONFIGTOOL_API`
      (`ffi/tests/header_sync.rs` fails otherwise)
- [ ] Add documentation comment in header
- [ ] Test memory management (no leaks)

## Building and Testing

### Build Commands

```bash
# Build Rust library
cargo build --lib

# Build release (optimized)
cargo build --lib --release

# Build the C library (shared + static)
cargo build -p sz-configtool-ffi --release
# Output: target/release/libSzConfigTool.{so,dylib,a}, SzConfigTool.dll

# Build examples
cargo build --examples
```

### Testing Commands

```bash
# Run all tests (library + FFI crate + C ABI tests)
cargo test --workspace

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc --workspace
```

### Binding Tests (real native library, no mocks)

```bash
# Python (venv anywhere; README in bindings/python)
cd bindings/python && python3 -m venv .venv && .venv/bin/pip install maturin pytest ruff \
  && .venv/bin/maturin build --release && .venv/bin/pip install ../../target/wheels/sz_configtool-*.whl \
  && .venv/bin/pytest
# Java
cargo build -p sz-configtool-jni --release && (cd bindings/java && mvn test)
# C#
cargo build -p sz-configtool-ffi --release && dotnet test bindings/csharp/Sz.ConfigTool.sln
# C++ (add -DSZCONFIGTOOL_ENABLE_SANITIZERS=ON in a separate build dir for ASan/UBSan)
cargo build -p sz-configtool-ffi --release && cmake -S bindings/cpp -B bindings/cpp/build -G Ninja \
  && cmake --build bindings/cpp/build && ctest --test-dir bindings/cpp/build
# Node + tRPC
cd bindings/node && npm ci && npm run build && npm test && cd trpc && npm ci && npm run build && npm test
```

### Quality Checks

```bash
# Format code
cargo fmt

# Lint code (must pass)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Security audit
cargo deny check

# Generate documentation
cargo doc --no-deps --open
```

## Relationship to CLI Tool

This library is used by the [sz_configtool](https://github.com/brianmacy/sz_configtool_rust) CLI tool. The CLI tool:

- Adds interactive shell features (rustyline)
- Adds display formatting (tables, JSON, JSONL)
- Adds output paging (less, minus)
- Adds colorization (owo-colors)
- Provides user-facing command interface

**Separation of Concerns**:

- **Library**: Pure business logic, JSON manipulation
- **CLI**: User interface, display, interactivity

## Dependencies

### Production Dependencies

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["preserve_order"] }
anyhow = "1.0"
```

### Development Dependencies

```toml
tempfile = "3.24"  # For temporary files in tests
```

**No other dependencies should be added** without strong justification. This library must remain minimal and focused.

## Release Process

Versions are `X.Y.Z-N` (`X.Y` = Senzing line, `-N` = release counter; first
release `4.4.0-1`); see README "Versioning" and `packaging/README.md`.

1. **Update Version**: workspace version in `Cargo.toml` + binding manifests and npm lockfiles (`packaging/README.md`, "Cutting a release")
2. **Update CHANGELOG**: Document changes in `CHANGELOG.md`
3. **Run Quality Checks**: Ensure all tests pass, clippy clean, deny pass, `packaging/gates/check-versions.sh v<version>`
4. **Create Git Tag**: `git tag -a v4.4.0-1 -m "Release v4.4.0-1"`
5. **Push to GitHub**: `git push origin main && git push origin v4.4.0-1` (the tag push publishes the GitHub Release)

Nothing is published to crates.io or other registries; Rust users depend on
the git tag (`tag = "v4.4.0-1"`).

## Common Tasks

### Adding a New Function

1. Implement in appropriate module (e.g., `src/datasources.rs`)
2. Add rustdoc comments with examples
3. Export from module in `src/lib.rs`
4. Add unit tests in module
5. Add integration test in `tests/`
6. Update module count in README if needed
7. Add C FFI wrapper if needed in `ffi/src/lib.rs`
8. Update header file `ffi/include/libSzConfigTool.h`
9. Add a manifest entry + conformance case (`api/manifest/`), then
   `cargo run -p sz-configtool-codegen` to regenerate every binding

### Adding a New Module

1. Create `src/new_module.rs`
2. Implement functions following patterns
3. Add module declaration in `src/lib.rs`
4. Add comprehensive tests
5. Document module in README
6. Update function counts

### Fixing a Bug

1. Add a failing test that reproduces the bug
2. Fix the implementation
3. Verify test passes
4. Check for similar issues in other modules
5. Update CHANGELOG

## Documentation Standards

- **Public Functions**: Must have rustdoc comments
- **Examples**: Include working code examples
- **Parameters**: Document all parameters
- **Returns**: Document return values and errors
- **Module Docs**: Add module-level documentation
- **README**: Keep synchronized with code

## Version Compatibility

- **Semantic Versioning**: Follow semver strictly
- **Breaking Changes**: Require major version bump
- **FFI Stability**: C FFI interface is very stable, avoid breaking changes
- **Rust API**: Can evolve, but avoid gratuitous changes

## Performance Considerations

- **JSON Parsing**: Parse config only once per operation
- **String Cloning**: Minimize unnecessary clones
- **Allocations**: Reuse allocations where possible
- **Error Handling**: Use Result, avoid panics

## Security Considerations

- **Input Validation**: Validate all inputs before processing
- **JSON Injection**: Prevent malformed JSON from corrupting config
- **Memory Safety**: Rust prevents most issues, but be careful with FFI
- **Dependency Security**: Run `cargo deny check` regularly

## Future Enhancements

- [ ] Add C FFI wrappers for library functions that have none (e.g. search profiles)
- [ ] Add Python bindings (ctypes or PyO3)
- [ ] Improve test coverage to >80%
- [ ] Add benchmarking suite
- [ ] Config validation functions
- [ ] Config diff and merge operations
- [ ] Schema migration helpers

## Contact

- **Repository**: https://github.com/brianmacy/sz-rust-sdk-configtool
- **Issues**: https://github.com/brianmacy/sz-rust-sdk-configtool/issues
- **CLI Tool**: https://github.com/brianmacy/sz_configtool_rust
