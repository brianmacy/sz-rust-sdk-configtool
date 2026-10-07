//! TypeScript/Node wrapper generator (owner: the node binding).
//!
//! Renders, from the manifest, everything under `bindings/node/**/generated`
//! (see `bindings/CONTRACT.md` and `bindings/node/README.md`):
//!
//! * `ts/generated/functions.ts` — one typed camelCase function per
//!   implemented manifest function (`status: not_implemented` is skipped), an
//!   options-object interface per function, TSDoc from `doc`/`semantics`/
//!   `notes`/`errors`. Every function calls the native `invoke` seam through
//!   `ts/runtime.ts`.
//! * `ts/generated/reason-codes.ts` — the wire error taxonomy.
//! * `trpc/src/generated/schemas.ts` — one Zod input schema per function.
//! * `trpc/src/generated/router.ts` — one tRPC procedure per function.
//! * `test/generated/paths.ts` — workspace-relative paths (from
//!   `project.yaml`) the npm test suite reads the manifest/conformance from.
//!
//! Typed TS mapping: `str`→`string`, `int`→`number | bigint` (a `bigint`
//! carries the full i64 range exactly), `bool`→`boolean`,
//! `json`→`JsonValue`, `str_list`→`readonly string[]`, `int_or_str`→
//! `number | bigint | string` (a call id or a feature code); optional→`name?: T`;
//! tri-state→`name?: T | null` (undefined = leave, null = clear, value = set);
//! `required: true`→ a required property even though Rust takes `Option`.
//!
//! Results (CONTRACT table): `json` → JSON text; every config-changing
//! function → the config text; a `config_and_json` function also gets a
//! companion `<fn>Result` (same options) → the record JSON text, or for
//! `tuple_names` a `<Fn>Record` whose camelCase fields are each member's JSON
//! TEXT. The companion's tRPC procedure is a query (it returns no config).

use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};

use crate::Generated;
use crate::emit::GENERATED_BANNER;
use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// Generated node files (workspace-relative paths), deterministic.
pub fn generate(inputs: &Inputs) -> Vec<Generated> {
    let typed: Vec<&Function> = inputs
        .functions
        .iter()
        .filter(|f| f.status == Status::Implemented)
        .collect();
    let out = &inputs.project.bindings.node;
    vec![
        output(&out.functions, functions_ts(&typed)),
        output(
            &out.reason_codes,
            reason_codes_ts(&inputs.project.reason_codes),
        ),
        output(&out.trpc_schemas, schemas_ts(&typed)),
        output(&out.trpc_router, router_ts(&typed)),
        output(&out.test_paths, paths_ts(inputs)),
    ]
}

fn output(path: &str, contents: String) -> Generated {
    Generated {
        path: PathBuf::from(path),
        contents,
    }
}

// ---------------------------------------------------------------- naming

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .map(|c| c.to_ascii_uppercase().to_string() + chars.as_str())
        .unwrap_or_default()
}

/// `add_data_source` → `addDataSource` (split on `_`, no acronym casing).
pub fn camel(snake: &str) -> String {
    let mut words = snake.split('_').filter(|w| !w.is_empty());
    let first = words.next().unwrap_or_default().to_string();
    first + &words.map(capitalize).collect::<String>()
}

/// `add_data_source` → `AddDataSource`.
pub fn pascal(snake: &str) -> String {
    snake.split('_').map(capitalize).collect()
}

// ---------------------------------------------------------------- docs

/// A TSDoc block from paragraphs (blank-line separated), `*/` neutralized.
fn doc_block(paragraphs: &[String], indent: &str) -> String {
    let mut out = format!("{indent}/**\n");
    for (i, para) in paragraphs.iter().enumerate() {
        if i > 0 {
            let _ = writeln!(out, "{indent} *");
        }
        for line in para.trim_end().lines() {
            let line = line.replace("*/", "*\\/");
            let sep = if line.is_empty() { "" } else { " " };
            let _ = writeln!(out, "{indent} *{sep}{line}");
        }
    }
    let _ = writeln!(out, "{indent} */");
    out
}

