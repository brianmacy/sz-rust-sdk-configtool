"""The ``_core`` call helpers against the REAL native extension."""

from __future__ import annotations

import pytest
import sz_configtool as sct
from sz_configtool import _core

# No manifest function is ``returns: int`` today, so ``call_int`` is driven with
# a real ``config_and_json`` function whose result is integer JSON (rows changed).
INT_FN = "remove_config_section_field"


@pytest.mark.parametrize(
    ("field", "removed"), [("gplan_desc", 2), ("NO_SUCH_FIELD", 0)]
)
def test_call_int_parses_a_real_integer_result(
    template: str, field: str, removed: int
) -> None:
    args = {"section_name": "CFG_GPLAN", "field_name": field}
    wire = sct.invoke(INT_FN, template, args)
    assert (wire.kind, wire.result) == ("config_and_json", str(removed))
    n = _core.call_int(INT_FN, template, args)
    assert type(n) is int
    assert n == removed


def test_call_int_propagates_the_native_reason_code(template: str) -> None:
    with pytest.raises(sct.SzConfigToolError) as e:
        _core.call_int(
            INT_FN, template, {"section_name": "CFG_NOPE", "field_name": "X"}
        )
    assert e.value.reason_code == "NOT_FOUND"
