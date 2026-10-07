/**
 * Strict options (issue #76), against the REAL built .node: unknown option
 * keys, non-object option containers and mistyped structured options are
 * INVALID_INPUT before the native call; native errors name the JS option.
 */
import assert from "node:assert/strict";
import { describe, test } from "node:test";

import * as sz from "../dist/index.js";
import * as rt from "../dist/runtime.js";
import { byName, camel, fixture, manifest, type Json } from "./helpers.ts";

type TypedFn = (config: string, options?: unknown) => unknown;
const typed = sz as unknown as Record<string, TypedFn | undefined>;

function caught(fn: () => unknown): sz.SzConfigToolError {
  try {
    fn();
  } catch (e) {
    assert.ok(e instanceof sz.SzConfigToolError, `not an SzConfigToolError: ${String(e)}`);
    return e;
  }
  assert.fail("expected a throw");
}

function invalid(fn: () => unknown, message: string | RegExp): sz.SzConfigToolError {
  const err = caught(fn);
  assert.equal(err.code, "INVALID_INPUT", err.message);
  if (typeof message === "string") assert.equal(err.message, message);
  else assert.match(err.message, message);
  return err;
}

/** The typed functions (+ companions) of every implemented manifest function with args. */
const optionFns = manifest.functions
  .filter((f) => f.status === "implemented" && f.args.length > 0)
  .flatMap((f) => {
    const names = [camel(f.name)];
    if (f.returns === "config_and_json") names.push(`${camel(f.name)}Result`);
    return names.map((name) => ({ f, name }));
  });

describe("issue #76 regressions", () => {
  test("(1) unknown option keys are rejected, not silently dropped", () => {
    const opts = {
      code: "P2",
      genericPlan: "SEARCH",
      candidate: "Off",
      descripton: "typo",
      bogus: 1,
    };
    invalid(
      () => sz.addSearchProfile(fixture, opts as unknown as sz.AddSearchProfileOptions),
      "unknown options for addSearchProfile: 'candidate' (did you mean 'candidates'?), " +
        "'descripton' (did you mean 'description'?), 'bogus'; valid options: code, genericPlan, " +
        "candidates, description, elements",
    );
  });

  test("(2) a string where the options object belongs is rejected", () => {
    invalid(
      () => typed["listSearchProfiles"]!(fixture, "NOSUCH"),
      'listSearchProfiles: options must be a plain object, got string; pass { filter: "NOSUCH" }',
    );
    invalid(
      () => typed["getSearchProfile"]!(fixture, "SEARCH"),
      'getSearchProfile: options must be a plain object, got string; pass { code: "SEARCH" }',
    );
    assert.equal(sz.listSearchProfiles(fixture, { filter: "NOSUCH" }), "[]");
  });

  test("(3) native errors name the JS option, with the wire name once", () => {
    const one = caught(() =>
      sz.addSearchProfile(fixture, { code: "X" } as unknown as sz.AddSearchProfileOptions),
    );
    assert.equal(one.code, "MISSING_FIELD");
    assert.equal(one.message, "Missing required field: genericPlan (generic_plan)");
    assert.ok(one.cause instanceof Error, "the native error is kept as cause");
    const many = caught(() =>
      sz.addGenericThreshold(fixture, { plan: "INGEST", behavior: "NAME" } as unknown as sz.AddGenericThresholdOptions),
    );
    assert.equal(many.code, "MISSING_FIELD");
    assert.equal(
      many.message,
      "Missing required field: scoringCap (scoring_cap), candidateCap (candidate_cap), sendToRedo (send_to_redo)",
    );
  });

  test("(4) structured options are validated by shape, naming the path", () => {
    invalid(
      () => sz.addFeature(fixture, { feature: "FX", elementList: '["A"]' as unknown as readonly string[] }),
      "addFeature: elementList must be an array, got string",
    );
    invalid(
      () =>
        sz.addSearchProfile(fixture, {
          code: "P2",
          genericPlan: "SEARCH",
          elements: [
            { feature: "NAME", flag: "Yes" },
            { feature: "ADDRESS", flag: "Maybe" as "Yes" },
          ],
        }),
      'addSearchProfile: elements[1].flag must be one of Yes, No, Y, N (got "Maybe")',
    );
    const cfg = sz.addSearchProfile(fixture, {
      code: "P2",
      genericPlan: "SEARCH",
      candidates: "Off",
      description: "typed",
      elements: [{ feature: "NAME", flag: "N" }],
    });
    const row = JSON.parse(sz.getSearchProfile(cfg, { code: "P2" })) as Record<string, Json>;
    assert.equal(row["DEFAULT_USED_FOR_CAND"], "Off");
    assert.equal(row["SPROFILE_DESC"], "typed");
  });
});

