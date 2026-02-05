# Contributing to sz-rust-sdk-configtool

Thank you for your interest in contributing to the sz-rust-sdk-configtool! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Documentation](#documentation)
- [License](#license)

## Code of Conduct

Be respectful, professional, and collaborative. We value contributions from everyone and strive to create a welcoming environment.

## Getting Started

### Prerequisites

- Rust 1.85 or later
- Git
- cargo-deny (for security checks): `cargo install cargo-deny`
- cargo-audit (for vulnerability scanning): `cargo install cargo-audit`

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/sz-rust-sdk-configtool.git
   cd sz-rust-sdk-configtool
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/brianmacy/sz-rust-sdk-configtool.git
   ```

## Development Environment

### Build the Library

```bash
# Build debug version
cargo build --lib

# Build release version
cargo build --lib --release

# Build examples
cargo build --examples
```

### Run Tests

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

## Making Changes

### Branch Strategy

- Create a feature branch from `main`:

  ```bash
  git checkout -b feature/your-feature-name
  ```

- Use descriptive branch names:
  - `feature/add-xyz` for new features
  - `fix/issue-123` for bug fixes
  - `docs/improve-readme` for documentation
  - `refactor/cleanup-module` for refactoring

### Commit Messages

Write clear, descriptive commit messages:

```
Short summary (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.
Explain what changed and why, not how.

- Bullet points are okay
- Use present tense: "Add feature" not "Added feature"
- Reference issues: "Fixes #123" or "Related to #456"
```

Example:

```
Add support for config validation functions

Implement new validation module with functions to check:
- Required fields presence
- Data type correctness
- Cross-reference integrity

Fixes #42
```

## Testing

### Test Requirements

1. **Unit Tests**: Every module should have unit tests

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_add_data_source() {
           let config = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;
           let result = add_data_source(config, "TEST", None, None, None);
           assert!(result.is_ok());
       }
   }
   ```

2. **Integration Tests**: Add tests in `tests/` directory

   ```rust
   // tests/integration_test.rs
   use sz_configtool_lib::datasources;

   #[test]
   fn test_full_workflow() {
       // Test complete workflows
   }
   ```

3. **Doc Tests**: Include working examples in documentation
   ````rust
   /// Add a data source to configuration.
   ///
   /// # Example
   /// ```
   /// use sz_configtool_lib::datasources::add_data_source;
   /// let config = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;
   /// let result = add_data_source(config, "TEST", None, None, None);
   /// assert!(result.is_ok());
   /// ```
   pub fn add_data_source(/* ... */) -> Result<String> {
       // Implementation
   }
   ````

### Test Coverage

- Aim for >80% code coverage
- Test success cases and error cases
- Test edge cases and boundary conditions
- Test with valid and invalid JSON

## Code Quality

### Required Checks

Before submitting a PR, ensure all these pass:

```bash
# 1. Format code
cargo fmt

# 2. Check formatting
cargo fmt --check

# 3. Run clippy (must pass with no warnings)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Run tests
cargo test

# 5. Check documentation builds
cargo doc --no-deps

# 6. Security audit
cargo deny check

# 7. Build release
cargo build --lib --release
```

### Clippy Warnings

- Fix ALL clippy warnings
- Do not use `#[allow(clippy::...)]` without justification
- Prefer idiomatic Rust over clever code

### Formatting

- Use `cargo fmt` with default settings
- 4 spaces for indentation (no tabs)
- 100 character line limit (soft)
- Follow Rust style guidelines

## Pull Request Process

### Before Submitting

1. Sync with upstream:

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Run all quality checks (see Code Quality section)

3. Update documentation:
   - README.md if adding features
   - CHANGELOG.md with your changes
   - Module documentation
   - Function documentation

4. Update tests:
   - Add tests for new functionality
   - Update tests for changed functionality
   - Ensure all tests pass

### Submitting the PR

1. Push your branch:

   ```bash
   git push origin feature/your-feature-name
   ```

2. Create PR on GitHub with:
   - Clear title describing the change
   - Detailed description of what changed and why
   - Reference related issues
   - Screenshots/examples if applicable

3. PR description template:

   ```markdown
   ## Description

   Brief description of changes

   ## Type of Change

   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing

   - [ ] Unit tests added/updated
   - [ ] Integration tests added/updated
   - [ ] All tests pass locally

   ## Checklist

   - [ ] Code follows project style guidelines
   - [ ] Self-review completed
   - [ ] Documentation updated
   - [ ] CHANGELOG.md updated
   - [ ] No new warnings from clippy
   - [ ] cargo fmt applied
   - [ ] cargo deny check passes

   ## Related Issues

   Fixes #123
   ```

### Review Process

- Maintainers will review your PR
- Address feedback and comments
- Make requested changes in new commits
- Once approved, PR will be merged

## Coding Standards

### Rust Edition and Version

- **Edition**: 2024
- **Rust Version**: 1.85+
- Use modern Rust features appropriately

### Function Signatures

Follow the standard pattern:

````rust
/// Brief description of function.
///
/// Detailed explanation if needed.
///
/// # Arguments
/// * `config_json` - The configuration JSON string
/// * `param1` - Description of parameter
/// * `param2` - Optional parameter (use None to skip)
///
/// # Returns
/// * `Ok(String)` - Modified configuration on success
/// * `Err(SzConfigError)` - Error with descriptive message
///
/// # Errors
/// Returns error if:
/// - JSON parsing fails
/// - Entity not found
/// - Entity already exists
///
/// # Example
/// ```no_run
/// use sz_configtool_lib::module::function;
/// let config = r#"{ ... }"#;
/// let result = function(&config, "value")?;
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

- Use `Result<T, SzConfigError>` for all fallible operations
- Use descriptive error messages
- Provide context in errors:
  ```rust
  Err(SzConfigError::NotFound(
      "Data Source".to_string(),
      dsrc_code.to_string()
  ))
  ```

### Naming Conventions

- **Functions**: `snake_case` - `add_data_source`
- **Types**: `PascalCase` - `SzConfigError`
- **Constants**: `SCREAMING_SNAKE_CASE` - `MAX_RETRY_COUNT`
- **Modules**: `snake_case` - `datasources`

### Documentation

- All public functions must have rustdoc comments
- Include working code examples
- Document all parameters and return values
- Explain error conditions
- Add module-level documentation

### No Display Logic

This is a pure library with no display formatting:

- No dependencies on formatting libraries (tabled, prettytable, etc.)
- No color output (colored, owo-colors, etc.)
- No interactive features (rustyline, etc.)
- Return data, don't print it

### Minimal Dependencies

- Keep dependencies minimal
- Justify any new dependency
- Current dependencies:
  - `serde` - JSON serialization (required)
  - `serde_json` - JSON parsing (required)
  - `anyhow` - Error handling (required)

## Documentation

### Types of Documentation

1. **README.md**: User-facing quick start guide
2. **API.md**: Complete API reference
3. **FFI_GUIDE.md**: C FFI usage guide
4. **CLAUDE.md**: Developer/AI guidance
5. **CHANGELOG.md**: Version history

### Updating Documentation

When adding features:

- Update README.md with usage examples
- Add function to API.md reference
- Update function counts in README
- Add entry to CHANGELOG.md
- Update FFI_GUIDE.md if adding FFI functions

### Documentation Style

- Use clear, concise language
- Include working code examples
- Use markdown formatting consistently
- Keep line length reasonable (<100 chars)

## C FFI Contributions

### Adding FFI Functions

1. Implement Rust function first
2. Add FFI wrapper in `src/ffi.rs`:
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
3. Add declaration to `include/libSzConfigTool.h`
4. Document in FFI_GUIDE.md
5. Update FFI function count in README

### FFI Guidelines

- Handle NULL pointers for optional parameters
- Use `handle_result!` macro for simple functions
- Store errors in thread-local storage
- Ensure thread safety
- Test memory management (no leaks)

## Performance Considerations

- Profile before optimizing
- JSON parsing is typically the bottleneck
- Minimize string allocations
- Reuse allocations where reasonable
- Don't sacrifice readability for minor gains

## Security

### Reporting Vulnerabilities

- Email security@senzing.com for security issues
- Do not open public issues for vulnerabilities
- Allow time for fix before public disclosure

### Security Practices

- Run `cargo deny check` regularly
- Keep dependencies updated
- Validate all inputs
- Prevent JSON injection attacks
- Use safe Rust (no unsafe unless necessary)

## Release Process

Maintainers follow this process for releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.x.0 -m "Release v0.x.0"`
4. Push: `git push origin main && git push origin v0.x.0`
5. Create GitHub release
6. Publish to crates.io (when ready)

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.

## Questions?

- Open an issue for questions
- Check existing issues and PRs first
- Be patient and respectful

## Thank You!

Your contributions make this project better for everyone. We appreciate your time and effort!
