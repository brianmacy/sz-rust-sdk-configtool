//! C++ wrapper generator (owner: the cpp binding, `bindings/cpp`).
//!
//! Emits three deterministic headers from the manifest:
//!
//! * `error_kinds.hpp` — `ErrorKind` enum + reason-code mapping (from
//!   `project.yaml reason_codes`).
//! * `api.hpp` — one inline PascalCase function per IMPLEMENTED manifest
//!   function (`status: not_implemented` is skipped; it stays reachable via
//!   `Invoke`), its `<Name>Options` struct when it has optional / tri-state
//!   args, and a `<Name>Record` struct (one JSON-text field per name) for
//!   `tuple_names`. Every config-changing function returns the config text; a
//!   `config_and_json` function also gets a companion `<Name>Result` (same
//!   parameters and overloads) returning the record JSON text (or the
//!   `<Name>Record`). A required `int_or_str` arg yields two overloads
//!   (`std::int64_t` / `std::string_view`).
//! * `typed_dispatch.hpp` (tests only) — maps each wire name to a lambda that
//!   decodes conformance `args` and calls the TYPED function, plus the
//!   workspace-relative input paths from `project.yaml` (never hardcoded in
//!   the C++ tests).
//!
//! The runtime the generated code calls (`detail::ArgsWriter`,
//! `detail::Call`, `FieldUpdate`, ...) is hand-written in
//! `bindings/cpp/include/szconfigtool/core.hpp`.

use std::fmt::Write as _;
use std::path::PathBuf;

use crate::Generated;
use crate::emit::GENERATED_BANNER;
use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// C++20 keywords and alternative tokens: an arg with one of these names gets
/// a trailing `_` (e.g. `class` -> `class_`).
const CPP_KEYWORDS: &[&str] = &[
    "alignas",
    "alignof",
    "and",
    "and_eq",
    "asm",
    "auto",
    "bitand",
    "bitor",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char8_t",
    "char16_t",
    "char32_t",
    "class",
    "compl",
    "concept",
    "const",
    "consteval",
    "constexpr",
    "constinit",
    "const_cast",
    "continue",
    "co_await",
    "co_return",
    "co_yield",
    "decltype",
    "default",
    "delete",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "not",
    "not_eq",
    "nullptr",
    "operator",
    "or",
    "or_eq",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "requires",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "template",
    "this",
    "thread_local",
    "throw",
    "true",
    "try",
    "typedef",
    "typeid",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
    "xor",
    "xor_eq",
];

/// Identifiers the generated function bodies use themselves; an arg with one
/// of these names is suffixed like a keyword.
const RESERVED_LOCALS: &[&str] = &["config_json", "options", "sz_args", "sz_env", "sz_out"];

/// `add_data_source` -> `AddDataSource` (split on `_`, no acronym rules).
pub fn pascal(snake: &str) -> String {
    snake
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut chars = p.chars();
            chars.next().map_or_else(String::new, |c| {
                c.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
            })
        })
        .collect()
}

/// The C++ identifier for an arg (keywords and the generated bodies' own
/// locals get a trailing `_`).
pub fn ident(name: &str) -> String {
    if CPP_KEYWORDS.contains(&name) || RESERVED_LOCALS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

/// A C++ string literal for `s` (manifest names/paths are ASCII).
fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Escape text for a Doxygen comment body.
fn doxy_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if matches!(c, '\\' | '@' | '<' | '>' | '&' | '#' | '%') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// `/// ` comment lines for `text` (blank lines kept, trailing space trimmed).
fn doc_lines(indent: &str, text: &str) -> String {
    doc_cmd(indent, "", text)
}

/// Like [`doc_lines`], with an unescaped Doxygen command (e.g. `@brief `)
/// before the escaped text.
fn doc_cmd(indent: &str, cmd: &str, text: &str) -> String {
    text.trim_end()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let lead = if i == 0 { cmd } else { "" };
            let body = format!("{lead}{}", doxy_escape(l.trim_end()));
            if body.is_empty() {
                format!("{indent}///\n")
            } else {
                format!("{indent}/// {body}\n")
            }
        })
        .collect()
}

fn is_option(a: &Arg) -> bool {
    a.is_optional() && !a.required
}

fn has_options(f: &Function) -> bool {
    f.args.iter().any(is_option)
}

fn options_name(f: &Function) -> String {
    format!("{}Options", pascal(&f.name))
}

fn tuple_struct_name(f: &Function) -> Option<String> {
    (!f.tuple_names.is_empty()).then(|| format!("{}Record", pascal(&f.name)))
}

/// Which typed function of a manifest function is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// `<Name>`: the config for config-changing functions, else the result.
    Primary,
    /// `<Name>Result` of a `config_and_json` function: the record only.
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

