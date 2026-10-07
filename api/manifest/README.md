# Binding manifest — how to add a group

The manifest describes every configuration function once; `invoke`, the C
`SzConfigTool_invoke`, and every language binding are generated or checked
against it. Field reference: [schema.md](schema.md). Worked examples:
[datasources.yaml](datasources.yaml), [attributes.yaml](attributes.yaml) and
their [conformance](conformance/) files.

## Files you own (one group = one agent, no shared edits)

| File | Purpose |
|---|---|
| `api/manifest/<group>.yaml` | function entries + this group's `excluded:` list |
| `api/manifest/conformance/<group>.yaml` | conformance cases |

Do NOT hand-edit generated files (`api/src/dispatch_gen.rs`,
`api/manifest/generated/*.json`). If they conflict on merge, take either side
and re-run the generator. Avoid editing `excluded.yaml` (use your group's
`excluded:`), `project.yaml`, the generator, or `api/src/convert.rs`; if you
must add a converter, append a clearly-named fn with its own unit test.

## Steps

1. **Read the Rust source** of the module (`src/<module>.rs`) and its C
   wrappers in `ffi/src/lib.rs` (`grep -n '<module>::' ffi/src/lib.rs`) and
   header declarations. Every claim you record must come from that code (or a
   test run) — if unsure, trace it; never guess.
2. **Decide coverage**: every column-0 `pub fn` of the module is either a
   function entry or an `excluded:` entry with a reason. Run
   `cargo test -p sz-configtool-api --test drift -- --nocapture` to list
   `unmapped:` functions for your module.
3. **Write each function entry** (`schema.md` → Function fields):
   - `name` = the Rust fn name unless it collides across groups.
   - `rust` = DEFINING path (`calls::standardize::add_standardize_call`,
     not the `calls::` re-export).
   - Params struct → `rust_params_struct`; list EVERY struct field as an arg
     (the compiler rejects missing ones). Args the fn takes positionally before
     the struct get `positional: true`; use `field:` when the wire name should
     differ from the struct field.
   - Types: `FieldUpdate<T>` fields → `tristate: true` (ONLY those);
     `Option<T>` → `optional: true` (add `required: true` when the library
     rejects absent with MISSING_FIELD); non-wire Rust types (`usize`,
     slices) → `rust_convert`; a `CallSelector` (id OR feature code) →
     `type: int_or_str` + `rust_convert: call_selector`; every `type: json`
     arg needs a `json_type` (its structure from the Rust code, or `any`; see
     schema.md).
   - `returns`: `(String, T)` → `config_and_json` (never drop T); 3+-tuples
     add `tuple_names`; a non-config `String` (e.g. a version) is `json`; a
     non-config tuple is `json` + `tuple_names` (never a bare array).
   - Placeholders that always fail NOT_IMPLEMENTED: `status: not_implemented`.
   - `c_symbol`: the typed C export or `null`; legacy duplicates (e.g.
     `set*FunctionWithJson`) go in `c_aliases` — expose ONE typed function.
   - `errors`: reason codes the library can actually return (read the code).
   - `semantics`/`notes`: record normalization, `<= 0`/absent sentinels,
     auto-allocation (ids, exec order) — language-neutral only.
   - `c_notes`: every C-export delta (params not exposed, empty string =
     None, negative int = None, dropped tuple halves, non-standard return
     codes, camelCase update keys). Never put C-ABI text in `notes`/`semantics`
     (codegen rejects it; those render into every binding's docs).
4. **Write conformance cases**: at least one per function, covering success
   (verified through a later get/list step for `config` results) and each
   listed error you can trigger from the template fixture. Use the inline form
   for single calls, `steps` for sequences. Quote `"Yes"`/`"No"`.
5. **Generate and test**:
   ```bash
   cargo run -p sz-configtool-codegen
   cargo test -p sz-configtool-api -p sz-configtool-codegen
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo fmt --all
   ```
   A compile error in `dispatch_gen.rs` means the manifest does not match the
   Rust signature — fix the YAML, not the generated file. A failing
   conformance case means the expectation or the recorded semantics are wrong:
   re-read the code before changing either.

## Done when

- No `unmapped:` lines for your module; every function has a conformance case.
- All commands above pass; generated files are fresh (`-- --check`).
- When ALL groups are done, set `enforce_complete: true` in `excluded.yaml`;
  the drift test then fails on any new unmapped `pub fn`.

## Remaining groups (A2)

Root-lib `pub fn` counts per module (A1 mapped `datasources` 5, `attributes` 5):

| Module | pub fns | Module | pub fns |
|---|---|---|---|
| features | 16 | functions/standardize | 6 |
| thresholds | 14 | functions/expression | 6 |
| elements | 10 | functions/comparison | 6 |
| calls/standardize | 8 | functions/distinct | 5 |
| calls/expression | 8 | functions/candidate | 6 |
| calls/comparison | 8 | functions/matching | 6 |
| calls/distinct | 8 | functions/scoring | 6 |
| config_sections | 7 | functions/validation | 6 |
| behavior_overrides | 5 | fragments | 5 |
| rules | 5 | generic_plans | 4 |
| search_profiles | 4 | | |
| versioning | 4 | system_params | 2 |
| settings | 1 | export | 1 |
| validation | 1 | behavior_domain | 3 |
| filter | 4 | helpers | 28 |

`behavior_domain`, `filter` and `helpers` are mostly non-config utilities
(not config-first, or not `Result`): likely exclusions, decided per function.
