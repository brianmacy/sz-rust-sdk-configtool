//! C# (.NET) wrapper generator (owner: the csharp binding).
//!
//! Emits, under `bindings/csharp/src/Sz.ConfigTool/Generated/`:
//! * `SzConfigTool.g.cs` — the typed `SzConfigTool` static partial class (one
//!   PascalCase method per implemented manifest function, camelCase args, XML
//!   doc from the manifest) plus `<Name>Record` records for `tuple_names`
//!   functions. Every config-changing method returns the config text; a
//!   `config_and_json` function also gets a companion `<Name>Result` (same
//!   parameters and overloads) returning the record JSON text (or the
//!   `<Name>Record`);
//! * `SzConfigToolErrorKind.g.cs` — the error-kind enum from `reason_codes`.
//!
//! Every method calls `SzConfigTool_invoke` through the hand-written
//! `Native.NativeCall` seam; see `bindings/CONTRACT.md`.

use std::fmt::Write as _;
use std::path::PathBuf;

use crate::Generated;
use crate::emit::GENERATED_BANNER;
use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// Name of the implicit first parameter (the configuration JSON).
const CONFIG_PARAM: &str = "configJson";

/// C# reserved keywords (escaped with `@` when used as a parameter name).
const KEYWORDS: &[&str] = &[
    "abstract",
    "as",
    "base",
    "bool",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "checked",
    "class",
    "const",
    "continue",
    "decimal",
    "default",
    "delegate",
    "do",
    "double",
    "else",
    "enum",
    "event",
    "explicit",
    "extern",
    "false",
    "finally",
    "fixed",
    "float",
    "for",
    "foreach",
    "goto",
    "if",
    "implicit",
    "in",
    "int",
    "interface",
    "internal",
    "is",
    "lock",
    "long",
    "namespace",
    "new",
    "null",
    "object",
    "operator",
    "out",
    "override",
    "params",
    "private",
    "protected",
    "public",
    "readonly",
    "ref",
    "return",
    "sbyte",
    "sealed",
    "short",
    "sizeof",
    "stackalloc",
    "static",
    "string",
    "struct",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "uint",
    "ulong",
    "unchecked",
    "unsafe",
    "ushort",
    "using",
    "virtual",
    "void",
    "volatile",
    "while",
];

/// Generated C# files.
pub fn generate(inputs: &Inputs) -> Vec<Generated> {
    vec![
        Generated {
            path: PathBuf::from(&inputs.project.bindings.csharp.api),
            contents: api_file(&inputs.functions),
        },
        Generated {
            path: PathBuf::from(&inputs.project.bindings.csharp.error_kinds),
            contents: kind_file(&inputs.project.reason_codes),
        },
    ]
}

/// `add_data_source` -> `AddDataSource`; `JSON_PARSE` -> `JsonParse`.
pub fn pascal(snake: &str) -> String {
    snake
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let lower = p.to_ascii_lowercase();
            let mut chars = lower.chars();
            chars.next().map_or_else(String::new, |c| {
                c.to_ascii_uppercase().to_string() + chars.as_str()
            })
        })
        .collect()
}

/// `default_value` -> `defaultValue`.
pub fn camel(snake: &str) -> String {
    let p = pascal(snake);
    let mut chars = p.chars();
    chars.next().map_or_else(String::new, |c| {
        c.to_ascii_lowercase().to_string() + chars.as_str()
    })
}

/// The C# parameter identifier for an arg (keywords get `@`; never collides
/// with the config parameter).
pub fn param_name(arg_name: &str) -> String {
    let c = camel(arg_name);
    if c == CONFIG_PARAM {
        format!("{c}Arg")
    } else if KEYWORDS.contains(&c.as_str()) {
        format!("@{c}")
    } else {
        c
    }
}

/// Escape text for an XML doc comment.
fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Emit `/// <tag>text</tag>` (multi-line text keeps one `///` per line).
fn doc_tag(out: &mut String, indent: &str, open: &str, close: &str, text: &str) {
    let lines: Vec<String> = text.trim().lines().map(|l| xml(l.trim_end())).collect();
    if lines.len() <= 1 {
        let _ = writeln!(out, "{indent}/// {open}{}{close}", lines.join(""));
        return;
    }
    let _ = writeln!(out, "{indent}/// {open}");
    for l in lines {
        let _ = writeln!(out, "{indent}/// {l}");
    }
    let _ = writeln!(out, "{indent}/// {close}");
}

