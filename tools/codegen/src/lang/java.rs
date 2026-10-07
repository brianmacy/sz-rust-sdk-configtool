//! Java wrapper generator (owner: the Java binding; see `bindings/CONTRACT.md`).
//!
//! Emits, from the validated manifest:
//! * `SzConfigTool.java` — `final class SzConfigTool` with one static camelCase
//!   method per implemented function (plus an overload taking a generated
//!   `*Options` builder when the function has optional / tri-state args),
//!   Javadoc from the manifest, and `*Record` records for `tuple_names`.
//!   Every config-changing method returns the config text: a
//!   `config_and_json` function also gets a companion `<fn>Result` (same
//!   parameters and overloads) returning the record JSON text (or the
//!   `*Record` for `tuple_names`).
//!   An `int_or_str` arg (call selector) becomes Java overloads: one taking
//!   `long` (sent as a JSON integer) and one taking `String` (a JSON string);
//!   N such positional args yield the 2^N cartesian overloads.
//! * `SzConfigToolErrorKind.java` — enum of `project.yaml` `reason_codes`.
//! * `TypedDispatch.java` (test source) — routes a conformance step (name +
//!   parsed args) through the TYPED methods, so the JUnit conformance runner
//!   exercises the generated wrappers, not just `NativeBridge.invoke`.
//!
//! Functions with `status: not_implemented` are skipped (they stay reachable
//! through `NativeBridge.invoke`). Every call goes through the hand-written
//! `Args`/`Invoker` helpers in `bindings/java`.

use std::fmt::Write as _;
use std::path::PathBuf;

use crate::Generated;
use crate::emit::GENERATED_BANNER;
use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// Java package of every generated class.
const PACKAGE: &str = "io.github.brianmacy.szconfigtool";
/// Javadoc wrap width (text only, after ` * `).
const WRAP: usize = 88;

/// Java reserved words (plus contextual ones) that cannot be identifiers, and
/// names the generated bodies use for themselves.
const RESERVED: &[&str] = &[
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "false",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "null",
    "package",
    "permits",
    "private",
    "protected",
    "public",
    "record",
    "return",
    "sealed",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "true",
    "try",
    "var",
    "void",
    "volatile",
    "while",
    "yield",
    // identifiers of the generated method bodies
    "configJson",
    "options",
    "wire",
    "in",
    "o",
    "r",
    "f",
];

/// `add_data_source` -> `addDataSource` (split on `_`, no acronym handling).
pub fn camel(snake: &str) -> String {
    let pascal = pascal(snake);
    let mut chars = pascal.chars();
    chars
        .next()
        .map(|c| c.to_ascii_lowercase().to_string() + chars.as_str())
        .unwrap_or_default()
}

/// `add_data_source` -> `AddDataSource`.
pub fn pascal(snake: &str) -> String {
    snake
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            c.next()
                .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect()
}

/// A safe Java identifier for a snake name: camelCase, with `Value` appended
/// when it collides with a reserved word (e.g. `class` -> `classValue`).
pub fn ident(snake: &str) -> String {
    let c = camel(snake);
    if RESERVED.contains(&c.as_str()) {
        c + "Value"
    } else {
        c
    }
}

/// Positional (required) parameter of the typed method, vs. an Options field.
fn is_positional(a: &Arg) -> bool {
    !a.is_optional() || a.required
}

/// Java type of a positional parameter (`int_or_str`: see [`Overload`]).
fn positional_type(a: &Arg) -> &'static str {
    match a.ty {
        ArgType::Str | ArgType::Json => "String",
        ArgType::Int => "long",
        ArgType::Bool => "boolean",
        ArgType::StrList => "java.util.List<String>",
        ArgType::IntOrStr => unreachable!("int_or_str is typed per overload"),
    }
}

/// The two Java renderings of an `int_or_str` arg: `(type, Args method)`.
const INT_OR_STR: [(&str, &str); 2] = [("long", "integer"), ("String", "str")];

/// One generated overload: for each positional `int_or_str` arg (in order),
/// `true` = the `long` rendering, `false` = the `String` rendering.
type Overload = Vec<bool>;

/// Every overload of `f`: the cartesian product over its positional
/// `int_or_str` args, `long` first (a single overload when there are none).
fn overloads(f: &Function) -> Vec<Overload> {
    let n = f
        .args
        .iter()
        .filter(|a| is_positional(a) && a.ty == ArgType::IntOrStr)
        .count();
    (0..1usize << n)
        .map(|bits| (0..n).map(|i| bits & (1 << (n - 1 - i)) == 0).collect())
        .collect()
}

/// Positional args of `f` with their Java type and `Args` method under `ov`.
fn positional_params<'a>(
    f: &'a Function,
    ov: &Overload,
) -> Vec<(&'a Arg, &'static str, &'static str)> {
    let mut choice = ov.iter();
    f.args
        .iter()
        .filter(|a| is_positional(a))
        .map(|a| match a.ty {
            ArgType::IntOrStr => {
                let as_long = *choice.next().expect("overload covers every int_or_str arg");
                let (ty, method) = INT_OR_STR[usize::from(!as_long)];
                (a, ty, method)
            }
            _ => (a, positional_type(a), args_method(a)),
        })
        .collect()
}

/// Java type of an Options setter parameter.
fn option_type(a: &Arg) -> &'static str {
    match (a.tristate, a.ty) {
        (true, ArgType::Int) => "FieldUpdate<Long>",
        (true, _) => "FieldUpdate<String>",
        (false, _) => positional_type(a),
    }
}

