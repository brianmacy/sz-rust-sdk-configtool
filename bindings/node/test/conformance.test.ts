/** Runs api/manifest/generated/conformance.json against the REAL built .node. */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import {
  byName,
  camel,
  conformance,
  fixture,
  subsetMatch,
  type Json,
  type ManifestFunction,
  type Step,
} from "./helpers.ts";

interface Outcome {
  kind: string;
  config?: string;
  result?: Json;
}

type TypedFn = (config: string, options?: Record<string, Json>) => unknown;
const typed = sz as unknown as Record<string, TypedFn | undefined>;

function camelArgs(args: Record<string, Json>): Record<string, Json> {
  return Object.fromEntries(Object.entries(args).map(([k, v]) => [camel(k), v]));
}

function viaInvoke(step: Step, config: string): Outcome {
  const env = sz.invoke(step.fn, config, step.args);
  const out: Outcome = { kind: env.kind };
  if (env.config !== undefined) out.config = env.config;
  if (env.result !== undefined) out.result = JSON.parse(env.result) as Json;
  return out;
}

/** Named results are JSON texts: every field must be a string that parses. */
function namedToWire(f: ManifestFunction, ret: Record<string, string>): Json {
  assert.deepEqual(Object.keys(ret).sort(), (f.tuple_names ?? []).map(camel).sort());
  return Object.fromEntries(
    (f.tuple_names ?? []).map((n) => {
      const text = ret[camel(n)];
      assert.equal(typeof text, "string", `${f.name}.${camel(n)} must be JSON text`);
      return [n, JSON.parse(text!) as Json];
    }),
  );
}

/**
 * Shape a typed return value back into the wire outcome for checking. For a
 * `config_and_json` function `ret` is the primary's config text and
 * `record` the `<fn>Result` companion's value (same args, same input).
 */
function shape(f: ManifestFunction, ret: unknown, record?: unknown): Outcome {
  const kind = f.returns;
  if (kind === "config_and_json") {
    assert.equal(typeof ret, "string", `${f.name} must return the config text`);
    const result = f.tuple_names?.length
      ? namedToWire(f, record as Record<string, string>)
      : (JSON.parse(record as string) as Json);
    return { kind, config: ret as string, result };
  }
  if (f.tuple_names?.length) {
    return { kind, result: namedToWire(f, ret as Record<string, string>) };
  }
  switch (kind) {
    case "config":
      assert.equal(typeof ret, "string", `${f.name} must return the config text`);
      return { kind, config: ret as string };
    case "json":
      return { kind, result: JSON.parse(ret as string) as Json };
    case "int":
      return { kind, result: ret as number };
    case "unit":
      assert.equal(ret, undefined);
      return { kind };
  }
}

function run(step: Step, config: string): Outcome {
  const f = byName.get(step.fn);
  assert.ok(f, `unknown fn ${step.fn}`);
  const fn = typed[camel(step.fn)];
  if (f.status === "not_implemented") {
    assert.equal(fn, undefined, `${step.fn} must not be a typed export`);
    return viaInvoke(step, config);
  }
  // wire_only: omits a `required` arg to test MISSING_FIELD (not typeable).
  if (step.wire_only) return viaInvoke(step, config);
  assert.equal(typeof fn, "function", `typed export ${camel(step.fn)}`);
  const ret = fn!(config, camelArgs(step.args));
  if (f.returns !== "config_and_json") return shape(f, ret);
  const companion = typed[`${camel(step.fn)}Result`];
  assert.equal(typeof companion, "function", `typed companion ${camel(step.fn)}Result`);
  return shape(f, ret, companion!(config, camelArgs(step.args)));
}

function checkArray(step: Step, result: Json | undefined): void {
  const { contains, excludes, len } = step.expect;
  if (contains === undefined && excludes === undefined && len === undefined) return;
  assert.ok(Array.isArray(result), "result must be an array");
  if (len !== undefined) assert.equal(result.length, len);
  for (const want of contains ?? []) {
    assert.ok(result.some((r) => subsetMatch(want, r)), `contains ${JSON.stringify(want)}`);
  }
  for (const bad of excludes ?? []) {
    assert.ok(!result.some((r) => subsetMatch(bad, r)), `excludes ${JSON.stringify(bad)}`);
  }
}

function runStep(step: Step, current: string): string {
  const config = step.config_literal ?? current;
  const want = step.expect.error;
  if (want !== undefined) {
    assert.throws(
      () => run(step, config),
      (e: unknown) => e instanceof sz.SzConfigToolError && e.code === want,
      `${step.fn} must fail with ${want}`,
    );
    const companion = typed[`${camel(step.fn)}Result`];
    if (byName.get(step.fn)?.returns === "config_and_json" && !step.wire_only) {
      assert.throws(
        () => companion!(config, camelArgs(step.args)),
        (e: unknown) => e instanceof sz.SzConfigToolError && e.code === want,
        `${step.fn}Result must fail with ${want}`,
      );
    }
    return current;
  }
  const out = run(step, config);
  if (step.expect.kind !== undefined) assert.equal(out.kind, step.expect.kind);
  if (step.expect.result !== undefined) {
    assert.ok(
      subsetMatch(step.expect.result, out.result ?? null),
      `result ${JSON.stringify(out.result)} !~ ${JSON.stringify(step.expect.result)}`,
    );
  }
  checkArray(step, out.result);
  return out.config ?? current;
}

describe("conformance", () => {
  test("has cases", () => assert.ok(conformance.cases.length > 0));
  for (const c of conformance.cases) {
    test(`${c.group}/${c.name}`, () => {
      let current = fixture;
      for (const step of c.steps) current = runStep(step, current);
    });
  }
});
