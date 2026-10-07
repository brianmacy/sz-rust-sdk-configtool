"""Run api/manifest/generated/conformance.json against the REAL extension.

Typed steps call the generated function AND the raw ``invoke`` seam and
require byte-identical configs and results (the typed layer is a pure,
opaque pass-through). ``wire_only`` steps and functions without a typed
wrapper (``status: not_implemented``) run through ``invoke`` only.
"""

from __future__ import annotations

import json
from typing import Any

import pytest
import sz_configtool as sct
from conftest import CONFORMANCE, FUNCTIONS
from matching import check
from naming import py_name

CASES = CONFORMANCE["cases"]


def record_json(value: tuple, names: list[str]) -> str:
    """Rebuild the wire record from a named result's JSON-text fields."""
    fields = value[-len(names) :]
    obj = {n: json.loads(v) for n, v in zip(names, fields, strict=True)}
    return json.dumps(obj, separators=(",", ":"), ensure_ascii=False)


def call_typed(fn: str, config: str, args: dict[str, Any]) -> sct.Invocation:
    """Call the typed wrapper(s); normalize the value(s) to an ``Invocation``.

    A ``config_and_json`` function is the primary (the new config text) plus
    its ``<fn>_result`` companion (the record), both called with the step's
    arguments on the same input config.
    """
    kwargs = {py_name(k): v for k, v in args.items()}
    value = getattr(sct, fn)(config, **kwargs)
    names = FUNCTIONS[fn].get("tuple_names") or []
    match (FUNCTIONS[fn]["returns"], bool(names)):
        case ("config", _):
            assert isinstance(value, str)
            return sct.Invocation("config", value, None)
        case ("json", False):
            assert isinstance(value, str)
            return sct.Invocation("json", None, value)
        case ("json", True):
            assert value._fields == tuple(names)
            return sct.Invocation("json", None, record_json(value, names))
        case ("config_and_json", False):
            assert isinstance(value, str)
            record = getattr(sct, f"{fn}_result")(config, **kwargs)
            assert isinstance(record, str)
            return sct.Invocation("config_and_json", value, record)
        case ("config_and_json", True):
            assert isinstance(value, str)
            record = getattr(sct, f"{fn}_result")(config, **kwargs)
            assert record._fields == tuple(names)
            return sct.Invocation("config_and_json", value, record_json(record, names))
        case ("int", _):
            assert isinstance(value, int)
            return sct.Invocation("int", None, json.dumps(value))
        case ("unit", _):
            assert value is None
            return sct.Invocation("unit", None, None)
    raise AssertionError(f"unknown returns for {fn}")


def run(step: dict[str, Any], config: str) -> sct.Invocation:
    fn, args = step["fn"], step["args"]
    raw = sct.invoke(fn, config, args)
    if step.get("wire_only") or not hasattr(sct, fn):
        return raw
    typed = call_typed(fn, config, args)
    assert typed.kind == raw.kind
    assert typed.config == raw.config, "typed config differs from invoke"
    assert typed.result == raw.result, "typed result differs from invoke"
    return typed


def expect_error(step: dict[str, Any], config: str) -> None:
    want = step["expect"]["error"]
    with pytest.raises(sct.SzConfigToolError) as raw:
        sct.invoke(step["fn"], config, step["args"])
    assert raw.value.reason_code == want, raw.value
    if step.get("wire_only") or not hasattr(sct, step["fn"]):
        return
    typed_args = {py_name(k): v for k, v in step["args"].items()}
    with pytest.raises(sct.SzConfigToolError) as typed:
        getattr(sct, step["fn"])(config, **typed_args)
    assert typed.value.reason_code == want, typed.value
    if FUNCTIONS[step["fn"]]["returns"] == "config_and_json":
        with pytest.raises(sct.SzConfigToolError) as companion:
            getattr(sct, f"{step['fn']}_result")(config, **typed_args)
        assert companion.value.reason_code == want, companion.value


@pytest.mark.parametrize(
    "case", CASES, ids=[f"{c['group']}::{c['name']}" for c in CASES]
)
def test_conformance(case: dict[str, Any], template: str) -> None:
    config = template
    for i, step in enumerate(case["steps"]):
        current = step.get("config_literal", config)
        if "error" in step["expect"]:
            expect_error(step, current)
            continue
        out = run(step, current)
        result = None if out.result is None else json.loads(out.result)
        problems = check(step["expect"], out.kind, result)
        assert not problems, f"step {i} ({step['fn']}): {problems}"
        if out.config is not None:
            config = out.config


def test_every_case_has_steps():
    assert len(CASES) > 0
    assert all(c["steps"] for c in CASES)