describe("did you mean", () => {
  const cases: Array<[string, () => unknown, string]> = [
    [
      "edit distance",
      () => typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", candidate: "Off" }),
      "unknown option 'candidate' for addSearchProfile; did you mean 'candidates'?",
    ],
    [
      "suffix of a longer option",
      () => typed["addSearchProfile"]!(fixture, { code: "X", plan: "SEARCH" }),
      "unknown option 'plan' for addSearchProfile; did you mean 'genericPlan'?",
    ],
    [
      "the wire (snake_case) name",
      () => typed["addSearchProfile"]!(fixture, { code: "X", generic_plan: "SEARCH" }),
      "unknown option 'generic_plan' for addSearchProfile; did you mean 'genericPlan'?",
    ],
    [
      "case only",
      () => typed["addDataSource"]!(fixture, { CODE: "X" }),
      "unknown option 'CODE' for addDataSource; did you mean 'code'?",
    ],
    [
      "nothing close lists the valid options",
      () => typed["addDataSource"]!(fixture, { code: "X", zzz: 1 }),
      "unknown option 'zzz' for addDataSource; valid options: code, retentionLevel, id",
    ],
    [
      "the companion names itself",
      () => typed["addAttributeResult"]!(fixture, { attribute: "A", feature: "NAME", elemnt: "X" }),
      "unknown option 'elemnt' for addAttributeResult; did you mean 'element'?",
    ],
  ];
  for (const [label, fn, message] of cases) {
    test(label, () => {
      invalid(fn, message);
    });
  }
});

test("several unknown options, each with a suggestion, omit the valid list", () => {
  invalid(
    () => typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", candidate: "Off", descripton: "x" }),
    "unknown options for addSearchProfile: 'candidate' (did you mean 'candidates'?), " +
      "'descripton' (did you mean 'description'?)",
  );
});

test("wireArgs: every shape kind (synthetic spec, no native call)", () => {
  const spec: rt.FnSpec = {
    name: "fn",
    wire: "fn",
    args: [["v", "v_wire", false, { array: { oneOf: ["bool", "int", { object: { "a?": "any" } }] } }]],
  };
  assert.deepEqual(rt.wireArgs(spec, { v: [true, 1, 2n, { a: [null] }, {}] }), {
    v_wire: [true, 1, 2n, { a: [null] }, {}],
  });
  invalid(() => rt.wireArgs(spec, { v: ["s"] }), "fn: v[0] must be a boolean, an integer or an object, got string");
  invalid(
    () => rt.wireArgs({ ...spec, args: [["b", "b", true, "bool"]] }, { b: 1 }),
    "fn: b must be a boolean, got number",
  );
  invalid(
    () => rt.wireArgs({ ...spec, args: [["e", "e", true, { enum: ["A"] }]] }, { e: 3 }),
    "fn: e must be one of A (got number)",
  );
  assert.equal(rt.closest("zz", ["code", "id"]), undefined);
  assert.equal(rt.closest("ID", ["code", "id"]), "id");
});

