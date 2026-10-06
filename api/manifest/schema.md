# Binding manifest schema

The manifest is the single, hand-maintained description of the configuration
API that every binding is generated or checked against. It is YAML, one file
per **group**, in `api/manifest/`. `cargo run -p sz-configtool-codegen`
validates it and writes the checked-in outputs named in `project.yaml`:

| Output | Consumer |
|---|---|
| `api/src/dispatch_gen.rs` | `sz_configtool_api::invoke` (C `SzConfigTool_invoke`, RPC) |
| `api/manifest/generated/manifest.json` | drift tests; other-language generators (no YAML parser needed) |
| `api/manifest/generated/conformance.json` | the Rust conformance test; every language's conformance runner |

Each output starts with `GENERATED — do not edit`. A test
(`tools/codegen/tests/up_to_date.rs`) fails when an output is stale;
`cargo run -p sz-configtool-codegen -- --check` does the same for CI.

YAML is parsed with `serde_yaml_ng` (YAML 1.2: `yes`/`no` are strings). Still
**quote** `"Yes"`, `"No"`, `"yes"` and similar so other YAML 1.1 tools read
them the same way. Unknown keys are errors everywhere.

## project.yaml

| Field | Meaning |
|---|---|
| `root_crate` | Crate name the dispatcher calls (`sz_configtool_lib`). |
| `paths.*` | Workspace-relative paths: `root_src`, `c_header`, `fixture`, `manifest_dir`, `conformance_dir`, `excluded`, `dispatch_out`, `manifest_json_out`, `conformance_json_out`. Code never hardcodes these. |
| `reason_codes` | The complete wire error taxonomy (see [Errors](#errors)). |
| `bindings.<lang>.*` | Every per-language generated FILE, by role (`python`: `module`, `stub`, `init`, `test_paths`; `java`/`cpp`: `api`, `error_kinds`, `test_dispatch`; `csharp`: `api`, `error_kinds`; `node`: `package_dir`, `functions`, `reason_codes`, `trpc_schemas`, `trpc_router`, `test_paths`). Generators never hardcode output locations. |

## Group files: `api/manifest/<group>.yaml`

```yaml
group: datasources        # must equal the file name
functions:
  - name: add_data_source # ...one entry per function
excluded:                 # optional: this group's exclusions (see excluded.yaml)
  - rust: datasources::some_internal_fn
    reason: Why it is not exposed
```

### Function fields

| Field | Req | Meaning |
|---|---|---|
| `name` | yes | snake_case, unique across ALL groups. The wire name for `invoke`; languages derive their casing from it (Java/TS camelCase, C# PascalCase). |
| `group` | no | Implied by the file; if given must equal it. Emitted per function in manifest.json. |
| `doc` | yes | One-line description (becomes each binding's doc comment). |
| `rust` | yes | Root-library fn at its **defining** module path, e.g. `datasources::add_data_source`, `calls::standardize::add_standardize_call` (not a re-export path). Compile-checked by the generated dispatcher; the drift test requires the defining path. |
| `rust_params_struct` | no | When the Rust fn takes a params struct, its path (e.g. `datasources::AddDataSourceParams`). See [Rust call mapping](#rust-call-mapping). |
| `c_symbol` | yes (may be `null`) | The typed C export implementing this function, or `null` if none. Must be declared in the header. |
| `c_aliases` | no | Other (legacy) C exports for the same operation, e.g. a `set*FunctionWithJson` variant. The manifest exposes ONE typed function; aliases are recorded only so the header drift test covers them. |
| `args` | no | Ordered argument list (after the implicit config). Default `[]`. |
| `returns` | yes | `config` \| `json` \| `config_and_json` \| `int` \| `unit`. |
| `tuple_names` | no | Names the values of a Rust tuple return so bindings get an object, never a bare array. `config_and_json` with `(String, A, B, ...)`: names for A, B, ...; the record becomes `{"<name0>": A, "<name1>": B}` (omit for a 2-tuple: record = the second value). `json` with a non-config tuple `(A, B, ...)`: the result becomes `{"<name0>": A, "<name1>": B}`. At least 2 names. |
| `errors` | yes | Reason codes the LIBRARY can return for this function (not the universal wire errors below). Conformance `expect.error` must be listed here. |
| `notes` | no | Language-neutral traps and semantics (see [Recording semantics](#recording-semantics)). Rendered into every binding's docs. Must not contain C-ABI text (codegen rejects `SzConfigTool_`, `C export`, `C typed`, `returnCode`, ... in `doc`/`notes`/`semantics`). |
| `c_notes` | no | C-ABI-only deltas: typed `SzConfigTool_*` export differences, numeric return codes, NULL/empty-string coercions. Emitted in manifest.json ONLY — never rendered into Python/Java/C#/C++/TS docs. The C header (`ffi/include/libSzConfigTool.h`) and `docs/c-api.html` are hand-written, so `c_notes` is reference data for their maintainers (no generated C artifact renders it). |
| `status` | no | `implemented` (default) \| `not_implemented`. `not_implemented` marks a placeholder that ALWAYS fails with `NOT_IMPLEMENTED` (its `errors` must list it, and every conformance step calling it must expect it). It stays in the manifest and in `invoke`; typed language generators SKIP it. Always emitted in manifest.json. |

### Arg fields

| Field | Req | Meaning |
|---|---|---|
| `name` | yes | snake_case; the `args_json` key, and the language parameter name. |
| `type` | yes | `str` \| `int` (i64) \| `bool` \| `json` (any JSON value) \| `str_list` (array of strings) \| `int_or_str` (a JSON integer OR string, e.g. a call selector: id or feature code; requires `rust_convert`; typed bindings expose a natural union / overloads). |
| `optional` | no | `true`: may be absent (Rust `Option<T>`). |
| `tristate` | no | `true`: absent = Leave, `null` = Clear, value = Set (Rust `FieldUpdate<T>`; `str`/`int` only; implies optional). Only for fields whose Rust type is `FieldUpdate`. |
| `required` | no | With `optional: true` only (not `tristate`, not `default`): the Rust type is `Option<T>` but the LIBRARY rejects absent with `MISSING_FIELD`. Typed bindings MUST make it a required parameter (and pass `Some`). `invoke` is unchanged: absent reaches the library as `None`. Always emitted in manifest.json (`false` by default). |
| `default` | no | DOCUMENTARY: the value the LIBRARY applies when absent. Bindings must NOT substitute it; they omit the arg. |
| `semantics` | no | Normalization, sentinel values, validation (required wherever behaviour is not obvious). |
| `field` | no | With `rust_params_struct`: the struct field name, if it differs from `name`. |
| `positional` | no | With `rust_params_struct`: pass this arg positionally instead of as a struct field. |
| `owned` | no | `json` only: the Rust fn takes an owned `serde_json::Value` instead of `&Value`. |
| `rust_convert` | no | Name of a converter fn in `api/src/convert.rs` producing a non-wire Rust type (enum, `usize`, slice, ...). See that module's docs. |

`rust_convert: required_str_as_some` (a plain, NON-optional `str` arg whose
Rust field is `Option<&str>`) is equivalent to `optional: true` +
`required: true`: both make the typed parameter required and reject absent
with `MISSING_FIELD`. The difference is only where that error comes from —
the converter rejects absent at the wire (universal `MISSING_FIELD`), while
`required: true` passes `None` to the library, which returns it (so it must
be in `errors[]`). Prefer `required: true` for new entries.

List converters (`expression_element_list`, `search_profile_elements`)
report a missing required key inside an item as `MISSING_FIELD` and a
wrong-typed value, an unknown key, or a non-object item as `INVALID_INPUT`.

### Rust call mapping

Mechanical, so typed shims (pyo3, JNI, napi) can be generated to call the
root library DIRECTLY with the same rules:

1. The first Rust parameter is always the config `&str`.
2. Without `rust_params_struct`: every arg, in manifest order, is a positional
   parameter.
3. With `rust_params_struct`: args marked `positional: true` are passed
   positionally (in order) after the config; then ONE struct literal of
   `rust_params_struct` is passed last, with every other arg as a field
   (`field` or `name`). Every struct field must be covered — no
   `..Default::default()`; a missing field is a compile error in
   `dispatch_gen.rs`.
4. Rust type per arg: `str`→`&str`, `int`→`i64`, `bool`→`bool`,
   `json`→`&Value` (`owned`: `Value`), `str_list`→`Vec<String>`,
   `int_or_str`→ only via `rust_convert`;
   `optional`→`Option<T>` (also when `required`: the typed parameter is
   required, passed as `Some`); `tristate`→`FieldUpdate<T>`; `rust_convert`
   overrides all of these.
5. Rust return per `returns`: `config`→`Result<String>`;
   `json`→`Result<T: Serialize>` (a `Result<String>` that is NOT a config,
   e.g. a version string, is `json`); `config_and_json`→`Result<(String, T…)>`;
   `int`→`Result<i64>`; `unit`→`Result<()>`. With `tuple_names`, the tuple
   values are mapped to the named object fields in order.

Functions not taking a config first, or not returning `Result`, are not
supported by the generator: exclude them (or extend the generator first).

## Wire convention (`invoke` / `SzConfigTool_invoke` / RPC)

- `args_json` is a JSON **object** keyed by arg `name`.
  - key absent → Leave (tri-state) / None (optional) / missing (required).
  - `null` → Clear, for **tri-state args only**; `null` for any other arg is
    `INVALID_INPUT` (never silently "absent").
  - value → Set; it must match the arg `type` (`int` must be an integer).
  - unknown keys → `INVALID_INPUT`.
- `config` is an **opaque string**: wrappers never parse or re-serialize it.
  The envelope carries it as a JSON *string* (escaped, byte-exact).
- Envelope: `{"kind": "<returns>", "config"?: "<string>", "result"?: <value>}`.
  `config` present for `config` and `config_and_json`; `result` for `json`,
  `config_and_json` (the record — never dropped) and `int`; `unit` has
  neither. The envelope `result` is a JSON value, but typed bindings return
  JSON TEXT (a string), never a parsed object: `config` → config string;
  `json` → JSON text; `config_and_json` → record `ConfigAndJson(config,
  json)`; with `tuple_names` → record `<Fn>Result` (plus `config` for
  `config_and_json`) whose named fields are each JSON text; `int` → integer;
  `unit` → nothing. See `bindings/CONTRACT.md`.
- `json` results follow the response shape convention (repo `CLAUDE.md`,
  "Response shape convention"): `get_*` return the stored row (except the
  summary gets `get_feature`, `get_element`, `get_fragment`, `get_rule`);
  `list_*` return code-resolved summaries, with raw rows via
  `get_config_section`. `list_expression_calls` / `list_comparison_calls` /
  `list_distinct_calls` give `elementList` as bare element codes, omitting the
  stored BOM columns; read `CFG_EFBOM` / `CFG_CFBOM` / `CFG_DFBOM` with
  `get_config_section` for them.

## Errors

The taxonomy is the 13 `SzConfigError::reason_code()` values plus `INTERNAL`:

| Reason code | `SzConfigError` | Typical cause |
|---|---|---|
| `JSON_PARSE` | `JsonParse` | the CONFIG is not valid JSON |
| `NOT_FOUND` | `NotFound` | referenced code/id does not exist |
| `NOT_ON_CALL` | `NotOnCall` | call-element delete: element not on the call |
| `NOT_IN_FEATURE` | `NotInFeature` | call-element op: element not in the feature |
| `ALREADY_EXISTS` | `AlreadyExists` | duplicate code, or explicit id/order taken |
| `ALREADY_PRESENT` | `AlreadyPresent` | benign no-op add (call / element already there) |
| `INVALID_INPUT` | `InvalidInput` | bad value; also all wire errors below |
| `MISSING_SECTION` | `MissingSection` | required `CFG_*` section absent |
| `INVALID_STRUCTURE` | `InvalidStructure` | malformed config structure |
| `MISSING_FIELD` | `MissingField` | required field absent (incl. a required arg) |
| `INVALID_CONFIG` | `InvalidConfig` | invalid configuration state |
| `NOT_IMPLEMENTED` | `NotImplemented` | stubbed operation |
| `VALIDATION_ERRORS` | `ValidationErrors` | aggregated field failures; details JSON `sz-configtool.validation-errors/v1` |
| `INTERNAL` | — (`ApiError::Internal`) | result serialization failure or caught panic |

Universal wire errors (valid for EVERY function, not listed in `errors[]`):
unknown function name, unparsable / non-object `args_json`, unknown or
mistyped arg, `null` for a non-tri-state arg → `INVALID_INPUT`; missing
required arg → `MISSING_FIELD`; `INTERNAL`.

Bindings expose the reason code (and validation details) as the error KIND:
every binding's error has both the reason code string and a `kind` with the
same identity (the string itself, or an enum generated from `reason_codes`).
The numeric C `returnCode` (-1 boundary, -2 library) is NOT the taxonomy;
some legacy typed C exports use other codes (recorded in `c_notes`).

## Recording semantics

Record in `semantics` / `notes` (verified against code, never guessed):

- Normalization: uppercasing, case-insensitive domains, values stored verbatim.
- Sentinels in the ROOT library: e.g. `id` absent or `<= 0` = auto-allocate;
  `addRule` `ERRULE_ID` 0/negative = auto-allocate (`rules.rs`); exec-order
  auto-allocation when omitted (`calls/mod.rs`).
- C typed-export deltas (in `c_notes`, never `notes`): params not exposed (e.g.
  `SzConfigTool_addDataSource` only takes the code), dropped tuple halves
  (`addAttribute` drops the row), empty string = None and negative int = None
  coercions in the C wrappers (`ffi/src/lib.rs`), camelCase update-JSON keys,
  non-standard return codes. `invoke` has none of these: it exposes the root
  library's semantics exactly.

## excluded.yaml

```yaml
enforce_complete: false   # flip to true when coverage is complete
entries:
  - rust: helpers::get_next_id          # one fn, or
    reason: Internal id allocator
  - module: filter                      # every pub fn in a module file
    reason: Display-substrate helpers, not config operations
```

A group file may also carry its own top-level `excluded:` list (same entry
shape); it is merged into these entries. Prefer that for exclusions of the
group's own module, so parallel authors never edit `excluded.yaml`.

The coverage drift test (`api/tests/drift.rs`) scans column-0 `pub fn`s in
public modules of `paths.root_src`. Always fatal: a manifest/excluded `rust`
that is not a root pub fn at its defining path, an unknown `module`, a
duplicate exclusion, a function both mapped and excluded. Unmapped functions
are printed (`cargo test -p sz-configtool-api --test drift -- --nocapture`)
and become fatal once `enforce_complete: true`.

## Conformance files: `api/manifest/conformance/<group>.yaml`

```yaml
group: datasources
cases:
  - name: add_then_get            # unique within the group
    steps:                        # run in order on one evolving config
      - fn: add_data_source
        args: {code: crm}
        expect: {kind: config}
      - fn: get_data_source
        args: {code: CRM}
        expect: {result: {DSRC_ID: 1000}}
  - name: get_missing             # inline single-step form
    fn: get_data_source
    args: {code: NOPE}
    expect: {error: NOT_FOUND}
```

Every case starts from the REAL fixture (`paths.fixture`). A step may set
`config_literal` to run on that literal text instead (for `JSON_PARSE` /
`MISSING_SECTION` cases). After a successful step whose output carries a
config, that config becomes the current one; a failing step leaves it.

`expect` (all optional; empty = must succeed):

| Key | Rule |
|---|---|
| `error` | must fail with this reason code; excludes every other key; must be in the function's `errors[]`. |
| `kind` | output kind must equal it (and must equal the function's `returns`). |
| `result` | SUBSET match against the result value (json result, record, or int). |
| `contains` | result MUST be an array; each listed value subset-matches SOME element. |
| `excludes` | result MUST be an array; NO element subset-matches any listed value. |
| `len` | result MUST be an array of exactly this length. |

A non-array result FAILS `contains`/`excludes`/`len` (it is never treated as
`[]`, which would make `excludes` and `len: 0` pass vacuously). Codegen rejects
them at generation time for functions whose result cannot be an array
(`int`, `config_and_json` records, `json` with `tuple_names`).

Subset match: objects — every expected key present with a subset-matching
value (expected `null` requires an actual `null`); arrays — same length,
element-wise subset; scalars — JSON equality. `result`/`contains`/`excludes`/
`len` are rejected for `config`/`unit` functions (configs are opaque: verify
them with a later get/list step).

Cases must be expressible through TYPED bindings: codegen rejects unknown
args, missing required args, wrong types and `null` for non-tri-state args.
One exception: a step may omit a `required: true` arg ONLY when it expects
`MISSING_FIELD`; codegen marks such steps `"wire_only": true` in
conformance.json (computed — never written in YAML). Typed runners execute
`wire_only` steps through `invoke` (or skip them).
Wire-only error behaviour is tested in `api` and `ffi` unit tests instead.
Every manifest function needs at least one case
(`test_every_manifest_function_has_a_conformance_case`).
