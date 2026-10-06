/** The router through a REAL in-process caller over the REAL built .node. */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import { TRPCError } from "@trpc/server";
import * as sz from "sz-configtool";

import { configToolRouter, errorData, schemas, TRPC_CODE_BY_REASON, toTRPCError } from "../dist/index.js";
import { camel, errorExamples, fixture, manifest, type Json } from "../../test/helpers.ts";

type Proc = (input: Record<string, Json>) => Promise<unknown>;
const caller = configToolRouter.createCaller({}) as unknown as Record<string, Proc>;
const procedures = configToolRouter._def.procedures as Record<string, { _def: { type: string } }>;
const implemented = manifest.functions.filter((f) => f.status === "implemented");

async function rejects(p: Promise<unknown>): Promise<TRPCError> {
  try {
    await p;
  } catch (e) {
    assert.ok(e instanceof TRPCError, String(e));
    return e;
  }
  assert.fail("expected a rejection");
}

describe("router shape", () => {
  test("one procedure per typed function, none for not_implemented", () => {
    assert.deepEqual(Object.keys(procedures).sort(), implemented.map((f) => camel(f.name)).sort());
  });

  test("config-returning functions are mutations, read-only ones queries", () => {
    for (const f of implemented) {
      const want = f.returns === "config" || f.returns === "config_and_json" ? "mutation" : "query";
      assert.equal(procedures[camel(f.name)]?._def.type, want, f.name);
    }
  });

  test("one Zod schema per typed function", () => {
    assert.equal(Object.keys(schemas).length, implemented.length);
  });
});

describe("procedures", () => {
  test("add then get, config byte-identical to the direct binding", async () => {
    const cfg = (await caller["addDataSource"]!({ config: fixture, code: "crm" })) as string;
    assert.equal(cfg, sz.addDataSource(fixture, { code: "crm" }));
    const row = (await caller["getDataSource"]!({ config: cfg, code: "CRM" })) as string;
    assert.match(row, /"DSRC_CODE":"CRM"/);
  });

  test("tri-state null survives Zod (clear differs from leave)", async () => {
    const code = "SNAME_SSTAB";
    const clear = await caller["setFragment"]!({ config: fixture, code, description: null });
    const leave = await caller["setFragment"]!({ config: fixture, code });
    assert.equal(clear, sz.setFragment(fixture, { code, description: null }));
    assert.equal(leave, sz.setFragment(fixture, { code }));
    assert.notEqual(clear, leave);
  });

  test("named results are typed records of JSON texts", async () => {
    const v = (await caller["verifyCompatibilityVersion"]!({
      config: fixture,
      expectedVersion: "11",
    })) as sz.VerifyCompatibilityVersionResult;
    assert.deepEqual(v, { currentVersion: '"11"', matches: "true" });
    const p = (await caller["setGenericPlan"]!({
      config: fixture,
      gplanCode: "my_plan",
      gplanDesc: "mine",
    })) as sz.SetGenericPlanResult;
    assert.deepEqual([p.planId, p.wasCreated], ["3", "true"]);
    assert.equal(p.config, sz.setGenericPlan(fixture, { gplanCode: "my_plan", gplanDesc: "mine" }).config);
  });

  test("config_and_json output is { config, json }", async () => {
    const out = (await caller["addAttribute"]!({
      config: fixture,
      attribute: "my_attr",
      feature: "name",
      element: "full_name",
      class: "OTHER",
    })) as sz.ConfigAndJson;
    assert.deepEqual(Object.keys(out).sort(), ["config", "json"]);
    assert.match(out.json, /"ATTR_CODE":"MY_ATTR"/);
  });

  test("int_or_str call selector: id and feature code", async () => {
    const byId = await caller["getComparisonCall"]!({ config: fixture, call: 1 });
    const byCode = await caller["getComparisonCall"]!({ config: fixture, call: "NAME" });
    assert.equal(byId, sz.getComparisonCall(fixture, { call: 1 }));
    assert.equal(byCode, byId);
    for (const call of [1.5, true, null]) {
      const err = await rejects(caller["getComparisonCall"]!({ config: fixture, call }));
      assert.equal(err.code, "BAD_REQUEST", JSON.stringify(call));
      assert.equal(errorData(err), null, "rejected by Zod, not the library");
    }
  });

  test("an in-process bigint id reaches Rust digit-exact; an unsafe number does not pass Zod", async () => {
    const cfg = (await caller["addDataSource"]!({ config: fixture, code: "big", id: 2n ** 63n - 1n } as unknown as Record<string, Json>)) as string;
    assert.match(sz.getDataSource(cfg, { code: "BIG" }), /"DSRC_ID":9223372036854775807[,}]/);
    const err = await rejects(caller["addDataSource"]!({ config: fixture, code: "big", id: 2 ** 63 }));
    assert.equal(err.code, "BAD_REQUEST");
  });

  test("Zod rejects unknown keys, wrong types and missing required args", async () => {
    for (const input of [
      { config: fixture, code: "a", cdoe: "b" },
      { config: fixture, code: 1 },
      { config: fixture },
      { code: "a" },
      { config: fixture, code: "a", id: 1.5 },
    ]) {
      const err = await rejects(caller["addDataSource"]!(input as unknown as Record<string, Json>));
      assert.equal(err.code, "BAD_REQUEST", JSON.stringify(Object.keys(input)));
      assert.equal(errorData(err), null);
    }
  });
});

describe("error mapping", () => {
  // NOT_IMPLEMENTED only comes from the skipped placeholders (no procedure).
  // The caller is a Proxy (any key is callable), so filter on the router.
  const routable = [...errorExamples()].filter(([, { step }]) => camel(step.fn) in procedures);
  for (const [code, { config, step }] of routable) {
    test(`${camel(step.fn)} ${code} -> ${TRPC_CODE_BY_REASON[code as sz.ReasonCode]}`, async () => {
      const proc = caller[camel(step.fn)]!;
      const input = Object.fromEntries(Object.entries(step.args).map(([k, v]) => [camel(k), v]));
      const err = await rejects(proc({ config, ...input }));
      assert.equal(err.code, TRPC_CODE_BY_REASON[code as sz.ReasonCode]);
      assert.ok(err.cause instanceof sz.SzConfigToolError);
      assert.equal(errorData(err)?.reasonCode, code);
      assert.equal(errorData(err)?.kind, code);
      const details = errorData(err)?.details ?? null;
      if (code === "VALIDATION_ERRORS") {
        assert.equal(typeof details, "string", "details is JSON text");
        assert.equal(JSON.parse(details!).schema, "sz-configtool.validation-errors/v1");
      } else {
        assert.equal(details, null);
      }
    });
  }

  test("every reason code has a tRPC code", () => {
    assert.deepEqual(Object.keys(TRPC_CODE_BY_REASON).sort(), [...sz.REASON_CODES].sort());
  });

  test("toTRPCError passes TRPCErrors through and wraps unknown throws", () => {
    const e = new TRPCError({ code: "CONFLICT" });
    assert.equal(toTRPCError(e), e);
    assert.equal(toTRPCError("boom").code, "INTERNAL_SERVER_ERROR");
    const internal = new sz.SzConfigToolError("INTERNAL", "x");
    assert.equal(toTRPCError(internal).code, "INTERNAL_SERVER_ERROR");
  });
});
