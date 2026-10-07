"""Conformance expectation matching (api/manifest/schema.md, "Subset match")."""

from __future__ import annotations

from typing import Any


def subset(expected: Any, actual: Any) -> bool:
    """Objects: expected keys present and matching; arrays: same length,
    element-wise; ``None`` needs ``None``; scalars: strict JSON equality."""
    match expected:
        case dict():
            return isinstance(actual, dict) and all(
                k in actual and subset(v, actual[k]) for k, v in expected.items()
            )
        case list():
            return (
                isinstance(actual, list)
                and len(actual) == len(expected)
                and all(subset(e, a) for e, a in zip(expected, actual))
            )
        case None:
            return actual is None
        case bool():
            return isinstance(actual, bool) and actual == expected
        case int() | float():
            return (
                isinstance(actual, (int, float))
                and not isinstance(actual, bool)
                and actual == expected
            )
        case _:
            return type(actual) is type(expected) and actual == expected


def check(expect: dict[str, Any], kind: str, result: Any) -> list[str]:
    """Every failed success check of ``expect`` (empty list = pass)."""
    problems = []
    if "kind" in expect and expect["kind"] != kind:
        problems.append(f"kind {kind!r} != {expect['kind']!r}")
    if "result" in expect and not subset(expect["result"], result):
        problems.append(f"result {result!r} !~ {expect['result']!r}")
    if not any(k in expect for k in ("len", "contains", "excludes")):
        return problems
    if not isinstance(result, list):
        problems.append(f"len/contains/excludes need an array result, got {result!r}")
        return problems
    for want in expect.get("contains", []):
        if not any(subset(want, item) for item in result):
            problems.append(f"no element matches {want!r}")
    for bad in expect.get("excludes", []):
        if any(subset(bad, item) for item in result):
            problems.append(f"an element matches excluded {bad!r}")
    if "len" in expect and len(result) != expect["len"]:
        problems.append(f"len {len(result)} != {expect['len']}")
    return problems
