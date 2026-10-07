//! `json_type` descriptors (issue #76): every `json` arg declares its shape
//! (or `any`); codegen validates the descriptor, emits it in manifest.json,
//! marks conformance steps whose value does not fit it `wire_only`, and the
//! node generator renders precise TS types, runtime specs and strict Zod.

mod common;

use std::path::Path;

use common::{error_of, workspace};
use sz_configtool_codegen::generate;
use sz_configtool_codegen::model::ArgType;

const GROUP: &str = "\
group: things
functions:
  - name: add_thing
    doc: Add a thing.
    rust: things::add_thing
    c_symbol: null
    args:
      - {name: code, type: str}
      - name: element_list
        type: json
        json_type: @TYPE
    returns: config
    errors: [NOT_FOUND, INVALID_INPUT, MISSING_FIELD]
";

const CASES: &str = "\
group: things
cases:
  - name: add_one
    fn: add_thing
    args: {code: X, element_list: @VALUE}
    expect: {error: @ERROR}
";

const ELEMENTS: &str = "{array: {object: {feature: string, \"flag?\": {enum: [\"Yes\", \"No\"]}}}}";

fn group(ty: &str) -> String {
    GROUP.replace("@TYPE", ty)
}

fn cases(value: &str, error: &str) -> String {
    CASES.replace("@VALUE", value).replace("@ERROR", error)
}

fn rejected(name: &str, ty: &str) -> String {
    error_of(name, &group(ty), &cases("[]", "NOT_FOUND"))
}

fn generated(
    name: &str,
    ty: &str,
    value: &str,
    error: &str,
) -> Vec<sz_configtool_codegen::Generated> {
    let root = workspace(name, &group(ty), &cases(value, error));
    generate(&root, Path::new("m/project.yaml")).expect("valid")
}

fn file<'a>(out: &'a [sz_configtool_codegen::Generated], suffix: &str) -> &'a str {
    &out.iter()
        .find(|g| g.path.to_string_lossy().ends_with(suffix))
        .unwrap_or_else(|| panic!("no output {suffix}"))
        .contents
}

#[test]
fn test_json_arg_requires_json_type() {
    let g = GROUP.replace("        json_type: @TYPE\n", "");
    let err = error_of("jt_missing", &g, &cases("[]", "NOT_FOUND"));
    assert!(
        err.contains("add_thing.element_list: json args need json_type"),
        "{err}"
    );
    // The shared scratch group (no json arg): a json_type on a str arg.
    let g = common::GROUP.replace(
        "{name: code, type: str}",
        "{name: code, type: str, json_type: string}",
    );
    let err = error_of("jt_on_str", &g, common::CASES);
    assert!(
        err.contains("get_thing.code: json_type is only valid for type json"),
        "{err}"
    );
}

#[test]
fn test_rejects_malformed_descriptors() {
    for (name, ty, want) in [
        (
            "jt_enum_empty",
            "{enum: []}",
            "enum needs at least one value",
        ),
        (
            "jt_enum_dup",
            "{enum: [A, A]}",
            "enum value 'A' is repeated",
        ),
        (
            "jt_obj_empty",
            "{object: {}}",
            "object needs at least one field",
        ),
        (
            "jt_obj_dup",
            "{object: {a: string, \"a?\": int}}",
            "field 'a' is repeated",
        ),
        (
            "jt_obj_name",
            "{object: {\"?\": string}}",
            "empty field name",
        ),
        (
            "jt_one_short",
            "{one_of: [string]}",
            "one_of needs at least two alternatives",
        ),
        (
            "jt_one_kind",
            "{one_of: [string, {enum: [A]}]}",
            "one_of alternatives must differ in JSON kind",
        ),
        (
            "jt_one_any",
            "{one_of: [any, string]}",
            "one_of cannot contain any",
        ),
        (
            "jt_nested",
            "{array: {array: {one_of: [int]}}}",
            "element_list[][]: one_of needs at least two",
        ),
    ] {
        let err = rejected(name, ty);
        assert!(err.contains(want), "{name}: {err}");
    }
    for (name, ty) in [
        ("jt_unknown_scalar", "float"),
        ("jt_unknown_tag", "{tuple: [string]}"),
        ("jt_two_tags", "{array: string, enum: [A]}"),
    ] {
        let err = rejected(name, ty);
        assert!(err.contains("json_type"), "{name}: {err}");
    }
}

