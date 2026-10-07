/** Typed-API behaviour against the REAL built .node: naming, args, opacity. */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import { loadNative } from "../dist/native.js";
import { camel, fixture, manifest } from "./helpers.ts";


interface FtypeConfig {
  G2_CONFIG: { CFG_FTYPE: Array<{ FTYPE_ID: number; FTYPE_CODE: string }> };
}

describe("naming", () => {
  test("every implemented function is exported in camelCase", () => {
    for (const f of manifest.functions.filter((f) => f.status === "implemented")) {
      assert.equal(typeof (sz as Record<string, unknown>)[camel(f.name)], "function", f.name);
    }
  });

  test("TYPED_FUNCTION_NAMES lists exactly the implemented functions in order", () => {
    const want = manifest.functions.filter((f) => f.status === "implemented").map((f) => f.name);
    assert.deepEqual([...sz.TYPED_FUNCTION_NAMES], want);
  });

  test("no acronym special-casing", () => {
    assert.equal(typeof sz.addComparisonThreshold, "function");
    assert.equal(typeof sz.getConfigSection, "function");
  });
});

describe("arguments", () => {
  const base = sz.addDataSource(fixture, { code: "crm" });

  test("optional: undefined is absent on the wire", () => {
    const typed = sz.addDataSource(fixture, { code: "crm", id: undefined });
    assert.equal(typed, base);
    assert.equal(sz.invoke("add_data_source", fixture, { code: "crm" }).config, base);
  });

  test("optional: a value is passed under the snake_case wire name", () => {
    const typed = sz.addDataSource(fixture, { code: "crm", id: 4242, retentionLevel: "Forget" });
    const wire = sz.invoke("add_data_source", fixture, {
      code: "crm",
      id: 4242,
      retention_level: "Forget",
    });
    assert.equal(typed, wire.config);
    assert.match(sz.getDataSource(typed, { code: "CRM" }), /"DSRC_ID":4242/);
  });

  test("tri-state: undefined = leave, null = clear, value = set", () => {
    const code = "SNAME_SSTAB";
    const leave = sz.setFragment(fixture, { code, description: undefined });
    const clear = sz.setFragment(fixture, { code, description: null });
    const set = sz.setFragment(fixture, { code, description: "x" });
    assert.equal(leave, sz.invoke("set_fragment", fixture, { code }).config);
    assert.equal(clear, sz.invoke("set_fragment", fixture, { code, description: null }).config);
    assert.equal(set, sz.invoke("set_fragment", fixture, { code, description: "x" }).config);
    assert.notEqual(leave, clear);
    assert.notEqual(clear, set);
  });

  test("null for a non-tri-state arg is INVALID_INPUT", () => {
    assert.throws(
      () => sz.addDataSource(fixture, { code: "crm", id: null as unknown as number }),
      (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "INVALID_INPUT",
    );
  });

  test("required (Option in Rust) args are required in TS; absent is MISSING_FIELD", () => {
    const opts = { cfuncCode: "STR_COMP", ftypeCode: "NAME", cfuncRtnval: "x_score" };
    assert.equal(typeof sz.addComparisonThreshold(fixture, opts), "string");
    // @ts-expect-error cfuncRtnval is required by the typed API
    const missing: sz.AddComparisonThresholdOptions = { cfuncCode: "STR_COMP", ftypeCode: "NAME" };
    assert.throws(
      () => sz.addComparisonThreshold(fixture, missing),
      (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "MISSING_FIELD",
    );
  });

  test("json and str_list args travel as JSON values", () => {
    const value = { nested: [1, "two", null] };
    const cfg = sz.setSetting(fixture, { name: "foo", value });
    assert.equal(cfg, sz.invoke("set_setting", fixture, { name: "foo", value }).config);
    const out = sz.addComparisonCallResult(fixture, {
      ftypeCode: "NAME_KEY",
      cfuncCode: "EXACT_COMP",
      elementList: ["FULL_NAME", "GIVEN_NAME"],
    });
    assert.match(out, /"CFCALL_ID":1000/);
  });

  test("functions with only optional args default options to {}", () => {
    assert.equal(typeof sz.listDataSources(fixture), "string");
  });
});

describe("return shapes", () => {
  test("json returns JSON text, not a parsed object", () => {
    const out = sz.listDataSources(fixture);
    assert.equal(typeof out, "string");
    assert.ok(Array.isArray(JSON.parse(out)));
  });

  test("config_and_json: the config text, and the record from <name>Result", () => {
    const options = { attribute: "my_attr", feature: "name", element: "full_name", class: "OTHER" };
    const cfg: string = sz.addAttribute(fixture, options);
    const row: string = sz.addAttributeResult(fixture, options);
    assert.equal(typeof cfg, "string");
    assert.equal(typeof row, "string");
    assert.equal((JSON.parse(row) as { ATTR_CODE: string }).ATTR_CODE, "MY_ATTR");
    assert.match(sz.getAttribute(cfg, { code: "MY_ATTR" }), /"ATTR_CODE":"MY_ATTR"/);
  });

  test("json tuple_names: a camelCase record of JSON texts", () => {
    const v: sz.VerifyCompatibilityVersionRecord = sz.verifyCompatibilityVersion(fixture, {
      expectedVersion: "11",
    });
    assert.deepEqual(v, { currentVersion: '"11"', matches: "true" });
    const miss = sz.verifyCompatibilityVersion(fixture, { expectedVersion: "12" });
    assert.deepEqual(miss, { currentVersion: '"11"', matches: "false" });
  });

  test("config_and_json tuple_names: config text, and JSON texts from <name>Result", () => {
    const options = { gplanCode: "my_plan", gplanDesc: "mine" };
    const created: sz.SetGenericPlanRecord = sz.setGenericPlanResult(fixture, options);
    assert.deepEqual(Object.keys(created).sort(), ["planId", "wasCreated"]);
    assert.equal(created.planId, "3");
    assert.equal(created.wasCreated, "true");
    const cfg: string = sz.setGenericPlan(fixture, options);
    assert.equal(cfg, sz.invoke("set_generic_plan", fixture, {
      gplan_code: "my_plan",
      gplan_desc: "mine",
    }).config);
    const updated = sz.setGenericPlanResult(cfg, { gplanCode: "MY_PLAN", gplanDesc: "x" });
    assert.equal(updated.planId, "3");
    assert.equal(updated.wasCreated, "false");
  });

  test("unit returns undefined", () => {
    assert.equal(sz.validateConfig(fixture), undefined);
  });
});

describe("int_or_str call selector", () => {
  test("number = call id, string = feature code; both select the same call", () => {
    const byId = sz.getComparisonCall(fixture, { call: 1 });
    const row = JSON.parse(byId) as { CFCALL_ID: number; FTYPE_ID: number };
    assert.equal(row.CFCALL_ID, 1);
    const ftypes = (JSON.parse(fixture) as FtypeConfig).G2_CONFIG.CFG_FTYPE;
    const name = ftypes.find((t) => t.FTYPE_CODE === "NAME");
    assert.equal(name?.FTYPE_ID, row.FTYPE_ID, "fixture call 1 is bound to NAME");
    assert.equal(sz.getComparisonCall(fixture, { call: "NAME" }), byId);
    assert.equal(sz.getComparisonCall(fixture, { call: "name" }), byId);
  });

  test("the wire value is a JSON integer or string", () => {
    assert.equal(
      sz.getComparisonCall(fixture, { call: 1 }),
      sz.invoke("get_comparison_call", fixture, { call: 1 }).result,
    );
    assert.equal(
      sz.getComparisonCall(fixture, { call: "NAME" }),
      sz.invoke("get_comparison_call", fixture, '{"call":"NAME"}').result,
    );
  });

  test("non-integer numbers and other types are INVALID_INPUT", () => {
    for (const call of [1.5, true, null, ["NAME"]]) {
      assert.throws(
        () => sz.getComparisonCall(fixture, { call: call as unknown as number }),
        (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "INVALID_INPUT",
        JSON.stringify(call),
      );
    }
  });
});

describe("config opacity", () => {
  test("typed result is byte-identical to the native envelope's config", () => {
    const native = loadNative().invoke("add_data_source", fixture, '{"code":"crm"}');
    assert.equal(sz.addDataSource(fixture, { code: "crm" }), native.config);
  });

  test("non-ASCII, escapes and line separators survive byte-exact", () => {
    const tricky = "Ünïcødé   \"q\" \\ \u{1F600}";
    const cfg = sz.addDataSource(fixture, { code: "crm" });
    const args = { parameterName: "RELATIONSHIPSBREAKMATCHES", parameterValue: tricky };
    const native = loadNative().invoke(
      "set_system_parameter",
      cfg,
      JSON.stringify({ parameter_name: args.parameterName, parameter_value: tricky }),
    );
    const typed = sz.setSystemParameter(cfg, args);
    assert.equal(typed, native.config);
    assert.ok(typed.includes("\u{1F600}") && typed.includes("Ünïcødé"));
  });

  test("the ~300KB template passes through without truncation", () => {
    assert.ok(fixture.length > 100_000);
    const out = sz.addDataSource(fixture, { code: "crm" });
    assert.equal(out, loadNative().invoke("add_data_source", fixture, '{"code":"crm"}').config);
    assert.ok(out.length > 100_000);
    assert.match(sz.getDataSource(out, { code: "CRM" }), /"DSRC_CODE":"CRM"/);
  });
});
