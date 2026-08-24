# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2] - 2026-08-24

Patch: fixes a call-element delete regression found during the CLI's v0.6.x adoption, verified
against the stock config and Python `sz_configtool` 4.4.0. Gates green: 329 test cases, clippy/fmt
clean.

### Fixed

- **`delete_{comparison,expression,distinct}_call_element` can now disambiguate an element that
  appears under multiple features in one call.** The v0.6.0 redesign (#40) derived `EXEC_ORDER` from
  `(call, element)` alone, so when one call carried the same `FELEM_ID` under multiple `FTYPE_ID`s it
  errored `Ambiguous …` instead of deleting the right row. The **stock config** ships exactly this
  (`EFCALL_ID 97` / `TOKENIZED_NM` under both `GROUP_ASSOCIATION` and `EMPLOYER`), so
  `deleteExpressionCallElement` was broken on shipped data. The three delete functions (and their FFI
  wrappers) now take an optional **`element_feature`** which, when supplied, resolves the collision to
  the feature-matched BOM row — mirroring Python's `(call_id, FTYPE_ID, FELEM_ID)` addressing. When
  `(call, element)` is unambiguous the feature is optional (`None` / `NULL`).

### Changed (API/FFI — additive optional parameter)

- `delete_comparison_call_element`, `delete_expression_call_element`, `delete_distinct_call_element`
  gain a trailing `element_feature: Option<&str>`.
- The FFI wrappers `SzConfigTool_delete{Comparison,Distinct,Expression}CallElement` gain a trailing
  **nullable** `element_feature` C-string argument (`include/libSzConfigTool.h` updated). Pass `NULL`
  when unambiguous.
- The `deleteComparisonCallElement` / `deleteDistinctCallElement` script commands accept an optional
  `elementFeature` parameter.

## [0.6.1] - 2026-08-24

Patch: rule-validation parity fixes found during the downstream CLI's adoption of v0.6.0,
verified against the Python `sz_configtool` reference (`/opt/senzing/er/bin/sz_configtool`, 4.4.0)
and coordinated with the CLI. All three tighten rule validation (reject inputs 0.6.0 accepted);
the CLI already enforced all three locally, confirmed none break it, and needs them to re-delegate
rule validation to the SDK. Gates green: 328 test cases, clippy/fmt clean.

### Fixed

- **`set_rule`/`add_rule`: a blank `fragment` code is now rejected** (`Fragment "" not found`),
  matching Python's unconditional `lookupFragment("")`. v0.6.0 silently accepted a blank fragment
  as a no-op (the `!c.is_empty()` skip in `validate_fragment_code`) — reverting that 0.6.0
  behavioural note. A blank **disqualifier** is still accepted (nullable — Python guards its lookup
  with `if record.get("DISQ_ERFRAG_CODE"):`). (#45)

### Changed (breaking — behaviour; Python parity, folded in with #45)

- **`add_rule` now requires a fragment.** An absent `QUAL_ERFRAG_CODE` is a `MissingField`
  (`Fragment is required`), matching Python `do_addRule` which lists FRAGMENT as a required param.
  v0.6.0 accepted a rule with no fragment.
- **`RESOLVE="Yes"` now requires a non-zero tier.** An absent tier or `0` is rejected
  (`A tier … must be specified`), matching Python `validateRule` (`if not tier`). This reverses the
  v0.6.0 decision D15 to omit the check, which was made on the incorrect premise that the Python
  reference did not enforce it — it does.

## [0.6.0] - 2026-08-22

Coordinated breaking release resolving the SDK audit (issues #32–#43). Delivered in six
reviewed waves; every public-API and behavioural change is listed below. Test suite grew
from ~90 to 216 unit tests + 80 doc-tests (all green; clippy `-D warnings`, `cargo deny`,
`cargo fmt` clean).

> Both parity questions raised during review are now **verified** against the Python
> `sz_configtool` reference (`/opt/senzing/er/bin/sz_configtool`, 4.4.0): `list_rules` sorts by
> `ERRULE_ID` ascending (`do_listRules` `key=lambda k: k["ERRULE_ID"]`), and the `BEHAVIOR_CODES`
> ordering — including the A1-family slot — is byte-identical to Python's `valid_behavior_codes`.
> The `LOCKED_FEATURES` protected set was ratified by the maintainer.

### Fixed (correctness — some were silent data corruption)

- **`set_generic_threshold` no longer corrupts the wrong row.** It now matches on the full
  `(GPLAN_ID, BEHAVIOR, FTYPE_ID)` identity; `feature` is a lookup key (with `all` → `FTYPE_ID 0`)
  and is never written into the matched row's `FTYPE_ID`. A set with no matching per-feature row
  now returns `NotFound` instead of silently editing (and corrupting) a `(plan, behavior)` row.
  Unknown `sendToRedo` is rejected; the value is stored canonical title-case `"Yes"`/`"No"`. (#32)
- **`add_generic_threshold`** stores `SEND_TO_REDO` as `"Yes"`/`"No"` (was `"YES"`/`"NO"`, which
  disagreed with the shipped config). (#32)
- **`add_config_section_field` no longer overwrites existing values** — it inserts only where the
  key is absent. (#32)
- **`LOCKED_FEATURES` corrected** to the ratified protected set `NAME, ADDRESS, PHONE, DOB,
  REL_LINK, REL_ANCHOR, REL_POINTER`. `EMAIL, RECORD_TYPE, NATIONAL_ID, TAX_ID, ACCT_NUM` are now
  correctly deletable; `DOB` and the `REL_*` features are now correctly protected; four inert
  non-feature-code entries were removed. (#35)
- **Read projections preserve stored `null`.** `get_rule`/`list_rules`, `get_fragment`/
  `list_fragments`, and all four `list_*_functions` now emit JSON `null` (not `""`) for a stored
  or absent nullable column — the engine's loader distinguishes them. `list_comparison_functions`
  also now emits the previously-missing `description` and a fixed field order. (#33)
- **`get_standardize_call`/`get_distinct_call` return the correct call** when addressed by
  feature: they now scan `CFG_SFCALL`/`CFG_DFCALL` by `FTYPE_ID` instead of using the feature id
  as the call id. (#40)
- **`add_rule` now validates** (duplicate code, fragment/disqualifier existence, RESOLVE/RELATE
  domain and mutual exclusivity, RTYPE_ID coherence) via a validator shared with `set_rule`, so
  the two cannot drift; it auto-assigns `ERRULE_ID` (seed 1000) and returns the assigned id. (#39)
- **`get_config_section` filter** now matches on `json.dumps`-spaced JSON (Python parity) instead
  of compact JSON, so filter terms spanning a `": "`/`", "` boundary behave correctly. (#36)
- Fixed truncated not-found messages: `Standardize call ID {id}` and the comparison get path now
  read `… does not exist`, consistent across all four call families. (#42)

### Added

- **`add_element_to_feature` / `delete_element_from_feature`** — typed `CFG_FBOM` add/remove with
  duplicate detection and whole-table `EXEC_ORDER` allocation. (#38)
- **`settings` module + `set_setting`** — manage `G2_CONFIG.SETTINGS` (uppercases NAME,
  create-if-absent, overwrite). (#38)
- **Cascading function deletes** — `delete_{comparison,expression,standardize}_function_cascade`
  (the existing non-cascading deletes are unchanged). (#38)
- **Caller-supplied ids** — `id: Option<i64>` on `AddDataSourceParams`, `AddAttributeParams`,
  `AddElementParams`, `AddFeatureParams`, `AddComparisonCallParams`; `add_fragment` now honours a
  supplied `ERFRAG_ID`. Absent/≤0 auto-assigns; a taken id is rejected. (#37)
- **By-feature call resolution** — `calls::CallSelector { Id | Feature }` accepted by all four
  `get_*_call`; by-feature helpers resolve a feature code to its call id. (#40)
- **`list_behavior_overrides_resolved`** — display shape `{feature, usageType, behavior}` with
  id→code resolution, composed behaviour, sorted `(FTYPE_ID, UTYPE_CODE)`. (#43)
- **`validate_config`** (structure-only gate) and **`render_config(config, indent)`** (canonical
  key-sorted export renderer). (#43)
- **`config_section_is_empty`** — distinguishes an empty section from a filter that matched
  nothing, without changing `get_config_section`'s return type. (#36)
- **Public config domains** — `behavior_domain::{BEHAVIOR_CODES, compute_behavior,
  parse_behavior_code}` (the two private copies collapsed into one), `ATTRIBUTE_CLASSES`. (#43)
- **`SzErrorKind` + `SzConfigError::kind()`/`reason_code()`** — a stable machine-readable error
  surface so callers branch on a discriminant rather than message text. (#42)
- **Shared building blocks** — `FieldUpdate<T>` tri-state, `field_or_null`, and a `filter`
  substrate module (`Compact`/`JsonDumps`/`PythonRepr`).
- New additive FFI wrappers + header declarations for all of the above; the pre-existing
  `SzConfigTool_listBehaviorOverrides` gained its missing header declaration.

### Changed (breaking — Rust API)

- `delete_comparison_threshold(config, cfunc_code, ftype_code, cfunc_rtnval)` — new required
  `cfunc_rtnval`; full 3-key match; `all` → `FTYPE_ID 0`. (#32)
- `add_config_section_field` returns `(String, AddFieldCounts { existed, updated })` (was
  `(String, usize)`). (#32)
- Four `Add*FunctionParams`: `connect_str: &str` → `Option<&str>`; the four function row structs'
  `connect_str: String` → `Option<String>` (can now serialise `null`). The `add_distinct_function`
  blank-CONNECTSTR rejection was removed (Python accepts blank). (#34)
- Tri-state via `FieldUpdate<T>` on `SetRuleParams.{fragment, disqualifier, tier}`, the four
  `Set*FunctionParams.connect_str`, and a new `SetFragmentParams` (replacing `set_fragment`'s
  `&Value`; clearing SOURCE also clears DEPENDS). (#34)
- `get_*_call(config, CallSelector)` (was `(config, id: i64)`); `delete_*_call_element(config,
  CallSelector, element_code)` now derive `EXEC_ORDER` internally. Removed
  `DeleteComparisonCallElementParams`, `DeleteDistinctCallElementParams`, `ExpressionCallElementKey`.
  (#40)
- `Add*Params` gained a public `id` field — exhaustive struct literals must add it; builder /
  `..Default::default()` callers are unaffected. (#37)

### Changed (breaking — behaviour/output; no signature change)

- `add_data_source`/`add_attribute` now seed ids at 1000 (were unseeded `max+1`); a fresh data
  source moves from `DSRC_ID 3` to `1000` and is no longer accidentally treated as a protected
  low id. (#37)
- List functions (`list_*_calls`, `list_generic_thresholds`, `list_rules`) now return rows in
  SDK-owned sorted order; the three call lists with BOMs emit an `EXEC_ORDER`-ordered
  `elementList`; `list_generic_thresholds` gained an `id`. Snapshot tests asserting the old stored
  order will observe the change. (#41)
- Not-found wording for standardize get/delete and comparison get gained `… does not exist`. (#42)
- `set_rule` with an explicit empty-string fragment now stores `""` (was `NotFound`); the taken-id
  message now includes the id. (#39)
- `add_feature` now validates per-element `elementList` `DISPLAY_LEVEL` and `DERIVED` via the shared
  strict validators (`elements::validate_display_level` / `validate_derived`): a negative display
  level and an unknown `DERIVED` value in an element are now rejected rather than stored verbatim /
  silently coerced to `"No"`. Unifies the last of the three DISPLAY_LEVEL/DERIVED code paths (D25).

### Notes

- FFI C ABI: no existing wrapper signature changed; the call-element delete redesign had no prior
  C wrapper, so it is additive at the C level.
- No dependency changes in this release (`Cargo.toml`/`Cargo.lock` unchanged bar the version bump);
  `cargo deny` posture is identical to 0.5.0.

## [0.5.0] - 2026-08-21

### Fixed

- **Config rows are now always written with every key present.** `set_rule` dropped a
  null `DISQ_ERFRAG_CODE` when updating a rule, producing a `CFG_ERRULE` row missing that
  key; the Senzing engine's config loader then rejected the saved config with
  `SENZ9117 (CONFIG information for DISQ_ERFRAG_CODE not found in CFG_ERRULE)`. Fixed here
  and generalized across the crate.
- **Schema conformance** against the authoritative Senzing v4 column set for each
  `CFG_*` section (from `config/engine/*.data`, confirmed against a real engine template):
  - `CFG_DFUNC` now always includes `ANON_SUPPORT` (default `"No"`), which
    `add_distinct_function` previously omitted. `list_distinct_functions` already read it.
  - `CFG_DFCALL` is now written with exactly its three authoritative columns
    (`DFCALL_ID, FTYPE_ID, DFUNC_ID`). Both builders previously added spurious `FELEM_ID`
    and/or `EXEC_ORDER` keys to the header row; those belong to `CFG_DFBOM`, which is
    unchanged. Distinct-call identity is now `(FTYPE_ID, DFUNC_ID)`.

### Changed

- Config-section rows are now built from typed serde row structs. The row structs for
  the eight sections that previously had **two** independent definitions (one in
  `features.rs`, one in a `calls/*` or `elements` module) — `FelemRow`, `SfcallRow`,
  `EfcallRow`, `CfcallRow`, `EfbomRow`, `CfbomRow`, `DfcallRow`, `DfbomRow`, plus the
  single-definition `FtypeRow`/`FbomRow` — are now consolidated into one crate-internal
  `src/config_rows.rs` module (one `pub(crate)` struct per section). The remaining
  per-module row structs (`ErruleRow`, `ErfragRow`, `AttrRow`, `DsrcRow`, `FbovrRow`,
  `GplanRow`, `CfrtnRow`, and the function rows) are unchanged. All structs derive
  `Serialize` with no `skip_serializing_if`, so optional fields serialize as JSON `null`
  instead of being omitted — every section key is always present in the emitted JSON.
- `fragments::set_fragment` now carries `ERFRAG_ID` and `ERFRAG_DESC` (and
  `ERFRAG_SOURCE`/`ERFRAG_DEPENDS` when the source is not being updated) forward from the
  existing row rather than dropping them.

### Removed

- **BREAKING:** `CFG_FELEM` no longer carries a `TOKENIZE`/`TOKENIZED` field — it is not a
  column in the Senzing v4 schema. Following the v0.4.0 approach for the deprecated
  data-source fields, the `tokenized` parameter has been removed from `AddElementParams`
  and `SetElementParams` (and their `TryFrom<&Value>` impls), `add_element` no longer
  emits `TOKENIZE`, `set_element` no longer writes `TOKENIZED`, and the FFI element
  wrappers no longer read `tokenized`/`TOKENIZED`.
- **BREAKING:** Removed `functions::comparison::add_comparison_func_return_code` (and its
  re-export). It wrote a non-existent `CFG_CFRTN` shape (`{CFRTN_ID, CFUNC_ID, CFRTN_CODE,
  CFRTN_DESC}`). `CFG_CFRTN` is the 10-column score row owned by `thresholds.rs`, which is
  unchanged.

### Added

- `src/config_rows.rs`: one crate-internal serde row struct per consolidated `CFG_*`
  section (see Changed), eliminating the duplicate/divergent struct definitions.
- `helpers::field_as_string` (crate-internal) to carry an existing string/nullable field
  forward during updates.
- Per-module "all keys present" unit tests and a `tests/roundtrip_completeness.rs`
  integration test. The real-config round-trip now runs by **default** against the
  committed engine template `tests/fixtures/g2config_template.json` (resolved via
  `CARGO_MANIFEST_DIR`), running every `CFG_ERRULE` row through `set_rule` and every
  `CFG_ERFRAG` row through `set_fragment` and asserting no row loses a key.
  `SZ_CONFIG_FIXTURE=/path/to/g2config.json` overrides the fixture with your own config
  (optionally `SZ_CONFIG_OUT=/path` to write the round-tripped result).

## [0.4.0] - 2026-08-20

### Removed

- **BREAKING:** Removed the deprecated `conversational` and `reliability` fields from
  `AddDataSourceParams` and `SetDataSourceParams`. These are no longer part of the Senzing data
  source schema. They may still appear in older configs, where they are now ignored, and they are
  never written. `add_data_source` no longer emits `DSRC_RELY` or `CONVERSATIONAL`, and the FFI
  `SzConfigTool_setDataSource` no longer reads them from its `updates_json`.

### Changed

- Bumped dependencies: `serde` 1.0.229, `serde_json` 1.0.151, `anyhow` 1.0.104.
- Bumped GitHub Actions: `actions/checkout`, `dtolnay/rust-toolchain`, and
  `softprops/action-gh-release` (v3.0.2).

## [0.3.2] - 2026-07-02

### Security

- Bump `anyhow` to 1.0.103 to resolve **RUSTSEC-2026-0190** (unsoundness in `Error::downcast_mut()`),
  clearing the `cargo deny` advisories check.

### Removed

- Dropped a stray committed compiled test binary (`tests/c/test_basic`, Mach-O); it rebuilds from
  `tests/c/test_basic.c` and is already in `.gitignore`.

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

### Fixed

- Optional fields in `add_*` functions now always include the field as null instead of omitting it when None
  - `add_feature`: DISPLAY_DELIM in CFG_FBOM (reported by SzCompare: "CONFIG information for DISPLAY_DELIM not found in CFG_FBOM!")
  - `add_feature_comparison`: EXEC_ORDER, DISPLAY_LEVEL, DISPLAY_DELIM, DERIVED
  - `add_feature_distinct_call_element`: EXEC_ORDER
  - `add_comparison_threshold` / `add_comparison_threshold_by_id`: EXEC_ORDER, SAME_SCORE, CLOSE_SCORE, LIKELY_SCORE, PLAUSIBLE_SCORE, UN_LIKELY_SCORE
  - `add_standardize_function`: SFUNC_DESC, LANGUAGE
  - `add_expression_function`: EFUNC_DESC, LANGUAGE
  - `add_comparison_function`: CFUNC_DESC, LANGUAGE
  - `add_comparison_func_return_code`: CFRTN_DESC
  - `add_distinct_function`: DFUNC_DESC, LANGUAGE
  - `add_standardize_call_element`: EXEC_ORDER

### Added

- Integration tests for DISPLAY_DELIM field presence in CFG_FBOM records

### Planned

- [ ] Additional FFI functions (22 remaining for 100% coverage)
- [ ] Python bindings (ctypes or PyO3)
- [ ] Improved test coverage (target >80%)
- [ ] Performance benchmarking suite
- [ ] Config validation functions
- [ ] Config diff and merge operations
- [ ] Import/export utilities
- [ ] Schema migration helpers

## [0.3.0] - 2026-02-16

### Added

- Complete API.md rewrite with parameter struct documentation
- Missing module documentation: `functions::matching`, `config_sections`
- "See Also" cross-references section for improved navigation
- Session history tracking in API.md (Sessions 96-99)

### Changed

- Applied inline format args modernization (clippy compliance)
- Added domain validation with automatic case normalization (Yes/No/Any/Desired)
- Improved element list validation (empty list checks)
- Fixed all doctests for new parameter struct API
- Updated to reflect v0.2.0 breaking changes in documentation

### Fixed

- Suppressed appropriate dead code warnings (#[allow(dead_code)])
- 100% clippy compliance with modern Rust idioms
- Enhanced Python parity in validation logic

## [0.2.0] - 2026-02-05

### Changed

- **BREAKING**: Refactored API to use code-based parameters instead of numeric IDs
  - Public functions now accept string codes (e.g., `feature_code: "NAME"`) instead of numeric IDs
  - Internal ID lookups happen automatically via `helpers::lookup_*_id()` functions
  - Makes code self-documenting and eliminates manual foreign key lookups
- **BREAKING**: Refactored all multi-parameter functions to use parameter structs
  - Functions with 3+ parameters now use dedicated parameter structs with builder pattern
  - Example: `SetFeatureElementParams::new("NAME", "FIRST_NAME").with_display_level(1)`
  - All optional parameters use `Option<T>` types

### Added

- Command script processor for `.gtc` files (batch configuration operations)
- Behavior overrides module (`behavior_overrides`) for CFG_FBOVR operations
- Configuration validation examples with comprehensive error reporting
- Real upgrade script examples demonstrating practical migration workflows
- Session summary documentation with detailed statistics
- Support for `..Default::default()` pattern in parameter structs
- SDK usage warnings in documentation
- Comprehensive integration tests for command processor

### Fixed

- Critical gap: Added `behavior`, `class`, and `rtype_id` to `set_feature()`
- Fixed CFG_DBOM typo in `calls/distinct.rs` (should be CFG_DFBOM)
- Fixed GitHub Pages deployment
- Fixed library name to use snake_case convention
- Applied `cargo fmt` to validate_config example

### Improved

- Updated documentation to extensively demonstrate `..Default::default()` pattern
- Modernized API examples in documentation
- Updated docs landing page with modern API example

---

[0.3.0]: https://github.com/brianmacy/sz-rust-sdk-configtool/releases/tag/v0.3.0
[0.2.0]: https://github.com/brianmacy/sz-rust-sdk-configtool/releases/tag/v0.2.0
[0.1.0]: https://github.com/brianmacy/sz-rust-sdk-configtool/releases/tag/v0.1.0