/// Typed parameter is required: not optional, or `required: true`.
fn is_required(a: &Arg) -> bool {
    !a.is_optional() || a.required
}

fn arg_doc(a: &Arg) -> Vec<String> {
    let mut paras = Vec::new();
    if let Some(s) = &a.semantics {
        paras.push(s.clone());
    }
    if a.tristate {
        paras.push(
            "Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set."
                .to_string(),
        );
    }
    if a.optional && a.required {
        paras.push("Required: the library rejects an absent value (MISSING_FIELD).".to_string());
    }
    if let Some(d) = &a.default {
        paras.push(format!(
            "Library default when omitted: `{d}` (applied by the library, not this binding)."
        ));
    }
    paras.push(format!("Wire name: `{}`.", a.name));
    paras
}

/// Which typed function of a manifest function is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// `<fn>`: the config for config-changing functions, else the result.
    Primary,
    /// `<fn>Result` of a `config_and_json` function: the record only.
    Companion,
}

/// The typed functions of `f`: the primary, plus the companion when it has one.
fn roles(f: &Function) -> &'static [Role] {
    if f.companion().is_some() {
        &[Role::Primary, Role::Companion]
    } else {
        &[Role::Primary]
    }
}

/// The camelCase TS name of `f`'s typed function in `role`.
fn typed_name(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => camel(&f.name),
        Role::Companion => camel(&f.companion().unwrap_or_default()),
    }
}

/// Whether the typed function in `role` returns the named record.
fn returns_record(f: &Function, role: Role) -> bool {
    !f.tuple_names.is_empty() && (f.returns == Returns::Json || role == Role::Companion)
}

fn returns_doc(f: &Function, role: Role) -> String {
    if returns_record(f, role) {
        return format!(
            "@returns A record of the named result values ({}); each value is JSON text.",
            tuple_field_list(f)
        );
    }
    let text = match (f.returns, role) {
        (Returns::Config, _) => {
            "The modified configuration JSON text (opaque, byte-exact).".to_string()
        }
        (Returns::ConfigAndJson, Role::Primary) => format!(
            "The modified configuration JSON text (opaque, byte-exact). {{@link {}}} (same \
             options) returns the record this operation produces.",
            typed_name(f, Role::Companion)
        ),
        (Returns::ConfigAndJson, Role::Companion) => {
            "The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`)."
                .to_string()
        }
        (Returns::Json, _) => "The result as JSON text (not parsed; use `JSON.parse`).".to_string(),
        (Returns::Int, _) => "The integer result.".to_string(),
        (Returns::Unit, _) => "Nothing; throws on failure.".to_string(),
    };
    format!("@returns {text}")
}

fn tuple_field_list(f: &Function) -> String {
    let names: Vec<String> = f
        .tuple_names
        .iter()
        .map(|n| format!("`{}`", camel(n)))
        .collect();
    names.join(", ")
}

/// The summary paragraph of `f`'s typed function in `role`.
fn summary(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => f.doc.clone(),
        Role::Companion => format!(
            "The record (row / ids) of {{@link {}}}: same options and operation, but returns \
             the record instead of the configuration. Operation: {}",
            camel(&f.name),
            f.doc
        ),
    }
}

fn function_doc(f: &Function, role: Role) -> String {
    let mut paras = vec![summary(f, role)];
    if let Some(n) = &f.notes {
        paras.push(format!("@remarks\n{n}"));
    }
    paras.push(format!("Wire name: `{}` (group `{}`).", f.name, f.group));
    let mut tags = vec![
        "@param config - The configuration JSON text (opaque; never parsed by this binding)."
            .to_string(),
    ];
    if !f.args.is_empty() {
        tags.push(format!(
            "@param options - Arguments; see {{@link {}}}.",
            options_name(f)
        ));
    }
    tags.push(returns_doc(f, role));
    let codes = if f.errors.is_empty() {
        "none from the library".to_string()
    } else {
        f.errors.join(", ")
    };
    tags.push(format!(
        "@throws {{@link SzConfigToolError}} `code` one of: {codes}; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL."
    ));
    paras.push(tags.join("\n"));
    doc_block(&paras, "")
}

