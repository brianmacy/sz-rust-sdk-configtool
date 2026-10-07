"""Runtime support shared by the generated typed functions.

Only this module talks to the native seam ``_native.invoke``. The config
string is OPAQUE: it is passed to and returned from the library unchanged.
"""

from __future__ import annotations

import enum
import json
from collections.abc import Callable, Mapping
from typing import Any, NamedTuple, TypeVar

from . import _native
from ._errors import SzConfigToolError

_R = TypeVar("_R")


class UnsetType(enum.Enum):
    """Type of :data:`UNSET`."""

    UNSET = enum.auto()

    def __repr__(self) -> str:
        return "UNSET"

    def __bool__(self) -> bool:
        return False


__version__: str = _native.LIBRARY_VERSION
"""The library version (e.g. ``"4.4.0-1"``), same as :func:`library_version`."""


def library_version() -> str:
    """The native library version (the workspace version, e.g. ``"4.4.0-1"``)."""
    return _native.LIBRARY_VERSION


def abi_version() -> int:
    """The C ABI version this build implements (``SZCONFIGTOOL_ABI_VERSION``)."""
    return _native.ABI_VERSION


UNSET = UnsetType.UNSET
"""Tri-state "leave unchanged": omit the field (``None`` clears it)."""


class Invocation(NamedTuple):
    """Raw result of :func:`invoke`."""

    kind: str
    config: str | None
    result: str | None


def opt(value: Any) -> Any:
    """Optional arg: ``None`` means absent (the key is omitted)."""
    return UNSET if value is None else value


def _invalid_input(message: str) -> SzConfigToolError:
    return SzConfigToolError("INVALID_INPUT", message)


def _pack(args: Mapping[str, Any]) -> str:
    """``args`` as a JSON object, or ``INVALID_INPUT`` when JSON cannot carry it.

    A value ``json.dumps`` rejects (``set``, ``Decimal``, ``bytes``, any
    object) or would alter (``NaN``/``Infinity`` are not JSON) is the same
    ``INVALID_INPUT`` the other bindings raise, never ``TypeError`` /
    ``ValueError``.
    """
    if not isinstance(args, Mapping):
        raise _invalid_input(f"args must be a mapping, not {type(args).__name__}")
    try:
        return json.dumps(
            {k: v for k, v in args.items() if v is not UNSET}, allow_nan=False
        )
    except (TypeError, ValueError) as e:
        raise _invalid_input(f"args cannot be encoded as JSON: {e}") from None


def invoke(
    name: str, config_json: str, args: Mapping[str, Any] | None = None
) -> Invocation:
    """Call any manifest function by its wire name (the dynamic seam).

    ``args`` is keyed by the manifest's snake_case arg names; a missing key
    means absent, ``None`` (JSON null) clears a tri-state field. This also
    reaches functions without a typed wrapper (``status: not_implemented``).

    Raises:
        SzConfigToolError: On any failure, with its ``reason_code``.
    """
    if not isinstance(name, str):
        raise _invalid_input(f"name must be a str, not {type(name).__name__}")
    packed = _pack({} if args is None else args)
    try:
        kind, config, result = _native.invoke(name, config_json, packed)
    except _native.NativeError as e:
        raise SzConfigToolError(*e.args) from None
    return Invocation(kind, config, result)


def call_config(name: str, config_json: str, args: Mapping[str, Any]) -> str:
    """``returns: config`` / ``config_and_json`` -> the modified config string."""
    return str(invoke(name, config_json, args).config)


def call_json(name: str, config_json: str, args: Mapping[str, Any]) -> str:
    """``returns: json`` (or a ``config_and_json`` record) -> the JSON string."""
    return str(invoke(name, config_json, args).result)


def _member_json(result: str, names: tuple[str, ...]) -> list[str]:
    """Each named record member as compact JSON text (serde_json style)."""
    record = json.loads(result)
    return [
        json.dumps(record[n], separators=(",", ":"), ensure_ascii=False) for n in names
    ]


def call_json_record(
    name: str,
    config_json: str,
    args: Mapping[str, Any],
    record: Callable[..., _R],
    names: tuple[str, ...],
) -> _R:
    """A ``tuple_names`` result -> ``record(*member JSON texts)``.

    For ``returns: config_and_json`` this is the companion's record; the
    config is the primary's return value.
    """
    out = invoke(name, config_json, args)
    return record(*_member_json(str(out.result), names))


def call_int(name: str, config_json: str, args: Mapping[str, Any]) -> int:
    """``returns: int`` -> the integer result."""
    return int(str(invoke(name, config_json, args).result))


def call_unit(name: str, config_json: str, args: Mapping[str, Any]) -> None:
    """``returns: unit`` -> nothing (raises on failure)."""
    invoke(name, config_json, args)
