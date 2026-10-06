//! Fails when `include/libSzConfigTool.h` and the exported
//! `#[unsafe(no_mangle)] extern "C"` functions in `src/lib.rs` diverge: every
//! export must be declared exactly once (with `SZCONFIGTOOL_API`), every
//! declaration must be exported, and parameter/return types must agree.
//!
//! Deliberately a simple text parse with no extra dependencies; it relies on
//! the source being rustfmt-formatted and the header using one declaration per
//! statement.

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct Sig {
    ret: String,
    params: Vec<String>,
}

fn read(rel: &str) -> String {
    let path = format!("{}/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read '{path}': {e}"))
}

/// Normalize a Rust FFI type to its C spelling.
fn rust_to_c(ty: &str) -> String {
    match ty.trim() {
        "*const c_char" => "const char *".to_string(),
        "*mut c_char" => "char *".to_string(),
        "i64" => "int64_t".to_string(),
        "i32" => "int32_t".to_string(),
        "SzConfigTool_result" => "SzConfigTool_result".to_string(),
        "" | "()" => "void".to_string(),
        other => panic!("unmapped Rust FFI type '{other}'"),
    }
}

/// Normalize a C type: drop `struct`, and space every token (incl. `*`) once.
fn normalize_c(ty: &str) -> String {
    let ty = ty.replace("struct ", "").replace('*', " * ");
    ty.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn rust_exports(src: &str) -> BTreeMap<String, Sig> {
    let mut out = BTreeMap::new();
    for (idx, _) in src.match_indices("#[unsafe(no_mangle)]") {
        let rest = &src[idx..];
        let fn_pos = rest
            .find("extern \"C\" fn ")
            .expect("extern fn after no_mangle");
        let rest = &rest[fn_pos + "extern \"C\" fn ".len()..];
        let open = rest.find('(').unwrap();
        let name = rest[..open].trim().to_string();
        let close = rest.find(')').unwrap();
        let params = rest[open + 1..close]
            .lines()
            .map(|l| l.split_once("//").map_or(l, |(code, _)| code))
            .collect::<String>()
            .split(',')
            .filter(|p| !p.trim().is_empty())
            .map(|p| rust_to_c(p.split_once(':').expect("name: type").1))
            .map(|t| normalize_c(&t))
            .collect();
        let body = rest.find('{').unwrap();
        let ret = rest[close + 1..body].trim().trim_start_matches("->");
        let sig = Sig {
            ret: normalize_c(&rust_to_c(ret)),
            params,
        };
        assert!(
            out.insert(name.clone(), sig).is_none(),
            "{name} exported twice"
        );
    }
    out
}

fn strip_c_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(i) = rest.find(['/']) {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        if let Some(t) = tail.strip_prefix("/*") {
            rest = t.split_once("*/").map_or("", |(_, r)| r);
        } else if let Some(t) = tail.strip_prefix("//") {
            rest = t.split_once('\n').map_or("", |(_, r)| r);
            out.push('\n');
        } else {
            out.push('/');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

/// Returns (declarations, names declared without SZCONFIGTOOL_API).
fn header_decls(header: &str) -> (BTreeMap<String, Sig>, Vec<String>) {
    let code = strip_c_comments(header);
    let mut decls = BTreeMap::new();
    let mut missing_api = Vec::new();
    for stmt in code.split(';') {
        let stmt = stmt
            .lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join(" ");
        if stmt.contains("typedef") {
            continue;
        }
        // The declared name is the identifier immediately before the first '('.
        let Some(open) = stmt.find('(') else {
            continue;
        };
        let before = stmt[..open].trim_end();
        let name_pos = before
            .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
            .map_or(0, |p| p + 1);
        let name = before[name_pos..].to_string();
        if !name.starts_with("SzConfigTool_") {
            continue;
        }
        let close = stmt.rfind(')').unwrap();
        let mut prefix = stmt[..name_pos].trim().to_string();
        // Tolerate `extern "C" {` and similar wrappers before the first decl.
        if let Some(brace) = prefix.rfind(['{', '}']) {
            prefix = prefix[brace + 1..].trim().to_string();
        }
        match prefix.strip_prefix("SZCONFIGTOOL_API") {
            Some(ret) => prefix = ret.trim().to_string(),
            None => missing_api.push(name.clone()),
        }
        let params: Vec<String> = stmt[open + 1..close]
            .split(',')
            .map(str::trim)
            .filter(|p| !p.is_empty() && *p != "void")
            .map(|p| {
                // Drop the parameter name (last identifier).
                let ty = p.trim_end_matches(|c: char| c.is_alphanumeric() || c == '_');
                normalize_c(ty)
            })
            .collect();
        let sig = Sig {
            ret: normalize_c(&prefix),
            params,
        };
        assert!(
            decls.insert(name.clone(), sig).is_none(),
            "{name} declared more than once in the header"
        );
    }
    (decls, missing_api)
}

#[test]
fn test_header_matches_exports() {
    let exports = rust_exports(&read("src/lib.rs"));
    let (decls, missing_api) = header_decls(&read("include/libSzConfigTool.h"));
    assert!(exports.len() > 100, "parsed only {} exports", exports.len());

    let undeclared: Vec<_> = exports.keys().filter(|k| !decls.contains_key(*k)).collect();
    let unimplemented: Vec<_> = decls.keys().filter(|k| !exports.contains_key(*k)).collect();
    assert!(
        undeclared.is_empty() && unimplemented.is_empty(),
        "header/export drift\n  exported but not declared: {undeclared:?}\n  declared but not exported: {unimplemented:?}"
    );
    assert!(
        missing_api.is_empty(),
        "declarations missing SZCONFIGTOOL_API: {missing_api:?}"
    );
    let mismatched: Vec<_> = exports
        .iter()
        .filter(|(name, sig)| decls[*name] != **sig)
        .map(|(name, sig)| format!("{name}: rust {sig:?} vs header {:?}", decls[name]))
        .collect();
    assert!(
        mismatched.is_empty(),
        "signature drift:\n{}",
        mismatched.join("\n")
    );
}
