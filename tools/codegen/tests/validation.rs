//! Codegen rejects malformed manifests. Each test writes a small, real
//! manifest tree into a scratch workspace and runs the real generator on it.

mod common;

use std::path::Path;

use common::{CASES, GROUP, error_of, workspace};
use sz_configtool_codegen::generate;

#[test]
fn test_valid_manifest_generates() {
    let root = workspace("valid", GROUP, CASES);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let rs = &out[0].contents;
    assert!(rs.contains("lib::things::get_thing("), "{rs}");
    assert!(rs.contains("args.tri_int(\"tier\")?"), "{rs}");
    assert!(out[2].contents.contains("\"get_missing\""));
}

#[test]
fn test_group_exclusions_merge_into_manifest() {
    let group =
        format!("{GROUP}excluded:\n  - {{rust: things::helper, reason: Internal helper}}\n");
    let root = workspace("group_excluded", &group, CASES);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let manifest: serde_json::Value = serde_json::from_str(&out[1].contents).unwrap();
    assert_eq!(manifest["excluded"]["entries"][0]["rust"], "things::helper");
    let bad = format!("{GROUP}excluded:\n  - {{rust: things::helper, reason: \"\"}}\n");
    assert!(error_of("group_excluded_bad", &bad, CASES).contains("needs a reason"));
}

#[test]
fn test_rejects_unknown_conformance_arg() {
    let cases = CASES.replace("tier: null", "tierr: 1");
    assert!(error_of("unknown_arg", GROUP, &cases).contains("unknown arg 'tierr'"));
}

#[test]
fn test_rejects_null_for_non_tristate_and_missing_required() {
    let cases = CASES.replace("{code: X, tier: null}", "{tier: 1}");
    assert!(error_of("missing_req", GROUP, &cases).contains("missing required arg 'code'"));
    let cases = CASES.replace("code: X", "code: null");
    assert!(error_of("null_req", GROUP, &cases).contains("null is only valid for tristate"));
}

#[test]
fn test_rejects_error_not_listed_for_function() {
    let cases = CASES.replace("error: NOT_FOUND", "error: INVALID_INPUT");
    assert!(error_of("unlisted_err", GROUP, &cases).contains("not listed in get_thing.errors"));
}

#[test]
fn test_rejects_unknown_reason_code_and_bad_tristate() {
    let group = GROUP.replace("errors: [NOT_FOUND]", "errors: [NOT_FOUND, BOGUS]");
    assert!(error_of("bogus_code", &group, CASES).contains("unknown reason code 'BOGUS'"));
    let group = GROUP.replace("type: int, tristate", "type: bool, tristate");
    let cases = CASES.replace("tier: null", "tier: true");
    assert!(error_of("bool_tri", &group, &cases).contains("tristate is only supported"));
}

#[test]
fn test_int_or_str_needs_rust_convert_and_accepts_int_or_string() {
    let group = GROUP.replace("{name: code, type: str}", "{name: code, type: int_or_str}");
    assert!(error_of("ios_no_convert", &group, CASES).contains("int_or_str needs rust_convert"));
    let group = GROUP.replace(
        "{name: code, type: str}",
        "{name: code, type: int_or_str, rust_convert: call_selector}",
    );
    for (name, value) in [("ios_str", "X"), ("ios_int", "7")] {
        let root = workspace(
            name,
            &group,
            &CASES.replace("code: X", &format!("code: {value}")),
        );
        let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
        let manifest = out
            .iter()
            .find(|g| g.path.ends_with("manifest.json"))
            .unwrap();
        assert!(manifest.contents.contains("\"int_or_str\""), "{name}");
    }
    let bad = CASES.replace("code: X", "code: [X]");
    assert!(error_of("ios_list", &group, &bad).contains("arg 'code' must be int_or_str"));
}

#[test]
fn test_rejects_unknown_yaml_field_and_group_mismatch() {
    let group = GROUP.replace("    returns: json", "    retruns: json\n    returns: json");
    assert!(error_of("typo_field", &group, CASES).contains("unknown field"));
    let group = GROUP.replace("group: things", "group: other");
    assert!(error_of("group_name", &group, CASES).contains("must equal the file name"));
}

#[test]
fn test_rejects_result_checks_on_opaque_config() {
    let group = GROUP.replace("returns: json", "returns: config");
    let cases = CASES.replace("{error: NOT_FOUND}", "{result: {A: 1}}");
    assert!(error_of("opaque", &group, &cases).contains("config result is opaque"));
}