describe("nested structured options", () => {
  test("an unknown key inside an element entry", () => {
    invalid(
      () =>
        typed["addSearchProfile"]!(fixture, {
          code: "P",
          genericPlan: "SEARCH",
          elements: [{ feature: "NAME", flg: "Yes" }],
        }),
      "unknown key 'flg' in addSearchProfile elements[0]; did you mean 'flag'?",
    );
    invalid(
      () => typed["addFeature"]!(fixture, { feature: "F", elementList: [{ element: "E", displayLevel: 1 }] }),
      "unknown key 'displayLevel' in addFeature elementList[0]; did you mean 'displaylevel'?",
    );
    invalid(
      () =>
        typed["addRule"]!(fixture, {
          id: 0,
          ruleConfig: { ERRULE_CODE: "R", QUAL_ERFRAG_CODE: "SAME_NAME", TIER: 5, zz: 1 },
        }),
      "unknown keys in addRule ruleConfig: 'TIER' (did you mean 'ERRULE_TIER'?), 'zz'; valid keys: " +
        "ERRULE_CODE, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, RESOLVE, RELATE, RTYPE_ID, ERRULE_TIER, ERRULE_ID",
    );
  });

  test("a missing required key is MISSING_FIELD naming the path", () => {
    const err = caught(() =>
      typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", elements: [{ feature: "NAME" }] }),
    );
    assert.equal(err.code, "MISSING_FIELD");
    assert.equal(err.message, "addSearchProfile: missing required key elements[0].flag");
  });

  test("wrong types name the path and the received type", () => {
    const cases: Array<[unknown, string]> = [
      [[1], "addFeature: elementList[0] must be a string or an object, got number"],
      [[null], "addFeature: elementList[0] must be a string or an object, got null"],
      [[{ element: 5 }], "addFeature: elementList[0].element must be a string, got number"],
      [[{ element: "E", displaylevel: "1" }], "addFeature: elementList[0].displaylevel must be an integer, got string"],
      [[{ element: "E", displaylevel: 1.5 }], "addFeature: elementList[0].displaylevel must be an integer, got number"],
      [[["E"]], "addFeature: elementList[0] must be a string or an object, got array"],
      [[new Map()], "addFeature: elementList[0] must be a string or an object, got Map"],
      [[true], "addFeature: elementList[0] must be a string or an object, got boolean"],
    ];
    for (const [elementList, message] of cases) {
      invalid(() => typed["addFeature"]!(fixture, { feature: "F", elementList }), message);
    }
    invalid(
      () => typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", elements: [{ feature: "NAME", flag: true }] }),
      "addSearchProfile: elements[0].flag must be one of Yes, No, Y, N (got boolean)",
    );
    invalid(
      () => typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", elements: ["NAME"] }),
      "addSearchProfile: elements[0] must be an object, got string",
    );
  });

  test("valid structured values pass (bigint ints, any-typed json, string entries)", () => {
    const withFeature = sz.addFeature(fixture, {
      feature: "F_TYPED",
      elementList: ["E_ONE", { element: "E_TWO", displaylevel: 0n, derived: "No" }],
    });
    assert.match(sz.getFeature(withFeature, { feature: "F_TYPED" }), /E_TWO/);
    const withSetting = sz.setSetting(fixture, { name: "S1", value: { any: ["shape", 1, null] } });
    assert.equal(typeof withSetting, "string");
    const withRule = sz.addRule(fixture, {
      id: 0,
      ruleConfig: { ERRULE_CODE: "R_TYPED", QUAL_ERFRAG_CODE: "SAME_NAME", ERRULE_TIER: 5n, RESOLVE: "Yes" },
    });
    assert.equal(typeof withRule, "string");
  });
});

describe("option containers", () => {
  const bad: Array<[unknown, string]> = [
    [42, "number"],
    [true, "boolean"],
    [["a"], "array"],
    [null, "null"],
    [new Date(0), "Date"],
    [Object.create(null) as unknown, ""],
  ];

  test("a null-prototype object is a plain object", () => {
    const o = Object.create(null) as Record<string, unknown>;
    o["code"] = "NULLPROTO";
    assert.equal(typeof typed["addDataSource"]!(fixture, o), "string");
  });

  test("every options-taking function rejects a non-object container", () => {
    for (const { name } of optionFns) {
      const fn = typed[name];
      assert.equal(typeof fn, "function", name);
      for (const [value, type] of bad) {
        if (type === "") continue;
        invalid(() => fn!(fixture, value), new RegExp(`^${name}: options must be a plain object, got ${type}`));
      }
    }
  });

  test("a string hints the single (required) option; none when ambiguous", () => {
    invalid(
      () => typed["addDataSource"]!(fixture, "CRM"),
      'addDataSource: options must be a plain object, got string; pass { code: "CRM" }',
    );
    invalid(
      () => typed["addAttribute"]!(fixture, "X"),
      "addAttribute: options must be a plain object, got string",
    );
  });

  test("options are required unless every option is optional", () => {
    for (const { f, name } of optionFns) {
      const anyRequired = f.args.some((a) => !(a.optional || a.tristate) || a.required);
      if (!anyRequired) continue;
      invalid(
        () => typed[name]!(fixture),
        new RegExp(`^${name}: options are required \\(required: [a-zA-Z, ]+\\), got undefined$`),
      );
    }
    assert.equal(typeof sz.listSearchProfiles(fixture), "string", "all-optional: omitted = {}");
  });

  test("every options-taking function rejects an unknown key (manifest-driven)", () => {
    for (const { name } of optionFns) {
      invalid(
        () => typed[name]!(fixture, { zzUnknownOption: 1 }),
        new RegExp(`^unknown option 'zzUnknownOption' for ${name}; valid options: `),
      );
    }
    assert.ok(optionFns.length > 100, `${optionFns.length} options-taking functions`);
  });

  test("an explicitly undefined option is absent (not unknown)", () => {
    const out = typed["addDataSource"]!(fixture, { code: "U1", id: undefined });
    assert.equal(typeof out, "string");
  });
});