/// The `Args` builder method that adds this arg.
fn args_method(a: &Arg) -> &'static str {
    match (a.tristate, a.ty) {
        (true, ArgType::Int) => "intUpdate",
        (true, _) => "strUpdate",
        (false, ArgType::Str) => "str",
        (false, ArgType::Int) => "integer",
        (false, ArgType::Bool) => "bool",
        (false, ArgType::Json) => "json",
        (false, ArgType::StrList) => "strList",
        (false, ArgType::IntOrStr) => unreachable!("int_or_str is typed per overload"),
    }
}

/// Escape free text for a Javadoc comment (HTML, comment end, unicode
/// escapes, tag starts).
fn doc_escape(s: &str) -> String {
    let escaped = s
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\\', "&#92;")
        .replace('@', "&#64;")
        .replace("*/", "*&#47;");
    code_spans(&escaped)
}

/// Balanced Markdown backtick spans -> `<code>..</code>` (unbalanced: kept).
fn code_spans(s: &str) -> String {
    if !s.matches('`').count().is_multiple_of(2) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    for (i, part) in s.split('`').enumerate() {
        if i > 0 {
            out.push_str(if i % 2 == 1 { "<code>" } else { "</code>" });
        }
        out.push_str(part);
    }
    out
}

/// Word-wrap escaped text into Javadoc lines with `indent`.
fn doc_lines(out: &mut String, indent: &str, text: &str) {
    let mut line = String::new();
    for word in doc_escape(text).split_whitespace() {
        if !line.is_empty() && line.len() + 1 + word.len() > WRAP {
            let _ = writeln!(out, "{indent} * {line}");
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        let _ = writeln!(out, "{indent} * {line}");
    }
}

/// Description of one arg for `@param` / setter docs.
fn arg_doc(a: &Arg) -> String {
    let mut text = format!("`{}` ({})", a.name, a.ty.as_str());
    if a.tristate {
        text.push_str(" tri-state: leave / clear / set.");
    } else if a.required {
        text.push_str(" required by the library.");
    }
    if let Some(s) = &a.semantics {
        text.push(' ');
        text.push_str(s);
    }
    if let Some(shape) = a.shape() {
        let _ = write!(text, " Shape: `{shape}`.");
    }
    if let Some(d) = &a.default {
        let _ = write!(text, " Library default when omitted: {d}.");
    }
    text
}

/// Generated record name for a `tuple_names` function.
fn record_name(f: &Function) -> String {
    format!("{}Record", pascal(&f.name))
}

/// Which typed method of a manifest function is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// `<fn>`: the config for config-changing functions, else the result.
    Primary,
    /// `<fn>Result` of a `config_and_json` function: the record only.
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

/// The camelCase Java name of `f`'s typed method in `role`.
fn method_name(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => camel(&f.name),
        Role::Companion => camel(&f.companion().unwrap_or_default()),
    }
}

/// Whether the typed method in `role` returns the named record.
fn returns_record(f: &Function, role: Role) -> bool {
    !f.tuple_names.is_empty() && (f.returns == Returns::Json || role == Role::Companion)
}

fn options_name(f: &Function) -> String {
    format!("{}Options", pascal(&f.name))
}

fn has_options(f: &Function) -> bool {
    f.args.iter().any(|a| !is_positional(a))
}

/// Java return type of the typed method in `role`.
fn return_type(f: &Function, role: Role) -> String {
    if returns_record(f, role) {
        return record_name(f);
    }
    match f.returns {
        Returns::Config | Returns::Json | Returns::ConfigAndJson => "String".to_string(),
        Returns::Int => "long".to_string(),
        Returns::Unit => "void".to_string(),
    }
}

fn return_doc(f: &Function, role: Role) -> String {
    if returns_record(f, role) {
        return "the named result values, each as JSON text".to_string();
    }
    match (f.returns, role) {
        (Returns::Config, _) => "the modified configuration JSON document (opaque)".to_string(),
        (Returns::ConfigAndJson, Role::Primary) => format!(
            "the modified configuration JSON document (opaque); {{@link #{}}} (same \
             arguments) returns the record this operation produces",
            method_name(f, Role::Companion)
        ),
        (Returns::ConfigAndJson, Role::Companion) => {
            "the record (e.g. the created row or ids) as JSON text".to_string()
        }
        (Returns::Json, _) => "the result as JSON text".to_string(),
        (Returns::Int, _) => "the integer result".to_string(),
        (Returns::Unit, _) => String::new(),
    }
}

/// The summary paragraph of `f`'s typed method in `role`.
fn summary(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => f.doc.clone(),
        Role::Companion => format!(
            "The record (row / ids) of `{}`: same arguments and operation, but returns the \
             record instead of the configuration. Operation: {}",
            camel(&f.name),
            f.doc
        ),
    }
}

