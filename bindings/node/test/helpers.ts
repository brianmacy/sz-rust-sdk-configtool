/** Shared test data: the generated manifest/conformance JSON and the fixture. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { CONFORMANCE_JSON, MANIFEST_JSON, WORKSPACE_ROOT } from "./generated/paths.ts";

export type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

export interface ManifestArg {
  name: string;
  type: string;
  optional: boolean;
  tristate: boolean;
  required: boolean;
  /** `json` args only: the manifest `json_type` descriptor. */
  json_type?: Json;
}

export interface ManifestFunction {
  name: string;
  returns: "config" | "json" | "config_and_json" | "int" | "unit";
  status: "implemented" | "not_implemented";
  args: ManifestArg[];
  tuple_names?: string[];
  errors: string[];
}

export interface Expect {
  error?: string;
  kind?: string;
  result?: Json;
  contains?: Json[];
  excludes?: Json[];
  len?: number;
}

export interface Step {
  fn: string;
  args: Record<string, Json>;
  config_literal?: string;
  expect: Expect;
  wire_only?: boolean;
}

export interface Case {
  group: string;
  name: string;
  steps: Step[];
}

const PACKAGE_DIR = join(dirname(fileURLToPath(import.meta.url)), "..");
const ROOT = join(PACKAGE_DIR, WORKSPACE_ROOT);

function readJson<T>(rel: string): T {
  return JSON.parse(readFileSync(join(ROOT, rel), "utf8")) as T;
}

export const manifest = readJson<{ functions: ManifestFunction[]; reason_codes: string[] }>(
  MANIFEST_JSON,
);
export const conformance = readJson<{ fixture: string; cases: Case[] }>(CONFORMANCE_JSON);
/** The REAL template configuration every conformance case starts from. */
export const fixture = readFileSync(join(ROOT, conformance.fixture), "utf8");
export const packageDir = PACKAGE_DIR;

export const byName = new Map(manifest.functions.map((f) => [f.name, f]));

/** Independent re-implementation of the contract's naming rule. */
export function camel(snake: string): string {
  const [first = "", ...rest] = snake.split("_");
  return first + rest.map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");
}

function isObject(v: Json): v is { [key: string]: Json } {
  return typeof v === "object" && v !== null && !Array.isArray(v);
}

/** The conformance subset rule (api/manifest/schema.md). */
export function subsetMatch(expected: Json, actual: Json): boolean {
  if (Array.isArray(expected)) {
    return (
      Array.isArray(actual) &&
      actual.length === expected.length &&
      expected.every((e, i) => subsetMatch(e, actual[i] as Json))
    );
  }
  if (isObject(expected)) {
    return (
      isObject(actual) &&
      Object.entries(expected).every(([k, v]) => k in actual && subsetMatch(v, actual[k] as Json))
    );
  }
  return expected === actual;
}

/** One reachable conformance step per expected error code (first in file order). */
export function errorExamples(): Map<string, { config: string; step: Step }> {
  const out = new Map<string, { config: string; step: Step }>();
  for (const c of conformance.cases) {
    const [step] = c.steps;
    const code = step?.expect.error;
    if (step && code && c.steps.length === 1 && !step.wire_only && !out.has(code)) {
      out.set(code, { config: step.config_literal ?? fixture, step });
    }
  }
  return out;
}