describe("wire -> JS name translation", () => {
  test("binding-side value checks name the JS option path", () => {
    invalid(
      () =>
        sz.addExpressionCall(fixture, {
          efuncCode: "PARSE_NAME",
          elementList: [{ element: "E\uD800", required: "Yes" }],
          isVirtual: "No",
        }),
      "elementList[0].element contains a lone UTF-16 surrogate (not valid Unicode)",
    );
  });

  test("translateError: messages, details and pass-through", () => {
    const spec: rt.FnSpec = {
      name: "addThing",
      wire: "add_thing",
      args: [
        ["genericPlan", "generic_plan", true],
        ["code", "code", false],
      ],
    };
    const details = JSON.stringify({
      schema: "sz-configtool.validation-errors/v1",
      failures: [
        { field: "generic_plan", reasonCode: "OUT_OF_DOMAIN", offendingValue: "X" },
        { field: "code", reasonCode: "DUPLICATE", offendingValue: null },
      ],
    });
    const v = rt.translateError(spec, new sz.SzConfigToolError("VALIDATION_ERRORS", "bad", details));
    assert.equal(v.message, "bad");
    assert.deepEqual(
      v.validationDetails()?.failures.map((f) => f.field),
      ["genericPlan", "code"],
    );
    const same = new sz.SzConfigToolError("VALIDATION_ERRORS", "bad", '{"failures":[{"field":"code"}]}');
    assert.equal(rt.translateError(spec, same), same, "nothing to translate: same error");
    const nf = new sz.SzConfigToolError("NOT_FOUND", "Plan not found: generic_plan");
    assert.equal(rt.translateError(spec, nf), nf, "only MISSING_FIELD / INVALID_INPUT messages");
    const repeated = rt.translateError(
      spec,
      new sz.SzConfigToolError("MISSING_FIELD", "Missing required field: generic_plan, generic_plan, generic_plans"),
    );
    assert.equal(repeated.message, "Missing required field: genericPlan (generic_plan), genericPlan, generic_plans");
    for (const odd of ["not json", "{}", "null", '{"failures":[{"field":1}]}']) {
      const e = new sz.SzConfigToolError("VALIDATION_ERRORS", "bad", odd);
      assert.equal(rt.translateError(spec, e), e, `details ${odd} left as is`);
    }
    const native = new Error("plain");
    assert.equal(rt.translateError(spec, native), native, "not an SzConfigToolError: untouched");
  });
});

test("manifest json_type is emitted for every json arg", () => {
  for (const f of manifest.functions) {
    for (const a of f.args) {
      if (a.type === "json") assert.ok(a.json_type !== undefined, `${f.name}.${a.name}`);
      else assert.equal(a.json_type, undefined, `${f.name}.${a.name}`);
    }
  }
  assert.deepEqual(byName.get("add_search_profile")?.args.find((a) => a.name === "elements")?.json_type, {
    array: { object: { feature: "string", flag: { enum: ["Yes", "No", "Y", "N"] } } },
  });
});

