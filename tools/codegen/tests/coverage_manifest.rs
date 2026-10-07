//! Every load and validation rejection, each driven by a real scratch
//! manifest, plus the public `mark_wire_only` / `model` helpers, and a scratch
//! manifest that uses every wire type and return kind the real manifest does
//! not (bool, optional str_list, int_or_str, `returns: int` / `unit`, an empty
//! `errors` list, a multi-paragraph doc with quotes, backslashes and newlines)
//! rendered through every generator.

mod common;

use std::path::Path;

use common::{CASES, GROUP, error_of, workspace};
use sz_configtool_codegen::load::load;
use sz_configtool_codegen::model::{ArgType, Returns};
use sz_configtool_codegen::validate::{mark_wire_only, validate};
use sz_configtool_codegen::{Generated, generate};

const ARG: &str = "{name: code, type: str}";

/// GROUP with a second function block (`name`, `rust`, `c_symbol` given).
fn with_second_function(name: &str, rust: &str, symbol: &str) -> String {
    format!(
        "{GROUP}  - name: {name}\n    doc: Another.\n    rust: {rust}\n    c_symbol: {symbol}\n    args: []\n    returns: json\n    errors: [NOT_FOUND]\n"
    )
}

/// GROUP with `rust_params_struct` and the `code` arg replaced by `arg`.
fn with_struct_arg(arg: &str) -> String {
    GROUP
        .replace(
            "    c_symbol:",
            "    rust_params_struct: things::Params\n    c_symbol:",
        )
        .replace(ARG, arg)
}

#[test]
fn test_rejects_every_function_and_arg_shape_problem() {
    let table: Vec<(&str, String, &str)> = vec![
        (
            "cov_requires_options",
            GROUP
                .replace("    c_symbol:", "    requires_options: true\n    c_symbol:")
                .replace("      - {name: tier, type: int, tristate: true}\n", ""),
            "get_thing: requires_options needs an optional argument",
        ),
        (
            "cov_arg_name",
            GROUP.replace(ARG, "{name: Code, type: str}"),
            "get_thing.Code: arg name must be snake_case",
        ),
        (
            "cov_owned_str",
            GROUP.replace(ARG, "{name: code, type: str, owned: true}"),
            "owned is only valid for type json",
        ),
        (
            "cov_owned_conv",
            GROUP.replace(
                ARG,
                "{name: code, type: json, owned: true, rust_convert: as_thing}",
            ),
            "owned and rust_convert are exclusive",
        ),
        (
            "cov_conv_case",
            GROUP.replace(ARG, "{name: code, type: str, rust_convert: AsThing}"),
            "rust_convert must be a snake_case fn name",
        ),
        (
            "cov_positional_nostruct",
            GROUP.replace(ARG, "{name: code, type: str, positional: true}"),
            "positional/field require rust_params_struct",
        ),
        (
            "cov_positional_field",
            with_struct_arg("{name: code, type: str, positional: true, field: c}"),
            "positional and field are exclusive",
        ),
        (
            "cov_field_case",
            with_struct_arg("{name: code, type: str, field: Code}"),
            "field must be snake_case",
        ),
        (
            "cov_fn_name",
            GROUP.replace("name: get_thing", "name: GetThing"),
            "GetThing: name must be snake_case",
        ),
        (
            "cov_doc_empty",
            GROUP.replace("doc: Get a thing.", "doc: \" \""),
            "get_thing: doc is required",
        ),
        (
            "cov_rust_path",
            GROUP.replace("rust: things::get_thing", "rust: get_thing"),
            "rust 'get_thing' must be module::fn",
        ),
        (
            "cov_struct_path",
            GROUP.replace(
                "    c_symbol:",
                "    rust_params_struct: Params\n    c_symbol:",
            ),
            "rust_params_struct 'Params' must be module::Type",
        ),
        (
            "cov_dup_arg",
            GROUP.replace(ARG, &format!("{ARG}\n      - {ARG}")),
            "get_thing: duplicate arg 'code'",
        ),
        (
            "cov_dup_error",
            GROUP.replace("errors: [NOT_FOUND]", "errors: [NOT_FOUND, NOT_FOUND]"),
            "get_thing: duplicate error 'NOT_FOUND'",
        ),
        (
            "cov_dup_fn",
            with_second_function("get_thing", "things::other", "SzConfigTool_other"),
            "get_thing: duplicate function name",
        ),
        (
            "cov_dup_rust",
            with_second_function("other", "things::get_thing", "SzConfigTool_other"),
            "other: rust path 'things::get_thing' mapped twice",
        ),
        (
            "cov_symbol_prefix",
            GROUP.replace("SzConfigTool_getThing", "Sz_getThing"),
            "C symbol 'Sz_getThing' must start with SzConfigTool_",
        ),
        (
            "cov_symbol_twice",
            GROUP.replace(
                "    c_symbol: SzConfigTool_getThing\n",
                "    c_symbol: SzConfigTool_getThing\n    c_aliases: [SzConfigTool_getThing]\n",
            ),
            "C symbol 'SzConfigTool_getThing' used twice",
        ),
        (
            "cov_excluded_both",
            format!("{GROUP}excluded:\n  - {{rust: things::h, module: things, reason: R}}\n"),
            "each entry needs exactly one of a valid `rust` or `module`",
        ),
    ];
    for (name, group, want) in table {
        let err = error_of(name, &group, CASES);
        assert!(err.contains(want), "{name}: want '{want}' in: {err}");
    }
}

