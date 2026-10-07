//! Builds the C library, then compiles and runs the C test
//! (`tests/c/test_basic.c`) and the C example (`examples/c_ffi_example.c`)
//! against it with the system C compiler (`$CC`, default `cc`). Either program
//! exiting non-zero fails this test.
//!
//! Cargo does not produce a cdylib for integration tests, so the test invokes
//! `cargo build -p sz-configtool-ffi` itself into the same target directory.
//! Unix only (uses `cc`-style flags and pthreads).
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

/// `target/<profile>` — the directory containing this test binary's `deps/`.
fn profile_dir() -> PathBuf {
    let exe = std::env::current_exe().expect("current_exe");
    exe.parent()
        .and_then(Path::parent)
        .expect("test binary lives in target/<profile>/deps")
        .to_path_buf()
}

fn build_library(profile_dir: &Path) {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let target_dir = profile_dir.parent().expect("target dir");
    let profile = match profile_dir.file_name().and_then(|n| n.to_str()) {
        Some("debug") => "dev",
        Some(other) => other,
        None => panic!("unrecognized profile dir {}", profile_dir.display()),
    };
    let status = Command::new(cargo)
        .args(["build", "-p", "sz-configtool-ffi", "--profile", profile])
        .arg("--target-dir")
        .arg(target_dir)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("spawn cargo build");
    assert!(status.success(), "cargo build -p sz-configtool-ffi failed");
}

fn compile_and_run(source: &str, lib_dir: &Path) {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = lib_dir.join("c-abi-tests");
    std::fs::create_dir_all(&out_dir).expect("create output dir");
    let stem = Path::new(source).file_stem().unwrap().to_str().unwrap();
    let exe = out_dir.join(stem);

    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let output = Command::new(&cc)
        .args(["-std=c11", "-Wall", "-Wextra", "-Werror", "-pthread"])
        .arg(format!("-I{}", manifest.join("include").display()))
        .arg(manifest.join(source))
        .arg("-o")
        .arg(&exe)
        .arg(format!("-L{}", lib_dir.display()))
        .arg("-lSzConfigTool")
        .arg(format!("-Wl,-rpath,{}", lib_dir.display()))
        .output()
        .unwrap_or_else(|e| panic!("spawn {cc}: {e}"));
    assert!(
        output.status.success(),
        "compiling {source} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let run = Command::new(&exe)
        .output()
        .unwrap_or_else(|e| panic!("run {}: {e}", exe.display()));
    println!("{}", String::from_utf8_lossy(&run.stdout));
    assert!(
        run.status.success(),
        "{source} exited with {}:\nstdout:\n{}\nstderr:\n{}",
        run.status,
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}

#[test]
fn test_c_programs_build_and_pass() {
    let lib_dir = profile_dir();
    build_library(&lib_dir);
    compile_and_run("tests/c/test_basic.c", &lib_dir);
    compile_and_run("examples/c_ffi_example.c", &lib_dir);
}