/// The PascalCase C++ name of `f`'s typed function in `role`.
fn fn_name(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => pascal(&f.name),
        Role::Companion => pascal(&f.companion().unwrap_or_default()),
    }
}

/// The record struct the typed function in `role` returns, if any.
fn returned_struct(f: &Function, role: Role) -> Option<String> {
    tuple_struct_name(f).filter(|_| f.returns == Returns::Json || role == Role::Companion)
}

/// C++ type of an `int_or_str` value that is not overloaded (an option field).
const INT_OR_STR: &str = "std::variant<std::int64_t, std::string>";

/// C++ value type of an arg.
fn value_type(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str | ArgType::Json => "std::string",
        ArgType::IntOrStr => INT_OR_STR,
        ArgType::Int => "std::int64_t",
        ArgType::Bool => "bool",
        ArgType::StrList => "std::vector<std::string>",
    }
}

/// C++ parameter type of a positional (required) arg. `int_or_str` never
/// reaches here: [`overloads`] replaces it with `int` / `str`.
fn param_type(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str | ArgType::Json => "std::string_view",
        ArgType::IntOrStr => "const std::variant<std::int64_t, std::string>&",
        ArgType::Int => "std::int64_t",
        ArgType::Bool => "bool",
        ArgType::StrList => "const std::vector<std::string>&",
    }
}

/// `ArgsWriter` method that writes one value of `ty`.
fn writer_method(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str => "Str",
        ArgType::Json => "Json",
        ArgType::IntOrStr => "IntOrStr",
        ArgType::Int => "Int",
        ArgType::Bool => "Bool",
        ArgType::StrList => "StrList",
    }
}

fn return_type(f: &Function, role: Role) -> String {
    if let Some(name) = returned_struct(f, role) {
        return name;
    }
    match f.returns {
        Returns::Config | Returns::Json | Returns::ConfigAndJson => "std::string".into(),
        Returns::Int => "std::int64_t".into(),
        Returns::Unit => "void".into(),
    }
}

fn returns_enum(r: Returns) -> &'static str {
    match r {
        Returns::Config => "ResultKind::Config",
        Returns::Json => "ResultKind::Json",
        Returns::ConfigAndJson => "ResultKind::ConfigAndJson",
        Returns::Int => "ResultKind::Int",
        Returns::Unit => "ResultKind::Unit",
    }
}

/// The documentation of one arg: semantics plus a documentary default.
fn arg_doc(a: &Arg) -> String {
    let mut parts = vec![format!("Wire arg `{}` ({}).", a.name, a.ty.as_str())];
    if a.tristate {
        parts.push("Tri-state: Leave (absent) / Clear (null) / Set.".into());
    }
    if let Some(s) = &a.semantics {
        parts.push(s.trim().to_string());
    }
    if let Some(d) = &a.default {
        parts.push(format!("Library default when absent: {d}."));
    }
    parts.join(" ")
}

fn options_struct(f: &Function) -> String {
    let mut out = format!(
        "/// Optional arguments of {}(); omitted fields are not sent.\nstruct {} {{\n",
        pascal(&f.name),
        options_name(f)
    );
    for a in f.args.iter().filter(|a| is_option(a)) {
        out.push_str(&doc_lines("    ", &arg_doc(a)));
        let ty = value_type(a.ty);
        let wrapper = if a.tristate {
            format!("FieldUpdate<{ty}>")
        } else {
            format!("std::optional<{ty}>")
        };
        let _ = writeln!(out, "    {wrapper} {}{{}};", ident(&a.name));
    }
    out.push_str("};\n\n");
    out
}

fn tuple_struct(f: &Function) -> String {
    let Some(name) = tuple_struct_name(f) else {
        return String::new();
    };
    let mut out = format!(
        "/// Result of {}(): each named record field as JSON text (exactly the\n\
         /// record member's JSON, e.g. `1001`, `true`, `\"4.0.0\"`).\nstruct {name} {{\n",
        fn_name(f, *roles(f).last().unwrap_or(&Role::Primary))
    );
    for t in &f.tuple_names {
        let _ = writeln!(
            out,
            "    /// Record field `{t}` (JSON text).\n    std::string {}{{}};",
            ident(t)
        );
    }
    out.push_str("};\n\n");
    out
}

/// The summary of `f`'s typed function in `role`.
fn summary(f: &Function, role: Role) -> String {
    match role {
        Role::Primary => f.doc.trim().to_string(),
        Role::Companion => format!(
            "The record (row / ids) of {}(): same arguments and operation, but returns the \
             record instead of the configuration. Operation: {}",
            pascal(&f.name),
            f.doc.trim()
        ),
    }
}

