/**
 * Strict options (issue #76): every typed function checks its options object
 * against a generated {@link FnSpec} BEFORE the native call, and translates
 * native errors that name wire (snake_case) fields to the JS option names.
 *
 * - Not a plain object (string, number, array, null, ...) -> INVALID_INPUT.
 * - Missing options -> `{}` only when every option is optional.
 * - Unknown keys (top level and inside structured options) -> INVALID_INPUT
 *   naming the key and the closest valid one.
 * - `json` options are checked against their manifest `json_type` (a
 *   {@link Shape}); a missing required key inside one is MISSING_FIELD.
 */
import { SzConfigToolError } from "./errors.js";

/** The runtime form of a manifest `json_type` (object keys ending in `?` are optional). */
export type Shape =
  | "any"
  | "string"
  | "int"
  | "bool"
  | { readonly enum: readonly string[] }
  | { readonly array: Shape }
  | { readonly object: Readonly<Record<string, Shape>> }
  | { readonly oneOf: readonly Shape[] };

/** One option: `[jsName, wireName, required, shape?]`. */
export type ArgSpec = readonly [js: string, wire: string, required: boolean, shape?: Shape];

/** A typed function's options, generated from the manifest. */
export interface FnSpec {
  /** The typed function's JS name (used in messages). */
  readonly name: string;
  /** The wire (manifest) function name passed to `invoke`. */
  readonly wire: string;
  readonly args: readonly ArgSpec[];
}

function invalid(message: string): SzConfigToolError {
  return new SzConfigToolError("INVALID_INPUT", message);
}

/** `null`, `array`, `object` (plain), a class tag (`Date`, `Map`), or `typeof`. */
function typeName(v: unknown): string {
  if (v === null) return "null";
  if (Array.isArray(v)) return "array";
  if (typeof v !== "object") return typeof v;
  const proto: unknown = Object.getPrototypeOf(v);
  if (proto === null || proto === Object.prototype) return "object";
  return Object.prototype.toString.call(v).slice(8, -1);
}

function isPlainObject(v: unknown): v is Record<string, unknown> {
  return typeName(v) === "object";
}

function distance(a: string, b: string): number {
  let prev = Array.from({ length: b.length + 1 }, (_, j) => j);
  for (let i = 1; i <= a.length; i++) {
    const row = [i];
    for (let j = 1; j <= b.length; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1;
      row.push(Math.min(prev[j]! + 1, row[j - 1]! + 1, prev[j - 1]! + cost));
    }
    prev = row;
  }
  return prev[b.length]!;
}

/** Case- and underscore-insensitive form (`generic_plan` ~ `genericPlan`). */
function norm(s: string): string {
  return s.toLowerCase().replaceAll("_", "");
}

/**
 * The valid name closest to `key`: a small edit distance, or one contains
 * the other as a prefix/suffix/substring (`plan` -> `genericPlan`).
 */
export function closest(key: string, valid: readonly string[]): string | undefined {
  const k = norm(key);
  let best: string | undefined;
  let bestScore = Number.POSITIVE_INFINITY;
  for (const v of valid) {
    const n = norm(v);
    const d = distance(k, n);
    const near =
      d <= Math.max(1, Math.floor(k.length / 3)) ||
      (k.length >= 3 && n.includes(k)) ||
      (n.length >= 4 && (k.startsWith(n) || k.endsWith(n)));
    const score = d * 100 + distance(key, v);
    if (near && score < bestScore) {
      best = v;
      bestScore = score;
    }
  }
  return best;
}

