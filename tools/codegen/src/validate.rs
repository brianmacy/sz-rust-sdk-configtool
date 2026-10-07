//! Structural validation of the loaded inputs. Every problem is collected so a
//! manifest author sees all of them at once.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::load::Inputs;
use crate::model::{Arg, ArgType, Case, Excluded, Function, JsonType, Returns, Status, Step};

const MISSING_FIELD: &str = "MISSING_FIELD";
const NOT_IMPLEMENTED: &str = "NOT_IMPLEMENTED";

fn is_snake(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// `module::fn` (two or more snake segments).
fn is_fn_path(s: &str) -> bool {
    let segs: Vec<&str> = s.split("::").collect();
    segs.len() >= 2 && segs.iter().all(|seg| is_snake(seg))
}

/// `module::Struct` (snake modules, CamelCase last segment).
fn is_type_path(s: &str) -> bool {
    // At least one module segment before the type name.
    let Some((modules, last)) = s.rsplit_once("::") else {
        return false;
    };
    modules.split("::").all(is_snake)
        && last.chars().next().is_some_and(|c| c.is_ascii_uppercase())
        && last.chars().all(|c| c.is_ascii_alphanumeric())
}

fn check_arg(f: &Function, a: &Arg, errs: &mut Vec<String>) {
    let at = format!("{}.{}", f.name, a.name);
    if !is_snake(&a.name) {
        errs.push(format!("{at}: arg name must be snake_case"));
    }
    if a.tristate && !matches!(a.ty, ArgType::Str | ArgType::Int) {
        errs.push(format!("{at}: tristate is only supported for str and int"));
    }
    if a.required && (!a.optional || a.tristate) {
        errs.push(format!(
            "{at}: required needs optional: true and no tristate (a library-required Option arg)"
        ));
    }
    if a.required && a.default.is_some() {
        errs.push(format!("{at}: required and default are exclusive"));
    }
    if a.owned && a.ty != ArgType::Json {
        errs.push(format!("{at}: owned is only valid for type json"));
    }
    if a.ty == ArgType::IntOrStr && a.rust_convert.is_none() {
        errs.push(format!(
            "{at}: int_or_str needs rust_convert (no plain Rust mapping)"
        ));
    }
    if a.owned && a.rust_convert.is_some() {
        errs.push(format!("{at}: owned and rust_convert are exclusive"));
    }
    if let Some(conv) = &a.rust_convert
        && !is_snake(conv)
    {
        errs.push(format!(
            "{at}: rust_convert must be a snake_case fn name in sz_configtool_api::convert"
        ));
    }
    let has_struct = f.rust_params_struct.is_some();
    if !has_struct && (a.positional || a.field.is_some()) {
        errs.push(format!("{at}: positional/field require rust_params_struct"));
    }
    if a.positional && a.field.is_some() {
        errs.push(format!("{at}: positional and field are exclusive"));
    }
    if let Some(field) = &a.field
        && !is_snake(field)
    {
        errs.push(format!("{at}: field must be snake_case"));
    }
    match (&a.json_type, a.ty) {
        (None, ArgType::Json) => errs.push(format!(
            "{at}: json args need json_type (use `any` for free-form JSON)"
        )),
        (Some(_), ty) if ty != ArgType::Json => {
            errs.push(format!("{at}: json_type is only valid for type json"))
        }
        (Some(ty), _) => check_json_type(&at, &a.name, ty, errs),
        (None, _) => {}
    }
}

/// Structural rules of a `json_type` descriptor; `path` names the position
/// (`arg[]` = array item, `arg.field`, `arg<i>` = one_of alternative).
fn check_json_type(at: &str, path: &str, ty: &JsonType, errs: &mut Vec<String>) {
    let mut err = |msg: String| errs.push(format!("{at}: json_type {path}: {msg}"));
    match ty {
        JsonType::Any | JsonType::String | JsonType::Int | JsonType::Bool => {}
        JsonType::Enum(values) => {
            if values.is_empty() {
                err("enum needs at least one value".into());
            }
            let mut seen = BTreeSet::new();
            for v in values.iter().filter(|v| !seen.insert(v.as_str())) {
                err(format!("enum value '{v}' is repeated"));
            }
        }
        JsonType::Array(item) => check_json_type(at, &format!("{path}[]"), item, errs),
        JsonType::Object(fields) => {
            if fields.is_empty() {
                err("object needs at least one field".into());
            }
            let mut seen = BTreeSet::new();
            for f in fields {
                if f.name.is_empty() {
                    err("empty field name".into());
                } else if !seen.insert(f.name.as_str()) {
                    err(format!("field '{}' is repeated", f.name));
                }
            }
            for f in fields {
                check_json_type(at, &format!("{path}.{}", f.name), &f.ty, errs);
            }
        }
        JsonType::OneOf(alts) => {
            if alts.len() < 2 {
                err("one_of needs at least two alternatives".into());
            }
            let kinds: Vec<_> = alts.iter().map(JsonType::kind).collect();
            if kinds.contains(&None) {
                err("one_of cannot contain any or a nested one_of".into());
            }
            let distinct: BTreeSet<_> = kinds.iter().flatten().collect();
            if distinct.len() != kinds.iter().flatten().count() {
                err("one_of alternatives must differ in JSON kind".into());
            }
            for (i, alt) in alts.iter().enumerate() {
                check_json_type(at, &format!("{path}<{i}>"), alt, errs);
            }
        }
    }
}

/// Phrases that mark C-ABI-specific text (typed C exports, numeric return
/// codes). Such text belongs in `c_notes`, never in the language-neutral
/// `doc` / `notes` / `semantics` that every binding renders.
const C_ABI_MARKERS: &[&str] = &[
    "SzConfigTool_",
    "C export",
    "C typed",
    "typed C",
    "C-export",
    "C wrapper",
    "C ABI",
    "C NULL",
    "legacy C",
    "returnCode",
];

fn check_c_abi_free(at: &str, field: &str, text: Option<&str>, errs: &mut Vec<String>) {
    let Some(text) = text else { return };
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(m) = C_ABI_MARKERS.iter().find(|m| flat.contains(*m)) {
        errs.push(format!(
            "{at}: {field} mentions '{m}'; move C-ABI text to c_notes"
        ));
    }
}

fn check_function(f: &Function, reason_codes: &BTreeSet<&str>, errs: &mut Vec<String>) {
    let n = &f.name;
    if !is_snake(n) {
        errs.push(format!("{n}: name must be snake_case"));
    }
    if f.doc.trim().is_empty() {
        errs.push(format!("{n}: doc is required"));
    }
    if !is_fn_path(&f.rust) {
        errs.push(format!("{n}: rust '{}' must be module::fn", f.rust));
    }
    if let Some(s) = &f.rust_params_struct
        && !is_type_path(s)
    {
        errs.push(format!(
            "{n}: rust_params_struct '{s}' must be module::Type"
        ));
    }
    if f.requires_options && !f.args.iter().any(|a| a.is_optional() && !a.required) {
        errs.push(format!("{n}: requires_options needs an optional argument"));
    }
    check_c_abi_free(n, "doc", Some(&f.doc), errs);
    check_c_abi_free(n, "notes", f.notes.as_deref(), errs);
    let mut arg_names = BTreeSet::new();
    for a in &f.args {
        check_c_abi_free(
            &format!("{n}.{}", a.name),
            "semantics",
            a.semantics.as_deref(),
            errs,
        );
        if !arg_names.insert(a.name.as_str()) {
            errs.push(format!("{n}: duplicate arg '{}'", a.name));
        }
        check_arg(f, a, errs);
    }
    let tuple_return = matches!(f.returns, Returns::ConfigAndJson | Returns::Json);
    if !f.tuple_names.is_empty() && (!tuple_return || f.tuple_names.len() < 2) {
        errs.push(format!(
            "{n}: tuple_names needs returns config_and_json or json and >= 2 names"
        ));
    }
    if f.status == Status::NotImplemented && !f.errors.iter().any(|e| e == NOT_IMPLEMENTED) {
        errs.push(format!(
            "{n}: status not_implemented must list NOT_IMPLEMENTED in errors"
        ));
    }
    let mut seen = BTreeSet::new();
    for e in &f.errors {
        if !reason_codes.contains(e.as_str()) {
            errs.push(format!("{n}: unknown reason code '{e}'"));
        }
        if !seen.insert(e) {
            errs.push(format!("{n}: duplicate error '{e}'"));
        }
    }
}

fn check_functions(inputs: &Inputs, errs: &mut Vec<String>) {
    let codes: BTreeSet<&str> = inputs
        .project
        .reason_codes
        .iter()
        .map(String::as_str)
        .collect();
    let mut names = BTreeSet::new();
    let mut symbols = BTreeSet::new();
    let mut rust_paths = BTreeSet::new();
    for f in &inputs.functions {
        check_function(f, &codes, errs);
        if !names.insert(f.name.as_str()) {
            errs.push(format!("{}: duplicate function name", f.name));
        }
        if !rust_paths.insert(f.rust.as_str()) {
            errs.push(format!("{}: rust path '{}' mapped twice", f.name, f.rust));
        }
        for sym in f.c_symbol.iter().chain(&f.c_aliases) {
            if !sym.starts_with("SzConfigTool_") {
                errs.push(format!(
                    "{}: C symbol '{sym}' must start with SzConfigTool_",
                    f.name
                ));
            }
            if !symbols.insert(sym.as_str()) {
                errs.push(format!("{}: C symbol '{sym}' used twice", f.name));
            }
        }
    }
    for f in &inputs.functions {
        if let Some(c) = f.companion().filter(|c| names.contains(c.as_str())) {
            errs.push(format!(
                "{c}: collides with the typed companion of {}",
                f.name
            ));
        }
    }
}

fn check_excluded(ex: &Excluded, errs: &mut Vec<String>) {
    for e in &ex.entries {
        match (&e.rust, &e.module) {
            (Some(r), None) if is_fn_path(r) => {}
            (None, Some(m)) if m.split("::").all(is_snake) => {}
            _ => errs.push(format!(
                "excluded: each entry needs exactly one of a valid `rust` or `module` ({e:?})"
            )),
        }
        if e.reason.trim().is_empty() {
            errs.push(format!("excluded: entry {e:?} needs a reason"));
        }
    }
}

/// Does `v` have the JSON shape of wire type `ty`?
fn value_matches(ty: ArgType, v: &Value) -> bool {
    match ty {
        ArgType::Str => v.is_string(),
        ArgType::Int => v.is_i64(),
        ArgType::Bool => v.is_boolean(),
        ArgType::Json => true,
        ArgType::StrList => v.as_array().is_some_and(|a| a.iter().all(Value::is_string)),
        ArgType::IntOrStr => v.is_i64() || v.is_string(),
    }
}

fn check_step_args(at: &str, f: &Function, step: &Step, errs: &mut Vec<String>) {
    for (key, v) in &step.args {
        match f.arg(key) {
            None => errs.push(format!("{at}: unknown arg '{key}' for {}", f.name)),
            Some(a) if v.is_null() && !a.tristate => {
                errs.push(format!(
                    "{at}: null is only valid for tristate args ('{key}')"
                ));
            }
            Some(a) if !v.is_null() && !value_matches(a.ty, v) => {
                errs.push(format!("{at}: arg '{key}' must be {}", a.ty.as_str()))
            }
            Some(_) => {}
        }
    }
    for a in f.args.iter().filter(|a| !a.is_optional()) {
        if !step.args.contains_key(&a.name) {
            errs.push(format!("{at}: missing required arg '{}'", a.name));
        }
    }
    let tests_missing_field = step.expect.error.as_deref() == Some(MISSING_FIELD);
    for a in omitted_required(f, step) {
        if !tests_missing_field {
            errs.push(format!(
                "{at}: omits required arg '{}' (only valid in a step expecting MISSING_FIELD)",
                a.name
            ));
        }
    }
}

/// Does the step pass a `json` arg value outside the arg's `json_type`?
fn untypeable_json(f: &Function, step: &Step) -> bool {
    step.args.iter().any(|(key, v)| {
        f.arg(key)
            .and_then(|a| a.json_type.as_ref())
            .is_some_and(|ty| !ty.matches(v))
    })
}

/// `required: true` args a step leaves out.
fn omitted_required<'f>(f: &'f Function, step: &Step) -> impl Iterator<Item = &'f Arg> {
    f.args
        .iter()
        .filter(move |a| a.required && !step.args.contains_key(&a.name))
}