fn function_doc(f: &Function, role: Role) -> String {
    let mut out = doc_cmd("", "@brief ", &summary(f, role));
    if let Some(n) = &f.notes {
        out.push_str("///\n");
        out.push_str(&doc_lines("", n));
    }
    out.push_str("///\n/// @param config_json Configuration JSON (opaque; passed byte-exact).\n");
    for a in f.args.iter().filter(|a| !is_option(a)) {
        let req = if a.required { "REQUIRED. " } else { "" };
        let cmd = format!("@param {} ", ident(&a.name));
        out.push_str(&doc_cmd("", &cmd, &format!("{req}{}", arg_doc(a))));
    }
    if has_options(f) {
        let _ = writeln!(
            out,
            "/// @param options Optional arguments (see {}).",
            options_name(f)
        );
    }
    let ret = match (f.returns, role) {
        _ if returned_struct(f, role).is_some() => format!(
            "{} (fields: {}).",
            tuple_struct_name(f).unwrap_or_default(),
            f.tuple_names.join(", ")
        ),
        (Returns::Config, _) => "The modified configuration JSON.".to_string(),
        (Returns::Json, _) => "The result as JSON text.".to_string(),
        (Returns::ConfigAndJson, Role::Primary) => format!(
            "The modified configuration JSON. {}() (same arguments) returns the record this \
             operation produces.",
            fn_name(f, Role::Companion)
        ),
        (Returns::ConfigAndJson, Role::Companion) => {
            "The record (e.g. the created row or ids) as JSON text.".into()
        }
        (Returns::Int, _) => "The integer result.".into(),
        (Returns::Unit, _) => String::new(),
    };
    if !ret.is_empty() {
        let _ = writeln!(out, "/// @return {ret}");
    }
    let errors = if f.errors.is_empty() {
        "none beyond the universal wire errors".to_string()
    } else {
        f.errors.join(", ")
    };
    let _ = writeln!(
        out,
        "/// @throws SzConfigToolException Library reason codes: {}.",
        doxy_escape(&errors)
    );
    out
}

fn signature(f: &Function, role: Role) -> String {
    let mut params = vec!["const std::string& config_json".to_string()];
    for a in f.args.iter().filter(|a| !is_option(a)) {
        params.push(format!("{} {}", param_type(a.ty), ident(&a.name)));
    }
    if has_options(f) {
        params.push(format!("const {}& options = {{}}", options_name(f)));
    }
    let nodiscard = if f.returns == Returns::Unit {
        ""
    } else {
        "[[nodiscard]] "
    };
    format!(
        "{nodiscard}inline {} {}({})",
        return_type(f, role),
        fn_name(f, role),
        params.join(", ")
    )
}

fn write_arg(a: &Arg) -> String {
    let id = ident(&a.name);
    let key = lit(&a.name);
    let m = writer_method(a.ty);
    if !is_option(a) {
        return format!("    sz_args.{m}({key}, {id});\n");
    }
    if a.tristate {
        return format!(
            "    if (options.{id}.IsSet()) {{\n        sz_args.{m}({key}, options.{id}.Value());\n    \
             }} else if (options.{id}.IsClear()) {{\n        sz_args.Null({key});\n    }}\n"
        );
    }
    format!("    if (options.{id}) {{\n        sz_args.{m}({key}, *options.{id});\n    }}\n")
}

fn tuple_return_body(f: &Function, name: &str) -> String {
    let fields: Vec<String> = f
        .tuple_names
        .iter()
        .map(|t| format!(".{} = sz_rec.Member({})", ident(t), lit(t)))
        .collect();
    format!(
        "    const detail::Record sz_rec(std::move(sz_env.result));\n    return {name}{{{}}};\n",
        fields.join(", ")
    )
}

fn return_body(f: &Function, role: Role) -> String {
    if let Some(name) = returned_struct(f, role) {
        return tuple_return_body(f, &name);
    }
    match (f.returns, role) {
        (Returns::Config, _) | (Returns::ConfigAndJson, Role::Primary) => {
            "    return std::move(sz_env.config);\n".into()
        }
        (Returns::Json, _) | (Returns::ConfigAndJson, Role::Companion) => {
            "    return std::move(sz_env.result);\n".into()
        }
        (Returns::Int, _) => "    return detail::ParseInt(sz_env.result);\n".into(),
        (Returns::Unit, _) => String::new(),
    }
}

/// Positional `int_or_str` args (each becomes an `int` / `str` overload).
fn is_overloaded(a: &Arg) -> bool {
    a.ty == ArgType::IntOrStr && !is_option(a)
}

