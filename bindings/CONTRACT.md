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
  SzConfig `export()` style):

  | `returns` | `tuple_names` | Typed result |
  |---|---|---|
  | `config` | — | config string |
  | `json` | — | JSON text |
  | `json` | `[a, b]` | record `<Fn>Result` with fields `a`, `b` (each JSON text) |
  | `config_and_json` | — | record `ConfigAndJson` with `config` + `json` (record JSON text) |
  | `config_and_json` | `[a, b]` | record `<Fn>Result` with `config` + fields `a`, `b` (each JSON text) |
  | `int` | — | integer |
  | `unit` | — | nothing |

  `<Fn>` is the PascalCase function name (e.g. `SetGenericPlanResult`); field
  names follow the language's casing (Python/C++ `plan_id`, Java/TS `planId`,
  C# `PlanId`). Python uses `NamedTuple`s, Java records, C# records, C++
  structs, TS interfaces. A field's JSON text is exactly the record member's
  JSON (`1001`, `true`, `"4.0.0"` including quotes).

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
  comments (Python docstring + .pyi, Javadoc, XML doc, Doxygen, TSDoc).
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
`UnicodeEncodeError`). C# throws `ArgumentNullException` for a null
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
built native library (cases flagged `wire_only` call the raw `invoke` seam),
plus tests for naming, optional/tri-state/required args, error mapping (all 14
reason codes where reachable), config opacity (byte-exact round trip), and the
absence of leaked `SzConfigTool_*` exports (Rust-native seams).

## Docs and delivery

Per-binding README with install-from-GitHub-Release instructions and a runnable
example; nothing is published to public registries (GitHub Releases only).
Release packaging/CI is assembled separately; bindings expose a single build
command documented in their README.
