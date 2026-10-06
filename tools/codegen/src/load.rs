//! Reads the YAML inputs named by `project.yaml` and normalizes them.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;

use crate::model::{
    Case, ConformanceFile, Excluded, ExcludedEntry, Function, GroupFile, Project, Step,
};

/// Every input, loaded and normalized (groups and cases in file-name order).
#[derive(Debug, Clone)]
pub struct Inputs {
    pub project: Project,
    pub functions: Vec<Function>,
    pub excluded: Excluded,
    pub cases: Vec<Case>,
}

fn read_yaml<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_yaml_ng::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

/// `*.yaml` files directly in `dir`, sorted by name, excluding `skip` stems.
fn yaml_files(dir: &Path, skip: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // One fallible step for the listing and every entry (an I/O error on
    // either is the same "listing" failure).
    let entries = std::fs::read_dir(dir)
        .and_then(|rd| rd.collect::<std::io::Result<Vec<_>>>())
        .with_context(|| format!("listing {}", dir.display()))?;
    for entry in entries {
        let path = entry.path();
        let is_yaml = path.extension().is_some_and(|e| e == "yaml");
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if is_yaml && !skip.contains(&stem) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}

fn load_groups(dir: &Path, skip: &[&str]) -> Result<(Vec<Function>, Vec<ExcludedEntry>)> {
    let mut functions = Vec::new();
    let mut excluded = Vec::new();
    for path in yaml_files(dir, skip)? {
        let file: GroupFile = read_yaml(&path)?;
        if file.group != stem(&path) {
            bail!(
                "{}: group '{}' must equal the file name",
                path.display(),
                file.group
            );
        }
        for mut f in file.functions {
            if !f.group.is_empty() && f.group != file.group {
                bail!(
                    "{}: function '{}' declares group '{}'",
                    path.display(),
                    f.name,
                    f.group
                );
            }
            f.group = file.group.clone();
            functions.push(f);
        }
        excluded.extend(file.excluded);
    }
    Ok((functions, excluded))
}

fn normalize_case(group: &str, input: crate::model::CaseInput) -> Result<Case> {
    let inline = input.func.is_some()
        || input.args.is_some()
        || input.expect.is_some()
        || input.config_literal.is_some();
    let steps = match (input.steps, inline) {
        (Some(steps), false) => steps,
        (None, true) => vec![Step {
            func: input
                .func
                .with_context(|| format!("case '{}': inline form needs `fn`", input.name))?,
            args: input.args.unwrap_or_default(),
            config_literal: input.config_literal,
            expect: input.expect.unwrap_or_default(),
            wire_only: false,
        }],
        _ => bail!(
            "case '{}': use EITHER inline fn/args/expect OR steps",
            input.name
        ),
    };
    Ok(Case {
        group: group.to_string(),
        name: input.name,
        doc: input.doc,
        steps,
    })
}

fn load_cases(dir: &Path) -> Result<Vec<Case>> {
    let mut cases = Vec::new();
    for path in yaml_files(dir, &[])? {
        let file: ConformanceFile = read_yaml(&path)?;
        if file.group != stem(&path) {
            bail!(
                "{}: group '{}' must equal the file name",
                path.display(),
                file.group
            );
        }
        for input in file.cases {
            cases.push(normalize_case(&file.group, input)?);
        }
    }
    Ok(cases)
}

/// Load every input relative to `workspace_root`, starting from `project_file`.
pub fn load(workspace_root: &Path, project_file: &Path) -> Result<Inputs> {
    let project: Project = read_yaml(&workspace_root.join(project_file))?;
    let p = &project.paths;
    let excluded_path = workspace_root.join(&p.excluded);
    let project_stem = stem(project_file);
    let excluded_stem = stem(&excluded_path);
    let (functions, group_excluded) = load_groups(
        &workspace_root.join(&p.manifest_dir),
        &[&project_stem, &excluded_stem],
    )?;
    let mut excluded: Excluded = read_yaml(&excluded_path)?;
    excluded.entries.extend(group_excluded);
    let cases = load_cases(&workspace_root.join(&p.conformance_dir))?;
    Ok(Inputs {
        project,
        functions,
        excluded,
        cases,
    })
}
