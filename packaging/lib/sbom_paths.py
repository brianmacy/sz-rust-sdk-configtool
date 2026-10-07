"""Replace the absolute workspace path in a CycloneDX SBOM with the remap prefix.

cargo-cyclonedx writes the workspace path into package URLs / bom-refs in
whatever form the host uses:

* POSIX:             ``file:///home/runner/work/x/x/ffi``
* Windows URL:       ``file:///D:/a/x/x/ffi`` (or ``file://D:/a/...``)
* Windows, in JSON:  ``D:\\\\a\\\\x\\\\x\\\\ffi`` (a backslash path, JSON-escaped)

Every form of every given root (either drive-letter case) is replaced, so
``packaging/gates/check-no-build-paths.sh`` finds no build path afterwards.

Usage: sbom_paths.py <src> <dst> <prefix> <root>...
"""

from __future__ import annotations

import sys


def root_forms(root: str) -> set[str]:
    """Spellings of one root: as given, forward-slash, backslash, JSON-escaped."""
    fwd = root.replace("\\", "/").rstrip("/")
    forms = {fwd}
    if len(fwd) > 1 and fwd[1] == ":":  # Windows drive path
        back = fwd.replace("/", "\\")
        forms |= {back, back.replace("\\", "\\\\")}
        forms |= {f[0].swapcase() + f[1:] for f in list(forms)}
    return forms


def rewrite(text: str, prefix: str, roots: list[str]) -> str:
    """``text`` with every spelling of every root mapped to ``prefix``."""
    forms = set().union(*(root_forms(r) for r in roots))
    # Longest first, so a root never matches inside a longer spelling of itself.
    for form in sorted(forms, key=len, reverse=True):
        for url in ("file:///", "file://"):
            text = text.replace(url + form, "file://" + prefix)
        text = text.replace(form, prefix)
    return text


def main(argv: list[str]) -> int:
    if len(argv) < 4:
        sys.exit(__doc__)
    src, dst, prefix, *roots = argv
    with open(src, encoding="utf-8") as f:
        text = f.read()
    with open(dst, "w", encoding="utf-8", newline="\n") as f:
        f.write(rewrite(text, prefix, roots))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
