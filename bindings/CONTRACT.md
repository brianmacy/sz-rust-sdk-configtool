# Language binding contract

Every binding exposes the same functions, generated from `api/manifest/*.yaml`
(see `api/manifest/schema.md`). This document is the shared contract; each
language owns only `tools/codegen/src/lang/<lang>.rs` and `bindings/<lang>/**`.

## Architecture (uniform)

```
generated typed wrappers (host language)   <- tools/codegen/src/lang/<lang>.rs
        |  name + config + args JSON
native seam: invoke(name, config, args_json) -> (kind, config?, result?)
        |  Rust: sz-configtool-api::invoke      C#/C++: SzConfigTool_invoke (C ABI)
sz_configtool_lib (pure Rust)
```

* Rust-native seams (pyo3, jni, napi-rs) are tiny hand-written crates under
  `bindings/{python,jni,node}` that depend on `sz-configtool-api` — never on
  `sz-configtool-ffi` (no `SzConfigTool_*` symbol may be exported from them; a
  test asserts this with `nm`/`dumpbin`).
* C# (P/Invoke) and C++ (header-only C++20) call `SzConfigTool_invoke` from the
  C ABI library `libSzConfigTool`. Do NOT edit `ffi/`; if you need a C ABI
  change, report it.
* Wrappers are generated; checked-in; `cargo run -p sz-configtool-codegen`
  regenerates, `-- --check` fails CI when stale. Output file locations come
  from `api/manifest/project.yaml` `bindings:` (never hardcoded in a generator).

## Wire (see `api/src/lib.rs`, `api/manifest/schema.md`)

* `config` is an OPAQUE JSON string: never parsed/re-serialized by wrappers.
* `args_json` is a JSON object keyed by the manifest's snake_case arg names.
  Key absent = leave/None; `null` = clear (tri-state args only); value = set.
* Success envelope `{"kind": "config|json|config_and_json|int|unit", "config"?, "result"?}`;
  `config` travels as an escaped JSON string; `result` is JSON.
* Failure: error with `kind`, `reason_code` (one of `project.yaml reason_codes`),
  `message`, optional `details` (validation-failures/v1 JSON).
* Return types in typed wrappers (every value that is not the config or an
  `int` is JSON TEXT — a string — never a parsed object, matching the official
  SzConfig `export()` style). EVERY config-changing function returns the new
  config string, so calls chain uniformly (`cfg = f(cfg, ...)`). A
  `config_and_json` function (the wire carries config AND a record) is split
  into two typed functions with the SAME parameters/overloads/options type
  that perform the same operation: the primary `<fn>` returns the config, the
  companion `<fn>Result` returns only the record (the created/removed row,
  ids or counts):

  | `returns` | `tuple_names` | Typed result |
  |---|---|---|
  | `config` | — | config string |
  | `json` | — | JSON text |
  | `json` | `[a, b]` | record `<Fn>Record` with fields `a`, `b` (each JSON text) |
  | `config_and_json` | — | `<fn>` → config string; `<fn>Result` → record JSON text |
  | `config_and_json` | `[a, b]` | `<fn>` → config string; `<fn>Result` → record `<Fn>Record` with fields `a`, `b` (each JSON text) |
  | `int` | — | integer |
  | `unit` | — | nothing |

  Companion names follow each language's casing of `<name>_result`: Python
  `add_attribute_result`, Java/TS `addAttributeResult`, C#/C++
  `AddAttributeResult`. `<Fn>Record` is the PascalCase function name +
  `Record` (e.g. `SetGenericPlanRecord`, `VerifyCompatibilityVersionRecord`);
  it is not `<Fn>Result` because in C# and C++ the companion METHOD is
  `<Fn>Result` (a C++ function and struct of one name cannot coexist
  usefully). There is no `ConfigAndJson` type. Field names follow the
  language's casing (Python/C++ `plan_id`, Java/TS `planId`, C# `PlanId`).
  Python uses `NamedTuple`s, Java records, C# records, C++ structs, TS
  interfaces. A field's JSON text is exactly the record member's JSON
  (`1001`, `true`, `"4.0.0"` including quotes). Codegen rejects a manifest
  function named like another's companion. The tRPC router has a procedure
  per typed function: config-returning ones are mutations, companions are
  queries.
