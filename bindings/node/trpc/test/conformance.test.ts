/**
 * api/manifest/generated/conformance.json through the router's REAL caller
 * over the REAL built .node: every success matches the direct typed binding
 * exactly, every expected error arrives as its reason code's tRPC error.
 */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import { TRPCError } from "@trpc/server";
import * as sz from "sz-configtool";

import { configToolRouter, errorData, TRPC_CODE_BY_REASON } from "../dist/index.js";
import { byName, camel, conformance, fixture, type Json, type Step } from "../../test/helpers.ts";

type Proc = (input: Record<string, Json>) => Promise<unknown>;
type TypedFn = (config: string, options?: Record<string, Json>) => unknown;

const caller = configToolRouter.createCaller({}) as unknown as Record<string, Proc>;
const procedures = Object.keys(configToolRouter._def.procedures);
const typed = sz as unknown as Record<string, TypedFn>;
const driven = new Set<string>();

function camelArgs(args: Record<string, Json>): Record<string, Json> {
  return Object.fromEntries(Object.entries(args).map(([k, v]) => [camel(k), v]));
}

/** The configuration a successful typed result carries, if any. */
function configOf(returns: string, out: unknown): string | undefined {
  if (returns === "config") return out as string;
  if (returns === "config_and_json") return (out as { config: string }).config;
  return undefined;
}

/** Steps with no procedure: not_implemented placeholders and wire-only arg sets. */
function wireStep(step: Step, config: string, current: string): string {
  if (step.expect.error !== undefined) return current;
  return sz.invoke(step.fn, config, step.args).config ?? current;
}

async function expectError(proc: Proc, input: Record<string, Json>, want: string): Promise<void> {
  try {
    await proc(input);
  } catch (e) {
    assert.ok(e instanceof TRPCError, String(e));
    assert.equal(e.code, TRPC_CODE_BY_REASON[want as sz.ReasonCode]);
    const data = errorData(e);
    // Zod rejects some INVALID_INPUT args before the library sees them.
    if (data === null) assert.equal(want, "INVALID_INPUT", e.message);
    else assert.equal(data.reasonCode, want);
    return;
  }
  assert.fail(`expected ${want}`);
}

async function runStep(step: Step, current: string): Promise<string> {
  const f = byName.get(step.fn);
  assert.ok(f, `unknown fn ${step.fn}`);
  const config = step.config_literal ?? current;
  const name = camel(step.fn);
  if (f.status === "not_implemented" || step.wire_only) return wireStep(step, config, current);
  const proc = caller[name]!;
  const options = camelArgs(step.args);
  const input = { config, ...options };
  driven.add(name);
  const want = step.expect.error;
  if (want !== undefined) {
    await expectError(proc, input, want);
    return current;
  }
  const out = await proc(input);
  const direct = typed[name]!(config, options);
  assert.deepEqual(out, direct, `${name}: router result differs from the direct binding`);
  return configOf(f.returns, out) ?? current;
}

describe("conformance through the router", () => {
  for (const c of conformance.cases) {
    test(`${c.group}/${c.name}`, async () => {
      let current = fixture;
      for (const step of c.steps) current = await runStep(step, current);
    });
  }
  // Runs last (tests in a suite run in order).
  test("every procedure is driven by at least one conformance step", () => {
    assert.deepEqual(procedures.filter((p) => !driven.has(p)), []);
  });
});