#[test]
fn test_descriptor_is_emitted_in_manifest_json() {
    let out = generated("jt_emit", ELEMENTS, "[{feature: NAME}]", "NOT_FOUND");
    let manifest: serde_json::Value = serde_json::from_str(file(&out, "manifest.json")).unwrap();
    assert_eq!(
        manifest["functions"][0]["args"][1]["json_type"],
        serde_json::json!({"array": {"object": {"feature": "string", "flag?": {"enum": ["Yes", "No"]}}}})
    );
    assert!(
        manifest["functions"][0]["args"][0]
            .get("json_type")
            .is_none()
    );
}

#[test]
fn test_steps_whose_value_does_not_fit_are_wire_only() {
    for (value, fits) in [
        ("[{feature: NAME, flag: \"Yes\"}]", true),
        ("[{feature: NAME}]", true),
        ("[]", true),
        ("[{feature: NAME, flag: maybe}]", false),
        ("[{flag: \"No\"}]", false),
        ("[{feature: NAME, extra: 1}]", false),
        ("[{feature: 1}]", false),
        ("[NAME]", false),
        ("NAME", false),
    ] {
        let out = generated("jt_wire_only", ELEMENTS, value, "INVALID_INPUT");
        let conf: serde_json::Value = serde_json::from_str(file(&out, "conformance.json")).unwrap();
        let step = &conf["cases"][0]["steps"][0];
        assert_eq!(step.get("wire_only").is_none(), fits, "{value}: {step}");
    }
}

#[test]
fn test_node_renders_types_specs_and_strict_zod() {
    let ty = "{array: {one_of: [string, {object: {element: string, \"level?\": int, \"on?\": bool, \"x?\": any}}]}}";
    let out = generated("jt_node", ty, "[E]", "NOT_FOUND");
    let ts = file(&out, "node/f.ts");
    assert!(
        ts.contains(
            "  readonly elementList: ReadonlyArray<string | { readonly element: string; readonly level?: number | bigint; readonly on?: boolean; readonly x?: rt.JsonValue }>;\n"
        ),
        "{ts}"
    );
    assert!(
        ts.contains(
            "[\"elementList\", \"element_list\", true, { array: { oneOf: [\"string\", { object: { \"element\": \"string\", \"level?\": \"int\", \"on?\": \"bool\", \"x?\": \"any\" } }] } }]"
        ),
        "{ts}"
    );
    assert!(
        ts.contains("Shape: `[string | {element: string, level?: int, on?: bool, x?: any}]`."),
        "{ts}"
    );
    let zod = file(&out, "node/s.ts");
    assert!(
        zod.contains(
            "  elementList: z.array(z.union([z.string(), z.strictObject({ \"element\": z.string(), \"level\": z.union([z.int(), z.bigint()]).optional(), \"on\": z.boolean().optional(), \"x\": z.json().optional() })])),\n"
        ),
        "{zod}"
    );
    let out = generated("jt_node_enum", ELEMENTS, "[]", "NOT_FOUND");
    assert!(
        file(&out, "node/f.ts").contains(
            "ReadonlyArray<{ readonly feature: string; readonly flag?: \"Yes\" | \"No\" }>"
        )
    );
    assert!(file(&out, "node/s.ts").contains("\"flag\": z.enum([\"Yes\", \"No\"]).optional()"));
}

#[test]
fn test_other_bindings_document_the_shape() {
    let out = generated("jt_docs", ELEMENTS, "[]", "NOT_FOUND");
    let shape = "[{feature: string, flag?: \"Yes\"|\"No\"}]";
    for (suffix, want) in [
        ("cpp/api.hpp", format!("Shape: `{shape}`.")),
        ("cs/Api.g.cs", format!("Shape: `{shape}`.")),
        (
            "java/Api.java",
            "Shape: <code>[{feature: string, flag?:".to_string(),
        ),
        (
            "py/generated.py",
            "Shape: ``[{feature: string, flag?:".to_string(),
        ),
        (
            "py/generated.pyi",
            "Shape: ``[{feature: string, flag?:".to_string(),
        ),
    ] {
        let text = file(&out, suffix);
        assert!(text.contains(&want), "{suffix}: {want}\n{text}");
    }
    // `any` adds no shape line.
    let out = generated("jt_docs_any", "any", "[]", "NOT_FOUND");
    assert!(!file(&out, "cpp/api.hpp").contains("Shape:"));
}