fn method_doc(out: &mut String, f: &Function, with_options: bool, role: Role) {
    out.push_str("    /**\n");
    doc_lines(out, "    ", &summary(f, role));
    if let Some(notes) = &f.notes {
        out.push_str("     *\n     * <p>Notes:\n");
        doc_lines(out, "    ", notes);
    }
    out.push_str("     *\n");
    let _ = writeln!(
        out,
        "     * <p>Wire name: {{@code {}}}; group: {{@code {}}}.\n     *",
        f.name, f.group
    );
    out.push_str(
        "     * @param configJson the configuration JSON document (opaque; never parsed here)\n",
    );
    for a in f.args.iter().filter(|a| is_positional(a)) {
        let mut text = String::new();
        doc_lines(&mut text, "    ", &arg_doc(a));
        let first = text.trim_start().trim_start_matches("* ");
        let _ = write!(out, "     * @param {} {first}", ident(&a.name));
    }
    if with_options {
        let _ = writeln!(
            out,
            "     * @param options optional arguments ({{@code null}} = none); see {{@link {}}}",
            options_name(f)
        );
    }
    if f.returns != Returns::Unit {
        let _ = writeln!(out, "     * @return {}", return_doc(f, role));
    }
    let _ = writeln!(
        out,
        "     * @throws SzConfigToolException library reason codes: {}; plus the universal \
         INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors",
        if f.errors.is_empty() {
            "none".to_string()
        } else {
            f.errors.join(", ")
        }
    );
    out.push_str("     */\n");
}

/// Parameter list after `configJson` (positional args, optionally options).
fn params(f: &Function, ov: &Overload, with_options: bool) -> String {
    let mut ps = vec!["String configJson".to_string()];
    for (a, ty, _) in positional_params(f, ov) {
        ps.push(format!("{ty} {}", ident(&a.name)));
    }
    if with_options {
        ps.push(format!("{} options", options_name(f)));
    }
    ps.join(", ")
}

/// The `return ...;` statement(s) of the full method body.
fn call_body(f: &Function, role: Role) -> String {
    let name = &f.name;
    let names: Vec<String> = f.tuple_names.iter().map(|n| format!("\"{n}\"")).collect();
    let names = names.join(", ");
    let fields: Vec<String> = (0..f.tuple_names.len())
        .map(|i| format!("f[{i}]"))
        .collect();
    let fields = fields.join(", ");
    let wire_kind = f.returns.as_str();
    if returns_record(f, role) {
        return format!(
            "        String[] f = Invoker.fields(Invoker.result(\"{name}\", \"{wire_kind}\", configJson, wire), {names});\n        \
             return new {rec}({fields});\n",
            rec = record_name(f),
        );
    }
    match (f.returns, role) {
        (Returns::Config, _) => {
            format!("        return Invoker.config(\"{name}\", configJson, wire);\n")
        }
        (Returns::ConfigAndJson, Role::Primary) => format!(
            "        return Invoker.call(\"{name}\", \"{wire_kind}\", configJson, wire)[1];\n"
        ),
        (Returns::ConfigAndJson, Role::Companion) => format!(
            "        return Invoker.result(\"{name}\", \"{wire_kind}\", configJson, wire);\n"
        ),
        (Returns::Json, _) => {
            format!("        return Invoker.json(\"{name}\", configJson, wire);\n")
        }
        (Returns::Int, _) => format!(
            "        return Long.parseLong(Invoker.call(\"{name}\", \"int\", configJson, wire)[2]);\n"
        ),
        (Returns::Unit, _) => format!("        Invoker.unit(\"{name}\", configJson, wire);\n"),
    }
}

/// The typed method(s) of one function: every [`Overload`] of every role.
fn methods(out: &mut String, f: &Function) {
    for role in roles(f) {
        for ov in overloads(f) {
            overload_methods(out, f, &ov, *role);
        }
    }
}

/// The typed method(s) of one overload (plus its options-less forwarder).
fn overload_methods(out: &mut String, f: &Function, ov: &Overload, role: Role) {
    let ret = return_type(f, role);
    let java = method_name(f, role);
    let opts = has_options(f);
    if opts && !f.requires_options {
        method_doc(out, f, false, role);
        let forwarded: Vec<String> = std::iter::once("configJson".to_string())
            .chain(
                f.args
                    .iter()
                    .filter(|a| is_positional(a))
                    .map(|a| ident(&a.name)),
            )
            .chain(std::iter::once("null".to_string()))
            .collect();
        let keyword = if f.returns == Returns::Unit {
            ""
        } else {
            "return "
        };
        let _ = writeln!(
            out,
            "    public static {ret} {java}({}) throws SzConfigToolException {{\n        \
             {keyword}{java}({});\n    }}\n",
            params(f, ov, false),
            forwarded.join(", ")
        );
    }
    method_doc(out, f, opts, role);
    let _ = writeln!(
        out,
        "    public static {ret} {java}({}) throws SzConfigToolException {{",
        params(f, ov, opts)
    );
    if opts {
        out.push_str("        Args wire = options == null ? new Args() : options.wire.copy();\n");
    } else {
        out.push_str("        Args wire = new Args();\n");
    }
    for (a, _, method) in positional_params(f, ov) {
        let _ = writeln!(
            out,
            "        wire.{method}(\"{}\", {});",
            a.name,
            ident(&a.name)
        );
    }
    out.push_str(&call_body(f, role));
    out.push_str("    }\n\n");
}

/// The nested `*Options` builder of one function.
fn options_class(out: &mut String, f: &Function) {
    let name = options_name(f);
    let _ = writeln!(
        out,
        "    /** Optional arguments of {{@link #{}}}; unset = omitted (library default). */\n    \
         public static final class {name} {{\n        final Args wire = new Args();\n",
        camel(&f.name)
    );
    for a in f.args.iter().filter(|a| !is_positional(a)) {
        let renderings: Vec<(&str, &str)> = if a.ty == ArgType::IntOrStr {
            INT_OR_STR.to_vec()
        } else {
            vec![(option_type(a), args_method(a))]
        };
        let id = ident(&a.name);
        for (ty, method) in renderings {
            out.push_str("        /**\n");
            doc_lines(out, "        ", &arg_doc(a));
            let _ = writeln!(
                out,
                "         *\n         * @param {id} the value\n         * @return this builder\n         */\n        \
                 public {name} {id}({ty} {id}) {{\n            wire.{method}(\"{}\", {id});\n            return this;\n        }}\n",
                a.name
            );
        }
    }
    // Both the header and every builder method end with a blank line: drop it.
    out.pop();
    out.push_str("    }\n\n");
}

