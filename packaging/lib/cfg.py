#!/usr/bin/env python3
"""Read a scalar or list map keys from packaging/config.yaml (stdlib only).

Supports the subset config.yaml uses: block-style nested maps of scalars,
`#` comments, single/double-quoted or bare scalars.

Usage:
  cfg.py <file> <dotted.key>          print the scalar (exit 2 if absent)
  cfg.py <file> --keys <dotted.key>   print the child keys of a map, one per line
"""

import sys


def parse(path: str) -> dict:
    root: dict = {}
    stack: list[tuple[int, dict]] = [(-1, root)]
    with open(path, encoding="utf-8") as fh:
        for lineno, raw in enumerate(fh, 1):
            line = strip_comment(raw.rstrip("\n"))
            if not line.strip():
                continue
            indent = len(line) - len(line.lstrip(" "))
            key, sep, value = line.strip().partition(":")
            if not sep:
                raise SystemExit(f"{path}:{lineno}: expected 'key: value'")
            while indent <= stack[-1][0]:
                stack.pop()
            parent = stack[-1][1]
            value = value.strip()
            if value:
                parent[key] = unquote(value)
            else:
                child: dict = {}
                parent[key] = child
                stack.append((indent, child))
    return root


def strip_comment(line: str) -> str:
    quote = None
    for i, ch in enumerate(line):
        match ch:
            case '"' | "'" if quote is None:
                quote = ch
            case _ if ch == quote:
                quote = None
            case "#" if quote is None and (i == 0 or line[i - 1] == " "):
                return line[:i]
    return line


def unquote(value: str) -> str:
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
        return value[1:-1]
    return value


def lookup(tree: dict, dotted: str):
    node = tree
    for part in dotted.split("."):
        if not isinstance(node, dict) or part not in node:
            raise KeyError(dotted)
        node = node[part]
    return node


def main(argv: list[str]) -> int:
    match argv:
        case [path, "--keys", key]:
            node = lookup(parse(path), key)
            if not isinstance(node, dict):
                print(f"{key} is not a map", file=sys.stderr)
                return 2
            print("\n".join(node))
        case [path, key]:
            try:
                node = lookup(parse(path), key)
            except KeyError:
                print(f"missing key: {key}", file=sys.stderr)
                return 2
            if isinstance(node, dict):
                print(f"{key} is a map, not a scalar", file=sys.stderr)
                return 2
            print(node)
        case _:
            print(__doc__, file=sys.stderr)
            return 64
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