fn check_step_expect(at: &str, f: &Function, step: &Step, errs: &mut Vec<String>) {
    let ex = &step.expect;
    if let Some(code) = &ex.error {
        if !f.errors.contains(code) {
            errs.push(format!(
                "{at}: error '{code}' is not listed in {}.errors",
                f.name
            ));
        }
        if ex.has_success_checks() {
            errs.push(format!("{at}: `error` excludes every other expectation"));
        }
    }
    if f.status == Status::NotImplemented && ex.error.as_deref() != Some(NOT_IMPLEMENTED) {
        errs.push(format!(
            "{at}: a not_implemented function must expect NOT_IMPLEMENTED"
        ));
    }
    if let Some(kind) = ex.kind
        && kind != f.returns
    {
        errs.push(format!(
            "{at}: kind '{}' but {} returns '{}'",
            kind.as_str(),
            f.name,
            f.returns.as_str()
        ));
    }
    let inspects_result =
        ex.result.is_some() || ex.contains.is_some() || ex.excludes.is_some() || ex.len.is_some();
    if inspects_result && matches!(f.returns, Returns::Config | Returns::Unit) {
        errs.push(format!(
            "{at}: a config result is opaque; check it with a later get/list step"
        ));
    }
    // `len`/`contains`/`excludes` inspect an ARRAY. Only a plain `json`
    // result can be one: an `int`, a `config_and_json` record, or a
    // `tuple_names` object would make them pass (or fail) meaninglessly.
    let inspects_array = ex.contains.is_some() || ex.excludes.is_some() || ex.len.is_some();
    let may_be_array = f.returns == Returns::Json && f.tuple_names.is_empty();
    if inspects_array && !may_be_array && !matches!(f.returns, Returns::Config | Returns::Unit) {
        errs.push(format!(
            "{at}: len/contains/excludes need an array result; {} returns an object or scalar (use `result`)",
            f.name
        ));
    }
}

