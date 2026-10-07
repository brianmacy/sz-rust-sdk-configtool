//! Python native seam: ONE function, `sz_configtool._native.invoke`, plus
//! the `LIBRARY_VERSION` / `ABI_VERSION` constants from `sz-configtool-api`.
//!
//! It forwards to [`sz_configtool_api::invoke`] with the GIL released and
//! returns `(kind, config | None, result_json | None)`. Failures raise
//! `_native.NativeError(reason_code, message, details_json | None)`;
//! the pure-Python layer maps that to `SzConfigToolError`. The config string is
//! opaque: it is handed to and returned from the library untouched.
//!
//! This crate depends on `sz-configtool-api` only (never the C ABI crate), so
//! the extension exports no `SzConfigTool_*` symbols.

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyString;
use sz_configtool_api::{ApiError, Output};

create_exception!(
    _native,
    NativeError,
    PyException,
    "invoke failure: args = (reason_code, message, details_json | None)."
);

fn to_py_err(e: &ApiError) -> PyErr {
    NativeError::new_err((e.reason_code(), e.to_string(), e.details_json()))
}

type Invocation = (&'static str, Option<String>, Option<String>);

fn split(out: Output) -> Invocation {
    let result = out.result().map(|v| v.to_string());
    let kind = out.kind();
    let config = match out {
        Output::Config(c) | Output::ConfigAndJson { config: c, .. } => Some(c),
        _ => None,
    };
    (kind, config, result)
}

/// A `str` argument (`name`, `config`) as UTF-8, or `INVALID_INPUT` when it
/// cannot cross the boundary unchanged: not a `str`, or a `str` with a lone
/// surrogate (no UTF-8 form). Same reason code as the Java/TS/C# bindings
/// (CONTRACT.md), never a `TypeError` / `UnicodeEncodeError`.
fn text_arg<'a>(obj: &'a Bound<'_, PyAny>, param: &str) -> PyResult<std::borrow::Cow<'a, str>> {
    let invalid = |what: &str| {
        NativeError::new_err(("INVALID_INPUT", format!("{param} {what}"), None::<String>))
    };
    let s = obj
        .cast::<PyString>()
        .map_err(|_| invalid(&format!("must be a str, not {}", type_name(obj))))?;
    s.to_str()
        .map(std::borrow::Cow::Borrowed)
        .map_err(|_| invalid("is not valid Unicode (lone surrogate); it has no UTF-8 form"))
}

fn type_name(obj: &Bound<'_, PyAny>) -> String {
    obj.get_type()
        .name()
        .map(|n| n.to_string())
        .unwrap_or_else(|_| "?".to_owned())
}

/// Call manifest function `name` on `config` with `args_json` (a JSON object).
#[pyfunction]
fn invoke(
    py: Python<'_>,
    name: &Bound<'_, PyAny>,
    config: &Bound<'_, PyAny>,
    args_json: &str,
) -> PyResult<Invocation> {
    let name = text_arg(name, "name")?;
    let config = text_arg(config, "config")?;
    py.detach(|| sz_configtool_api::invoke(&name, &config, args_json))
        .map(split)
        .map_err(|e| to_py_err(&e))
}

#[pymodule(name = "_native")]
fn native_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Each step fails only on interpreter allocation failure during import;
    // the first failure is the import error.
    wrap_pyfunction!(invoke, m)
        .and_then(|f| m.add_function(f))
        // Single source: sz-configtool-api (the workspace version / C ABI version).
        .and_then(|()| m.add("LIBRARY_VERSION", sz_configtool_api::LIBRARY_VERSION))
        .and_then(|()| m.add("ABI_VERSION", sz_configtool_api::ABI_VERSION))
        .and_then(|()| m.add("NativeError", m.py().get_type::<NativeError>()))
}
