"""The extension must not leak the C ABI (``SzConfigTool_*``) symbols."""

from __future__ import annotations

import shutil
import subprocess
import sys

import pytest
from sz_configtool import _native


def exported_symbols(path: str) -> str:
    match sys.platform:
        case "darwin":
            cmd = ["nm", "-gU", path]
        case "win32":
            pytest.skip("use dumpbin /exports on Windows")
        case _:
            cmd = ["nm", "-D", "--defined-only", path]
    if shutil.which(cmd[0]) is None:
        pytest.skip("nm not available")
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout


def test_no_c_abi_exports() -> None:
    symbols = exported_symbols(_native.__file__)
    assert "PyInit__native" in symbols  # nm really listed the exports
    assert "SzConfigTool_" not in symbols