/** Reject keys of `obj` not in `valid` (`noun` = option | key; `where` = context). */
function checkKeys(obj: object, valid: readonly string[], noun: string, where: string): void {
  const unknown = Object.keys(obj).filter((k) => !valid.includes(k));
  if (unknown.length === 0) return;
  const hints = unknown.map((k) => [k, closest(k, valid)] as const);
  const list = `valid ${noun}s: ${valid.join(", ")}`;
  if (hints.length === 1) {
    const [k, hint] = hints[0]!;
    throw invalid(`unknown ${noun} '${k}' ${where}; ${hint ? `did you mean '${hint}'?` : list}`);
  }
  const parts = hints.map(([k, hint]) => (hint ? `'${k}' (did you mean '${hint}'?)` : `'${k}'`));
  const tail = hints.some(([, hint]) => hint === undefined) ? `; ${list}` : "";
  throw invalid(`unknown ${noun}s ${where}: ${parts.join(", ")}${tail}`);
}

type Kind = "string" | "int" | "bool" | "array" | "object";

const KIND_WORDS: Readonly<Record<Kind, string>> = {
  string: "a string",
  int: "an integer",
  bool: "a boolean",
  array: "an array",
  object: "an object",
};

/** The JSON kind a `one_of` alternative accepts (never `any` / a nested `one_of`). */
const KIND_OF_TAG: Readonly<Record<string, Kind>> = { enum: "string", array: "array", object: "object" };

function shapeKind(shape: Shape): Kind {
  return typeof shape === "string" ? (shape as Kind) : KIND_OF_TAG[Object.keys(shape)[0]!]!;
}

function valueKind(v: unknown): string {
  if (typeof v === "number" || typeof v === "bigint") return "int";
  if (typeof v === "boolean") return "bool";
  return typeName(v);
}

/** Check `v` (at `path`) against `shape`; `fn` names the typed function. */
function checkShape(fn: string, path: string, v: unknown, shape: Shape): void {
  if (shape === "any") return;
  const fail = (expected: string): never => {
    throw invalid(`${fn}: ${path} must be ${expected}, got ${typeName(v)}`);
  };
  if (typeof shape === "string") {
    const ok =
      shape === "string"
        ? typeof v === "string"
        : shape === "bool"
          ? typeof v === "boolean"
          : typeof v === "bigint" || Number.isInteger(v);
    if (!ok) fail(KIND_WORDS[shape]);
    return;
  }
  if ("enum" in shape) {
    if (typeof v === "string" && shape.enum.includes(v)) return;
    const got = typeof v === "string" ? JSON.stringify(v) : typeName(v);
    throw invalid(`${fn}: ${path} must be one of ${shape.enum.join(", ")} (got ${got})`);
  }
  if ("array" in shape) {
    if (!Array.isArray(v)) fail("an array");
    (v as unknown[]).forEach((item, i) => checkShape(fn, `${path}[${i}]`, item, shape.array));
    return;
  }
  if ("object" in shape) {
    if (!isPlainObject(v)) fail("an object");
    checkObject(fn, path, v as Record<string, unknown>, shape.object);
    return;
  }
  const alt = shape.oneOf.find((s) => shapeKind(s) === valueKind(v));
  if (alt === undefined) {
    const words = shape.oneOf.map((s) => KIND_WORDS[shapeKind(s)]);
    fail(`${words.slice(0, -1).join(", ")} or ${words.at(-1)!}`);
  }
  checkShape(fn, path, v, alt!);
}

function checkObject(
  fn: string,
  path: string,
  obj: Record<string, unknown>,
  fields: Readonly<Record<string, Shape>>,
): void {
  const entries = Object.entries(fields).map(([key, shape]) => {
    const optional = key.endsWith("?");
    return { name: optional ? key.slice(0, -1) : key, optional, shape };
  });
  checkKeys(obj, entries.map((e) => e.name), "key", `in ${fn} ${path}`);
  for (const { name, optional, shape } of entries) {
    const v = obj[name];
    if (v === undefined) {
      if (optional) continue;
      throw new SzConfigToolError("MISSING_FIELD", `${fn}: missing required key ${path}.${name}`);
    }
    checkShape(fn, `${path}.${name}`, v, shape);
  }
}

