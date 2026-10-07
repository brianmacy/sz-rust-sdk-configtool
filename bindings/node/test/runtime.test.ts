/** The runtime call helpers' envelope shaping, against the REAL built .node. */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import * as rt from "../dist/runtime.js";
import { fixture } from "./helpers.ts";

function internal(fn: () => unknown, message: RegExp): void {
  assert.throws(
    fn,
    (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "INTERNAL" && message.test(e.message),
  );
}

// No manifest function is `returns: int` today, so callInt is driven with a
// real `config_and_json` function whose result is integer JSON (rows changed).
const INT_FN = "remove_config_section_field";
const INT_ARGS = { section_name: "cfg_gplan", field_name: "gplan_desc" };
const INT_SPEC: rt.FnSpec = {
  name: "removeConfigSectionField",
  wire: INT_FN,
  args: [
    ["sectionName", "section_name", true],
    ["fieldName", "field_name", true],
  ],
};

/** A spec passing wire-named options through unchanged. */
function wireSpec(wire: string, ...names: string[]): rt.FnSpec {
  return { name: wire, wire, args: names.map((n) => [n, n, false] as const) };
}

describe("callInt", () => {
  test("parses a real native integer result", () => {
    const wire = sz.invoke(INT_FN, fixture, INT_ARGS);
    assert.equal(wire.kind, "config_and_json");
    assert.equal(wire.result, "2");
    assert.equal(rt.callInt(INT_SPEC, fixture, { sectionName: "cfg_gplan", fieldName: "gplan_desc" }), 2);
    assert.equal(rt.callInt(INT_SPEC, fixture, { sectionName: "CFG_GPLAN", fieldName: "NO_SUCH_FIELD" }), 0);
  });

  test("a native error propagates as its reason code", () => {
    assert.throws(
      () => rt.callInt(INT_SPEC, fixture, { sectionName: "CFG_NOPE", fieldName: "X" }),
      (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "NOT_FOUND",
    );
  });
});

describe("an envelope without the member a helper needs is INTERNAL", () => {
  test("callConfig on a json-only function (no config)", () => {
    internal(() => rt.callConfig(wireSpec("list_data_sources"), fixture), /^list_data_sources: native envelope has no config$/);
  });
  test("callJson on a config-only function (no result)", () => {
    internal(() => rt.callJson(wireSpec("add_data_source", "code"), fixture, { code: "x" }), /^add_data_source: native envelope has no result$/);
  });
});

describe("callNamed", () => {
  test("a member the result lacks is the JSON text null", () => {
    const out = rt.callNamed<Record<string, string>>(
      wireSpec("verify_compatibility_version", "expected_version"),
      fixture,
      { expected_version: "11" },
      [
        ["current_version", "currentVersion"],
        ["absent_member", "absentMember"],
      ],
    );
    assert.deepEqual(out, { currentVersion: '"11"', absentMember: "null" });
  });
});

describe("wire helpers", () => {
  test("an undefined array item is written as null (like JSON.stringify)", () => {
    const items = [undefined, 1n, "x"];
    assert.equal(rt.wireJson(items), "[null,1,\"x\"]");
    assert.equal(rt.wireJson([undefined]), JSON.stringify([undefined]));
  });
  test("memberTexts of text that is not JSON is INTERNAL", () => {
    internal(() => rt.memberTexts("not json"), /^native result is not JSON: SyntaxError/);
  });
});
