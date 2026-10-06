#!/usr/bin/env python3
"""Coverage gate: every gated line, branch and (Rust) region is executed,
except the reviewed exclusions in coverage/policy.yaml (stdlib only).

Usage: coverage_gate.py <policy.yaml> <component> <report-dir>

Reads the component's report(s) from <report-dir> (written by
packaging/coverage.sh): lcov (`report`), JaCoCo XML (`jacoco`) and/or an
llvm-cov JSON export (`regions`, Rust). Prints a per-file table and fails
(exit 1) when a gated file has an uncovered line, branch or region that no
exclusion names, or when an exclusion no longer matches anything (stale
exclusions must be deleted, so the list stays exactly the real gaps).

An exclusion names a file (path suffix, "" = every gated file) and its ANCHOR
lines: `line` (exact whitespace-trimmed source text) or `pattern` (regex on
that text); optionally `function` (regex, full match on the Rust fn name)
limits anchors to those functions' bodies, and `scope: block` widens what an
anchor excuses from the anchor line itself to the brace block the anchor
opens (when the line ends with `{` or the next line is just `{`) or sits in. `platform_dependent: "true"`
marks a gap that exists only on some hosts (exempt from the stale check). It excuses every uncovered line, branch and region
starting on an excused line. Line numbers are not used, so an exclusion
survives unrelated edits but not a change of the anchor line itself.
"""

from __future__ import annotations

import json
import re
import sys
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cfg  # noqa: E402  (packaging/lib/cfg.py: the YAML subset parser)


@dataclass
class FileCov:
    lines: dict[int, int] = field(default_factory=dict)  # line -> hits
    branches: dict[tuple[int, str], int] = field(default_factory=dict)  # (line, id) -> taken
    regions: dict[tuple[int, int, int, int], int] = field(default_factory=dict)  # coords -> hits


def merge_max(target: dict, key, value: int) -> None:
    target[key] = max(target.get(key, 0), value)


def read_lcov(path: Path, files: dict[str, FileCov]) -> None:
    current: FileCov | None = None
    for raw in path.read_text(encoding="utf-8").splitlines():
        tag, _, rest = raw.partition(":")
        match tag:
            case "SF":
                current = files.setdefault(normalize(rest), FileCov())
            case "DA" if current is not None:
                line, hits = rest.split(",")[:2]
                merge_max(current.lines, int(line), int(hits))
            case "BRDA" if current is not None:
                line, block, branch, taken = rest.split(",")
                merge_max(current.branches, (int(line), f"{block}.{branch}"), 0 if taken == "-" else int(taken))
            case "end_of_record":
                current = None


def read_jacoco(path: Path, files: dict[str, FileCov]) -> None:
    root = ET.parse(path).getroot()
    for package in root.iter("package"):
        for source in package.iter("sourcefile"):
            cov = files.setdefault(f"{package.get('name')}/{source.get('name')}", FileCov())
            for line in source.iter("line"):
                nr, ci, mb, cb = (int(line.get(k, "0")) for k in ("nr", "ci", "mb", "cb"))
                merge_max(cov.lines, nr, ci)
                for i in range(mb + cb):
                    merge_max(cov.branches, (nr, str(i)), 1 if i < cb else 0)


def read_llvm_regions(path: Path, files: dict[str, FileCov]) -> None:
    """Code regions of an llvm-cov JSON export; a region of a generic is
    covered when any instantiation executed it."""
    data = json.loads(path.read_text(encoding="utf-8"))
    for export in data["data"]:
        for function in export["functions"]:
            names = function["filenames"]
            for ls, cs, le, ce, count, file_id, _expanded, kind in function["regions"]:
                if kind != 0:  # 0 = code region (not expansion/skipped/gap/branch)
                    continue
                cov = files.setdefault(normalize(names[file_id]), FileCov())
                merge_max(cov.regions, (ls, cs, le, ce), count)


def normalize(path: str) -> str:
    return path.replace("\\", "/")