#[test]
fn test_rejects_every_case_problem() {
    let dup_case = format!(
        "{CASES}  - name: get_missing\n    fn: get_thing\n    args: {{code: Y}}\n    expect: {{error: NOT_FOUND}}\n"
    );
    let table: Vec<(&str, String, &str)> = vec![
        (
            "cov_case_kind",
            CASES.replace("{error: NOT_FOUND}", "{kind: config}"),
            "things/get_missing step 0: kind 'config' but get_thing returns 'json'",
        ),
        (
            "cov_case_error_plus",
            CASES.replace("{error: NOT_FOUND}", "{error: NOT_FOUND, len: 0}"),
            "`error` excludes every other expectation",
        ),
        (
            "cov_case_dup",
            dup_case,
            "things/get_missing: duplicate case name",
        ),
        (
            "cov_case_no_steps",
            "group: things\ncases:\n  - name: empty\n    steps: []\n".to_string(),
            "things/empty: no steps",
        ),
        (
            "cov_case_unknown_fn",
            CASES.replace("fn: get_thing", "fn: nope"),
            "things/get_missing step 0: unknown fn 'nope'",
        ),
    ];
    for (name, cases, want) in table {
        let err = error_of(name, GROUP, &cases);
        assert!(err.contains(want), "{name}: want '{want}' in: {err}");
    }
}

#[test]
fn test_rejects_every_load_problem() {
    let table: Vec<(&str, String, String, &str)> = vec![
        (
            "cov_load_fn_group",
            GROUP.replace("    returns: json", "    group: other\n    returns: json"),
            CASES.to_string(),
            "function 'get_thing' declares group 'other'",
        ),
        (
            "cov_load_inline_no_fn",
            GROUP.to_string(),
            CASES.replace("    fn: get_thing\n", ""),
            "case 'get_missing': inline form needs `fn`",
        ),
        (
            "cov_load_both_forms",
            GROUP.to_string(),
            CASES.replace("    fn: get_thing\n", "    fn: get_thing\n    steps: []\n"),
            "case 'get_missing': use EITHER inline fn/args/expect OR steps",
        ),
        (
            "cov_load_cases_group",
            GROUP.to_string(),
            CASES.replace("group: things", "group: stuff"),
            "group 'stuff' must equal the file name",
        ),
    ];
    for (name, group, cases, want) in table {
        let err = error_of(name, &group, &cases);
        assert!(err.contains(want), "{name}: want '{want}' in: {err}");
    }
}

