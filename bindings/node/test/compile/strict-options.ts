/**
 * Compile-time contract of the typed options (issue #76): never executed,
 * only type-checked by test/typecheck.test.ts. Every `@ts-expect-error` line
 * MUST fail to compile (tsc reports an unused directive otherwise).
 */
import * as sz from "../../dist/index.js";

export function shapes(cfg: string): string[] {
  return [
    // A correct structured option compiles.
    sz.addSearchProfile(cfg, {
      code: "P",
      genericPlan: "SEARCH",
      elements: [{ feature: "NAME", flag: "Yes" }],
    }),
    sz.addFeature(cfg, { feature: "F", elementList: ["E", { element: "E2", displaylevel: 1 }] }),
    sz.addExpressionCall(cfg, {
      efuncCode: "PARSE_NAME",
      elementList: [{ element: "E", required: "Yes", feature: "PARENT" }],
      isVirtual: "No",
    }),
    sz.addFragment(cfg, { fragmentConfig: { ERFRAG_CODE: "F", ERFRAG_SOURCE: "", ERFRAG_ID: 7n } }),
    sz.addRule(cfg, { id: 0, ruleConfig: { ERRULE_CODE: "R", QUAL_ERFRAG_CODE: "SAME_NAME" } }),
    sz.setSetting(cfg, { name: "S", value: { free: ["form", 1, null] } }),
    // @ts-expect-error -- flag is 'Yes' | 'No' | 'Y' | 'N'
    sz.addSearchProfile(cfg, { code: "P", genericPlan: "SEARCH", elements: [{ feature: "NAME", flag: "Maybe" }] }),
    // @ts-expect-error -- unknown key inside an element entry
    sz.addSearchProfile(cfg, { code: "P", genericPlan: "SEARCH", elements: [{ feature: "NAME", flag: "Y", x: 1 }] }),
    // @ts-expect-error -- elements is an array, not JSON text
    sz.addSearchProfile(cfg, { code: "P", genericPlan: "SEARCH", elements: '[{"feature":"NAME","flag":"Y"}]' }),
    // @ts-expect-error -- unknown option key
    sz.addSearchProfile(cfg, { code: "P", genericPlan: "SEARCH", candidate: "Off" }),
    // @ts-expect-error -- elementList is an array of codes / element objects
    sz.addFeature(cfg, { feature: "F", elementList: '["E"]' }),
    // @ts-expect-error -- an element entry is a string or an element object
    sz.addFeature(cfg, { feature: "F", elementList: [1] }),
    // @ts-expect-error -- displaylevel is an integer
    sz.addFeature(cfg, { feature: "F", elementList: [{ element: "E", displaylevel: "1" }] }),
    // @ts-expect-error -- `required` is mandatory in an expression-call element
    sz.addExpressionCall(cfg, { efuncCode: "PARSE_NAME", elementList: [{ element: "E" }], isVirtual: "No" }),
    // @ts-expect-error -- ERFRAG_SOURCE is required
    sz.addFragment(cfg, { fragmentConfig: { ERFRAG_CODE: "F" } }),
    // @ts-expect-error -- unknown rule key
    sz.addRule(cfg, { id: 0, ruleConfig: { ERRULE_CODE: "R", QUAL_ERFRAG_CODE: "Q", TIER: 1 } }),
    // @ts-expect-error -- options is an object, not a string
    sz.getSearchProfile(cfg, "SEARCH"),
    // @ts-expect-error -- options is an object, not a string
    sz.listSearchProfiles(cfg, "NOSUCH"),
  ];
}
