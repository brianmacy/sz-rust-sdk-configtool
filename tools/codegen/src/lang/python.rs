//! Python wrapper generator (owner: the Python binding; see
//! `bindings/CONTRACT.md`).
//!
//! Emits (at the `project.yaml` `bindings.python` paths) a typed module of
//! stateless functions over the native seam plus its stub, the package
//! `__init__.py`, and the workspace paths the pytest suite reads (taken from
//! `project.yaml`, never hardcoded in the tests). Output is ruff-format stable:
//! every bracketed construct is exploded with a magic trailing comma.
//!
//! Typed-API rules: `status: not_implemented` functions are skipped (they stay
//! reachable through `invoke`); required parameters (incl. `required: true`
//! optional args) come first in manifest order, then keyword-only optional
//! (`None` = absent) and tri-state (`UNSET` = leave, `None` = clear) args.
//! `int_or_str` selectors are `int | str`. A function with `tuple_names`
//! returns a generated `NamedTuple` `<PascalFn>Result` whose named fields are
//! each the record member's JSON text (`config` first for `config_and_json`).

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::Generated;
use crate::emit::GENERATED_BANNER;
use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// Docstring / comment line width (ruff's default line length).
const LINE_WIDTH: usize = 88;
/// The implicit first parameter of every typed function.
const CONFIG_PARAM: &str = "config_json";

/// Python hard keywords (`keyword.kwlist`): an arg with one of these names
/// gets a trailing underscore (PEP 8), e.g. `class` -> `class_`.
const PY_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

/// Generated Python files (workspace-relative paths).
pub fn generate(inputs: &Inputs) -> Vec<Generated> {
    let funcs: Vec<&Function> = inputs
        .functions
        .iter()
        .filter(|f| f.status == Status::Implemented)
        .collect();
    let codes = &inputs.project.reason_codes;
    let out = &inputs.project.bindings.python;
    vec![
        Generated {
            path: PathBuf::from(&out.module),
            contents: module(&funcs, codes, Flavor::Module),
        },
        Generated {
            path: PathBuf::from(&out.stub),
            contents: module(&funcs, codes, Flavor::Stub),
        },
        Generated {
            path: PathBuf::from(&out.init),
            contents: init_module(&funcs, &module_stem(&out.module)),
        },
        paths_module(inputs),
    ]
}

/// Hand-written runtime names re-exported by the package, per module.
const RUNTIME_EXPORTS: &[(&str, &[&str])] = &[
    (
        "._core",
        &[
            "UNSET",
            "ConfigAndJson",
            "Invocation",
            "UnsetType",
            "__version__",
            "abi_version",
            "invoke",
            "library_version",
        ],
    ),
    ("._errors", &["SzConfigToolError"]),
];

/// isort / ruff `order-by-type` key: CONSTANTS, then Classes, then the rest.
fn isort_key(name: &str) -> (u8, String) {
    let rank = match name.chars().next() {
        _ if name.len() > 1 && name.chars().all(|c| !c.is_ascii_lowercase()) => 0,
        Some(c) if c.is_ascii_uppercase() => 1,
        _ => 2,
    };
    (rank, name.to_string())
}

fn isorted(mut names: Vec<&str>) -> Vec<&str> {
    names.sort_by_key(|n| isort_key(n));
    names
}

/// The import name of a module file (its stem).
fn module_stem(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}

/// The package `__init__.py`: explicit re-exports (static for type checkers).
fn init_module(funcs: &[&Function], module_stem: &str) -> String {
    let mut out = format!("# {GENERATED_BANNER}\n{INIT_DOC}\n");
    let mut all: Vec<&str> = vec!["REASON_CODES"];
    for (module, names) in RUNTIME_EXPORTS {
        let sorted = isorted(names.to_vec());
        let one_line = format!("from {module} import {}", sorted.join(", "));
        if one_line.len() <= 88 {
            let _ = writeln!(out, "{one_line}");
        } else {
            // ruff format style: one name per line, magic trailing comma.
            let _ = writeln!(out, "from {module} import (");
            for n in &sorted {
                let _ = writeln!(out, "    {n},");
            }
            out.push_str(")\n");
        }
        all.extend(names.iter());
    }
    let records: Vec<String> = funcs
        .iter()
        .filter(|f| has_record(f))
        .map(|f| record_name(f))
        .collect();
    let generated = isorted(
        std::iter::once("REASON_CODES")
            .chain(records.iter().map(String::as_str))
            .chain(funcs.iter().map(|f| f.name.as_str()))
            .collect(),
    );
    let _ = writeln!(out, "from .{module_stem} import (");
    for n in &generated {
        let _ = writeln!(out, "    {n},");
    }
    out.push_str(")\n\n__all__ = [\n");
    all.extend(records.iter().map(String::as_str));
    all.extend(funcs.iter().map(|f| f.name.as_str()));
    for n in isorted(all) {
        let _ = writeln!(out, "    \"{n}\",");
    }
    out.push_str("]\n");
    out
}

