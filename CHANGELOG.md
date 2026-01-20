# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-20

### Added

#### Core Library
- Initial release of sz_configtool_lib as standalone SDK
- 147 functions across 30 modules for Senzing configuration manipulation
- Pure Rust implementation with no SDK dependencies for core operations
- Type-safe error handling with `SzConfigError` enum
- Comprehensive rustdoc documentation for all public functions

#### Modules
- **Data Management** (15 functions)
  - `datasources` - Data source CRUD operations (CFG_DSRC)
  - `attributes` - Attribute management (CFG_ATTR)
- **Feature Management** (37 functions)
  - `features` - Feature operations with elements, comparisons, and distinct calls
  - `elements` - Element operations (CFG_FELEM)
  - Feature types, behaviors, and candidates
- **Configuration** (25 functions)
  - `thresholds` - Comparison and generic thresholds
  - `rules` - Entity resolution rules (CFG_ERRULE)
  - `fragments` - Rule fragments (CFG_ERFRAG)
  - `generic_plans` - Generic plan management (CFG_GPLAN)
  - `hashes` - Name and SSN hash management
- **System Management** (12 functions)
  - `config_sections` - G2_CONFIG section manipulation
  - `system_params` - System parameter operations
  - `versioning` - Version management
- **Functions** (28 functions)
  - `functions/standardize` - Standardization functions (CFG_SFUNC)
  - `functions/expression` - Expression functions (CFG_EFUNC)
  - `functions/comparison` - Comparison functions (CFG_CFUNC)
  - `functions/distinct` - Distinct functions (CFG_DFUNC)
  - `functions/matching` - Matching functions (CFG_RTYPE)
- **Calls** (32 functions)
  - `calls/standardize` - Standardize calls with BOM (CFG_SFCALL, CFG_SBOM)
  - `calls/expression` - Expression calls with BOM (CFG_EFCALL, CFG_EFBOM)
  - `calls/comparison` - Comparison calls with BOM (CFG_CFCALL, CFG_CFBOM)
  - `calls/distinct` - Distinct calls with BOM (CFG_DFCALL, CFG_DFBOM)

#### C FFI Interface
- 98 C-compatible FFI functions in `src/ffi.rs` (294KB)
- C header file at `include/libSzConfigTool.h`
- Thread-safe error handling for FFI calls
- Memory management utilities (`SzConfigTool_free`)
- JSON parameter marshalling for complex types
- Support for shared library builds (cdylib, staticlib)

#### Documentation
- Comprehensive README with installation, usage, and examples
- CLAUDE.md with development guidelines and architecture
- C FFI usage guide in README
- Module-level documentation for all public APIs
- Working code examples in rustdoc

#### Build Configuration
- Rust 2024 edition support
- Multi-platform build support (Linux, macOS, Windows)
- cargo-deny configuration for security auditing
- Minimal dependencies (serde, serde_json, anyhow)

### Technical Details

**Dependencies**:
- `serde = "1.0"` with derive feature
- `serde_json = "1.0"` with preserve_order feature
- `anyhow = "1.0"` for error handling

**Build Targets**:
- `lib` - Rust library
- `cdylib` - C dynamic library (.so, .dylib, .dll)
- `staticlib` - Static library

**Rust Version**: 1.85+

**License**: Apache-2.0

### Notes

This is the initial extraction from the [sz_configtool_rust](https://github.com/brianmacy/sz_configtool_rust) CLI tool repository. The library provides the core JSON manipulation logic that powers the CLI tool, now available as a standalone SDK for use in other projects and languages.

The library maintains 100% API compatibility with the sz_configtool CLI commands, ensuring consistent behavior across both the library and CLI interfaces.

## [Unreleased]

### Planned for v0.2.0
- [ ] Additional FFI functions (22 remaining for 100% coverage)
- [ ] Python bindings (ctypes or PyO3)
- [ ] Improved test coverage (target >80%)
- [ ] Performance benchmarking suite

### Planned for v0.3.0
- [ ] Config validation functions
- [ ] Config diff and merge operations
- [ ] Import/export utilities
- [ ] Schema migration helpers

---

[0.1.0]: https://github.com/brianmacy/sz-rust-sdk-configtool/releases/tag/v0.1.0
