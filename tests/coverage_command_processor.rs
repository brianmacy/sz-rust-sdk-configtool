//! Public-API coverage for `CommandProcessor`: file input, script error
//! reporting, dry-run bookkeeping and the read-only accessors.

use std::io::Write;
use sz_configtool_lib::SzErrorKind;
use sz_configtool_lib::command_processor::CommandProcessor;
use sz_configtool_lib::versioning::get_compatibility_version;

const TEMPLATE: &str = include_str!("fixtures/g2config_template.json");

const SCRIPT: &str = "\
# comment lines and blank lines are skipped

verifyCompatibilityVersion {\"expectedVersion\": \"11\"}
updateCompatibilityVersion {\"toVersion\": \"12\"}
save
";

#[test]
fn test_process_file_applies_script() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(SCRIPT.as_bytes()).unwrap();

    let mut processor = CommandProcessor::new(TEMPLATE.to_string());
    let out = processor.process_file(file.path()).unwrap();

    assert_eq!(get_compatibility_version(&out).unwrap(), "12");
    assert_eq!(processor.get_config(), out);
    // `save` is a no-op and is not tracked; comments/blank lines are skipped.
    assert_eq!(
        processor.get_executed_commands(),
        [
            "Line 3: verifyCompatibilityVersion {\"expectedVersion\": \"11\"}",
            "Line 4: updateCompatibilityVersion {\"toVersion\": \"12\"}",
        ]
    );
    assert_eq!(processor.summary(), "Executed 2 commands");
}

#[test]
fn test_process_file_missing_file_is_invalid_config() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("absent.gtc");

    let mut processor = CommandProcessor::new(TEMPLATE.to_string());
    let err = processor.process_file(&missing).unwrap_err();

    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert!(
        err.to_string()
            .starts_with("Invalid configuration: Failed to read script file: "),
        "{err}"
    );
    assert!(processor.get_executed_commands().is_empty());
}

#[test]
fn test_process_script_error_reports_line_and_command() {
    let script = "save\nremoveConfigSection {\"section\": \"CFG_NOPE\"}\n";
    let mut processor = CommandProcessor::new(TEMPLATE.to_string());
    let err = processor.process_script(script).unwrap_err();

    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert_eq!(
        err.to_string(),
        "Invalid configuration: Line 2: removeConfigSection {\"section\": \"CFG_NOPE\"} \
         - Error: Config section not found: CFG_NOPE"
    );
    // The failing command is not recorded and the config is unchanged.
    assert!(processor.get_executed_commands().is_empty());
    assert_eq!(processor.get_config(), TEMPLATE);
}

#[test]
fn test_dry_run_validates_without_applying() {
    let mut processor = CommandProcessor::new(TEMPLATE.to_string()).dry_run(true);
    let out = processor.process_script(SCRIPT).unwrap();

    assert_eq!(out, TEMPLATE);
    assert_eq!(processor.summary(), "Executed 2 commands (DRY RUN)");

    // A failing command still fails in dry-run mode.
    let err = processor
        .process_script("verifyCompatibilityVersion {}")
        .unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::InvalidConfig);
    assert!(
        err.to_string()
            .ends_with("Error: Missing required field: expectedVersion"),
        "{err}"
    );
}
