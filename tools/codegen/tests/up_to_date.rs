//! The checked-in generated files must equal a fresh generation from the real
//! manifest; otherwise someone edited a YAML file without regenerating (or
//! hand-edited a generated file).

use std::path::Path;

use sz_configtool_codegen::{DEFAULT_PROJECT_FILE, generate, stale};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("tools/codegen lives two levels below the workspace root")
}

#[test]
fn test_generated_files_are_up_to_date() {
    let root = workspace_root();
    let outputs = generate(root, Path::new(DEFAULT_PROJECT_FILE)).expect("manifest is valid");
    let stale = stale(root, &outputs);
    assert!(
        stale.is_empty(),
        "stale generated files {stale:?}: run `cargo run -p sz-configtool-codegen`"
    );
}

#[test]
fn test_generated_files_carry_banner() {
    let root = workspace_root();
    for g in generate(root, Path::new(DEFAULT_PROJECT_FILE)).expect("manifest is valid") {
        assert!(
            g.contents.contains("GENERATED — do not edit"),
            "{} lacks the banner",
            g.path.display()
        );
        assert!(
            g.contents.ends_with('\n'),
            "{} lacks EOF newline",
            g.path.display()
        );
    }
}
