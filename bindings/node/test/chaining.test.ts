/**
 * Every config-changing function returns the new config TEXT (issue #75): a
 * `config_and_json` function's primary returns only the config; its
 * companion `<name>Result` (same arguments) returns the record JSON text, or
 * the named-fields record for `tuple_names`. Plain-JavaScript style: values
 * are reached through an untyped view of the module, as a `.mjs` script would.
 */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import { camel, fixture, manifest } from "./helpers.ts";

type AnyFn = (config: unknown, options?: Record<string, unknown>) => unknown;
const js = sz as unknown as Record<string, AnyFn | undefined>;

const implemented = manifest.functions.filter((f) => f.status === "implemented");
const changing = implemented.filter((f) => f.returns === "config" || f.returns === "config_and_json");
const paired = implemented.filter((f) => f.returns === "config_and_json");

describe("config-changing functions return the config text", () => {
  test("the manifest has config_and_json functions to split", () => {
    assert.ok(paired.length > 0);
    assert.ok(changing.length > paired.length);
  });

  test("every config_and_json function has a <name>Result companion", () => {
    for (const f of paired) {
      assert.equal(typeof js[camel(f.name)], "function", f.name);
      assert.equal(typeof js[`${camel(f.name)}Result`], "function", `${f.name} companion`);
    }
  });

  test("no other function has a companion", () => {
    for (const f of implemented.filter((g) => g.returns !== "config_and_json")) {
      assert.equal(js[`${camel(f.name)}Result`], undefined, f.name);
    }
  });

  test("chaining 7 different config-changing functions on a plain value", () => {
    const call = (name: string, cfg: unknown, options: Record<string, unknown>): unknown => {
      const out = js[name]!(cfg, options);
      assert.equal(typeof out, "string", `${name} must return the config text`);
      return out;
    };
    let cfg: unknown = fixture;
    cfg = call("addElement", cfg, { code: "DEMO_EL", dataType: "string" });
    cfg = call("addFeature", cfg, { feature: "DEMO_FEAT", elementList: ["DEMO_EL"] });
    cfg = call("addAttribute", cfg, {
      attribute: "DEMO_ATTR",
      feature: "DEMO_FEAT",
      element: "DEMO_EL",
      class: "OTHER",
    });
    cfg = call("addFragment", cfg, {
      fragmentConfig: { ERFRAG_CODE: "DEMO_FRAG", ERFRAG_SOURCE: "./FRAGMENT[./SAME_NAME>0]" },
    });
    cfg = call("addComparisonCall", cfg, {
      ftypeCode: "DEMO_FEAT",
      cfuncCode: "EXACT_COMP",
      elementList: ["DEMO_EL"],
    });
    cfg = call("addComparisonFunction", cfg, { code: "DEMO_COMP" });
    cfg = call("addDataSource", cfg, { code: "DEMO_DS" });
    const config = cfg as string;
    const attr = JSON.parse(sz.getAttribute(config, { code: "DEMO_ATTR" })) as { ATTR_CODE: string };
    assert.equal(attr.ATTR_CODE, "DEMO_ATTR");
    assert.match(sz.getFragment(config, { codeOrId: "DEMO_FRAG" }), /DEMO_FRAG/);
    assert.match(sz.getComparisonFunction(config, { code: "DEMO_COMP" }), /DEMO_COMP/);
    assert.match(sz.listDataSources(config), /DEMO_DS/);
  });

  test("the companion returns the row of the same operation", () => {
    const options = { attribute: "X_ATTR", feature: "NAME", element: "FULL_NAME", class: "OTHER" };
    const row = sz.addAttributeResult(fixture, options);
    assert.equal(typeof row, "string");
    assert.equal((JSON.parse(row) as { ATTR_CODE: string }).ATTR_CODE, "X_ATTR");
    const wire = sz.invoke("add_attribute", fixture, {
      attribute: "X_ATTR",
      feature: "NAME",
      element: "FULL_NAME",
      class: "OTHER",
    });
    assert.equal(row, wire.result);
    assert.equal(sz.addAttribute(fixture, options), wire.config);
  });

  test("tuple_names companion: the named fields only", () => {
    const created: sz.SetGenericPlanRecord = sz.setGenericPlanResult(fixture, {
      gplanCode: "my_plan",
      gplanDesc: "mine",
    });
    assert.deepEqual(created, { planId: "3", wasCreated: "true" });
    const cfg: string = sz.setGenericPlan(fixture, { gplanCode: "my_plan", gplanDesc: "mine" });
    const updated = sz.setGenericPlanResult(cfg, { gplanCode: "MY_PLAN", gplanDesc: "x" });
    assert.deepEqual(updated, { planId: "3", wasCreated: "false" });
  });
});

describe("a non-string config fails early with a clear error", () => {
  const record = { config: fixture, json: "{}" };
  for (const [label, bad] of [
    ["object (a previous call's record)", record],
    ["number", 7],
    ["undefined", undefined],
    ["null", null],
  ] as const) {
    test(label, () => {
      assert.throws(
        () => js["addDataSource"]!(bad, { code: "X" }),
        (e: unknown) => {
          assert.ok(e instanceof sz.SzConfigToolError);
          assert.equal(e.code, "INVALID_INPUT");
          const got = bad === null ? "null" : typeof bad;
          assert.ok(
            e.message.startsWith(
              `config must be a string (the configuration JSON text), got ${got}; ` +
                "if this came from a previous call, use the value that call returned",
            ),
            e.message,
          );
          assert.match(e.message, /use <name>Result for the created row/);
          return true;
        },
      );
    });
  }

  test("raw invoke rejects it the same way", () => {
    assert.throws(
      () => sz.invoke("list_data_sources", record as unknown as string),
      (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "INVALID_INPUT",
    );
  });
});
