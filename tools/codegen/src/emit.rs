//! Renders the generated outputs: the Rust dispatcher and the two JSON files.

use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};

use serde_json::json;

use crate::load::Inputs;
use crate::model::{Arg, ArgType, Function, Returns, Status};

/// Banner placed at the top of every generated file.
pub const GENERATED_BANNER: &str = "GENERATED — do not edit. Source: api/manifest/*.yaml; \
regenerate with `cargo run -p sz-configtool-codegen`.";

/// The `Args` accessor that yields this arg's Rust value.
fn accessor(a: &Arg) -> String {
    if let Some(conv) = &a.rust_convert {
        return format!("crate::convert::{conv}(args, \"{}\")?", a.name);
    }
    let mode = if a.tristate {
        "tri"
    } else if a.optional {
        "opt"
    } else {
        "req"
    };
    let ty = match (a.ty, a.owned) {
        (ArgType::Json, true) => "json_owned",
        (ty, _) => ty.as_str(),
    };
    format!("args.{mode}_{ty}(\"{}\")?", a.name)
}

/// The call expression `<crate>::<rust>(config, ...)`.
fn call_expr(root: &str, f: &Function) -> String {
    let mut params = vec!["config".to_string()];
    let in_struct = |a: &&Arg| f.rust_params_struct.is_some() && !a.positional;
    for a in f.args.iter().filter(|a| !in_struct(a)) {
        params.push(accessor(a));
    }
    if let Some(st) = &f.rust_params_struct {
        let mut lit = format!("{root}::{st} {{\n");
        for a in f.args.iter().filter(in_struct) {
            let field = a.field.as_deref().unwrap_or(&a.name);
            let _ = writeln!(lit, "            {field}: {},", accessor(a));
        }
        lit.push_str("        }");
        params.push(lit);
    }
    let joined = params
        .iter()
        .map(|p| format!("        {p},\n"))
        .collect::<String>();
    format!("{root}::{}(\n{joined}    )", f.rust)
}

/// Statements converting `result` into an `Output`.
fn return_stmt(f: &Function) -> String {
    match f.returns {
        Returns::Config => "    Ok(Output::Config(result))\n".to_string(),
        Returns::Json if f.tuple_names.is_empty() => "    crate::output::json(result)\n".to_string(),
        Returns::Json => {
            let (vars, fields) = tuple_record(f);
            format!(
                "    let ({vars}) = result;\n    crate::output::json(serde_json::json!({{ {fields} }}))\n"
            )
        }
        Returns::Int => "    Ok(Output::Int(result))\n".to_string(),
        Returns::Unit => "    Ok(Output::Unit)\n".to_string(),
        Returns::ConfigAndJson if f.tuple_names.is_empty() => {
            "    let (config, record) = result;\n    crate::output::config_and_json(config, record)\n"
                .to_string()
        }
        Returns::ConfigAndJson => {
            let (vars, fields) = tuple_record(f);
            format!(
                "    let (config, {vars}) = result;\n    crate::output::config_and_json(config, serde_json::json!({{ {fields} }}))\n"
            )
        }
    }
}

/// For `tuple_names`: the tuple bindings (`t0, t1`) and the record fields
/// (`"name0": t0, "name1": t1`).
fn tuple_record(f: &Function) -> (String, String) {
    let vars: Vec<String> = (0..f.tuple_names.len()).map(|i| format!("t{i}")).collect();
    let fields: Vec<String> = f
        .tuple_names
        .iter()
        .zip(&vars)
        .map(|(n, v)| format!("\"{n}\": {v}"))
        .collect();
    (vars.join(", "), fields.join(", "))
}