/// One concrete signature per combination of `int` / `str` for the
/// positional `int_or_str` args (just `f` itself when it has none).
fn overloads(f: &Function) -> Vec<Function> {
    let n = f.args.iter().filter(|a| is_overloaded(a)).count();
    (0..1usize << n)
        .map(|mask| {
            let mut g = f.clone();
            for (bit, a) in g.args.iter_mut().filter(|a| is_overloaded(a)).enumerate() {
                a.ty = if mask & (1 << bit) == 0 {
                    ArgType::Int
                } else {
                    ArgType::Str
                };
            }
            g
        })
        .collect()
}

fn function(f: &Function) -> String {
    let mut out = String::new();
    if has_options(f) {
        out.push_str(&options_struct(f));
    }
    out.push_str(&tuple_struct(f));
    for role in roles(f) {
        for g in overloads(f) {
            out.push_str(&function_body(f, &g, *role));
        }
    }
    out
}

/// One overload: docs from the manifest function `f`, signature and body
/// from its concrete overload `g`, for `f`'s typed function in `role`.
fn function_body(f: &Function, g: &Function, role: Role) -> String {
    let mut out = function_doc(f, role);
    out.push_str(&signature(g, role));
    out.push_str(" {\n    detail::ArgsWriter sz_args;\n");
    for a in &g.args {
        out.push_str(&write_arg(a));
    }
    let bind = if f.returns == Returns::Unit {
        "(void)"
    } else {
        "auto sz_env = "
    };
    let _ = writeln!(
        out,
        "    {bind}detail::Call({}, config_json, sz_args.Finish(), {});",
        lit(&f.name),
        returns_enum(f.returns)
    );
    out.push_str(&return_body(f, role));
    out.push_str("}\n\n");
    out
}

fn implemented(inputs: &Inputs) -> impl Iterator<Item = &Function> {
    inputs
        .functions
        .iter()
        .filter(|f| f.status == Status::Implemented)
}

fn header_open(guard: &str, includes: &[&str]) -> String {
    let mut out = format!("// {GENERATED_BANNER}\n\n#ifndef {guard}\n#define {guard}\n\n");
    for i in includes {
        let _ = writeln!(out, "#include {i}");
    }
    out.push('\n');
    out
}

/// `error_kinds.hpp`.
pub fn error_kinds_hpp(inputs: &Inputs) -> String {
    let codes = &inputs.project.reason_codes;
    let mut out = header_open(
        "SZCONFIGTOOL_GENERATED_ERROR_KINDS_HPP",
        &["<array>", "<string_view>", "<utility>"],
    );
    out.push_str(
        "namespace szconfigtool {\n\n/// The wire error taxonomy (`project.yaml reason_codes`), plus\n\
         /// `Unknown` for a reason code this header does not know.\nenum class ErrorKind {\n",
    );
    for c in codes {
        let _ = writeln!(out, "    /// Reason code `{c}`.\n    {},", pascal(c));
    }
    out.push_str("    /// Reason code not in this header's taxonomy.\n    Unknown,\n};\n\n");
    let _ = writeln!(
        out,
        "/// Every known reason code and its kind.\ninline constexpr std::array<std::pair<std::string_view, ErrorKind>, {}> \
         kReasonCodes{{{{",
        codes.len()
    );
    for c in codes {
        let _ = writeln!(out, "    {{{}, ErrorKind::{}}},", lit(c), pascal(c));
    }
    out.push_str(
        "}};\n\n/// Map a reason code to its kind (`Unknown` if not in the taxonomy).\n\
         [[nodiscard]] constexpr ErrorKind ErrorKindFromReasonCode(std::string_view code) noexcept {\n    \
         for (const auto& [name, kind] : kReasonCodes) {\n        if (name == code) {\n            \
         return kind;\n        }\n    }\n    return ErrorKind::Unknown;\n}\n\n\
         }  // namespace szconfigtool\n\n#endif  // SZCONFIGTOOL_GENERATED_ERROR_KINDS_HPP\n",
    );
    out
}

/// `api.hpp`.
pub fn api_hpp(inputs: &Inputs) -> String {
    let mut out = header_open(
        "SZCONFIGTOOL_GENERATED_API_HPP",
        &[
            "<cstdint>",
            "<optional>",
            "<string>",
            "<string_view>",
            "<utility>",
            "<variant>",
            "<vector>",
            "\"szconfigtool/core.hpp\"",
        ],
    );
    out.push_str("namespace szconfigtool {\n\n");
    for f in implemented(inputs) {
        out.push_str(&function(f));
    }
    out.push_str("}  // namespace szconfigtool\n\n#endif  // SZCONFIGTOOL_GENERATED_API_HPP\n");
    out
}