// ---------------------------------------------------------------- typed functions

fn options_name(f: &Function) -> String {
    format!("{}Options", pascal(&f.name))
}

fn result_name(f: &Function) -> String {
    format!("{}Record", pascal(&f.name))
}

fn ts_type(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str => "string",
        ArgType::Int => "number | bigint",
        ArgType::Bool => "boolean",
        ArgType::Json => "rt.JsonValue",
        ArgType::IntOrStr => "number | bigint | string",
        ArgType::StrList => "readonly string[]",
    }
}

/// One options-interface property, e.g. `readonly id?: number;`.
fn ts_property(a: &Arg) -> String {
    let opt = if is_required(a) { "" } else { "?" };
    let null = if a.tristate { " | null" } else { "" };
    format!(
        "  readonly {}{opt}: {}{null};\n",
        camel(&a.name),
        ts_type(a.ty)
    )
}

fn options_interface(f: &Function) -> String {
    let mut out = doc_block(&[format!("Arguments of {{@link {}}}.", camel(&f.name))], "");
    let _ = writeln!(out, "export interface {} {{", options_name(f));
    for a in &f.args {
        out.push_str(&doc_block(&arg_doc(a), "  "));
        out.push_str(&ts_property(a));
    }
    out.push_str("}\n\n");
    out
}

fn result_interface(f: &Function) -> String {
    let role = *roles(f).last().unwrap_or(&Role::Primary);
    let mut out = doc_block(
        &[format!(
            "Named result of {{@link {}}}.",
            typed_name(f, role)
        )],
        "",
    );
    let _ = writeln!(out, "export interface {} {{", result_name(f));
    for n in &f.tuple_names {
        let _ = writeln!(
            out,
            "  /** JSON text of result member `{n}`. */\n  readonly {}: string;",
            camel(n)
        );
    }
    out.push_str("}\n\n");
    out
}

/// `(TS return type, runtime call prefix)`.
fn return_shape(f: &Function, role: Role) -> (String, String) {
    if returns_record(f, role) {
        let r = result_name(f);
        return (r.clone(), format!("rt.callNamed<{r}>"));
    }
    let (ty, helper) = match (f.returns, role) {
        (Returns::Config, _) | (Returns::ConfigAndJson, Role::Primary) => ("string", "callConfig"),
        (Returns::Json, _) | (Returns::ConfigAndJson, Role::Companion) => ("string", "callJson"),
        (Returns::Int, _) => ("number", "callInt"),
        (Returns::Unit, _) => ("void", "callUnit"),
    };
    (ty.to_string(), format!("rt.{helper}"))
}

/// The wire args object literal (`{ snake: options.camel, ... }`).
fn wire_args(f: &Function, indent: &str) -> String {
    if f.args.is_empty() {
        return "{}".to_string();
    }
    let mut out = "{\n".to_string();
    for a in &f.args {
        let _ = writeln!(out, "{indent}  {}: options.{},", a.name, camel(&a.name));
    }
    out.push_str(indent);
    out.push('}');
    out
}

fn tuple_pairs(f: &Function) -> String {
    let pairs: Vec<String> = f
        .tuple_names
        .iter()
        .map(|n| format!("[\"{n}\", \"{}\"]", camel(n)))
        .collect();
    format!("[{}]", pairs.join(", "))
}

fn options_param(f: &Function) -> String {
    match (f.args.is_empty(), f.args.iter().any(is_required)) {
        (true, _) => String::new(),
        (false, true) => format!(", options: {}", options_name(f)),
        (false, false) => format!(", options: {} = {{}}", options_name(f)),
    }
}

