/** Inputs that JSON / napi would silently change are rejected (INVALID_INPUT). */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import * as rt from "../dist/runtime.js";
import { fixture, packageDir } from "./helpers.ts";

function code(fn: () => unknown): string {
  try {
    fn();
  } catch (e) {
    assert.ok(e instanceof sz.SzConfigToolError, `not an SzConfigToolError: ${String(e)}`);
    return e.code;
  }
  assert.fail("expected a throw");
}

const RELATE_RULE = "CNAME_CFF_DEXCL"; // RESOLVE=No: clearing its tier would succeed

describe("lone UTF-16 surrogates are INVALID_INPUT (napi would turn them into U+FFFD)", () => {
  const cases: Array<[string, () => unknown]> = [
    ["str arg", () => sz.addDataSource(fixture, { code: "A\uD800" })],
    ["str arg (lone low)", () => sz.addDataSource(fixture, { code: "\uDC00B" })],
    ["config", () => sz.listDataSources(fixture + "\uD83D")],
    ["function name", () => sz.invoke("list\uD800", fixture)],
    ["nested json value", () => sz.addFeature(fixture, { feature: "F1", elementList: ["E\uD800"] })],
    ["nested json key", () => sz.setSetting(fixture, { name: "S1", value: { ["e\uDFFF"]: "x" } })],
    ["raw args_json text", () => sz.invoke("add_data_source", fixture, '{"code":"A\uD800"}')],
  ];
  for (const [label, fn] of cases) {
    test(label, () => assert.equal(code(fn), "INVALID_INPUT"));
  }

  test("a valid surrogate pair round-trips exactly", () => {
    const added = sz.addDataSource(fixture, { code: "E😀" });
    const row = JSON.parse(sz.getDataSource(added, { code: "E😀" })) as { DSRC_CODE: string };
    assert.equal(row.DSRC_CODE, "E😀");
  });
});

describe("numbers JSON cannot carry exactly are INVALID_INPUT (never null / rounded)", () => {
  for (const bad of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
    test(`tri-state int ${bad} is not Clear`, () => {
      assert.equal(code(() => sz.setRule(fixture, { code: RELATE_RULE, tier: bad })), "INVALID_INPUT");
    });
  }
  test("unsafe integer int arg", () => {
    assert.equal(code(() => sz.addDataSource(fixture, { code: "A", id: 2 ** 53 })), "INVALID_INPUT");
  });
  test("unsafe integer call selector", () => {
    assert.equal(code(() => sz.getComparisonCall(fixture, { call: -(2 ** 53) })), "INVALID_INPUT");
  });
  test("non-finite number inside a json arg", () => {
    assert.equal(
      code(() => sz.setSetting(fixture, { name: "S1", value: { level: Number.NaN } })),
      "INVALID_INPUT",
    );
  });
  test("null still clears a tri-state int", () => {
    const out = sz.setRule(fixture, { code: RELATE_RULE, tier: null });
    assert.equal(typeof out, "string");
  });
  test("safe integers pass", () => {
    const out = sz.addDataSource(fixture, { code: "A", id: Number.MAX_SAFE_INTEGER });
    assert.equal(typeof out, "string");
  });
});

describe("version accessors", () => {
  const cargo = readFileSync(join(packageDir, "..", "..", "Cargo.toml"), "utf8");
  const workspace = /\[workspace\.package\][^[]*?\nversion = "([^"]+)"/.exec(cargo)?.[1];
  test("libraryVersion() is the workspace version", () => {
    assert.ok(workspace);
    assert.equal(sz.libraryVersion(), workspace);
  });
  test("abiVersion() is the C ABI version", () => assert.equal(sz.abiVersion(), 2));
});

describe("bigint int args carry the full i64 range exactly", () => {
  const I64_MAX = 2n ** 63n - 1n;
  const I64_MIN = -(2n ** 63n);
  test("2^63-1 data source id round-trips digit-exact", () => {
    const added = sz.addDataSource(fixture, { code: "BIG", id: I64_MAX });
    const text = sz.getDataSource(added, { code: "BIG" });
    assert.match(text, /"DSRC_ID":\s*9223372036854775807[,}]/);
  });
  test("a bigint call selector is accepted", () => {
    const call = JSON.parse(sz.getComparisonCall(fixture, { call: "NAME" })) as { CFCALL_ID: number };
    const byId = sz.getComparisonCall(fixture, { call: BigInt(call.CFCALL_ID) });
    assert.equal(byId, sz.getComparisonCall(fixture, { call: call.CFCALL_ID }));
  });
  test("bigint through raw invoke args", () => {
    const out = sz.invoke("add_data_source", fixture, { code: "BIG2", id: 2n ** 62n + 1n });
    assert.equal(out.kind, "config");
    assert.match(sz.getDataSource(out.config!, { code: "BIG2" }), /"DSRC_ID":\s*4611686018427387905[,}]/);
  });
  test("bigint beyond i64 is INVALID_INPUT", () => {
    assert.equal(code(() => sz.addDataSource(fixture, { code: "A", id: I64_MAX + 1n })), "INVALID_INPUT");
    assert.equal(code(() => sz.addDataSource(fixture, { code: "A", id: I64_MIN - 1n })), "INVALID_INPUT");
  });
  test("unsafe number is still INVALID_INPUT", () => {
    assert.equal(code(() => sz.addDataSource(fixture, { code: "A", id: 2 ** 63 })), "INVALID_INPUT");
  });
  test("i64::MIN is accepted on the wire (<= 0 = auto-allocate for a data source id)", () => {
    assert.equal(rt.wireJson({ id: I64_MIN }), '{"id":-9223372036854775808}');
    assert.equal(typeof sz.addDataSource(fixture, { code: "BIG3", id: I64_MIN }), "string");
  });
  test("bigint wire JSON keeps undefined members absent", () => {
    assert.equal(rt.wireJson({ a: 1n, b: undefined, c: [1, 2n, "x"], d: null }), '{"a":1,"c":[1,2,"x"],"d":null}');
  });
});

describe("named results are each member's exact JSON text (no re-serialization)", () => {
  test("nested object, exponent, escapes and spacing survive", () => {
    const m = rt.memberTexts('{ "a" : {"x" : 1.0, "y":"\\u00e9"}, "b": 1e3, "c":"\\u00e9", "d" : [ 1 , 2 ] }');
    assert.equal(m.get("a"), '{"x" : 1.0, "y":"\\u00e9"}');
    assert.equal(m.get("b"), "1e3");
    assert.equal(m.get("c"), '"\\u00e9"');
    assert.equal(m.get("d"), "[ 1 , 2 ]");
  });
  test("strings containing braces, quotes and backslashes", () => {
    const m = rt.memberTexts('{"k\\"}":"v}\\\\\\"{","n":null,"t":true,"z":-0.5E+2}');
    assert.equal(m.get('k"}'), '"v}\\\\\\"{"');
    assert.equal(m.get("n"), "null");
    assert.equal(m.get("t"), "true");
    assert.equal(m.get("z"), "-0.5E+2");
  });
  test("a duplicated key keeps the last value, like JSON.parse", () => {
    assert.equal(rt.memberTexts('{"a":1,"a":2}').get("a"), "2");
  });
  test("non-object text is INTERNAL", () => {
    assert.throws(() => rt.memberTexts("[1]"), (e: unknown) => e instanceof sz.SzConfigToolError && e.code === "INTERNAL");
  });
});
