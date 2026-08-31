# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