/// How an arg is passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Required,
    Optional,
    Tristate,
}

fn mode(a: &Arg) -> Mode {
    if a.tristate {
        Mode::Tristate
    } else if a.optional && !a.required {
        Mode::Optional
    } else {
        Mode::Required
    }
}

/// C# value type of a concrete arg type (required form). `int_or_str` never
/// reaches here: each overload substitutes `Int` or `Str` (see [`overloads`]).
fn base_type(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str | ArgType::Json | ArgType::IntOrStr => "string",
        ArgType::Int => "long",
        ArgType::Bool => "bool",
        ArgType::StrList => "System.Collections.Generic.IReadOnlyList<string>",
    }
}

/// `ArgsWriter` method suffix of a concrete arg type.
fn writer_kind(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str | ArgType::IntOrStr => "Str",
        ArgType::Int => "Int",
        ArgType::Bool => "Bool",
        ArgType::Json => "Json",
        ArgType::StrList => "StrList",
    }
}

/// One typed overload of a function: the concrete C# arg type of each
/// manifest arg (manifest order).
type Overload = Vec<ArgType>;

/// Every overload of `f`: the cartesian product of `long` / `string` over its
/// `int_or_str` args (one overload when there are none). The wire value is a
/// JSON integer or string accordingly.
fn overloads(f: &Function) -> Vec<Overload> {
    let mut all: Vec<Overload> = vec![Vec::new()];
    for a in &f.args {
        let choices: &[ArgType] = if a.ty == ArgType::IntOrStr {
            &[ArgType::Int, ArgType::Str]
        } else {
            std::slice::from_ref(&a.ty)
        };
        all = all
            .into_iter()
            .flat_map(|prefix| {
                choices.iter().map(move |&t| {
                    let mut o = prefix.clone();
                    o.push(t);
                    o
                })
            })
            .collect();
    }
    all
}

/// The C# parameter declaration (`type name[ = default]`) for `ty`.
fn param_decl(a: &Arg, ty: ArgType) -> String {
    let name = param_name(&a.name);
    let base = base_type(ty);
    match mode(a) {
        Mode::Required => format!("{base} {name}"),
        Mode::Optional => format!("{base}? {name} = null"),
        Mode::Tristate => format!("FieldUpdate<{base}> {name} = default"),
    }
}

/// The `ArgsWriter` statement for an arg passed as `ty`.
fn writer_stmt(a: &Arg, ty: ArgType) -> String {
    let name = param_name(&a.name);
    let kind = writer_kind(ty);
    let key = &a.name;
    // Reference-typed values need the parameter name for argument exceptions.
    let needs_param = matches!(ty, ArgType::Json | ArgType::StrList)
        || (ty == ArgType::Str && mode(a) == Mode::Required);
    let suffix = if needs_param && mode(a) != Mode::Tristate {
        format!(", nameof({name})")
    } else {
        String::new()
    };
    let method = match mode(a) {
        Mode::Required => kind.to_string(),
        Mode::Optional => format!("Opt{kind}"),
        Mode::Tristate => format!("Tri{}", if ty == ArgType::Int { "Int" } else { "Str" }),
    };
    format!("args.{method}(\"{key}\", {name}{suffix});")
}

/// Indices of `f.args`: required args first (manifest order), then
/// optional/tri-state ones: C# requires optional parameters last.
fn ordered_args(f: &Function) -> Vec<usize> {
    let (req, opt): (Vec<usize>, Vec<usize>) =
        (0..f.args.len()).partition(|&i| mode(&f.args[i]) == Mode::Required);
    req.into_iter().chain(opt).collect()
}

/// Name of the generated result record for a function with `tuple_names`
/// (`json` or `config_and_json`), if any.
fn record_name(f: &Function) -> Option<String> {
    (matches!(f.returns, Returns::ConfigAndJson | Returns::Json) && !f.tuple_names.is_empty())
        .then(|| format!("{}Record", pascal(&f.name)))
}

/// Which typed method of a manifest function is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// `<Name>`: the config for config-changing functions, else the result.
    Primary,
    /// `<Name>Result` of a `config_and_json` function: the record only.
    Companion,
}