#[test]
fn test_rejects_array_checks_on_object_or_scalar_results() {
    for (name, returns) in [
        ("arr_int", "returns: int"),
        ("arr_record", "returns: config_and_json"),
        ("arr_tuple", "returns: json\n    tuple_names: [a, b]"),
    ] {
        let group = GROUP.replace("returns: json", returns);
        for check in ["{len: 0}", "{excludes: [{A: 1}]}", "{contains: [{A: 1}]}"] {
            let cases = CASES.replace("{error: NOT_FOUND}", check);
            let err = error_of(name, &group, &cases);
            assert!(
                err.contains("need an array result"),
                "{returns} {check}: {err}"
            );
        }
    }
    // A plain `json` function may still use them.
    let cases = CASES.replace("{error: NOT_FOUND}", "{len: 0}");
    let root = workspace("arr_json_ok", GROUP, &cases);
    generate(&root, Path::new("m/project.yaml")).expect("json may use len");
}

/// GROUP plus a library-required `Option` arg (`required: true`).
fn group_with_required() -> String {
    GROUP.replace(
        "      - {name: tier, type: int, tristate: true}\n",
        "      - {name: tier, type: int, tristate: true}\n      - {name: plan, type: str, optional: true, required: true}\n",
    )
    .replace("errors: [NOT_FOUND]", "errors: [NOT_FOUND, MISSING_FIELD]")
}

#[test]
fn test_required_flag_is_emitted_and_invoke_stays_optional() {
    let group = group_with_required();
    let cases = CASES.replace("{code: X, tier: null}", "{code: X, tier: null, plan: P}");
    let root = workspace("required_ok", &group, &cases);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let manifest: serde_json::Value = serde_json::from_str(&out[1].contents).unwrap();
    let args = &manifest["functions"][0]["args"];
    assert_eq!(args[2]["name"], "plan");
    assert_eq!(args[2]["required"], true);
    assert_eq!(args[0]["required"], false);
    // invoke keeps the library's Option semantics (absent = None -> MISSING_FIELD).
    assert!(out[0].contents.contains("args.opt_str(\"plan\")?"));
}

#[test]
fn test_required_flag_omission_only_for_missing_field_and_marked_wire_only() {
    let group = group_with_required();
    let cases = CASES.replace("{error: NOT_FOUND}", "{error: MISSING_FIELD}");
    let root = workspace("required_omit", &group, &cases);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let conf: serde_json::Value = serde_json::from_str(&out[2].contents).unwrap();
    assert_eq!(conf["cases"][0]["steps"][0]["wire_only"], true);
    // Omitting it with any other expectation is not expressible by a typed binding.
    let err = error_of("required_omit_bad", &group, CASES);
    assert!(err.contains("omits required arg 'plan'"), "{err}");
    // A step that passes it is not wire-only.
    let cases = CASES.replace("{code: X, tier: null}", "{code: X, tier: null, plan: P}");
    let root = workspace("required_given", &group, &cases);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let conf: serde_json::Value = serde_json::from_str(&out[2].contents).unwrap();
    assert!(conf["cases"][0]["steps"][0].get("wire_only").is_none());
}

#[test]
fn test_required_flag_needs_plain_optional() {
    let bad = GROUP.replace(
        "{name: code, type: str}",
        "{name: code, type: str, required: true}",
    );
    assert!(error_of("required_not_opt", &bad, CASES).contains("required needs optional"));
    let bad = GROUP.replace("tristate: true}", "tristate: true, required: true}");
    assert!(error_of("required_tri", &bad, CASES).contains("required needs optional"));
    let bad = GROUP.replace(
        "{name: code, type: str}",
        "{name: code, type: str, optional: true, required: true, default: X}",
    );
    assert!(error_of("required_default", &bad, CASES).contains("required and default"));
}

#[test]
fn test_tuple_names_name_a_json_tuple() {
    let group = GROUP.replace(
        "returns: json",
        "returns: json\n    tuple_names: [version, matches]",
    );
    let root = workspace("json_tuple", &group, CASES);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let rs = &out[0].contents;
    assert!(rs.contains("let (t0, t1) = result;"), "{rs}");
    assert!(
        rs.contains("crate::output::json(serde_json::json!({ \"version\": t0, \"matches\": t1 }))"),
        "{rs}"
    );
    let bad = GROUP.replace("returns: json", "returns: config\n    tuple_names: [a, b]");
    assert!(error_of("config_tuple", &bad, CASES).contains("tuple_names needs returns"));
}

