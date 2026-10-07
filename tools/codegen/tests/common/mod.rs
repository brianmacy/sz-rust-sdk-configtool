//! Shared scratch-manifest helpers for the codegen integration tests. Each
//! test writes a small, real manifest tree into a scratch workspace under
//! `CARGO_TARGET_TMPDIR` and runs the real generator (or binary) on it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::thread::ThreadId;

use sz_configtool_codegen::generate;

/// `project.yaml` with every path prefixed by `@` (see [`project`]).
const PROJECT_TEMPLATE: &str = "\
root_crate: lib
paths:
  root_src: @src
  c_header: @h.h
  fixture: @f.json
  manifest_dir: @m
  conformance_dir: @m/conformance
  excluded: @m/excluded.yaml
  dispatch_out: @out/dispatch.rs
  manifest_json_out: @out/manifest.json
  conformance_json_out: @out/conformance.json
reason_codes: [NOT_FOUND, INVALID_INPUT, MISSING_FIELD, NOT_IMPLEMENTED, INTERNAL]
bindings:
  python: {module: @out/py/generated.py, stub: @out/py/generated.pyi, init: @out/py/__init__.py, test_paths: @out/py/tests/paths.py}
  java: {api: @out/java/Api.java, error_kinds: @out/java/Kinds.java, test_dispatch: @out/java/Dispatch.java}
  csharp: {api: @out/cs/Api.g.cs, error_kinds: @out/cs/Kinds.g.cs}
  cpp: {api: @out/cpp/api.hpp, error_kinds: @out/cpp/kinds.hpp, test_dispatch: @out/cpp/dispatch.hpp}
  node: {package_dir: @out/node, functions: @out/node/f.ts, reason_codes: @out/node/r.ts, trpc_schemas: @out/node/s.ts, trpc_router: @out/node/rt.ts, test_paths: @out/node/p.ts}
";

pub const GROUP: &str = "\
group: things
functions:
  - name: get_thing
    doc: Get a thing.
    rust: things::get_thing
    c_symbol: SzConfigTool_getThing
    args:
      - {name: code, type: str}
      - {name: tier, type: int, tristate: true}
    returns: json
    errors: [NOT_FOUND]
";

pub const CASES: &str = "\
group: things
cases:
  - name: get_missing
    fn: get_thing
    args: {code: X, tier: null}
    expect: {error: NOT_FOUND}
";

pub const EXCLUDED: &str = "enforce_complete: false\nentries: []\n";

/// `project.yaml` whose every path starts with `prefix` (`""` = relative to
/// the workspace root; an absolute directory = outside it).
pub fn project(prefix: &str) -> String {
    PROJECT_TEMPLATE.replace('@', prefix)
}

/// Scratch names handed out in this test process, with the test thread
/// that owns each.
static OWNERS: Mutex<BTreeMap<String, ThreadId>> = Mutex::new(BTreeMap::new());

/// An empty scratch directory named `name`. Tests run concurrently on their
/// own threads, so a name belongs to the first test that uses it: another
/// test reusing it would delete and rewrite that tree while it is read.
pub fn scratch_root(name: &str) -> PathBuf {
    let me = std::thread::current().id();
    let owner = *OWNERS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .entry(name.to_owned())
        .or_insert(me);
    assert_eq!(owner, me, "scratch dir name '{name}' is used by two tests");
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("m/conformance")).unwrap();
    root
}

/// Write the manifest tree (`m/...`) under `root`.
pub fn workspace_with(root: &Path, project: &str, group: &str, cases: &str) {
    std::fs::write(root.join("m/project.yaml"), project).unwrap();
    std::fs::write(root.join("m/excluded.yaml"), EXCLUDED).unwrap();
    std::fs::write(root.join("m/things.yaml"), group).unwrap();
    std::fs::write(root.join("m/conformance/things.yaml"), cases).unwrap();
}

/// Write a scratch workspace with workspace-relative paths; returns its root.
pub fn workspace(name: &str, group: &str, cases: &str) -> PathBuf {
    let root = scratch_root(name);
    workspace_with(&root, &project(""), group, cases);
    root
}

/// The full error chain of generating a manifest that must be rejected.
pub fn error_of(name: &str, group: &str, cases: &str) -> String {
    let root = workspace(name, group, cases);
    format!(
        "{:#}",
        generate(&root, Path::new("m/project.yaml")).expect_err("should be rejected")
    )
}