/// The typed methods of `f`: the primary, plus the companion when it has one.
fn roles(f: &Function) -> &'static [Role] {
    if f.companion().is_some() {
        &[Role::Primary, Role::Companion]
    } else {
        &[Role::Primary]
    }
}

/// The PascalCase method name of `f` in `role`.
fn method_name(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => pascal(&f.name),
        Role::Companion => pascal(&f.companion().unwrap_or_default()),
    }
}

/// The record the typed method in `role` returns, if any.
fn returned_record(f: &Function, role: Role) -> Option<String> {
    record_name(f).filter(|_| f.returns == Returns::Json || role == Role::Companion)
}

fn return_type(f: &Function, role: Role) -> String {
    if let Some(r) = returned_record(f, role) {
        return r;
    }
    match f.returns {
        Returns::Config | Returns::Json | Returns::ConfigAndJson => "string".into(),
        Returns::Int => "long".into(),
        Returns::Unit => "void".into(),
    }
}

fn returns_doc(f: &Function, role: Role) -> Option<String> {
    let fields: Vec<String> = f.tuple_names.iter().map(|n| pascal(n)).collect();
    let fields = fields.join(", ");
    if returned_record(f, role).is_some() {
        return Some(format!(
            "The named result fields {fields} (each as JSON text)."
        ));
    }
    match (f.returns, role) {
        (Returns::Config, _) => Some("The modified configuration JSON.".into()),
        (Returns::ConfigAndJson, Role::Primary) => Some(format!(
            "The modified configuration JSON. <see cref=\"{}\"/> (same arguments) returns the \
             record this operation produces.",
            method_name(f, Role::Companion)
        )),
        (Returns::ConfigAndJson, Role::Companion) => {
            Some("The record (e.g. the created row or ids), as JSON text.".into())
        }
        (Returns::Json, _) => Some("The result, as JSON text.".into()),
        (Returns::Int, _) => Some("The integer result.".into()),
        (Returns::Unit, _) => None,
    }
}

/// The summary of `f`'s typed method in `role` (XML-escaped by `doc_tag`).
fn summary(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => f.doc.clone(),
        Role::Companion => format!(
            "The record (row / ids) of {}: same arguments and operation, but returns the \
             record instead of the configuration. Operation: {}",
            pascal(&f.name),
            f.doc
        ),
    }
}

fn param_doc(a: &Arg, ty: ArgType) -> String {
    let mut text = match mode(a) {
        Mode::Required if a.required => "Required.".to_string(),
        Mode::Required => String::new(),
        Mode::Optional => "Optional; null omits it.".into(),
        Mode::Tristate => "Tri-state: Leave (default) / Clear / Set.".into(),
    };
    match (a.ty, ty) {
        (ArgType::Json, _) => text.push_str(" Raw JSON text."),
        (ArgType::IntOrStr, ArgType::Int) => {
            text.push_str(" Integer form (sent as a JSON integer).")
        }
        (ArgType::IntOrStr, _) => text.push_str(" String form (sent as a JSON string)."),
        _ => {}
    }
    if let Some(d) = &a.default {
        let _ = write!(text, " Library default when omitted: {d}.");
    }
    if let Some(s) = &a.semantics {
        text.push(' ');
        text.push_str(s.trim());
    }
    if let Some(shape) = a.shape() {
        let _ = write!(text, " Shape: `{shape}`.");
    }
    text.trim().to_string()
}

fn method_doc(out: &mut String, f: &Function, o: &[ArgType], role: Role) {
    let ind = "        ";
    doc_tag(out, ind, "<summary>", "</summary>", &summary(f, role));
    let _ = writeln!(
        out,
        "{ind}/// <param name=\"{CONFIG_PARAM}\">The configuration JSON (opaque; passed byte-exact).</param>"
    );
    for i in ordered_args(f) {
        let a = &f.args[i];
        let open = format!(
            "<param name=\"{}\">",
            param_name(&a.name).trim_start_matches('@')
        );
        doc_tag(out, ind, &open, "</param>", &param_doc(a, o[i]));
    }
    if let Some(r) = returns_doc(f, role) {
        let _ = writeln!(out, "{ind}/// <returns>{r}</returns>");
    }
    let mut remarks = format!("Wire name: <c>{}</c>.", f.name);
    if let Some(n) = &f.notes {
        remarks = format!("{}\n{remarks}", xml(n.trim()));
    }
    let _ = writeln!(out, "{ind}/// <remarks>");
    for l in remarks.lines() {
        let _ = writeln!(out, "{ind}/// {}", l.trim_end());
    }
    let _ = writeln!(out, "{ind}/// </remarks>");
    let errors = if f.errors.is_empty() {
        "wire errors only".to_string()
    } else {
        f.errors.join(", ")
    };
    let _ = writeln!(
        out,
        "{ind}/// <exception cref=\"SzConfigToolException\">Reason codes: {errors} (plus the universal wire errors).</exception>"
    );
}