/// The nested `*Record` record of a `tuple_names` function.
fn result_record(out: &mut String, f: &Function) {
    let comps: Vec<String> = f
        .tuple_names
        .iter()
        .map(|n| format!("String {}", ident(n)))
        .collect();
    let role = *roles(f).last().unwrap_or(&Role::Primary);
    let _ = writeln!(
        out,
        "    /**\n     * Result of {{@link #{}}} (named by the manifest's {{@code tuple_names}}).\n     *",
        method_name(f, role)
    );
    for n in &f.tuple_names {
        let _ = writeln!(
            out,
            "     * @param {} the {{@code {n}}} value as JSON text",
            ident(n)
        );
    }
    let _ = writeln!(
        out,
        "     */\n    public record {}({}) {{\n    }}\n",
        record_name(f),
        comps.join(", ")
    );
}

fn typed(inputs: &Inputs) -> impl Iterator<Item = &Function> {
    inputs
        .functions
        .iter()
        .filter(|f| f.status == Status::Implemented)
}

/// `SzConfigTool.java`.
fn sz_config_tool(inputs: &Inputs) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n\npackage {PACKAGE};\n\n\
         /**\n * Typed, stateless operations on Senzing configuration JSON documents\n \
         * (generated from {{@code api/manifest}}). Every method takes the configuration\n \
         * as an opaque string. A config-changing method returns the modified\n \
         * configuration; when the operation also produces a record (e.g. the new row),\n \
         * the companion {{@code <name>Result}} (same arguments) returns that record\n \
         * instead. Other methods return JSON text; failures throw\n \
         * {{@link SzConfigToolException}}.\n */\n\
         public final class SzConfigTool {{\n    private SzConfigTool() {{\n    }}\n\n"
    );
    for f in typed(inputs) {
        if has_options(f) {
            options_class(&mut out, f);
        }
        if !f.tuple_names.is_empty() {
            result_record(&mut out, f);
        }
        methods(&mut out, f);
    }
    let trimmed = out.trim_end().to_string();
    trimmed + "\n}\n"
}

/// `SzConfigToolErrorKind.java`.
fn error_kind(inputs: &Inputs) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n\npackage {PACKAGE};\n\n\
         /** Every wire reason code ({{@code api/manifest/project.yaml}} {{@code reason_codes}}). */\n\
         public enum SzConfigToolErrorKind {{\n"
    );
    let codes = &inputs.project.reason_codes;
    for (i, code) in codes.iter().enumerate() {
        let sep = if i + 1 == codes.len() { ";" } else { "," };
        let _ = writeln!(out, "    /** {{@code {code}}}. */\n    {code}{sep}");
    }
    out.push_str(
        "\n    /**\n     * The kind for a reason code.\n     *\n     * @param code a reason code\n     \
         * @return its kind, or {@code INTERNAL} when unrecognized\n     */\n    \
         public static SzConfigToolErrorKind fromCode(String code) {\n        \
         for (SzConfigToolErrorKind k : values()) {\n            \
         if (k.name().equals(code)) {\n                return k;\n            }\n        }\n        \
         return INTERNAL;\n    }\n}\n",
    );
    out
}

/// `Conv` call converting a parsed conformance arg value to the Java type.
fn conv(a: &Arg) -> &'static str {
    match (a.tristate, a.ty) {
        (true, ArgType::Int) => "Conv.intUpdate",
        (true, _) => "Conv.strUpdate",
        (false, ArgType::Str) => "Conv.str",
        (false, ArgType::Int) => "Conv.lng",
        (false, ArgType::Bool) => "Conv.bool",
        (false, ArgType::Json) => "Conv.json",
        (false, ArgType::StrList) => "Conv.strList",
        (false, ArgType::IntOrStr) => unreachable!("int_or_str is dispatched per overload"),
    }
}

/// Expression turning the typed method's return into `{kind, config, result}`.
/// For a `config_and_json` function `call` is the primary's call; the
/// companion (same arguments) is called too and both must agree on failure.
fn normalize(f: &Function, call: &str) -> String {
    let companion = call.replacen(
        &format!("SzConfigTool.{}(", camel(&f.name)),
        &format!("SzConfigTool.{}(", method_name(f, Role::Companion)),
        1,
    );
    let names: Vec<String> = f.tuple_names.iter().map(|n| format!("\"{n}\"")).collect();
    let getters: Vec<String> = f
        .tuple_names
        .iter()
        .map(|n| format!("r.{}()", ident(n)))
        .collect();
    let record = format!(
        "Conv.record(new String[] {{{}}}, {})",
        names.join(", "),
        getters.join(", ")
    );
    match (f.returns, f.tuple_names.is_empty()) {
        (Returns::Config, _) => format!("return Conv.config({call});"),
        (Returns::Json, true) => format!("return Conv.json({call});"),
        (Returns::ConfigAndJson, true) => {
            format!("return Conv.configAndJson(() -> {call},\n            () -> {companion});")
        }
        (Returns::Json, false) => {
            format!("var r = {call};\n        return Conv.jsonResult({record});")
        }
        (Returns::ConfigAndJson, false) => format!(
            "return Conv.configAndJson(() -> {call},\n            () -> {{\n                var r = {companion};\n                \
             return {record};\n            }});"
        ),
        (Returns::Int, _) => format!("return Conv.integer({call});"),
        (Returns::Unit, _) => format!("{call};\n        return Conv.unit();"),
    }
}