/// Expression decoding conformance arg `a` from `args` (a `TestArgs`).
fn decode(a: &Arg) -> String {
    format!("args.{}({})", writer_method(a.ty), lit(&a.name))
}

fn dispatch_option(a: &Arg) -> String {
    let id = ident(&a.name);
    let key = lit(&a.name);
    let ty = value_type(a.ty);
    if a.tristate {
        return format!(
            "        if (args.Has({key})) {{\n            options.{id} = args.IsNull({key}) ? \
             szconfigtool::FieldUpdate<{ty}>::Clear() : szconfigtool::FieldUpdate<{ty}>::Set({});\n        }}\n",
            decode(a)
        );
    }
    format!(
        "        if (args.Has({key})) {{\n            options.{id} = {};\n        }}\n",
        decode(a)
    )
}

/// Lambda parameter carrying positional `int_or_str` arg `a` inside the
/// dispatch `std::visit`.
fn visit_param(a: &Arg) -> String {
    format!("sz_v_{}", a.name)
}

/// Statement(s) turning the typed `call` into an `Outcome`. A
/// `config_and_json` step calls the primary AND its companion (same args;
/// `Outcome::FromConfigAndJson` requires both to agree on failure).
fn dispatch_return(f: &Function, call: &str, indent: &str) -> String {
    let fields: Vec<String> = f
        .tuple_names
        .iter()
        .map(|t| format!("{{{}, sz_r.{}}}", lit(t), ident(t)))
        .collect();
    let fields = fields.join(", ");
    if f.returns == Returns::ConfigAndJson {
        let companion = call.replacen(
            &format!("szconfigtool::{}(", pascal(&f.name)),
            &format!("szconfigtool::{}(", fn_name(f, Role::Companion)),
            1,
        );
        let record = if f.tuple_names.is_empty() {
            format!("[&] {{ return {companion}; }}")
        } else {
            format!(
                "[&] {{\n{indent}        const auto sz_r = {companion};\n\
                 {indent}        return Outcome::RecordJson({{{fields}}});\n{indent}    }}"
            )
        };
        return format!(
            "{indent}return Outcome::FromConfigAndJson(\n{indent}    [&] {{ return {call}; }},\n\
             {indent}    {record});\n"
        );
    }
    if !f.tuple_names.is_empty() {
        return format!(
            "{indent}const auto sz_r = {call};\n{indent}return Outcome::FromRecord({}, {{{fields}}});\n",
            lit(f.returns.as_str()),
        );
    }
    match f.returns {
        Returns::Unit => format!("{indent}{call};\n{indent}return Outcome::Unit();\n"),
        r => format!(
            "{indent}return Outcome::From{}({call});\n",
            pascal(r.as_str())
        ),
    }
}

fn dispatch_entry(f: &Function) -> String {
    let known: Vec<String> = f.args.iter().map(|a| lit(&a.name)).collect();
    let mut out = format!(
        "    {{{}, [](const std::string& config, const TestArgs& args) -> Outcome {{\n        \
         args.CheckKnown({{{}}});\n",
        lit(&f.name),
        known.join(", ")
    );
    let mut call_args = vec!["config".to_string()];
    call_args.extend(f.args.iter().filter(|a| !is_option(a)).map(|a| {
        if is_overloaded(a) {
            visit_param(a)
        } else {
            decode(a)
        }
    }));
    if has_options(f) {
        let _ = writeln!(out, "        szconfigtool::{} options;", options_name(f));
        for a in f.args.iter().filter(|a| is_option(a)) {
            out.push_str(&dispatch_option(a));
        }
        call_args.push("options".into());
    }
    let call = format!(
        "szconfigtool::{}({})",
        pascal(&f.name),
        call_args.join(", ")
    );
    let overloaded: Vec<&Arg> = f.args.iter().filter(|a| is_overloaded(a)).collect();
    if overloaded.is_empty() {
        out.push_str(&dispatch_return(f, &call, "        "));
    } else {
        // The JSON type of each selector picks the int / str overload.
        let params: Vec<String> = overloaded
            .iter()
            .map(|a| format!("const auto& {}", visit_param(a)))
            .collect();
        let values: Vec<String> = overloaded.iter().map(|a| decode(a)).collect();
        let _ = write!(
            out,
            "        return std::visit([&]({}) -> Outcome {{\n{}        }}, {});\n",
            params.join(", "),
            dispatch_return(f, &call, "            "),
            values.join(", ")
        );
    }
    out.push_str("    }},\n");
    out
}