fn call_expr(f: &Function, args: &str, role: Role) -> String {
    let helper = match (f.returns, role) {
        (Returns::Config, _) => "Config",
        (Returns::Json, _) => "Json",
        (Returns::ConfigAndJson, Role::Primary) => "ConfigAndJsonConfig",
        (Returns::ConfigAndJson, Role::Companion) => "ConfigAndJsonResult",
        // Emitted inline: no runtime helper that only int functions would use.
        (Returns::Int, _) => {
            return format!(
                "long.Parse(NativeCall.Require(NativeCall.Expect(\"int\", \"{name}\", {CONFIG_PARAM}, {args}).Result, \"{name}: envelope without result\"), \
                 System.Globalization.NumberStyles.AllowLeadingSign, System.Globalization.CultureInfo.InvariantCulture)",
                name = f.name
            );
        }
        (Returns::Unit, _) => "Unit",
    };
    format!(
        "NativeCall.{helper}(\"{}\", {CONFIG_PARAM}, {args})",
        f.name
    )
}

/// The method body's return statement(s) for `call`.
fn body(f: &Function, call: &str, role: Role) -> String {
    let Some(r) = returned_record(f, role) else {
        return match f.returns {
            Returns::Unit => format!("{call};"),
            _ => format!("return {call};"),
        };
    };
    let names: Vec<String> = f.tuple_names.iter().map(|n| format!("\"{n}\"")).collect();
    let fields: Vec<String> = (0..f.tuple_names.len())
        .map(|i| format!("m[{i}]"))
        .collect();
    format!(
        "var r = {call};\n            string[] m = NativeCall.Members(r, {});\n            \
         return new {r}({});",
        names.join(", "),
        fields.join(", ")
    )
}

fn method(out: &mut String, f: &Function, o: &[ArgType], role: Role) {
    method_doc(out, f, o, role);
    let mut params = vec![format!("string {CONFIG_PARAM}")];
    params.extend(
        ordered_args(f)
            .into_iter()
            .map(|i| param_decl(&f.args[i], o[i])),
    );
    let _ = writeln!(
        out,
        "        public static {} {}({})\n        {{",
        return_type(f, role),
        method_name(f, role),
        params.join(", ")
    );
    let args = if f.args.is_empty() {
        "\"{}\"".to_string()
    } else {
        let _ = writeln!(out, "            var args = new ArgsWriter();");
        for (a, &ty) in f.args.iter().zip(o) {
            let _ = writeln!(out, "            {}", writer_stmt(a, ty));
        }
        "args.ToJson()".to_string()
    };
    let _ = writeln!(
        out,
        "            {}\n        }}",
        body(f, &call_expr(f, &args, role), role)
    );
}

fn record(out: &mut String, f: &Function, name: &str) {
    let _ = writeln!(
        out,
        "    /// <summary>Named result of <see cref=\"SzConfigTool.{}\"/>; each field is the record member's JSON text (e.g. <c>1001</c>, <c>true</c>, <c>\"4.0.0\"</c>).</summary>",
        method_name(f, *roles(f).last().unwrap_or(&Role::Primary))
    );
    let mut fields = Vec::new();
    for field in &f.tuple_names {
        let p = pascal(field);
        let _ = writeln!(
            out,
            "    /// <param name=\"{p}\">The <c>{field}</c> value, as JSON text.</param>"
        );
        fields.push(format!("string {p}"));
    }
    let _ = writeln!(
        out,
        "    public sealed record {name}({});",
        fields.join(", ")
    );
}

fn header() -> String {
    format!(
        "// <auto-generated>\n// {GENERATED_BANNER}\n// </auto-generated>\n\n#nullable enable\n\n"
    )
}

