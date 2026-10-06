"""Error mapping: one class, stable reason codes, structured details."""

from __future__ import annotations

import json

import pytest
import sz_configtool as sct
from conftest import CONFORMANCE, FUNCTIONS

# Raised only for result-serialization failures or caught panics, which no
# valid input can trigger.
UNREACHABLE = {"INTERNAL"}


def expected_errors() -> set[str]:
    return {
        s["expect"]["error"]
        for c in CONFORMANCE["cases"]
        for s in c["steps"]
        if "error" in s["expect"]
    }


def test_every_reachable_reason_code_is_exercised() -> None:
    # test_conformance asserts each of these is raised with that reason code.
    assert expected_errors() == set(sct.REASON_CODES) - UNREACHABLE


def test_error_fields_and_str(template: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.get_data_source(template, "NOPE")
    err = e.value
    assert isinstance(err, Exception)
    assert (err.reason_code, err.kind, err.details) == ("NOT_FOUND", "NOT_FOUND", None)
    assert "NOPE" in err.message
    assert str(err) == f"[NOT_FOUND] {err.message}"
    assert err.args == ("NOT_FOUND", err.message, None)
    assert err.__cause__ is None and err.__suppress_context__


def test_validation_errors_carry_versioned_details(template: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.add_search_profile(template, "P2", "SEARCH", candidates="Sometimes")
    err = e.value
    assert err.reason_code == "VALIDATION_ERRORS"
    details = json.loads(err.details)
    assert details["schema"] == "sz-configtool.validation-errors/v1"
    assert details["failures"] == [
        {
            "field": "candidates",
            "reasonCode": "OUT_OF_DOMAIN",
            "offendingValue": "Sometimes",
        }
    ]


@pytest.mark.parametrize(
    ("name", "args", "reason"),
    [
        ("no_such_function", {}, "INVALID_INPUT"),
        ("add_data_source", {}, "MISSING_FIELD"),
        ("add_data_source", {"code": "A", "cdoe": "B"}, "INVALID_INPUT"),
        ("add_data_source", {"code": 1}, "INVALID_INPUT"),
        ("add_data_source", {"code": "A", "id": None}, "INVALID_INPUT"),
    ],
)
def test_wire_errors(template: str, name: str, args: dict, reason: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke(name, template, args)
    assert e.value.reason_code == reason
    assert e.value.kind == reason


def test_typed_wrong_type_reaches_wire_validation(template: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.add_data_source(template, 7)  # type: ignore[arg-type]
    assert e.value.reason_code == "INVALID_INPUT"


def test_bad_config_is_json_parse() -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.add_data_source("not json", "X")
    assert e.value.reason_code == "JSON_PARSE"


def test_manifest_error_lists_are_reason_codes() -> None:
    for f in FUNCTIONS.values():
        assert set(f["errors"]) <= set(sct.REASON_CODES), f["name"]


@pytest.mark.parametrize(
    "config",
    ['{"x":"\ud800"}', "\udfff", 42, None, b"{}"],
    ids=["lone-high-surrogate", "lone-low-surrogate", "int", "none", "bytes"],
)
def test_config_that_cannot_cross_the_boundary_is_invalid_input(config: object) -> None:
    # Same reason code as Java/TS/C# (CONTRACT.md "Input that cannot cross the
    # boundary unchanged"), never UnicodeEncodeError / TypeError.
    with pytest.raises(sct.SzConfigToolError) as typed:
        sct.list_data_sources(config)  # type: ignore[arg-type]
    assert typed.value.reason_code == "INVALID_INPUT"
    assert typed.value.kind == "INVALID_INPUT"
    with pytest.raises(sct.SzConfigToolError) as raw:
        sct.invoke("list_data_sources", config)  # type: ignore[arg-type]
    assert raw.value.reason_code == "INVALID_INPUT"


def test_lone_surrogate_in_args_is_invalid_input(template: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.add_data_source(template, "\ud800")
    assert e.value.reason_code == "INVALID_INPUT"


@pytest.mark.parametrize(
    "arg",
    [{"x"}, __import__("decimal").Decimal("1.5"), object(), b"bytes"],
    ids=["set", "decimal", "object", "bytes"],
)
def test_arg_that_is_not_json_is_invalid_input(template: str, arg: object) -> None:
    # Same reason code as TS (CONTRACT.md), never TypeError from json.dumps.
    with pytest.raises(sct.SzConfigToolError) as typed:
        sct.add_data_source(template, arg)  # type: ignore[arg-type]
    assert typed.value.reason_code == "INVALID_INPUT"
    with pytest.raises(sct.SzConfigToolError) as raw:
        sct.invoke("add_data_source", template, {"code": arg})
    assert raw.value.reason_code == "INVALID_INPUT"


@pytest.mark.parametrize(
    "args", [{1: "x"}, ["code"], "code"], ids=["int-key", "list", "str"]
)
def test_args_that_are_not_a_str_keyed_mapping_are_invalid_input(
    template: str, args: object
) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke("add_data_source", template, args)  # type: ignore[arg-type]
    assert e.value.reason_code == "INVALID_INPUT"


@pytest.mark.parametrize(
    "name",
    [1, None, b"list_data_sources", "list\ud800"],
    ids=["int", "none", "bytes", "surrogate"],
)
def test_name_that_is_not_a_str_is_invalid_input(template: str, name: object) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke(name, template)  # type: ignore[arg-type]
    assert e.value.reason_code == "INVALID_INPUT"
    assert e.value.kind == "INVALID_INPUT"


def test_nul_passes_through_unchanged(template: str) -> None:
    # NUL is a valid str character and crosses unchanged (no C string here):
    # in an arg it reaches the library as \u0000; in name/config the library
    # judges it (unknown function / JSON with a raw control character).
    out = sct.add_data_source(template, "A\0B")
    assert "A\\u0000B" in sct.list_data_sources(out)
    with pytest.raises(sct.SzConfigToolError) as name_err:
        sct.invoke("list_data_sources\0", template)
    assert name_err.value.reason_code == "INVALID_INPUT"
    with pytest.raises(sct.SzConfigToolError) as cfg_err:
        sct.list_data_sources(template + "\0")
    assert cfg_err.value.reason_code == "JSON_PARSE"


def test_nan_arg_is_invalid_input(template: str) -> None:
    # json.dumps would write the non-JSON token NaN by default.
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke("add_data_source", template, {"code": "A", "id": float("nan")})
    assert e.value.reason_code == "INVALID_INPUT"


class _NamelessMeta(type):
    """A metaclass whose ``__name__`` lookup fails (overrides ``type.__name__``)."""

    @property
    def __name__(cls) -> str:  # type: ignore[override]
        raise RuntimeError("no name")


def test_config_whose_type_has_no_readable_name_is_invalid_input() -> None:
    nameless = _NamelessMeta("Nameless", (), {})()
    with pytest.raises(sct.SzConfigToolError) as e:
        sct.invoke("list_data_sources", nameless)  # type: ignore[arg-type]
    assert e.value.reason_code == "INVALID_INPUT"
    assert e.value.message == "config must be a str, not ?"
