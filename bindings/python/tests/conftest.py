"""Shared fixtures: the generated manifest/conformance JSON and the template.

Paths come from ``generated_paths`` (rendered from project.yaml by the codegen).
"""

from __future__ import annotations

import json
from typing import Any

import generated_paths
import pytest


def load_json(path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


MANIFEST: dict[str, Any] = load_json(generated_paths.MANIFEST_JSON)
CONFORMANCE: dict[str, Any] = load_json(generated_paths.CONFORMANCE_JSON)
FUNCTIONS: dict[str, dict[str, Any]] = {f["name"]: f for f in MANIFEST["functions"]}


def read_fixture(rel: str) -> str:
    """Workspace-relative file, decoded WITHOUT newline translation."""
    return (generated_paths.WORKSPACE_ROOT / rel).read_bytes().decode("utf-8")


@pytest.fixture(scope="session")
def template() -> str:
    """The REAL configuration every conformance case starts from."""
    return read_fixture(CONFORMANCE["fixture"])


@pytest.fixture(scope="session")
def manifest() -> dict[str, Any]:
    return MANIFEST