fn handler(root: &str, f: &Function) -> String {
    let known: Vec<String> = f.args.iter().map(|a| format!("\"{}\"", a.name)).collect();
    if f.status == Status::NotImplemented {
        // The library call always fails (NOT_IMPLEMENTED), so there is no
        // success conversion to emit: the shared helper returns its error.
        return format!(
            "fn call_{name}(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {{\n    \
             args.check_known(&[{known}])?;\n    \
             crate::output::not_implemented(\"{name}\", {call})\n}}\n",
            name = f.name,
            known = known.join(", "),
            call = call_expr(root, f),
        );
    }
    let binding = if f.returns == Returns::Unit {
        String::new()
    } else {
        "let result = ".to_string()
    };
    format!(
        "fn call_{name}(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {{\n    \
         args.check_known(&[{known}])?;\n    {binding}{call}?;\n{ret}}}\n",
        name = f.name,
        known = known.join(", "),
        call = call_expr(root, f),
        ret = return_stmt(f),
    )
}

/// Relative path from directory `from_dir` to file `to` (both workspace-relative).
fn relative(from_dir: &Path, to: &Path) -> PathBuf {
    let from: Vec<Component> = from_dir.components().collect();
    let to_c: Vec<Component> = to.components().collect();
    let common = from.iter().zip(&to_c).take_while(|(a, b)| a == b).count();
    let mut out = PathBuf::new();
    for _ in common..from.len() {
        out.push("..");
    }
    for c in &to_c[common..] {
        out.push(c.as_os_str());
    }
    out
}

/// The generated dispatcher module (`paths.dispatch_out`).
pub fn dispatch_rs(inputs: &Inputs) -> String {
    let p = &inputs.project.paths;
    let root = &inputs.project.root_crate;
    let dispatch_dir = Path::new(&p.dispatch_out).parent().unwrap_or(Path::new(""));
    let manifest_rel = relative(dispatch_dir, Path::new(&p.manifest_json_out));
    let mut out = format!(
        "// {GENERATED_BANNER}\n\n\
         use crate::args::Args;\nuse crate::output::Output;\nuse crate::ApiError;\n\n\
         /// Signature shared by every generated handler.\n\
         pub(crate) type Handler = fn(&str, &Args<'_>) -> Result<Output, ApiError>;\n\n\
         /// The generated manifest (functions, args, errors, paths) as JSON.\n\
         pub const MANIFEST_JSON: &str = include_str!(\"{}\");\n\n\
         /// Every function name `invoke` dispatches, in manifest order.\n\
         pub const FUNCTION_NAMES: &[&str] = &[\n",
        manifest_rel.to_string_lossy().replace('\\', "/")
    );
    for f in &inputs.functions {
        let _ = writeln!(out, "    \"{}\",", f.name);
    }
    out.push_str("];\n\n/// Resolve a function name to its handler.\n");
    out.push_str("pub(crate) fn lookup(name: &str) -> Option<Handler> {\n    match name {\n");
    for f in &inputs.functions {
        let _ = writeln!(out, "        \"{0}\" => Some(call_{0}),", f.name);
    }
    out.push_str("        _ => None,\n    }\n}\n");
    for f in &inputs.functions {
        out.push('\n');
        out.push_str(&handler(root, f));
    }
    out
}

fn pretty(value: &serde_json::Value) -> String {
    let mut s = serde_json::to_string_pretty(value).unwrap_or_default();
    s.push('\n');
    s
}

/// `paths.manifest_json_out`.
pub fn manifest_json(inputs: &Inputs) -> String {
    pretty(&json!({
        "generated": GENERATED_BANNER,
        "root_crate": inputs.project.root_crate,
        "paths": inputs.project.paths,
        "reason_codes": inputs.project.reason_codes,
        "functions": inputs.functions,
        "excluded": inputs.excluded,
    }))
}

