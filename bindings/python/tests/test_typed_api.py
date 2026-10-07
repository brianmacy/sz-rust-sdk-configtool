"""Naming, signatures and UNSET / None / value semantics of the typed API."""

from __future__ import annotations

import inspect
import json
import keyword

import pytest
import sz_configtool as sct
from conftest import FUNCTIONS
from naming import py_name

IMPLEMENTED = [f for f in FUNCTIONS.values() if f["status"] == "implemented"]
TUPLE_NAMED = [f for f in IMPLEMENTED if f.get("tuple_names")]
P = inspect.Parameter


def expected_params(f: dict) -> list[tuple[str, object, object]]:
    """(name, kind, default) per parameter, per bindings/CONTRACT.md."""
    out = [("config_json", P.POSITIONAL_OR_KEYWORD, P.empty)]
    keyword_only = []
    for a in f["args"]:
        match (a["tristate"], a["optional"] and not a["required"]):
            case (True, _):
                keyword_only.append((py_name(a["name"]), P.KEYWORD_ONLY, sct.UNSET))
            case (False, True):
                keyword_only.append((py_name(a["name"]), P.KEYWORD_ONLY, None))
            case _:
                out.append((py_name(a["name"]), P.POSITIONAL_OR_KEYWORD, P.empty))
    return out + keyword_only


@pytest.mark.parametrize("f", IMPLEMENTED, ids=lambda f: f["name"])
def test_signature_matches_manifest(f: dict) -> None:
    fn = getattr(sct, f["name"])
    params = inspect.signature(fn).parameters.values()
    assert [(p.name, p.kind, p.default) for p in params] == expected_params(f)
    assert f["name"] in sct.__all__
    assert fn.__doc__ and fn.__doc__.startswith(f["doc"].split()[0])


def test_names_are_snake_case_and_keyword_safe() -> None:
    for f in IMPLEMENTED:
        assert f["name"].islower() and f["name"].isidentifier()
        for p in inspect.signature(getattr(sct, f["name"])).parameters:
            assert p.isidentifier() and not keyword.iskeyword(p)
    params = inspect.signature(sct.add_attribute).parameters
    assert "class_" in params and "class" not in params


def test_required_true_optional_is_a_required_parameter(template: str) -> None:
    params = inspect.signature(sct.add_generic_threshold).parameters
    assert params["plan"].default is P.empty
    with pytest.raises(TypeError):
        sct.add_generic_threshold(template, behavior="F1")  # type: ignore[call-arg]
    # The wire seam still reports the library's MISSING_FIELD when absent.
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke("add_generic_threshold", template, {"behavior": "F1"})
    assert e.value.reason_code == "MISSING_FIELD"


def test_optional_none_means_omitted(template: str) -> None:
    args = ("MY_ATTR", "NAME", "FULL_NAME", "OTHER")
    omitted = sct.add_attribute_result(template, *args)
    explicit_none = sct.add_attribute_result(template, *args, internal=None, id=None)
    assert omitted == explicit_none
    assert sct.add_attribute(template, *args) == sct.add_attribute(
        template, *args, internal=None, id=None
    )
    assert json.loads(omitted)["INTERNAL"] == "No"
    valued = sct.add_attribute_result(template, *args, internal="yes", id=5000)
    assert json.loads(valued)["INTERNAL"] == "Yes"
    assert json.loads(valued)["ATTR_ID"] == 5000


def test_tristate_unset_none_value(template: str) -> None:
    def connect(config: str) -> object:
        row = json.loads(sct.get_comparison_function(config, "STR_COMP"))
        return row["CONNECT_STR"]

    original = connect(template)
    assert original is not None
    left = sct.set_comparison_function(template, "str_comp", language="C")
    assert connect(left) == original
    unset = sct.set_comparison_function(template, "str_comp", connect_str=sct.UNSET)
    assert connect(unset) == original
    valued = sct.set_comparison_function(template, "str_comp", connect_str="g2New")
    assert connect(valued) == "g2New"
    cleared = sct.set_comparison_function(valued, "str_comp", connect_str=None)
    assert connect(cleared) is None


def test_unset_sentinel() -> None:
    assert repr(sct.UNSET) == "UNSET"
    assert not sct.UNSET
    assert sct.UNSET is sct.UnsetType.UNSET
    assert isinstance(sct.UNSET, sct.UnsetType)


def record_class(f: dict) -> type:
    return getattr(
        sct, "".join(w.capitalize() for w in f["name"].split("_")) + "Record"
    )


@pytest.mark.parametrize("f", TUPLE_NAMED, ids=lambda f: f["name"])
def test_tuple_names_give_a_named_record(f: dict) -> None:
    cls = record_class(f)
    assert cls._fields == tuple(f["tuple_names"])
    assert cls.__name__ in sct.__all__
    typed = f["name"] + ("_result" if f["returns"] == "config_and_json" else "")
    hint = inspect.signature(getattr(sct, typed)).return_annotation
    assert hint == cls.__name__


def test_set_generic_plan_record_fields_are_json_text(template: str) -> None:
    created = sct.set_generic_plan_result(template, "my_plan", "Mine")
    assert isinstance(created, sct.SetGenericPlanRecord)
    assert (created.plan_id, created.was_created) == ("3", "true")
    raw = sct.invoke(
        "set_generic_plan", template, {"gplan_code": "my_plan", "gplan_desc": "Mine"}
    )
    config = sct.set_generic_plan(template, "my_plan", "Mine")
    assert config == raw.config
    updated = sct.set_generic_plan_result(config, "MY_PLAN", "Renamed")
    assert (updated.plan_id, updated.was_created) == ("3", "false")


def test_verify_compatibility_version_record_fields(template: str) -> None:
    current = json.loads(sct.get_compatibility_version(template))
    hit = sct.verify_compatibility_version(template, current)
    assert isinstance(hit, sct.VerifyCompatibilityVersionRecord)
    assert hit == (json.dumps(current), "true")
    miss = sct.verify_compatibility_version(template, "no-such-version")
    assert (miss.current_version, miss.matches) == (json.dumps(current), "false")


def test_int_or_str_selector_accepts_id_and_feature_code(template: str) -> None:
    params = inspect.signature(sct.get_standardize_call).parameters
    assert params["call"].annotation == "int | str"
    by_id = json.loads(sct.get_standardize_call(template, 3))
    by_code = json.loads(sct.get_standardize_call(template, "address"))
    assert by_id == by_code
    assert by_id["SFCALL_ID"] == 3
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.get_standardize_call(template, "NAME")
    assert e.value.reason_code == "INVALID_INPUT"


def test_int_or_str_args_are_annotated_as_a_union() -> None:
    selectors = [
        (f["name"], a["name"])
        for f in IMPLEMENTED
        for a in f["args"]
        if a["type"] == "int_or_str"
    ]
    assert selectors
    for fn, arg in selectors:
        hint = inspect.signature(getattr(sct, fn)).parameters[py_name(arg)].annotation
        assert hint.startswith("int | str"), (fn, hint)


def test_no_official_sdk_error_names() -> None:
    for name in ("SzError", "SzNotFoundError", "SzBadInputError"):
        assert not hasattr(sct, name), name
    exported = [getattr(sct, n) for n in sct.__all__]
    errors = [x for x in exported if isinstance(x, type) and issubclass(x, Exception)]
    assert errors == [sct.SzConfigToolError]


def test_reason_codes_exported(manifest: dict) -> None:
    assert list(sct.REASON_CODES) == manifest["reason_codes"]
