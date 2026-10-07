# sz-configtool (Python)

Typed, stateless Python functions for editing Senzing configuration JSON
(`g2config.json`), backed by the Rust `sz_configtool_lib` through a small pyo3
extension. **Unofficial**: this is not the engine-bound Senzing `SzConfig` API,
and it does not depend on the `senzing` package.

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

* Distribution `sz-configtool`, import package `sz_configtool`.
* Python 3.10+ (one `abi3` wheel per platform covers every later version).
* Every function takes the configuration JSON string first and returns a new
  string; the config is opaque and is never parsed or re-serialized by Python.
* Functions, arguments and docstrings are generated from
  `api/manifest/*.yaml` (`cargo run -p sz-configtool-codegen`); do not edit
  `python/sz_configtool/generated.py[i]` or `__init__.py` by hand.

## Install (GitHub Release wheel)

Wheels are attached to the project's
[GitHub Releases](https://github.com/brianmacy/sz-rust-sdk-configtool/releases)
(nothing is published to PyPI). Download the wheel for your platform, then:

```bash
pip install ./sz_configtool-<version>-cp310-abi3-<platform>.whl
```

`<version>` is the PEP 440 form of the release version (`4.4.0.post1` for
`v4.4.0-1`, `4.5.0rc1` for `v4.5.0-rc.1`). Verify the wheel with the release's `SHA256SUMS`
and attestation (root README, "Pre-built packages"); its CycloneDX SBOM is the
release asset `sz-configtool-python-<release-version>-<os>-<arch>.cdx.json`.

## Build from source

Requires a Rust toolchain (see the workspace `rust-version`) and Python 3.10+.
The single build command (run in `bindings/python`):

```bash
python3 -m venv .venv && .venv/bin/pip install maturin pytest ruff
.venv/bin/maturin build --release      # wheel in ../../target/wheels/
.venv/bin/pip install ../../target/wheels/sz_configtool-*.whl
.venv/bin/pytest                        # conformance + binding tests
```

Coverage (coverage.py, with the pyo3 seam instrumented; 100% gate):
`packaging/coverage.sh rust python` from the repository root (see
`packaging/README.md`, Coverage).

## Example

```python
import json
from pathlib import Path

import sz_configtool as sct

config = Path("g2config.json").read_text()  # e.g. tests/fixtures/g2config_template.json

config = sct.add_data_source(config, "crm")
print(json.loads(sct.get_data_source(config, "CRM"))["DSRC_CODE"])  # CRM

# Functions returning a config AND a record give a ConfigAndJson named tuple.
config, row = sct.add_attribute(config, "MY_NAME", "NAME", "FULL_NAME", "NAME")
print(json.loads(row)["ATTR_CODE"])  # MY_NAME

# Functions with named results give a <Function>Result named tuple whose
# fields are JSON text.
plan = sct.set_generic_plan(config, "MY_PLAN", "Mine")
print(plan.plan_id, plan.was_created)  # 3 true
config = plan.config

# Call selectors take an id (int) or a feature code (str).
print(
    sct.get_standardize_call(config, 3) == sct.get_standardize_call(config, "ADDRESS")
)

try:
    sct.get_data_source(config, "NOPE")
except sct.SzConfigToolError as err:
    print(err.reason_code)  # NOT_FOUND
```

## API conventions

| Manifest | Python |
|---|---|
| required arg (incl. `required: true`) | positional parameter |
| optional arg | keyword-only, `None` (default) = omit |
| tri-state arg | keyword-only, `UNSET` (default) = leave, `None` = clear, value = set |
| `returns: config` | `str` (modified config) |
| `returns: json` | `str` (JSON text; `json.loads` it) |
| `returns: config_and_json` | `ConfigAndJson(config, json)` (`json` is the record's JSON text) |
| `returns: json` + `tuple_names: [a, b]` | `<Fn>Result(a, b)` named tuple, e.g. `VerifyCompatibilityVersionResult(current_version, matches)` |
| `returns: config_and_json` + `tuple_names: [a, b]` | `<Fn>Result(config, a, b)`, e.g. `SetGenericPlanResult(config, plan_id, was_created)` |
| `returns: int` | `int` |
| `returns: unit` | `None` |
| `int_or_str` arg (call selector) | `int \| str`: a call id or a feature code, sent unchanged |
| `status: not_implemented` | no typed function; reachable via `invoke` |

Each named field of a `<Fn>Result` is exactly that record member's compact JSON
text (`3`, `true`, `"4.0.0"` including quotes): `json.loads` it. `<Fn>` is the
PascalCase function name; the record types are exported from the package.

Argument names are the manifest's snake_case names; a Python keyword gets a
trailing underscore (`class` -> `class_`).

`invoke(name, config_json, args)` is the dynamic seam: it calls any manifest
function by name with an args mapping and returns `Invocation(kind, config,
result)`.

## Errors

Every failure raises `SzConfigToolError` with:

* `reason_code` — one of `REASON_CODES` (`NOT_FOUND`, `ALREADY_EXISTS`,
  `INVALID_INPUT`, `VALIDATION_ERRORS`, ...); branch on this.
* `message` — human-readable text.
* `details` — for `VALIDATION_ERRORS`, JSON text with schema
  `sz-configtool.validation-errors/v1`; otherwise `None`.
* `kind` — the same string as `reason_code` (the reason code IS the kind in
  every binding; Java/C#/C++ expose it as an enum, Python and TS as a string).

A `name` or `config` that is not a `str`, a `str` containing a lone surrogate
(no UTF-8 form), or an argument JSON cannot carry (`set`, `Decimal`, `bytes`,
any other object, `NaN`/`±Infinity`) raises `SzConfigToolError` with
`INVALID_INPUT` (never `TypeError` / `ValueError` / `UnicodeEncodeError`), as
in every other binding.

There are deliberately no subclasses such as `SzNotFoundError` or
`SzBadInputError`: a same-named class that is not the official `senzing` one
would silently miss callers' `except senzing.SzBadInputError`. Branch on
`reason_code` instead.

## Concurrency

The native call releases the GIL, so independent configs can be edited from
several threads in parallel.
