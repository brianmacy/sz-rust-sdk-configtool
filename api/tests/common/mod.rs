//! Shared helpers for the `api` integration tests. Paths come from the
//! generated manifest (which copies `project.yaml`), never from this code.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// The workspace root (this crate lives in `<root>/api`).
pub fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("api crate has a parent directory")
}

/// The generated manifest (embedded in the crate).
pub fn manifest() -> Value {
    serde_json::from_str(sz_configtool_api::MANIFEST_JSON).expect("manifest.json parses")
}

/// A workspace-relative path named by `paths.<key>` in the manifest.
pub fn project_path(key: &str) -> PathBuf {
    let m = manifest();
    let rel = m["paths"][key]
        .as_str()
        .unwrap_or_else(|| panic!("manifest paths.{key} missing"));
    workspace_root().join(rel)
}

pub fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// Strings at `manifest.functions[*].<key>` (skipping nulls).
pub fn function_strings(key: &str) -> Vec<String> {
    manifest()["functions"]
        .as_array()
        .expect("functions array")
        .iter()
        .filter_map(|f| f[key].as_str().map(str::to_string))
        .collect()
}