describe("review fixes (PR #77)", () => {
  type Row = Record<string, Json>;
  const section = (name: string): Row[] => JSON.parse(sz.getConfigSection(fixture, { sectionName: name })) as Row[];

  test("every template CFG_ERRULE row round-trips through addRule (optional keys may be null)", () => {
    const rows = section("CFG_ERRULE");
    assert.equal(rows.length, 38);
    assert.ok(rows.some((r) => r["DISQ_ERFRAG_CODE"] === null) && rows.some((r) => r["ERRULE_TIER"] === null));
    for (const row of rows) {
      const ruleConfig = { ...row, ERRULE_CODE: `${String(row["ERRULE_CODE"])}_RT` };
      const cfg = sz.addRule(fixture, { id: 0, ruleConfig: ruleConfig as unknown as sz.AddRuleOptions["ruleConfig"] });
      assert.equal(typeof cfg, "string", String(row["ERRULE_CODE"]));
    }
  });

  test("every template CFG_ERFRAG row round-trips through addFragment", () => {
    const rows = section("CFG_ERFRAG");
    assert.equal(rows.length, 96);
    assert.ok(rows.some((r) => r["ERFRAG_DEPENDS"] === null));
    for (const row of rows) {
      const fragmentConfig = { ...row, ERFRAG_CODE: `${String(row["ERFRAG_CODE"])}_RT`, ERFRAG_ID: null };
      const cfg = sz.addFragment(fixture, {
        fragmentConfig: fragmentConfig as unknown as sz.AddFragmentOptions["fragmentConfig"],
      });
      assert.equal(typeof cfg, "string", String(row["ERFRAG_CODE"]));
    }
  });

  test("null is absent for optional element keys the library reads with as_str / as_i64", () => {
    const cfg = sz.addFeature(fixture, {
      feature: "F_NULLS",
      elementList: [{ element: "E1", derived: null, displaylevel: null, displaydelim: null }],
    });
    assert.equal(typeof cfg, "string");
  });

  test("required keys and non-nullable optional keys still reject null", () => {
    invalid(
      () => typed["addRule"]!(fixture, { id: 0, ruleConfig: { ERRULE_CODE: null, QUAL_ERFRAG_CODE: "SAME_NAME" } }),
      "addRule: ruleConfig.ERRULE_CODE must be a string, got null",
    );
    // The library rejects a null expression-call `feature` (not a string), so the type does too.
    invalid(
      () =>
        typed["addExpressionCall"]!(fixture, {
          efuncCode: "PARSE_NAME",
          elementList: [{ element: "PHONE_NUM", required: "Yes", feature: null }],
          isVirtual: "No",
        }),
      "addExpressionCall: elementList[0].feature must be a string, got null",
    );
  });

  test("a quoted user value equal to a wire name is never rewritten", () => {
    const err = caught(() => sz.addFeature(fixture, { feature: "FQ", elementList: ["E"], matchkey: "rtype_id" }));
    assert.equal(err.code, "INVALID_INPUT");
    assert.match(err.message, /'rtype_id'/);
    assert.doesNotMatch(err.message, /rtypeId/);
    const spec: rt.FnSpec = { name: "f", wire: "f", args: [["genericPlan", "generic_plan", true]] };
    for (const message of [
      "Invalid value 'generic_plan' for plan",
      'Invalid value "generic_plan" for plan',
      "argument 'generic_plan' must be a string",
      "Missing required field: x 'generic_plan'",
    ]) {
      const e = new sz.SzConfigToolError("INVALID_INPUT", message);
      assert.equal(rt.translateError(spec, e), e, message);
    }
    const m = rt.translateError(spec, new sz.SzConfigToolError("MISSING_FIELD", "Missing required field: generic_plan[0].x, other"));
    assert.equal(m.message, "Missing required field: genericPlan[0].x (generic_plan[0].x), other");
  });

  test("undefined-valued keys are absent, not unknown (top level and nested)", () => {
    assert.equal(typeof typed["addDataSource"]!(fixture, { code: "E2", bogus: undefined }), "string");
    const extra = { more: undefined };
    assert.equal(
      typeof typed["addSearchProfile"]!(fixture, {
        code: "P_UNDEF",
        genericPlan: "SEARCH",
        ...extra,
        elements: [{ feature: "NAME", flag: "Y", note: undefined }],
      }),
      "string",
    );
    invalid(() => typed["addDataSource"]!(fixture, { code: "E2", bogus: 1 }), /^unknown option 'bogus'/);
    invalid(
      () => typed["addSearchProfile"]!(fixture, { code: "P", genericPlan: "SEARCH", elements: [{ feature: "NAME", flag: "Y", note: 1 }] }),
      /^unknown key 'note'/,
    );
  });
});