fn check_cases(cases: &[Case], by_name: &BTreeMap<&str, &Function>, errs: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for case in cases {
        let id = format!("{}/{}", case.group, case.name);
        if !seen.insert(id.clone()) {
            errs.push(format!("{id}: duplicate case name"));
        }
        if case.steps.is_empty() {
            errs.push(format!("{id}: no steps"));
        }
        for (i, step) in case.steps.iter().enumerate() {
            let at = format!("{id} step {i}");
            match by_name.get(step.func.as_str()) {
                None => errs.push(format!("{at}: unknown fn '{}'", step.func)),
                Some(f) => {
                    check_step_args(&at, f, step, errs);
                    check_step_expect(&at, f, step, errs);
                }
            }
        }
    }
}

/// Mark every conformance step that omits a `required: true` arg, or passes
/// a `json` arg value that does not fit its `json_type`, as `wire_only` (see
/// `model::Step::wire_only`): a typed binding cannot express it. Run after
/// [`validate`].
pub fn mark_wire_only(inputs: &mut Inputs) {
    let by_name: BTreeMap<String, Function> = inputs
        .functions
        .iter()
        .map(|f| (f.name.clone(), f.clone()))
        .collect();
    for step in inputs.cases.iter_mut().flat_map(|c| c.steps.iter_mut()) {
        if let Some(f) = by_name.get(&step.func) {
            let omits = omitted_required(f, step).next().is_some();
            step.wire_only = omits || untypeable_json(f, step);
        }
    }
}