#[test]
fn test_status_not_implemented() {
    let root = workspace("status_default", GROUP, CASES);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let manifest: serde_json::Value = serde_json::from_str(&out[1].contents).unwrap();
    assert_eq!(manifest["functions"][0]["status"], "implemented");

    let group = GROUP
        .replace(
            "returns: json",
            "returns: json\n    status: not_implemented",
        )
        .replace("errors: [NOT_FOUND]", "errors: [NOT_IMPLEMENTED]");
    let cases = CASES.replace("NOT_FOUND", "NOT_IMPLEMENTED");
    let root = workspace("status_ni", &group, &cases);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    let manifest: serde_json::Value = serde_json::from_str(&out[1].contents).unwrap();
    assert_eq!(manifest["functions"][0]["status"], "not_implemented");
    // Still dispatched by invoke.
    assert!(
        out[0]
            .contents
            .contains("\"get_thing\" => Some(call_get_thing)")
    );

    let no_code = GROUP.replace(
        "returns: json",
        "returns: json\n    status: not_implemented",
    );
    assert!(error_of("status_no_code", &no_code, CASES).contains("must list NOT_IMPLEMENTED"));
    let both = group.replace(
        "errors: [NOT_IMPLEMENTED]",
        "errors: [NOT_IMPLEMENTED, NOT_FOUND]",
    );
    let err = error_of("status_bad_case", &both, CASES);
    assert!(
        err.contains("not_implemented function must expect NOT_IMPLEMENTED"),
        "{err}"
    );
}

#[test]
fn test_c_notes_reach_manifest_json_only() {
    let group = GROUP.replace(
        "    errors: [NOT_FOUND]\n",
        "    errors: [NOT_FOUND]\n    notes: LIB_NOTE_MARKER.\n    c_notes: C_NOTE_MARKER (SzConfigTool_getThing returns -5).\n",
    );
    let root = workspace("c_notes", &group, CASES);
    let out = generate(&root, Path::new("m/project.yaml")).expect("valid");
    for g in &out {
        let in_manifest = g.path.ends_with("manifest.json");
        assert_eq!(
            g.contents.contains("C_NOTE_MARKER"),
            in_manifest,
            "c_notes must appear only in manifest.json: {}",
            g.path.display()
        );
    }
    // Every language that documents notes still gets the library note.
    let documented = out
        .iter()
        .filter(|g| g.contents.contains("LIB_NOTE_MARKER"))
        .count();
    assert!(
        documented >= 6,
        "notes reach the language docs ({documented})"
    );
}

#[test]
fn test_rejects_c_abi_text_in_notes_doc_and_semantics() {
    for (name, group) in [
        (
            "c_in_notes",
            GROUP.replace(
                "    errors: [NOT_FOUND]\n",
                "    errors: [NOT_FOUND]\n    notes: The C typed export drops the row.\n",
            ),
        ),
        (
            "c_in_doc",
            GROUP.replace(
                "doc: Get a thing.",
                "doc: Get a thing (SzConfigTool_getThing returns -5).",
            ),
        ),
        (
            "c_in_semantics",
            GROUP.replace(
                "{name: code, type: str}",
                "{name: code, type: str, semantics: \"C NULL = None.\"}",
            ),
        ),
    ] {
        let err = error_of(name, &group, CASES);
        assert!(err.contains("move C-ABI text to c_notes"), "{name}: {err}");
    }
}

/// A `config_and_json` function's typed bindings add a `<name>_result`
/// companion, so a manifest function of that name would collide.
#[test]
fn test_rejects_function_named_like_a_companion() {
    let pair = GROUP.replace("returns: json", "returns: config_and_json");
    let clash = format!(
        "{pair}  - name: get_thing_result\n    doc: Clash.\n    rust: things::get_thing_result\n    \
         returns: json\n"
    );
    assert!(
        error_of("companion_clash", &clash, CASES)
            .contains("get_thing_result: collides with the typed companion of get_thing")
    );
    let root = workspace("companion_ok", &pair, CASES);
    generate(&root, Path::new("m/project.yaml")).expect("no clash");
}