/// The package docstring (in `__init__.py`).
const INIT_DOC: &str = "\"\"\"Typed, stateless functions for editing Senzing configuration JSON.

Every function takes the configuration JSON string first and returns the
modified configuration (or a JSON string / a named-tuple record). This is
an UNOFFICIAL library; it is not the engine-bound Senzing ``SzConfig`` API.
\"\"\"
";

/// Which file is being rendered: runtime module or type stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flavor {
    Module,
    Stub,
}

/// How a parameter is passed (decides annotation, default and wire value).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Positional, required (incl. `required: true` optional args).
    Required,
    /// Keyword-only; `None` = absent.
    Optional,
    /// Keyword-only; `UNSET` = leave, `None` = clear, value = set.
    Tristate,
}

fn mode(a: &Arg) -> Mode {
    match (a.tristate, a.optional && !a.required) {
        (true, _) => Mode::Tristate,
        (false, true) => Mode::Optional,
        (false, false) => Mode::Required,
    }
}

/// The Python parameter name for a wire arg name.
pub fn py_name(name: &str) -> String {
    if PY_KEYWORDS.contains(&name) || name == CONFIG_PARAM {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

fn py_type(ty: ArgType) -> &'static str {
    match ty {
        ArgType::Str => "str",
        ArgType::Int => "int",
        ArgType::Bool => "bool",
        ArgType::Json => "Any",
        ArgType::IntOrStr => "int | str",
        ArgType::StrList => "list[str]",
    }
}

/// `name: annotation[ = default]` for one parameter.
fn param(a: &Arg, flavor: Flavor) -> String {
    let (name, ty) = (py_name(&a.name), py_type(a.ty));
    match (mode(a), flavor) {
        (Mode::Required, _) => format!("{name}: {ty}"),
        (Mode::Optional, _) => format!("{name}: {ty} | None = None"),
        (Mode::Tristate, Flavor::Module) => format!("{name}: {ty} | None | UnsetType = UNSET"),
        (Mode::Tristate, Flavor::Stub) => format!("{name}: {ty} | None | UnsetType = ..."),
    }
}

/// Parameter lines: config, required args, then `*` and the keyword-only rest.
fn params(f: &Function, flavor: Flavor) -> Vec<String> {
    let mut out = vec![format!("{CONFIG_PARAM}: str")];
    let (required, keyword): (Vec<&Arg>, Vec<&Arg>) =
        f.args.iter().partition(|a| mode(a) == Mode::Required);
    out.extend(required.iter().map(|a| param(a, flavor)));
    if !keyword.is_empty() {
        out.push("*".to_string());
        out.extend(keyword.iter().map(|a| param(a, flavor)));
    }
    out
}

/// The value placed in the args mapping for one parameter.
fn wire_value(a: &Arg) -> String {
    let name = py_name(&a.name);
    match mode(a) {
        Mode::Optional => format!("_core.opt({name})"),
        Mode::Required | Mode::Tristate => name,
    }
}

/// Whether `f` returns a generated named record (`tuple_names` set).
fn has_record(f: &Function) -> bool {
    !f.tuple_names.is_empty() && matches!(f.returns, Returns::Json | Returns::ConfigAndJson)
}

/// `<PascalFn>Result`: split the snake name on `_`, no acronym special-casing.
fn record_name(f: &Function) -> String {
    let mut out: String = f
        .name
        .split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|h| h.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect();
    out.push_str("Result");
    out
}

/// (return annotation, `_core` helper) per manifest `returns`.
fn returns(f: &Function) -> (String, &'static str) {
    match (f.returns, has_record(f)) {
        (Returns::Json, true) => (record_name(f), "call_json_record"),
        (Returns::ConfigAndJson, true) => (record_name(f), "call_config_and_record"),
        (Returns::Config, _) => ("str".into(), "call_config"),
        (Returns::Json, _) => ("str".into(), "call_json"),
        (Returns::ConfigAndJson, _) => ("ConfigAndJson".into(), "call_config_and_json"),
        (Returns::Int, _) => ("int".into(), "call_int"),
        (Returns::Unit, _) => ("None".into(), "call_unit"),
    }
}

/// The `NamedTuple` class for a function with `tuple_names`.
fn record_class(f: &Function) -> String {
    let mut out = format!("class {}(NamedTuple):\n", record_name(f));
    let _ = writeln!(
        out,
        "    \"\"\"Result of ``{}``; every named field is JSON text.\"\"\"\n",
        f.name
    );
    if f.returns == Returns::ConfigAndJson {
        out.push_str("    config: str\n");
        out.push_str("    \"\"\"The modified configuration JSON (opaque string).\"\"\"\n");
    }
    for n in &f.tuple_names {
        let _ = writeln!(out, "    {n}: str");
        let _ = writeln!(out, "    \"\"\"``{n}`` as JSON text.\"\"\"");
    }
    out
}

/// Greedy word wrap of `text` into lines of at most `width` characters
/// (a single longer word stays on its own line).
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Make manifest text safe inside a `"""` docstring.
fn escape_doc(text: &str) -> String {
    text.replace('\\', "\\\\").replace("\"\"\"", "\\\"\\\"\\\"")
}

/// Append prose `text` wrapped at `indent`.
fn push_prose(out: &mut String, indent: usize, text: &str) {
    for line in wrap(&escape_doc(text), LINE_WIDTH - indent) {
        let _ = writeln!(out, "{}{line}", " ".repeat(indent));
    }
}

/// Append a `name: description` item wrapped at `indent`, continuation lines
/// hanging at `indent + 4`.
fn push_wrapped(out: &mut String, indent: usize, text: &str) {
    let first = wrap(&escape_doc(text), LINE_WIDTH - indent);
    let Some((head, tail)) = first.split_first() else {
        return;
    };
    let _ = writeln!(out, "{}{head}", " ".repeat(indent));
    let rest = wrap(&tail.join(" "), LINE_WIDTH - indent - 4);
    for line in rest {
        let _ = writeln!(out, "{}{line}", " ".repeat(indent + 4));
    }
}

/// The description of one arg in the `Args:` section.
fn arg_doc(a: &Arg) -> String {
    let mut parts = vec![a.semantics.clone().unwrap_or_else(|| {
        format!(
            "``{}``.",
            if a.ty == ArgType::Json {
                "JSON value"
            } else {
                py_type(a.ty)
            }
        )
    })];
    if py_name(&a.name) != a.name {
        parts.push(format!("Wire name ``{}``.", a.name));
    }
    match mode(a) {
        Mode::Optional => parts.push("``None`` omits it.".to_string()),
        Mode::Tristate => parts.push("``UNSET`` leaves it, ``None`` clears it.".to_string()),
        Mode::Required => {}
    }
    if let Some(d) = &a.default {
        parts.push(format!("Library default when omitted: ``{d}``."));
    }
    parts.join(" ")
}

fn returns_doc(f: &Function) -> Option<String> {
    if has_record(f) {
        let lead = (f.returns == Returns::ConfigAndJson).then_some("config");
        let fields: Vec<&str> = lead
            .into_iter()
            .chain(f.tuple_names.iter().map(String::as_str))
            .collect();
        let named = if lead.is_some() {
            " (``config`` is the modified configuration)"
        } else {
            ""
        };
        return Some(format!(
            "``{}({})``{named}; each named field is that record member's JSON text.",
            record_name(f),
            fields.join(", ")
        ));
    }
    Some(
        match f.returns {
            Returns::Config => "The modified configuration JSON string.",
            Returns::Json => "The result as a JSON string.",
            Returns::ConfigAndJson => {
                "``ConfigAndJson(config, json)``: the modified configuration and the record as \
                 a JSON string."
            }
            Returns::Int => "The integer result.",
            Returns::Unit => return None,
        }
        .to_string(),
    )
}

/// The indented docstring (including quotes) for one function.
fn docstring(f: &Function) -> String {
    let mut out = String::new();
    let summary = wrap(&escape_doc(&f.doc), LINE_WIDTH - 7);
    let _ = writeln!(out, "    \"\"\"{}", summary.join("\n    "));
    out.push_str("\n    Args:\n");
    push_wrapped(
        &mut out,
        8,
        &format!("{CONFIG_PARAM}: The configuration JSON (opaque)."),
    );
    for a in &f.args {
        push_wrapped(
            &mut out,
            8,
            &format!("{}: {}", py_name(&a.name), arg_doc(a)),
        );
    }
    if let Some(r) = returns_doc(f) {
        out.push_str("\n    Returns:\n");
        push_prose(&mut out, 8, &r);
    }
    out.push_str("\n    Raises:\n");
    let codes = if f.errors.is_empty() {
        "argument errors only".to_string()
    } else {
        f.errors.join(", ")
    };
    push_wrapped(
        &mut out,
        8,
        &format!(
            "SzConfigToolError: reason codes {codes}; any call may also raise INVALID_INPUT \
             (bad arguments) or INTERNAL."
        ),
    );
    if let Some(notes) = &f.notes {
        out.push_str("\n    Notes:\n");
        push_prose(&mut out, 8, notes);
    }
    out.push_str("    \"\"\"\n");
    out
}

/// One `def` (signature + docstring, and a body unless it is a stub).
fn function(f: &Function, flavor: Flavor) -> String {
    let (ret_ty, helper) = returns(f);
    let mut out = format!("def {}(\n", f.name);
    for p in params(f, flavor) {
        let _ = writeln!(out, "    {p},");
    }
    let _ = writeln!(out, ") -> {ret_ty}:");
    out.push_str(&docstring(f));
    if flavor == Flavor::Stub {
        return out;
    }
    let _ = write!(
        out,
        "    return _core.{helper}(\n        \"{}\",\n        {CONFIG_PARAM},\n",
        f.name
    );
    if f.args.is_empty() {
        out.push_str("        {},\n");
    } else {
        out.push_str("        {\n");
        for a in &f.args {
            let _ = writeln!(out, "            \"{}\": {},", a.name, wire_value(a));
        }
        out.push_str("        },\n");
    }
    if has_record(f) {
        let _ = writeln!(out, "        {},\n        (", record_name(f));
        for n in &f.tuple_names {
            let _ = writeln!(out, "            \"{n}\",");
        }
        out.push_str("        ),\n");
    }
    out.push_str("    )\n");
    out
}

/// `from ._core import ...` names actually used (so ruff F401 stays clean).
fn core_imports(funcs: &[&Function], flavor: Flavor) -> Vec<&'static str> {
    let any_tri = funcs.iter().flat_map(|f| &f.args).any(|a| a.tristate);
    let any_pair = funcs
        .iter()
        .any(|f| f.returns == Returns::ConfigAndJson && !has_record(f));
    let mut names = Vec::new();
    if any_tri && flavor == Flavor::Module {
        names.push("UNSET");
    }
    if any_pair {
        names.push("ConfigAndJson");
    }
    if any_tri {
        names.push("UnsetType");
    }
    names
}

