//! The real `sz-configtool-codegen` binary (argument handling, `--check`,
//! write mode, every error exit) and the library's `stale` / `write`.
//! Scratch manifests use ABSOLUTE paths so the binary, whose workspace root is
//! fixed at build time, reads and writes only inside the scratch directory.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{CASES, GROUP, error_of, project, scratch_root, workspace, workspace_with};
use sz_configtool_codegen::{Generated, generate, stale, write};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sz-configtool-codegen"))
        .args(args)
        .output()
        .expect("spawn sz-configtool-codegen")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Whether some line of `text` is `prefix` followed by `path`. Compared as
/// paths, not text: the binary prints the manifest's spelling
/// (`<root>/out/dispatch.rs`) while `Path::join` uses the native separator
/// (`<root>\out/dispatch.rs` on Windows).
fn lists_path(text: &str, prefix: &str, path: &Path) -> bool {
    text.lines()
        .filter_map(|line| line.strip_prefix(prefix))
        .any(|listed| Path::new(listed) == path)
}

/// A scratch manifest whose project paths are absolute; returns
/// (root, absolute project.yaml path).
fn absolute_workspace(name: &str, group: &str) -> (PathBuf, String) {
    let root = scratch_root(name);
    let prefix = format!("{}/", root.display());
    workspace_with(&root, &project(&prefix), group, CASES);
    let project_file = root.join("m/project.yaml").display().to_string();
    (root, project_file)
}

#[test]
fn test_check_passes_on_the_real_checked_in_outputs() {
    let out = run(&["--check"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(out.stdout.is_empty());
}

#[test]
fn test_bad_arguments_exit_2() {
    for (args, msg) in [
        (&["--bogus"][..], "error: unknown argument '--bogus'"),
        (
            &["--check", "--project"][..],
            "error: --project needs a path",
        ),
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(stderr(&out).contains(msg), "{args:?}: {}", stderr(&out));
    }
}

#[test]
fn test_unloadable_project_exits_1() {
    let missing = scratch_root("cov_cli_missing").join("nope.yaml");
    let out = run(&["--project", &missing.display().to_string()]);
    assert_eq!(out.status.code(), Some(1));
    let err = stderr(&out);
    assert!(err.contains("error: reading"), "{err}");
    assert!(err.contains("nope.yaml"), "{err}");
}

#[test]
fn test_invalid_manifest_exits_1_with_the_library_error() {
    let group = GROUP.replace("errors: [NOT_FOUND]", "errors: [BOGUS]");
    let (_, project_file) = absolute_workspace("cov_cli_invalid", &group);
    let out = run(&["--check", "--project", &project_file]);
    assert_eq!(out.status.code(), Some(1));
    let expected = error_of("cov_cli_invalid_lib", &group, CASES);
    assert!(
        expected.contains("unknown reason code 'BOGUS'"),
        "{expected}"
    );
    assert_eq!(stderr(&out), format!("error: {expected}\n"));
}

#[test]
fn test_write_then_check_fresh_then_stale() {
    let (root, project_file) = absolute_workspace("cov_cli_write", GROUP);
    let out = run(&["--project", &project_file]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let dispatch = root.join("out/dispatch.rs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(lists_path(&stdout, "wrote ", &dispatch), "{stdout}");
    assert!(
        std::fs::read_to_string(&dispatch)
            .unwrap()
            .contains("call_get_thing")
    );

    let fresh = run(&["--check", "--project", &project_file]);
    assert_eq!(fresh.status.code(), Some(0), "{}", stderr(&fresh));

    std::fs::write(&dispatch, "// hand edit\n").unwrap();
    let stale_run = run(&["--project", &project_file, "--check"]);
    assert_eq!(stale_run.status.code(), Some(1));
    let err = stderr(&stale_run);
    assert!(err.contains("stale generated files"), "{err}");
    assert!(lists_path(&err, "  ", &dispatch), "{err}");
}

#[test]
fn test_write_failure_exits_1() {
    let (root, project_file) = absolute_workspace("cov_cli_write_fail", GROUP);
    // The dispatcher's parent directory is a regular file.
    std::fs::write(root.join("out"), "not a directory").unwrap();
    let out = run(&["--project", &project_file]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).starts_with("error: "), "{}", stderr(&out));
    assert!(out.stdout.is_empty());
}

fn one(path: &Path, contents: &str) -> Vec<Generated> {
    vec![Generated {
        path: path.to_path_buf(),
        contents: contents.to_string(),
    }]
}

#[test]
fn test_write_creates_dirs_and_stale_compares_contents() {
    let root = workspace("cov_lib_write", GROUP, CASES);
    let outputs = generate(&root, Path::new("m/project.yaml")).expect("valid");
    assert_eq!(
        stale(&root, &outputs).len(),
        outputs.len(),
        "nothing written yet"
    );
    write(&root, &outputs).expect("write");
    assert!(stale(&root, &outputs).is_empty());
    let changed = one(&outputs[0].path, "different\n");
    assert_eq!(stale(&root, &changed), vec![outputs[0].path.clone()]);
}

#[test]
fn test_write_errors_on_blocked_dir_and_directory_target() {
    // Opening a directory for writing: EISDIR on Unix; on Windows CreateFileW
    // fails with ERROR_ACCESS_DENIED, which std reports as PermissionDenied.
    let directory_target = if cfg!(windows) {
        std::io::ErrorKind::PermissionDenied
    } else {
        std::io::ErrorKind::IsADirectory
    };
    let root = scratch_root("cov_lib_write_err");
    std::fs::write(root.join("blocker"), "file").unwrap();
    for (path, kind) in [
        ("blocker/x.rs", std::io::ErrorKind::AlreadyExists),
        ("m/conformance", directory_target),
    ] {
        let err = write(&root, &one(Path::new(path), "x\n")).expect_err(path);
        let io = err.downcast_ref::<std::io::Error>().expect("an io::Error");
        assert_eq!(io.kind(), kind, "{path}: {io}");
    }
}

#[test]
fn test_write_of_an_empty_path_has_no_parent_and_fails() {
    // `"".join("")` has no parent directory to create; the write itself fails.
    let err = write(Path::new(""), &one(Path::new(""), "x\n")).expect_err("empty path");
    let io = err.downcast_ref::<std::io::Error>().expect("an io::Error");
    assert_eq!(io.kind(), std::io::ErrorKind::NotFound, "{io}");
}