/// `typed_dispatch.hpp` (test support).
pub fn dispatch_hpp(inputs: &Inputs) -> String {
    let p = &inputs.project.paths;
    let mut out = header_open(
        "SZCONFIGTOOL_TESTS_GENERATED_TYPED_DISPATCH_HPP",
        &[
            "<map>",
            "<optional>",
            "<set>",
            "<string>",
            "<variant>",
            "\"conformance_support.hpp\"",
            "\"szconfigtool/szconfigtool.hpp\"",
        ],
    );
    let _ = write!(
        out,
        "namespace szconfigtool_test {{\n\n\
         /// Workspace-relative inputs (from project.yaml `paths`).\n\
         inline constexpr const char* kManifestJson = {};\n\
         inline constexpr const char* kConformanceJson = {};\n\n\
         /// Wire names whose typed call runs the primary AND its `<Name>Result`\n\
         /// companion (every implemented `config_and_json` function).\n\
         inline const std::set<std::string>& TypedCompanions() {{\n    \
         static const std::set<std::string> names = {{{}}};\n    return names;\n}}\n\n\
         /// Wire name -> typed call, for every IMPLEMENTED manifest function.\n\
         inline const std::map<std::string, TypedCall>& TypedFunctions() {{\n    \
         static const std::map<std::string, TypedCall> table = {{\n",
        lit(&p.manifest_json_out),
        lit(&p.conformance_json_out),
        implemented(inputs)
            .filter(|f| f.companion().is_some())
            .map(|f| lit(&f.name))
            .collect::<Vec<_>>()
            .join(", ")
    );
    for f in implemented(inputs) {
        out.push_str(&dispatch_entry(f));
    }
    out.push_str(
        "    };\n    return table;\n}\n\n}  // namespace szconfigtool_test\n\n\
         #endif  // SZCONFIGTOOL_TESTS_GENERATED_TYPED_DISPATCH_HPP\n",
    );
    out
}