/// Generate after `mutate` changes the scratch tree; returns the error chain.
fn tree_error(name: &str, mutate: impl FnOnce(&Path)) -> String {
    let root = workspace(name, GROUP, CASES);
    mutate(&root);
    format!(
        "{:#}",
        generate(&root, Path::new("m/project.yaml")).expect_err("should be rejected")
    )
}

#[test]
fn test_rejects_missing_or_unparsable_files() {
    let err = tree_error("cov_tree_no_excluded", |r| {
        std::fs::remove_file(r.join("m/excluded.yaml")).unwrap();
    });
    assert!(err.starts_with("reading "), "{err}");
    assert!(err.contains("excluded.yaml"), "{err}");

    let err = tree_error("cov_tree_no_cases", |r| {
        std::fs::remove_dir_all(r.join("m/conformance")).unwrap();
    });
    assert!(err.starts_with("listing "), "{err}");
    assert!(err.contains("conformance"), "{err}");

    let err = tree_error("cov_tree_bad_project", |r| {
        std::fs::write(r.join("m/project.yaml"), "paths: [not, a, map]\n").unwrap();
    });
    assert!(err.starts_with("parsing "), "{err}");

    let err = tree_error("cov_tree_bad_group_yaml", |r| {
        std::fs::write(r.join("m/things.yaml"), "group: [\n").unwrap();
    });
    assert!(err.starts_with("parsing "), "{err}");
    assert!(err.contains("things.yaml"), "{err}");

    let err = tree_error("cov_tree_bad_cases_yaml", |r| {
        std::fs::write(r.join("m/conformance/things.yaml"), "cases: {\n").unwrap();
    });
    assert!(err.starts_with("parsing "), "{err}");
    assert!(err.contains("conformance"), "{err}");

    let err = tree_error("cov_tree_no_manifest_dir", |r| {
        let project = std::fs::read_to_string(r.join("m/project.yaml")).unwrap();
        let moved = project.replace("manifest_dir: m\n", "manifest_dir: gone\n");
        std::fs::write(r.join("m/project.yaml"), moved).unwrap();
    });
    assert!(err.starts_with("listing "), "{err}");
    assert!(err.contains("gone"), "{err}");
}

#[test]
fn test_module_exclusion_and_non_yaml_files_are_accepted() {
    let group = format!("{GROUP}excluded:\n  - {{module: things::internal, reason: Internal}}\n");
    let root = workspace("cov_module_excluded", &group, CASES);
    std::fs::write(root.join("m/README.md"), "not a group file\n").unwrap();
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let manifest: serde_json::Value = serde_json::from_str(&out[1].contents).unwrap();
    assert_eq!(
        manifest["excluded"]["entries"][0]["module"],
        "things::internal"
    );
    assert_eq!(manifest["functions"].as_array().map(Vec::len), Some(1));
}

#[test]
fn test_mark_wire_only_skips_steps_of_unknown_functions() {
    let root = workspace(
        "cov_wire_only_unknown",
        GROUP,
        &CASES.replace("fn: get_thing", "fn: nope"),
    );
    let mut inputs = load(&root, Path::new("m/project.yaml")).expect("loads");
    let errs = validate(&inputs).expect_err("unknown fn");
    assert!(
        errs.iter().any(|e| e.contains("unknown fn 'nope'")),
        "{errs:?}"
    );
    mark_wire_only(&mut inputs);
    assert!(!inputs.cases[0].steps[0].wire_only);
}

#[test]
fn test_model_wire_names() {
    let types = [
        (ArgType::Str, "str"),
        (ArgType::Int, "int"),
        (ArgType::Bool, "bool"),
        (ArgType::Json, "json"),
        (ArgType::StrList, "str_list"),
        (ArgType::IntOrStr, "int_or_str"),
    ];
    for (ty, name) in types {
        assert_eq!(ty.as_str(), name);
    }
    let returns = [
        (Returns::Config, "config"),
        (Returns::Json, "json"),
        (Returns::ConfigAndJson, "config_and_json"),
        (Returns::Int, "int"),
        (Returns::Unit, "unit"),
    ];
    for (r, name) in returns {
        assert_eq!(r.as_str(), name);
    }
}

