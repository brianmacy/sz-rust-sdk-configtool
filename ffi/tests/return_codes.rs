//! Fails when the return-code documentation in `include/libSzConfigTool.h`
//! ("Return codes") no longer matches what the exports in `src/lib.rs` do.
//!
//! The typed exports do not share one numbering (they were added in waves);
//! the header documents the actual families and lists the exports of each
//! irregular family. This test recomputes those lists from the source with a
//! plain text parse (rustfmt-formatted source, no extra dependencies) and
//! compares them with the header's lists, so neither can drift.

use std::collections::BTreeSet;

fn read(rel: &str) -> String {
    let path = format!("{}/{rel}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read '{path}': {e}"))
}

/// The exported `SzConfigTool_result` functions: (name, body text).
fn result_exports(src: &str) -> Vec<(String, String)> {
    let src = src.split("#[cfg(test)]").next().unwrap_or(src);
    let mut out = Vec::new();
    for chunk in src.split("#[unsafe(no_mangle)]").skip(1) {
        let Some(fn_at) = chunk.find("extern \"C\" fn ") else {
            continue;
        };
        let rest = &chunk[fn_at + "extern \"C\" fn ".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let Some(open) = rest.find('{') else { continue };
        if !rest[..open].contains("-> SzConfigTool_result") {
            continue;
        }
        out.push((name, balanced_block(&rest[open..]).to_string()));
    }
    assert!(out.len() > 100, "parsed only {} exports", out.len());
    out
}

/// The text of the `{...}` block starting at `s[0]`.
fn balanced_block(s: &str) -> &str {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &s[..=i];
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced braces")
}

/// Codes passed to `set_error(..., <code>)` right after each occurrence of
/// `needle` (the message text) in `body`.
fn codes_after(body: &str, needle: &str) -> BTreeSet<i64> {
    body.match_indices(needle)
        .filter_map(|(at, _)| {
            let tail = &body[at..];
            let end = tail.find(");")?;
            let call = &tail[..end];
            let code = call.rsplit(',').next()?.trim();
            code.parse().ok()
        })
        .collect()
}

/// Every literal return code (`returnCode: -N`, `error_result(-N)`,
/// `set_error(.., -N)`) in the file.
fn all_codes(src: &str) -> BTreeSet<i64> {
    let mut codes = BTreeSet::new();
    for (pat, close) in [("returnCode: ", ['\n', ',']), ("error_result(", [')', ')'])] {
        for (at, _) in src.match_indices(pat) {
            let tail = &src[at + pat.len()..];
            let end = tail.find(close).unwrap_or(tail.len());
            if let Ok(code) = tail[..end].trim().trim_end_matches(',').parse::<i64>() {
                codes.insert(code);
            }
        }
    }
    codes
}

/// The `SzConfigTool_*` names listed under the header line containing
/// `marker`, up to the next empty comment line (` *`) or the comment's end.
fn header_list(header: &str, marker: &str) -> BTreeSet<String> {
    let mut lines = header.lines().skip_while(|l| !l.contains(marker));
    assert!(lines.next().is_some(), "header has no '{marker}'");
    lines
        .take_while(|l| l.trim() != "*" && !l.contains("*/"))
        .flat_map(|l| l.split(|c: char| !(c.is_alphanumeric() || c == '_')))
        .filter(|w| w.starts_with("SzConfigTool_"))
        .map(str::to_string)
        .collect()
}

fn names<'a>(
    exports: &'a [(String, String)],
    pred: impl Fn(&str) -> bool + 'a,
) -> BTreeSet<String> {
    exports
        .iter()
        .filter(|(_, b)| pred(b))
        .map(|(n, _)| n.clone())
        .collect()
}

#[test]
fn test_header_return_code_families_match_source() {
    let src = read("src/lib.rs");
    let header = read("include/libSzConfigTool.h");
    let exports = result_exports(&src);
    let all: BTreeSet<String> = exports.iter().map(|(n, _)| n.clone()).collect();

    // Only -1..-5 exist.
    assert_eq!(
        all_codes(&src),
        BTreeSet::from([0, -1, -2, -3, -4, -5]),
        "a new return code needs a header 'Return codes' entry"
    );

    // Library errors: -5 without a reason code, -2 without one (ffi_json_value),
    // or -2 with one (handle_result! / set_error_from). Exactly one each.
    let lib5 = names(&exports, |b| b.contains("set_error(e.to_string(), -5)"));
    let lib2_bare = names(&exports, |b| b.contains("ffi_json_value!("));
    let lib2_reason = names(&exports, |b| {
        b.contains("set_error_from")
            || b.replace("handle_result!(Ok", "")
                .contains("handle_result!(")
    });
    for name in &all {
        let n = [&lib5, &lib2_bare, &lib2_reason]
            .iter()
            .filter(|s| s.contains(name))
            .count();
        assert_eq!(n, 1, "{name}: in {n} library-error families");
    }
    assert_eq!(
        lib5,
        header_list(&header, "List B ("),
        "List B (-5) drifted"
    );
    assert_eq!(
        lib2_bare,
        header_list(&header, "List C ("),
        "List C drifted"
    );

    // Invalid UTF-8: -1 (List A, plus SzConfigTool_invoke via required_c_str)
    // or -2 (everything else).
    let utf8_m1 = names(&exports, |b| {
        codes_after(b, "Invalid UTF-8").contains(&-1) || b.contains("required_c_str(")
    });
    let utf8_m2 = names(&exports, |b| {
        codes_after(b, "Invalid UTF-8").contains(&-2)
            || b.contains("ffi_required_str!(")
            || b.contains("required_str!(")
    });
    assert_eq!(utf8_m1, header_list(&header, "List A ("), "List A drifted");
    assert_eq!(&utf8_m1 | &utf8_m2, all, "an export with no UTF-8 check?");
    let mixed: BTreeSet<String> = &utf8_m1 & &utf8_m2;
    assert_eq!(
        mixed,
        BTreeSet::from(
            [
                "SzConfigTool_deleteComparisonCallElement",
                "SzConfigTool_deleteDistinctCallElement",
                "SzConfigTool_deleteExpressionCallElement",
            ]
            .map(String::from)
        ),
        "mixed -1/-2 UTF-8 exports (header List A note)"
    );

    // Single-export irregularities named in the header prose.
    let json_m1 = names(&exports, |b| {
        codes_after(b, "Invalid JSON in").contains(&-1)
    });
    assert_eq!(
        json_m1,
        BTreeSet::from(["SzConfigTool_setAttribute".to_string()])
    );
    let lookup_m4 = names(&exports, |b| b.contains("set_error(e.to_string(), -4)"));
    assert_eq!(
        lookup_m4,
        BTreeSet::from(["SzConfigTool_setGenericThreshold".to_string()])
    );
    let rc_section: String = header
        .lines()
        .skip_while(|l| !l.contains("Return codes (SzConfigTool_result.returnCode)"))
        .take_while(|l| !l.contains("*/"))
        .collect::<Vec<_>>()
        .join("\n");
    for name in json_m1.iter().chain(&lookup_m4) {
        assert!(
            rc_section.contains(name.as_str()),
            "header must name {name}"
        );
    }
}
