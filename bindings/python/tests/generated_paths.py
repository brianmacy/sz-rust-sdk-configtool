# GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.
"""Workspace paths (from project.yaml) used by the tests."""

from pathlib import Path

WORKSPACE_ROOT = Path(__file__).resolve().parents[3]
MANIFEST_JSON = WORKSPACE_ROOT / "api/manifest/generated/manifest.json"
CONFORMANCE_JSON = WORKSPACE_ROOT / "api/manifest/generated/conformance.json"
FIXTURE = WORKSPACE_ROOT / "tests/fixtures/g2config_template.json"