def repo_path(path: str, repo: Path, spec: dict) -> str:
    """Repo-relative form of a report path: `path_strip` (regex) is removed,
    absolute paths under the repo lose the repo prefix, and relative paths get
    `path_prefix` (the directory the tool reported relative to)."""
    if "path_strip" in spec:
        path = re.sub(spec["path_strip"], "", path)
    root = normalize(str(repo)) + "/"
    if path.startswith(root):
        return path[len(root):]
    if not path.startswith("/"):
        path = spec.get("path_prefix", "") + path
    return path


@dataclass
class Exclusion:
    name: str
    file: str
    why: str
    line: str | None = None
    pattern: str | None = None
    function: str | None = None
    scope: str = "line"
    platform_dependent: bool = False
    used: bool = False

    def anchors(self, text: str) -> bool:
        if self.line is not None:
            return text == self.line
        return re.search(self.pattern or r"(?!)", text) is not None

    def excused_lines(self, src: list[str], uncovered: set[int]) -> set[int]:
        """Lines this exclusion excuses in `src` (1-based numbers)."""
        spans = [(1, len(src))] if not self.function else function_spans(src, self.function)
        out: set[int] = set()
        for lo, hi in spans:
            for n in range(lo, hi + 1):
                if not self.anchors(src[n - 1].strip()):
                    continue
                lines = set(range(*block_span(src, n))) if self.scope == "block" else {n}
                if lines & uncovered:
                    out |= lines
        return out


FN_DECL = re.compile(r"^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+(\w+)")


def function_spans(src: list[str], name_regex: str) -> list[tuple[int, int]]:
    """(first, last) lines of every Rust fn whose name fully matches
    `name_regex`: from its `fn` line to the closing brace of its body."""
    rx = re.compile(name_regex)
    spans = []
    for i, text in enumerate(src, 1):
        m = FN_DECL.match(text)
        if m and rx.fullmatch(m.group(1)):
            start, end = block_span(src, i, opens_from=i)
            spans.append((i, end - 1))
    return spans


def block_span(src: list[str], anchor: int, opens_from: int | None = None) -> tuple[int, int]:
    """[start, end) line range of the brace block the anchor line opens (it
    ends with `{`) or sits in; with `opens_from`, the first block opened at or
    after that line."""
    if opens_from is not None:
        start = opens_from
        while start <= len(src) and "{" not in src[start - 1]:
            start += 1
    elif src[anchor - 1].rstrip().endswith("{"):
        start = anchor
    elif anchor < len(src) and src[anchor].strip() == "{":  # brace on the next line (C#)
        start = anchor
    else:
        depth, start = 0, anchor - 1
        while start >= 1:
            depth += src[start - 1].count("}") - src[start - 1].count("{")
            if depth < 0:
                break
            start -= 1
        start = max(start, 1)
    depth = 0
    for end in range(start, len(src) + 1):
        depth += src[end - 1].count("{") - src[end - 1].count("}")
        if depth <= 0 and end > start or (depth == 0 and "{" in src[end - 1]):
            return start, end + 1
    return start, len(src) + 1


def source_lines(path: str, repo: Path) -> list[str]:
    for candidate in (Path(path), repo / path):
        if candidate.is_file():
            return candidate.read_text(encoding="utf-8").splitlines()
    return []