* JSON results follow the library's response shape convention (repo
  `CLAUDE.md`; summary in `api/manifest/schema.md`, Wire convention): `get_*`
  return the stored row (except `get_feature`, `get_element`, `get_fragment`,
  `get_rule`), `list_*` return code-resolved summaries. Bindings add no
  `describe_*`/view layer. The three call lists (`list_expression_calls`,
  `list_comparison_calls`, `list_distinct_calls`) omit the stored BOM columns;
  raw BOM rows come from `get_config_section` (`CFG_EFBOM`/`CFG_CFBOM`/`CFG_DFBOM`).

## Typed API rules

* Functional/stateless: `f(config_json, ...) -> str`. Class names must NOT
  imitate engine-bound official classes (`SzConfig`, `SzConfigManager`).
* Skip functions with `status: not_implemented` in typed wrappers (they stay in
  `invoke`). Skip nothing else.
* Args with `required: true` are required even if the Rust type is Option.
* `int_or_str` args (call selectors: a call id OR a feature code) take the
  natural union: Python `int | str`; Java and C# overloads (`long` / `String`);
  C++ overloads (`std::int64_t` / `std::string_view`); TS `number | bigint | string`.
  The wire value is a JSON integer or string; `invoke` is unchanged.
  Optional args use the language's natural optional form; tri-state args need
  leave/clear/set (Python `UNSET`/None; Java/C#/C++ `FieldUpdate<T>`; TS
  undefined/null/value).
* Each function's doc/semantics/notes/errors from the manifest become doc
  comments (Python docstring + .pyi, Javadoc, XML doc, Doxygen, TSDoc). A
  structured `json_type` adds a `Shape: ...` line to the arg's doc.
* Unknown or misspelled argument names never pass silently. Python
  (keyword-only parameters: `TypeError`), Java (typed `Options` builders), C#
  (named parameters / options objects) and C++ (options structs) reject them
  at call/compile time. TS options are plain objects, so the TS wrappers check
  them at run time: a non-object `options`, an unknown key (top level or inside
  a structured `json` option) or a value outside the arg's `json_type` is
  `INVALID_INPUT` before the native call (a missing required key inside one:
  `MISSING_FIELD`), and native errors that name wire fields are translated to
  the camelCase option name with the wire name in parentheses once
  (`genericPlan (generic_plan)`). TS types and the tRPC Zod schemas
  (strict objects) are generated from `json_type`; elsewhere `json` args stay
  JSON values / JSON text (unknown keys inside them are judged by the library).
* Names: Python snake_case; Java camelCase; TS camelCase; C# PascalCase
  methods/camelCase args; C++ PascalCase methods/snake_case args. Split the
  snake name on `_`, no acronym special-casing.

## Errors (one class per language; no reuse of engine-semantics classes)

The error KIND is the reason code (schema.md, Errors): every binding exposes
both the reason code string and `kind`, and `kind` has the reason code's
identity (the same string, or an enum constant generated from
`reason_codes`). The TYPE of `kind` differs by language, the set of values
does not: an enum in Java (`SzConfigToolErrorKind`), C# (`SzConfigToolErrorKind`)
and C++ (`ErrorKind`), one constant per reason code; a plain string in Python
(`str`) and TS (the `ReasonCode` string-literal union), equal to the reason
code. `details` is the `sz-configtool.validation-errors/v1` JSON
for `VALIDATION_ERRORS` (built once, by `sz_configtool_api::validation_details_json`),
exposed as JSON TEXT (a string, never a parsed object) in every binding; a
binding may add a typed parsing helper (TS `validationDetails()`).

| Language | Class | Reason code / kind |
|---|---|---|
| Python | `SzConfigToolError(Exception)` | `reason_code`; `kind` (same string); `message`, `details` (JSON text) |
| Java | `SzConfigToolException extends Exception` (checked) | `getReasonCode()`; `getKind()` (`SzConfigToolErrorKind`); `getDetails()` |
| C# | `SzConfigToolException : Exception` | `ReasonCode`; `Kind` (`SzConfigToolErrorKind`); `Details` |
| C++ | `SzConfigToolException : std::runtime_error` | `ReasonCode()`; `Kind()` (`ErrorKind`); `Details()` |
| TS | `SzConfigToolError extends Error` | `code` / `reasonCode` / `errorType`; `kind` (same `ReasonCode`); `details` (JSON text); `validationDetails()` (parsed) |

Python deliberately has NO subclasses named like the official SDK's
(`SzNotFoundError`, `SzBadInputError`): a same-named class that is not the
official one would silently miss users' `except senzing.SzBadInputError`.

## Version accessors (one source)