/// Validate everything; `Err` carries one line per problem.
pub fn validate(inputs: &Inputs) -> Result<(), Vec<String>> {
    let mut errs = Vec::new();
    check_functions(inputs, &mut errs);
    check_excluded(&inputs.excluded, &mut errs);
    let by_name: BTreeMap<&str, &Function> = inputs
        .functions
        .iter()
        .map(|f| (f.name.as_str(), f))
        .collect();
    check_cases(&inputs.cases, &by_name, &mut errs);
    if errs.is_empty() { Ok(()) } else { Err(errs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_shapes() {
        assert!(is_fn_path("datasources::add_data_source"));
        assert!(is_fn_path("calls::standardize::add_standardize_call"));
        assert!(!is_fn_path("add_data_source"));
        assert!(!is_fn_path("datasources::AddDataSource"));
        assert!(is_type_path("datasources::AddDataSourceParams"));
        assert!(!is_type_path("AddDataSourceParams"));
        assert!(!is_type_path("datasources::add"));
    }

    #[test]
    fn test_value_matches_wire_types() {
        assert!(value_matches(ArgType::Str, &Value::from("x")));
        assert!(!value_matches(ArgType::Str, &Value::from(1)));
        assert!(value_matches(ArgType::Int, &Value::from(-3)));
        assert!(!value_matches(ArgType::Int, &Value::from(1.5)));
        assert!(value_matches(
            ArgType::StrList,
            &serde_json::json!(["a", "b"])
        ));
        assert!(!value_matches(
            ArgType::StrList,
            &serde_json::json!(["a", 1])
        ));
        assert!(value_matches(ArgType::Bool, &Value::from(true)));
        assert!(value_matches(ArgType::IntOrStr, &Value::from(7)));
        assert!(value_matches(ArgType::IntOrStr, &Value::from("NAME")));
        assert!(!value_matches(ArgType::IntOrStr, &Value::from(1.5)));
        assert!(!value_matches(ArgType::IntOrStr, &Value::from(true)));
        assert!(!value_matches(
            ArgType::IntOrStr,
            &serde_json::json!(["NAME"])
        ));
    }
}