const EXTRA: &str = r#"  - name: set_flag
    doc: "Set a \"flag\" under C:\\flags.\n\nA second paragraph that is deliberately long enough to wrap across the documentation line width of every generated language binding."
    rust: things::set_flag
    c_symbol: SzConfigTool_setFlag
    args:
      - {name: on, type: bool, semantics: "Turns the \"flag\" on."}
      - {name: maybe, type: bool, optional: true}
      - {name: tags, type: str_list}
      - {name: more_tags, type: str_list, optional: true}
      - {name: sel, type: int_or_str, rust_convert: call_selector}
    returns: int
    errors: []
  - name: drop_flag
    doc: Drop a flag.
    rust: things::drop_flag
    args: []
    returns: unit
    errors: [NOT_FOUND]
"#;

fn outputs() -> Vec<Generated> {
    let root = workspace("cov_outputs", &format!("{GROUP}{EXTRA}"), CASES);
    generate(&root, Path::new("m/project.yaml")).expect("valid")
}

fn file<'a>(out: &'a [Generated], suffix: &str) -> &'a str {
    &out.iter()
        .find(|g| g.path.to_string_lossy().ends_with(suffix))
        .unwrap_or_else(|| panic!("no output ending in {suffix}"))
        .contents
}

#[test]
fn test_dispatcher_renders_int_unit_bool_and_list_args() {
    let out = outputs();
    let rs = file(&out, "dispatch.rs");
    for want in [
        "args.req_bool(\"on\")?",
        "args.opt_bool(\"maybe\")?",
        "args.req_str_list(\"tags\")?",
        "args.opt_str_list(\"more_tags\")?",
        "crate::convert::call_selector(args, \"sel\")?",
        "    Ok(Output::Int(result))\n",
        "    lib::things::drop_flag(\n        config,\n    )?;\n    Ok(Output::Unit)\n",
    ] {
        assert!(rs.contains(want), "missing {want:?} in:\n{rs}");
    }
}

#[test]
fn test_every_binding_renders_the_extra_shapes() {
    let out = outputs();
    // Each binding: its int / unit call, and its doc escaping of the quotes
    // and backslash in `set_flag`'s doc.
    let table: [(&str, &[&str]); 5] = [
        (
            "cpp/api.hpp",
            &[
                "ResultKind::Int);",
                "ResultKind::Unit);",
                "/// @param on Wire arg `on` (bool).",
                "/// @brief Set a \"flag\" under C:\\\\flags.",
            ],
        ),
        (
            "cs/Api.g.cs",
            &[
                "return long.Parse(NativeCall.Require(NativeCall.Expect(\"int\", \"set_flag\"",
                "NativeCall.Unit(\"drop_flag\"",
                "/// Set a \"flag\" under C:\\flags.",
            ],
        ),
        (
            "java/Api.java",
            &[
                "return Long.parseLong(Invoker.call(\"set_flag\", \"int\"",
                "Invoker.unit(\"drop_flag\"",
                "* Set a \"flag\" under C:&#92;flags.",
            ],
        ),
        (
            "node/f.ts",
            &[
                "return rt.callInt(\"set_flag\"",
                "rt.callUnit(\"drop_flag\"",
                "* Set a \"flag\" under C:\\flags.",
            ],
        ),
        (
            "py/generated.py",
            &[
                "\n    \"\"\"Set a \"flag\" under C:\\\\flags.",
                "        on: Turns the \"flag\" on.",
            ],
        ),
    ];
    for (suffix, wants) in table {
        let text = file(&out, suffix);
        for want in wants {
            assert!(text.contains(want), "{suffix}: missing {want:?}");
        }
    }
}
