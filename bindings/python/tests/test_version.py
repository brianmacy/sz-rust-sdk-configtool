"""Version accessors return the single source (sz-configtool-api) value."""

from __future__ import annotations

import sz_configtool as sz
import tomllib
from generated_paths import WORKSPACE_ROOT


def workspace_version() -> str:
    with (WORKSPACE_ROOT / "Cargo.toml").open("rb") as f:
        return tomllib.load(f)["workspace"]["package"]["version"]


def test_version_equals_workspace_version() -> None:
    assert sz.__version__ == workspace_version()
    assert sz.library_version() == workspace_version()


def test_abi_version_is_the_c_abi_version() -> None:
    assert sz.abi_version() == 2
    assert isinstance(sz.abi_version(), int)


def test_accessors_are_exported() -> None:
    for name in ("__version__", "library_version", "abi_version"):
        assert name in sz.__all__, name
