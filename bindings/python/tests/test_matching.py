"""The subset matcher itself must be strict (it guards every conformance case)."""

from matching import check, subset


def test_subset_objects_and_arrays():
    assert subset({"a": 1}, {"a": 1, "b": 2})
    assert not subset({"a": 1}, {"b": 1})
    assert not subset({"a": 1}, [1])
    assert subset([{"a": 1}], [{"a": 1, "c": 3}])
    assert not subset([1], [1, 2])
    assert not subset([1], {"0": 1})


def test_subset_scalars_are_strict():
    assert subset(None, None)
    assert not subset(None, 0)
    assert not subset(1, True)
    assert not subset(True, 1)
    assert subset(True, True)
    assert not subset("1", 1)
    assert subset(1.5, 1.5)
    assert not subset(1, "1")


def test_check_reports_each_failure():
    assert check({}, "json", None) == []
    assert check({"kind": "config"}, "json", None)
    assert check({"contains": [{"a": 1}]}, "json", [{"a": 1}]) == []
    assert check({"contains": [{"a": 2}]}, "json", [{"a": 1}])
    assert check({"excludes": [{"a": 1}]}, "json", [{"a": 1}])
    assert check({"len": 2}, "json", [1]) == [("len 1 != 2")]


def test_array_checks_fail_on_non_array_results():
    """An object/int result must never satisfy len/contains/excludes vacuously."""
    for expect in ({"len": 0}, {"excludes": [{"a": 1}]}, {"contains": ["a"]}):
        assert check(expect, "json", {"a": 1}), expect
        assert check(expect, "int", 3), expect
    assert check({"len": 1, "excludes": [{"a": 2}]}, "json", [{"a": 1}]) == []
