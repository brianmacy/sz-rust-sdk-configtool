# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) up to
0.10.0. From 4.4.0-1 versions are `X.Y.Z-N`: `X.Y` is the Senzing line whose configuration
template the release is tested against, `Z` and the release counter `-N` are this project's,
and the Rust API is additive only within `4.x` (README, "Versioning").

## [Unreleased]

## [4.4.0-2] - 2026-10-07

Follow-up to 4.4.0-1 (tag `v4.4.0-2`): the two Node-binding issues #75 and #76, Python platform
alignment with Senzing, and a much smaller release asset list. **Breaking for 4.4.0-1 users of
the typed bindings** (config-changing return shapes, Node option validation, Python wheels).

### Changed

- **BREAKING (all five typed bindings, #75):** every config-changing function now returns the
  **new config text**. In 4.4.0-1, 30 of them returned a `{config, json}` record
  (`ConfigAndJson`) while the other 46 returned the config text, so plain-JavaScript callers
  chaining `cfg = ct.addFeature(cfg, ...)` then `cfg = ct.addAttribute(cfg, ...)` failed on the
  *next* call. The created or removed row, or the assigned ids, now come from a companion
  `<name>Result` with the same arguments (Python `<name>_result`, Java/TypeScript
  `<name>Result`, C#/C++ `<Name>Result`); it returns the row JSON text, or the named-fields
  record for `set_generic_plan`. The 30 functions: `add_attribute`; `add_{comparison,
  distinct,expression,standardize}_call` and their `_element` forms; `add_config_section_field`,
  `remove_config_section_field`; `add_fragment`; `add_rule`; `add_/set_/delete_{comparison,
  distinct,expression,standardize}_function`; `delete_{comparison,expression,standardize}_function_cascade`;
  `clone_generic_plan`; `set_generic_plan`. `ConfigAndJson` is removed and the named-fields record
  types are renamed `<Fn>Record` (e.g. `SetGenericPlanRecord`; no `config` field). Node and
  Python now fail early with `INVALID_INPUT` ("config must be a string ...") when a non-string
  config is passed. A `<name>Result` companion re-runs the operation, so call it with the SAME
  input config you gave the primary (on the already-modified config an add fails with
  `ALREADY_EXISTS`, a delete with `NOT_FOUND`, and `set_generic_plan`'s result reports
  `was_created: false`). The manifest, the `invoke` wire format and the C ABI are unchanged.
- **BREAKING (Node binding, #76):** options are validated before the native call. Unknown option
  keys fail with `INVALID_INPUT` and a closest-name hint (`unknown option 'candidate' for
  addSearchProfile; did you mean 'candidates'?`); a string, array or other non-object where
  options are expected fails instead of being silently ignored; native errors name the JS
  option with the wire name in parentheses once (`Missing required field: genericPlan
  (generic_plan)`). Structured options are now exactly typed (`addSearchProfile.elements`,
  `addFeature`/`addExpressionCall` `elementList`, `addFragment.fragmentConfig`,
  `addRule.ruleConfig`) with runtime shape checks naming the path, and the tRPC router's Zod
  objects are strict. Optional nested keys the library itself reads as optional accept `null`
  (stored `CFG_ERRULE`/`CFG_ERFRAG` rows round-trip through `addRule`/`addFragment`), keys set to
  `undefined` count as absent, and wire-name translation never rewrites quoted values. Node is
  stricter than the library here: it accepts only `Yes`/`No`/`Y`/`N`
  for search-profile element flags and rejects keys the library would ignore; `invoke` is
  unchanged. Python, Java, C# and C++ already reject unknown or misspelled options at call or
  compile time.
- **Python wheels are Linux-only** (`manylinux_2_34` x64 and arm64), matching Senzing's Linux-only
  Python SDK (v4 System Requirements). The macOS and Windows wheels and their SBOMs are no longer
  built, and were deleted from the published 4.4.0-1 release. Java, C#, C++, Node and the C
  library still ship for Linux x64/arm64, macOS arm64 and Windows x64.
- Wheel, crate and package metadata: the author is name-only (the 4.4.0-1 wheels carried a
  senzing.com email address).

### Added

- Optional `json_type` descriptor (with a `nullable` form) for `json` manifest arguments (`api/manifest/schema.md`):
  drives the exact TypeScript types, runtime shape checks and strict tRPC schemas; other bindings
  get a `Shape:` doc line.
- Release notes are the CHANGELOG section for the tag (`packaging/release-notes.sh`, self-tested
  in CI), and the publish step is re-runnable: if the release already exists it edits the notes
  and uploads with `--clobber` instead of failing.
- `packaging/gates/check-release-assets.sh` (self-tested in CI) requires exactly the expected
  asset set plus `SHA256SUMS` and fails on any SBOM, attestation-bundle, subdirectory or stray
  file.

### Removed

- Releases no longer carry the per-component `*.cdx.json` SBOMs or the `*.intoto.jsonl`
  attestation bundle. The C archive still embeds its own SBOM (`sbom/sz-configtool-c.cdx.json`),
  and the full dependency list is `Cargo.lock` at the tag. Attestations are unchanged and are
  verified online: `gh attestation verify <file> --repo brianmacy/sz-rust-sdk-configtool`; check
  integrity with `sha256sum -c SHA256SUMS` (`--ignore-missing` for a partial download).
  The published 4.4.0-1 release was trimmed the same way (37 -> 22 assets).
- The "Go (via cgo)" bullet in `docs/FFI_GUIDE.md`: Senzing publishes no Go V4 SDK, and this
  project ships no Go.

## [4.4.0-1] - 2026-10-07

First release under the Senzing-aligned version scheme (0.10.0 -> 4.4.0-1; tag `v4.4.0-1`).
SemVer tools (Cargo, npm, NuGet) order `4.4.0-1` BEFORE `4.4.0` (a prerelease), while Python
(`4.4.0.post1`) and Maven order it AFTER; releases are GitHub-only with exact pins, so this
matters only to range resolvers (README, "Versioning").

### Removed

- **BREAKING (Rust API and C ABI; covered by the `SZCONFIGTOOL_ABI_VERSION` 2 bump below):**
  the never-functional placeholder functions, 34 in all. Reason: none ever did anything (they
  always returned `NotImplemented` / a "not yet implemented" `InvalidInput`, or returned the
  config unchanged ignoring their arguments) and nothing calls them: G2's `sz-tools` and the
  Python `sz_configtool` do not.
  - Modules `functions::{matching, scoring, candidate, validation}` entirely:
    `add_/delete_/get_/set_/remove_*_function` and `list_*_functions` for each (24
    functions), their re-exports from `functions`, the manifest groups
    `functions_{matching,scoring,candidate,validation}` (20 `status: not_implemented`
    functions) and their conformance cases.
  - `thresholds::get_threshold`, `thresholds::set_threshold` and `SetThresholdParams` (the
    comparison- and generic-threshold functions are unchanged).
  - The no-op setters `calls::{standardize,expression,comparison,distinct}::set_*_call` and
    `set_*_call_element`, with `Set{Standardize,Expression,Comparison,Distinct}CallParams` and
    `Set{Standardize,Comparison,Distinct}CallElementParams` (`ExpressionCallElementParams`
    stays: `add_expression_call_element` uses it).
  - C exports (25): `SzConfigTool_{add,delete,get,set}{Matching,Scoring,Candidate,Validation}Function`,
    `SzConfigTool_list{Matching,Scoring,Candidate,Validation}Functions`, `SzConfigTool_getThreshold`
    and `SzConfigTool_set{Standardize,Expression,Comparison,Distinct}Call` (header
    declarations, `ffi/expected-exports`, return-code lists updated). The manifest no longer
    has a `status: not_implemented` function; the schema keeps supporting it.
- **BREAKING (Rust API and C ABI; `SZCONFIGTOOL_ABI_VERSION` 1 -> 2):** the SYS_OOM hash
  functions. Rust `hashes` module (`add_to_name_hash`, `delete_from_name_hash`,
  `add_to_ssn_last4_hash`, `delete_from_ssn_last4_hash`), the C exports
  `SzConfigTool_addToSsnLast4Hash` / `SzConfigTool_deleteFromSsnLast4Hash`, the manifest
  `hashes` group and its typed wrappers in every binding (`invoke` now reports
  `INVALID_INPUT` "unknown function"). Reason: they required `SYS_OOM` to be an object, but in
  real configurations (and the shipped template) `SYS_OOM` is an array of OOM rows, so the adds
  were silent no-ops and the deletes always `NOT_FOUND`; G2's own CLI retired the commands.

### Changed

- **BREAKING (packaging): the C ABI moved to its own workspace crate.** The repo is now a Cargo
  workspace. `sz_configtool_lib` is a pure `lib` crate (no `cdylib`, no `ffi` module —
  `sz_configtool_lib::ffi` is gone); its Rust API is otherwise unchanged and plain `cargo test`
  still covers only it. The C interface lives in `ffi/` (package `sz-configtool-ffi`,
  crate-type `cdylib` + `staticlib`). Previously every downstream cdylib depending on the crate
  re-exported all 144 `SzConfigTool_*` symbols; now it exports none.
- **BREAKING (packaging): library renamed** from `libsz_configtool_lib.{so,dylib}` /
  `sz_configtool_lib.dll` to `libSzConfigTool.{so,dylib}` / `SzConfigTool.dll`, plus a new static
  `libSzConfigTool.a`. The header moved to `ffi/include/libSzConfigTool.h`.
- Header: every declaration carries the new `SZCONFIGTOOL_API` export macro (dllexport/dllimport
  on Windows, default visibility elsewhere; define `SZCONFIGTOOL_STATIC` for static linking), and
  `SZCONFIGTOOL_ABI_VERSION` is defined.
- `helpers::field_update_str`, `helpers::field_update_i64`, `helpers::lookup_gplan_code` and
  `thresholds::{add,set,delete}_comparison_threshold_by_id` are now `#[doc(hidden)] pub` (were
  `pub(crate)`) so the FFI crate can reach them; they are not supported Rust API.

### Added

- Binding manifest (`api/manifest/*.yaml`, one file per group; see `api/manifest/schema.md`):
  126 functions in 24 groups (coverage enforced by the drift test) and 717
  conformance cases run against the real template config by Rust and every binding. New
  workspace crates: `sz-configtool-api` (`invoke(name, config, args_json)` dynamic dispatcher,
  generated `dispatch_gen.rs`, and the shared `validation_details_json` used by the C ABI and
  every native seam) and `sz-configtool-codegen` (`cargo run -p sz-configtool-codegen`
  regenerates the dispatcher, JSON manifests and all five language wrappers; `-- --check`
  fails when stale). Output locations come from `api/manifest/project.yaml` (`paths`,
  `bindings`).
- Language bindings generated from the manifest, all functional/stateless (`f(config_json,
  ...)`), one error class each whose `kind` is the reason code, `json` results as JSON text,
  `ConfigAndJson` / `<Fn>Result` named records, and natural `int | str` call selectors:
  - Python (`bindings/python`, distribution `sz-configtool`, import `sz_configtool`, pyo3
    abi3-py310, maturin). **Breaking vs earlier development snapshots of this unreleased
    binding:** it was `senzing-configtool` / `senzing_configtool` (renamed: the official Senzing
    packages are `senzing` / `senzing_core`, so a `senzing_*` name was misleading). A `config`
    that is not a `str` or holds a lone surrogate raises `SzConfigToolError` `INVALID_INPUT`
    (was `TypeError` / `UnicodeEncodeError`).
  - Java (`bindings/java` + JNI seam `bindings/jni`, package `io.github.brianmacy.szconfigtool`,
    JDK 17, natives bundled in the jar).
  - C# (`bindings/csharp`, namespace `Sz.ConfigTool`, netstandard2.0, P/Invoke over
    `SzConfigTool_invoke`). A string with a lone surrogate raises `SzConfigToolException`
    `INVALID_INPUT` (was `ArgumentException`), like every other binding.
  - C++ (`bindings/cpp`, header-only C++20 `szconfigtool.hpp`, CMake package, over
    `SzConfigTool_invoke`).
  - Node/TypeScript (`bindings/node`, napi-rs v3) plus a tRPC router (`bindings/node/trpc`).
- `config_sections::AddFieldCounts` and `thresholds::GenericThresholdCheck` implement
  `serde::Serialize` (the latter as the versioned `sz-configtool.generic-threshold-check/v1`
  object, now the single serializer behind `SzConfigTool_validateGenericThreshold`; new const
  `thresholds::GENERIC_THRESHOLD_CHECK_SCHEMA`). `add_config_section_field` and
  `validate_generic_threshold` are now in the binding manifest.
- Manifest schema: arg type `int_or_str` (call selectors: id or feature code), arg
  `required: true` (library-required `Option` args), function
  `status: not_implemented` (placeholders typed generators skip), `tuple_names` on `json`
  returns (`verify_compatibility_version` now returns `{current_version, matches}`, not an
  array), and computed conformance `wire_only` steps.
- `SzConfigTool_invoke(name, config_json, args_json)`: calls any
  manifest function by name, returning a `{"kind","config","result"}` JSON envelope and stable
  reason codes on error.
- `SzConfigTool_getLibraryVersion()` (static NUL-terminated crate version) and
  `SzConfigTool_getAbiVersion()`.
- `SzConfigTool_set{Standardize,Expression,Comparison,Distinct}FunctionWithJson` — these were
  declared in the header but never implemented. `CONNECT_STR` is tri-state (absent = leave,
  `null` = clear, string = set), which the direct-arg setters cannot express.
- Version accessors in every binding, from ONE definition (`sz_configtool_api::{LIBRARY_VERSION,
  ABI_VERSION}`, also behind `SzConfigTool_getLibraryVersion` / `SzConfigTool_getAbiVersion`):
  Python `__version__` / `library_version()` / `abi_version()`, Java
  `SzConfigToolVersion.libraryVersion()` / `abiVersion()`, TS `libraryVersion()` /
  `abiVersion()`, C++ `AbiVersion()` (C# `LibraryVersion` / `AbiVersion` and C++
  `LibraryVersion()` already existed). See `bindings/CONTRACT.md`.
- Manifest `c_notes` field: C-ABI-only deltas (typed `SzConfigTool_*` exports, return codes) moved
  out of `notes`, so Python/Java/C#/C++/TS docs no longer carry C-specific text; codegen rejects
  C-ABI text in `doc`/`notes`/`semantics`.
- Header declarations for the already-exported `SzConfigTool_addBehaviorOverride`,
  `SzConfigTool_deleteBehaviorOverride`, `SzConfigTool_deleteExpressionFunction` and
  `SzConfigTool_deleteStandardizeFunction`.
- `ffi/tests/header_sync.rs` (fails on header/export drift in names or signatures) and
  `ffi/tests/c_abi.rs` (builds the library, then compiles and runs the C test and C example).

### Removed

- Stale header declarations with no implementation:
  `SzConfigTool_set{Matching,Candidate,Validation,Scoring}FunctionWithJson` (the underlying
  Rust functions are `NotImplemented` stubs; the direct-arg `set*Function` exports remain), and
  12 duplicate `get*/list*Function(s)` declarations.

### Fixed

- `features::update_feature_version` panicked on a config whose top level is not an object
  (e.g. `[]`: JSON `IndexMut` on an array); it now returns `MissingSection("COMPATIBILITY_VERSION")`,
  the error it already returned for any config lacking that path (bindings: `MISSING_SECTION`
  instead of `INTERNAL`).
- `command_processor`: `addComparisonThreshold` with feature `ALL` (any case) always failed
  with `MissingField("ftype_code")` (the processor turned `ALL` into `None`). It now passes the
  code through, so the library stores the all-features sentinel `FTYPE_ID 0`, as G2's CLI does.
- `calls::expression::add_expression_call` stored BOM `FTYPE_ID -1` for an element-list item
  with feature `"PARENT"`: a `.filter(!"PARENT")` dropped it before the intended `parent -> 0`
  branch ran. It now stores `0` (case-insensitive), the G2 parent feature link
  (`EFBomConfig::PARENT_FEATURE_LINKED_FTYPE`, `G2/dev/libs/configTables/EFBomConfig.cpp:26`;
  `-1` = `WILDCARDED_FTYPE`, any feature, `:27`), matching Senzing's Python configtool
  (`G2ConfigTool.py:3079,3120` store 0; `:3569` reads 0 back as `featureLink: parent`). An
  absent feature still stores `-1`. Comparison/distinct call BOMs are unchanged (they require a
  real FTYPE).
- `SzConfigTool_getLastError()` returned a pointer to a Rust `String` with no NUL terminator, so C
  callers read past the message. All last-error strings are now NUL-terminated `CString`s.
- The last-error slots were process-global `Mutex` statics, so concurrent threads overwrote each
  other's errors (and a poisoned lock would panic). They are now per-thread (`thread_local!`);
  returned pointers stay valid until the next `SzConfigTool_*` call on the same thread.
- A Rust panic inside any `extern "C"` function unwound across the C boundary (undefined
  behaviour). Every export now catches panics and returns `-2` / NULL with an
  `internal panic in <function>: ...` last error.
- Six header declarations disagreed with the exported ABI (C callers following the header passed
  wrong argument types or counts): `updateCompatibilityVersion`, `updateFeatureVersion` and
  `verifyCompatibilityVersion` take the version as `const char *` (header said `int64_t`);
  `addConfigSection` takes no `section_json`; `setGenericPlan` takes no `updates_json`;
  `listStandardizeCalls` takes no filter arguments. The header now matches the exports.
- Removed `expect`/`unwrap` calls from library code paths (`settings::set_setting`,
  `thresholds::add_generic_threshold`, `config_sections::get_config_section`); each was guarded
  by an earlier check, so behaviour is unchanged, but no panic path remains.
- `ffi/examples/c_ffi_example.c` did not compile against the header (wrong field name and
  argument counts) and released library memory with `free()`; rewritten and now run in tests.
- `SzConfigTool_deleteGenericThreshold` parsed its `plan` argument but ignored it, always deleting
  the matching row from plan `INGEST` (so a `SEARCH` delete removed the `INGEST` row, and an
  unknown plan succeeded). It now deletes from the given plan; an unknown plan is an error.

- Java (JNI) and TS (napi) silently replaced a lone UTF-16 surrogate with U+FFFD (e.g. a data
  source code `"A\uD800"` was stored as `"A\uFFFD"`); it is now `INVALID_INPUT`. The JNI seam
  decodes modified UTF-8 strictly.
- TS: `NaN` / `±Infinity` args became JSON `null` (a tri-state Clear) and integers beyond
  `Number.MAX_SAFE_INTEGER` were sent rounded; both are now `INVALID_INPUT`.
- C ABI: an FFI call from a thread-local destructor (thread teardown) could abort the process
  (last-error slot already destroyed, then a double panic in the panic guard). The slot is now
  accessed with `try_with`; during teardown the error is reported by return code only.
- `SzConfigTool_set*FunctionWithJson`: a non-string value, both spellings of one field (e.g.
  `{"SFUNC_DESC": null, "description": "x"}` ignored `"x"`), an unknown key, or `ANON_SUPPORT`
  for standardize/expression functions (accepted, then dropped) are now `INVALID_INPUT` instead
  of being silently ignored.
- `invoke`: a missing `feature`/`flag` key in an `add_search_profile` `elements` item is now
  `MISSING_FIELD` (was `INVALID_INPUT`), matching the expression-call element list and missing
  required args.
- Manifest `errors[]` completed: `add_feature` (`INVALID_STRUCTURE`), `delete_element` and
  `add_expression_call` (`MISSING_FIELD`), `add_search_profile` (`MISSING_FIELD`); a new probe
  test calls every function with systematic argument sets and fails on any unlisted code.
- Conformance runners (Rust, Python, Java, C#, C++) treated a non-array result as `[]`, so
  `excludes` and `len: 0` passed vacuously; they now fail, and codegen rejects
  `len`/`contains`/`excludes` on functions whose result cannot be an array.
- C#: `SzConfigToolException.ReasonCode` is non-null (`INTERNAL` when none was reported), like
  Java.
- Header return-code documentation was wrong (it said -2 = library error, <= -3 = JSON
  failures). It now documents what the exports actually return (no behaviour change): 77
  typed exports return -5 for library errors without a reason code, four `get*CallByFeature`
  return -2 without one, `setGenericThreshold` returns -4 for an unknown plan id, and invalid
  UTF-8 is -1 for 16 exports and -2 for the rest; `SzConfigTool_invoke` uses only 0/-1/-2 with
  reason codes. `ffi/tests/return_codes.rs` fails when the header lists drift from the source.
- TS: `int` args accept `bigint` (sent with exact digits; full i64 range, e.g. 2^63-1) and the
  generated types are `number | bigint`; unsafe `number`s stay `INVALID_INPUT`. Named results
  (`memberTexts`) now keep each member's exact JSON text on every Node version (Node 20
  re-serialized it). The runtime supports Node 20 (compiled `dist/`); running the tests and
  `.ts` examples directly needs Node 22.6+.
- Python: a non-`str` function name, non-mapping args, or an arg that cannot be JSON-encoded
  (`set`, `Decimal`, `bytes`, NaN) raises `SzConfigToolError` `INVALID_INPUT` (was `TypeError` /
  `ValueError`), like TS.
- C#: a NUL character in the name, config or raw args JSON raises `SzConfigToolException`
  `INVALID_INPUT` (was `ArgumentException`); NUL handling per binding is in
  `bindings/CONTRACT.md`.
- JNI: the strict modified-UTF-8 decoder accepted overlong encodings other than `C0 80` (the
  modified-UTF-8 NUL), e.g. `C0 81` or `E0 80 80`; they are now rejected.

### Coverage

- Coverage exclusions cut from 46 to 3 by removing the code shapes that needed them, not by
  hiding code: infallible serialization (`Value` Display; one `row_value` helper for derived
  row structs), each config parsed once (Value-based lookups instead of re-parsing),
  verified sections fetched with one `verified_section_mut` instead of `if let` with a dead
  `else`, the FFI result plumbing as shared functions (`lib_result` / `plain_result` /
  `text_result`) instead of per-export macro copies, placeholder functions removed, panic
  guards factored into tested `guarded` helpers, and host-independent C# platform mapping.
  Behaviour-neutral except as listed under Fixed and here: C ABI C-string conversion failures
  share one message ("Failed to convert result to C string: ..."; codes unchanged, -3
  "serialize failed" can no longer occur); Java `addExpressionCall`, `addStandardizeCall` and
  `setFeature` lose their options-less overload, which the library always rejected (new
  manifest field `requires_options`); C# `LibraryVersion` reports a NULL from the native
  library as an `INTERNAL` protocol error instead of `""`. The never-functional placeholder
  functions were removed (see Removed) rather than covered. The remaining exclusions
  (`coverage/policy.yaml`) are a JVM allocation failure, napi-derive's registration error arm
  and a cross-process extraction race.

### Documented

- Response shape convention (CLAUDE.md, `api/manifest/schema.md`, `bindings/CONTRACT.md`; no
  behaviour change): `get_*` return the stored row (summary gets `get_feature`, `get_element`,
  `get_fragment`, `get_rule` grandfathered), `list_*` return code-resolved summaries, raw rows via
  `get_config_section`, no `describe_*` layer. `list_expression_calls` / `list_comparison_calls`
  / `list_distinct_calls` omit the stored BOM columns from `elementList` (manifest notes give
  the `get_config_section("CFG_EFBOM" | "CFG_CFBOM" | "CFG_DFBOM")` workaround).

### Releases and CI

- Releases: GitHub Releases only (no public registries); see packaging/README.md.
- CI can be started manually on any branch (`workflow_dispatch` on `ci.yml`); Dependabot now also
  covers npm (`bindings/node`, `bindings/node/trpc`), Maven, pip (`packaging/`) and NuGet with the
  same 21-day cooldown, weekly, one grouped PR per ecosystem.
- `ruff` 0.16.7 is hash-pinned in `packaging/requirements-test.txt`, and the generated-Python style
  tests now fail instead of silently skipping when it is missing.
- Per-target release pipeline in `packaging/` (C ABI archive, C++ package, Python wheel, Node
  and tRPC tarballs, Java jar, NuGet package, `SHA256SUMS`, build-provenance attestation) for
  `linux-x64`, `linux-arm64`, `macos-arm64` and `windows-x64`, with export, linkage (Linux:
  also a non-executable `PT_GNU_STACK`), glibc 2.34, build-path and C-test gates.
- CI (`ci.yml`): one Linux job per PR runs lint, codegen check, Rust tests, MSRV, the
  `linux-x64` release pipeline with every binding's tests, and the C++ ASan + UBSan suite (plus
  a plain `cargo test` on macOS and Windows); the full 4-platform matrix (`release.yml`) runs
  only on tags and manual dispatch.
- `security.yml`: PRs run only `cargo deny check` (prebuilt, sha256-verified binary);
  `cargo audit` and `cargo vet` run weekly, on tags and on manual dispatch.
- Release assets include per-artifact CycloneDX SBOMs
  (`sz-configtool[-jni|-node|-python]-<v>-<os>-<arch>.cdx.json`), listed in `SHA256SUMS` and
  covered by the attestation. Only a tag **push** publishes; a manual dispatch (even on a tag)
  is a dry run that uploads the assembled assets as the `release-assets` workflow artifact.
- Windows DLL / `.node` / `.pyd` are linked with the static MSVC runtime: no VC++
  Redistributable needed (`SzConfigTool_static.lib` stays `/MD`). The macOS
  `libSzConfigTool.a` ships without debug info (`llvm-strip --strip-debug`; symbols kept). Binaries are not
  code-signed; packaging/README.md documents verification and the macOS quarantine workaround.
- Version policy enforced by `packaging/gates/check-versions.sh` (self-test
  `test-version-spellings.sh`, run in CI): only `X.Y.Z`, `X.Y.Z-N` (wheel `X.Y.Z.postN`) and
  `X.Y.Z-rc.N` / `-alpha.N` / `-beta.N` (wheel `rcN` / `aN` / `bN`); dev/post/pre spellings are
  rejected (PEP 440 and Maven sort some above the release). The gate also checks both npm
  lockfiles. Numeric macOS dylib `current_version`; numeric CMake package version. Only
  `-rc.N` / `-alpha.N` / `-beta.N` tags become GitHub prereleases (`-N` is a full release).
- Supported platforms documented: Linux x86_64/arm64 glibc >= 2.34 (RHEL 9+, Amazon Linux
  2023, Ubuntu 22.04+, Debian 12+), macOS 15+ arm64, Windows x64. RHEL 8 (glibc 2.28) is not
  supported by the binaries (build from source).
- `gates/check-glibc-ceiling.sh` fails closed when `objdump -T` fails or prints no dynamic
  symbol table (it passed as "(none)"); `run-c-tests.sh` / `package-cpp.sh` no longer abort
  under macOS bash 3.2 `set -u` when ninja is missing; `lib/build-env.sh` no longer masks an
  `msvc_tool` failure behind `cygpath` and documents that it replaces caller `RUSTFLAGS`.
- `.gitattributes` forces LF checkouts (Windows runners); `release-target.sh all` no longer
  ignores a failing gate (`set -e` was suspended inside an `&&` chain).

## [0.10.0] - 2026-09-10

Greenfield `CFG_SPROFILE` (search profile) support (#63), modelled on `CFG_DSRC`, coordinated
with the downstream `sz_configtool` CLI (new `addSearchProfile` / `listSearchProfiles` commands).
The CLI delegates all `CFG_SPROFILE` reads and writes to this library; it never mutates config
JSON itself. Exercised against the stock Senzing v4 template.

### Added

- **New module `search_profiles` with four public functions (#63).** Code-based API: callers
  pass `SPROFILE_CODE`, `GPLAN_CODE`, and feature `FTYPE_CODE`s; the library resolves them to the
  numeric ids stored on disk and owns the `FTYPE_OVERRIDES` mini-format
  (`"[]"` / `"[{<ftypeId>,<Y|N>},...]"`, ascending by id).
  - `add_search_profile(config_json, AddSearchProfileParams) -> Result<String>` — resolves the
    generic plan and each feature, builds the mini-format, allocates `SPROFILE_ID`, and appends a
    complete row. **Creates the `CFG_SPROFILE` section if absent** (unlike `add_data_source`).
    `AddSearchProfileParams` (constructed via builder methods) takes the profile code,
    generic-plan code, `candidates` (`Normal`/`Off`, default `Normal`), optional description, and
    `elements: Vec<(feature_code, "Yes"|"No")>`.
  - `get_search_profile(config_json, code) -> Result<Value>` — raw row, case-insensitive,
    `NotFound` (including when the optional section is absent).
  - `delete_search_profile(config_json, search_value) -> Result<String>` — removes a profile
    resolved by `SPROFILE_CODE` (case-insensitive) or `SPROFILE_ID`. The shipped profiles
    `INGEST`/`SEARCH` are protected (`RESERVED_PROFILES`) and refused with `InvalidInput`, mirroring
    the `deleteFeature` `LOCKED_FEATURES` precedent; existence is resolved before the guard, so a
    truly-absent value reports `NotFound`.
  - `list_search_profiles(config_json, filter: Option<&str>) -> Result<Vec<Value>>` — display
    projection resolving ids to codes (`profile`, `genericPlan`, structured `overrides`), plus the
    raw `overridesRaw` mini-format; sorted by id, tolerant of a missing section.
  - Error mapping reuses existing variants: duplicate profile code → `AlreadyExists`; unknown
    generic plan / feature → `NotFound`; bad `candidates`, bad `Yes`/`No` flag, or a duplicated
    feature → structured `ValidationErrors` (`candidates`/`overrides` field with `OutOfDomain` /
    `Duplicate` reason). No new `SzConfigError` variant — the public surface is purely additive.
  - `ValidationReason::Duplicate` is now emitted (search-profile override dedup); the
    generic-threshold path's "duplicates stay warning-success" policy is unchanged (per-context).

## [0.9.0] - 2026-08-31

Structured generic-threshold validation errors (#59, **breaking**) plus a cosmetic
resolver-message fix (#61), coordinated with the downstream `sz_configtool` CLI. Verified
against Python `sz_configtool` 4.4.0 (`validateGenericThreshold`, `do_addGenericThreshold`,
`do_setGenericThreshold`, `lookupBehaviorCode`) and the stock Senzing v4 template.

### Added

- **`SzConfigError::ValidationErrors(Vec<ValidationFailure>)` / `SzErrorKind::ValidationErrors`
  (`reason_code` `"VALIDATION_ERRORS"`) — structured, aggregated field validation (#59).**
  Generic-threshold add/set no longer flatten field failures into a lossy `"; "`-joined
  `InvalidInput` string. Instead every failure is carried as DATA in a `ValidationFailure`
  (`field`, `reason_code`, `offending_value`), aggregated in canonical order
  `[behavior, sendToRedo]`, so a consumer reproduces its own wording without sniffing prose.
  - New public types `ValidationFailure` and `ValidationReason` (a `#[non_exhaustive]`,
    DATA-only taxonomy: `Missing | WrongType | OutOfDomain | UnknownReferenceCode | NotFound |
    Duplicate`; only `UnknownReferenceCode` for a non-canonical behaviour and `OutOfDomain` for
    a `sendToRedo` outside `[Yes, No]` are emitted today). `SzConfigError::validation_failures()`
    recovers the vector. `Display` re-creates a `"; "`-joined summary for logs/FFI (wording is
    **not** contract).
  - **`thresholds::validate_generic_threshold(...) -> Result<GenericThresholdCheck>`** — a
    validate-only orchestration surface returning every staged outcome as `Ok(..)` DATA
    (`NotFound { which: GenericThresholdRef, value }` fatal-first for plan/feature, `Duplicate`
    warning-success, `Invalid(Vec<ValidationFailure>)`, `Ok`), mirroring Python's staging order
    (plan → feature → duplicate → behaviour+`sendToRedo` aggregate). Reserves `Err` for genuine
    internal errors (unparseable config).
  - `add_generic_threshold` now returns `ValidationErrors` for behaviour/`sendToRedo` failures
    (plan/feature stay `NotFound`; duplicate stays `AlreadyExists` on the direct-call path).
    `set_generic_threshold` now validates `sendToRedo` **after** the row lookup (Python order: a
    missing row wins over a bad `sendToRedo`) and aggregates it into `ValidationErrors` — a
    behaviour change from the previous scalar `InvalidInput`. An unknown behaviour on SET remains
    `NotFound` (behaviour is part of the lookup key, never re-validated as a reference code —
    matching Python, whose merged-record behaviour is always canonical).
  - Caps stay strictly typed `i64` at both the Rust (`optional_i64`) and FFI boundaries; a
    non-numeric or boolean cap remains a scalar `InvalidInput("... must be an integer")` and is
    **never** folded into `ValidationErrors` (bool-as-int rejection is a deliberate divergence
    from Python — Ant 17/08/2026).
  - **FFI:** new `SzConfigTool_getLastErrorReasonCode()` (discriminate the error kind at the C
    boundary — match this first, only then fetch details) and `SzConfigTool_getLastErrorDetails()`
    (versioned, namespaced JSON: `{"schema":"sz-configtool.validation-errors/v1","failures":[...]}`).
    New `SzConfigTool_validateGenericThreshold(...)` returns the staged `GenericThresholdCheck` as
    versioned JSON (`schema` `"sz-configtool.generic-threshold-check/v1"`). Every library error
    now also populates the reason code across the boundary.
  - **Breaking:** `SzConfigError` is not `#[non_exhaustive]`, so the new variant adds an arm to
    exhaustive downstream matches (minor bump under 0.x semver).

### Changed

- **Resolver error messages name the feature CODE, not the internal `ftype_id` (#61).**
  `resolve_call_id_for_feature` (comparison/distinct/standardize/expression) now reverse-maps
  `FTYPE_ID` to `FTYPE_CODE` via `CFG_FTYPE`, emitting `"No comparison call found for feature NAME"`
  / `"Ambiguous ... for feature NAME"`, falling back to `"feature id {n}"` only when no `CFG_FTYPE`
  row matches. Non-breaking: the variant (`NotFound` / `InvalidInput`), `kind()`, and
  `reason_code()` are unchanged; only the (non-contract) `Display` wording improves.

## [0.8.0] - 2026-08-25

Resolves #58 from the CLI's v0.7.0 delete re-delegation. Verified against Python `sz_configtool`
4.4.0 (`prepCallElement`) and the stock Senzing v4 template, and adversarially reviewed by subagents
(no classification defect found). Gates green: 380 test cases + 82 doctests, clippy/fmt clean.

### Added

- **`SzConfigError::NotInFeature` / `SzErrorKind::NotInFeature` (`reason_code` `"NOT_IN_FEATURE"`) —
  the hard-error counterpart to `NotOnCall` (#58).** A call-element delete addressed *with* an element
  feature now distinguishes Python's two-tier check: the element must first be a member of that
  feature (a `CFG_FBOM` element) — a non-member (or a nonexistent element code) is a hard
  `NotInFeature` (`"{element} is not an element of {feature}"`), mirroring Python's
  `lookupFeatureElement` error, rather than the benign `NotOnCall` ("the element is valid but not on
  this call"). Previously both collapsed into `NotOnCall`, so a consumer could not tell an error from
  a warning off `kind()` alone. `delete_{comparison,expression,distinct}_call_element` gained a shared
  `resolve_feature_element_id` guard for this; the feature-less path (`element_feature: None`) keeps a
  plain global element lookup and cannot raise it, matching Python's `ftype_id < 0` branch.
  - The delete paths now resolve the element feature **before** the element (Python's order), so a
    delete naming both a missing feature and a missing element reports the feature first.
  - The library emits the core `"{element} is not an element of {feature}"`; the CLI-specific
    `(use command "getFeature ...")` hint Python appends is left to the CLI (no display logic in the
    library).
  - **Breaking:** `SzConfigError` is not `#[non_exhaustive]`, so the new variant adds an arm to
    exhaustive downstream matches.
  - Verified against `tests/fixtures/g2config_template.json` (every real-feature BOM element is a
    `CFG_FBOM` member; only `FTYPE_ID = -1` sentinel rows are not, and those take the `None` path).

## [0.7.0] - 2026-08-25

SDK-surface wave resolving issues #49, #50, #52, #53, #54, #55 and #56, developed on
`feat/v0.7.0-sdk-surface` in six reviewed steps and coordinated with the downstream
`sz_configtool` CLI. Every write/validate/delete-path change is verified against
`tests/fixtures/g2config_template.json` (the real Senzing v4 template), not synthetic
data. Gates green: 376 test cases (247 unit + 47 integration + 82 doc), clippy
`-D warnings` / `cargo fmt` clean.

**Breaking changes** (folded into a single minor bump under the crate's stability
policy — details in the sections below):

1. **`settings::set_setting` value type + FFI value semantics (#52).** The `value`
   parameter is now `impl Into<serde_json::Value>` (was `&str`) and is stored verbatim;
   the `SzConfigTool_setSetting` FFI now parses `value` as JSON and rejects invalid JSON,
   so a bare string must be passed as quoted JSON (`"\"hello\""`).
2. **`*CallElementParams::exec_order` is now `Option<i64>` (was `i64`) (#55).** Affects
   `AddComparisonCallElementParams`, `ExpressionCallElementParams` (and its `new()`
   signature) and `AddDistinctCallElementParams`; `None` auto-allocates the next order.
3. **New `SzConfigError` variants `NotOnCall` / `AlreadyPresent` (#53, #54).**
   `SzConfigError` is not `#[non_exhaustive]`, so this breaks exhaustive downstream matches.

### Added

- **Error sub-case surface: `SzConfigError::NotOnCall` / `AlreadyPresent`
  (#53, #54).** Two benign single-call sub-cases are carved out of the broader
  `NotFound` / `AlreadyExists` families and given their own stable
  `SzErrorKind` discriminants and `reason_code()` strings (`"NOT_ON_CALL"` /
  `"ALREADY_PRESENT"`, both a permanent machine contract):
  - **`NotOnCall`** — a call-element delete against an **existing** call found
    the element is not one of its BOM rows. Re-pointed site: the shared
    `derive_bom_exec_order` "not found" arm (comparison/expression/distinct
    delete). The three delete paths first call a new `ensure_call_exists` guard
    so a **non-existent call id** (which `CallSelector::Id` does not otherwise
    validate) stays a hard `NotFound` — "call ID N does not exist" — matching
    Python `prepCallElement`, which errors on a missing call record before ever
    looking at the BOM. `delete_standardize_call_element` is **not** re-pointed:
    standardize has no call/BOM split (the `CFG_SFCALL` row *is* the element) and
    no benign "not on call" concept, so any miss stays `NotFound` (its nearest
    Python parity, `deleteStandardizeCall`, hard-errors on a miss).
  - **`AlreadyPresent`** — a call/call-element add is a no-op: a per-feature
    comparison/distinct call is already set, or the element is already on the
    call (all four `add_*_call_element` duplicate checks).
  - Hard collisions are deliberately **unchanged**: a taken explicit id and a
    taken exec-order stay `AlreadyExists`; a genuinely missing id/lookup stays
    `NotFound`.
  - New `SzConfigError::message(&self) -> &str` returns the bare inner payload
    for every variant; `Display` output is byte-for-byte unchanged (both new
    variants render the bare message, exactly like their parents). New
    `not_on_call` / `already_present` constructors mirror the sibling variants.
  - **Breaking:** `SzConfigError` is not `#[non_exhaustive]`, so adding these
    variants breaks exhaustive downstream matches.
  - Verified against `tests/fixtures/g2config_template.json` (real Senzing v4
    template): the `NotOnCall` split across the three BOM-backed families, the
    `NotFound` guard for a missing call id (all families, standardize included),
    and a delete-path regression guard that a delete of an on-call element
    removes exactly one BOM row.
  - *Note (behavioural, Python-consistent):* a call-element delete now requires
    the `CFG_?CALL` section to contain the call id, so a delete against a
    BOM-only config fragment (no owning call row) now returns `NotFound` where it
    previously proceeded. This also closes an orphan-BOM bug where such a row
    could be deleted without its call existing.
- **`FilterSubstrate::ValuesJoin` filter substrate (#56)** reproducing Python
  `do_listComparisonThresholds`' filter rendering: the record's **values only**
  (keys dropped) are each rendered via Python `str()` semantics — bare unquoted
  strings, `None`/`True`/`False`, decimal numbers — and joined with a single
  space (`" ".join(str(v).lower() for v in record.values())`). New public
  `to_values_join_string(&Value)` renders a value under this substrate;
  `matches_filter` gains the corresponding arm. `JsonDumps` remains the default.
  Verified against real `CFG_CFRTN` rows from `tests/fixtures/g2config_template.json`.

### Fixed

- **Threshold cap fields now reject present-but-wrong-type values (#50).** A new
  module-private `optional_i64` helper backs every threshold param `TryFrom`:
  a missing or `null` field parses to `None`, an integer to `Some(n)`, but a
  present-but-wrong-type value (JSON string/float/bool) is now rejected as
  `InvalidInput` instead of being silently coerced to `None`. Previously
  `{"candidateCap": "500"}` was dropped by `.and_then(Value::as_i64)` and the
  update became a silent no-op. Wired into `AddComparisonThresholdParams`,
  `SetComparisonThresholdParams`, `AddGenericThresholdParams` and
  `SetGenericThresholdParams` `TryFrom<&Value>` (all their i64 fields —
  `execOrder`, the score fields, and the `candidateCap`/`scoringCap` caps). The
  `SzConfigTool_setGenericThreshold` FFI, which bypassed `TryFrom` with inline
  `.and_then(as_i64)` cap parsing, now routes through
  `SetGenericThresholdParams::try_from` so it inherits the same strict typing.
  - *Deliberate parity divergence:* for the **comparison-threshold** score/exec
    fields this is stricter than the Python CLI, which coerces digit-strings
    (`{"sameScore": "100"}` → `100`) via `validate_parms`; the SDK rejects a
    quoted number there as `InvalidInput`. Generic-threshold caps match Python
    exactly (Python also rejects non-int for those). A typed JSON library
    rejecting quoted numbers is the intended contract; pass JSON numbers.
- **`add_generic_threshold` aggregates validation like Python `sz_configtool`
  (#49).** Missing required fields (`plan`, `behavior`, `scoring_cap`,
  `candidate_cap`, `send_to_redo`) are now collected into a single
  `MissingField` error rather than reported one at a time, mirroring Python
  `do_addGenericThreshold`'s up-front `validate_parms`. The plan / feature /
  duplicate checks keep their fail-fast ordering, then a collect-all validity
  block (mirroring `validateGenericThreshold`) validates the `BEHAVIOR` code
  against the canonical 17-code set via `behavior_domain::behavior_position`
  (`BEHAVIOR_CODES`) — the exact set Python's `lookupBehaviorCode` checks, not
  the broader `parse_behavior_code` — and `SEND_TO_REDO` via
  `send_to_redo_canonical`, joining any failures into one `InvalidInput`. A
  bogus behaviour code on an existing plan is now rejected instead of written.
  All four changes are exercised against `tests/fixtures/g2config_template.json`
  (bogus-behaviour rejection, a valid new per-feature add, a no-op check that
  every shipped behaviour passes the new validator, and wrong-typed-cap
  rejection).
- **`thresholds::add_comparison_threshold` no longer regresses the CFRTN scoring
  tier order (part of #55).** It (and the FFI-facing
  `add_comparison_threshold_by_id`) now implement Python
  `do_addComparisonThreshold`'s three-step order logic via
  `resolve_cfrtn_exec_order`: an existing all-features (`FTYPE_ID = 0`)
  return-value tier row's `EXEC_ORDER` is **reused verbatim** (load-bearing
  scoring invariant — a naive max+1 there was a silent regression), else an
  explicit order is honoured-or-rejected, else the next free order within
  `(CFUNC_ID, FTYPE_ID = 0)` is allocated. `EXEC_ORDER` is never emitted as
  `null`. Verified against CFRTN tier reuse across the 20 shipped all-features
  tier rows in the real Senzing v4 template.
- **`command_processor` `addComparisonCallElement` scope bug fixed (part of
  #55).** It drops its manual next-order calc and passes `exec_order: None`,
  fixing a per-`(call, feature)` scope bug — it now numbers the whole call,
  per-`CFCALL_ID`.

### Changed

- **Execution-order auto-allocation policy across all add paths (#55).** Every
  add path that writes an `EXEC_ORDER` now resolves it through one shared helper,
  `helpers::get_desired_or_next_order(array, order_field, scope, desired)`
  (mirroring Python `getDesiredValueOrNext` with its default `seed_order` of 0):
  `None` auto-allocates the next free order within the row's scope, `Some(n > 0)`
  is honoured when free and **rejected with `AlreadyExists` when already taken**
  (SDK-wide reject-if-taken), and a value is always written as a concrete order,
  never `null`. Scopes: `(FTYPE_ID, FELEM_ID)` for `add_standardize_call` /
  `add_standardize_call_element` / `add_expression_call`; the call id for the
  comparison / expression / distinct `add_*_call_element` BOM rows; the whole
  table for `features::add_feature_comparison`.
  - **Breaking:** the **comparison call-element** (`AddComparisonCallElementParams`),
    **expression call-element** (`ExpressionCallElementParams`, and its `new()`
    signature) and **distinct call-element** (`AddDistinctCallElementParams`)
    `exec_order` fields changed from `i64` to `Option<i64>`; passing `None` now
    auto-allocates instead of the field being mandatory.
  - **`calls::distinct::add_distinct_call_element`** duplicate detection realigned
    to `(DFCALL_ID, FTYPE_ID, FELEM_ID)` (dropping `EXEC_ORDER` from the identity),
    matching the comparison/expression siblings and Python `addCallElement`.
  - The `add_feature` bulk builder and positional BOM loops (`EXEC_ORDER` by
    1-based list position) are deliberately outside this policy and unchanged.
  - New `calls` module "Execution-order policy" doc section (with a scope table)
    and a README mirror. Verified end-to-end against the real Senzing v4 template
    (`tests/fixtures/g2config_template.json`).
- **`settings::set_setting` now stores typed JSON values (#52).** *(Breaking —
  API + FFI.)* The `value` parameter changed from `&str` to
  `impl Into<serde_json::Value>` and the value is inserted verbatim (no
  `json!(value)` stringification), matching Python `do_setSetting` which stores
  the parsed value as-is (e.g. `setSetting {"name": "metaphone_version", "value": 3}`
  stores the integer `3`). A `&str` value still stores a JSON string, so most
  Rust callers are unaffected. The `SzConfigTool_setSetting` FFI now parses its
  `value` C-string as JSON and **returns an error on invalid JSON** — no string
  fallback; a bare string must be quoted JSON (`"\"hello\""`). The C ABI (3×
  `char*`) is unchanged; the header comment documents the new value semantics.
  Verified against `tests/fixtures/g2config_template.json`
  (`SETTINGS.METAPHONE_VERSION`).

## [0.6.3] - 2026-08-25

Patch: two parity/robustness fixes raised during the CLI's Wave-2 read-path re-delegation, verified
against the stock config and Python `sz_configtool` 4.4.0 and coordinated with the CLI. Both are
**no-op on shipped data** — the FTYPE guard closes a latent trap that the stock config can't hit, and
the filter fix corrects a wrapper the CLI had not yet adopted. Gates green: 331 test cases, clippy/fmt
clean.

### Fixed

- **`matches_filter` is now case-insensitive**, matching every `sz_configtool` list-filter site
  (`arg.lower() in str(record).lower()`). It previously did a case-sensitive `contains`, so calling
  the wrapper directly regressed filters such as `listRules none` to zero matches — which is why the
  CLI kept its own case-insensitive `.contains` rather than adopting it. Both the rendered record and
  the filter term are now case-folded before the substring test. The rustdoc is corrected (the
  "mirroring Python's `in`" note referred to the bare operator the tool never uses) and now records
  that the tool's actual `str(record)` substrate is `FilterSubstrate::PythonRepr`, so callers
  reproducing the tool's filtering exactly should pass `PythonRepr`.
- **`delete_{expression,comparison,distinct}_call_element` no longer risk over-deleting a sibling BOM
  row.** The target `EXEC_ORDER` is derived FTYPE-aware (via the element's feature), but the final
  `retain` matched only `(call_id, FELEM_ID, EXEC_ORDER)`. If one call ever held two BOM rows sharing
  the same element **and** `EXEC_ORDER` under different features, disambiguating by feature would
  derive the right row then drop its sibling too. Not reachable on the stock config (add paths keep
  `EXEC_ORDER` unique per call, and the no-feature path already errors as ambiguous before `retain`);
  each `retain` now mirrors the derive predicate as a belt-and-braces guard. (`standardize` was
  already FTYPE-inclusive and is unchanged.)

### Behaviour note

- The `matches_filter` change is observable: case-insensitive matching returns more results than the
  previous case-sensitive test for mixed-case terms. This is a parity fix (the SDK behaviour now
  matches the Python tool) rather than a regression.

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