/// `SzConfigTool.g.cs`.
pub fn api_file(functions: &[Function]) -> String {
    let typed: Vec<&Function> = functions
        .iter()
        .filter(|f| f.status != Status::NotImplemented)
        .collect();
    let mut out = header();
    out.push_str(
        "using Sz.ConfigTool.Json;\nusing Sz.ConfigTool.Native;\n\nnamespace Sz.ConfigTool\n{\n",
    );
    out.push_str("    public static partial class SzConfigTool\n    {\n");
    let mut first = true;
    for f in &typed {
        for role in roles(f) {
            for o in overloads(f) {
                if !first {
                    out.push('\n');
                }
                first = false;
                method(&mut out, f, &o, *role);
            }
        }
    }
    out.push_str("    }\n");
    for f in &typed {
        if let Some(name) = record_name(f) {
            out.push('\n');
            record(&mut out, f, &name);
        }
    }
    out.push_str("}\n");
    out
}

/// `SzConfigToolErrorKind.g.cs`.
pub fn kind_file(reason_codes: &[String]) -> String {
    let mut out = header();
    out.push_str("namespace Sz.ConfigTool\n{\n");
    out.push_str(
        "    /// <summary>Error kind of a <see cref=\"SzConfigToolException\"/>, one per wire reason code.</summary>\n",
    );
    out.push_str("    public enum SzConfigToolErrorKind\n    {\n");
    out.push_str(
        "        /// <summary>No reason code was reported (or it is not known to this binding).</summary>\n        Unknown = 0,\n",
    );
    for code in reason_codes {
        let _ = writeln!(
            out,
            "\n        /// <summary>Reason code <c>{code}</c>.</summary>\n        {},",
            pascal(code)
        );
    }
    out.push_str("    }\n\n");
    out.push_str("    /// <summary>Maps wire reason codes to <see cref=\"SzConfigToolErrorKind\"/>.</summary>\n");
    out.push_str("    public static class SzConfigToolErrorKinds\n    {\n");
    out.push_str("        /// <summary>The kind for <paramref name=\"reasonCode\"/> (<see cref=\"SzConfigToolErrorKind.Unknown\"/> if null or unrecognized).</summary>\n");
    out.push_str("        /// <param name=\"reasonCode\">A wire reason code, e.g. <c>NOT_FOUND</c>.</param>\n");
    out.push_str("        /// <returns>The error kind.</returns>\n");
    out.push_str("        public static SzConfigToolErrorKind FromReasonCode(string? reasonCode)\n        {\n");
    out.push_str("            switch (reasonCode)\n            {\n");
    for code in reason_codes {
        let _ = writeln!(
            out,
            "                case \"{code}\": return SzConfigToolErrorKind.{};",
            pascal(code)
        );
    }
    out.push_str("                default: return SzConfigToolErrorKind.Unknown;\n");
    out.push_str("            }\n        }\n\n");
    out.push_str("        /// <summary>The wire reason code of <paramref name=\"kind\"/>, or null for <see cref=\"SzConfigToolErrorKind.Unknown\"/>.</summary>\n");
    out.push_str("        /// <param name=\"kind\">An error kind.</param>\n");
    out.push_str("        /// <returns>The reason code.</returns>\n");
    out.push_str(
        "        public static string? ToReasonCode(SzConfigToolErrorKind kind)\n        {\n",
    );
    out.push_str("            switch (kind)\n            {\n");
    for code in reason_codes {
        let _ = writeln!(
            out,
            "                case SzConfigToolErrorKind.{}: return \"{code}\";",
            pascal(code)
        );
    }
    out.push_str("                default: return null;\n");
    out.push_str("            }\n        }\n    }\n}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Status;

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
            json_type: None,
        }
    }

    fn func(name: &str, args: Vec<Arg>, returns: Returns) -> Function {
        Function {
            name: name.into(),
            group: "g".into(),
            doc: "Do <it> & more.".into(),
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

    #[test]
    fn test_casing_splits_on_underscore_without_acronyms() {
        assert_eq!(pascal("add_data_source"), "AddDataSource");
        assert_eq!(pascal("get_ftype_id"), "GetFtypeId");
        assert_eq!(pascal("JSON_PARSE"), "JsonParse");
        assert_eq!(camel("default_value"), "defaultValue");
        assert_eq!(camel("code"), "code");
    }

    #[test]
    fn test_param_name_escapes_keywords_and_config_collision() {
        assert_eq!(param_name("class"), "@class");
        assert_eq!(param_name("internal"), "@internal");
        assert_eq!(param_name("config_json"), "configJsonArg");
        assert_eq!(param_name("feature_code"), "featureCode");
    }

    #[test]
    fn test_param_decl_modes() {
        let mut a = arg("id", ArgType::Int);
        assert_eq!(param_decl(&a, a.ty), "long id");
        a.optional = true;
        assert_eq!(param_decl(&a, a.ty), "long? id = null");
        a.required = true;
        assert_eq!(param_decl(&a, a.ty), "long id");
        let mut t = arg("desc", ArgType::Str);
        t.tristate = true;
        assert_eq!(param_decl(&t, t.ty), "FieldUpdate<string> desc = default");
        assert_eq!(writer_stmt(&t, t.ty), "args.TriStr(\"desc\", desc);");
    }

    #[test]
    fn test_writer_stmt_passes_param_name_for_reference_args() {
        assert_eq!(
            writer_stmt(&arg("class", ArgType::Str), ArgType::Str),
            "args.Str(\"class\", @class, nameof(@class));"
        );
        let mut j = arg("rule_config", ArgType::Json);
        j.optional = true;
        assert_eq!(
            writer_stmt(&j, j.ty),
            "args.OptJson(\"rule_config\", ruleConfig, nameof(ruleConfig));"
        );
        let mut o = arg("default_value", ArgType::Str);
        o.optional = true;
        assert_eq!(
            writer_stmt(&o, o.ty),
            "args.OptStr(\"default_value\", defaultValue);"
        );
    }

    #[test]
    fn test_required_args_come_first() {
        let mut opt = arg("opt", ArgType::Str);
        opt.optional = true;
        let mut req_opt = arg("must", ArgType::Str);
        req_opt.optional = true;
        req_opt.required = true;
        let f = func(
            "f",
            vec![opt, arg("a", ArgType::Str), req_opt],
            Returns::Config,
        );
        let names: Vec<&str> = ordered_args(&f)
            .iter()
            .map(|&i| f.args[i].name.as_str())
            .collect();
        assert_eq!(names, vec!["a", "must", "opt"]);
    }

    #[test]
    fn test_skips_not_implemented_and_escapes_docs() {
        let mut stub = func("stub_fn", vec![], Returns::Config);
        stub.status = Status::NotImplemented;
        let out = api_file(&[func("list_things", vec![], Returns::Json), stub]);
        assert!(out.contains("public static string ListThings(string configJson)"));
        assert!(out.contains("NativeCall.Json(\"list_things\", configJson, \"{}\")"));
        assert!(out.contains("Do &lt;it&gt; &amp; more."));
        assert!(!out.contains("StubFn"));
        assert!(out.ends_with("}\n"));
    }

    #[test]
    fn test_config_and_json_tuple_names_generate_named_record() {
        let mut f = func(
            "set_plan",
            vec![arg("code", ArgType::Str)],
            Returns::ConfigAndJson,
        );
        f.tuple_names = vec!["plan_id".into(), "was_created".into()];
        let out = api_file(&[f]);
        assert!(out.contains("public static string SetPlan(string configJson, string code)"));
        assert!(out.contains(
            "return NativeCall.ConfigAndJsonConfig(\"set_plan\", configJson, args.ToJson());"
        ));
        assert!(
            out.contains(
                "public static SetPlanRecord SetPlanResult(string configJson, string code)"
            )
        );
        assert!(out.contains(
            "var r = NativeCall.ConfigAndJsonResult(\"set_plan\", configJson, args.ToJson());"
        ));
        assert!(out.contains("string[] m = NativeCall.Members(r, \"plan_id\", \"was_created\");"));
        assert!(out.contains("return new SetPlanRecord(m[0], m[1]);"));
        assert!(
            out.contains("public sealed record SetPlanRecord(string PlanId, string WasCreated);")
        );
        assert!(out.contains("Named result of <see cref=\"SzConfigTool.SetPlanResult\"/>"));
        assert!(
            out.contains("<see cref=\"SetPlanResult\"/> (same arguments)"),
            "{out}"
        );
        assert!(out.contains("The record (row / ids) of SetPlan:"), "{out}");
    }

    #[test]
    fn test_json_tuple_names_generate_named_record_without_config() {
        let mut f = func("verify_it", vec![], Returns::Json);
        f.tuple_names = vec!["current_version".into(), "matches".into()];
        let out = api_file(&[f]);
        assert!(out.contains("public static VerifyItRecord VerifyIt(string configJson)"));
        assert!(out.contains("var r = NativeCall.Json(\"verify_it\", configJson, \"{}\");"));
        assert!(out.contains("NativeCall.Members(r, \"current_version\", \"matches\");"));
        assert!(out.contains("return new VerifyItRecord(m[0], m[1]);"));
        assert!(out.contains(
            "public sealed record VerifyItRecord(string CurrentVersion, string Matches);"
        ));
        assert!(!out.contains("VerifyItResult"));
    }

    #[test]
    fn test_int_or_str_generates_long_and_string_overloads() {
        let mut feat = arg("element_feature", ArgType::Str);
        feat.optional = true;
        let f = func(
            "get_call",
            vec![
                arg("call", ArgType::IntOrStr),
                arg("element_code", ArgType::Str),
                feat,
            ],
            Returns::Json,
        );
        assert_eq!(overloads(&f).len(), 2);
        let out = api_file(&[f]);
        assert!(out.contains(
            "public static string GetCall(string configJson, long call, string elementCode, string? elementFeature = null)"
        ));
        assert!(out.contains(
            "public static string GetCall(string configJson, string call, string elementCode, string? elementFeature = null)"
        ));
        assert!(out.contains("args.Int(\"call\", call);"));
        assert!(out.contains("args.Str(\"call\", call, nameof(call));"));
        assert!(out.contains("Integer form (sent as a JSON integer)."));
        assert!(!out.contains("args.Json(\"call\""));
    }

    #[test]
    fn test_overloads_are_the_cartesian_product() {
        let f = func(
            "f",
            vec![
                arg("a", ArgType::IntOrStr),
                arg("b", ArgType::Bool),
                arg("c", ArgType::IntOrStr),
            ],
            Returns::Config,
        );
        use ArgType::{Bool, Int, Str};
        assert_eq!(
            overloads(&f),
            vec![
                vec![Int, Bool, Int],
                vec![Int, Bool, Str],
                vec![Str, Bool, Int],
                vec![Str, Bool, Str],
            ]
        );
        assert_eq!(
            overloads(&func("g", vec![], Returns::Unit)),
            vec![Vec::<ArgType>::new()]
        );
    }

    #[test]
    fn test_unit_and_config_and_json_bodies() {
        let out = api_file(&[
            func("check_it", vec![], Returns::Unit),
            func("add_it", vec![], Returns::ConfigAndJson),
        ]);
        assert!(out.contains("public static void CheckIt(string configJson)"));
        assert!(out.contains("NativeCall.Unit(\"check_it\", configJson, \"{}\");"));
        assert!(out.contains("public static string AddIt(string configJson)"));
        assert!(out.contains("public static string AddItResult(string configJson)"));
        assert!(
            out.contains("return NativeCall.ConfigAndJsonResult(\"add_it\", configJson, \"{}\");")
        );
    }

    #[test]
    fn test_int_body_parses_the_result_inline() {
        let out = api_file(&[func("count_it", vec![], Returns::Int)]);
        assert!(out.contains("public static long CountIt(string configJson)"));
        assert!(out.contains(
            "return long.Parse(NativeCall.Require(NativeCall.Expect(\"int\", \"count_it\", configJson, \"{}\").Result, \"count_it: envelope without result\"), System.Globalization.NumberStyles.AllowLeadingSign, System.Globalization.CultureInfo.InvariantCulture);"
        ));
    }

    #[test]
    fn test_kind_file_maps_every_reason_code() {
        let out = kind_file(&["NOT_FOUND".into(), "JSON_PARSE".into()]);
        assert!(out.contains("case \"NOT_FOUND\": return SzConfigToolErrorKind.NotFound;"));
        assert!(out.contains("case SzConfigToolErrorKind.JsonParse: return \"JSON_PARSE\";"));
        assert!(out.contains("Unknown = 0,"));
    }

    #[test]
    fn test_generation_is_deterministic() {
        let fs = vec![func("a_b", vec![arg("x", ArgType::Int)], Returns::Config)];
        assert_eq!(api_file(&fs), api_file(&fs));
    }
}
