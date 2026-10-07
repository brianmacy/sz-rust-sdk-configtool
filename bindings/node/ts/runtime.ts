/**
 * The single call path of every typed function: serialize the wire args,
 * call the native `invoke`, shape the envelope. `config` strings are passed
 * and returned untouched (opaque, byte-exact).
 */
import { SzConfigToolError, toSzConfigToolError } from "./errors.js";
import { loadNative } from "./native.js";
import { translateError, wireArgs, type FnSpec } from "./options.js";

export type { JsonValue } from "./json.js";
export { closest, translateError, wireArgs, type ArgSpec, type FnSpec, type Shape } from "./options.js";

/** A manifest `returns` value. */
export type ReturnKind = "config" | "json" | "config_and_json" | "int" | "unit";

/** The raw `invoke` envelope; `result` is JSON TEXT. */
export interface InvokeEnvelope {
  kind: ReturnKind;
  config?: string;
  result?: string;
}

/** Wire args: snake_case keys; `undefined` values are dropped (= absent). */
export type WireArgs = Readonly<Record<string, unknown>>;

/** `String.prototype.isWellFormed` (Node 20+; not in the ES2022 lib typings). */
function wellFormed(s: string): boolean {
  return (s as unknown as { isWellFormed(): boolean }).isWellFormed();
}

function invalid(message: string): SzConfigToolError {
  return new SzConfigToolError("INVALID_INPUT", message);
}

/**
 * `config` must be the configuration JSON TEXT. Checked before the native
 * call so a common mistake (passing a previous call's non-config value, such
 * as a companion's record, back as the config) fails here with a clear
 * message instead of napi's "Failed to convert JavaScript value" on a later
 * call.
 */
function checkConfig(config: unknown): void {
  if (typeof config === "string") return;
  const got = config === null ? "null" : typeof config;
  throw invalid(
    `config must be a string (the configuration JSON text), got ${got}; if this came from a ` +
      "previous call, use the value that call returned (config-changing functions return the " +
      "config text; use <name>Result for the created row)",
  );
}

/** A string napi would silently change (a lone surrogate becomes U+FFFD). */
function checkText(what: string, s: unknown): void {
  if (typeof s === "string" && !wellFormed(s)) {
    throw invalid(`${what} contains a lone UTF-16 surrogate (not valid Unicode)`);
  }
}

const I64_MIN = -(2n ** 63n);
const I64_MAX = 2n ** 63n - 1n;

/**
 * Reject values the wire would silently change: NaN/Infinity (`JSON.stringify`
 * makes them `null`, which a tri-state arg reads as Clear), `number` integers
 * beyond `Number.MAX_SAFE_INTEGER` (already rounded: pass a `bigint`),
 * `bigint`s outside the i64 range, circular references, and lone surrogates.
 */
function checkWire(path: string, v: unknown, ancestors: object[] = []): void {
  if (typeof v === "object" && v !== null) {
    if (ancestors.includes(v)) throw invalid(`${path} is a circular reference, which JSON cannot represent`);
    ancestors = [...ancestors, v];
  }
  if (typeof v === "number") {
    if (!Number.isFinite(v)) throw invalid(`${path} is ${v}, which JSON cannot represent`);
    if (Number.isInteger(v) && !Number.isSafeInteger(v)) {
      throw invalid(`${path} is not a safe integer (|n| > Number.MAX_SAFE_INTEGER); pass a bigint`);
    }
  } else if (typeof v === "bigint") {
    if (v < I64_MIN || v > I64_MAX) throw invalid(`${path} is ${v}, outside the 64-bit signed integer range`);
  } else if (typeof v === "string") {
    checkText(path, v);
  } else if (Array.isArray(v)) {
    v.forEach((item, i) => checkWire(`${path}[${i}]`, item, ancestors));
  } else if (typeof v === "object" && v !== null) {
    for (const [k, item] of Object.entries(v)) {
      checkText(`${path} key`, k);
      checkWire(`${path}.${k}`, item, ancestors);
    }
  }
}

/**
 * `JSON.stringify` that also writes a `bigint` as its exact decimal digits (a
 * JSON integer), so ids up to the full i64 range reach Rust unrounded. Same
 * rules otherwise: `undefined` members are dropped (= absent), `undefined`
 * array items become `null`.
 */
export function wireJson(v: unknown): string {
  if (typeof v === "bigint") return v.toString();
  if (Array.isArray(v)) return `[${v.map((item) => (item === undefined ? "null" : wireJson(item))).join(",")}]`;
  if (typeof v === "object" && v !== null && typeof (v as { toJSON?: unknown }).toJSON !== "function") {
    const members = Object.entries(v)
      .filter(([, item]) => item !== undefined)
      .map(([k, item]) => `${JSON.stringify(k)}:${wireJson(item)}`);
    return `{${members.join(",")}}`;
  }
  return JSON.stringify(v);
}

/**
 * Call any manifest function by its snake_case wire name (including the
 * `not_implemented` placeholders the typed API skips). `args` is an object
 * (serialized like `JSON.stringify`, so `undefined` = absent; a `bigint` is
 * written as its exact digits) or raw JSON text. Strings with lone
 * surrogates, non-finite numbers, unsafe `number` integers and `bigint`s
 * outside i64 are `INVALID_INPUT` (they cannot cross the boundary unchanged).
 */