def pct(covered: int, total: int) -> str:
    return "-" if total == 0 else f"{100.0 * covered / total:6.2f}%"


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__, file=sys.stderr)
        return 64
    policy_path, component, report_dir = Path(argv[0]), argv[1], Path(argv[2])
    policy = cfg.parse(str(policy_path))
    spec = policy["components"][component]
    repo = policy_path.resolve().parent.parent

    files: dict[str, FileCov] = {}
    for key, reader in (("report", read_lcov), ("jacoco", read_jacoco), ("regions", read_llvm_regions)):
        if key in spec:
            report = report_dir / spec[key]
            if not report.is_file():
                print(f"[{component}] missing report {report}", file=sys.stderr)
                return 1
            reader(report, files)
    files = {repo_path(p, repo, spec): c for p, c in files.items()}

    include = re.compile(spec["include"])
    exclude = re.compile(spec.get("exclude", r"(?!)"))
    generated = re.compile(spec.get("generated", r"(?!)"))
    exclusions = [
        Exclusion(name, e["file"], e["why"], e.get("line"), e.get("pattern"), e.get("function"), e.get("scope", "line"),
                  e.get("platform_dependent", "false") == "true")
        for name, e in policy.get("exclusions", {}).items()
        if e["component"] == component
    ]

    failures: list[str] = []
    rows: list[tuple[str, str, str, str, str]] = []
    totals = {"gen": [0, 0, 0, 0, 0, 0], "hand": [0, 0, 0, 0, 0, 0]}
    for path in sorted(files):
        if not include.search(path) or exclude.search(path):
            continue
        cov = files[path]
        src = source_lines(path, repo)

        uncovered = (
            {n for n, h in cov.lines.items() if h == 0}
            | {n for (n, _), t in cov.branches.items() if t == 0}
            | {c[0] for c, h in cov.regions.items() if h == 0}
        )
        excused: set[int] = set()
        for ex in exclusions:
            if path.endswith(ex.file):
                lines = ex.excused_lines(src, uncovered)
                ex.used |= bool(lines)
                excused |= lines

        missed_lines = sorted(n for n, h in cov.lines.items() if h == 0 and n not in excused)
        missed_br = sorted({n for (n, _), t in cov.branches.items() if t == 0 and n not in excused})
        missed_reg = sorted({c[0] for c, h in cov.regions.items() if h == 0 and c[0] not in excused})
        bucket = "gen" if generated.search(path) else "hand"
        t = totals[bucket]
        t[0] += sum(1 for h in cov.lines.values() if h > 0)
        t[1] += len(cov.lines)
        t[2] += sum(1 for v in cov.branches.values() if v > 0)
        t[3] += len(cov.branches)
        t[4] += sum(1 for v in cov.regions.values() if v > 0)
        t[5] += len(cov.regions)
        rows.append((
            ("[gen] " if bucket == "gen" else "") + path,
            pct(sum(1 for h in cov.lines.values() if h > 0), len(cov.lines)),
            pct(sum(1 for v in cov.branches.values() if v > 0), len(cov.branches)),
            pct(sum(1 for v in cov.regions.values() if v > 0), len(cov.regions)),
            ", ".join(
                f"{kind} {fmt_ranges(nums)}"
                for kind, nums in (("L", missed_lines), ("B", missed_br), ("R", missed_reg))
                if nums
            ),
        ))
        if missed_lines or missed_br or missed_reg:
            failures.append(path)

    print(f"\n[{component}] coverage (L=uncovered lines, B=lines with an untaken branch, R=lines starting an unexecuted region)")
    width = max((len(r[0]) for r in rows), default=10)
    print(f"  {'file':<{width}}  {'lines':>8} {'branches':>8} {'regions':>8}  not excused")
    for r in rows:
        print(f"  {r[0]:<{width}}  {r[1]:>8} {r[2]:>8} {r[3]:>8}  {r[4]}")
    for bucket, label in (("hand", "hand-written"), ("gen", "generated")):
        t = totals[bucket]
        if t[1] or t[5]:
            print(f"  TOTAL {label}: lines {pct(t[0], t[1])} ({t[0]}/{t[1]}), branches {pct(t[2], t[3])} "
                  f"({t[2]}/{t[3]}), regions {pct(t[4], t[5])} ({t[4]}/{t[5]})")
    if not rows:
        failures.append("(no gated files in the report)")

    stale = [ex.name for ex in exclusions if not ex.used and not ex.platform_dependent]
    for ex in exclusions:
        if ex.used:
            print(f"  excluded [{ex.name}] {ex.file or '*'}: `{ex.line if ex.line is not None else ex.pattern}` -- {ex.why}")
    if stale:
        print(f"[{component}] FAIL: stale exclusions (match nothing uncovered; delete them): {', '.join(stale)}")
    if failures:
        print(f"[{component}] FAIL: uncovered code in {len(failures)} file(s)")
    if failures or stale:
        return 1
    print(f"[{component}] OK")
    return 0


def fmt_ranges(nums: list[int]) -> str:
    out: list[str] = []
    for n in nums:
        if out and out[-1][1] == n - 1:
            out[-1][1] = n
        else:
            out.append([n, n])
    return ",".join(str(a) if a == b else f"{a}-{b}" for a, b in out)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