Both values have ONE definition, `sz_configtool_api::{LIBRARY_VERSION,
ABI_VERSION}` (the C ABI's `SzConfigTool_getLibraryVersion` /
`SzConfigTool_getAbiVersion` return the same constants). `library_version` is
the workspace version (`[workspace.package] version`); `abi_version` is the C
ABI version (`SZCONFIGTOOL_ABI_VERSION`). Every binding tests that its library
version equals the workspace version. Never put them in a generated class.

| Language | Library version | ABI version |
|---|---|---|
| Python | `__version__`, `library_version()` | `abi_version()` |
| Java | `SzConfigToolVersion.libraryVersion()` | `SzConfigToolVersion.abiVersion()` |
| C# | `SzConfigTool.LibraryVersion` | `SzConfigTool.AbiVersion` |
| C++ | `LibraryVersion()` | `AbiVersion()` (+ `AbiCompatible()`) |
| TS | `libraryVersion()` | `abiVersion()` |

## Input that cannot cross the boundary unchanged

No binding may silently alter input on the way to Rust. A string with a lone
UTF-16 surrogate has no UTF-8 form (JNI and napi would substitute U+FFFD); as
`config` or as an argument it is `INVALID_INPUT` in EVERY binding, raised as
the binding's error class before the call (Java, TS, C# strict encoder,
Python), and in args JSON `serde_json` also rejects lone-surrogate escapes
with `INVALID_INPUT`. Python also maps a `name` or `config` that is not a
`str`, and an arg JSON cannot carry (`set`, `Decimal`, `bytes`, any object,
`NaN`/`±Infinity`), to `INVALID_INPUT` (never `TypeError` / `ValueError` /
`UnicodeEncodeError`). Python and TS reject a `config` that is not a string
(e.g. a previous call's record passed back) with `INVALID_INPUT` before the
native call, naming the cause ("config must be a str/string (the
configuration JSON text) ...; use `<name>_result`/`<name>Result` for the
created row"). C# throws `ArgumentNullException` for a null
`name`/`config` (a programming error, not input). TS also rejects numbers `JSON.stringify` would change (`NaN`/`±Infinity`
become `null` — a tri-state Clear — and `number` integers beyond
`Number.MAX_SAFE_INTEGER` are already rounded) with `INVALID_INPUT`; TS `int`
args (and values inside `json` args) also take a `bigint`, serialized as its
exact digits, so the full i64 range crosses unchanged (a `bigint` outside i64
is `INVALID_INPUT`). TS named-result fields are the exact source text of each
member (scanned, never re-serialized) on every supported Node version.

NUL (U+0000) is a valid character, so whether it can cross depends on the
seam:

| Binding | NUL in `name` / `config` | NUL in a typed string arg |
|---|---|---|
| Python, Java, TS (Rust-native seams) | passed unchanged; the library judges it (`INVALID_INPUT` unknown function / `JSON_PARSE`) | passed unchanged (JSON `\u0000`) |
| C#, C++ (C ABI, NUL-terminated strings) | `INVALID_INPUT` before the call (also NUL in raw `args_json`) | passed unchanged (JSON `\u0000`) |

## Names and layout

| Binding | Path | Identity |
|---|---|---|
| Python | `bindings/python` | distribution `sz-configtool`, module `sz_configtool`, maturin, abi3-py310, pyo3 >= 0.29 |
| Java | `bindings/jni` (Rust) + `bindings/java` (Java) | package `io.github.brianmacy.szconfigtool`, class `SzConfigTool`, JDK 17, natives bundled in the jar and extracted (hashed dir + atomic rename) |
| C# | `bindings/csharp` | namespace `Sz.ConfigTool`, class `SzConfigTool`, netstandard2.0, `runtimes/<rid>/native/` layout |
| C++ | `bindings/cpp` | header `szconfigtool.hpp`, namespace `szconfigtool`, CMake package |
| TS/Node | `bindings/node` (+ `bindings/node/trpc`) | napi-rs v3, `.node` per target, tRPC router over it |

## Tests (real library, no mocks)

Each binding runs `api/manifest/generated/conformance.json` against the real
built native library (cases flagged `wire_only` call the raw `invoke` seam;
a `config_and_json` step calls the primary AND its companion, which must
agree with `invoke` and fail with the same reason code),
plus tests for naming, optional/tri-state/required args, error mapping (all 14
reason codes where reachable), config opacity (byte-exact round trip), and the
absence of leaked `SzConfigTool_*` exports (Rust-native seams).

## Docs and delivery

Per-binding README with install-from-GitHub-Release instructions and a runnable
example; nothing is published to public registries (GitHub Releases only).
Release packaging/CI is assembled separately; bindings expose a single build
command documented in their README.
