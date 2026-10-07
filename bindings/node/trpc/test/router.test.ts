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
  test("one procedure per typed function (+ companions), none for not_implemented", () => {
    const want = implemented.flatMap((f) =>
      f.returns === "config_and_json" ? [camel(f.name), `${camel(f.name)}Result`] : [camel(f.name)],
    );
    assert.deepEqual(Object.keys(procedures).sort(), want.sort());
  });

  test("config-returning functions are mutations; read-only ones and companions queries", () => {
    for (const f of implemented) {
      const want = f.returns === "config" || f.returns === "config_and_json" ? "mutation" : "query";
      assert.equal(procedures[camel(f.name)]?._def.type, want, f.name);
      if (f.returns === "config_and_json") {
        assert.equal(procedures[`${camel(f.name)}Result`]?._def.type, "query", f.name);
      }
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
    })) as sz.VerifyCompatibilityVersionRecord;
    assert.deepEqual(v, { currentVersion: '"11"', matches: "true" });
    const input = { config: fixture, gplanCode: "my_plan", gplanDesc: "mine" };
    const p = (await caller["setGenericPlanResult"]!(input)) as sz.SetGenericPlanRecord;
    assert.deepEqual(p, { planId: "3", wasCreated: "true" });
    const cfg = await caller["setGenericPlan"]!(input);
    assert.equal(cfg, sz.setGenericPlan(fixture, { gplanCode: "my_plan", gplanDesc: "mine" }));
  });

  test("config_and_json: the procedure returns the config, <name>Result the row", async () => {
    const input = {
      config: fixture,
      attribute: "my_attr",
      feature: "name",
      element: "full_name",
      class: "OTHER",
    };
    const cfg = await caller["addAttribute"]!(input);
    assert.equal(typeof cfg, "string");
    assert.match(sz.getAttribute(cfg as string, { code: "MY_ATTR" }), /"ATTR_CODE":"MY_ATTR"/);
    const row = await caller["addAttributeResult"]!(input);
    assert.equal(typeof row, "string");
    assert.match(row as string, /"ATTR_CODE":"MY_ATTR"/);
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

describe("strict structured inputs (issue #76)", () => {
  const profile = { config: fixture, code: "P2", genericPlan: "SEARCH" };

  test("nested unknown keys are rejected by Zod (strict element objects)", async () => {
    const inputs: Array<Record<string, unknown>> = [
      { config: fixture, feature: "F1", elementList: [{ element: "E1", bogus: 1 }] },
      { config: fixture, id: 0, ruleConfig: { ERRULE_CODE: "R", QUAL_ERFRAG_CODE: "SAME_NAME", TIER: 1 } },
      { ...profile, elements: [{ feature: "NAME", flag: "Yes", extra: true }] },
    ];
    const procs = ["addFeature", "addRule", "addSearchProfile"];
    for (const [i, input] of inputs.entries()) {
      const err = await rejects(caller[procs[i]!]!(input as Record<string, Json>));
      assert.equal(err.code, "BAD_REQUEST", procs[i]);
      assert.equal(errorData(err), null, `${procs[i]}: rejected by Zod, not the library`);
    }
  });

  test("typed structures: enum, array and element shapes are checked by Zod", async () => {
    for (const elements of [[{ feature: "NAME", flag: "Maybe" }], '[{"feature":"NAME","flag":"Y"}]', [{ feature: "NAME" }]]) {
      const err = await rejects(caller["addSearchProfile"]!({ ...profile, elements } as Record<string, Json>));
      assert.equal(err.code, "BAD_REQUEST", JSON.stringify(elements));
      assert.equal(errorData(err), null, "rejected by Zod, not the library");
    }
    const ok = await caller["addSearchProfile"]!({ ...profile, elements: [{ feature: "NAME", flag: "N" }] });
    assert.equal(ok, sz.addSearchProfile(fixture, { code: "P2", genericPlan: "SEARCH", elements: [{ feature: "NAME", flag: "N" }] }));
  });

  test("binding errors reach the client with JS option names", async () => {
    const err = await rejects(
      caller["addExpressionCall"]!({
        config: fixture,
        efuncCode: "PARSE_NAME",
        elementList: [{ element: "E\uD800", required: "Yes" }],
        isVirtual: "No",
      }),
    );
    assert.equal(err.code, "BAD_REQUEST");
    assert.equal(errorData(err)?.reasonCode, "INVALID_INPUT");
    assert.match(err.message, /^args\.elementList\[0\]\.element \(element_list\[0\]\.element\) contains/);
  });
});

describe("error mapping", () => {
  // Steps whose function has no procedure (wire-only) are skipped.
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
    const plain = new Error("plain failure");
    const wrapped = toTRPCError(plain);
    assert.equal(wrapped.code, "INTERNAL_SERVER_ERROR");
    assert.equal(wrapped.message, "plain failure");
    assert.equal(wrapped.cause, plain);
    const internal = new sz.SzConfigToolError("INTERNAL", "x");
    assert.equal(toTRPCError(internal).code, "INTERNAL_SERVER_ERROR");
  });
});
