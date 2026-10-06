//! `cargo run -p sz-configtool-codegen [-- --check] [--project <path>]`
//!
//! Regenerates the outputs named by `project.yaml`. With `--check`, writes
//! nothing and exits non-zero if any checked-in output is stale (for CI).

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sz_configtool_codegen::{DEFAULT_PROJECT_FILE, generate, stale, write};

struct Cli {
    check: bool,
    project: PathBuf,
}

fn parse_cli() -> Result<Cli, String> {
    let mut cli = Cli {
        check: false,
        project: PathBuf::from(DEFAULT_PROJECT_FILE),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => cli.check = true,
            "--project" => {
                cli.project = PathBuf::from(args.next().ok_or("--project needs a path")?);
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    Ok(cli)
}

/// The workspace root: two levels above this crate (`tools/codegen`).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .unwrap_or_default()
}

fn main() -> ExitCode {
    let cli = match parse_cli() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let root = workspace_root();
    let outputs = match generate(&root, &cli.project) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e:#}");
            return ExitCode::FAILURE;
        }
    };
    if cli.check {
        let stale = stale(&root, &outputs);
        if stale.is_empty() {
            return ExitCode::SUCCESS;
        }
        eprintln!("stale generated files (run `cargo run -p sz-configtool-codegen`):");
        for p in stale {
            eprintln!("  {}", p.display());
        }
        return ExitCode::FAILURE;
    }
    match write(&root, &outputs) {
        Ok(()) => {
            for g in &outputs {
                println!("wrote {}", g.path.display());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