/// `Conv` call for a positional arg under an overload's `int_or_str` choice.
fn conv_positional(a: &Arg, ty: &str) -> String {
    let f = match (a.ty, ty) {
        (ArgType::IntOrStr, "long") => "Conv.lng",
        (ArgType::IntOrStr, _) => "Conv.str",
        _ => conv(a),
    };
    format!("{f}(in.get(\"{}\"))", a.name)
}

/// Java condition selecting overload `ov` from the parsed conformance args
/// (a JSON integer parses to `Long`), or `None` for the all-`String` fallback.
fn overload_condition(f: &Function, ov: &Overload) -> Option<String> {
    if ov.iter().all(|long| !long) {
        return None;
    }
    let names = f
        .args
        .iter()
        .filter(|a| is_positional(a) && a.ty == ArgType::IntOrStr);
    let tests: Vec<String> = names
        .zip(ov)
        .map(|(a, long)| {
            let neg = if *long { "" } else { "!" };
            format!("{neg}(in.get(\"{}\") instanceof Long)", a.name)
        })
        .collect();
    Some(tests.join(" && "))
}

/// Options setter call(s) for one optional arg in `TypedDispatch`.
fn dispatch_option(out: &mut String, a: &Arg) {
    let (key, id) = (&a.name, ident(&a.name));
    if a.ty == ArgType::IntOrStr {
        let _ = writeln!(
            out,
            "        if (in.get(\"{key}\") instanceof Long v) {{\n            o.{id}(v.longValue());\n        \
             }} else if (in.containsKey(\"{key}\")) {{\n            o.{id}(Conv.str(in.get(\"{key}\")));\n        }}"
        );
    } else {
        let _ = writeln!(
            out,
            "        if (in.containsKey(\"{key}\")) {{\n            o.{id}({}(in.get(\"{key}\")));\n        }}",
            conv(a)
        );
    }
}

/// The overload-selecting calls of `f` (each a `return`), indented by
/// `indent`; `with_options` passes the built `o`, else the overload without
/// an Options parameter is called.
fn dispatch_calls(out: &mut String, f: &Function, with_options: bool, indent: &str) {
    let java = camel(&f.name);
    for ov in overloads(f) {
        let mut call_args = vec!["config".to_string()];
        for (a, ty, _) in positional_params(f, &ov) {
            call_args.push(conv_positional(a, ty));
        }
        if with_options {
            call_args.push("o".to_string());
        }
        let call = format!("SzConfigTool.{java}({})", call_args.join(", "));
        let body = normalize(f, &call).replace("\n        ", &format!("\n{indent}"));
        match overload_condition(f, &ov) {
            Some(cond) => {
                let inner = body.replace(&format!("\n{indent}"), &format!("\n{indent}    "));
                let _ = writeln!(
                    out,
                    "{indent}if ({cond}) {{\n{indent}    {inner}\n{indent}}}"
                );
            }
            None => {
                let _ = writeln!(out, "{indent}{body}");
            }
        }
    }
}

/// One `TypedDispatch` method. A step that passes none of the optional args
/// calls the overload WITHOUT an Options parameter (so both generated
/// overloads, and the `options == null` path, are exercised); otherwise the
/// Options builder is filled from the step's args.
fn dispatch_method(out: &mut String, f: &Function) {
    let _ = writeln!(
        out,
        "    private static String[] {}(String config, Map<String, Object> in)\n            \
         throws SzConfigToolException {{",
        camel(&f.name)
    );
    if has_options(f) {
        // (requires_options: no options-less overload; an empty Options is sent.)
        if !f.requires_options {
            let absent: Vec<String> = f
                .args
                .iter()
                .filter(|a| !is_positional(a))
                .map(|a| format!("!in.containsKey(\"{}\")", a.name))
                .collect();
            let _ = writeln!(out, "        if ({}) {{", absent.join(" && "));
            dispatch_calls(out, f, false, "            ");
            out.push_str("        }\n");
        }
        let _ = writeln!(
            out,
            "        SzConfigTool.{0} o = new SzConfigTool.{0}();",
            options_name(f)
        );
        for a in f.args.iter().filter(|a| !is_positional(a)) {
            dispatch_option(out, a);
        }
    }
    dispatch_calls(out, f, has_options(f), "        ");
    out.push_str("    }\n\n");
}

/// `TypedDispatch.java` (test source).
fn typed_dispatch(inputs: &Inputs) -> String {
    let mut out = format!(
        "// {GENERATED_BANNER}\n\npackage {PACKAGE};\n\nimport java.util.Map;\n\n\
         /** Routes conformance steps through the generated typed methods. */\n\
         final class TypedDispatch {{\n    private TypedDispatch() {{\n    }}\n\n    \
         /**\n     * Call {{@code fn}} through its typed wrapper.\n     *\n     \
         * @return {{kind, config, result}}, or {{@code null}} when {{@code fn}} has no typed wrapper\n     */\n    \
         static String[] call(String fn, String config, Map<String, Object> in)\n            \
         throws SzConfigToolException {{\n        return switch (fn) {{\n"
    );
    for f in typed(inputs) {
        let _ = writeln!(
            out,
            "            case \"{}\" -> {}(config, in);",
            f.name,
            camel(&f.name)
        );
    }
    out.push_str("            default -> null;\n        };\n    }\n\n");
    for f in typed(inputs) {
        dispatch_method(&mut out, f);
    }
    let trimmed = out.trim_end().to_string();
    trimmed + "\n}\n"
}