fn typed_function(f: &Function) -> String {
    let mut out = String::new();
    if !f.args.is_empty() {
        out.push_str(&options_interface(f));
    }
    if !f.tuple_names.is_empty() {
        out.push_str(&result_interface(f));
    }
    for role in roles(f) {
        out.push_str(&typed_fn(f, *role));
    }
    out
}

/// One exported function: `f`'s typed function in `role`.
fn typed_fn(f: &Function, role: Role) -> String {
    let (ret, call) = return_shape(f, role);
    let tail = if returns_record(f, role) {
        format!(", {}", tuple_pairs(f))
    } else {
        String::new()
    };
    let keyword = if f.returns == Returns::Unit {
        ""
    } else {
        "return "
    };
    let mut out = function_doc(f, role);
    let _ = write!(
        out,
        "export function {name}(config: string{opts}): {ret} {{\n  {keyword}{call}(\"{wire}\", config, {args}{tail});\n}}\n\n",
        name = typed_name(f, role),
        opts = options_param(f),
        wire = f.name,
        args = wire_args(f, "  "),
    );
    out
}

fn functions_ts(typed: &[&Function]) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n//\n// Typed wrappers over the native `invoke` seam; see bindings/CONTRACT.md.\n\n\
         import * as rt from \"../runtime.js\";\n\n"
    );
    for f in typed {
        out.push_str(&typed_function(f));
    }
    let names: Vec<String> = typed.iter().map(|f| format!("  \"{}\",", f.name)).collect();
    let _ = write!(
        out,
        "/** Wire names of every typed function above, in manifest order. */\n\
         export const TYPED_FUNCTION_NAMES: readonly string[] = [\n{}\n];\n",
        names.join("\n")
    );
    out
}

/// The type alias comes first so the file ends with emitted JavaScript: Node's
/// source-mapped coverage cannot attribute type-only lines after the last
/// mapping and reports them as uncovered.
fn reason_codes_ts(codes: &[String]) -> String {
    let list: Vec<String> = codes.iter().map(|c| format!("  \"{c}\",")).collect();
    format!(
        "// {GENERATED_BANNER}\n\n\
         /** One wire reason code. */\n\
         export type ReasonCode = (typeof REASON_CODES)[number];\n\n\
         /** The complete wire error taxonomy (`project.yaml` `reason_codes`). */\n\
         export const REASON_CODES = [\n{}\n] as const;\n",
        list.join("\n")
    )
}

// ---------------------------------------------------------------- tRPC

fn zod_type(a: &Arg) -> String {
    let base = match a.ty {
        ArgType::Str => "z.string()",
        ArgType::Int => "z.union([z.int(), z.bigint()])",
        ArgType::Bool => "z.boolean()",
        ArgType::Json => "z.json()",
        ArgType::IntOrStr => "z.union([z.int(), z.bigint(), z.string()])",
        ArgType::StrList => "z.array(z.string())",
    };
    let modifier = match (a.tristate, is_required(a)) {
        (true, _) => ".nullable().optional()",
        (false, false) => ".optional()",
        (false, true) => "",
    };
    format!("{base}{modifier}")
}

fn schema_name(f: &Function) -> String {
    format!("{}Input", camel(&f.name))
}

fn schemas_ts(typed: &[&Function]) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n//\n// Zod input schemas: `config` (opaque text) plus the function's arguments\n\
         // (camelCase). Strict: unknown keys are rejected, like the wire.\n\n\
         import {{ z }} from \"zod\";\n\n"
    );
    for f in typed {
        let _ = writeln!(out, "/** Input of `{}`. */", camel(&f.name));
        let _ = writeln!(out, "export const {} = z.strictObject({{", schema_name(f));
        out.push_str("  config: z.string(),\n");
        for a in &f.args {
            let _ = writeln!(out, "  {}: {},", camel(&a.name), zod_type(a));
        }
        out.push_str("});\n\n");
    }
    out
}

