"""Every config-changing function returns the new config TEXT (issue #75).

A ``config_and_json`` function's primary returns only the config; its
companion ``<name>_result`` (same arguments) returns the record JSON text, or
the named-fields record for ``tuple_names``.
"""

from __future__ import annotations

import inspect
import json

import pytest
import sz_configtool as sct
from conftest import FUNCTIONS

IMPLEMENTED = [f for f in FUNCTIONS.values() if f["status"] == "implemented"]
CHANGING = [f for f in IMPLEMENTED if f["returns"] in ("config", "config_and_json")]
PAIRED = [f for f in IMPLEMENTED if f["returns"] == "config_and_json"]


def record_name(f: dict) -> str:
    return "".join(w.capitalize() for w in f["name"].split("_")) + "Record"


@pytest.mark.parametrize("f", CHANGING, ids=lambda f: f["name"])
def test_config_changing_primary_returns_str(f: dict) -> None:
    hint = inspect.signature(getattr(sct, f["name"])).return_annotation
    assert hint == "str"


@pytest.mark.parametrize("f", PAIRED, ids=lambda f: f["name"])
def test_config_and_json_has_companion(f: dict) -> None:
    primary = getattr(sct, f["name"])
    companion = getattr(sct, f"{f['name']}_result")
    assert f"{f['name']}_result" in sct.__all__
    want = "str" if not f.get("tuple_names") else record_name(f)
    assert inspect.signature(companion).return_annotation == want
    same = inspect.signature(primary).parameters
    assert inspect.signature(companion).parameters == same
    doc = companion.__doc__ or ""
    assert doc.startswith(f"The record (row / ids) of ``{f['name']}``"), doc


def test_no_config_and_json_type() -> None:
    assert not hasattr(sct, "ConfigAndJson")


def test_chaining_five_plus_config_changing_functions(template: str) -> None:
    cfg = template
    cfg = sct.add_element(cfg, "DEMO_EL", data_type="string")
    cfg = sct.add_feature(cfg, "DEMO_FEAT", ["DEMO_EL"])
    cfg = sct.add_attribute(cfg, "DEMO_ATTR", "DEMO_FEAT", "DEMO_EL", "OTHER")
    frag = {"ERFRAG_CODE": "DEMO_FRAG", "ERFRAG_SOURCE": "./FRAGMENT[./SAME_NAME>0]"}
    cfg = sct.add_fragment(cfg, frag)
    cfg = sct.add_comparison_call(cfg, "DEMO_FEAT", "EXACT_COMP", ["DEMO_EL"])
    cfg = sct.add_comparison_function(cfg, "DEMO_COMP")
    cfg = sct.add_data_source(cfg, "DEMO_DS")
    assert isinstance(cfg, str)
    assert json.loads(sct.get_attribute(cfg, "DEMO_ATTR"))["ATTR_CODE"] == "DEMO_ATTR"
    assert "DEMO_FRAG" in sct.get_fragment(cfg, "DEMO_FRAG")
    assert "DEMO_COMP" in sct.get_comparison_function(cfg, "DEMO_COMP")
    assert "DEMO_DS" in sct.list_data_sources(cfg)


def test_companion_returns_the_row_of_the_same_operation(template: str) -> None:
    args = ("X_ATTR", "NAME", "FULL_NAME", "OTHER")
    row = sct.add_attribute_result(template, *args, internal="yes")
    assert json.loads(row)["ATTR_CODE"] == "X_ATTR"
    assert json.loads(row)["INTERNAL"] == "Yes"
    raw = sct.invoke(
        "add_attribute",
        template,
        {
            "attribute": "X_ATTR",
            "feature": "NAME",
            "element": "FULL_NAME",
            "class": "OTHER",
            "internal": "yes",
        },
    )
    assert row == raw.result
    assert sct.add_attribute(template, *args, internal="yes") == raw.config


def test_set_generic_plan_companion_is_a_named_record(template: str) -> None:
    created = sct.set_generic_plan_result(template, "my_plan", "Mine")
    assert isinstance(created, sct.SetGenericPlanRecord)
    assert created._fields == ("plan_id", "was_created")
    assert (created.plan_id, created.was_created) == ("3", "true")
    cfg = sct.set_generic_plan(template, "my_plan", "Mine")
    updated = sct.set_generic_plan_result(cfg, "MY_PLAN", "Renamed")
    assert (updated.plan_id, updated.was_created) == ("3", "false")


@pytest.mark.parametrize("bad", [{"config": "{}"}, ("{}", "{}"), 7, None])
def test_non_str_config_fails_early_with_a_hint(template: str, bad: object) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.add_data_source(bad, "X")  # type: ignore[arg-type]
    assert e.value.reason_code == "INVALID_INPUT"
    msg = e.value.message
    assert msg.startswith(
        f"config must be a str (the configuration JSON text), not {type(bad).__name__}"
    )
    assert "use the value that call returned" in msg
    assert "<name>_result" in msg
