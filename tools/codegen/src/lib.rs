//! Code generator for the binding manifest (`api/manifest`).
//!
//! Reads `project.yaml`, the per-group function files, `excluded.yaml` and the
//! conformance files; validates them; and renders the checked-in outputs named
//! by `project.yaml` (the Rust dispatcher plus JSON copies of the manifest and
//! conformance cases that other languages and the `api` tests consume without
//! a YAML parser).

pub mod emit;
pub mod lang;
pub mod load;
pub mod model;
pub mod validate;

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

/// Default location of `project.yaml`, relative to the workspace root.
pub const DEFAULT_PROJECT_FILE: &str = "api/manifest/project.yaml";

/// A generated file: workspace-relative path and full contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generated {
    pub path: PathBuf,
    pub contents: String,
}

/// Load, validate and render every output (nothing is written).
pub fn generate(workspace_root: &Path, project_file: &Path) -> Result<Vec<Generated>> {
    let mut inputs = load::load(workspace_root, project_file)?;
    if let Err(errs) = validate::validate(&inputs) {
        bail!("manifest validation failed:\n  {}", errs.join("\n  "));
    }
    validate::mark_wire_only(&mut inputs);
    let p = &inputs.project.paths;
    let mut outputs = vec![
        Generated {
            path: PathBuf::from(&p.dispatch_out),
            contents: emit::dispatch_rs(&inputs),
        },
        Generated {
            path: PathBuf::from(&p.manifest_json_out),
            contents: emit::manifest_json(&inputs),
        },
        Generated {
            path: PathBuf::from(&p.conformance_json_out),
            contents: emit::conformance_json(&inputs),
        },
    ];
    outputs.extend(lang::generate_all(&inputs));
    Ok(outputs)
}

/// Outputs whose checked-in contents differ from `outputs`.
pub fn stale(workspace_root: &Path, outputs: &[Generated]) -> Vec<PathBuf> {
    outputs
        .iter()
        .filter(|g| {
            std::fs::read_to_string(workspace_root.join(&g.path)).ok() != Some(g.contents.clone())
        })
        .map(|g| g.path.clone())
        .collect()
}

/// Write every output under `workspace_root`.
pub fn write(workspace_root: &Path, outputs: &[Generated]) -> Result<()> {
    for g in outputs {
        let path = workspace_root.join(&g.path);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&path, &g.contents)?;
    }
    Ok(())
}