/// Generated cpp files.
pub fn generate(inputs: &Inputs) -> Vec<Generated> {
    vec![
        Generated {
            path: PathBuf::from(&inputs.project.bindings.cpp.error_kinds),
            contents: error_kinds_hpp(inputs),
        },
        Generated {
            path: PathBuf::from(&inputs.project.bindings.cpp.api),
            contents: api_hpp(inputs),
        },
        Generated {
            path: PathBuf::from(&inputs.project.bindings.cpp.test_dispatch),
            contents: dispatch_hpp(inputs),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Every character lit() escapes, and the IntOrStr parameter type (the
    /// generator expands int_or_str per overload, so no manifest reaches it).
    #[test]
    fn test_lit_escapes_and_int_or_str_param_type() {
        assert_eq!(lit("a\"b\\c\nd"), "\"a\\\"b\\\\c\\nd\"");
        assert_eq!(
            param_type(ArgType::IntOrStr),
            "const std::variant<std::int64_t, std::string>&"
        );
    }

    fn real_inputs() -> Inputs {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root");
        crate::load::load(root, Path::new(crate::DEFAULT_PROJECT_FILE)).expect("manifest loads")
    }

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

    #[test]
    fn test_pascal_splits_on_underscore_without_acronyms() {
        assert_eq!(pascal("add_data_source"), "AddDataSource");
        assert_eq!(pascal("get_cfg_id"), "GetCfgId");
        assert_eq!(pascal("JSON_PARSE"), "JsonParse");
        assert_eq!(pascal("x"), "X");
    }

    #[test]
    fn test_keyword_args_are_suffixed() {
        assert_eq!(ident("class"), "class_");
        assert_eq!(ident("required"), "required");
        assert_eq!(ident("code"), "code");
        assert_eq!(ident("options"), "options_");
    }

    #[test]
    fn test_doxygen_escapes_commands() {
        assert_eq!(doxy_escape("a<b> @x \\y"), "a\\<b\\> \\@x \\\\y");
        assert_eq!(doc_lines("", "one\n\ntwo  \n"), "/// one\n///\n/// two\n");
        assert_eq!(doc_cmd("", "@brief ", "a@b"), "/// @brief a\\@b\n");
    }

    #[test]
    fn test_required_optional_arg_is_positional() {
        let mut req = arg("plan", ArgType::Str);
        req.optional = true;
        req.required = true;
        let mut opt = arg("feature", ArgType::Str);
        opt.optional = true;
        let f = func("add_x", vec![req, opt], Returns::Config);
        let sig = signature(&f, Role::Primary);
        assert_eq!(
            sig,
            "[[nodiscard]] inline std::string AddX(const std::string& config_json, \
             std::string_view plan, const AddXOptions& options = {})"
        );
        assert!(options_struct(&f).contains("std::optional<std::string> feature{};"));
        assert!(!options_struct(&f).contains("plan"));
    }

    #[test]
    fn test_tristate_writes_leave_clear_set() {
        let mut t = arg("tier", ArgType::Int);
        t.tristate = true;
        let w = write_arg(&t);
        assert!(w.contains("options.tier.IsSet()"), "{w}");
        assert!(
            w.contains("sz_args.Int(\"tier\", options.tier.Value())"),
            "{w}"
        );
        assert!(w.contains("sz_args.Null(\"tier\")"), "{w}");
        let f = func("set_x", vec![t], Returns::Config);
        assert!(options_struct(&f).contains("FieldUpdate<std::int64_t> tier{};"));
    }

    #[test]
    fn test_unit_and_tuple_returns() {
        let u = function(&func("check_it", vec![], Returns::Unit));
        assert!(u.contains("inline void CheckIt("), "{u}");
        assert!(!u.contains("[[nodiscard]]"), "{u}");
        assert!(!u.contains("auto sz_env"), "{u}");
        assert!(u.contains("    (void)detail::Call(\"check_it\""), "{u}");
    }

    #[test]
    fn test_config_and_json_tuple_is_config_plus_named_fields() {
        let mut t = func("set_plan", vec![], Returns::ConfigAndJson);
        t.tuple_names = vec!["plan_id".into(), "was_created".into()];
        let g = function(&t);
        assert!(g.contains("/// Result of SetPlanResult(): each"), "{g}");
        assert!(g.contains("struct SetPlanRecord {\n"), "{g}");
        assert!(!g.contains("std::string json"), "{g}");
        assert!(!g.contains("    std::string config{};"), "{g}");
        assert!(g.contains("    std::string plan_id{};"), "{g}");
        assert!(g.contains("inline std::string SetPlan("), "{g}");
        assert!(g.contains("    return std::move(sz_env.config);\n"), "{g}");
        assert!(
            g.contains("SetPlanResult() (same arguments) returns the record"),
            "{g}"
        );
        assert!(g.contains("inline SetPlanRecord SetPlanResult("), "{g}");
        assert!(
            g.contains("/// @brief The record (row / ids) of SetPlan():"),
            "{g}"
        );
        assert!(
            g.contains(
                "return SetPlanRecord{.plan_id = sz_rec.Member(\"plan_id\"), \
                 .was_created = sz_rec.Member(\"was_created\")};"
            ),
            "{g}"
        );
        let d = dispatch_entry(&t);
        assert!(
            d.contains(
                "        return Outcome::FromConfigAndJson(\n            \
                 [&] { return szconfigtool::SetPlan(config); },\n            [&] {\n                \
                 const auto sz_r = szconfigtool::SetPlanResult(config);\n                \
                 return Outcome::RecordJson({{\"plan_id\", sz_r.plan_id}, {\"was_created\", sz_r.was_created}});\n            \
                 });\n"
            ),
            "{d}"
        );
        let plain = function(&func("add_it", vec![], Returns::ConfigAndJson));
        assert!(plain.contains("inline std::string AddIt("), "{plain}");
        assert!(plain.contains("inline std::string AddItResult("), "{plain}");
        assert!(
            plain.contains("    return std::move(sz_env.result);\n"),
            "{plain}"
        );
        assert!(
            plain.contains("/// @return The record (e.g. the created row or ids) as JSON text."),
            "{plain}"
        );
        assert!(
            dispatch_entry(&func("add_it", vec![], Returns::ConfigAndJson))
                .contains("            [&] { return szconfigtool::AddItResult(config); });\n")
        );
    }

    #[test]
    fn test_json_tuple_is_named_fields_without_config() {
        let mut t = func(
            "verify_v",
            vec![arg("expected", ArgType::Str)],
            Returns::Json,
        );
        t.tuple_names = vec!["current_version".into(), "matches".into()];
        let g = function(&t);
        assert!(g.contains("struct VerifyVRecord {\n"), "{g}");
        assert!(g.contains("/// Result of VerifyV(): each"), "{g}");
        assert!(!g.contains("config{}"), "{g}");
        assert!(g.contains("inline VerifyVRecord VerifyV("), "{g}");
        assert!(g.contains("ResultKind::Json);"), "{g}");
        assert!(
            g.contains(
                "return VerifyVRecord{.current_version = sz_rec.Member(\"current_version\"), \
                 .matches = sz_rec.Member(\"matches\")};"
            ),
            "{g}"
        );
        let d = dispatch_entry(&t);
        assert!(
            d.contains("Outcome::FromRecord(\"json\", {{\"current_version\""),
            "{d}"
        );
    }

    #[test]
    fn test_int_or_str_yields_int_and_string_view_overloads() {
        let mut call = arg("call", ArgType::IntOrStr);
        call.rust_convert = Some("call_selector".into());
        let f = func(
            "get_call",
            vec![call, arg("element_code", ArgType::Str)],
            Returns::Json,
        );
        let g = function(&f);
        assert!(
            g.contains(
                "inline std::string GetCall(const std::string& config_json, std::int64_t call, \
                 std::string_view element_code) {\n    detail::ArgsWriter sz_args;\n    \
                 sz_args.Int(\"call\", call);"
            ),
            "{g}"
        );
        assert!(
            g.contains(
                "inline std::string GetCall(const std::string& config_json, std::string_view call, \
                 std::string_view element_code) {\n    detail::ArgsWriter sz_args;\n    \
                 sz_args.Str(\"call\", call);"
            ),
            "{g}"
        );
        assert_eq!(g.matches("inline std::string GetCall(").count(), 2, "{g}");
        // Docs keep the manifest type.
        assert!(g.contains("Wire arg `call` (int_or_str)."), "{g}");
        let d = dispatch_entry(&f);
        assert!(
            d.contains(
                "return std::visit([&](const auto& sz_v_call) -> Outcome {\n            \
                 return Outcome::FromJson(szconfigtool::GetCall(config, sz_v_call, args.Str(\"element_code\")));\n        \
                 }, args.IntOrStr(\"call\"));"
            ),
            "{d}"
        );
    }

    #[test]
    fn test_optional_int_or_str_is_a_variant_field() {
        let mut call = arg("call", ArgType::IntOrStr);
        call.optional = true;
        let f = func("find_call", vec![call], Returns::Json);
        assert_eq!(overloads(&f).len(), 1);
        assert!(
            options_struct(&f)
                .contains("std::optional<std::variant<std::int64_t, std::string>> call{};")
        );
        assert!(write_arg(&f.args[0]).contains("sz_args.IntOrStr(\"call\", *options.call);"));
        assert!(dispatch_entry(&f).contains("options.call = args.IntOrStr(\"call\");"));
    }

    #[test]
    fn test_real_manifest_int_or_str_args_are_overloaded() {
        let inputs = real_inputs();
        let api = api_hpp(&inputs);
        let mut seen = 0;
        for f in implemented(&inputs).filter(|f| f.args.iter().any(is_overloaded)) {
            seen += 1;
            let name = pascal(&f.name);
            assert!(
                api.contains(&format!(
                    " {name}(const std::string& config_json, std::int64_t call"
                )),
                "{name}"
            );
            assert!(
                api.contains(&format!(
                    " {name}(const std::string& config_json, std::string_view call"
                )),
                "{name}"
            );
        }
        assert!(seen > 0);
    }

    #[test]
    fn test_real_manifest_skips_not_implemented_and_is_deterministic() {
        let inputs = real_inputs();
        let api = api_hpp(&inputs);
        assert_eq!(api, api_hpp(&inputs));
        for f in &inputs.functions {
            let decl = format!(" {}(const std::string& config_json", pascal(&f.name));
            assert_eq!(
                api.contains(&decl),
                f.status == Status::Implemented,
                "{}",
                f.name
            );
        }
        assert!(api.ends_with('\n') && api.contains("GENERATED — do not edit"));
    }

    #[test]
    fn test_real_manifest_names_do_not_collide() {
        let inputs = real_inputs();
        let mut seen = std::collections::BTreeSet::new();
        for f in &inputs.functions {
            assert!(seen.insert(pascal(&f.name)), "duplicate {}", f.name);
            for a in &f.args {
                assert!(
                    !RESERVED_LOCALS.contains(&ident(&a.name).as_str()),
                    "{}",
                    a.name
                );
                assert!(
                    !CPP_KEYWORDS.contains(&ident(&a.name).as_str()),
                    "{}",
                    a.name
                );
            }
        }
    }

    #[test]
    fn test_error_kinds_cover_every_reason_code() {
        let inputs = real_inputs();
        let h = error_kinds_hpp(&inputs);
        for c in &inputs.project.reason_codes {
            assert!(
                h.contains(&format!("{{\"{c}\", ErrorKind::{}}}", pascal(c))),
                "{c}"
            );
        }
    }

    #[test]
    fn test_dispatch_covers_implemented_and_paths_come_from_project() {
        let inputs = real_inputs();
        let d = dispatch_hpp(&inputs);
        assert!(d.contains(&lit(&inputs.project.paths.conformance_json_out)));
        for f in &inputs.functions {
            assert_eq!(
                d.contains(&format!("{{{}, [](", lit(&f.name))),
                f.status == Status::Implemented,
                "{}",
                f.name
            );
        }
    }
}
