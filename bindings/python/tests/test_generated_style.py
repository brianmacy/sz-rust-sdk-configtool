"""Generated Python must be ruff-format/ruff-check stable."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

import sz_configtool

PACKAGE = Path(sz_configtool.__file__).parent
SOURCES = [str(p) for p in sorted(PACKAGE.glob("*.py*")) if p.suffix in {".py", ".pyi"}]


def ruff(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, "-m", "ruff", *args],
        capture_output=True,
        text=True,
        check=False,
    )


def test_ruff_format_is_stable() -> None:
    out = ruff("format", "--check", *SOURCES)
    assert out.returncode == 0, out.stdout + out.stderr


def test_ruff_check_is_clean() -> None:
    out = ruff("check", *SOURCES)
    assert out.returncode == 0, out.stdout + out.stderr


def test_py_typed_marker_ships() -> None:
    assert (PACKAGE / "py.typed").is_file()
    assert (PACKAGE / "generated.pyi").is_file()