export function invoke(name: string, config: string, args: WireArgs | string = {}): InvokeEnvelope {
  checkText("function name", name);
  checkConfig(config);
  checkText("config", config);
  if (typeof args === "string") checkText("args_json", args);
  else checkWire("args", args);
  const native = loadNative();
  try {
    const argsJson = typeof args === "string" ? args : wireJson(args);
    const out = native.invoke(name, config, argsJson);
    const env: InvokeEnvelope = { kind: out.kind as ReturnKind };
    if (typeof out.config === "string") env.config = out.config;
    if (typeof out.result === "string") env.result = out.result;
    return env;
  } catch (err) {
    throw toSzConfigToolError(err);
  }
}

/** The library version (the workspace version, e.g. `"4.4.0-1"`). */
export function libraryVersion(): string {
  return loadNative().libraryVersion();
}

/** The C ABI version this build implements (`SZCONFIGTOOL_ABI_VERSION`). */
export function abiVersion(): number {
  return loadNative().abiVersion();
}

function field(env: InvokeEnvelope, key: "config" | "result", name: string): string {
  const value = env[key];
  if (value === undefined) {
    throw new SzConfigToolError("INTERNAL", `${name}: native envelope has no ${key}`);
  }
  return value;
}

/**
 * A typed call: check `options` against `spec` and map them to wire args
 * (INVALID_INPUT / MISSING_FIELD before the native call), invoke, and name
 * JS options (not wire fields) in the resulting errors.
 */
function call(spec: FnSpec, config: string, options: unknown): InvokeEnvelope {
  const args = wireArgs(spec, options);
  try {
    return invoke(spec.wire, config, args);
  } catch (err) {
    throw translateError(spec, err);
  }
}

/** `returns: config`, or the primary of a `config_and_json` function: the config. */
export function callConfig(spec: FnSpec, config: string, options?: unknown): string {
  return field(call(spec, config, options), "config", spec.wire);
}

/** `returns: json`, or a `config_and_json` companion: the result JSON text. */
export function callJson(spec: FnSpec, config: string, options?: unknown): string {
  return field(call(spec, config, options), "result", spec.wire);
}

export function callInt(spec: FnSpec, config: string, options?: unknown): number {
  return JSON.parse(field(call(spec, config, options), "result", spec.wire)) as number;
}

export function callUnit(spec: FnSpec, config: string, options?: unknown): void {
  call(spec, config, options);
}

function skipWs(text: string, i: number): number {
  while (i < text.length && " \t\n\r".includes(text.charAt(i))) i++;
  return i;
}

/** Index just past the JSON string starting at `i` (the opening quote). */
function skipString(text: string, i: number): number {
  for (i++; text.charAt(i) !== '"'; i++) if (text.charAt(i) === "\\") i++;
  return i + 1;
}

/** Index just past the JSON value starting at `i` (input already validated). */
function skipValue(text: string, i: number): number {
  const c = text.charAt(i);
  if (c === '"') return skipString(text, i);
  if (c !== "{" && c !== "[") {
    while (i < text.length && !",}] \t\n\r".includes(text.charAt(i))) i++;
    return i;
  }
  let depth = 0;
  do {
    const d = text.charAt(i);
    if (d === '"') {
      i = skipString(text, i);
      continue;
    }
    if (d === "{" || d === "[") depth++;
    else if (d === "}" || d === "]") depth--;
    i++;
  } while (depth > 0);
  return i;
}

/**
 * The exact JSON text of each member of a JSON object text: the source
 * substring, never re-serialized (so `1.0`, `1e3`, escapes and spacing are
 * kept on every Node version). A repeated key keeps the last value, like
 * `JSON.parse`. Text that is not a JSON object is `INTERNAL`.
 */
export function memberTexts(text: string): Map<string, string> {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (err) {
    throw new SzConfigToolError("INTERNAL", `native result is not JSON: ${String(err)}`);
  }
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new SzConfigToolError("INTERNAL", "native named result is not a JSON object");
  }
  const out = new Map<string, string>();
  let i = skipWs(text, skipWs(text, 0) + 1);
  while (text.charAt(i) === '"') {
    const keyEnd = skipString(text, i);
    const key = JSON.parse(text.slice(i, keyEnd)) as string;
    const start = skipWs(text, skipWs(text, keyEnd) + 1); // past ':'
    const end = skipValue(text, start);
    out.set(key, text.slice(start, end));
    i = skipWs(text, end);
    if (text.charAt(i) === ",") i = skipWs(text, i + 1);
  }
  return out;
}

/**
 * A `tuple_names` result (a `json` function, or a `config_and_json`
 * companion): the wire record `{snake: value}` becomes
 * `{camel: "<value as JSON text>"}` (never the config). A missing member is
 * the JSON text `null`.
 */
export function callNamed<T>(
  spec: FnSpec,
  config: string,
  options: unknown,
  fields: ReadonlyArray<readonly [string, string]>,
): T {
  const members = memberTexts(field(call(spec, config, options), "result", spec.wire));
  const out: Record<string, string> = {};
  for (const [wire, camel] of fields) out[camel] = members.get(wire) ?? "null";
  return out as T;
}