/// The REAL manifest: every `json` arg declares its shape (`any` = free-form
/// JSON, explicitly), and the structured ones are the expected descriptors.
#[test]
fn test_real_manifest_types_every_json_arg() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap();
    let inputs = sz_configtool_codegen::load::load(
        root,
        Path::new(sz_configtool_codegen::DEFAULT_PROJECT_FILE),
    )
    .expect("real manifest loads");
    let mut seen = Vec::new();
    for f in &inputs.functions {
        for a in &f.args {
            let ty = a.json_type.as_ref();
            assert_eq!(ty.is_some(), a.ty == ArgType::Json, "{}.{}", f.name, a.name);
            if let Some(ty) = ty {
                seen.push(format!("{}.{} = {}", f.name, a.name, ty.describe()));
            }
        }
    }
    let want = [
        "add_expression_call.element_list = [{element: string, required: string, feature?: string}]",
        "add_config_section_field.field_value = any",
        "add_fragment.fragment_config = {ERFRAG_CODE: string, ERFRAG_SOURCE: string, ERFRAG_ID?: int|null, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}",
        "add_search_profile.elements = [{feature: string, flag: \"Yes\"|\"No\"|\"Y\"|\"N\"}]",
        "add_rule.rule_config = {ERRULE_CODE: string, QUAL_ERFRAG_CODE: string, DISQ_ERFRAG_CODE?: string|null, RESOLVE?: string|null, RELATE?: string|null, RTYPE_ID?: int|null, ERRULE_TIER?: int|null, ERRULE_ID?: int|null}",
        "set_setting.value = any",
        "set_system_parameter.parameter_value = any",
    ];
    for w in want {
        assert!(seen.iter().any(|s| s == w), "missing {w} in {seen:#?}");
    }
    assert!(
        seen.iter().any(|s| s.starts_with(
            "add_feature.element_list = [string | {element?: string|null, ELEMENT?: string|null,"
        )),
        "{seen:#?}"
    );
    assert_eq!(seen.len(), 8, "{seen:#?}");
}

#[test]
fn test_nullable_renders_and_fits_null() {
    let ty = "{array: {object: {element: string, \"tier?\": {nullable: int}}}}";
    let out = generated("jt_nullable", ty, "[{element: E, tier: null}]", "NOT_FOUND");
    let conf: serde_json::Value = serde_json::from_str(file(&out, "conformance.json")).unwrap();
    assert!(
        conf["cases"][0]["steps"][0].get("wire_only").is_none(),
        "null fits nullable"
    );
    let ts = file(&out, "node/f.ts");
    assert!(
        ts.contains(
            "ReadonlyArray<{ readonly element: string; readonly tier?: number | bigint | null }>"
        ),
        "{ts}"
    );
    assert!(ts.contains("\"tier?\": { nullable: \"int\" }"), "{ts}");
    assert!(
        ts.contains("Shape: `[{element: string, tier?: int|null}]`."),
        "{ts}"
    );
    assert!(
        file(&out, "node/s.ts")
            .contains("\"tier\": z.union([z.int(), z.bigint()]).nullable().optional()"),
        "{}",
        file(&out, "node/s.ts")
    );
    let manifest: serde_json::Value = serde_json::from_str(file(&out, "manifest.json")).unwrap();
    assert_eq!(
        manifest["functions"][0]["args"][1]["json_type"]["array"]["object"]["tier?"],
        serde_json::json!({"nullable": "int"})
    );
    // A required (non-nullable) key does not fit null.
    let out = generated("jt_nullable_req", ty, "[{element: null}]", "INVALID_INPUT");
    let conf: serde_json::Value = serde_json::from_str(file(&out, "conformance.json")).unwrap();
    assert_eq!(conf["cases"][0]["steps"][0]["wire_only"], true);
    for (name, bad, want) in [
        (
            "jt_nullable_any",
            "{nullable: any}",
            "nullable cannot wrap any or nullable",
        ),
        (
            "jt_nullable_twice",
            "{nullable: {nullable: int}}",
            "nullable cannot wrap any or nullable",
        ),
        (
            "jt_nullable_in_one_of",
            "{one_of: [string, {nullable: int}]}",
            "one_of cannot contain any, nullable or a nested one_of",
        ),
    ] {
        let err = rejected(name, bad);
        assert!(err.contains(want), "{name}: {err}");
    }
}