function containerMessage(spec: FnSpec, options: unknown): string {
  const base = `${spec.name}: options must be a plain object, got ${typeName(options)}`;
  if (typeof options !== "string") return base;
  const required = spec.args.filter((a) => a[2]);
  const single = required.length === 1 ? required[0] : spec.args.length === 1 ? spec.args[0] : undefined;
  return single ? `${base}; pass { ${single[0]}: ${JSON.stringify(options)} }` : base;
}

/**
 * Validate a typed function's `options` and map them to wire args
 * (`{snake: value}`; `undefined` values are absent).
 */
export function wireArgs(spec: FnSpec, options: unknown): Record<string, unknown> {
  if (options === undefined) {
    const required = spec.args.filter((a) => a[2]).map((a) => a[0]);
    if (required.length > 0) {
      throw invalid(`${spec.name}: options are required (required: ${required.join(", ")}), got undefined`);
    }
    return {};
  }
  if (!isPlainObject(options)) throw invalid(containerMessage(spec, options));
  checkKeys(options, spec.args.map((a) => a[0]), "option", `for ${spec.name}`);
  const out: Record<string, unknown> = {};
  for (const [js, wire, , shape] of spec.args) {
    const v = options[js];
    if (v === undefined) continue;
    if (shape !== undefined) checkShape(spec.name, js, v, shape);
    out[wire] = v;
  }
  return out;
}

/** Codes whose message may name an argument (translated to JS names). */
const NAMING_CODES: ReadonlySet<string> = new Set(["MISSING_FIELD", "INVALID_INPUT"]);

/**
 * Rewrite whole-token wire names (optionally quoted, with an index/field
 * path) to the JS name; the first occurrence of each also keeps the wire
 * spelling in parentheses: `generic_plan` -> `genericPlan (generic_plan)`.
 */
function translateMessage(names: ReadonlyMap<string, string>, message: string): string {
  const alts = [...names.keys()].sort((a, b) => b.length - a.length).join("|");
  const pattern = new RegExp(
    `(^|[^A-Za-z0-9_'"])(['"]?)(${alts})((?:\\[\\d+\\]|\\.[A-Za-z_][A-Za-z0-9_]*)*)\\2(?![A-Za-z0-9_])`,
    "g",
  );
  const seen = new Set<string>();
  return message.replace(pattern, (_m, pre: string, q: string, wire: string, path: string) => {
    const first = !seen.has(wire);
    seen.add(wire);
    const note = first ? ` (${wire}${path})` : "";
    return `${pre}${q}${names.get(wire)!}${path}${q}${note}`;
  });
}

/** Translate `failures[].field` values naming a wire arg (or a path under one). */
function translateDetails(names: ReadonlyMap<string, string>, details: string): string {
  let parsed: unknown;
  try {
    parsed = JSON.parse(details);
  } catch {
    return details;
  }
  const failures: unknown = (Object(parsed) as { failures?: unknown }).failures;
  if (!Array.isArray(failures)) return details;
  let changed = false;
  for (const failure of failures as Array<{ field?: unknown }>) {
    const field = failure.field;
    if (typeof field !== "string") continue;
    const head = field.split(/[[.]/, 1)[0]!;
    const js = names.get(head);
    if (js === undefined) continue;
    failure.field = js + field.slice(head.length);
    changed = true;
  }
  return changed ? JSON.stringify(parsed) : details;
}

/**
 * `err` with wire argument names translated to `spec`'s JS option names
 * (MISSING_FIELD / INVALID_INPUT messages and validation `details`), the
 * original kept as `cause`; anything else is returned unchanged.
 */
export function translateError<T>(spec: FnSpec, err: T): T | SzConfigToolError {
  if (!(err instanceof SzConfigToolError)) return err;
  const names = new Map(spec.args.filter(([js, wire]) => js !== wire).map(([js, wire]) => [wire, js]));
  if (names.size === 0) return err;
  const message = NAMING_CODES.has(err.code) ? translateMessage(names, err.message) : err.message;
  const details = err.details === undefined ? undefined : translateDetails(names, err.details);
  if (message === err.message && details === err.details) return err;
  return new SzConfigToolError(err.code, message, details, { cause: err });
}