/// Procedures returning a config are mutations; the rest (incl. companions,
/// which return only the record) are queries.
fn procedure_kind(f: &Function, role: Role) -> &'static str {
    match (f.returns, role) {
        (Returns::Config, _) | (Returns::ConfigAndJson, Role::Primary) => "mutation",
        _ => "query",
    }
}

fn procedure(f: &Function, role: Role) -> String {
    let name = typed_name(f, role);
    let body = if f.args.is_empty() {
        format!("szCall(() => api.{name}(input.config))")
    } else {
        format!(
            "szCall(() => {{\n        const {{ config, ...options }} = input;\n        \
             return api.{name}(config, options);\n      }})"
        )
    };
    format!(
        "{doc}  {name}: t.procedure\n    .input(schemas.{schema})\n    .{kind}(({{ input }}) =>\n      {body},\n    ),\n",
        doc = doc_block(&[summary(f, role)], "  "),
        schema = schema_name(f),
        kind = procedure_kind(f, role),
    )
}

fn router_ts(typed: &[&Function]) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n//\n// One procedure per typed function. Functions returning a config are\n\
         // mutations; the others (incl. the `<fn>Result` companions, which return\n\
         // only the record) are queries (send them with POST: the config is\n\
         // ~300KB, far too large for a GET URL).\n\n\
         import * as api from \"sz-configtool\";\n\n\
         import {{ szCall }} from \"../sz-call.js\";\n\
         import {{ t }} from \"../trpc.js\";\n\
         import * as schemas from \"./schemas.js\";\n\n\
         /** Type of {{@link configToolRouter}}, for typed clients. */\n\
         export type ConfigToolRouter = typeof configToolRouter;\n\n\
         /** The configuration-tool router. */\n\
         export const configToolRouter = t.router({{\n"
    );
    for f in typed {
        for role in roles(f) {
            out.push_str(&procedure(f, *role));
        }
    }
    // Ends with emitted JS: see reason_codes_ts (source-mapped coverage).
    out.push_str("});\n");
    out
}

// ---------------------------------------------------------------- test paths

/// `bindings/node` → `../..`.
fn up_to_root(dir: &Path) -> PathBuf {
    dir.components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .map(|_| Component::ParentDir)
        .collect()
}