fn header(funcs: &[&Function], flavor: Flavor) -> String {
    let mut out = format!(
        "# {GENERATED_BANNER}\n\"\"\"Typed configuration functions, one per manifest function.\"\"\"\n\n"
    );
    if flavor == Flavor::Module {
        out.push_str("from __future__ import annotations\n\n");
    }
    let any_json = funcs
        .iter()
        .flat_map(|f| &f.args)
        .any(|a| a.ty == ArgType::Json);
    let any_record = funcs.iter().any(|f| has_record(f));
    let typing: Vec<&str> = [("Any", any_json), ("NamedTuple", any_record)]
        .into_iter()
        .filter_map(|(n, used)| used.then_some(n))
        .collect();
    if !typing.is_empty() {
        let _ = writeln!(out, "from typing import {}\n", typing.join(", "));
    }
    if flavor == Flavor::Module {
        out.push_str("from . import _core\n");
    }
    let names = core_imports(funcs, flavor);
    if !names.is_empty() {
        let _ = writeln!(out, "from ._core import {}", names.join(", "));
    }
    out
}

fn reason_codes(codes: &[String], flavor: Flavor) -> String {
    let doc = "\"\"\"Every ``SzConfigToolError.reason_code`` value.\"\"\"\n";
    match flavor {
        Flavor::Stub => format!("REASON_CODES: tuple[str, ...]\n{doc}"),
        Flavor::Module => {
            let mut out = "REASON_CODES: tuple[str, ...] = (\n".to_string();
            for c in codes {
                let _ = writeln!(out, "    \"{c}\",");
            }
            out.push_str(")\n");
            out.push_str(doc);
            out
        }
    }
}