/// `paths.conformance_json_out`.
pub fn conformance_json(inputs: &Inputs) -> String {
    pretty(&json!({
        "generated": GENERATED_BANNER,
        "fixture": inputs.project.paths.fixture,
        "cases": inputs.cases,
    }))
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
            doc: "d".into(),
            rust: "m::do_it".into(),
            rust_params_struct: None,
            c_symbol: None,
            c_aliases: vec![],
            args,
            returns,
            tuple_names: vec![],
            errors: vec![],
            notes: None,
            c_notes: None,
            status: crate::model::Status::Implemented,
        }
    }

    #[test]
    fn test_accessor_modes() {
        let mut a = arg("x", ArgType::Str);
        assert_eq!(accessor(&a), "args.req_str(\"x\")?");
        a.optional = true;
        assert_eq!(accessor(&a), "args.opt_str(\"x\")?");
        a.tristate = true;
        assert_eq!(accessor(&a), "args.tri_str(\"x\")?");
        let mut j = arg("v", ArgType::Json);
        j.owned = true;
        assert_eq!(accessor(&j), "args.req_json_owned(\"v\")?");
        let mut c = arg("sel", ArgType::Json);
        c.rust_convert = Some("call_selector".into());
        assert_eq!(
            accessor(&c),
            "crate::convert::call_selector(args, \"sel\")?"
        );
    }

    #[test]
    fn test_positional_args_precede_params_struct() {
        let mut code = arg("code", ArgType::Str);
        code.positional = true;
        let mut src = arg("source", ArgType::Str);
        src.tristate = true;
        let mut desc = arg("desc", ArgType::Str);
        desc.optional = true;
        desc.field = Some("description".into());
        let mut f = func(vec![code, src, desc], Returns::Config);
        f.rust_params_struct = Some("m::Params".into());
        let call = call_expr("lib", &f);
        let code_at = call.find("args.req_str(\"code\")").unwrap();
        let struct_at = call.find("lib::m::Params {").unwrap();
        assert!(code_at < struct_at, "{call}");
        assert!(
            call.contains("source: args.tri_str(\"source\")?,"),
            "{call}"
        );
        assert!(
            call.contains("description: args.opt_str(\"desc\")?,"),
            "{call}"
        );
    }

    #[test]
    fn test_tuple_names_build_record_object() {
        let mut f = func(vec![], Returns::ConfigAndJson);
        f.tuple_names = vec!["plan_id".into(), "created".into()];
        let stmt = return_stmt(&f);
        assert!(stmt.contains("let (config, t0, t1) = result;"), "{stmt}");
        assert!(stmt.contains("\"plan_id\": t0, \"created\": t1"), "{stmt}");
    }

    #[test]
    fn test_tuple_names_on_json_build_named_result() {
        let mut f = func(vec![], Returns::Json);
        assert_eq!(return_stmt(&f), "    crate::output::json(result)\n");
        f.tuple_names = vec!["current_version".into(), "matches".into()];
        let stmt = return_stmt(&f);
        assert!(stmt.contains("let (t0, t1) = result;"), "{stmt}");
        assert!(
            stmt.contains("serde_json::json!({ \"current_version\": t0, \"matches\": t1 })"),
            "{stmt}"
        );
    }

    #[test]
    fn test_unit_return_discards_value() {
        let h = handler("lib", &func(vec![], Returns::Unit));
        assert!(!h.contains("let result"), "{h}");
        assert!(h.contains("Ok(Output::Unit)"), "{h}");
    }

    #[test]
    fn test_not_implemented_handler_returns_library_error_via_helper() {
        let mut f = func(vec![arg("code", ArgType::Str)], Returns::ConfigAndJson);
        f.status = Status::NotImplemented;
        let h = handler("lib", &f);
        assert!(
            h.contains("crate::output::not_implemented(\"do_it\", lib::m::do_it(\n"),
            "{h}"
        );
        assert!(h.contains("args.req_str(\"code\")?,"), "{h}");
        assert!(!h.contains("let result"), "{h}");
    }

    #[test]
    fn test_relative_path() {
        assert_eq!(
            relative(
                Path::new("api/src"),
                Path::new("api/manifest/generated/m.json")
            ),
            PathBuf::from("../manifest/generated/m.json")
        );
    }
}
