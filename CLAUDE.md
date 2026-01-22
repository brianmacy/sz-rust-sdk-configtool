# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the sz-rust-sdk-configtool codebase.

## Project Overview

This is a pure Rust library for manipulating Senzing configuration JSON documents (g2config.json). It provides 147 functions across 30 modules for programmatic configuration management without any display logic or CLI dependencies.

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
- **Minimal Dependencies**: Only serde, serde_json, and anyhow
- **C FFI Support**: Includes 98 C-compatible FFI functions for cross-language integration

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
- Functions needing ID-based access (e.g., for FFI) should be `pub(crate)`, not `pub`
- Example: `pub(crate) fn delete_comparison_threshold_by_id()` for FFI use only
- Use `#[doc(hidden)]` sparingly; prefer `pub(crate)` for truly internal functions

### Module Organization

```
src/
├── lib.rs              # Root module, re-exports
├── error.rs            # SzConfigError types
├── helpers.rs          # Core utilities (ID generation, array operations)
├── attributes.rs       # CFG_ATTR operations (8 functions)
├── datasources.rs      # CFG_DSRC operations (7 functions)
├── elements.rs         # CFG_FELEM operations (8 functions)
├── features.rs         # Feature operations (24 functions)
├── thresholds.rs       # Threshold operations (6 functions)
├── config_sections.rs  # G2_CONFIG section operations
├── fragments.rs        # CFG_ERFRAG operations
├── generic_plans.rs    # CFG_GPLAN operations
├── hashes.rs           # Hash management
├── rules.rs            # CFG_ERRULE operations
├── system_params.rs    # System parameters
├── versioning.rs       # Version management
├── ffi.rs              # C FFI wrapper (294KB, 98 functions)
├── calls/              # Call management (32 functions)
│   ├── mod.rs
│   ├── standardize.rs  # CFG_SFCALL, CFG_SBOM
│   ├── expression.rs   # CFG_EFCALL, CFG_EFBOM
│   ├── comparison.rs   # CFG_CFCALL, CFG_CFBOM
│   └── distinct.rs     # CFG_DFCALL, CFG_DFBOM
└── functions/          # Function management (28 functions)
    ├── mod.rs
    ├── standardize.rs  # CFG_SFUNC
    ├── expression.rs   # CFG_EFUNC
    ├── comparison.rs   # CFG_CFUNC
    ├── distinct.rs     # CFG_DFUNC
    ├── matching.rs     # CFG_RTYPE
    ├── scoring.rs      # Scoring functions (stubs)
    ├── candidate.rs    # Candidate functions (stubs)
    └── validation.rs   # Validation functions (stubs)
```

## Development Standards

### Code Quality Requirements

- **Rust Edition**: 2024
- **Rust Version**: 1.85+
- **Clippy**: Must pass with `--all-targets --all-features -- -D warnings`
- **Formatting**: Run `cargo fmt` before committing
- **Security**: Must pass `cargo deny check`
- **Tests**: All tests must pass with `cargo test`

### Function Signature Pattern

All public functions follow this pattern:

```rust
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
```

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

The FFI layer (`src/ffi.rs`) provides C-compatible wrappers for library functions.

### FFI Design Patterns

1. **Return Structure**: All FFI functions return `SzConfigTool_result`:
   ```rust
   #[repr(C)]
   pub struct SzConfigTool_result {
       pub return_code: i32,  // 0 = success, 1 = error
       pub response: *mut c_char,
   }
   ```

2. **Memory Management**: Rust allocates, C must free using `SzConfigTool_free()`

3. **Error Handling**: Thread-local error storage retrieved with `SzConfigTool_getLastError()`

4. **JSON Marshalling**: Complex parameters passed as JSON strings

5. **Helper Macro**: Use `handle_result!` for simple Result<String> returns:
   ```rust
   #[no_mangle]
   pub extern "C" fn SzConfigTool_functionName(
       config_json: *const c_char,
       param: *const c_char,
   ) -> SzConfigTool_result {
       handle_result!(|| {
           let config = unsafe { CStr::from_ptr(config_json) }.to_str()?;
           let param_str = unsafe { CStr::from_ptr(param) }.to_str()?;
           module::function_name(config, param_str)
       })
   }
   ```

### FFI Implementation Checklist

When adding new FFI functions:
- [ ] Verify Rust function signature first
- [ ] Use correct return type (String vs tuple)
- [ ] Handle NULL pointers for optional parameters
- [ ] Update `include/libSzConfigTool.h` with declaration
- [ ] Add documentation comment in header
- [ ] Test memory management (no leaks)
- [ ] Verify thread safety

## Building and Testing

### Build Commands

```bash
# Build Rust library
cargo build --lib

# Build release (optimized)
cargo build --lib --release

# Build shared library for C FFI
cargo build --lib --release
# Output: target/release/libsz_configtool_lib.{so,dylib,dll}

# Build examples
cargo build --examples
```

### Testing Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Quality Checks

```bash
# Format code
cargo fmt

# Lint code (must pass)
cargo clippy --all-targets --all-features -- -D warnings

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

1. **Update Version**: Update version in `Cargo.toml`
2. **Update CHANGELOG**: Document changes in `CHANGELOG.md`
3. **Run Quality Checks**: Ensure all tests pass, clippy clean, deny pass
4. **Create Git Tag**: `git tag -a v0.x.0 -m "Release v0.x.0"`
5. **Push to GitHub**: `git push origin main && git push origin v0.x.0`
6. **Publish to crates.io** (future): `cargo publish`

## Common Tasks

### Adding a New Function

1. Implement in appropriate module (e.g., `src/datasources.rs`)
2. Add rustdoc comments with examples
3. Export from module in `src/lib.rs`
4. Add unit tests in module
5. Add integration test in `tests/`
6. Update module count in README if needed
7. Add C FFI wrapper if needed in `src/ffi.rs`
8. Update header file `include/libSzConfigTool.h`

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

- [ ] Complete remaining C FFI functions (22 missing)
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