/// The full `generated.py` / `generated.pyi` text.
fn module(funcs: &[&Function], codes: &[String], flavor: Flavor) -> String {
    let mut out = header(funcs, flavor);
    out.push('\n');
    out.push_str(&reason_codes(codes, flavor));
    let sep = match flavor {
        Flavor::Module => "\n\n",
        Flavor::Stub => "\n",
    };
    for f in funcs.iter().filter(|f| has_record(f)) {
        out.push_str(sep);
        out.push_str(&record_class(f));
    }
    for f in funcs {
        out.push_str(sep);
        out.push_str(&function(f, flavor));
    }
    out
}

/// `tests/generated_paths.py`: workspace paths from `project.yaml`.
fn paths_module(inputs: &Inputs) -> Generated {
    let path = PathBuf::from(&inputs.project.bindings.python.test_paths);
    // parents[0] is the file's directory; one more per path component.
    let parents = path.components().count() - 1;
    let p = &inputs.project.paths;
    let contents = format!(
        "# {GENERATED_BANNER}\n\"\"\"Workspace paths (from project.yaml) used by the tests.\"\"\"\n\n\
         from pathlib import Path\n\n\
         WORKSPACE_ROOT = Path(__file__).resolve().parents[{}]\n\
         MANIFEST_JSON = WORKSPACE_ROOT / \"{}\"\n\
         CONFORMANCE_JSON = WORKSPACE_ROOT / \"{}\"\n\
         FIXTURE = WORKSPACE_ROOT / \"{}\"\n",
        parents, p.manifest_json_out, p.conformance_json_out, p.fixture,
    );
    Generated { path, contents }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn func(args: Vec<Arg>, returns: Returns) -> Function {
        Function {
            name: "do_it".into(),
            group: "g".into(),
            doc: "Do it.".into(),
            rust: "m::do_it".into(),
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
        }
    }

    fn real_inputs() -> Inputs {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();
        crate::load::load(root, Path::new(crate::DEFAULT_PROJECT_FILE)).unwrap()
    }

    #[test]
    fn test_py_name_escapes_keywords_and_config_param() {
        assert_eq!(py_name("class"), "class_");
        assert_eq!(py_name("config_json"), "config_json_");
        assert_eq!(py_name("id"), "id");
        assert_eq!(py_name("match"), "match");
    }

    #[test]
    fn test_required_first_then_keyword_only() {
        let mut opt = arg("desc", ArgType::Str);
        opt.optional = true;
        let mut req_opt = arg("plan", ArgType::Str);
        req_opt.optional = true;
        req_opt.required = true;
        let mut tri = arg("tier", ArgType::Int);
        tri.tristate = true;
        let f = func(
            vec![opt, arg("class", ArgType::Str), tri, req_opt],
            Returns::Config,
        );
        assert_eq!(
            params(&f, Flavor::Module),
            [
                "config_json: str",
                "class_: str",
                "plan: str",
                "*",
                "desc: str | None = None",
                "tier: int | None | UnsetType = UNSET",
            ]
        );
        assert_eq!(
            params(&f, Flavor::Stub)[5],
            "tier: int | None | UnsetType = ..."
        );
        let body = function(&f, Flavor::Module);
        assert!(body.contains("\"class\": class_,"), "{body}");
        assert!(body.contains("\"desc\": _core.opt(desc),"), "{body}");
        assert!(body.contains("\"tier\": tier,"), "{body}");
        assert!(body.contains("\"plan\": plan,"), "{body}");
    }

    #[test]
    fn test_arg_types() {
        let f = func(
            vec![
                arg("a", ArgType::Bool),
                arg("b", ArgType::Json),
                arg("c", ArgType::StrList),
                arg("d", ArgType::IntOrStr),
            ],
            Returns::Unit,
        );
        assert_eq!(
            params(&f, Flavor::Module)[1..],
            ["a: bool", "b: Any", "c: list[str]", "d: int | str"]
        );
        let body = function(&f, Flavor::Module);
        assert!(body.contains("\"d\": d,"), "{body}");
        assert!(docstring(&f).contains("d: ``int | str``."), "{body}");
    }

    #[test]
    fn test_returns_mapping_and_no_args_body() {
        for (r, ty, helper) in [
            (Returns::Config, "-> str:", "_core.call_config("),
            (Returns::Json, "-> str:", "_core.call_json("),
            (
                Returns::ConfigAndJson,
                "-> ConfigAndJson:",
                "_core.call_config_and_json(",
            ),
            (Returns::Int, "-> int:", "_core.call_int("),
            (Returns::Unit, "-> None:", "_core.call_unit("),
        ] {
            let body = function(&func(vec![], r), Flavor::Module);
            assert!(body.contains(ty) && body.contains(helper), "{body}");
            assert!(body.contains("        {},\n"), "{body}");
            assert!(!function(&func(vec![], r), Flavor::Stub).contains("return"));
        }
    }

    #[test]
    fn test_docstring_sections() {
        let mut tri = arg("source", ArgType::Str);
        tri.tristate = true;
        tri.semantics = Some("Keep or clear.".into());
        let mut opt = arg("internal", ArgType::Str);
        opt.optional = true;
        opt.default = Some(serde_json::json!("No"));
        let mut f = func(
            vec![tri, opt, arg("class", ArgType::Str)],
            Returns::ConfigAndJson,
        );
        f.tuple_names = vec!["plan_id".into(), "was_created".into()];
        f.notes = Some("A \"\"\" trap \\ here.".into());
        let pair = docstring(&func(vec![], Returns::ConfigAndJson));
        assert!(pair.contains("``ConfigAndJson(config, json)``"), "{pair}");
        let d = docstring(&f);
        assert!(d.starts_with("    \"\"\"Do it.\n\n    Args:\n"), "{d}");
        assert!(d.contains("source: Keep or clear. ``UNSET`` leaves it, ``None`` clears it."));
        assert!(d.contains("``None`` omits it. Library default when omitted: ``\"No\"``."));
        assert!(d.contains("class_: ``str``. Wire name ``class``."), "{d}");
        assert!(
            d.contains("``DoItResult(config, plan_id, was_created)``"),
            "{d}"
        );
        assert!(d.contains("record member's JSON text"), "{d}");
        assert!(d.contains("NOT_FOUND"), "{d}");
        assert!(d.contains("A \\\"\\\"\\\" trap \\\\ here."), "{d}");
        assert!(d.ends_with("    \"\"\"\n"));
        let unit = docstring(&func(vec![], Returns::Unit));
        assert!(!unit.contains("Returns:"), "{unit}");
    }

    #[test]
    fn test_named_records() {
        let mut f = func(vec![arg("code", ArgType::Str)], Returns::ConfigAndJson);
        f.name = "set_generic_plan".into();
        f.tuple_names = vec!["plan_id".into(), "was_created".into()];
        assert_eq!(record_name(&f), "SetGenericPlanResult");
        let class = record_class(&f);
        assert!(
            class.starts_with("class SetGenericPlanResult(NamedTuple):\n"),
            "{class}"
        );
        let fields: Vec<&str> = class
            .lines()
            .filter(|l| l.ends_with(": str"))
            .map(str::trim)
            .collect();
        assert_eq!(fields, ["config: str", "plan_id: str", "was_created: str"]);
        let body = function(&f, Flavor::Module);
        assert!(body.contains(") -> SetGenericPlanResult:"), "{body}");
        assert!(
            body.contains(
                "_core.call_config_and_record(\n        \"set_generic_plan\",\n        config_json,\n        {\n            \"code\": code,\n        },\n        SetGenericPlanResult,\n        (\n            \"plan_id\",\n            \"was_created\",\n        ),\n    )\n"
            ),
            "{body}"
        );
        f.returns = Returns::Json;
        f.name = "verify_x".into();
        assert!(!record_class(&f).contains("config: str"));
        assert!(function(&f, Flavor::Module).contains("_core.call_json_record("));
        assert!(function(&f, Flavor::Stub).contains(") -> VerifyXResult:"));
        // Without tuple_names: the plain mapping, and no record type.
        f.tuple_names.clear();
        assert!(!has_record(&f));
        assert_eq!(returns(&f), ("str".to_string(), "call_json"));
        let header_only = header(&[&func(vec![], Returns::ConfigAndJson)], Flavor::Module);
        assert!(!header_only.contains("typing"), "{header_only}");
        assert!(
            header_only.contains("import ConfigAndJson"),
            "{header_only}"
        );
    }

    #[test]
    fn test_wrap() {
        assert_eq!(wrap("a bb ccc", 4), ["a bb", "ccc"]);
        assert_eq!(wrap("  ", 4), Vec::<String>::new());
        assert_eq!(wrap("toolongword x", 4), ["toolongword", "x"]);
        let mut out = String::new();
        push_wrapped(&mut out, 8, &"word ".repeat(40));
        for line in out.lines() {
            assert!(line.chars().count() <= LINE_WIDTH, "{line}");
        }
        assert!(out.lines().nth(1).unwrap().starts_with("            word"));
        let mut prose = String::new();
        push_prose(&mut prose, 8, &"word ".repeat(40));
        assert!(
            prose.lines().all(|l| l.starts_with("        word")),
            "{prose}"
        );
    }

    #[test]
    fn test_generate_real_manifest() {
        let inputs = real_inputs();
        let out = generate(&inputs);
        assert_eq!(out.len(), 4);
        let py = &out[0].contents;
        let pyi = &out[1].contents;
        for f in &inputs.functions {
            let def = format!("\ndef {}(\n", f.name);
            let expected = f.status == Status::Implemented;
            assert_eq!(py.contains(&def), expected, "{}", f.name);
            assert_eq!(pyi.contains(&def), expected, "{}", f.name);
        }
        for g in &out {
            assert!(g.contents.ends_with('\n'), "{}", g.path.display());
            assert!(!g.contents.ends_with("\n\n"), "{}", g.path.display());
            for line in g.contents.lines() {
                assert!(line.chars().count() <= LINE_WIDTH + 40, "{line}");
                assert_eq!(
                    line.trim_end(),
                    line,
                    "trailing space in {}",
                    g.path.display()
                );
            }
        }
        assert_eq!(generate(&inputs), out, "deterministic");
    }

    #[test]
    fn test_init_reexports_sorted_isort_style() {
        let f = func(vec![], Returns::Config);
        let mut rec = func(vec![], Returns::Json);
        rec.name = "a_rec".into();
        rec.tuple_names = vec!["x".into(), "y".into()];
        let with_record = init_module(&[&f, &rec], "generated");
        assert!(
            with_record.contains("    REASON_CODES,\n    ARecResult,\n    a_rec,\n    do_it,\n"),
            "{with_record}"
        );
        assert!(
            with_record.contains("    \"ARecResult\",\n"),
            "{with_record}"
        );
        let init = init_module(&[&f], "generated");
        assert!(
            init.contains(
                "from ._core import (\n    UNSET,\n    ConfigAndJson,\n    Invocation,\n    UnsetType,\n    __version__,\n    abi_version,\n    invoke,\n    library_version,\n)\n"
            )
        );
        assert!(init.contains("from .generated import (\n    REASON_CODES,\n    do_it,\n)\n"));
        let all = init.split("__all__ = [").nth(1).unwrap();
        assert!(
            all.starts_with(
                "\n    \"REASON_CODES\",\n    \"UNSET\",\n    \"ConfigAndJson\",\n    \"Invocation\",\n    \"SzConfigToolError\",\n    \"UnsetType\",\n    \"__version__\",\n    \"abi_version\",\n    \"do_it\",\n    \"invoke\",\n    \"library_version\",\n]"
            ),
            "{all}"
        );
        assert_eq!(isort_key("A"), (1, "A".into()));
    }

    #[test]
    fn test_paths_module_uses_project_paths() {
        let inputs = real_inputs();
        let g = paths_module(&inputs);
        let p = &inputs.project.paths;
        assert!(g.contents.contains(&p.conformance_json_out));
        assert!(g.contents.contains(&p.fixture));
        assert!(g.contents.contains(&p.manifest_json_out));
        // bindings/python/tests/<file>: parents[3] is the workspace root.
        assert!(g.contents.contains(".parents[3]"), "{}", g.contents);
    }
}
