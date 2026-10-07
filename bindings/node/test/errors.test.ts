/** Error mapping against the REAL built .node: every reachable reason code. */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import { toSzConfigToolError } from "../dist/errors.js";
import { loadNative } from "../dist/native.js";
import { camel, errorExamples, fixture, manifest, type Json } from "./helpers.ts";

type TypedFn = (config: string, options?: Record<string, Json>) => unknown;

function caught(fn: () => unknown): sz.SzConfigToolError {
  try {
    fn();
  } catch (e) {
    assert.ok(e instanceof sz.SzConfigToolError, `not an SzConfigToolError: ${String(e)}`);
    return e;
  }
  assert.fail("expected a throw");
}

describe("reason codes", () => {
  test("REASON_CODES equals the manifest taxonomy and the native one", () => {
    assert.deepEqual([...sz.REASON_CODES], manifest.reason_codes);
    assert.deepEqual(loadNative().reasonCodes(), manifest.reason_codes);
    assert.equal(sz.REASON_CODES.length, 14);
  });

  const examples = errorExamples();
  // NOT_IMPLEMENTED is kept for `status: not_implemented` functions, of which the manifest has none.
  test("conformance reaches every library reason code (all but INTERNAL / NOT_IMPLEMENTED)", () => {
    const unreachable = new Set(["INTERNAL", "NOT_IMPLEMENTED"]);
    const missing = manifest.reason_codes.filter((c) => !unreachable.has(c) && !examples.has(c));
    assert.deepEqual(missing, []);
  });

  for (const [code, { config, step }] of examples) {
    test(`typed ${camel(step.fn)} -> ${code}`, () => {
      const fn = (sz as unknown as Record<string, TypedFn | undefined>)[camel(step.fn)];
      const args = Object.fromEntries(Object.entries(step.args).map(([k, v]) => [camel(k), v]));
      const err = caught(() =>
        fn ? fn(config, args) : sz.invoke(step.fn, config, step.args),
      );
      assert.equal(err.code, code);
      assert.equal(err.errorType, code);
      assert.equal(err.reasonCode, code);
      assert.equal(err.kind, code, "kind is the reason code");
      assert.equal(err.name, "SzConfigToolError");
      assert.ok(err instanceof Error);
      assert.ok(err.message.length > 0);
      assert.ok(err.cause instanceof Error, "native error kept as cause");
      if (code === "VALIDATION_ERRORS") {
        assert.equal(typeof err.details, "string", "details is JSON text (CONTRACT.md)");
        const parsed = JSON.parse(err.details ?? "null") as sz.ValidationDetails;
        assert.equal(parsed.schema, "sz-configtool.validation-errors/v1");
        assert.ok(parsed.failures.length > 0);
        assert.deepEqual(err.validationDetails(), parsed, "typed helper parses details");
      } else {
        assert.equal(err.details, undefined);
        assert.equal(err.validationDetails(), undefined);
      }
    });
  }
});

describe("wire errors", () => {
  const cases: Array<[string, () => unknown, string]> = [
    ["unknown function", () => sz.invoke("no_such_fn", fixture), "INVALID_INPUT"],
    ["args not JSON", () => sz.invoke("list_data_sources", fixture, "{"), "INVALID_INPUT"],
    ["args not object", () => sz.invoke("list_data_sources", fixture, "[]"), "INVALID_INPUT"],
    [
      "unknown arg",
      () => sz.invoke("add_data_source", fixture, { code: "a", cdoe: "b" }),
      "INVALID_INPUT",
    ],
    ["mistyped arg", () => sz.invoke("add_data_source", fixture, { code: 1 }), "INVALID_INPUT"],
    [
      "non-integer call selector",
      () => sz.getComparisonCall(fixture, { call: 1.5 }),
      "INVALID_INPUT",
    ],
    [
      "non-integer int",
      () => sz.addDataSource(fixture, { code: "a", id: 1.5 }),
      "INVALID_INPUT",
    ],
    ["missing required", () => sz.invoke("add_data_source", fixture, {}), "MISSING_FIELD"],
    ["config not JSON", () => sz.listDataSources("not json"), "JSON_PARSE"],
  ];
  for (const [label, fn, code] of cases) {
    test(`${label} -> ${code}`, () => assert.equal(caught(fn).code, code));
  }

  test("a JS value napi cannot convert is INVALID_INPUT with the native cause", () => {
    // (A non-string `config` is rejected before the native call; a
    // non-string function name still reaches napi.)
    const err = caught(() => sz.invoke(42 as unknown as string, fixture));
    assert.equal(err.code, "INVALID_INPUT");
    assert.equal(err.kind, "INVALID_INPUT");
    assert.ok(err.cause instanceof Error);
  });

  test("unserializable args (a circular json value) are INVALID_INPUT", () => {
    const value: Record<string, unknown> = { a: 1 };
    value["self"] = value;
    const err = caught(() => sz.setSetting(fixture, { name: "S1", value: value as sz.JsonValue }));
    assert.equal(err.code, "INVALID_INPUT");
  });
});

describe("raw native error object", () => {
  test("carries code, reasonCode and kind (thrown by Rust)", () => {
    try {
      loadNative().invoke("get_data_source", fixture, '{"code":"NOPE"}');
      assert.fail("expected a throw");
    } catch (e) {
      const n = e as { code: string; reasonCode: string; kind: string };
      assert.ok(e instanceof Error);
      assert.equal(n.code, "NOT_FOUND");
      assert.equal(n.reasonCode, "NOT_FOUND");
      assert.equal(n.kind, "NOT_FOUND");
    }
  });

  // The typed layer always passes argsJson as a string; only a direct native
  // caller can hand napi a value it cannot convert (no reason code: napi's own).
  for (const argsJson of [42, {}]) {
    test(`a non-string argsJson (${JSON.stringify(argsJson)}) is napi's StringExpected`, () => {
      const native = loadNative() as unknown as { invoke(n: string, c: string, a: unknown): unknown };
      assert.throws(
        () => native.invoke("list_data_sources", fixture, argsJson),
        (e: unknown) =>
          e instanceof Error &&
          (e as { code?: string }).code === "StringExpected" &&
          (e as { reasonCode?: string }).reasonCode === undefined &&
          /into rust type `String`/.test(e.message),
      );
    });
  }
});

describe("toSzConfigToolError (anything thrown at the native boundary)", () => {
  test("an SzConfigToolError passes through unchanged", () => {
    const e = new sz.SzConfigToolError("NOT_FOUND", "x");
    assert.equal(toSzConfigToolError(e), e);
  });

  const inputs: Array<[string, unknown, string]> = [
    ["a thrown string", "boom", "boom"],
    ["a thrown null", null, "null"],
    ["an object without a string message", { message: 5 }, "[object Object]"],
    ["an object with an unknown reason code", { message: "m", reasonCode: "NOPE" }, "m"],
  ];
  for (const [label, thrown, message] of inputs) {
    test(`${label} is INVALID_INPUT with the value as cause`, () => {
      const err = toSzConfigToolError(thrown);
      assert.equal(err.code, "INVALID_INPUT");
      assert.equal(err.message, message);
      assert.equal(err.cause, thrown);
      assert.equal(err.details, undefined);
    });
  }

  test("a reason code keeps its code; non-string details are dropped", () => {
    const err = toSzConfigToolError({ message: "m", reasonCode: "VALIDATION_ERRORS", details: 7 });
    assert.equal(err.code, "VALIDATION_ERRORS");
    assert.equal(err.details, undefined);
  });
});
