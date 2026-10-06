"""The config string is OPAQUE: never parsed or re-serialized by Python."""

from __future__ import annotations

import json
import threading

import sz_configtool as sct


def test_template_round_trip_is_byte_exact(template: str) -> None:
    added = sct.add_data_source(template, "crm")
    removed = sct.delete_data_source(added, "CRM")
    # The library emits compact JSON in document order; the Python layer
    # returns exactly those bytes (same as the raw seam).
    compact = json.dumps(
        json.loads(template), separators=(",", ":"), ensure_ascii=False
    )
    assert removed == compact
    assert sct.invoke("delete_data_source", added, {"code": "CRM"}).config == removed
    assert sct.add_data_source(removed, "crm") == added


def test_non_ascii_and_escapes_pass_through(template: str) -> None:
    out = sct.add_attribute(
        template, "UNI", "NAME", "FULL_NAME", "OTHER", default_value='café ☃ "q" \\'
    )
    assert '"DEFAULT_VALUE":"café ☃ \\"q\\" \\\\"' in out.config
    assert json.loads(out.json)["DEFAULT_VALUE"] == 'café ☃ "q" \\'


def test_input_whitespace_is_not_normalized_by_python() -> None:
    spaced = '{ "G2_CONFIG" : { "CFG_DSRC" : [ ] } }'
    out = sct.invoke("list_data_sources", spaced, {})
    assert out.config is None and json.loads(out.result) == []


def test_concurrent_calls_are_independent(template: str) -> None:
    results: dict[int, str] = {}

    def work(i: int) -> None:
        cfg = sct.add_data_source(template, f"ds{i}")
        results[i] = json.loads(sct.get_data_source(cfg, f"DS{i}"))["DSRC_CODE"]

    threads = [threading.Thread(target=work, args=(i,)) for i in range(16)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    assert results == {i: f"DS{i}" for i in range(16)}