fn paths_ts(inputs: &Inputs) -> String {
    let p = &inputs.project.paths;
    let root = up_to_root(Path::new(&inputs.project.bindings.node.package_dir));
    format!(
        "// {GENERATED_BANNER}\n\n\
         /** Workspace root, relative to the npm package directory. */\n\
         export const WORKSPACE_ROOT = \"{}\";\n\
         /** Generated manifest JSON (workspace-relative). */\n\
         export const MANIFEST_JSON = \"{}\";\n\
         /** Generated conformance cases (workspace-relative). */\n\
         export const CONFORMANCE_JSON = \"{}\";\n",
        root.to_string_lossy().replace('\\', "/"),
        p.manifest_json_out,
        p.conformance_json_out,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Excluded, Paths, Project};

    fn arg(name: &str, ty: ArgType) -> Arg {
        Arg {
            name: name.into(),
            ty,
            optional: false,
            tristate: false,
            required: false,
            default: None,
            semantics: None,
            field: None,
            positional: false,
            owned: false,
            rust_convert: None,
        }
    }

    fn func(name: &str, args: Vec<Arg>, returns: Returns) -> Function {
        Function {
            name: name.into(),
            group: "g".into(),
            doc: "Does it.".into(),
            rust: "m::f".into(),
            rust_params_struct: None,
            c_symbol: None,
            c_aliases: vec![],
            args,
            returns,
            tuple_names: vec![],
            errors: vec!["NOT_FOUND".into()],
            notes: None,
            c_notes: None,
            status: Status::Implemented,
            requires_options: false,
        }
    }

    fn inputs(functions: Vec<Function>) -> Inputs {
        Inputs {
            project: Project {
                root_crate: "lib".into(),
                paths: Paths {
                    root_src: "src".into(),
                    c_header: "h.h".into(),
                    fixture: "f.json".into(),
                    manifest_dir: "m".into(),
                    conformance_dir: "c".into(),
                    excluded: "e.yaml".into(),
                    dispatch_out: "d.rs".into(),
                    manifest_json_out: "gen/manifest.json".into(),
                    conformance_json_out: "gen/conformance.json".into(),
                },
                reason_codes: vec!["NOT_FOUND".into(), "INTERNAL".into()],
                bindings: crate::model::Bindings::from_real_project(),
            },
            functions,
            excluded: Excluded {
                enforce_complete: false,
                entries: vec![],
            },
            cases: vec![],
        }
    }

    #[test]
    fn test_naming_splits_on_underscore_without_acronyms() {
        assert_eq!(camel("add_data_source"), "addDataSource");
        assert_eq!(camel("get_ftype_id"), "getFtypeId");
        assert_eq!(camel("set_cfunc_rtnval_2"), "setCfuncRtnval2");
        assert_eq!(camel("list"), "list");
        assert_eq!(pascal("add_data_source"), "AddDataSource");
        assert_eq!(pascal(""), "");
    }

    #[test]
    fn test_property_modes() {
        let req = arg("code", ArgType::Str);
        assert_eq!(ts_property(&req), "  readonly code: string;\n");
        let mut opt = arg("id", ArgType::Int);
        opt.optional = true;
        assert_eq!(ts_property(&opt), "  readonly id?: number | bigint;\n");
        let mut must = arg("connect_str", ArgType::Str);
        must.optional = true;
        must.required = true;
        assert_eq!(ts_property(&must), "  readonly connectStr: string;\n");
        let mut tri = arg("source", ArgType::Str);
        tri.tristate = true;
        assert_eq!(ts_property(&tri), "  readonly source?: string | null;\n");
        assert_eq!(
            ts_property(&arg("names", ArgType::StrList)),
            "  readonly names: readonly string[];\n"
        );
        assert_eq!(
            ts_property(&arg("v", ArgType::Json)),
            "  readonly v: rt.JsonValue;\n"
        );
        assert_eq!(
            ts_property(&arg("b", ArgType::Bool)),
            "  readonly b: boolean;\n"
        );
        assert_eq!(
            ts_property(&arg("call", ArgType::IntOrStr)),
            "  readonly call: number | bigint | string;\n"
        );
    }

    #[test]
    fn test_zod_modes() {
        let mut tri = arg("source", ArgType::Str);
        tri.tristate = true;
        assert_eq!(zod_type(&tri), "z.string().nullable().optional()");
        let mut opt = arg("id", ArgType::Int);
        opt.optional = true;
        assert_eq!(zod_type(&opt), "z.union([z.int(), z.bigint()]).optional()");
        opt.required = true;
        assert_eq!(zod_type(&opt), "z.union([z.int(), z.bigint()])");
        assert_eq!(zod_type(&arg("v", ArgType::Json)), "z.json()");
        assert_eq!(zod_type(&arg("b", ArgType::Bool)), "z.boolean()");
        assert_eq!(zod_type(&arg("l", ArgType::StrList)), "z.array(z.string())");
        assert_eq!(
            zod_type(&arg("call", ArgType::IntOrStr)),
            "z.union([z.int(), z.bigint(), z.string()])"
        );
    }

    #[test]
    fn test_typed_function_maps_args_to_wire_names() {
        let mut id = arg("dsrc_id", ArgType::Int);
        id.optional = true;
        let f = func(
            "add_data_source",
            vec![arg("code", ArgType::Str), id],
            Returns::Config,
        );
        let ts = typed_function(&f);
        assert!(
            ts.contains("export interface AddDataSourceOptions {"),
            "{ts}"
        );
        assert!(
            ts.contains("export function addDataSource(config: string, options: AddDataSourceOptions): string {"),
            "{ts}"
        );
        assert!(
            ts.contains("return rt.callConfig(\"add_data_source\", config, {"),
            "{ts}"
        );
        assert!(ts.contains("    dsrc_id: options.dsrcId,\n"), "{ts}");
        assert!(
            ts.contains("@throws {@link SzConfigToolError} `code` one of: NOT_FOUND;"),
            "{ts}"
        );
    }

    #[test]
    fn test_all_optional_options_default_to_empty() {
        let mut a = arg("filter", ArgType::Str);
        a.optional = true;
        let f = func("list_things", vec![a], Returns::Json);
        assert_eq!(options_param(&f), ", options: ListThingsOptions = {}");
        let none = func("list_all", vec![], Returns::Json);
        let ts = typed_function(&none);
        assert!(
            ts.contains("export function listAll(config: string): string {"),
            "{ts}"
        );
        assert!(
            ts.contains("rt.callJson(\"list_all\", config, {});"),
            "{ts}"
        );
    }

    #[test]
    fn test_return_shapes() {
        let shape = |r| return_shape(&func("f", vec![], r), Role::Primary);
        assert_eq!(shape(Returns::Config).0, "string");
        assert_eq!(shape(Returns::Json).0, "string");
        assert_eq!(
            shape(Returns::ConfigAndJson),
            ("string".into(), "rt.callConfig".into())
        );
        assert_eq!(
            return_shape(&func("f", vec![], Returns::ConfigAndJson), Role::Companion),
            ("string".into(), "rt.callJson".into())
        );
        let pair = typed_function(&func("add_it", vec![], Returns::ConfigAndJson));
        assert!(
            pair.contains(
                "export function addIt(config: string): string {\n  return rt.callConfig(\"add_it\""
            ),
            "{pair}"
        );
        assert!(
            pair.contains("export function addItResult(config: string): string {\n  return rt.callJson(\"add_it\""),
            "{pair}"
        );
        assert!(
            pair.contains("{@link addItResult} (same options)"),
            "{pair}"
        );
        assert!(
            pair.contains("The record (row / ids) of {@link addIt}"),
            "{pair}"
        );
        assert_eq!(shape(Returns::Int).0, "number");
        assert_eq!(shape(Returns::Unit).0, "void");
        let unit = typed_function(&func("check", vec![], Returns::Unit));
        assert!(
            unit.contains("  rt.callUnit(\"check\", config, {});"),
            "{unit}"
        );
    }

    #[test]
    fn test_tuple_names_become_named_record() {
        let mut f = func("set_plan", vec![], Returns::ConfigAndJson);
        f.tuple_names = vec!["plan_id".into(), "was_created".into()];
        let ts = typed_function(&f);
        assert!(
            ts.contains("Named result of {@link setPlanResult}."),
            "{ts}"
        );
        assert!(ts.contains("export interface SetPlanRecord {"), "{ts}");
        assert!(!ts.contains("readonly config: string;"), "{ts}");
        assert!(ts.contains("readonly wasCreated: string;"), "{ts}");
        assert!(ts.contains("each value is JSON text"), "{ts}");
        assert!(
            ts.contains("export function setPlan(config: string): string {\n  return rt.callConfig(\"set_plan\", config, {});"),
            "{ts}"
        );
        assert!(
            ts.contains("export function setPlanResult(config: string): SetPlanRecord {\n  return rt.callNamed<SetPlanRecord>(\"set_plan\", config, {}, [[\"plan_id\", \"planId\"], [\"was_created\", \"wasCreated\"]]);"),
            "{ts}"
        );
        let mut j = func("verify", vec![], Returns::Json);
        j.tuple_names = vec!["current_version".into(), "matches".into()];
        assert!(!result_interface(&j).contains("config"));
        assert!(result_interface(&j).contains("Named result of {@link verify}."));
    }

    #[test]
    fn test_docs_carry_semantics_and_escape_comment_end() {
        let mut a = arg("level", ArgType::Str);
        a.optional = true;
        a.semantics = Some("Uppercased. */ not a terminator".into());
        a.default = Some(serde_json::json!("Remember"));
        let mut f = func("add", vec![a], Returns::Config);
        f.notes = Some("Note line 1\nline 2".into());
        let ts = typed_function(&f);
        assert!(ts.contains("Uppercased. *\\/ not a terminator"), "{ts}");
        assert!(
            ts.contains("Library default when omitted: `\"Remember\"`"),
            "{ts}"
        );
        assert!(
            ts.contains(" * @remarks\n * Note line 1\n * line 2\n"),
            "{ts}"
        );
        assert!(!ts.contains("*/ not"), "{ts}");
    }

    #[test]
    fn test_router_kinds_and_bodies() {
        let mut code = arg("code", ArgType::Str);
        code.optional = true;
        let add = func("add_x", vec![code], Returns::Config);
        let list = func("list_x", vec![], Returns::Json);
        let pair = func("add_y", vec![], Returns::ConfigAndJson);
        let r = router_ts(&[&add, &list, &pair]);
        assert!(
            r.contains("  addY: t.procedure\n    .input(schemas.addYInput)\n    .mutation("),
            "{r}"
        );
        assert!(
            r.contains("  addYResult: t.procedure\n    .input(schemas.addYInput)\n    .query(({ input }) =>\n      szCall(() => api.addYResult(input.config)),"),
            "{r}"
        );
        assert!(
            r.contains("  addX: t.procedure\n    .input(schemas.addXInput)\n    .mutation("),
            "{r}"
        );
        assert!(r.contains("return api.addX(config, options);"), "{r}");
        assert!(
            r.contains("    .query(({ input }) =>\n      szCall(() => api.listX(input.config)),"),
            "{r}"
        );
        assert!(
            r.contains("export type ConfigToolRouter = typeof configToolRouter;\n\n/** The configuration-tool router. */"),
            "{r}"
        );
        assert!(r.ends_with("    ),\n});\n"), "ends with emitted JS: {r}");
        let s = schemas_ts(&[&add]);
        assert!(s.contains("export const addXInput = z.strictObject({\n  config: z.string(),\n  code: z.string().optional(),\n});"), "{s}");
    }

    #[test]
    fn test_generate_skips_not_implemented_and_is_deterministic() {
        let mut stub = func("score_it", vec![], Returns::Json);
        stub.status = Status::NotImplemented;
        let real = func("list_x", vec![], Returns::Json);
        let inp = inputs(vec![stub, real]);
        let a = generate(&inp);
        assert_eq!(a, generate(&inp));
        for g in &a {
            assert!(
                g.path.starts_with(&inp.project.bindings.node.package_dir),
                "{:?}",
                g.path
            );
            assert!(
                g.path.to_string_lossy().contains("generated"),
                "{:?}",
                g.path
            );
            assert!(g.contents.ends_with('\n'), "{:?}", g.path);
            assert!(!g.contents.contains("score_it"), "{:?}", g.path);
            assert!(!g.contents.contains("scoreIt"), "{:?}", g.path);
        }
        let funcs = &a[0].contents;
        assert!(funcs.contains("export function listX("), "{funcs}");
        assert!(funcs.contains("TYPED_FUNCTION_NAMES: readonly string[] = [\n  \"list_x\",\n];"));
        let codes = &a[1].contents;
        assert!(
            codes.ends_with("  \"NOT_FOUND\",\n  \"INTERNAL\",\n] as const;\n"),
            "ends with emitted JS (source-mapped coverage): {codes}"
        );
        assert!(
            codes.contains("export type ReasonCode = (typeof REASON_CODES)[number];"),
            "{codes}"
        );
        let paths = &a[4].contents;
        assert!(paths.contains("WORKSPACE_ROOT = \"../..\""), "{paths}");
        assert!(
            paths.contains("CONFORMANCE_JSON = \"gen/conformance.json\""),
            "{paths}"
        );
    }
}