/// Generated Java files (workspace-relative paths).
pub fn generate(inputs: &Inputs) -> Vec<Generated> {
    vec![
        Generated {
            path: PathBuf::from(&inputs.project.bindings.java.api),
            contents: sz_config_tool(inputs),
        },
        Generated {
            path: PathBuf::from(&inputs.project.bindings.java.error_kinds),
            contents: error_kind(inputs),
        },
        Generated {
            path: PathBuf::from(&inputs.project.bindings.java.test_dispatch),
            contents: typed_dispatch(inputs),
        },
    ]
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
            json_type: None,
        }
    }

    fn func(name: &str, returns: Returns, args: Vec<Arg>) -> Function {
        Function {
            name: name.into(),
            group: "g".into(),
            doc: "Does <things> & */ \\u0041 @see stuff.".into(),
            rust: format!("g::{name}"),
            rust_params_struct: None,
            c_symbol: None,
            c_aliases: Vec::new(),
            args,
            returns,
            tuple_names: Vec::new(),
            errors: vec!["NOT_FOUND".into()],
            notes: Some("A note.".into()),
            c_notes: None,
            status: Status::Implemented,
            requires_options: false,
        }
    }

    /// int_or_str args are expanded into per-type overloads before these
    /// per-arg mappings run; reaching one with IntOrStr is a generator bug.
    #[test]
    #[should_panic(expected = "int_or_str is typed per overload")]
    fn test_positional_type_rejects_int_or_str() {
        positional_type(&arg("x", ArgType::IntOrStr));
    }

    #[test]
    #[should_panic(expected = "int_or_str is typed per overload")]
    fn test_args_method_rejects_int_or_str() {
        args_method(&arg("x", ArgType::IntOrStr));
    }

    #[test]
    #[should_panic(expected = "int_or_str is dispatched per overload")]
    fn test_conv_rejects_int_or_str() {
        conv(&arg("x", ArgType::IntOrStr));
    }

    /// Blank doc text writes no line; a unit return has no @return text.
    #[test]
    fn test_doc_lines_blank_and_unit_return_doc() {
        let mut out = String::new();
        doc_lines(&mut out, "    ", "   ");
        assert_eq!(out, "");
        assert_eq!(
            return_doc(&func("f", Returns::Unit, vec![]), Role::Primary),
            ""
        );
    }

    fn inputs(functions: Vec<Function>) -> Inputs {
        let path = String::new;
        Inputs {
            project: Project {
                root_crate: "lib".into(),
                paths: Paths {
                    root_src: path(),
                    c_header: path(),
                    fixture: path(),
                    manifest_dir: path(),
                    conformance_dir: path(),
                    excluded: path(),
                    dispatch_out: path(),
                    manifest_json_out: path(),
                    conformance_json_out: path(),
                },
                reason_codes: vec!["NOT_FOUND".into(), "INTERNAL".into()],
                bindings: crate::model::Bindings::from_real_project(),
            },
            functions,
            excluded: Excluded {
                enforce_complete: false,
                entries: Vec::new(),
            },
            cases: Vec::new(),
        }
    }

    #[test]
    fn test_naming() {
        assert_eq!(camel("add_data_source"), "addDataSource");
        assert_eq!(pascal("add_data_source"), "AddDataSource");
        assert_eq!(camel("get_ftype_id"), "getFtypeId");
        assert_eq!(camel("x"), "x");
        assert_eq!(camel(""), "");
        assert_eq!(ident("class"), "classValue");
        assert_eq!(ident("default_value"), "defaultValue");
        assert_eq!(ident("options"), "optionsValue");
    }

    #[test]
    fn test_doc_escape_neutralizes_comment_syntax() {
        let e = doc_escape("a <b> & */ \\u0041 @x `c` `d");
        assert_eq!(e, "a &lt;b&gt; &amp; *&#47; &#92;u0041 &#64;x `c` `d");
        assert_eq!(
            doc_escape("`a_b` x `c`"),
            "<code>a_b</code> x <code>c</code>"
        );
    }

    #[test]
    fn test_skips_not_implemented_and_keeps_others() {
        let mut stub = func("stub_fn", Returns::Config, vec![]);
        stub.status = Status::NotImplemented;
        let real = func("real_fn", Returns::Config, vec![arg("code", ArgType::Str)]);
        let out = sz_config_tool(&inputs(vec![stub, real]));
        assert!(!out.contains("stubFn"));
        assert!(out.contains("public static String realFn(String configJson, String code)"));
        assert!(out.contains("wire.str(\"code\", code);"));
        assert!(out.contains("Invoker.config(\"real_fn\", configJson, wire)"));
        let dispatch = typed_dispatch(&inputs(vec![func("a_b", Returns::Unit, vec![])]));
        assert!(dispatch.contains("case \"a_b\" -> aB(config, in);"));
    }

    #[test]
    fn test_required_optional_and_tristate_args() {
        let mut req = arg("feature", ArgType::Str);
        req.optional = true;
        req.required = true;
        let mut opt = arg("id", ArgType::Int);
        opt.optional = true;
        opt.default = Some(serde_json::json!(0));
        let mut tri = arg("description", ArgType::Str);
        tri.tristate = true;
        let mut tri_int = arg("tier", ArgType::Int);
        tri_int.tristate = true;
        let f = func(
            "set_x",
            Returns::ConfigAndJson,
            vec![req, opt, tri, tri_int],
        );
        let out = sz_config_tool(&inputs(vec![f]));
        assert!(
            out.contains("public static String setX(String configJson, String feature) throws")
        );
        assert!(out.contains(
            "public static String setX(String configJson, String feature, SetXOptions options)"
        ));
        assert!(out.contains("return setX(configJson, feature, null);"));
        // The companion takes the same parameters (and Options type).
        assert!(
            out.contains(
                "public static String setXResult(String configJson, String feature) throws"
            )
        );
        assert!(out.contains(
            "public static String setXResult(String configJson, String feature, SetXOptions options)"
        ));
        assert!(out.contains("return setXResult(configJson, feature, null);"));
        assert!(out.contains(
            "        return Invoker.call(\"set_x\", \"config_and_json\", configJson, wire)[1];\n"
        ));
        assert!(out.contains(
            "        return Invoker.result(\"set_x\", \"config_and_json\", configJson, wire);\n"
        ));
        assert!(
            out.contains("{@link #setXResult} (same arguments)"),
            "{out}"
        );
        assert!(
            out.contains("The record (row / ids) of <code>setX</code>"),
            "{out}"
        );
        assert_eq!(
            out.matches("public static final class SetXOptions").count(),
            1
        );
        assert!(out.contains("public SetXOptions id(long id)"));
        assert!(out.contains("public SetXOptions description(FieldUpdate<String> description)"));
        assert!(out.contains("wire.strUpdate(\"description\", description);"));
        assert!(out.contains("public SetXOptions tier(FieldUpdate<Long> tier)"));
        assert!(out.contains("Library default when omitted: 0."));
        assert!(out.contains("required by the library."));
    }

    #[test]
    fn test_return_shapes_and_tuple_records() {
        let mut both = func("set_plan", Returns::ConfigAndJson, vec![]);
        both.tuple_names = vec!["plan_id".into(), "was_created".into()];
        let mut pair = func("check_ver", Returns::Json, vec![]);
        pair.tuple_names = vec!["current_version".into(), "matches".into()];
        let fns = vec![
            both,
            pair,
            func("get_n", Returns::Int, vec![]),
            func("check", Returns::Unit, vec![]),
            func("list_x", Returns::Json, vec![]),
        ];
        let out = sz_config_tool(&inputs(fns.clone()));
        assert!(out.contains("public record SetPlanRecord(String planId, String wasCreated)"));
        assert!(out.contains("Result of {@link #setPlanResult}"));
        assert!(out.contains("public static String setPlan(String configJson)"));
        assert!(out.contains("public static SetPlanRecord setPlanResult(String configJson)"));
        assert!(out.contains(
            "Invoker.fields(Invoker.result(\"set_plan\", \"config_and_json\", configJson, wire), \"plan_id\", \"was_created\");"
        ));
        assert!(out.contains("return new SetPlanRecord(f[0], f[1]);"));
        assert!(
            out.contains("public record CheckVerRecord(String currentVersion, String matches)")
        );
        assert!(out.contains("Result of {@link #checkVer}"));
        assert!(out.contains("public static long getN(String configJson)"));
        assert!(out.contains(
            "        return Long.parseLong(Invoker.call(\"get_n\", \"int\", configJson, wire)[2]);"
        ));
        assert!(out.contains("public static void check(String configJson)"));
        assert!(out.contains("        Invoker.unit(\"check\", configJson, wire);"));
        assert!(out.contains("public static String listX(String configJson)"));
        let dispatch = typed_dispatch(&inputs(fns));
        assert!(dispatch.contains(
            "Conv.record(new String[] {\"plan_id\", \"was_created\"}, r.planId(), r.wasCreated())"
        ));
        assert!(dispatch.contains("return Conv.unit();"));
    }

    #[test]
    fn test_int_or_str_positional_overloads() {
        let mut opt = arg("tag", ArgType::Str);
        opt.optional = true;
        let f = func(
            "get_call",
            Returns::Json,
            vec![arg("call", ArgType::IntOrStr), opt],
        );
        let out = sz_config_tool(&inputs(vec![f.clone()]));
        for ty in ["long", "String"] {
            assert!(out.contains(&format!(
                "public static String getCall(String configJson, {ty} call) throws"
            )));
            assert!(out.contains(&format!(
                "public static String getCall(String configJson, {ty} call, GetCallOptions options)"
            )));
        }
        assert!(out.contains("wire.integer(\"call\", call);"));
        assert!(out.contains("wire.str(\"call\", call);"));
        assert!(!out.contains("wire.json(\"call\""));
        let dispatch = typed_dispatch(&inputs(vec![f]));
        assert!(dispatch.contains(
            "        if ((in.get(\"call\") instanceof Long)) {\n            \
             return Conv.json(SzConfigTool.getCall(config, Conv.lng(in.get(\"call\")), o));\n        }\n        \
             return Conv.json(SzConfigTool.getCall(config, Conv.str(in.get(\"call\")), o));\n"
        ));
        // A step without the optional arg calls the overloads without Options.
        assert!(dispatch.contains(
            "        if (!in.containsKey(\"tag\")) {\n            \
             if ((in.get(\"call\") instanceof Long)) {\n                \
             return Conv.json(SzConfigTool.getCall(config, Conv.lng(in.get(\"call\"))));\n            }\n            \
             return Conv.json(SzConfigTool.getCall(config, Conv.str(in.get(\"call\"))));\n        }\n        \
             SzConfigTool.GetCallOptions o = new SzConfigTool.GetCallOptions();\n"
        ));
    }

    #[test]
    fn test_dispatch_without_options_indents_multiline_bodies() {
        let mut opt = arg("tag", ArgType::Str);
        opt.optional = true;
        let mut f = func(
            "make",
            Returns::ConfigAndJson,
            vec![arg("call", ArgType::IntOrStr), opt],
        );
        f.tuple_names = vec!["id".into()];
        let dispatch = typed_dispatch(&inputs(vec![f]));
        assert!(dispatch.contains(
            "        if (!in.containsKey(\"tag\")) {\n            \
             if ((in.get(\"call\") instanceof Long)) {\n                \
             return Conv.configAndJson(() -> SzConfigTool.make(config, Conv.lng(in.get(\"call\"))),\n                    \
             () -> {\n                        var r = SzConfigTool.makeResult(config, Conv.lng(in.get(\"call\")));\n                        \
             return Conv.record(new String[] {\"id\"}, r.id());\n                    });\n            }\n            \
             return Conv.configAndJson(() -> SzConfigTool.make(config, Conv.str(in.get(\"call\"))),\n                \
             () -> {\n                    var r = SzConfigTool.makeResult(config, Conv.str(in.get(\"call\")));\n                    \
             return Conv.record(new String[] {\"id\"}, r.id());\n                });\n        }\n"
        ));
        // A function without optional args has no Options branch at all.
        let plain = typed_dispatch(&inputs(vec![func(
            "plain",
            Returns::Config,
            vec![arg("code", ArgType::Str)],
        )]));
        assert!(!plain.contains("containsKey"));
        assert!(plain.contains(
            "        return Conv.config(SzConfigTool.plain(config, Conv.str(in.get(\"code\"))));\n"
        ));
    }

    #[test]
    fn test_int_or_str_cartesian_and_option_setters() {
        let mut sel = arg("sel", ArgType::IntOrStr);
        sel.optional = true;
        let f = func(
            "pick",
            Returns::Unit,
            vec![
                arg("a", ArgType::IntOrStr),
                arg("b", ArgType::IntOrStr),
                sel,
            ],
        );
        assert_eq!(
            overloads(&f),
            vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false]
            ]
        );
        let out = sz_config_tool(&inputs(vec![f.clone()]));
        for (a, b) in [
            ("long", "long"),
            ("long", "String"),
            ("String", "long"),
            ("String", "String"),
        ] {
            assert!(out.contains(&format!(
                "public static void pick(String configJson, {a} a, {b} b, PickOptions options)"
            )));
        }
        assert!(out.contains("public PickOptions sel(long sel) {\n            wire.integer("));
        assert!(out.contains("public PickOptions sel(String sel) {\n            wire.str("));
        let dispatch = typed_dispatch(&inputs(vec![f]));
        assert!(
            dispatch.contains(
                "if ((in.get(\"a\") instanceof Long) && !(in.get(\"b\") instanceof Long))"
            )
        );
        assert!(dispatch.contains("if (in.get(\"sel\") instanceof Long v) {"));
        assert!(dispatch.contains("o.sel(Conv.str(in.get(\"sel\")));"));
        // Four overloads, each called with and without Options.
        assert_eq!(dispatch.matches("SzConfigTool.pick(").count(), 8);
        assert!(dispatch.contains("if (!in.containsKey(\"sel\")) {"));
        assert!(overloads(&func("none", Returns::Unit, vec![])) == vec![Vec::<bool>::new()]);
    }

    #[test]
    fn test_javadoc_is_escaped_and_complete() {
        let mut a = arg("class", ArgType::Str);
        a.semantics = Some("Exactly one of <A>, B.".into());
        let out = sz_config_tool(&inputs(vec![func("add_c", Returns::Config, vec![a])]));
        assert!(out.contains("Does &lt;things&gt; &amp; *&#47; &#92;u0041 &#64;see stuff."));
        assert!(
            out.contains(
                "* @param classValue <code>class</code> (str) Exactly one of &lt;A&gt;, B."
            )
        );
        assert!(out.contains("library reason codes: NOT_FOUND;"));
        assert!(out.contains("<p>Notes:"));
        assert!(
            !out.contains("\\u"),
            "a raw unicode escape would break javac"
        );
    }

    #[test]
    fn test_error_kind_enum_from_reason_codes() {
        let out = error_kind(&inputs(vec![]));
        assert!(out.contains("    NOT_FOUND,\n"));
        assert!(out.contains("    INTERNAL;\n"));
        assert!(out.contains("return INTERNAL;"));
    }

    #[test]
    fn test_generate_is_deterministic_with_banner_and_paths() {
        let i = inputs(vec![func(
            "a_b",
            Returns::Json,
            vec![arg("x", ArgType::StrList)],
        )]);
        let first = generate(&i);
        assert_eq!(first, generate(&i));
        let paths: Vec<String> = first
            .iter()
            .map(|g| g.path.to_string_lossy().replace('\\', "/"))
            .collect();
        assert_eq!(
            paths,
            [
                "bindings/java/src/main/java/io/github/brianmacy/szconfigtool/SzConfigTool.java",
                "bindings/java/src/main/java/io/github/brianmacy/szconfigtool/SzConfigToolErrorKind.java",
                "bindings/java/src/test/java/io/github/brianmacy/szconfigtool/TypedDispatch.java",
            ]
        );
        for g in &first {
            assert!(g.contents.starts_with(&format!("// {GENERATED_BANNER}")));
            assert!(g.contents.ends_with("}\n"));
        }
        assert!(first[0].contents.contains("java.util.List<String> x"));
    }
}
