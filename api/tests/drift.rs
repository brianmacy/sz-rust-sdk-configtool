//! Drift tests between the manifest, the root library and the C header.
//!
//! (1) Coverage: every root-lib `pub fn` (in a public module) is mapped by a
//!     manifest group or listed in excluded.yaml. Stale names, duplicates and
//!     mapped+excluded overlaps always fail; unmapped functions are REPORTED
//!     until excluded.yaml sets `enforce_complete: true`, then they fail.
//! (2) Every manifest `c_symbol` / `c_aliases` entry is declared in the header.
//! (3) `rust` paths exist: compile-checked by the generated dispatcher, and
//!     additionally required here to be the DEFINING module path.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::Value;

/// `pub mod <name>;` declarations in a module file.
fn pub_mods(src: &str) -> BTreeSet<String> {
    src.lines()
        .filter_map(|l| l.trim().strip_prefix("pub mod "))
        .filter_map(|r| r.strip_suffix(';'))
        .map(str::to_string)
        .collect()
}

/// Column-0 `pub fn` names (module-level, not methods or test helpers).
fn pub_fns(src: &str) -> Vec<String> {
    src.lines()
        .filter_map(|l| l.strip_prefix("pub fn "))
        .map(|r| r.split(['(', '<']).next().unwrap_or("").trim().to_string())
        .collect()
}

/// Module path segments for a file under the root `src` dir, e.g.
/// `calls/standardize.rs` -> ["calls", "standardize"], `calls/mod.rs` -> ["calls"].
fn module_segments(rel: &Path) -> Vec<String> {
    let mut segs: Vec<String> = rel
        .iter()
        .map(|s| s.to_string_lossy().trim_end_matches(".rs").to_string())
        .collect();
    if matches!(segs.last().map(String::as_str), Some("mod" | "lib")) {
        segs.pop();
    }
    segs
}

/// Is every segment declared `pub mod` in its parent module file?
fn is_public(src_dir: &Path, segs: &[String]) -> bool {
    (0..segs.len()).all(|i| {
        let parent = if i == 0 {
            src_dir.join("lib.rs")
        } else {
            src_dir.join(segs[..i].join("/")).join("mod.rs")
        };
        std::fs::read_to_string(parent).is_ok_and(|s| pub_mods(&s).contains(&segs[i]))
    })
}

fn rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read root src dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Root-lib public functions: module path -> fn paths (`module::fn`).
fn root_functions() -> BTreeMap<String, BTreeSet<String>> {
    let src_dir = common::project_path("root_src");
    let mut files = Vec::new();
    rs_files(&src_dir, &mut files);
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for file in files {
        let segs = module_segments(file.strip_prefix(&src_dir).expect("under src"));
        if segs.is_empty() || !is_public(&src_dir, &segs) {
            continue;
        }
        let module = segs.join("::");
        let fns = out.entry(module.clone()).or_default();
        for f in pub_fns(&common::read(&file)) {
            fns.insert(format!("{module}::{f}"));
        }
    }
    out
}

fn excluded_entries(key: &str) -> Vec<String> {
    common::manifest()["excluded"]["entries"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|e| e[key].as_str().map(str::to_string))
        .collect()
}

#[test]
fn test_scanner_sees_root_library() {
    let root = root_functions();
    assert!(
        root.contains_key("calls::standardize"),
        "nested modules scanned"
    );
    assert!(
        !root.contains_key("config_rows"),
        "pub(crate) modules skipped"
    );
    assert!(root["datasources"].contains("datasources::add_data_source"));
    assert!(
        !root["datasources"].contains("datasources::with_id"),
        "methods skipped"
    );
}

#[test]
fn test_manifest_covers_root_library() {
    let root = root_functions();
    let all: BTreeSet<&String> = root.values().flatten().collect();
    let mapped: BTreeSet<String> = common::function_strings("rust").into_iter().collect();
    let ex_fns = excluded_entries("rust");
    let ex_mods = excluded_entries("module");

    let mut problems = Vec::new();
    for path in &mapped {
        if !all.contains(path) {
            problems.push(format!(
                "manifest rust '{path}' is not a root pub fn at its defining path"
            ));
        }
    }
    let mut ex_seen = BTreeSet::new();
    for path in &ex_fns {
        if !all.contains(path) {
            problems.push(format!("excluded rust '{path}' is not a root pub fn"));
        }
        if !ex_seen.insert(path.clone()) {
            problems.push(format!("excluded rust '{path}' listed twice"));
        }
    }
    for m in &ex_mods {
        match root.get(m) {
            None => problems.push(format!("excluded module '{m}' does not exist")),
            Some(fns) => ex_seen.extend(fns.iter().cloned()),
        }
    }
    for path in mapped.intersection(&ex_seen) {
        problems.push(format!("'{path}' is both mapped and excluded"));
    }
    assert!(
        problems.is_empty(),
        "manifest drift:\n{}",
        problems.join("\n")
    );

    let unmapped: Vec<&&String> = all
        .iter()
        .filter(|p| !mapped.contains(**p) && !ex_seen.contains(**p))
        .collect();
    let enforce = common::manifest()["excluded"]["enforce_complete"]
        .as_bool()
        .expect("excluded.enforce_complete");
    println!(
        "manifest coverage: {} mapped, {} excluded, {} unmapped of {}",
        mapped.len(),
        ex_seen.len(),
        unmapped.len(),
        all.len()
    );
    for p in &unmapped {
        println!("  unmapped: {p}");
    }
    assert!(
        !enforce || unmapped.is_empty(),
        "enforce_complete is true but {} root pub fns are unmapped: {unmapped:?}",
        unmapped.len()
    );
}

/// `SzConfigTool_*` names declared (followed by `(`) in the header, comments stripped.
fn header_symbols(header: &str) -> BTreeSet<String> {
    let mut code = String::new();
    let mut rest = header;
    while let Some(i) = rest.find("/*") {
        code.push_str(&rest[..i]);
        rest = rest[i..].split_once("*/").map_or("", |(_, r)| r);
    }
    code.push_str(rest);
    code.lines()
        .map(|l| l.split_once("//").map_or(l, |(c, _)| c))
        .flat_map(|l| l.match_indices("SzConfigTool_").map(move |(i, _)| &l[i..]))
        .filter_map(|s| {
            let end = s.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
            s[end..]
                .trim_start()
                .starts_with('(')
                .then(|| s[..end].to_string())
        })
        .collect()
}

#[test]
fn test_manifest_c_symbols_are_declared_in_header() {
    let declared = header_symbols(&common::read(&common::project_path("c_header")));
    assert!(
        declared.contains("SzConfigTool_invoke"),
        "header parse sanity"
    );
    let manifest = common::manifest();
    let missing: Vec<String> = manifest["functions"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|f| {
            let aliases = f["c_aliases"].as_array().cloned().unwrap_or_default();
            std::iter::once(f["c_symbol"].clone()).chain(aliases)
        })
        .filter_map(|v: Value| v.as_str().map(str::to_string))
        .filter(|s| !declared.contains(s))
        .collect();
    assert!(
        missing.is_empty(),
        "c_symbols not declared in the header: {missing:?}"
    );
}

#[test]
fn test_reason_codes_match_project() {
    let project: Vec<String> = common::manifest()["reason_codes"]
        .as_array()
        .expect("reason_codes")
        .iter()
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect();
    assert_eq!(project, sz_configtool_api::REASON_CODES.to_vec());
}

#[test]
fn test_dispatcher_matches_manifest_functions() {
    assert_eq!(
        common::function_strings("name"),
        sz_configtool_api::FUNCTION_NAMES.to_vec()
    );
}
