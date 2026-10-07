// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.
//
// Typed wrappers over the native `invoke` seam; see bindings/CONTRACT.md.

import * as rt from "../runtime.js";

/**
 * Arguments of {@link addAttribute}.
 */
export interface AddAttributeOptions {
  /**
   * Uppercased before storage and duplicate check.
   *
   * Wire name: `attribute`.
   */
  readonly attribute: string;
  /**
   * Must name an existing CFG_FTYPE (case-insensitive) or NOT_FOUND; stored uppercased in FTYPE_CODE.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * Must name an existing CFG_FELEM (case-insensitive) or NOT_FOUND; stored uppercased in FELEM_CODE.
   *
   * Wire name: `element`.
   */
  readonly element: string;
  /**
   * CASE-SENSITIVE (not uppercased): must be exactly one of NAME, ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
   *
   * Wire name: `class`.
   */
  readonly class: string;
  /**
   * Absent stores DEFAULT_VALUE null; any string (including "") is stored verbatim.
   *
   * Wire name: `default_value`.
   */
  readonly defaultValue?: string;
  /**
   * Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `internal`.
   */
  readonly internal?: string;
  /**
   * Case-insensitive; normalized to Yes, No, Any or Desired (stored in FELEM_REQ), else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `required`.
   */
  readonly required?: string;
  /**
   * Requested ATTR_ID. Absent OR <= 0 means auto-allocate (max existing + 1, floor 1000). A taken id > 0 is ALREADY_EXISTS.
   *
   * Wire name: `id`.
   */
  readonly id?: number | bigint;
}

const addAttributeSpec: rt.FnSpec = {
  name: "addAttribute",
  wire: "add_attribute",
  args: [
    ["attribute", "attribute", true],
    ["feature", "feature", true],
    ["element", "element", true],
    ["class", "class", true],
    ["defaultValue", "default_value", false],
    ["internal", "internal", false],
    ["required", "required", false],
    ["id", "id", false],
  ],
};

/**
 * Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
 *
 * @remarks
 * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
 *
 * Wire name: `add_attribute` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddAttributeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addAttributeResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addAttribute(config: string, options: AddAttributeOptions): string {
  return rt.callConfig(addAttributeSpec, config, options);
}

const addAttributeResultSpec: rt.FnSpec = { ...addAttributeSpec, name: "addAttributeResult" };

/**
 * The record (row / ids) of {@link addAttribute}: same options and operation, but returns the record instead of the configuration. Operation: Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
 *
 * @remarks
 * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
 *
 * Wire name: `add_attribute` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddAttributeOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addAttributeResult(config: string, options: AddAttributeOptions): string {
  return rt.callJson(addAttributeResultSpec, config, options);
}

/**
 * Arguments of {@link deleteAttribute}.
 */
export interface DeleteAttributeOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteAttributeSpec: rt.FnSpec = {
  name: "deleteAttribute",
  wire: "delete_attribute",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete an attribute by code.
 *
 * @remarks
 * No dependency or system-attribute protection; any attribute can be deleted.
 *
 * Wire name: `delete_attribute` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteAttributeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteAttribute(config: string, options: DeleteAttributeOptions): string {
  return rt.callConfig(deleteAttributeSpec, config, options);
}

/**
 * Arguments of {@link getAttribute}.
 */
export interface GetAttributeOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getAttributeSpec: rt.FnSpec = {
  name: "getAttribute",
  wire: "get_attribute",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one attribute's raw CFG_ATTR row by code.
 *
 * @remarks
 * Result uses on-disk keys (ATTR_ID, ATTR_CODE, ATTR_CLASS, FTYPE_CODE, FELEM_CODE, FELEM_REQ, DEFAULT_VALUE, INTERNAL).
 *
 * Wire name: `get_attribute` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetAttributeOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getAttribute(config: string, options: GetAttributeOptions): string {
  return rt.callJson(getAttributeSpec, config, options);
}

const listAttributesSpec: rt.FnSpec = {
  name: "listAttributes",
  wire: "list_attributes",
  args: [],
};

/**
 * List all attributes as camelCase summaries.
 *
 * @remarks
 * Result is an array of {id, attribute, class, feature, element, required, default, internal} in config order; feature/element/default may be null.
 *
 * Wire name: `list_attributes` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listAttributes(config: string): string {
  return rt.callJson(listAttributesSpec, config);
}

/**
 * Arguments of {@link setAttribute}.
 */
export interface SetAttributeOptions {
  /**
   * Attribute code; uppercased before lookup.
   *
   * Wire name: `attribute`.
   */
  readonly attribute: string;
  /**
   * Absent leaves INTERNAL unchanged; else case-insensitive, normalized to Yes or No, else INVALID_INPUT.
   *
   * Wire name: `internal`.
   */
  readonly internal?: string;
  /**
   * Absent leaves FELEM_REQ unchanged; else normalized to Yes, No, Any or Desired, else INVALID_INPUT.
   *
   * Wire name: `required`.
   */
  readonly required?: string;
  /**
   * Absent leaves DEFAULT_VALUE unchanged; a string is stored verbatim. NOT tri-state: there is no way to clear DEFAULT_VALUE back to null.
   *
   * Wire name: `default_value`.
   */
  readonly defaultValue?: string;
}

const setAttributeSpec: rt.FnSpec = {
  name: "setAttribute",
  wire: "set_attribute",
  args: [
    ["attribute", "attribute", true],
    ["internal", "internal", false],
    ["required", "required", false],
    ["defaultValue", "default_value", false],
  ],
};

/**
 * Update an attribute's internal / required / default value.
 *
 * @remarks
 * Validation is interleaved with mutation but the input config is never modified on error (a new config string is returned only on success).
 *
 * Wire name: `set_attribute` (group `attributes`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetAttributeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setAttribute(config: string, options: SetAttributeOptions): string {
  return rt.callConfig(setAttributeSpec, config, options);
}

/**
 * Arguments of {@link addBehaviorOverride}.
 */
export interface AddBehaviorOverrideOptions {
  /**
   * Feature code, matched case-insensitively against CFG_FTYPE (NOT_FOUND); stored as its FTYPE_ID.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * Uppercased; stored as UTYPE_CODE. Any string is accepted (no domain check).
   *
   * Wire name: `usage_type`.
   */
  readonly usageType: string;
  /**
   * Behavior code, case-insensitive: a frequency A1, F1, FF, FM, FVM (with optional E = exclusive and/or S = stable letters in any order) or the bare NAME / NONE; anything else is INVALID_INPUT. Split into FTYPE_FREQ, FTYPE_EXCL (Yes/No), FTYPE_STAB (Yes/No).
   *
   * Wire name: `behavior`.
   */
  readonly behavior: string;
}

const addBehaviorOverrideSpec: rt.FnSpec = {
  name: "addBehaviorOverride",
  wire: "add_behavior_override",
  args: [
    ["feature", "feature", true],
    ["usageType", "usage_type", true],
    ["behavior", "behavior", true],
  ],
};

/**
 * Add a behavior override (CFG_FBOVR row) for a feature and usage type.
 *
 * @remarks
 * Validation order: feature, behavior, CFG_FBOVR present (MISSING_SECTION), duplicate (same FTYPE_ID + uppercased UTYPE_CODE, ALREADY_EXISTS). The row always carries FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB.
 *
 * Wire name: `add_behavior_override` (group `behavior_overrides`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddBehaviorOverrideOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT, MISSING_SECTION, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addBehaviorOverride(config: string, options: AddBehaviorOverrideOptions): string {
  return rt.callConfig(addBehaviorOverrideSpec, config, options);
}

/**
 * Arguments of {@link deleteBehaviorOverride}.
 */
export interface DeleteBehaviorOverrideOptions {
  /**
   * Feature code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * Uppercased, then matched exactly against UTYPE_CODE (a stored lowercase UTYPE_CODE cannot be matched).
   *
   * Wire name: `usage_type`.
   */
  readonly usageType: string;
}

const deleteBehaviorOverrideSpec: rt.FnSpec = {
  name: "deleteBehaviorOverride",
  wire: "delete_behavior_override",
  args: [
    ["feature", "feature", true],
    ["usageType", "usage_type", true],
  ],
};

/**
 * Delete the behavior override for a feature and usage type.
 *
 * @remarks
 * No override for the pair is NOT_FOUND; a config without CFG_FBOVR is MISSING_SECTION.
 *
 * Wire name: `delete_behavior_override` (group `behavior_overrides`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteBehaviorOverrideOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteBehaviorOverride(config: string, options: DeleteBehaviorOverrideOptions): string {
  return rt.callConfig(deleteBehaviorOverrideSpec, config, options);
}

/**
 * Arguments of {@link getBehaviorOverride}.
 */
export interface GetBehaviorOverrideOptions {
  /**
   * Feature code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * Uppercased, then matched exactly against UTYPE_CODE.
   *
   * Wire name: `usage_type`.
   */
  readonly usageType: string;
}

const getBehaviorOverrideSpec: rt.FnSpec = {
  name: "getBehaviorOverride",
  wire: "get_behavior_override",
  args: [
    ["feature", "feature", true],
    ["usageType", "usage_type", true],
  ],
};

/**
 * Get the raw CFG_FBOVR row for a feature and usage type.
 *
 * @remarks
 * Result is the stored row with on-disk keys (FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB).
 *
 * Wire name: `get_behavior_override` (group `behavior_overrides`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetBehaviorOverrideOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getBehaviorOverride(config: string, options: GetBehaviorOverrideOptions): string {
  return rt.callJson(getBehaviorOverrideSpec, config, options);
}

const listBehaviorOverridesSpec: rt.FnSpec = {
  name: "listBehaviorOverrides",
  wire: "list_behavior_overrides",
  args: [],
};

/**
 * List the raw CFG_FBOVR rows sorted by FTYPE_ID.
 *
 * @remarks
 * Result is an array of stored rows (on-disk keys), stable-sorted by FTYPE_ID only (rows sharing a FTYPE_ID keep config order). A config without CFG_FBOVR is MISSING_SECTION (not []).
 *
 * Wire name: `list_behavior_overrides` (group `behavior_overrides`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listBehaviorOverrides(config: string): string {
  return rt.callJson(listBehaviorOverridesSpec, config);
}

const listBehaviorOverridesResolvedSpec: rt.FnSpec = {
  name: "listBehaviorOverridesResolved",
  wire: "list_behavior_overrides_resolved",
  args: [],
};

/**
 * List behavior overrides as {feature, usageType, behavior} display records.
 *
 * @remarks
 * Result is an array of {feature, usageType, behavior} sorted by (FTYPE_ID, UTYPE_CODE). feature is the FTYPE_CODE resolved from FTYPE_ID, or the id as a string when no CFG_FTYPE row matches; behavior is FTYPE_FREQ plus E when FTYPE_EXCL and S when FTYPE_STAB is Y/YES/1 (case-insensitive). A missing G2_CONFIG or CFG_FBOVR is MISSING_SECTION.
 *
 * Wire name: `list_behavior_overrides_resolved` (group `behavior_overrides`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listBehaviorOverridesResolved(config: string): string {
  return rt.callJson(listBehaviorOverridesResolvedSpec, config);
}

/**
 * Arguments of {@link addComparisonCall}.
 */
export interface AddComparisonCallOptions {
  /**
   * Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode: string;
  /**
   * Comparison function code; case-insensitive lookup in CFG_CFUNC, else NOT_FOUND.
   *
   * Wire name: `cfunc_code`.
   */
  readonly cfuncCode: string;
  /**
   * Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position (outside the exec-order allocation policy). Duplicate items are not rejected.
   *
   * Wire name: `element_list`.
   */
  readonly elementList: readonly string[];
  /**
   * Requested CFCALL_ID. Absent OR <= 0 means auto-allocate (max existing + 1, floor 1000). A taken id > 0 is ALREADY_EXISTS (checked before any lookup).
   *
   * Wire name: `id`.
   */
  readonly id?: number | bigint;
}

const addComparisonCallSpec: rt.FnSpec = {
  name: "addComparisonCall",
  wire: "add_comparison_call",
  args: [
    ["ftypeCode", "ftype_code", true],
    ["cfuncCode", "cfunc_code", true],
    ["elementList", "element_list", true],
    ["id", "id", false],
  ],
};

/**
 * Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with its element list (CFG_CFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array; ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function, empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to the feature is not checked.
 *
 * Wire name: `add_comparison_call` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addComparisonCallResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonCall(config: string, options: AddComparisonCallOptions): string {
  return rt.callConfig(addComparisonCallSpec, config, options);
}

const addComparisonCallResultSpec: rt.FnSpec = { ...addComparisonCallSpec, name: "addComparisonCallResult" };

/**
 * The record (row / ids) of {@link addComparisonCall}: same options and operation, but returns the record instead of the configuration. Operation: Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with its element list (CFG_CFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array; ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function, empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to the feature is not checked.
 *
 * Wire name: `add_comparison_call` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonCallOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonCallResult(config: string, options: AddComparisonCallOptions): string {
  return rt.callJson(addComparisonCallResultSpec, config, options);
}

/**
 * Arguments of {@link deleteComparisonCall}.
 */
export interface DeleteComparisonCallOptions {
  /**
   * CFCALL_ID; must exist in CFG_CFCALL else NOT_FOUND (an absent section is also NOT_FOUND).
   *
   * Wire name: `cfcall_id`.
   */
  readonly cfcallId: number | bigint;
}

const deleteComparisonCallSpec: rt.FnSpec = {
  name: "deleteComparisonCall",
  wire: "delete_comparison_call",
  args: [
    ["cfcallId", "cfcall_id", true],
  ],
};

/**
 * Delete a comparison call by CFCALL_ID, cascading to its CFG_CFBOM rows.
 *
 * @remarks
 * Removes the CFG_CFCALL row and every CFG_CFBOM row with that CFCALL_ID. No dependency or system-call protection: template calls (ids < 1000) can be deleted.
 *
 * Wire name: `delete_comparison_call` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonCall(config: string, options: DeleteComparisonCallOptions): string {
  return rt.callConfig(deleteComparisonCallSpec, config, options);
}

/**
 * Arguments of {@link getComparisonCall}.
 */
export interface GetComparisonCallOptions {
  /**
   * A JSON integer selects by CFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_CFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
}

const getComparisonCallSpec: rt.FnSpec = {
  name: "getComparisonCall",
  wire: "get_comparison_call",
  args: [
    ["call", "call", true],
  ],
};

/**
 * Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
 *
 * @remarks
 * Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it does not include the CFBOM elements (codes: list_comparison_calls; raw rows: get_config_section("CFG_CFBOM")).
 *
 * Wire name: `get_comparison_call` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetComparisonCallOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getComparisonCall(config: string, options: GetComparisonCallOptions): string {
  return rt.callJson(getComparisonCallSpec, config, options);
}

const listComparisonCallsSpec: rt.FnSpec = {
  name: "listComparisonCalls",
  wire: "list_comparison_calls",
  args: [],
};

/**
 * List all comparison calls with feature/function codes resolved and their ordered element lists.
 *
 * @remarks
 * Result is an array of {id, feature, function, elementList} sorted by (FTYPE_ID, CFCALL_ID) — not config order. elementList is the call's CFG_CFBOM element codes ordered by EXEC_ORDER. Unresolvable ids render as the string "unknown". Missing sections are treated as empty (never MISSING_SECTION). LIMITATION: elementList omits the stored CFG_CFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows with get_config_section("CFG_CFBOM").
 *
 * Wire name: `list_comparison_calls` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listComparisonCalls(config: string): string {
  return rt.callJson(listComparisonCallsSpec, config);
}

/**
 * Arguments of {@link addComparisonCallElement}.
 */
export interface AddComparisonCallElementOptions {
  /**
   * CFCALL_ID written verbatim. NOT validated — the call need not exist.
   *
   * Wire name: `cfcall_id`.
   */
  readonly cfcallId: number | bigint;
  /**
   * The ELEMENT's feature id written to the BOM row's FTYPE_ID. Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
   *
   * Wire name: `ftype_id`.
   */
  readonly ftypeId: number | bigint;
  /**
   * FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
   *
   * Wire name: `felem_id`.
   */
  readonly felemId: number | bigint;
  /**
   * Allocated per CFCALL_ID. Absent OR <= 0 means auto-allocate (max EXEC_ORDER on this call + 1, seed 0 so an empty call starts at 1). A taken order > 0 on the same call is ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
}

const addComparisonCallElementSpec: rt.FnSpec = {
  name: "addComparisonCallElement",
  wire: "add_comparison_call_element",
  args: [
    ["cfcallId", "cfcall_id", true],
    ["ftypeId", "ftype_id", true],
    ["felemId", "felem_id", true],
    ["execOrder", "exec_order", false],
  ],
};

/**
 * Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: ftype_id < 0, duplicate, exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added under a different ftype_id, which makes a later feature-less delete_comparison_call_element ambiguous.
 *
 * Wire name: `add_comparison_call_element` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addComparisonCallElementResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonCallElement(config: string, options: AddComparisonCallElementOptions): string {
  return rt.callConfig(addComparisonCallElementSpec, config, options);
}

const addComparisonCallElementResultSpec: rt.FnSpec = { ...addComparisonCallElementSpec, name: "addComparisonCallElementResult" };

/**
 * The record (row / ids) of {@link addComparisonCallElement}: same options and operation, but returns the record instead of the configuration. Operation: Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: ftype_id < 0, duplicate, exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added under a different ftype_id, which makes a later feature-less delete_comparison_call_element ambiguous.
 *
 * Wire name: `add_comparison_call_element` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonCallElementOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonCallElementResult(config: string, options: AddComparisonCallElementOptions): string {
  return rt.callJson(addComparisonCallElementResultSpec, config, options);
}

/**
 * Arguments of {@link deleteComparisonCallElement}.
 */
export interface DeleteComparisonCallElementOptions {
  /**
   * A JSON integer selects by CFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
  /**
   * Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * The element's feature code (case-insensitive; unknown is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
   *
   * Wire name: `element_feature`.
   */
  readonly elementFeature?: string;
}

const deleteComparisonCallElementSpec: rt.FnSpec = {
  name: "deleteComparisonCallElement",
  wire: "delete_comparison_call_element",
  args: [
    ["call", "call", true],
    ["elementCode", "element_code", true],
    ["elementFeature", "element_feature", false],
  ],
};

/**
 * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or feature code plus element code.
 *
 * @remarks
 * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no renumbering).
 *
 * Wire name: `delete_comparison_call_element` (group `calls_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonCallElement(config: string, options: DeleteComparisonCallElementOptions): string {
  return rt.callConfig(deleteComparisonCallElementSpec, config, options);
}

/**
 * Arguments of {@link addDistinctCall}.
 */
export interface AddDistinctCallOptions {
  /**
   * Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one distinct call per feature: if any CFG_DFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode: string;
  /**
   * Distinct function code; case-insensitive lookup in CFG_DFUNC, else NOT_FOUND.
   *
   * Wire name: `dfunc_code`.
   */
  readonly dfuncCode: string;
  /**
   * Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT (checked before anything else). One CFG_DFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position. Duplicate items are not rejected.
   *
   * Wire name: `element_list`.
   */
  readonly elementList: readonly string[];
}

const addDistinctCallSpec: rt.FnSpec = {
  name: "addDistinctCall",
  wire: "add_distinct_call",
  args: [
    ["ftypeCode", "ftype_code", true],
    ["dfuncCode", "dfunc_code", true],
    ["elementList", "element_list", true],
  ],
};

/**
 * Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its element list (CFG_DFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike add_comparison_call there is no `id` parameter. Validation order: empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
 *
 * Wire name: `add_distinct_call` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addDistinctCallResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctCall(config: string, options: AddDistinctCallOptions): string {
  return rt.callConfig(addDistinctCallSpec, config, options);
}

const addDistinctCallResultSpec: rt.FnSpec = { ...addDistinctCallSpec, name: "addDistinctCallResult" };

/**
 * The record (row / ids) of {@link addDistinctCall}: same options and operation, but returns the record instead of the configuration. Operation: Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its element list (CFG_DFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike add_comparison_call there is no `id` parameter. Validation order: empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
 *
 * Wire name: `add_distinct_call` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctCallOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctCallResult(config: string, options: AddDistinctCallOptions): string {
  return rt.callJson(addDistinctCallResultSpec, config, options);
}

/**
 * Arguments of {@link deleteDistinctCall}.
 */
export interface DeleteDistinctCallOptions {
  /**
   * DFCALL_ID; must exist in CFG_DFCALL else NOT_FOUND (an absent section is also NOT_FOUND).
   *
   * Wire name: `dfcall_id`.
   */
  readonly dfcallId: number | bigint;
}

const deleteDistinctCallSpec: rt.FnSpec = {
  name: "deleteDistinctCall",
  wire: "delete_distinct_call",
  args: [
    ["dfcallId", "dfcall_id", true],
  ],
};

/**
 * Delete a distinct call by DFCALL_ID, cascading to its CFG_DFBOM rows.
 *
 * @remarks
 * Removes the CFG_DFCALL row and every CFG_DFBOM row with that DFCALL_ID. No dependency or system-call protection.
 *
 * Wire name: `delete_distinct_call` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteDistinctCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteDistinctCall(config: string, options: DeleteDistinctCallOptions): string {
  return rt.callConfig(deleteDistinctCallSpec, config, options);
}

/**
 * Arguments of {@link getDistinctCall}.
 */
export interface GetDistinctCallOptions {
  /**
   * A JSON integer selects by DFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_DFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no distinct call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
}

const getDistinctCallSpec: rt.FnSpec = {
  name: "getDistinctCall",
  wire: "get_distinct_call",
  args: [
    ["call", "call", true],
  ],
};

/**
 * Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
 *
 * @remarks
 * Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it does not include the DFBOM elements (codes: list_distinct_calls; raw rows: get_config_section("CFG_DFBOM")).
 *
 * Wire name: `get_distinct_call` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetDistinctCallOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getDistinctCall(config: string, options: GetDistinctCallOptions): string {
  return rt.callJson(getDistinctCallSpec, config, options);
}

const listDistinctCallsSpec: rt.FnSpec = {
  name: "listDistinctCalls",
  wire: "list_distinct_calls",
  args: [],
};

/**
 * List all distinct calls with feature/function codes resolved and their ordered element lists.
 *
 * @remarks
 * Result is an array of {id, feature, function, execOrder, elementList} sorted by (FTYPE_ID, DFCALL_ID) — not config order. execOrder is the CFG_DFCALL row's EXEC_ORDER, which the v4 schema (and every template / add_distinct_call row) lacks, so it is 1. elementList is the call's CFG_DFBOM element codes ordered by EXEC_ORDER. Unresolvable ids render as "unknown". Missing sections are treated as empty. LIMITATION: elementList omits the stored CFG_DFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows with get_config_section("CFG_DFBOM").
 *
 * Wire name: `list_distinct_calls` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listDistinctCalls(config: string): string {
  return rt.callJson(listDistinctCallsSpec, config);
}

/**
 * Arguments of {@link addDistinctCallElement}.
 */
export interface AddDistinctCallElementOptions {
  /**
   * DFCALL_ID written verbatim. NOT validated — the call need not exist.
   *
   * Wire name: `dfcall_id`.
   */
  readonly dfcallId: number | bigint;
  /**
   * The ELEMENT's feature id written to the BOM row's FTYPE_ID. NOT validated at all — unlike add_comparison_call_element, a negative id is accepted and stored.
   *
   * Wire name: `ftype_id`.
   */
  readonly ftypeId: number | bigint;
  /**
   * FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
   *
   * Wire name: `felem_id`.
   */
  readonly felemId: number | bigint;
  /**
   * Allocated per DFCALL_ID. Absent OR <= 0 means auto-allocate (max EXEC_ORDER on this call + 1, seed 0). A taken order > 0 on the same call is ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
}

const addDistinctCallElementSpec: rt.FnSpec = {
  name: "addDistinctCallElement",
  wire: "add_distinct_call_element",
  args: [
    ["dfcallId", "dfcall_id", true],
    ["ftypeId", "ftype_id", true],
    ["felemId", "felem_id", true],
    ["execOrder", "exec_order", false],
  ],
};

/**
 * Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then MISSING_SECTION if CFG_DFBOM is absent.
 *
 * Wire name: `add_distinct_call_element` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addDistinctCallElementResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctCallElement(config: string, options: AddDistinctCallElementOptions): string {
  return rt.callConfig(addDistinctCallElementSpec, config, options);
}

const addDistinctCallElementResultSpec: rt.FnSpec = { ...addDistinctCallElementSpec, name: "addDistinctCallElementResult" };

/**
 * The record (row / ids) of {@link addDistinctCallElement}: same options and operation, but returns the record instead of the configuration. Operation: Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then MISSING_SECTION if CFG_DFBOM is absent.
 *
 * Wire name: `add_distinct_call_element` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctCallElementOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctCallElementResult(config: string, options: AddDistinctCallElementOptions): string {
  return rt.callJson(addDistinctCallElementResultSpec, config, options);
}

/**
 * Arguments of {@link deleteDistinctCallElement}.
 */
export interface DeleteDistinctCallElementOptions {
  /**
   * A JSON integer selects by DFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
  /**
   * Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * The element's feature code (case-insensitive; unknown is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
   *
   * Wire name: `element_feature`.
   */
  readonly elementFeature?: string;
}

const deleteDistinctCallElementSpec: rt.FnSpec = {
  name: "deleteDistinctCallElement",
  wire: "delete_distinct_call_element",
  args: [
    ["call", "call", true],
    ["elementCode", "element_code", true],
    ["elementFeature", "element_feature", false],
  ],
};

/**
 * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature code plus element code.
 *
 * @remarks
 * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; no renumbering.
 *
 * Wire name: `delete_distinct_call_element` (group `calls_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteDistinctCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteDistinctCallElement(config: string, options: DeleteDistinctCallElementOptions): string {
  return rt.callConfig(deleteDistinctCallElementSpec, config, options);
}

/**
 * Arguments of {@link addExpressionCall}.
 */
export interface AddExpressionCallOptions {
  /**
   * Expression function code (CFG_EFUNC, case-insensitive) or NOT_FOUND.
   *
   * Wire name: `efunc_code`.
   */
  readonly efuncCode: string;
  /**
   * JSON array of {"element": str, "required": str, "feature"?: str} objects (unknown keys, non-objects, non-string values = INVALID_INPUT; missing element/required = MISSING_FIELD). One CFG_EFBOM row per item, EXEC_ORDER = 1-based list position. element: global CFG_FELEM lookup (case-insensitive) or NOT_FOUND. required: stored verbatim in FELEM_REQ (not validated or normalized). feature: absent stores BOM FTYPE_ID -1 (G2 WILDCARDED_FTYPE: any feature in the record carrying the element); "PARENT" (case-insensitive) stores BOM FTYPE_ID 0 (G2 PARENT_FEATURE_LINKED_FTYPE: the feature that triggered the call); otherwise a feature code (case-insensitive) or NOT_FOUND. The element is NOT checked for membership in that feature. [] is allowed.
   *
   * Shape: `[{element: string, required: string, feature?: string}]`.
   *
   * Wire name: `element_list`.
   */
  readonly elementList: ReadonlyArray<{ readonly element: string; readonly required: string; readonly feature?: string }>;
  /**
   * Feature code (case-insensitive) or NOT_FOUND; "ALL" (case-insensitive) = absent; absent stores FTYPE_ID -1.
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode?: string;
  /**
   * Element code (case-insensitive) or NOT_FOUND; "N/A" (case-insensitive) = absent; absent stores FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else INVALID_INPUT.
   *
   * Wire name: `felem_code`.
   */
  readonly felemCode?: string;
  /**
   * CFG_EFCALL EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <= 0 = auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken = ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * Feature code stored as EFEAT_FTYPE_ID (case-insensitive) or NOT_FOUND; absent or "N/A" (case-insensitive) stores -1.
   *
   * Wire name: `expression_feature`.
   */
  readonly expressionFeature?: string;
  /**
   * Stored verbatim in IS_VIRTUAL (not validated or normalized; the Rust `new()` default is "No").
   *
   * Wire name: `is_virtual`.
   */
  readonly isVirtual: string;
}

const addExpressionCallSpec: rt.FnSpec = {
  name: "addExpressionCall",
  wire: "add_expression_call",
  args: [
    ["efuncCode", "efunc_code", true],
    ["elementList", "element_list", true, { array: { object: { "element": "string", "required": "string", "feature?": "string" } } }],
    ["ftypeCode", "ftype_code", false],
    ["felemCode", "felem_code", false],
    ["execOrder", "exec_order", false],
    ["expressionFeature", "expression_feature", false],
    ["isVirtual", "is_virtual", true],
  ],
};

/**
 * Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order, expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any feature. The BOM-feature column is not rendered by get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
 *
 * Wire name: `add_expression_call` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addExpressionCallResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionCall(config: string, options: AddExpressionCallOptions): string {
  return rt.callConfig(addExpressionCallSpec, config, options);
}

const addExpressionCallResultSpec: rt.FnSpec = { ...addExpressionCallSpec, name: "addExpressionCallResult" };

/**
 * The record (row / ids) of {@link addExpressionCall}: same options and operation, but returns the record instead of the configuration. Operation: Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
 *
 * @remarks
 * Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order, expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any feature. The BOM-feature column is not rendered by get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
 *
 * Wire name: `add_expression_call` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionCallOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionCallResult(config: string, options: AddExpressionCallOptions): string {
  return rt.callJson(addExpressionCallResultSpec, config, options);
}

/**
 * Arguments of {@link deleteExpressionCall}.
 */
export interface DeleteExpressionCallOptions {
  /**
   * Must match an existing EFCALL_ID, else NOT_FOUND.
   *
   * Wire name: `efcall_id`.
   */
  readonly efcallId: number | bigint;
}

const deleteExpressionCallSpec: rt.FnSpec = {
  name: "deleteExpressionCall",
  wire: "delete_expression_call",
  args: [
    ["efcallId", "efcall_id", true],
  ],
};

/**
 * Delete an expression call by EFCALL_ID, cascading its CFG_EFBOM rows.
 *
 * @remarks
 * Removes the CFG_EFCALL row(s) with that id and every CFG_EFBOM row with that EFCALL_ID.
 *
 * Wire name: `delete_expression_call` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionCall(config: string, options: DeleteExpressionCallOptions): string {
  return rt.callConfig(deleteExpressionCallSpec, config, options);
}

/**
 * Arguments of {@link getExpressionCall}.
 */
export interface GetExpressionCallOptions {
  /**
   * Call selector: an integer = EFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
}

const getExpressionCallSpec: rt.FnSpec = {
  name: "getExpressionCall",
  wire: "get_expression_call",
  args: [
    ["call", "call", true],
  ],
};

/**
 * Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
 *
 * @remarks
 * Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows: get_config_section("CFG_EFBOM")). Expression calls are many-per-feature (template NAME has 7), so by-feature is often ambiguous.
 *
 * Wire name: `get_expression_call` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetExpressionCallOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getExpressionCall(config: string, options: GetExpressionCallOptions): string {
  return rt.callJson(getExpressionCallSpec, config, options);
}

const listExpressionCallsSpec: rt.FnSpec = {
  name: "listExpressionCalls",
  wire: "list_expression_calls",
  args: [],
};

/**
 * List all expression calls with resolved codes and element lists.
 *
 * @remarks
 * Array of {id, feature, element, execOrder, function, isVirtual, expressionFeature, elementList}, stably sorted by (FTYPE_ID, FELEM_ID, EXEC_ORDER). feature "all" / element "n/a" / function "unknown" when the id is <= 0 or unresolved; expressionFeature "n/a" when EFEAT_FTYPE_ID <= 0. elementList is the BOM element codes ordered by BOM EXEC_ORDER. Missing sections yield []. LIMITATION: elementList omits the stored CFG_EFBOM columns FTYPE_ID (0 = parent feature, -1 = any feature), EXEC_ORDER and FELEM_REQ, which the engine uses; read the raw rows with get_config_section("CFG_EFBOM").
 *
 * Wire name: `list_expression_calls` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listExpressionCalls(config: string): string {
  return rt.callJson(listExpressionCallsSpec, config);
}

/**
 * Arguments of {@link addExpressionCallElement}.
 */
export interface AddExpressionCallElementOptions {
  /**
   * Stored as EFCALL_ID. NOT validated — the call need not exist.
   *
   * Wire name: `efcall_id`.
   */
  readonly efcallId: number | bigint;
  /**
   * The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID. < 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT validated against CFG_FTYPE.
   *
   * Wire name: `ftype_id`.
   */
  readonly ftypeId: number | bigint;
  /**
   * Stored verbatim as FELEM_ID. NOT validated against CFG_FELEM.
   *
   * Wire name: `felem_id`.
   */
  readonly felemId: number | bigint;
  /**
   * BOM EXEC_ORDER scoped per EFCALL_ID. Absent or <= 0 = auto-allocate (max on the call + 1); > 0 and free = verbatim; > 0 and taken = ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * Stored verbatim in FELEM_REQ (not validated or normalized).
   *
   * Wire name: `felem_req`.
   */
  readonly felemReq: string;
}

const addExpressionCallElementSpec: rt.FnSpec = {
  name: "addExpressionCallElement",
  wire: "add_expression_call_element",
  args: [
    ["efcallId", "efcall_id", true],
    ["ftypeId", "ftype_id", true],
    ["felemId", "felem_id", true],
    ["execOrder", "exec_order", false],
    ["felemReq", "felem_req", true],
  ],
};

/**
 * Add one CFG_EFBOM row to an expression call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER, FELEM_REQ}). Check order: ftype_id < 0, ALREADY_PRESENT when (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then MISSING_SECTION if CFG_EFBOM is absent.
 *
 * Wire name: `add_expression_call_element` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addExpressionCallElementResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionCallElement(config: string, options: AddExpressionCallElementOptions): string {
  return rt.callConfig(addExpressionCallElementSpec, config, options);
}

const addExpressionCallElementResultSpec: rt.FnSpec = { ...addExpressionCallElementSpec, name: "addExpressionCallElementResult" };

/**
 * The record (row / ids) of {@link addExpressionCallElement}: same options and operation, but returns the record instead of the configuration. Operation: Add one CFG_EFBOM row to an expression call, addressed by raw ids.
 *
 * @remarks
 * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER, FELEM_REQ}). Check order: ftype_id < 0, ALREADY_PRESENT when (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then MISSING_SECTION if CFG_EFBOM is absent.
 *
 * Wire name: `add_expression_call_element` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionCallElementOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionCallElementResult(config: string, options: AddExpressionCallElementOptions): string {
  return rt.callJson(addExpressionCallElementResultSpec, config, options);
}

/**
 * Arguments of {@link deleteExpressionCallElement}.
 */
export interface DeleteExpressionCallElementOptions {
  /**
   * Call selector: an integer = EFCALL_ID; a string = feature code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call = INVALID_INPUT). A call id that does not exist = NOT_FOUND.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
  /**
   * Element code (case-insensitive). Without element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call = NOT_ON_CALL.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * The element's feature code (case-insensitive; unknown = NOT_FOUND). Narrows the BOM match to that FTYPE_ID. Without it, an element present under more than one feature on the call is INVALID_INPUT (ambiguous) — e.g. template EFCALL 97 carries TOKENIZED_NM under GROUP_ASSOCIATION and EMPLOYER.
   *
   * Wire name: `element_feature`.
   */
  readonly elementFeature?: string;
}

const deleteExpressionCallElementSpec: rt.FnSpec = {
  name: "deleteExpressionCallElement",
  wire: "delete_expression_call_element",
  args: [
    ["call", "call", true],
    ["elementCode", "element_code", true],
    ["elementFeature", "element_feature", false],
  ],
};

/**
 * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
 *
 * @remarks
 * The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check order: call selector, call existence, element_feature, element, BOM row.
 *
 * Wire name: `delete_expression_call_element` (group `calls_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionCallElement(config: string, options: DeleteExpressionCallElementOptions): string {
  return rt.callConfig(deleteExpressionCallElementSpec, config, options);
}

/**
 * Arguments of {@link addStandardizeCall}.
 */
export interface AddStandardizeCallOptions {
  /**
   * Standardize function code (CFG_SFUNC, case-insensitive) or NOT_FOUND. Looked up before the feature/element.
   *
   * Wire name: `sfunc_code`.
   */
  readonly sfuncCode: string;
  /**
   * Feature code (case-insensitive) or NOT_FOUND. "ALL" (case-insensitive) is treated as absent. Stored as FTYPE_ID; absent stores FTYPE_ID -1.
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode?: string;
  /**
   * Element code (case-insensitive) or NOT_FOUND. "N/A" (case-insensitive) is treated as absent. Stored as FELEM_ID; absent stores FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else INVALID_INPUT.
   *
   * Wire name: `felem_code`.
   */
  readonly felemCode?: string;
  /**
   * EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID) of the new row (the -1 sentinel is part of the scope). Absent or <= 0 = auto-allocate (max in scope + 1, 1 for an empty scope); > 0 and free = used verbatim; > 0 and taken = ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
}

const addStandardizeCallSpec: rt.FnSpec = {
  name: "addStandardizeCall",
  wire: "add_standardize_call",
  args: [
    ["sfuncCode", "sfunc_code", true],
    ["ftypeCode", "ftype_code", false],
    ["felemCode", "felem_code", false],
    ["execOrder", "exec_order", false],
  ],
};

/**
 * Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or an element.
 *
 * @remarks
 * Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not include SFUNC_ID, so a second call on the same feature continues that feature's order sequence.
 *
 * Wire name: `add_standardize_call` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addStandardizeCallResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeCall(config: string, options: AddStandardizeCallOptions): string {
  return rt.callConfig(addStandardizeCallSpec, config, options);
}

const addStandardizeCallResultSpec: rt.FnSpec = { ...addStandardizeCallSpec, name: "addStandardizeCallResult" };

/**
 * The record (row / ids) of {@link addStandardizeCall}: same options and operation, but returns the record instead of the configuration. Operation: Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or an element.
 *
 * @remarks
 * Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not include SFUNC_ID, so a second call on the same feature continues that feature's order sequence.
 *
 * Wire name: `add_standardize_call` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeCallOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeCallResult(config: string, options: AddStandardizeCallOptions): string {
  return rt.callJson(addStandardizeCallResultSpec, config, options);
}

/**
 * Arguments of {@link deleteStandardizeCall}.
 */
export interface DeleteStandardizeCallOptions {
  /**
   * Must match an existing SFCALL_ID, else NOT_FOUND (also NOT_FOUND when CFG_SFCALL is absent).
   *
   * Wire name: `sfcall_id`.
   */
  readonly sfcallId: number | bigint;
}

const deleteStandardizeCallSpec: rt.FnSpec = {
  name: "deleteStandardizeCall",
  wire: "delete_standardize_call",
  args: [
    ["sfcallId", "sfcall_id", true],
  ],
};

/**
 * Delete a standardize call by SFCALL_ID.
 *
 * @remarks
 * Removes every CFG_SFCALL row with that id; no dependency checks (there is no standardize BOM).
 *
 * Wire name: `delete_standardize_call` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeCallOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeCall(config: string, options: DeleteStandardizeCallOptions): string {
  return rt.callConfig(deleteStandardizeCallSpec, config, options);
}

/**
 * Arguments of {@link getStandardizeCall}.
 */
export interface GetStandardizeCallOptions {
  /**
   * Call selector: an integer = SFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
   *
   * Wire name: `call`.
   */
  readonly call: number | bigint | string;
}

const getStandardizeCallSpec: rt.FnSpec = {
  name: "getStandardizeCall",
  wire: "get_standardize_call",
  args: [
    ["call", "call", true],
  ],
};

/**
 * Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
 *
 * @remarks
 * Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER). Element-bound calls (FTYPE_ID -1) are never found by feature. In the template NAME has two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME is ambiguous.
 *
 * Wire name: `get_standardize_call` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetStandardizeCallOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getStandardizeCall(config: string, options: GetStandardizeCallOptions): string {
  return rt.callJson(getStandardizeCallSpec, config, options);
}

const listStandardizeCallsSpec: rt.FnSpec = {
  name: "listStandardizeCalls",
  wire: "list_standardize_calls",
  args: [],
};

/**
 * List all standardize calls with resolved codes.
 *
 * @remarks
 * Array of {id, feature, element, execOrder, function}, stably sorted by (FTYPE_ID, EXEC_ORDER). feature is "all" when FTYPE_ID <= 0 or unresolved; element is "n/a" when FELEM_ID <= 0 or unresolved; function is "unknown" when unresolved. No elementList key. Missing sections yield [] (no MISSING_SECTION).
 *
 * Wire name: `list_standardize_calls` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listStandardizeCalls(config: string): string {
  return rt.callJson(listStandardizeCallsSpec, config);
}

/**
 * Arguments of {@link addStandardizeCallElement}.
 */
export interface AddStandardizeCallElementOptions {
  /**
   * Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE (use -1 for an element-bound row).
   *
   * Wire name: `ftype_id`.
   */
  readonly ftypeId: number | bigint;
  /**
   * Stored verbatim as SFUNC_ID. NOT validated against CFG_SFUNC.
   *
   * Wire name: `sfunc_id`.
   */
  readonly sfuncId: number | bigint;
  /**
   * Stored as FELEM_ID; absent = -1. NOT validated against CFG_FELEM.
   *
   * Wire name: `felem_id`.
   */
  readonly felemId?: number | bigint;
  /**
   * EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <= 0 = auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken = ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
}

const addStandardizeCallElementSpec: rt.FnSpec = {
  name: "addStandardizeCallElement",
  wire: "add_standardize_call_element",
  args: [
    ["ftypeId", "ftype_id", true],
    ["sfuncId", "sfunc_id", true],
    ["felemId", "felem_id", false],
    ["execOrder", "exec_order", false],
  ],
};

/**
 * Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
 *
 * @remarks
 * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike add_standardize_call there is no feature-xor-element rule and no id validation.
 *
 * Wire name: `add_standardize_call_element` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addStandardizeCallElementResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeCallElement(config: string, options: AddStandardizeCallElementOptions): string {
  return rt.callConfig(addStandardizeCallElementSpec, config, options);
}

const addStandardizeCallElementResultSpec: rt.FnSpec = { ...addStandardizeCallElementSpec, name: "addStandardizeCallElementResult" };

/**
 * The record (row / ids) of {@link addStandardizeCallElement}: same options and operation, but returns the record instead of the configuration. Operation: Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
 *
 * @remarks
 * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike add_standardize_call there is no feature-xor-element rule and no id validation.
 *
 * Wire name: `add_standardize_call_element` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeCallElementOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeCallElementResult(config: string, options: AddStandardizeCallElementOptions): string {
  return rt.callJson(addStandardizeCallElementResultSpec, config, options);
}

/**
 * Arguments of {@link deleteStandardizeCallElement}.
 */
export interface DeleteStandardizeCallElementOptions {
  /**
   * Matched against FTYPE_ID.
   *
   * Wire name: `ftype_id`.
   */
  readonly ftypeId: number | bigint;
  /**
   * Matched against SFUNC_ID.
   *
   * Wire name: `sfunc_id`.
   */
  readonly sfuncId: number | bigint;
  /**
   * Matched against FELEM_ID; absent = -1.
   *
   * Wire name: `felem_id`.
   */
  readonly felemId?: number | bigint;
}

const deleteStandardizeCallElementSpec: rt.FnSpec = {
  name: "deleteStandardizeCallElement",
  wire: "delete_standardize_call_element",
  args: [
    ["ftypeId", "ftype_id", true],
    ["sfuncId", "sfunc_id", true],
    ["felemId", "felem_id", false],
  ],
};

/**
 * Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
 *
 * @remarks
 * NOT_FOUND (not NOT_ON_CALL) when no row matches. Removes EVERY matching row.
 *
 * Wire name: `delete_standardize_call_element` (group `calls_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeCallElement(config: string, options: DeleteStandardizeCallElementOptions): string {
  return rt.callConfig(deleteStandardizeCallElementSpec, config, options);
}

/**
 * Arguments of {@link addConfigSection}.
 */
export interface AddConfigSectionOptions {
  /**
   * Uppercased before the duplicate check and storage. The new section is always an empty JSON array.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
}

const addConfigSectionSpec: rt.FnSpec = {
  name: "addConfigSection",
  wire: "add_config_section",
  args: [
    ["sectionName", "section_name", true],
  ],
};

/**
 * Add a new, empty top-level section (an empty array) to G2_CONFIG.
 *
 * @remarks
 * An existing key of the same (uppercased) name, of ANY JSON type, is ALREADY_EXISTS. A config without a G2_CONFIG key is NOT_FOUND (not MISSING_SECTION). No validation of the name (any string, e.g. not CFG_*, is accepted).
 *
 * Wire name: `add_config_section` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddConfigSectionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, ALREADY_EXISTS, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addConfigSection(config: string, options: AddConfigSectionOptions): string {
  return rt.callConfig(addConfigSectionSpec, config, options);
}

/**
 * Arguments of {@link removeConfigSection}.
 */
export interface RemoveConfigSectionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
}

const removeConfigSectionSpec: rt.FnSpec = {
  name: "removeConfigSection",
  wire: "remove_config_section",
  args: [
    ["sectionName", "section_name", true],
  ],
};

/**
 * Remove a top-level section from G2_CONFIG.
 *
 * @remarks
 * TRAP: no protection or dependency check; any section (including core sections such as CFG_DSRC, SETTINGS) can be removed. A missing section or a missing G2_CONFIG is NOT_FOUND.
 *
 * Wire name: `remove_config_section` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link RemoveConfigSectionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function removeConfigSection(config: string, options: RemoveConfigSectionOptions): string {
  return rt.callConfig(removeConfigSectionSpec, config, options);
}

/**
 * Arguments of {@link getConfigSection}.
 */
export interface GetConfigSectionOptions {
  /**
   * TRAP: matched EXACTLY (case-sensitive, NOT uppercased, unlike add/remove_config_section). Unknown name is NOT_FOUND.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
  /**
   * Absent returns every item. Otherwise keeps items whose json.dumps-spaced rendering (`{"K": 1, "J": null}`, crate::filter::to_json_dumps_string) contains the filter, case-insensitively.
   *
   * Wire name: `filter`.
   */
  readonly filter?: string;
}

const getConfigSectionSpec: rt.FnSpec = {
  name: "getConfigSection",
  wire: "get_config_section",
  args: [
    ["sectionName", "section_name", true],
    ["filter", "filter", false],
  ],
};

/**
 * Get the raw items of a top-level section, optionally filtered by a case-insensitive substring.
 *
 * @remarks
 * Result is an array. An array section returns its rows; a null or empty section returns []; a non-array, non-null section (e.g. SETTINGS, CONFIG_BASE_VERSION objects) returns a one-element array holding the value. An empty result does not distinguish "section empty" from "filter matched nothing": use config_section_is_empty.
 *
 * Wire name: `get_config_section` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetConfigSectionOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getConfigSection(config: string, options: GetConfigSectionOptions): string {
  return rt.callJson(getConfigSectionSpec, config, options);
}

/**
 * Arguments of {@link configSectionIsEmpty}.
 */
export interface ConfigSectionIsEmptyOptions {
  /**
   * Matched EXACTLY (case-sensitive, not uppercased). Unknown name is NOT_FOUND.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
}

const configSectionIsEmptySpec: rt.FnSpec = {
  name: "configSectionIsEmpty",
  wire: "config_section_is_empty",
  args: [
    ["sectionName", "section_name", true],
  ],
};

/**
 * Report whether a top-level section is empty (null or []).
 *
 * @remarks
 * Result is a JSON boolean: true for null or [], false otherwise (any non-array, non-null value such as an object counts as non-empty).
 *
 * Wire name: `config_section_is_empty` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link ConfigSectionIsEmptyOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function configSectionIsEmpty(config: string, options: ConfigSectionIsEmptyOptions): string {
  return rt.callJson(configSectionIsEmptySpec, config, options);
}

const listConfigSectionsSpec: rt.FnSpec = {
  name: "listConfigSections",
  wire: "list_config_sections",
  args: [],
};

/**
 * List the names of all top-level G2_CONFIG keys.
 *
 * @remarks
 * Result is an array of key names in config order, including non-CFG keys (SETTINGS, SYS_OOM, CONFIG_BASE_VERSION). A missing or non-object G2_CONFIG yields [] rather than an error.
 *
 * Wire name: `list_config_sections` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listConfigSections(config: string): string {
  return rt.callJson(listConfigSectionsSpec, config);
}

/**
 * Arguments of {@link removeConfigSectionField}.
 */
export interface RemoveConfigSectionFieldOptions {
  /**
   * Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
  /**
   * Uppercased before removal (a lowercase key in a row can never be removed).
   *
   * Wire name: `field_name`.
   */
  readonly fieldName: string;
}

const removeConfigSectionFieldSpec: rt.FnSpec = {
  name: "removeConfigSectionField",
  wire: "remove_config_section_field",
  args: [
    ["sectionName", "section_name", true],
    ["fieldName", "field_name", true],
  ],
};

/**
 * Remove a field from every item of an array section, returning how many items had it.
 *
 * @remarks
 * Record is the integer count of items the field was removed from (0 when no item had it; the config is still returned). Non-object items are skipped. A config with no G2_CONFIG key succeeds unchanged with count 0.
 *
 * Wire name: `remove_config_section_field` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link RemoveConfigSectionFieldOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link removeConfigSectionFieldResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function removeConfigSectionField(config: string, options: RemoveConfigSectionFieldOptions): string {
  return rt.callConfig(removeConfigSectionFieldSpec, config, options);
}

const removeConfigSectionFieldResultSpec: rt.FnSpec = { ...removeConfigSectionFieldSpec, name: "removeConfigSectionFieldResult" };

/**
 * The record (row / ids) of {@link removeConfigSectionField}: same options and operation, but returns the record instead of the configuration. Operation: Remove a field from every item of an array section, returning how many items had it.
 *
 * @remarks
 * Record is the integer count of items the field was removed from (0 when no item had it; the config is still returned). Non-object items are skipped. A config with no G2_CONFIG key succeeds unchanged with count 0.
 *
 * Wire name: `remove_config_section_field` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link RemoveConfigSectionFieldOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function removeConfigSectionFieldResult(config: string, options: RemoveConfigSectionFieldOptions): string {
  return rt.callJson(removeConfigSectionFieldResultSpec, config, options);
}

/**
 * Arguments of {@link addConfigSectionField}.
 */
export interface AddConfigSectionFieldOptions {
  /**
   * Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
   *
   * Wire name: `section_name`.
   */
  readonly sectionName: string;
  /**
   * Uppercased before insertion.
   *
   * Wire name: `field_name`.
   */
  readonly fieldName: string;
  /**
   * Any JSON value, stored verbatim (cloned) into each item that lacks the field.
   *
   * Wire name: `field_value`.
   */
  readonly fieldValue: rt.JsonValue;
}

const addConfigSectionFieldSpec: rt.FnSpec = {
  name: "addConfigSectionField",
  wire: "add_config_section_field",
  args: [
    ["sectionName", "section_name", true],
    ["fieldName", "field_name", true],
    ["fieldValue", "field_value", true],
  ],
};

/**
 * Add a field to every item of an array section that lacks it, returning existed/updated counts.
 *
 * @remarks
 * Record is {"existed": n, "updated": n}: items that already had the field (value preserved, never overwritten) vs. items it was inserted into. Non-object items are skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with both counts 0.
 *
 * Wire name: `add_config_section_field` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddConfigSectionFieldOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addConfigSectionFieldResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addConfigSectionField(config: string, options: AddConfigSectionFieldOptions): string {
  return rt.callConfig(addConfigSectionFieldSpec, config, options);
}

const addConfigSectionFieldResultSpec: rt.FnSpec = { ...addConfigSectionFieldSpec, name: "addConfigSectionFieldResult" };

/**
 * The record (row / ids) of {@link addConfigSectionField}: same options and operation, but returns the record instead of the configuration. Operation: Add a field to every item of an array section that lacks it, returning existed/updated counts.
 *
 * @remarks
 * Record is {"existed": n, "updated": n}: items that already had the field (value preserved, never overwritten) vs. items it was inserted into. Non-object items are skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with both counts 0.
 *
 * Wire name: `add_config_section_field` (group `config_sections`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddConfigSectionFieldOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addConfigSectionFieldResult(config: string, options: AddConfigSectionFieldOptions): string {
  return rt.callJson(addConfigSectionFieldResultSpec, config, options);
}

/**
 * Arguments of {@link addDataSource}.
 */
export interface AddDataSourceOptions {
  /**
   * Uppercased before storage and duplicate check; DSRC_DESC is set to the same uppercased code.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Case-insensitive; normalized to Remember or Forget. Any other value is INVALID_INPUT.
   *
   * Library default when omitted: `"Remember"` (applied by the library, not this binding).
   *
   * Wire name: `retention_level`.
   */
  readonly retentionLevel?: string;
  /**
   * Requested DSRC_ID. Absent OR <= 0 means auto-allocate (next free id, floor 1000). A taken id > 0 is ALREADY_EXISTS.
   *
   * Wire name: `id`.
   */
  readonly id?: number | bigint;
}

const addDataSourceSpec: rt.FnSpec = {
  name: "addDataSource",
  wire: "add_data_source",
  args: [
    ["code", "code", true],
    ["retentionLevel", "retention_level", false],
    ["id", "id", false],
  ],
};

/**
 * Add a data source (CFG_DSRC row) to the configuration.
 *
 * Wire name: `add_data_source` (group `datasources`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDataSourceOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDataSource(config: string, options: AddDataSourceOptions): string {
  return rt.callConfig(addDataSourceSpec, config, options);
}

/**
 * Arguments of {@link deleteDataSource}.
 */
export interface DeleteDataSourceOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteDataSourceSpec: rt.FnSpec = {
  name: "deleteDataSource",
  wire: "delete_data_source",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a data source by code.
 *
 * @remarks
 * System data sources (DSRC_ID <= 2, e.g. TEST and SEARCH in the template) are protected and fail with INVALID_INPUT. No dependency check is made against other sections.
 *
 * Wire name: `delete_data_source` (group `datasources`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteDataSourceOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteDataSource(config: string, options: DeleteDataSourceOptions): string {
  return rt.callConfig(deleteDataSourceSpec, config, options);
}

/**
 * Arguments of {@link getDataSource}.
 */
export interface GetDataSourceOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getDataSourceSpec: rt.FnSpec = {
  name: "getDataSource",
  wire: "get_data_source",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one data source's raw CFG_DSRC row by code.
 *
 * @remarks
 * Result is the stored row with on-disk keys (DSRC_ID, DSRC_CODE, DSRC_DESC, RETENTION_LEVEL).
 *
 * Wire name: `get_data_source` (group `datasources`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetDataSourceOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getDataSource(config: string, options: GetDataSourceOptions): string {
  return rt.callJson(getDataSourceSpec, config, options);
}

const listDataSourcesSpec: rt.FnSpec = {
  name: "listDataSources",
  wire: "list_data_sources",
  args: [],
};

/**
 * List all data sources as {id, dataSource} summaries.
 *
 * @remarks
 * Result is an array of {"id": DSRC_ID, "dataSource": DSRC_CODE} in config order.
 *
 * Wire name: `list_data_sources` (group `datasources`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listDataSources(config: string): string {
  return rt.callJson(listDataSourcesSpec, config);
}

/**
 * Arguments of {@link setDataSource}.
 */
export interface SetDataSourceOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves RETENTION_LEVEL unchanged. TRAP: unlike add_data_source the value is written VERBATIM, with no domain validation or case normalization.
   *
   * Wire name: `retention_level`.
   */
  readonly retentionLevel?: string;
}

const setDataSourceSpec: rt.FnSpec = {
  name: "setDataSource",
  wire: "set_data_source",
  args: [
    ["code", "code", true],
    ["retentionLevel", "retention_level", false],
  ],
};

/**
 * Update a data source's retention level.
 *
 * Wire name: `set_data_source` (group `datasources`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetDataSourceOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setDataSource(config: string, options: SetDataSourceOptions): string {
  return rt.callConfig(setDataSourceSpec, config, options);
}

/**
 * Arguments of {@link addElement}.
 */
export interface AddElementOptions {
  /**
   * Uppercased; duplicate (exact match on the uppercased code) is ALREADY_EXISTS.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * FELEM_DESC, stored verbatim; absent defaults to the uppercased code.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Case-insensitive; normalized to lowercase string, number, date, datetime or json, else INVALID_INPUT.
   *
   * Library default when omitted: `"string"` (applied by the library, not this binding).
   *
   * Wire name: `data_type`.
   */
  readonly dataType?: string;
  /**
   * Requested FELEM_ID. Absent OR <= 0 means auto-allocate (max existing + 1, floor 1000). A taken id > 0 is ALREADY_EXISTS.
   *
   * Wire name: `id`.
   */
  readonly id?: number | bigint;
}

const addElementSpec: rt.FnSpec = {
  name: "addElement",
  wire: "add_element",
  args: [
    ["code", "code", true],
    ["description", "description", false],
    ["dataType", "data_type", false],
    ["id", "id", false],
  ],
};

/**
 * Add an element (CFG_FELEM row).
 *
 * @remarks
 * Validation order: duplicate code, id, data_type.
 *
 * Wire name: `add_element` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addElement(config: string, options: AddElementOptions): string {
  return rt.callConfig(addElementSpec, config, options);
}

/**
 * Arguments of {@link deleteElement}.
 */
export interface DeleteElementOptions {
  /**
   * Uppercased before lookup; unknown is NOT_FOUND.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteElementSpec: rt.FnSpec = {
  name: "deleteElement",
  wire: "delete_element",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete an element that no feature uses.
 *
 * @remarks
 * INVALID_INPUT when any CFG_FBOM row maps the element to an existing feature ("Element linked to the following feature(s): ..."). TRAP: CFG_ATTR rows naming the element are NOT checked and are left dangling. MISSING_SECTION for an absent CFG_FELEM or CFG_FBOM; MISSING_FIELD when the matched row's FELEM_ID is not an integer.
 *
 * Wire name: `delete_element` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteElement(config: string, options: DeleteElementOptions): string {
  return rt.callConfig(deleteElementSpec, config, options);
}

/**
 * Arguments of {@link getElement}.
 */
export interface GetElementOptions {
  /**
   * Uppercased before lookup; unknown is NOT_FOUND.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getElementSpec: rt.FnSpec = {
  name: "getElement",
  wire: "get_element",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one element as a display summary by code.
 *
 * @remarks
 * Result is exactly {id, element, datatype}; FELEM_DESC is NOT included.
 *
 * Wire name: `get_element` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetElementOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getElement(config: string, options: GetElementOptions): string {
  return rt.callJson(getElementSpec, config, options);
}

const listElementsSpec: rt.FnSpec = {
  name: "listElements",
  wire: "list_elements",
  args: [],
};

/**
 * List all elements as display summaries, sorted by element code.
 *
 * @remarks
 * Array of {id, element, datatype}, sorted alphabetically by element code (not by id).
 *
 * Wire name: `list_elements` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listElements(config: string): string {
  return rt.callJson(listElementsSpec, config);
}

/**
 * Arguments of {@link setElement}.
 */
export interface SetElementOptions {
  /**
   * Uppercased before lookup; unknown is NOT_FOUND.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves FELEM_DESC; else stored verbatim.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * TRAP — absent leaves DATA_TYPE; else stored VERBATIM with no validation or normalization (unlike add_element).
   *
   * Wire name: `data_type`.
   */
  readonly dataType?: string;
}

const setElementSpec: rt.FnSpec = {
  name: "setElement",
  wire: "set_element",
  args: [
    ["code", "code", true],
    ["description", "description", false],
    ["dataType", "data_type", false],
  ],
};

/**
 * Update an element's description and/or data type.
 *
 * @remarks
 * Succeeds unchanged when no update args are given.
 *
 * Wire name: `set_element` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setElement(config: string, options: SetElementOptions): string {
  return rt.callConfig(setElementSpec, config, options);
}

/**
 * Arguments of {@link setFeatureElement}.
 */
export interface SetFeatureElementOptions {
  /**
   * Required. Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Required. Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * TRAP — stored verbatim; no uniqueness check and no <= 0 auto-allocation.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * Must be >= 0, else INVALID_INPUT.
   *
   * Wire name: `display_level`.
   */
  readonly displayLevel?: number | bigint;
  /**
   * Stored verbatim. Not tri-state; cannot be cleared back to null.
   *
   * Wire name: `display_delim`.
   */
  readonly displayDelim?: string;
  /**
   * Case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Wire name: `derived`.
   */
  readonly derived?: string;
}

const setFeatureElementSpec: rt.FnSpec = {
  name: "setFeatureElement",
  wire: "set_feature_element",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
    ["execOrder", "exec_order", false],
    ["displayLevel", "display_level", false],
    ["displayDelim", "display_delim", false],
    ["derived", "derived", false],
  ],
};

/**
 * Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter or derived flag.
 *
 * @remarks
 * NOT_FOUND when the (feature, element) mapping is absent. Succeeds unchanged when no update args are given. A missing CFG_FTYPE/CFG_FELEM surfaces as NOT_FOUND (code lookup).
 *
 * Wire name: `set_feature_element` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetFeatureElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setFeatureElement(config: string, options: SetFeatureElementOptions): string {
  return rt.callConfig(setFeatureElementSpec, config, options);
}

/**
 * Arguments of {@link addElementToFeature}.
 */
export interface AddElementToFeatureOptions {
  /**
   * Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Case-insensitive; unknown is NOT_FOUND (the element is never auto-created).
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * Must be >= 0, else INVALID_INPUT.
   *
   * Library default when omitted: `1` (applied by the library, not this binding).
   *
   * Wire name: `display_level`.
   */
  readonly displayLevel?: number | bigint;
  /**
   * Stored verbatim; absent stores null.
   *
   * Wire name: `display_delim`.
   */
  readonly displayDelim?: string;
  /**
   * Case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `derived`.
   */
  readonly derived?: string;
}

const addElementToFeatureSpec: rt.FnSpec = {
  name: "addElementToFeature",
  wire: "add_element_to_feature",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
    ["displayLevel", "display_level", false],
    ["displayDelim", "display_delim", false],
    ["derived", "derived", false],
  ],
};

/**
 * Map an existing element to a feature (append a CFG_FBOM row).
 *
 * @remarks
 * EXEC_ORDER is always auto-allocated as max(EXEC_ORDER over the WHOLE CFG_FBOM table) + 1 and cannot be requested (use features.add_feature_comparison for an explicit order). Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. Validation order: feature, element, display_level, derived, CFG_FBOM section, duplicate.
 *
 * Wire name: `add_element_to_feature` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddElementToFeatureOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addElementToFeature(config: string, options: AddElementToFeatureOptions): string {
  return rt.callConfig(addElementToFeatureSpec, config, options);
}

/**
 * Arguments of {@link deleteElementFromFeature}.
 */
export interface DeleteElementFromFeatureOptions {
  /**
   * Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
}

const deleteElementFromFeatureSpec: rt.FnSpec = {
  name: "deleteElementFromFeature",
  wire: "delete_element_from_feature",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
  ],
};

/**
 * Remove one feature-element (CFG_FBOM) mapping.
 *
 * @remarks
 * NOT_FOUND when the mapping is absent. The CFG_FELEM row is kept.
 *
 * Wire name: `delete_element_from_feature` (group `elements`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteElementFromFeatureOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteElementFromFeature(config: string, options: DeleteElementFromFeatureOptions): string {
  return rt.callConfig(deleteElementFromFeatureSpec, config, options);
}

/**
 * Arguments of {@link renderConfig}.
 */
export interface RenderConfigOptions {
  /**
   * Spaces per indentation level, applied verbatim (2 = CLI on-disk form, 4 = Python parity; 0 allowed). Negative is INVALID_INPUT (raised by the req_usize converter, before the library is called).
   *
   * Wire name: `indent`.
   */
  readonly indent: number | bigint;
}

const renderConfigSpec: rt.FnSpec = {
  name: "renderConfig",
  wire: "render_config",
  args: [
    ["indent", "indent", true],
  ],
};

/**
 * Render a config document in canonical export form (recursively key-sorted, pretty-printed).
 *
 * @remarks
 * Result is a JSON STRING holding the rendered text (not the working config envelope): every object's keys sorted recursively (Python json.dumps(sort_keys=True)), array order kept, no trailing newline. The text is itself a valid config and may be passed back in. Any JSON document is accepted (no G2_CONFIG check).
 *
 * Wire name: `render_config` (group `export`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link RenderConfigOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function renderConfig(config: string, options: RenderConfigOptions): string {
  return rt.callJson(renderConfigSpec, config, options);
}

/**
 * Arguments of {@link addFeature}.
 */
export interface AddFeatureOptions {
  /**
   * Uppercased; stored as FTYPE_CODE and FTYPE_DESC. Duplicate (exact match on the uppercased code) is ALREADY_EXISTS.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * Must be a non-empty JSON array, else INVALID_INPUT. Each item is either an element-code string, or an object with `element` (or `ELEMENT`, required, else INVALID_INPUT) and optional `expressed`/`EXPRESSED`, `compared`/`COMPARED` ("yes" case-insensitive = true), `display`/`DISPLAY` ("yes" = DISPLAY_LEVEL 1, anything else 0) or `displaylevel`/`DISPLAYLEVEL`/`display_level` (int, default 1, negative = INVALID_INPUT), `displaydelim`/`DISPLAYDELIM`/`display_delim`, `derived`/`DERIVED` (Yes/No case-insensitive, else INVALID_INPUT; default No). Any other item type is INVALID_INPUT. Element codes are uppercased; a code not in CFG_FELEM is AUTO-CREATED (FELEM_ID max+1 floor 1000, DATA_TYPE string, FELEM_DESC = code). The FBOM EXEC_ORDER is the item's 1-based position (per feature, not whole-table).
   *
   * Shape: `[string | {element?: string, ELEMENT?: string, expressed?: string, EXPRESSED?: string, compared?: string, COMPARED?: string, display?: string, DISPLAY?: string, displaylevel?: int, DISPLAYLEVEL?: int, display_level?: int, displaydelim?: string, DISPLAYDELIM?: string, display_delim?: string, derived?: string, DERIVED?: string}]`.
   *
   * Wire name: `element_list`.
   */
  readonly elementList: ReadonlyArray<string | { readonly element?: string; readonly ELEMENT?: string; readonly expressed?: string; readonly EXPRESSED?: string; readonly compared?: string; readonly COMPARED?: string; readonly display?: string; readonly DISPLAY?: string; readonly displaylevel?: number | bigint; readonly DISPLAYLEVEL?: number | bigint; readonly display_level?: number | bigint; readonly displaydelim?: string; readonly DISPLAYDELIM?: string; readonly display_delim?: string; readonly derived?: string; readonly DERIVED?: string }>;
  /**
   * CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND.
   *
   * Library default when omitted: `"OTHER"` (applied by the library, not this binding).
   *
   * Wire name: `class`.
   */
  readonly class?: string;
  /**
   * Behavior code (A1, F1, FF, FM, FVM, NONE, NAME; E/S suffixes set FTYPE_EXCL/FTYPE_STAB), case-insensitive; otherwise INVALID_INPUT.
   *
   * Library default when omitted: `"FM"` (applied by the library, not this binding).
   *
   * Wire name: `behavior`.
   */
  readonly behavior?: string;
  /**
   * USED_FOR_CAND; case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `candidates`.
   */
  readonly candidates?: string;
  /**
   * ANONYMIZE; case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `anonymize`.
   */
  readonly anonymize?: string;
  /**
   * DERIVED; case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `derived`.
   */
  readonly derived?: string;
  /**
   * PERSIST_HISTORY; case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Library default when omitted: `"Yes"` (applied by the library, not this binding).
   *
   * Wire name: `history`.
   */
  readonly history?: string;
  /**
   * SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial, normalized, else INVALID_INPUT. Absent defaults to Yes when `comparison` is given, else No.
   *
   * Wire name: `matchkey`.
   */
  readonly matchkey?: string;
  /**
   * CFG_SFUNC code (case-insensitive) or NOT_FOUND (an empty string is looked up too, so "" is NOT_FOUND). Creates a CFG_SFCALL row (SFCALL_ID max+1 floor 1000, EXEC_ORDER 1, FELEM_ID -1).
   *
   * Wire name: `standardize`.
   */
  readonly standardize?: string;
  /**
   * CFG_EFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND). Requires at least one element_list item with expressed=yes, else INVALID_INPUT. Creates a CFG_EFCALL row (EFCALL_ID max+1 floor 1000, EXEC_ORDER 1) and a CFG_EFBOM row (FELEM_REQ Yes) per expressed element.
   *
   * Wire name: `expression`.
   */
  readonly expression?: string;
  /**
   * CFG_CFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND). Requires at least one element_list item with compared=yes, else INVALID_INPUT. Creates a CFG_CFCALL row (CFCALL_ID max+1 floor 1000) and a CFG_CFBOM row per compared element.
   *
   * Wire name: `comparison`.
   */
  readonly comparison?: string;
  /**
   * Stored verbatim in VERSION.
   *
   * Library default when omitted: `1` (applied by the library, not this binding).
   *
   * Wire name: `version`.
   */
  readonly version?: number | bigint;
  /**
   * Stored verbatim in RTYPE_ID; not validated against CFG_RTYPE.
   *
   * Library default when omitted: `0` (applied by the library, not this binding).
   *
   * Wire name: `rtype_id`.
   */
  readonly rtypeId?: number | bigint;
  /**
   * Requested FTYPE_ID. Absent OR <= 0 means auto-allocate (max existing + 1, floor 1000). A taken id > 0 is ALREADY_EXISTS.
   *
   * Wire name: `id`.
   */
  readonly id?: number | bigint;
}

const addFeatureSpec: rt.FnSpec = {
  name: "addFeature",
  wire: "add_feature",
  args: [
    ["feature", "feature", true],
    ["elementList", "element_list", true, { array: { oneOf: ["string", { object: { "element?": "string", "ELEMENT?": "string", "expressed?": "string", "EXPRESSED?": "string", "compared?": "string", "COMPARED?": "string", "display?": "string", "DISPLAY?": "string", "displaylevel?": "int", "DISPLAYLEVEL?": "int", "display_level?": "int", "displaydelim?": "string", "DISPLAYDELIM?": "string", "display_delim?": "string", "derived?": "string", "DERIVED?": "string" } }] } }],
    ["class", "class", false],
    ["behavior", "behavior", false],
    ["candidates", "candidates", false],
    ["anonymize", "anonymize", false],
    ["derived", "derived", false],
    ["history", "history", false],
    ["matchkey", "matchkey", false],
    ["standardize", "standardize", false],
    ["expression", "expression", false],
    ["comparison", "comparison", false],
    ["version", "version", false],
    ["rtypeId", "rtype_id", false],
    ["id", "id", false],
  ],
};

/**
 * Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional standardize/expression/comparison calls.
 *
 * @remarks
 * Validation order: CFG_FTYPE section, duplicate code, element_list shape, candidates/anonymize/derived/history/matchkey domains, id, behavior, class, function codes, expressed/compared counts, then per-element (display level, derived). The input config is never modified on error.
 *
 * Wire name: `add_feature` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddFeatureOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND, INVALID_STRUCTURE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addFeature(config: string, options: AddFeatureOptions): string {
  return rt.callConfig(addFeatureSpec, config, options);
}

/**
 * Arguments of {@link deleteFeature}.
 */
export interface DeleteFeatureOptions {
  /**
   * A string that parses (after trim) as an integer is an FTYPE_ID; otherwise a feature code, uppercased. Unknown is NOT_FOUND.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
}

const deleteFeatureSpec: rt.FnSpec = {
  name: "deleteFeature",
  wire: "delete_feature",
  args: [
    ["feature", "feature", true],
  ],
};

/**
 * Delete a feature and cascade-delete its FBOM rows, attributes and standardize/expression/comparison/distinct calls.
 *
 * @remarks
 * Locked features NAME, ADDRESS, PHONE, DOB, REL_LINK, REL_ANCHOR, REL_POINTER are INVALID_INPUT. Cascade removes the feature's CFG_FBOM rows, CFG_ATTR rows whose FTYPE_CODE matches, CFG_SFCALL, CFG_EFCALL (+ their CFG_EFBOM), CFG_CFCALL (+ CFG_CFBOM) and CFG_DFCALL (+ CFG_DFBOM) rows; CFG_FELEM rows are kept. A missing CFG_FTYPE is MISSING_SECTION for an id but NOT_FOUND for a code.
 *
 * Wire name: `delete_feature` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteFeatureOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteFeature(config: string, options: DeleteFeatureOptions): string {
  return rt.callConfig(deleteFeatureSpec, config, options);
}

/**
 * Arguments of {@link getFeature}.
 */
export interface GetFeatureOptions {
  /**
   * Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
}

const getFeatureSpec: rt.FnSpec = {
  name: "getFeature",
  wire: "get_feature",
  args: [
    ["feature", "feature", true],
  ],
};

/**
 * Get one feature as a display summary including its elementList.
 *
 * @remarks
 * Result {id, feature, class, behavior, anonymize, candidates, standardize, expression, comparison, matchKey, version, elementList}; elementList items {element, expressed, compared, derived, display} sorted by EXEC_ORDER, display = "No" iff DISPLAY_LEVEL is 0. standardize/expression/comparison are "" when absent. DERIVED, PERSIST_HISTORY and RTYPE_ID of the feature are NOT in the result. A missing CFG_FTYPE section is NOT_FOUND (never MISSING_SECTION).
 *
 * Wire name: `get_feature` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetFeatureOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getFeature(config: string, options: GetFeatureOptions): string {
  return rt.callJson(getFeatureSpec, config, options);
}

const listFeaturesSpec: rt.FnSpec = {
  name: "listFeatures",
  wire: "list_features",
  args: [],
};

/**
 * List all features as display summaries, sorted by FTYPE_ID.
 *
 * @remarks
 * Array of the same objects get_feature returns.
 *
 * Wire name: `list_features` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listFeatures(config: string): string {
  return rt.callJson(listFeaturesSpec, config);
}

/**
 * Arguments of {@link setFeature}.
 */
export interface SetFeatureOptions {
  /**
   * Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased. Unknown is NOT_FOUND.
   *
   * Wire name: `feature`.
   */
  readonly feature: string;
  /**
   * USED_FOR_CAND; case-insensitive Yes/No, normalized, else INVALID_INPUT.
   *
   * Wire name: `candidates`.
   */
  readonly candidates?: string;
  /**
   * TRAP — stored VERBATIM in ANONYMIZE (no validation or normalization, unlike add_feature).
   *
   * Wire name: `anonymize`.
   */
  readonly anonymize?: string;
  /**
   * TRAP — stored VERBATIM in DERIVED (no validation or normalization).
   *
   * Wire name: `derived`.
   */
  readonly derived?: string;
  /**
   * TRAP — stored VERBATIM in PERSIST_HISTORY (no validation or normalization).
   *
   * Wire name: `history`.
   */
  readonly history?: string;
  /**
   * SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial, normalized, else INVALID_INPUT.
   *
   * Wire name: `matchkey`.
   */
  readonly matchkey?: string;
  /**
   * Behavior code (case-insensitive) parsed into FTYPE_FREQ/FTYPE_EXCL/FTYPE_STAB, else INVALID_INPUT.
   *
   * Wire name: `behavior`.
   */
  readonly behavior?: string;
  /**
   * CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `class`.
   */
  readonly class?: string;
  /**
   * Stored verbatim in VERSION.
   *
   * Wire name: `version`.
   */
  readonly version?: number | bigint;
  /**
   * Stored verbatim in RTYPE_ID; not validated.
   *
   * Wire name: `rtype_id`.
   */
  readonly rtypeId?: number | bigint;
}

const setFeatureSpec: rt.FnSpec = {
  name: "setFeature",
  wire: "set_feature",
  args: [
    ["feature", "feature", true],
    ["candidates", "candidates", false],
    ["anonymize", "anonymize", false],
    ["derived", "derived", false],
    ["history", "history", false],
    ["matchkey", "matchkey", false],
    ["behavior", "behavior", false],
    ["class", "class", false],
    ["version", "version", false],
    ["rtypeId", "rtype_id", false],
  ],
};

/**
 * Update a feature's flags, behavior, class, version or RTYPE_ID.
 *
 * @remarks
 * If no supplied value differs from the stored row (including when no update args are given) the call fails with INVALID_INPUT "No changes detected". The input config is never modified on error.
 *
 * Wire name: `set_feature` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetFeatureOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setFeature(config: string, options: SetFeatureOptions): string {
  return rt.callConfig(setFeatureSpec, config, options);
}

/**
 * Arguments of {@link addFeatureComparison}.
 */
export interface AddFeatureComparisonOptions {
  /**
   * Required. Feature code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Required. Element code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
  /**
   * WHOLE-TABLE scope: absent or <= 0 auto-allocates max(EXEC_ORDER over all of CFG_FBOM) + 1; a value > 0 already used by ANY CFG_FBOM row is ALREADY_EXISTS.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * TRAP — stored verbatim, NOT validated, and absent stores DISPLAY_LEVEL null (no default 1, unlike add_element_to_feature).
   *
   * Wire name: `display_level`.
   */
  readonly displayLevel?: number | bigint;
  /**
   * Stored verbatim; absent stores null.
   *
   * Wire name: `display_delim`.
   */
  readonly displayDelim?: string;
  /**
   * TRAP — stored verbatim, NOT validated; absent stores DERIVED null.
   *
   * Wire name: `derived`.
   */
  readonly derived?: string;
}

const addFeatureComparisonSpec: rt.FnSpec = {
  name: "addFeatureComparison",
  wire: "add_feature_comparison",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
    ["execOrder", "exec_order", false],
    ["displayLevel", "display_level", false],
    ["displayDelim", "display_delim", false],
    ["derived", "derived", false],
  ],
};

/**
 * Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.
 *
 * @remarks
 * Writes the same table as elements.add_element_to_feature. Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. A missing CFG_FTYPE/CFG_FELEM section surfaces as NOT_FOUND (code lookup); a missing CFG_FBOM as MISSING_SECTION.
 *
 * Wire name: `add_feature_comparison` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddFeatureComparisonOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addFeatureComparison(config: string, options: AddFeatureComparisonOptions): string {
  return rt.callConfig(addFeatureComparisonSpec, config, options);
}

/**
 * Arguments of {@link deleteFeatureComparison}.
 */
export interface DeleteFeatureComparisonOptions {
  /**
   * Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
}

const deleteFeatureComparisonSpec: rt.FnSpec = {
  name: "deleteFeatureComparison",
  wire: "delete_feature_comparison",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
  ],
};

/**
 * Delete one feature-element (CFG_FBOM) row.
 *
 * @remarks
 * Same effect as elements.delete_element_from_feature, except a missing CFG_FBOM section is NOT_FOUND here (MISSING_SECTION there). An absent mapping is NOT_FOUND.
 *
 * Wire name: `delete_feature_comparison` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteFeatureComparisonOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteFeatureComparison(config: string, options: DeleteFeatureComparisonOptions): string {
  return rt.callConfig(deleteFeatureComparisonSpec, config, options);
}

/**
 * Arguments of {@link getFeatureComparison}.
 */
export interface GetFeatureComparisonOptions {
  /**
   * Required. Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Required. Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `element_code`.
   */
  readonly elementCode: string;
}

const getFeatureComparisonSpec: rt.FnSpec = {
  name: "getFeatureComparison",
  wire: "get_feature_comparison",
  args: [
    ["featureCode", "feature_code", true],
    ["elementCode", "element_code", true],
  ],
};

/**
 * Get one raw CFG_FBOM row by feature and element code.
 *
 * @remarks
 * Result uses on-disk keys (FTYPE_ID, FELEM_ID, EXEC_ORDER, DISPLAY_LEVEL, DISPLAY_DELIM, DERIVED).
 *
 * Wire name: `get_feature_comparison` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetFeatureComparisonOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getFeatureComparison(config: string, options: GetFeatureComparisonOptions): string {
  return rt.callJson(getFeatureComparisonSpec, config, options);
}

const listFeatureComparisonsSpec: rt.FnSpec = {
  name: "listFeatureComparisons",
  wire: "list_feature_comparisons",
  args: [],
};

/**
 * List all raw CFG_FBOM rows sorted by (FTYPE_ID, EXEC_ORDER).
 *
 * Wire name: `list_feature_comparisons` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listFeatureComparisons(config: string): string {
  return rt.callJson(listFeatureComparisonsSpec, config);
}

/**
 * Arguments of {@link addFeatureDistinctCallElement}.
 */
export interface AddFeatureDistinctCallElementOptions {
  /**
   * Required. Case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `feature_code`.
   */
  readonly featureCode: string;
  /**
   * Required. CFG_DFUNC code, case-insensitive; unknown is NOT_FOUND.
   *
   * Wire name: `distinct_func_code`.
   */
  readonly distinctFuncCode: string;
  /**
   * TRAP — only validated (unknown is NOT_FOUND); it is NOT stored (CFG_DFCALL has no FELEM_ID column).
   *
   * Wire name: `element_code`.
   */
  readonly elementCode?: string;
  /**
   * TRAP — IGNORED entirely (CFG_DFCALL has no EXEC_ORDER column).
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
}

const addFeatureDistinctCallElementSpec: rt.FnSpec = {
  name: "addFeatureDistinctCallElement",
  wire: "add_feature_distinct_call_element",
  args: [
    ["featureCode", "feature_code", true],
    ["distinctFuncCode", "distinct_func_code", true],
    ["elementCode", "element_code", false],
    ["execOrder", "exec_order", false],
  ],
};

/**
 * Add a distinct-function call (CFG_DFCALL row) for a feature.
 *
 * @remarks
 * Duplicate (FTYPE_ID, DFUNC_ID) is ALREADY_EXISTS. New row is exactly {DFCALL_ID (max+1 floor 1000), FTYPE_ID, DFUNC_ID}; no CFG_DFBOM rows are written.
 *
 * Wire name: `add_feature_distinct_call_element` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddFeatureDistinctCallElementOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addFeatureDistinctCallElement(config: string, options: AddFeatureDistinctCallElementOptions): string {
  return rt.callConfig(addFeatureDistinctCallElementSpec, config, options);
}

const listFeatureClassesSpec: rt.FnSpec = {
  name: "listFeatureClasses",
  wire: "list_feature_classes",
  args: [],
};

/**
 * List all raw CFG_FCLASS rows sorted by FCLASS_ID.
 *
 * Wire name: `list_feature_classes` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listFeatureClasses(config: string): string {
  return rt.callJson(listFeatureClassesSpec, config);
}

/**
 * Arguments of {@link getFeatureClass}.
 */
export interface GetFeatureClassOptions {
  /**
   * Integer string (after trim) = FCLASS_ID; otherwise a code, uppercased. Unknown is NOT_FOUND.
   *
   * Wire name: `feature_class`.
   */
  readonly featureClass: string;
}

const getFeatureClassSpec: rt.FnSpec = {
  name: "getFeatureClass",
  wire: "get_feature_class",
  args: [
    ["featureClass", "feature_class", true],
  ],
};

/**
 * Get one raw CFG_FCLASS row by id or code.
 *
 * Wire name: `get_feature_class` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetFeatureClassOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getFeatureClass(config: string, options: GetFeatureClassOptions): string {
  return rt.callJson(getFeatureClassSpec, config, options);
}

/**
 * Arguments of {@link updateFeatureVersion}.
 */
export interface UpdateFeatureVersionOptions {
  /**
   * Stored verbatim as a string (inserted or overwritten); not validated.
   *
   * Wire name: `version`.
   */
  readonly version: string;
}

const updateFeatureVersionSpec: rt.FnSpec = {
  name: "updateFeatureVersion",
  wire: "update_feature_version",
  args: [
    ["version", "version", true],
  ],
};

/**
 * Set G2_CONFIG.CONFIG_BASE_VERSION.COMPATIBILITY_VERSION.FEATURE_VERSION.
 *
 * @remarks
 * MISSING_SECTION when COMPATIBILITY_VERSION is absent or not an object. No manifest function reads FEATURE_VERSION back (versioning reads CONFIG_VERSION).
 *
 * Wire name: `update_feature_version` (group `features`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link UpdateFeatureVersionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function updateFeatureVersion(config: string, options: UpdateFeatureVersionOptions): string {
  return rt.callConfig(updateFeatureVersionSpec, config, options);
}

/**
 * Arguments of {@link addFragment}.
 */
export interface AddFragmentOptions {
  /**
   * Object with on-disk keys. ERFRAG_CODE (string, required, else MISSING_FIELD) is uppercased for storage and the duplicate check (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is stored verbatim; every name referenced inside a FRAGMENT[...] clause (e.g. "./FRAGMENT[./SAME_NAME>0 and ./SAME_STAB>0]") must be an existing ERFRAG_CODE matched EXACTLY (case-sensitive), else INVALID_INPUT. A source without FRAGMENT[ (including "") is accepted unvalidated. ERFRAG_ID (integer, optional): absent or <= 0 auto-allocates (max + 1, floor 1, so 1000 on the template); a taken id > 0 is ALREADY_EXISTS. Any ERFRAG_DESC key is IGNORED.
   *
   * Shape: `{ERFRAG_CODE: string, ERFRAG_SOURCE: string, ERFRAG_ID?: int, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}`.
   *
   * Wire name: `fragment_config`.
   */
  readonly fragmentConfig: { readonly ERFRAG_CODE: string; readonly ERFRAG_SOURCE: string; readonly ERFRAG_ID?: number | bigint; readonly ERFRAG_DESC?: rt.JsonValue; readonly ERFRAG_DEPENDS?: rt.JsonValue };
}

const addFragmentSpec: rt.FnSpec = {
  name: "addFragment",
  wire: "add_fragment",
  args: [
    ["fragmentConfig", "fragment_config", true, { object: { "ERFRAG_CODE": "string", "ERFRAG_SOURCE": "string", "ERFRAG_ID?": "int", "ERFRAG_DESC?": "any", "ERFRAG_DEPENDS?": "any" } }],
  ],
};

/**
 * Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
 *
 * @remarks
 * Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is MISSING_SECTION.
 *
 * Wire name: `add_fragment` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddFragmentOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addFragmentResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addFragment(config: string, options: AddFragmentOptions): string {
  return rt.callConfig(addFragmentSpec, config, options);
}

const addFragmentResultSpec: rt.FnSpec = { ...addFragmentSpec, name: "addFragmentResult" };

/**
 * The record (row / ids) of {@link addFragment}: same options and operation, but returns the record instead of the configuration. Operation: Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
 *
 * @remarks
 * Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is MISSING_SECTION.
 *
 * Wire name: `add_fragment` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddFragmentOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addFragmentResult(config: string, options: AddFragmentOptions): string {
  return rt.callJson(addFragmentResultSpec, config, options);
}

/**
 * Arguments of {@link deleteFragment}.
 */
export interface DeleteFragmentOptions {
  /**
   * Uppercased, then matched exactly against ERFRAG_CODE. Not an id.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteFragmentSpec: rt.FnSpec = {
  name: "deleteFragment",
  wire: "delete_fragment",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a fragment by code.
 *
 * @remarks
 * TRAP: no dependency check; a fragment still referenced by a rule (QUAL_ERFRAG_CODE / DISQ_ERFRAG_CODE) or by another fragment's ERFRAG_DEPENDS is deleted anyway, leaving dangling references. A config without CFG_ERFRAG is NOT_FOUND.
 *
 * Wire name: `delete_fragment` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteFragmentOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteFragment(config: string, options: DeleteFragmentOptions): string {
  return rt.callConfig(deleteFragmentSpec, config, options);
}

/**
 * Arguments of {@link getFragment}.
 */
export interface GetFragmentOptions {
  /**
   * Uppercased, then matched exactly against ERFRAG_CODE first, then numerically against ERFRAG_ID (e.g. "11").
   *
   * Wire name: `code_or_id`.
   */
  readonly codeOrId: string;
}

const getFragmentSpec: rt.FnSpec = {
  name: "getFragment",
  wire: "get_fragment",
  args: [
    ["codeOrId", "code_or_id", true],
  ],
};

/**
 * Get one fragment, by code or ERFRAG_ID, as a summary record.
 *
 * @remarks
 * Result is {id, fragment, source, depends}: id/source/depends are null-preserving projections of ERFRAG_ID/ERFRAG_SOURCE/ERFRAG_DEPENDS; fragment is ERFRAG_CODE or "" when absent. ERFRAG_DESC is not reported.
 *
 * Wire name: `get_fragment` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetFragmentOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getFragment(config: string, options: GetFragmentOptions): string {
  return rt.callJson(getFragmentSpec, config, options);
}

const listFragmentsSpec: rt.FnSpec = {
  name: "listFragments",
  wire: "list_fragments",
  args: [],
};

/**
 * List all fragments as summary records in config order.
 *
 * @remarks
 * Result is an array of the get_fragment record shape, in config order (NOT sorted). A missing CFG_ERFRAG or G2_CONFIG yields [].
 *
 * Wire name: `list_fragments` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listFragments(config: string): string {
  return rt.callJson(listFragmentsSpec, config);
}

/**
 * Arguments of {@link setFragment}.
 */
export interface SetFragmentOptions {
  /**
   * Fragment code; uppercased, then matched exactly. Unknown is NOT_FOUND.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * ERFRAG_SOURCE. Absent = keep source AND ERFRAG_DEPENDS; null = clear BOTH source and ERFRAG_DEPENDS to null; a string is validated exactly as in add_fragment (INVALID_INPUT) and ERFRAG_DEPENDS is recomputed.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `source`.
   */
  readonly source?: string | null;
  /**
   * ERFRAG_DESC. Absent = keep; null = clear to null; a string is stored verbatim.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `description`.
   */
  readonly description?: string | null;
}

const setFragmentSpec: rt.FnSpec = {
  name: "setFragment",
  wire: "set_fragment",
  args: [
    ["code", "code", true],
    ["source", "source", false],
    ["description", "description", false],
  ],
};

/**
 * Update a fragment's source and/or description.
 *
 * @remarks
 * The row is rewritten with every CFG_ERFRAG key; ERFRAG_ID is preserved.
 *
 * Wire name: `set_fragment` (group `fragments`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetFragmentOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setFragment(config: string, options: SetFragmentOptions): string {
  return rt.callConfig(setFragmentSpec, config, options);
}

/**
 * Arguments of {@link addComparisonFunction}.
 */
export interface AddComparisonFunctionOptions {
  /**
   * Uppercased before the duplicate check and storage (CFUNC_CODE).
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string;
  /**
   * Absent stores CFUNC_DESC null; any string is stored verbatim.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent stores LANGUAGE null; any string is stored verbatim.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
  /**
   * Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `anon_support`.
   */
  readonly anonSupport?: string;
}

const addComparisonFunctionSpec: rt.FnSpec = {
  name: "addComparisonFunction",
  wire: "add_comparison_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
    ["anonSupport", "anon_support", false],
  ],
};

/**
 * Add a comparison function (CFG_CFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE, CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code, anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
 *
 * Wire name: `add_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addComparisonFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonFunction(config: string, options: AddComparisonFunctionOptions): string {
  return rt.callConfig(addComparisonFunctionSpec, config, options);
}

const addComparisonFunctionResultSpec: rt.FnSpec = { ...addComparisonFunctionSpec, name: "addComparisonFunctionResult" };

/**
 * The record (row / ids) of {@link addComparisonFunction}: same options and operation, but returns the record instead of the configuration. Operation: Add a comparison function (CFG_CFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE, CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code, anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
 *
 * Wire name: `add_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonFunctionResult(config: string, options: AddComparisonFunctionOptions): string {
  return rt.callJson(addComparisonFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteComparisonFunction}.
 */
export interface DeleteComparisonFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteComparisonFunctionSpec: rt.FnSpec = {
  name: "deleteComparisonFunction",
  wire: "delete_comparison_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a comparison function's CFG_CFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row; CFG_CFCALL rows referencing it are left dangling (use delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteComparisonFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonFunction(config: string, options: DeleteComparisonFunctionOptions): string {
  return rt.callConfig(deleteComparisonFunctionSpec, config, options);
}

const deleteComparisonFunctionResultSpec: rt.FnSpec = { ...deleteComparisonFunctionSpec, name: "deleteComparisonFunctionResult" };

/**
 * The record (row / ids) of {@link deleteComparisonFunction}: same options and operation, but returns the record instead of the configuration. Operation: Delete a comparison function's CFG_CFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row; CFG_CFCALL rows referencing it are left dangling (use delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonFunctionResult(config: string, options: DeleteComparisonFunctionOptions): string {
  return rt.callJson(deleteComparisonFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteComparisonFunctionCascade}.
 */
export interface DeleteComparisonFunctionCascadeOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteComparisonFunctionCascadeSpec: rt.FnSpec = {
  name: "deleteComparisonFunctionCascade",
  wire: "delete_comparison_function_cascade",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_comparison_function_cascade` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonFunctionCascadeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteComparisonFunctionCascadeResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonFunctionCascade(config: string, options: DeleteComparisonFunctionCascadeOptions): string {
  return rt.callConfig(deleteComparisonFunctionCascadeSpec, config, options);
}

const deleteComparisonFunctionCascadeResultSpec: rt.FnSpec = { ...deleteComparisonFunctionCascadeSpec, name: "deleteComparisonFunctionCascadeResult" };

/**
 * The record (row / ids) of {@link deleteComparisonFunctionCascade}: same options and operation, but returns the record instead of the configuration. Operation: Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_comparison_function_cascade` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonFunctionCascadeOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonFunctionCascadeResult(config: string, options: DeleteComparisonFunctionCascadeOptions): string {
  return rt.callJson(deleteComparisonFunctionCascadeResultSpec, config, options);
}

/**
 * Arguments of {@link getComparisonFunction}.
 */
export interface GetComparisonFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getComparisonFunctionSpec: rt.FnSpec = {
  name: "getComparisonFunction",
  wire: "get_comparison_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one comparison function's raw CFG_CFUNC row by code.
 *
 * @remarks
 * Result uses on-disk keys (CFUNC_ID, CFUNC_CODE, CFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `get_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetComparisonFunctionOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getComparisonFunction(config: string, options: GetComparisonFunctionOptions): string {
  return rt.callJson(getComparisonFunctionSpec, config, options);
}

const listComparisonFunctionsSpec: rt.FnSpec = {
  name: "listComparisonFunctions",
  wire: "list_comparison_functions",
  args: [],
};

/**
 * List all comparison functions as camelCase summaries.
 *
 * @remarks
 * Result is an array of {id, function, description, connectStr, anonSupport, language} in config order; all but function are null-preserving. A missing CFG_CFUNC section yields [] (no error).
 *
 * Wire name: `list_comparison_functions` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listComparisonFunctions(config: string): string {
  return rt.callJson(listComparisonFunctionsSpec, config);
}

/**
 * Arguments of {@link setComparisonFunction}.
 */
export interface SetComparisonFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string | null;
  /**
   * Absent leaves CFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
  /**
   * Absent leaves ANON_SUPPORT. TRAP: unlike add, NOT validated or normalized; any string is stored verbatim.
   *
   * Wire name: `anon_support`.
   */
  readonly anonSupport?: string;
}

const setComparisonFunctionSpec: rt.FnSpec = {
  name: "setComparisonFunction",
  wire: "set_comparison_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
    ["anonSupport", "anon_support", false],
  ],
};

/**
 * Update a comparison function's connect string / description / language / anon support.
 *
 * @remarks
 * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC section is NOT_FOUND.
 *
 * Wire name: `set_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetComparisonFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link setComparisonFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setComparisonFunction(config: string, options: SetComparisonFunctionOptions): string {
  return rt.callConfig(setComparisonFunctionSpec, config, options);
}

const setComparisonFunctionResultSpec: rt.FnSpec = { ...setComparisonFunctionSpec, name: "setComparisonFunctionResult" };

/**
 * The record (row / ids) of {@link setComparisonFunction}: same options and operation, but returns the record instead of the configuration. Operation: Update a comparison function's connect string / description / language / anon support.
 *
 * @remarks
 * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC section is NOT_FOUND.
 *
 * Wire name: `set_comparison_function` (group `functions_comparison`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetComparisonFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setComparisonFunctionResult(config: string, options: SetComparisonFunctionOptions): string {
  return rt.callJson(setComparisonFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link addDistinctFunction}.
 */
export interface AddDistinctFunctionOptions {
  /**
   * Uppercased before the duplicate check and storage (DFUNC_CODE).
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string;
  /**
   * Stored verbatim in DFUNC_DESC; absent stores null (NOT defaulted to the code).
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Stored verbatim in LANGUAGE; absent stores null.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
  /**
   * Case-insensitive; normalized to Yes or No, any other value is INVALID_INPUT.
   *
   * Library default when omitted: `"No"` (applied by the library, not this binding).
   *
   * Wire name: `anon_support`.
   */
  readonly anonSupport?: string;
}

const addDistinctFunctionSpec: rt.FnSpec = {
  name: "addDistinctFunction",
  wire: "add_distinct_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
    ["anonSupport", "anon_support", false],
  ],
};

/**
 * Add a distinct function (CFG_DFUNC row) to the configuration.
 *
 * @remarks
 * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE, DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
 *
 * Wire name: `add_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addDistinctFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctFunction(config: string, options: AddDistinctFunctionOptions): string {
  return rt.callConfig(addDistinctFunctionSpec, config, options);
}

const addDistinctFunctionResultSpec: rt.FnSpec = { ...addDistinctFunctionSpec, name: "addDistinctFunctionResult" };

/**
 * The record (row / ids) of {@link addDistinctFunction}: same options and operation, but returns the record instead of the configuration. Operation: Add a distinct function (CFG_DFUNC row) to the configuration.
 *
 * @remarks
 * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE, DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
 *
 * Wire name: `add_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddDistinctFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addDistinctFunctionResult(config: string, options: AddDistinctFunctionOptions): string {
  return rt.callJson(addDistinctFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteDistinctFunction}.
 */
export interface DeleteDistinctFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteDistinctFunctionSpec: rt.FnSpec = {
  name: "deleteDistinctFunction",
  wire: "delete_distinct_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a distinct function by code.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteDistinctFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteDistinctFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteDistinctFunction(config: string, options: DeleteDistinctFunctionOptions): string {
  return rt.callConfig(deleteDistinctFunctionSpec, config, options);
}

const deleteDistinctFunctionResultSpec: rt.FnSpec = { ...deleteDistinctFunctionSpec, name: "deleteDistinctFunctionResult" };

/**
 * The record (row / ids) of {@link deleteDistinctFunction}: same options and operation, but returns the record instead of the configuration. Operation: Delete a distinct function by code.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteDistinctFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteDistinctFunctionResult(config: string, options: DeleteDistinctFunctionOptions): string {
  return rt.callJson(deleteDistinctFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link getDistinctFunction}.
 */
export interface GetDistinctFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getDistinctFunctionSpec: rt.FnSpec = {
  name: "getDistinctFunction",
  wire: "get_distinct_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one distinct function's raw CFG_DFUNC row by code.
 *
 * @remarks
 * Result is the stored row with on-disk keys. A config without CFG_DFUNC is NOT_FOUND.
 *
 * Wire name: `get_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetDistinctFunctionOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getDistinctFunction(config: string, options: GetDistinctFunctionOptions): string {
  return rt.callJson(getDistinctFunctionSpec, config, options);
}

const listDistinctFunctionsSpec: rt.FnSpec = {
  name: "listDistinctFunctions",
  wire: "list_distinct_functions",
  args: [],
};

/**
 * List all distinct functions as {id, function, connectStr, anonSupport, language} summaries.
 *
 * @remarks
 * Result is an array in config order; id/connectStr/anonSupport/language are null-preserving (stored null stays null), DFUNC_DESC is not included. A config without G2_CONFIG.CFG_DFUNC yields [] (no MISSING_SECTION).
 *
 * Wire name: `list_distinct_functions` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listDistinctFunctions(config: string): string {
  return rt.callJson(listDistinctFunctionsSpec, config);
}

/**
 * Arguments of {@link setDistinctFunction}.
 */
export interface SetDistinctFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves CONNECT_STR; null writes null; a string (including "") is written.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string | null;
  /**
   * Absent leaves DFUNC_DESC; a string is written verbatim.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent leaves LANGUAGE; a string is written verbatim.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
  /**
   * Absent leaves ANON_SUPPORT. TRAP: unlike add_distinct_function the value is written VERBATIM, with no Yes/No validation or case normalization.
   *
   * Wire name: `anon_support`.
   */
  readonly anonSupport?: string;
}

const setDistinctFunctionSpec: rt.FnSpec = {
  name: "setDistinctFunction",
  wire: "set_distinct_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
    ["anonSupport", "anon_support", false],
  ],
};

/**
 * Update a distinct function's connect string, description, language or anon support.
 *
 * @remarks
 * Returns (modified config, the updated CFG_DFUNC row). The row is removed and re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without CFG_DFUNC is NOT_FOUND.
 *
 * Wire name: `set_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetDistinctFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link setDistinctFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setDistinctFunction(config: string, options: SetDistinctFunctionOptions): string {
  return rt.callConfig(setDistinctFunctionSpec, config, options);
}

const setDistinctFunctionResultSpec: rt.FnSpec = { ...setDistinctFunctionSpec, name: "setDistinctFunctionResult" };

/**
 * The record (row / ids) of {@link setDistinctFunction}: same options and operation, but returns the record instead of the configuration. Operation: Update a distinct function's connect string, description, language or anon support.
 *
 * @remarks
 * Returns (modified config, the updated CFG_DFUNC row). The row is removed and re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without CFG_DFUNC is NOT_FOUND.
 *
 * Wire name: `set_distinct_function` (group `functions_distinct`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetDistinctFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setDistinctFunctionResult(config: string, options: SetDistinctFunctionOptions): string {
  return rt.callJson(setDistinctFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link addExpressionFunction}.
 */
export interface AddExpressionFunctionOptions {
  /**
   * Uppercased before the duplicate check and storage (EFUNC_CODE).
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string;
  /**
   * Absent stores EFUNC_DESC null; any string is stored verbatim.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent stores LANGUAGE null; any string is stored verbatim.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
}

const addExpressionFunctionSpec: rt.FnSpec = {
  name: "addExpressionFunction",
  wire: "add_expression_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
  ],
};

/**
 * Add an expression function (CFG_EFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE, CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is absent.
 *
 * Wire name: `add_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addExpressionFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionFunction(config: string, options: AddExpressionFunctionOptions): string {
  return rt.callConfig(addExpressionFunctionSpec, config, options);
}

const addExpressionFunctionResultSpec: rt.FnSpec = { ...addExpressionFunctionSpec, name: "addExpressionFunctionResult" };

/**
 * The record (row / ids) of {@link addExpressionFunction}: same options and operation, but returns the record instead of the configuration. Operation: Add an expression function (CFG_EFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE, CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is absent.
 *
 * Wire name: `add_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddExpressionFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addExpressionFunctionResult(config: string, options: AddExpressionFunctionOptions): string {
  return rt.callJson(addExpressionFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteExpressionFunction}.
 */
export interface DeleteExpressionFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteExpressionFunctionSpec: rt.FnSpec = {
  name: "deleteExpressionFunction",
  wire: "delete_expression_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete an expression function's CFG_EFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row; CFG_EFCALL rows referencing it are left dangling (use delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteExpressionFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionFunction(config: string, options: DeleteExpressionFunctionOptions): string {
  return rt.callConfig(deleteExpressionFunctionSpec, config, options);
}

const deleteExpressionFunctionResultSpec: rt.FnSpec = { ...deleteExpressionFunctionSpec, name: "deleteExpressionFunctionResult" };

/**
 * The record (row / ids) of {@link deleteExpressionFunction}: same options and operation, but returns the record instead of the configuration. Operation: Delete an expression function's CFG_EFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row; CFG_EFCALL rows referencing it are left dangling (use delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionFunctionResult(config: string, options: DeleteExpressionFunctionOptions): string {
  return rt.callJson(deleteExpressionFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteExpressionFunctionCascade}.
 */
export interface DeleteExpressionFunctionCascadeOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteExpressionFunctionCascadeSpec: rt.FnSpec = {
  name: "deleteExpressionFunctionCascade",
  wire: "delete_expression_function_cascade",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_expression_function_cascade` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionFunctionCascadeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteExpressionFunctionCascadeResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionFunctionCascade(config: string, options: DeleteExpressionFunctionCascadeOptions): string {
  return rt.callConfig(deleteExpressionFunctionCascadeSpec, config, options);
}

const deleteExpressionFunctionCascadeResultSpec: rt.FnSpec = { ...deleteExpressionFunctionCascadeSpec, name: "deleteExpressionFunctionCascadeResult" };

/**
 * The record (row / ids) of {@link deleteExpressionFunctionCascade}: same options and operation, but returns the record instead of the configuration. Operation: Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_expression_function_cascade` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteExpressionFunctionCascadeOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteExpressionFunctionCascadeResult(config: string, options: DeleteExpressionFunctionCascadeOptions): string {
  return rt.callJson(deleteExpressionFunctionCascadeResultSpec, config, options);
}

/**
 * Arguments of {@link getExpressionFunction}.
 */
export interface GetExpressionFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getExpressionFunctionSpec: rt.FnSpec = {
  name: "getExpressionFunction",
  wire: "get_expression_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one expression function's raw CFG_EFUNC row by code.
 *
 * @remarks
 * Result uses on-disk keys (EFUNC_ID, EFUNC_CODE, EFUNC_DESC, CONNECT_STR, LANGUAGE). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `get_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetExpressionFunctionOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getExpressionFunction(config: string, options: GetExpressionFunctionOptions): string {
  return rt.callJson(getExpressionFunctionSpec, config, options);
}

const listExpressionFunctionsSpec: rt.FnSpec = {
  name: "listExpressionFunctions",
  wire: "list_expression_functions",
  args: [],
};

/**
 * List all expression functions as camelCase summaries.
 *
 * @remarks
 * Result is an array of {id, function, connectStr, language} in config order (description is NOT included); connectStr/language are null-preserving. A missing CFG_EFUNC section yields [] (no error).
 *
 * Wire name: `list_expression_functions` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listExpressionFunctions(config: string): string {
  return rt.callJson(listExpressionFunctionsSpec, config);
}

/**
 * Arguments of {@link setExpressionFunction}.
 */
export interface SetExpressionFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string | null;
  /**
   * Absent leaves EFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
}

const setExpressionFunctionSpec: rt.FnSpec = {
  name: "setExpressionFunction",
  wire: "set_expression_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
  ],
};

/**
 * Update an expression function's connect string / description / language.
 *
 * @remarks
 * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC section is NOT_FOUND.
 *
 * Wire name: `set_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetExpressionFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link setExpressionFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setExpressionFunction(config: string, options: SetExpressionFunctionOptions): string {
  return rt.callConfig(setExpressionFunctionSpec, config, options);
}

const setExpressionFunctionResultSpec: rt.FnSpec = { ...setExpressionFunctionSpec, name: "setExpressionFunctionResult" };

/**
 * The record (row / ids) of {@link setExpressionFunction}: same options and operation, but returns the record instead of the configuration. Operation: Update an expression function's connect string / description / language.
 *
 * @remarks
 * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC section is NOT_FOUND.
 *
 * Wire name: `set_expression_function` (group `functions_expression`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetExpressionFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setExpressionFunctionResult(config: string, options: SetExpressionFunctionOptions): string {
  return rt.callJson(setExpressionFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link addStandardizeFunction}.
 */
export interface AddStandardizeFunctionOptions {
  /**
   * Uppercased before the duplicate check and storage (SFUNC_CODE).
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string;
  /**
   * Absent stores SFUNC_DESC null; any string is stored verbatim.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent stores LANGUAGE null; any string is stored verbatim.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
}

const addStandardizeFunctionSpec: rt.FnSpec = {
  name: "addStandardizeFunction",
  wire: "add_standardize_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
  ],
};

/**
 * Add a standardize function (CFG_SFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE, CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is absent.
 *
 * Wire name: `add_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addStandardizeFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeFunction(config: string, options: AddStandardizeFunctionOptions): string {
  return rt.callConfig(addStandardizeFunctionSpec, config, options);
}

const addStandardizeFunctionResultSpec: rt.FnSpec = { ...addStandardizeFunctionSpec, name: "addStandardizeFunctionResult" };

/**
 * The record (row / ids) of {@link addStandardizeFunction}: same options and operation, but returns the record instead of the configuration. Operation: Add a standardize function (CFG_SFUNC row).
 *
 * @remarks
 * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE, CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is absent.
 *
 * Wire name: `add_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddStandardizeFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addStandardizeFunctionResult(config: string, options: AddStandardizeFunctionOptions): string {
  return rt.callJson(addStandardizeFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteStandardizeFunction}.
 */
export interface DeleteStandardizeFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteStandardizeFunctionSpec: rt.FnSpec = {
  name: "deleteStandardizeFunction",
  wire: "delete_standardize_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a standardize function's CFG_SFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row; CFG_SFCALL rows referencing it are left dangling (use delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteStandardizeFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeFunction(config: string, options: DeleteStandardizeFunctionOptions): string {
  return rt.callConfig(deleteStandardizeFunctionSpec, config, options);
}

const deleteStandardizeFunctionResultSpec: rt.FnSpec = { ...deleteStandardizeFunctionSpec, name: "deleteStandardizeFunctionResult" };

/**
 * The record (row / ids) of {@link deleteStandardizeFunction}: same options and operation, but returns the record instead of the configuration. Operation: Delete a standardize function's CFG_SFUNC row only (no cascade).
 *
 * @remarks
 * Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row; CFG_SFCALL rows referencing it are left dangling (use delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeFunctionResult(config: string, options: DeleteStandardizeFunctionOptions): string {
  return rt.callJson(deleteStandardizeFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link deleteStandardizeFunctionCascade}.
 */
export interface DeleteStandardizeFunctionCascadeOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteStandardizeFunctionCascadeSpec: rt.FnSpec = {
  name: "deleteStandardizeFunctionCascade",
  wire: "delete_standardize_function_cascade",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a standardize function and its CFG_SFCALL rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_standardize_function_cascade` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeFunctionCascadeOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link deleteStandardizeFunctionCascadeResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeFunctionCascade(config: string, options: DeleteStandardizeFunctionCascadeOptions): string {
  return rt.callConfig(deleteStandardizeFunctionCascadeSpec, config, options);
}

const deleteStandardizeFunctionCascadeResultSpec: rt.FnSpec = { ...deleteStandardizeFunctionCascadeSpec, name: "deleteStandardizeFunctionCascadeResult" };

/**
 * The record (row / ids) of {@link deleteStandardizeFunctionCascade}: same options and operation, but returns the record instead of the configuration. Operation: Delete a standardize function and its CFG_SFCALL rows.
 *
 * @remarks
 * Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
 *
 * Wire name: `delete_standardize_function_cascade` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteStandardizeFunctionCascadeOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteStandardizeFunctionCascadeResult(config: string, options: DeleteStandardizeFunctionCascadeOptions): string {
  return rt.callJson(deleteStandardizeFunctionCascadeResultSpec, config, options);
}

/**
 * Arguments of {@link getStandardizeFunction}.
 */
export interface GetStandardizeFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getStandardizeFunctionSpec: rt.FnSpec = {
  name: "getStandardizeFunction",
  wire: "get_standardize_function",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one standardize function's raw CFG_SFUNC row by code.
 *
 * @remarks
 * Result uses on-disk keys (SFUNC_ID, SFUNC_CODE, SFUNC_DESC, CONNECT_STR, LANGUAGE). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `get_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetStandardizeFunctionOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getStandardizeFunction(config: string, options: GetStandardizeFunctionOptions): string {
  return rt.callJson(getStandardizeFunctionSpec, config, options);
}

const listStandardizeFunctionsSpec: rt.FnSpec = {
  name: "listStandardizeFunctions",
  wire: "list_standardize_functions",
  args: [],
};

/**
 * List all standardize functions as camelCase summaries.
 *
 * @remarks
 * Result is an array of {id, function, connectStr, language} in config order (description is NOT included); connectStr/language are null-preserving. A missing CFG_SFUNC section yields [] (no error).
 *
 * Wire name: `list_standardize_functions` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listStandardizeFunctions(config: string): string {
  return rt.callJson(listStandardizeFunctionsSpec, config);
}

/**
 * Arguments of {@link setStandardizeFunction}.
 */
export interface SetStandardizeFunctionOptions {
  /**
   * Uppercased before lookup.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `connect_str`.
   */
  readonly connectStr?: string | null;
  /**
   * Absent leaves SFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
   *
   * Wire name: `language`.
   */
  readonly language?: string;
}

const setStandardizeFunctionSpec: rt.FnSpec = {
  name: "setStandardizeFunction",
  wire: "set_standardize_function",
  args: [
    ["code", "code", true],
    ["connectStr", "connect_str", false],
    ["description", "description", false],
    ["language", "language", false],
  ],
};

/**
 * Update a standardize function's connect string / description / language.
 *
 * @remarks
 * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC section is NOT_FOUND.
 *
 * Wire name: `set_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetStandardizeFunctionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link setStandardizeFunctionResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setStandardizeFunction(config: string, options: SetStandardizeFunctionOptions): string {
  return rt.callConfig(setStandardizeFunctionSpec, config, options);
}

const setStandardizeFunctionResultSpec: rt.FnSpec = { ...setStandardizeFunctionSpec, name: "setStandardizeFunctionResult" };

/**
 * The record (row / ids) of {@link setStandardizeFunction}: same options and operation, but returns the record instead of the configuration. Operation: Update a standardize function's connect string / description / language.
 *
 * @remarks
 * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC section is NOT_FOUND.
 *
 * Wire name: `set_standardize_function` (group `functions_standardize`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetStandardizeFunctionOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setStandardizeFunctionResult(config: string, options: SetStandardizeFunctionOptions): string {
  return rt.callJson(setStandardizeFunctionResultSpec, config, options);
}

/**
 * Arguments of {@link cloneGenericPlan}.
 */
export interface CloneGenericPlanOptions {
  /**
   * Uppercased, then matched exactly against GPLAN_CODE; unknown = NOT_FOUND.
   *
   * Wire name: `source_gplan_code`.
   */
  readonly sourceGplanCode: string;
  /**
   * Uppercased before the duplicate check and storage; an existing code = ALREADY_EXISTS.
   *
   * Wire name: `new_gplan_code`.
   */
  readonly newGplanCode: string;
  /**
   * Stored verbatim in GPLAN_DESC; absent = the uppercased new code.
   *
   * Wire name: `new_gplan_desc`.
   */
  readonly newGplanDesc?: string;
}

const cloneGenericPlanSpec: rt.FnSpec = {
  name: "cloneGenericPlan",
  wire: "clone_generic_plan",
  args: [
    ["sourceGplanCode", "source_gplan_code", true],
    ["newGplanCode", "new_gplan_code", true],
    ["newGplanDesc", "new_gplan_desc", false],
  ],
};

/**
 * Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
 *
 * @remarks
 * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's GPLAN_ID is not an integer.
 *
 * Wire name: `clone_generic_plan` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link CloneGenericPlanOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link cloneGenericPlanResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function cloneGenericPlan(config: string, options: CloneGenericPlanOptions): string {
  return rt.callConfig(cloneGenericPlanSpec, config, options);
}

const cloneGenericPlanResultSpec: rt.FnSpec = { ...cloneGenericPlanSpec, name: "cloneGenericPlanResult" };

/**
 * The record (row / ids) of {@link cloneGenericPlan}: same options and operation, but returns the record instead of the configuration. Operation: Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
 *
 * @remarks
 * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's GPLAN_ID is not an integer.
 *
 * Wire name: `clone_generic_plan` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link CloneGenericPlanOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function cloneGenericPlanResult(config: string, options: CloneGenericPlanOptions): string {
  return rt.callJson(cloneGenericPlanResultSpec, config, options);
}

/**
 * Arguments of {@link deleteGenericPlan}.
 */
export interface DeleteGenericPlanOptions {
  /**
   * Uppercased, then matched exactly against GPLAN_CODE; unknown = NOT_FOUND.
   *
   * Wire name: `gplan_code`.
   */
  readonly gplanCode: string;
}

const deleteGenericPlanSpec: rt.FnSpec = {
  name: "deleteGenericPlan",
  wire: "delete_generic_plan",
  args: [
    ["gplanCode", "gplan_code", true],
  ],
};

/**
 * Delete a generic plan and all of its generic thresholds.
 *
 * @remarks
 * System plans (GPLAN_ID <= 2, i.e. INGEST and SEARCH in the template) are protected: INVALID_INPUT. Removes the CFG_GPLAN row and every CFG_GENERIC_THRESHOLD row with that GPLAN_ID. An absent CFG_GPLAN section is NOT_FOUND (not MISSING_SECTION).
 *
 * Wire name: `delete_generic_plan` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteGenericPlanOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT, INVALID_CONFIG; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteGenericPlan(config: string, options: DeleteGenericPlanOptions): string {
  return rt.callConfig(deleteGenericPlanSpec, config, options);
}

/**
 * Arguments of {@link listGenericPlans}.
 */
export interface ListGenericPlansOptions {
  /**
   * Case-insensitive SUBSTRING match against the raw row serialized as JSON text — keys and numbers included (so e.g. "gplan" matches every row). Absent = no filtering.
   *
   * Wire name: `filter`.
   */
  readonly filter?: string;
}

const listGenericPlansSpec: rt.FnSpec = {
  name: "listGenericPlans",
  wire: "list_generic_plans",
  args: [
    ["filter", "filter", false],
  ],
};

/**
 * List generic plans as {id, plan, description}, optionally filtered.
 *
 * @remarks
 * Array of {id (GPLAN_ID), plan (GPLAN_CODE), description (GPLAN_DESC)} sorted by id; missing values become 0 / "". An absent CFG_GPLAN (or G2_CONFIG) yields an empty array, never MISSING_SECTION.
 *
 * Wire name: `list_generic_plans` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link ListGenericPlansOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listGenericPlans(config: string, options: ListGenericPlansOptions = {}): string {
  return rt.callJson(listGenericPlansSpec, config, options);
}

/**
 * Arguments of {@link setGenericPlan}.
 */
export interface SetGenericPlanOptions {
  /**
   * Uppercased, then matched exactly against GPLAN_CODE.
   *
   * Wire name: `gplan_code`.
   */
  readonly gplanCode: string;
  /**
   * Written verbatim to GPLAN_DESC.
   *
   * Wire name: `gplan_desc`.
   */
  readonly gplanDesc: string;
}

/**
 * Named result of {@link setGenericPlanResult}.
 */
export interface SetGenericPlanRecord {
  /** JSON text of result member `plan_id`. */
  readonly planId: string;
  /** JSON text of result member `was_created`. */
  readonly wasCreated: string;
}

const setGenericPlanSpec: rt.FnSpec = {
  name: "setGenericPlan",
  wire: "set_generic_plan",
  args: [
    ["gplanCode", "gplan_code", true],
    ["gplanDesc", "gplan_desc", true],
  ],
};

/**
 * Create a generic plan, or update the description of an existing one (upsert).
 *
 * @remarks
 * Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this create path.
 *
 * Wire name: `set_generic_plan` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetGenericPlanOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link setGenericPlanResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setGenericPlan(config: string, options: SetGenericPlanOptions): string {
  return rt.callConfig(setGenericPlanSpec, config, options);
}

const setGenericPlanResultSpec: rt.FnSpec = { ...setGenericPlanSpec, name: "setGenericPlanResult" };

/**
 * The record (row / ids) of {@link setGenericPlan}: same options and operation, but returns the record instead of the configuration. Operation: Create a generic plan, or update the description of an existing one (upsert).
 *
 * @remarks
 * Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this create path.
 *
 * Wire name: `set_generic_plan` (group `generic_plans`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetGenericPlanOptions}.
 * @returns A record of the named result values (`planId`, `wasCreated`); each value is JSON text.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setGenericPlanResult(config: string, options: SetGenericPlanOptions): SetGenericPlanRecord {
  return rt.callNamed<SetGenericPlanRecord>(setGenericPlanResultSpec, config, options, [["plan_id", "planId"], ["was_created", "wasCreated"]]);
}

/**
 * Arguments of {@link addRule}.
 */
export interface AddRuleOptions {
  /**
   * Requested ERRULE_ID. 0 or any negative value means auto-allocate (max existing + 1, floor 1000, so 1000 on the template). A taken id > 0 is ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
   *
   * Wire name: `id`.
   */
  readonly id: number | bigint;
  /**
   * Object with on-disk keys. ERRULE_CODE (string) is required, else MISSING_FIELD; uppercased for storage and the case-insensitive duplicate check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required: absent/non-string is MISSING_FIELD, "" or an unknown code is NOT_FOUND (existence is case-insensitive). DISQ_ERFRAG_CODE is optional: "" is accepted and stored as "", an unknown code is NOT_FOUND. TRAP: both fragment codes are stored VERBATIM (not uppercased). RESOLVE / RELATE default "No", must be Yes/No case-insensitively (stored title-case) else INVALID_INPUT, and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires RTYPE_ID in 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER defaults to null. A non-string / non-integer value for any of these keys is treated as absent.
   *
   * Shape: `{ERRULE_CODE: string, QUAL_ERFRAG_CODE: string, DISQ_ERFRAG_CODE?: string, RESOLVE?: string, RELATE?: string, RTYPE_ID?: int, ERRULE_TIER?: int, ERRULE_ID?: int}`.
   *
   * Wire name: `rule_config`.
   */
  readonly ruleConfig: { readonly ERRULE_CODE: string; readonly QUAL_ERFRAG_CODE: string; readonly DISQ_ERFRAG_CODE?: string; readonly RESOLVE?: string; readonly RELATE?: string; readonly RTYPE_ID?: number | bigint; readonly ERRULE_TIER?: number | bigint; readonly ERRULE_ID?: number | bigint };
}

const addRuleSpec: rt.FnSpec = {
  name: "addRule",
  wire: "add_rule",
  args: [
    ["id", "id", true],
    ["ruleConfig", "rule_config", true, { object: { "ERRULE_CODE": "string", "QUAL_ERFRAG_CODE": "string", "DISQ_ERFRAG_CODE?": "string", "RESOLVE?": "string", "RELATE?": "string", "RTYPE_ID?": "int", "ERRULE_TIER?": "int", "ERRULE_ID?": "int" } }],
  ],
};

/**
 * Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
 *
 * @remarks
 * Record is the assigned ERRULE_ID (integer). The written row always carries every CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order: fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
 *
 * Wire name: `add_rule` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddRuleOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact). {@link addRuleResult} (same options) returns the record this operation produces.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addRule(config: string, options: AddRuleOptions): string {
  return rt.callConfig(addRuleSpec, config, options);
}

const addRuleResultSpec: rt.FnSpec = { ...addRuleSpec, name: "addRuleResult" };

/**
 * The record (row / ids) of {@link addRule}: same options and operation, but returns the record instead of the configuration. Operation: Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
 *
 * @remarks
 * Record is the assigned ERRULE_ID (integer). The written row always carries every CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order: fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
 *
 * Wire name: `add_rule` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddRuleOptions}.
 * @returns The record (e.g. the created row or ids) as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addRuleResult(config: string, options: AddRuleOptions): string {
  return rt.callJson(addRuleResultSpec, config, options);
}

/**
 * Arguments of {@link deleteRule}.
 */
export interface DeleteRuleOptions {
  /**
   * Uppercased, then matched EXACTLY against ERRULE_CODE (a stored lowercase code cannot be deleted). Not an id.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const deleteRuleSpec: rt.FnSpec = {
  name: "deleteRule",
  wire: "delete_rule",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Delete a rule by code.
 *
 * @remarks
 * No dependency or system-rule protection; any rule can be deleted. A config without CFG_ERRULE is NOT_FOUND.
 *
 * Wire name: `delete_rule` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteRuleOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteRule(config: string, options: DeleteRuleOptions): string {
  return rt.callConfig(deleteRuleSpec, config, options);
}

/**
 * Arguments of {@link getRule}.
 */
export interface GetRuleOptions {
  /**
   * Uppercased, then matched exactly against ERRULE_CODE first, then numerically against ERRULE_ID (e.g. "100").
   *
   * Wire name: `code_or_id`.
   */
  readonly codeOrId: string;
}

const getRuleSpec: rt.FnSpec = {
  name: "getRule",
  wire: "get_rule",
  args: [
    ["codeOrId", "code_or_id", true],
  ],
};

/**
 * Get one rule, by code or ERRULE_ID, as a summary record.
 *
 * @remarks
 * Result is {id, rule, resolve, relate, rtype_id, fragment, disqualifier, tier} projected from ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE (null-preserving). TRAP: `tier` is the stored ERRULE_TIER only when RESOLVE is exactly "Yes", otherwise null.
 *
 * Wire name: `get_rule` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetRuleOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getRule(config: string, options: GetRuleOptions): string {
  return rt.callJson(getRuleSpec, config, options);
}

const listRulesSpec: rt.FnSpec = {
  name: "listRules",
  wire: "list_rules",
  args: [],
};

/**
 * List all rules as summary records, sorted by ERRULE_ID.
 *
 * @remarks
 * Result is an array of the get_rule record shape, sorted by id ascending (null/absent id sorts as 0). A missing CFG_ERRULE or G2_CONFIG yields [].
 *
 * Wire name: `list_rules` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listRules(config: string): string {
  return rt.callJson(listRulesSpec, config);
}

/**
 * Arguments of {@link setRule}.
 */
export interface SetRuleOptions {
  /**
   * Rule code; uppercased, then matched exactly. Unknown is NOT_FOUND.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * Absent keeps the stored RESOLVE. Must be Yes/No case-insensitively (stored title-case), else INVALID_INPUT.
   *
   * Wire name: `resolve`.
   */
  readonly resolve?: string;
  /**
   * Absent keeps the stored RELATE. Must be Yes/No case-insensitively, else INVALID_INPUT.
   *
   * Wire name: `relate`.
   */
  readonly relate?: string;
  /**
   * Absent keeps the stored RTYPE_ID. Forced to 1 when the merged RESOLVE is Yes; must be 2, 3 or 4 when RELATE is Yes.
   *
   * Wire name: `rtype_id`.
   */
  readonly rtypeId?: number | bigint;
  /**
   * QUAL_ERFRAG_CODE. Absent = keep (never re-validated); null = clear to null (TRAP: allowed here although add_rule requires a fragment); a string must name an existing fragment (case-insensitive; "" is NOT_FOUND) and is stored UPPERCASED (unlike add_rule).
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `fragment`.
   */
  readonly fragment?: string | null;
  /**
   * DISQ_ERFRAG_CODE. Absent = keep; null = clear to null; "" is accepted and stored ""; another string must name an existing fragment (NOT_FOUND) and is stored uppercased.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `disqualifier`.
   */
  readonly disqualifier?: string | null;
  /**
   * ERRULE_TIER. Absent = keep; null = clear; value = set. The merged rule with RESOLVE=Yes must have a non-zero tier, else INVALID_INPUT.
   *
   * Tri-state: omit (`undefined`) = leave unchanged, `null` = clear, a value = set.
   *
   * Wire name: `tier`.
   */
  readonly tier?: number | bigint | null;
}

const setRuleSpec: rt.FnSpec = {
  name: "setRule",
  wire: "set_rule",
  args: [
    ["code", "code", true],
    ["resolve", "resolve", false],
    ["relate", "relate", false],
    ["rtypeId", "rtype_id", false],
    ["fragment", "fragment", false],
    ["disqualifier", "disqualifier", false],
    ["tier", "tier", false],
  ],
};

/**
 * Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.
 *
 * @remarks
 * Merges the update into the stored row and re-applies add_rule's RESOLVE/RELATE/exclusivity/tier/RTYPE_ID rules to the merged row (no duplicate-code check); the row is rewritten with every CFG_ERRULE key and ERRULE_ID preserved.
 *
 * Wire name: `set_rule` (group `rules`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetRuleOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setRule(config: string, options: SetRuleOptions): string {
  return rt.callConfig(setRuleSpec, config, options);
}

/**
 * Arguments of {@link addSearchProfile}.
 */
export interface AddSearchProfileOptions {
  /**
   * SPROFILE_CODE; trimmed and uppercased. Empty after trimming is INVALID_INPUT; an existing code (case-insensitive) is ALREADY_EXISTS.
   *
   * Wire name: `code`.
   */
  readonly code: string;
  /**
   * GPLAN_CODE, matched case-insensitively against CFG_GPLAN (NOT_FOUND); stored as GPLAN_ID.
   *
   * Wire name: `generic_plan`.
   */
  readonly genericPlan: string;
  /**
   * DEFAULT_USED_FOR_CAND; trimmed, case-insensitive, normalized to Normal or Off. Absent or blank = Normal. Anything else is VALIDATION_ERRORS (field "candidates", OUT_OF_DOMAIN).
   *
   * Library default when omitted: `"Normal"` (applied by the library, not this binding).
   *
   * Wire name: `candidates`.
   */
  readonly candidates?: string;
  /**
   * SPROFILE_DESC, stored verbatim; absent = "".
   *
   * Library default when omitted: `""` (applied by the library, not this binding).
   *
   * Wire name: `description`.
   */
  readonly description?: string;
  /**
   * Feature candidate overrides: an array of {"feature": FTYPE_CODE, "flag": Yes|No|Y|N} objects (only those two keys, both strings, else INVALID_INPUT; a missing key = MISSING_FIELD); absent = none. Each feature is resolved case-insensitively against CFG_FTYPE (NOT_FOUND); a feature listed twice is VALIDATION_ERRORS (field "overrides", DUPLICATE); a flag other than Yes/Y/No/N (trimmed, case-insensitive) is VALIDATION_ERRORS (field "overrides", OUT_OF_DOMAIN). Stored in FTYPE_OVERRIDES as "[{<ftypeId>,<Y|N>},...]" sorted by FTYPE_ID, or "[]".
   *
   * Shape: `[{feature: string, flag: "Yes"|"No"|"Y"|"N"}]`.
   *
   * Wire name: `elements`.
   */
  readonly elements?: ReadonlyArray<{ readonly feature: string; readonly flag: "Yes" | "No" | "Y" | "N" }>;
}

const addSearchProfileSpec: rt.FnSpec = {
  name: "addSearchProfile",
  wire: "add_search_profile",
  args: [
    ["code", "code", true],
    ["genericPlan", "generic_plan", true],
    ["candidates", "candidates", false],
    ["description", "description", false],
    ["elements", "elements", false, { array: { object: { "feature": "string", "flag": { enum: ["Yes", "No", "Y", "N"] } } } }],
  ],
};

/**
 * Add a search profile (CFG_SPROFILE row) tying a generic plan and feature candidate overrides to a code.
 *
 * @remarks
 * SPROFILE_ID is always auto-allocated (max + 1, floor 1; 3 on the template, whose only profile is SEARCH = 2); no explicit id can be requested. CFG_SPROFILE is created when absent. Validation order: code, candidates, generic plan, overrides (per element: feature, duplicate, flag), duplicate code. A config without G2_CONFIG fails the generic-plan lookup (NOT_FOUND), so the library's MISSING_SECTION branch is unreachable; a non-array CFG_SPROFILE is INVALID_STRUCTURE.
 *
 * Wire name: `add_search_profile` (group `search_profiles`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddSearchProfileOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_INPUT, VALIDATION_ERRORS, NOT_FOUND, ALREADY_EXISTS, INVALID_STRUCTURE, MISSING_FIELD; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addSearchProfile(config: string, options: AddSearchProfileOptions): string {
  return rt.callConfig(addSearchProfileSpec, config, options);
}

/**
 * Arguments of {@link getSearchProfile}.
 */
export interface GetSearchProfileOptions {
  /**
   * Case-insensitive match against SPROFILE_CODE. Not an id.
   *
   * Wire name: `code`.
   */
  readonly code: string;
}

const getSearchProfileSpec: rt.FnSpec = {
  name: "getSearchProfile",
  wire: "get_search_profile",
  args: [
    ["code", "code", true],
  ],
};

/**
 * Get one search profile's raw CFG_SPROFILE row by code.
 *
 * @remarks
 * Result is the stored row (SPROFILE_ID, SPROFILE_CODE, SPROFILE_DESC, GPLAN_ID, DEFAULT_USED_FOR_CAND, FTYPE_OVERRIDES). A missing section is NOT_FOUND.
 *
 * Wire name: `get_search_profile` (group `search_profiles`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link GetSearchProfileOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getSearchProfile(config: string, options: GetSearchProfileOptions): string {
  return rt.callJson(getSearchProfileSpec, config, options);
}

/**
 * Arguments of {@link listSearchProfiles}.
 */
export interface ListSearchProfilesOptions {
  /**
   * Absent returns all. Otherwise keeps rows whose COMPACT raw-row JSON (serde_json::to_string, no spaces, on-disk keys and ids, e.g. "GPLAN_ID":2) contains the filter case-insensitively; it is applied to the raw row, not the projected record.
   *
   * Wire name: `filter`.
   */
  readonly filter?: string;
}

const listSearchProfilesSpec: rt.FnSpec = {
  name: "listSearchProfiles",
  wire: "list_search_profiles",
  args: [
    ["filter", "filter", false],
  ],
};

/**
 * List search profiles as display records with ids resolved to codes, sorted by id.
 *
 * @remarks
 * Result is an array of {id, profile, description, genericPlan, candidates, overrides: [{feature, flag: Yes|No}], overridesRaw} sorted by id. genericPlan / feature fall back to the numeric id as a string when unresolvable. A missing CFG_SPROFILE yields [].
 *
 * Wire name: `list_search_profiles` (group `search_profiles`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link ListSearchProfilesOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listSearchProfiles(config: string, options: ListSearchProfilesOptions = {}): string {
  return rt.callJson(listSearchProfilesSpec, config, options);
}

/**
 * Arguments of {@link deleteSearchProfile}.
 */
export interface DeleteSearchProfileOptions {
  /**
   * Matched case-insensitively against SPROFILE_CODE, or (when it parses as an integer after trimming) against SPROFILE_ID.
   *
   * Wire name: `search_value`.
   */
  readonly searchValue: string;
}

const deleteSearchProfileSpec: rt.FnSpec = {
  name: "deleteSearchProfile",
  wire: "delete_search_profile",
  args: [
    ["searchValue", "search_value", true],
  ],
};

/**
 * Delete a search profile by code or SPROFILE_ID.
 *
 * @remarks
 * The shipped profiles INGEST and SEARCH are protected: deleting one that exists is INVALID_INPUT. Existence is checked FIRST, so an absent reserved code (INGEST is not in the template) is NOT_FOUND.
 *
 * Wire name: `delete_search_profile` (group `search_profiles`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteSearchProfileOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteSearchProfile(config: string, options: DeleteSearchProfileOptions): string {
  return rt.callConfig(deleteSearchProfileSpec, config, options);
}

/**
 * Arguments of {@link setSetting}.
 */
export interface SetSettingOptions {
  /**
   * Uppercased; an existing setting of that name is overwritten silently.
   *
   * Wire name: `name`.
   */
  readonly name: string;
  /**
   * Stored VERBATIM as its typed JSON value (an integer stays an integer, a string a string); no validation.
   *
   * Wire name: `value`.
   */
  readonly value: rt.JsonValue;
}

const setSettingSpec: rt.FnSpec = {
  name: "setSetting",
  wire: "set_setting",
  args: [
    ["name", "name", true],
    ["value", "value", true],
  ],
};

/**
 * Create or overwrite a named setting in the G2_CONFIG.SETTINGS object.
 *
 * @remarks
 * SETTINGS is created when absent, and a non-object SETTINGS value (null, string, array) is REPLACED by a fresh object (prior content lost). A missing or non-object G2_CONFIG is MISSING_SECTION.
 *
 * Wire name: `set_setting` (group `settings`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetSettingOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setSetting(config: string, options: SetSettingOptions): string {
  return rt.callConfig(setSettingSpec, config, options);
}

const listSystemParametersSpec: rt.FnSpec = {
  name: "listSystemParameters",
  wire: "list_system_parameters",
  args: [],
};

/**
 * List system parameters as a name -> string-value map.
 *
 * @remarks
 * Result is an object. The only parameter is relationshipsBreakMatches, read from BREAK_RES of the FIRST CFG_RTYPE row with RCLASS_ID 2 and reported as a decimal STRING. TRAP: it is reported only when BREAK_RES is a JSON integer; the template's DISCLOSED row stores the string "No", so the template yields {}. A missing CFG_RTYPE/G2_CONFIG also yields {}.
 *
 * Wire name: `list_system_parameters` (group `system_params`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listSystemParameters(config: string): string {
  return rt.callJson(listSystemParametersSpec, config);
}

/**
 * Arguments of {@link setSystemParameter}.
 */
export interface SetSystemParameterOptions {
  /**
   * Case-insensitive; only relationshipsBreakMatches (or relationships_break_matches) is known. Any other name is INVALID_CONFIG (not INVALID_INPUT).
   *
   * Wire name: `parameter_name`.
   */
  readonly parameterName: string;
  /**
   * Written VERBATIM (any JSON value, no validation) to BREAK_RES of the first CFG_RTYPE row with RCLASS_ID 2. Only an integer value is visible to list_system_parameters afterwards.
   *
   * Wire name: `parameter_value`.
   */
  readonly parameterValue: rt.JsonValue;
}

const setSystemParameterSpec: rt.FnSpec = {
  name: "setSystemParameter",
  wire: "set_system_parameter",
  args: [
    ["parameterName", "parameter_name", true],
    ["parameterValue", "parameter_value", true],
  ],
};

/**
 * Set a system parameter (relationshipsBreakMatches).
 *
 * @remarks
 * NOT_FOUND when no CFG_RTYPE row has RCLASS_ID 2 (or CFG_RTYPE/G2_CONFIG is absent). The parameter name is checked AFTER the config is parsed.
 *
 * Wire name: `set_system_parameter` (group `system_params`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetSystemParameterOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, INVALID_CONFIG, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setSystemParameter(config: string, options: SetSystemParameterOptions): string {
  return rt.callConfig(setSystemParameterSpec, config, options);
}

/**
 * Arguments of {@link addComparisonThreshold}.
 */
export interface AddComparisonThresholdOptions {
  /**
   * REQUIRED by the library (absent = MISSING_FIELD). Comparison function code (CFG_CFUNC), matched case-insensitively; unknown = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `cfunc_code`.
   */
  readonly cfuncCode: string;
  /**
   * REQUIRED by the library (absent = MISSING_FIELD). Feature code matched case-insensitively (unknown = NOT_FOUND), or "all" (any case) for the all-features FTYPE_ID 0 sentinel.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode: string;
  /**
   * REQUIRED by the library (absent = MISSING_FIELD). Return value / score name; uppercased before storage and duplicate check.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `cfunc_rtnval`.
   */
  readonly cfuncRtnval: string;
  /**
   * Resolved in three steps: (1) if an all-features (FTYPE_ID 0) row already exists for this (cfunc, rtnval), its EXEC_ORDER is REUSED and this arg is ignored; (2) else a value > 0 is honoured, or ALREADY_EXISTS if taken within (CFUNC_ID, FTYPE_ID 0); (3) else (absent or <= 0) the next order within (CFUNC_ID, FTYPE_ID 0) is auto-allocated (max + 1). Never null.
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * Stored verbatim (no range check); absent stores SAME_SCORE null.
   *
   * Wire name: `same_score`.
   */
  readonly sameScore?: number | bigint;
  /**
   * Stored verbatim; absent stores CLOSE_SCORE null.
   *
   * Wire name: `close_score`.
   */
  readonly closeScore?: number | bigint;
  /**
   * Stored verbatim; absent stores LIKELY_SCORE null.
   *
   * Wire name: `likely_score`.
   */
  readonly likelyScore?: number | bigint;
  /**
   * Stored verbatim; absent stores PLAUSIBLE_SCORE null.
   *
   * Wire name: `plausible_score`.
   */
  readonly plausibleScore?: number | bigint;
  /**
   * Stored verbatim; absent stores UN_LIKELY_SCORE null.
   *
   * Wire name: `un_likely_score`.
   */
  readonly unLikelyScore?: number | bigint;
}

const addComparisonThresholdSpec: rt.FnSpec = {
  name: "addComparisonThreshold",
  wire: "add_comparison_threshold",
  args: [
    ["cfuncCode", "cfunc_code", true],
    ["ftypeCode", "ftype_code", true],
    ["cfuncRtnval", "cfunc_rtnval", true],
    ["execOrder", "exec_order", false],
    ["sameScore", "same_score", false],
    ["closeScore", "close_score", false],
    ["likelyScore", "likely_score", false],
    ["plausibleScore", "plausible_score", false],
    ["unLikelyScore", "un_likely_score", false],
  ],
};

/**
 * Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and return value.
 *
 * @remarks
 * CFRTN_ID is always auto-allocated (max existing + 1, no floor); there is no id arg. Duplicate key is (CFUNC_ID, FTYPE_ID, uppercased rtnval) = ALREADY_EXISTS. Order: missing fields, cfunc lookup, feature lookup, then CFG_CFRTN section, duplicate, exec order.
 *
 * Wire name: `add_comparison_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddComparisonThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, ALREADY_EXISTS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addComparisonThreshold(config: string, options: AddComparisonThresholdOptions): string {
  return rt.callConfig(addComparisonThresholdSpec, config, options);
}

/**
 * Arguments of {@link deleteComparisonThreshold}.
 */
export interface DeleteComparisonThresholdOptions {
  /**
   * Comparison function code, case-insensitive; unknown = NOT_FOUND.
   *
   * Wire name: `cfunc_code`.
   */
  readonly cfuncCode: string;
  /**
   * Feature code (case-insensitive) or "all" for FTYPE_ID 0; unknown = NOT_FOUND.
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode: string;
  /**
   * Return value, matched case-insensitively.
   *
   * Wire name: `cfunc_rtnval`.
   */
  readonly cfuncRtnval: string;
}

const deleteComparisonThresholdSpec: rt.FnSpec = {
  name: "deleteComparisonThreshold",
  wire: "delete_comparison_threshold",
  args: [
    ["cfuncCode", "cfunc_code", true],
    ["ftypeCode", "ftype_code", true],
    ["cfuncRtnval", "cfunc_rtnval", true],
  ],
};

/**
 * Delete a comparison threshold identified by (comparison function, feature, return value).
 *
 * @remarks
 * No matching row = NOT_FOUND. No tier/dependency protection: deleting the all-features tier row leaves per-feature rows in place.
 *
 * Wire name: `delete_comparison_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteComparisonThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteComparisonThreshold(config: string, options: DeleteComparisonThresholdOptions): string {
  return rt.callConfig(deleteComparisonThresholdSpec, config, options);
}

/**
 * Arguments of {@link setComparisonThreshold}.
 */
export interface SetComparisonThresholdOptions {
  /**
   * REQUIRED by the library (absent = MISSING_FIELD). Case-insensitive lookup; unknown = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `cfunc_code`.
   */
  readonly cfuncCode: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Feature code (case-insensitive) or "all" for FTYPE_ID 0.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `ftype_code`.
   */
  readonly ftypeCode: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Matched case-insensitively; no row = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `cfunc_rtnval`.
   */
  readonly cfuncRtnval: string;
  /**
   * Absent leaves EXEC_ORDER unchanged; a value is written VERBATIM (no uniqueness / tier check, any integer incl. <= 0).
   *
   * Wire name: `exec_order`.
   */
  readonly execOrder?: number | bigint;
  /**
   * Absent leaves unchanged; else written verbatim.
   *
   * Wire name: `same_score`.
   */
  readonly sameScore?: number | bigint;
  /**
   * Absent leaves unchanged; else written verbatim.
   *
   * Wire name: `close_score`.
   */
  readonly closeScore?: number | bigint;
  /**
   * Absent leaves unchanged; else written verbatim.
   *
   * Wire name: `likely_score`.
   */
  readonly likelyScore?: number | bigint;
  /**
   * Absent leaves unchanged; else written verbatim.
   *
   * Wire name: `plausible_score`.
   */
  readonly plausibleScore?: number | bigint;
  /**
   * Absent leaves unchanged; else written verbatim.
   *
   * Wire name: `un_likely_score`.
   */
  readonly unLikelyScore?: number | bigint;
}

const setComparisonThresholdSpec: rt.FnSpec = {
  name: "setComparisonThreshold",
  wire: "set_comparison_threshold",
  args: [
    ["cfuncCode", "cfunc_code", true],
    ["ftypeCode", "ftype_code", true],
    ["cfuncRtnval", "cfunc_rtnval", true],
    ["execOrder", "exec_order", false],
    ["sameScore", "same_score", false],
    ["closeScore", "close_score", false],
    ["likelyScore", "likely_score", false],
    ["plausibleScore", "plausible_score", false],
    ["unLikelyScore", "un_likely_score", false],
  ],
};

/**
 * Update the exec order and/or scores of an existing comparison threshold.
 *
 * @remarks
 * NOT tri-state: a score cannot be cleared back to null.
 *
 * Wire name: `set_comparison_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetComparisonThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setComparisonThreshold(config: string, options: SetComparisonThresholdOptions): string {
  return rt.callConfig(setComparisonThresholdSpec, config, options);
}

const listComparisonThresholdsSpec: rt.FnSpec = {
  name: "listComparisonThresholds",
  wire: "list_comparison_thresholds",
  args: [],
};

/**
 * List all comparison thresholds with resolved function and feature names.
 *
 * @remarks
 * Array of {id (CFRTN_ID), function (CFUNC_CODE or "unknown"), returnOrder (EXEC_ORDER), scoreName, feature ("all" for FTYPE_ID 0, else FTYPE_CODE or "unknown"), sameScore, closeScore, likelyScore, plausibleScore, unlikelyScore}, sorted by (CFUNC_ID, CFRTN_ID). TRAP: null/absent EXEC_ORDER and scores are reported as 0, not null. Requires CFG_CFRTN, CFG_CFUNC and CFG_FTYPE (else MISSING_SECTION).
 *
 * Wire name: `list_comparison_thresholds` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listComparisonThresholds(config: string): string {
  return rt.callJson(listComparisonThresholdsSpec, config);
}

/**
 * Arguments of {@link addGenericThreshold}.
 */
export interface AddGenericThresholdOptions {
  /**
   * REQUIRED (absent = MISSING_FIELD). Uppercased, then matched EXACTLY against GPLAN_CODE (so effectively case-insensitive for upper-case stored codes); unknown = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `plan`.
   */
  readonly plan: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Uppercased; must be a canonical behavior code (behavior_domain, e.g. NAME, F1, FM, A1) else VALIDATION_ERRORS (field "behavior", UnknownReferenceCode).
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `behavior`.
   */
  readonly behavior: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Stored verbatim (e.g. -1).
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `scoring_cap`.
   */
  readonly scoringCap: number | bigint;
  /**
   * REQUIRED (absent = MISSING_FIELD). Stored verbatim.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `candidate_cap`.
   */
  readonly candidateCap: number | bigint;
  /**
   * REQUIRED (absent = MISSING_FIELD). Case-insensitive Yes/No, stored canonical "Yes"/"No"; else VALIDATION_ERRORS (field "sendToRedo", OutOfDomain).
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `send_to_redo`.
   */
  readonly sendToRedo: string;
  /**
   * Absent or "all" (any case) = FTYPE_ID 0. Else uppercased and matched EXACTLY against FTYPE_CODE; unknown = NOT_FOUND.
   *
   * Library default when omitted: `"ALL"` (applied by the library, not this binding).
   *
   * Wire name: `feature`.
   */
  readonly feature?: string;
}

const addGenericThresholdSpec: rt.FnSpec = {
  name: "addGenericThreshold",
  wire: "add_generic_threshold",
  args: [
    ["plan", "plan", true],
    ["behavior", "behavior", true],
    ["scoringCap", "scoring_cap", true],
    ["candidateCap", "candidate_cap", true],
    ["sendToRedo", "send_to_redo", true],
    ["feature", "feature", false],
  ],
};

/**
 * Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and optional feature.
 *
 * @remarks
 * All absent required fields are reported in ONE MISSING_FIELD (order plan, behavior, scoring_cap, candidate_cap, send_to_redo), checked before the config is parsed. Then: plan lookup, feature lookup, duplicate (plan, behavior, feature) = ALREADY_EXISTS (Python treats it as a warning-success; the root library does not), then behavior + sendToRedo are validated TOGETHER into one VALIDATION_ERRORS (details schema sz-configtool.validation-errors/v1, order [behavior, sendToRedo]).
 *
 * Wire name: `add_generic_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link AddGenericThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS, VALIDATION_ERRORS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function addGenericThreshold(config: string, options: AddGenericThresholdOptions): string {
  return rt.callConfig(addGenericThresholdSpec, config, options);
}

/**
 * Arguments of {@link deleteGenericThreshold}.
 */
export interface DeleteGenericThresholdOptions {
  /**
   * REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `plan`.
   */
  readonly plan: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Uppercased and matched exactly; NOT validated against the behavior domain (no row = NOT_FOUND).
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `behavior`.
   */
  readonly behavior: string;
  /**
   * Absent or "all" = FTYPE_ID 0; else uppercased exact FTYPE_CODE match, unknown = NOT_FOUND.
   *
   * Library default when omitted: `"ALL"` (applied by the library, not this binding).
   *
   * Wire name: `feature`.
   */
  readonly feature?: string;
}

const deleteGenericThresholdSpec: rt.FnSpec = {
  name: "deleteGenericThreshold",
  wire: "delete_generic_threshold",
  args: [
    ["plan", "plan", true],
    ["behavior", "behavior", true],
    ["feature", "feature", false],
  ],
};

/**
 * Delete a generic threshold identified by (plan, behavior, feature).
 *
 * @remarks
 * No matching row = NOT_FOUND (also when CFG_GENERIC_THRESHOLD is absent). (Before the Unreleased fix it ignored `plan` and always deleted from INGEST.)
 *
 * Wire name: `delete_generic_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link DeleteGenericThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function deleteGenericThreshold(config: string, options: DeleteGenericThresholdOptions): string {
  return rt.callConfig(deleteGenericThresholdSpec, config, options);
}

/**
 * Arguments of {@link setGenericThreshold}.
 */
export interface SetGenericThresholdOptions {
  /**
   * REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `plan`.
   */
  readonly plan: string;
  /**
   * REQUIRED (absent = MISSING_FIELD). Lookup KEY only: uppercased and matched exactly, never validated against the domain (no row = NOT_FOUND).
   *
   * Required: the library rejects an absent value (MISSING_FIELD).
   *
   * Wire name: `behavior`.
   */
  readonly behavior: string;
  /**
   * Lookup KEY selecting the per-feature row (never written). Absent or "all" = FTYPE_ID 0; else case-insensitive feature lookup, unknown = NOT_FOUND.
   *
   * Library default when omitted: `"ALL"` (applied by the library, not this binding).
   *
   * Wire name: `feature`.
   */
  readonly feature?: string;
  /**
   * Absent leaves CANDIDATE_CAP unchanged; else written verbatim.
   *
   * Wire name: `candidate_cap`.
   */
  readonly candidateCap?: number | bigint;
  /**
   * Absent leaves SCORING_CAP unchanged; else written verbatim.
   *
   * Wire name: `scoring_cap`.
   */
  readonly scoringCap?: number | bigint;
  /**
   * Absent leaves unchanged; else case-insensitive Yes/No stored canonical, otherwise VALIDATION_ERRORS (field "sendToRedo", OutOfDomain) — checked AFTER the row lookup, so a missing row wins.
   *
   * Wire name: `send_to_redo`.
   */
  readonly sendToRedo?: string;
}

const setGenericThresholdSpec: rt.FnSpec = {
  name: "setGenericThreshold",
  wire: "set_generic_threshold",
  args: [
    ["plan", "plan", true],
    ["behavior", "behavior", true],
    ["feature", "feature", false],
    ["candidateCap", "candidate_cap", false],
    ["scoringCap", "scoring_cap", false],
    ["sendToRedo", "send_to_redo", false],
  ],
};

/**
 * Update the caps and/or send-to-redo flag of an existing generic threshold.
 *
 * Wire name: `set_generic_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link SetGenericThresholdOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, VALIDATION_ERRORS; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function setGenericThreshold(config: string, options: SetGenericThresholdOptions): string {
  return rt.callConfig(setGenericThresholdSpec, config, options);
}

const listGenericThresholdsSpec: rt.FnSpec = {
  name: "listGenericThresholds",
  wire: "list_generic_thresholds",
  args: [],
};

/**
 * List all generic thresholds with resolved plan and feature names.
 *
 * @remarks
 * Array of {id (GPLAN_ID), plan, behavior, feature ("all" for FTYPE_ID 0), candidateCap, scoringCap, sendToRedo}, sorted by (GPLAN_ID, canonical behavior position; unknown behaviors last), stable within ties. Requires CFG_GENERIC_THRESHOLD, CFG_GPLAN and CFG_FTYPE (else MISSING_SECTION).
 *
 * Wire name: `list_generic_thresholds` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function listGenericThresholds(config: string): string {
  return rt.callJson(listGenericThresholdsSpec, config);
}

/**
 * Arguments of {@link validateGenericThreshold}.
 */
export interface ValidateGenericThresholdOptions {
  /**
   * Uppercased, matched exactly against GPLAN_CODE; unknown = result notFound/plan (data, not an error).
   *
   * Wire name: `plan`.
   */
  readonly plan: string;
  /**
   * Uppercased; checked against the canonical behavior codes only in the last stage.
   *
   * Wire name: `behavior`.
   */
  readonly behavior: string;
  /**
   * Case-insensitive Yes/No; checked only in the last stage.
   *
   * Wire name: `send_to_redo`.
   */
  readonly sendToRedo: string;
  /**
   * Absent or "all" (any case) = FTYPE_ID 0. Else uppercased exact FTYPE_CODE match; unknown = result notFound/feature.
   *
   * Library default when omitted: `"ALL"` (applied by the library, not this binding).
   *
   * Wire name: `feature`.
   */
  readonly feature?: string;
}

const validateGenericThresholdSpec: rt.FnSpec = {
  name: "validateGenericThreshold",
  wire: "validate_generic_threshold",
  args: [
    ["plan", "plan", true],
    ["behavior", "behavior", true],
    ["sendToRedo", "send_to_redo", true],
    ["feature", "feature", false],
  ],
};

/**
 * Stage the checks of a generic-threshold add without mutating the config, returning the outcome as data.
 *
 * @remarks
 * Result is the versioned object (schema sz-configtool.generic-threshold-check/v1, root-library Serialize for GenericThresholdCheck): {"schema", "result": "ok" | "duplicate" | "notFound" (+ "which": "plan"|"feature", "value": uppercased code) | "invalid" (+ "failures": [{"field", "reasonCode", "offendingValue"}], order [behavior, sendToRedo])}. Stages stop at the first hit: plan, feature, duplicate (plan, behavior, feature), then field validation. Missing sections are NOT errors (a missing CFG_GPLAN is notFound/plan). Caps are not taken.
 *
 * Wire name: `validate_generic_threshold` (group `thresholds`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link ValidateGenericThresholdOptions}.
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function validateGenericThreshold(config: string, options: ValidateGenericThresholdOptions): string {
  return rt.callJson(validateGenericThresholdSpec, config, options);
}

const validateConfigSpec: rt.FnSpec = {
  name: "validateConfig",
  wire: "validate_config",
  args: [],
};

/**
 * Check that a document has the top-level shape of a config (structure only).
 *
 * @remarks
 * Checks, in order: valid JSON (JSON_PARSE); a G2_CONFIG key (MISSING_SECTION) that is an object (INVALID_STRUCTURE); every recognised CFG_* section (validation::EXPECTED_SECTIONS, 27 names) that is PRESENT is an array (INVALID_STRUCTURE). Absent sections, other keys and cross-references are not checked.
 *
 * Wire name: `validate_config` (group `validation`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns Nothing; throws on failure.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, MISSING_SECTION, INVALID_STRUCTURE; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function validateConfig(config: string): void {
  rt.callUnit(validateConfigSpec, config);
}

const getVersionSpec: rt.FnSpec = {
  name: "getVersion",
  wire: "get_version",
  args: [],
};

/**
 * Get the configuration VERSION string (G2_CONFIG.CONFIG_BASE_VERSION.VERSION).
 *
 * @remarks
 * Result is a JSON string (e.g. "4.4.0" in the template). Absent or non-string VERSION (or any missing parent) = NOT_FOUND.
 *
 * Wire name: `get_version` (group `versioning`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getVersion(config: string): string {
  return rt.callJson(getVersionSpec, config);
}

const getCompatibilityVersionSpec: rt.FnSpec = {
  name: "getCompatibilityVersion",
  wire: "get_compatibility_version",
  args: [],
};

/**
 * Get COMPATIBILITY_VERSION.CONFIG_VERSION.
 *
 * @remarks
 * Result is a JSON string (e.g. "11" in the template). Absent or non-string CONFIG_VERSION (or any missing parent) = NOT_FOUND.
 *
 * Wire name: `get_compatibility_version` (group `versioning`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @returns The result as JSON text (not parsed; use `JSON.parse`).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function getCompatibilityVersion(config: string): string {
  return rt.callJson(getCompatibilityVersionSpec, config);
}

/**
 * Arguments of {@link updateCompatibilityVersion}.
 */
export interface UpdateCompatibilityVersionOptions {
  /**
   * Stored verbatim as a JSON string; no format validation.
   *
   * Wire name: `new_version`.
   */
  readonly newVersion: string;
}

const updateCompatibilityVersionSpec: rt.FnSpec = {
  name: "updateCompatibilityVersion",
  wire: "update_compatibility_version",
  args: [
    ["newVersion", "new_version", true],
  ],
};

/**
 * Set COMPATIBILITY_VERSION.CONFIG_VERSION.
 *
 * @remarks
 * Absent CONFIG_BASE_VERSION or COMPATIBILITY_VERSION = NOT_FOUND; COMPATIBILITY_VERSION not an object = INVALID_CONFIG. TRAP: an absent G2_CONFIG is NOT an error — the config is returned unchanged (re-serialized).
 *
 * Wire name: `update_compatibility_version` (group `versioning`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link UpdateCompatibilityVersionOptions}.
 * @returns The modified configuration JSON text (opaque, byte-exact).
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND, INVALID_CONFIG; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function updateCompatibilityVersion(config: string, options: UpdateCompatibilityVersionOptions): string {
  return rt.callConfig(updateCompatibilityVersionSpec, config, options);
}

/**
 * Arguments of {@link verifyCompatibilityVersion}.
 */
export interface VerifyCompatibilityVersionOptions {
  /**
   * Compared by exact, case-sensitive string equality.
   *
   * Wire name: `expected_version`.
   */
  readonly expectedVersion: string;
}

/**
 * Named result of {@link verifyCompatibilityVersion}.
 */
export interface VerifyCompatibilityVersionRecord {
  /** JSON text of result member `current_version`. */
  readonly currentVersion: string;
  /** JSON text of result member `matches`. */
  readonly matches: string;
}

const verifyCompatibilityVersionSpec: rt.FnSpec = {
  name: "verifyCompatibilityVersion",
  wire: "verify_compatibility_version",
  args: [
    ["expectedVersion", "expected_version", true],
  ],
};

/**
 * Compare COMPATIBILITY_VERSION.CONFIG_VERSION with an expected value.
 *
 * @remarks
 * Rust returns (current_version, matches) where the String is NOT a config, so it is `json`; tuple_names makes the result the object {"current_version": "11", "matches": true}. Absent CONFIG_VERSION = NOT_FOUND.
 *
 * Wire name: `verify_compatibility_version` (group `versioning`).
 *
 * @param config - The configuration JSON text (opaque; never parsed by this binding).
 * @param options - Arguments; see {@link VerifyCompatibilityVersionOptions}.
 * @returns A record of the named result values (`currentVersion`, `matches`); each value is JSON text.
 * @throws {@link SzConfigToolError} `code` one of: JSON_PARSE, NOT_FOUND; plus the wire errors INVALID_INPUT, MISSING_FIELD, INTERNAL.
 */
export function verifyCompatibilityVersion(config: string, options: VerifyCompatibilityVersionOptions): VerifyCompatibilityVersionRecord {
  return rt.callNamed<VerifyCompatibilityVersionRecord>(verifyCompatibilityVersionSpec, config, options, [["current_version", "currentVersion"], ["matches", "matches"]]);
}

/** Wire names of every typed function above, in manifest order. */
export const TYPED_FUNCTION_NAMES: readonly string[] = [
  "add_attribute",
  "delete_attribute",
  "get_attribute",
  "list_attributes",
  "set_attribute",
  "add_behavior_override",
  "delete_behavior_override",
  "get_behavior_override",
  "list_behavior_overrides",
  "list_behavior_overrides_resolved",
  "add_comparison_call",
  "delete_comparison_call",
  "get_comparison_call",
  "list_comparison_calls",
  "add_comparison_call_element",
  "delete_comparison_call_element",
  "add_distinct_call",
  "delete_distinct_call",
  "get_distinct_call",
  "list_distinct_calls",
  "add_distinct_call_element",
  "delete_distinct_call_element",
  "add_expression_call",
  "delete_expression_call",
  "get_expression_call",
  "list_expression_calls",
  "add_expression_call_element",
  "delete_expression_call_element",
  "add_standardize_call",
  "delete_standardize_call",
  "get_standardize_call",
  "list_standardize_calls",
  "add_standardize_call_element",
  "delete_standardize_call_element",
  "add_config_section",
  "remove_config_section",
  "get_config_section",
  "config_section_is_empty",
  "list_config_sections",
  "remove_config_section_field",
  "add_config_section_field",
  "add_data_source",
  "delete_data_source",
  "get_data_source",
  "list_data_sources",
  "set_data_source",
  "add_element",
  "delete_element",
  "get_element",
  "list_elements",
  "set_element",
  "set_feature_element",
  "add_element_to_feature",
  "delete_element_from_feature",
  "render_config",
  "add_feature",
  "delete_feature",
  "get_feature",
  "list_features",
  "set_feature",
  "add_feature_comparison",
  "delete_feature_comparison",
  "get_feature_comparison",
  "list_feature_comparisons",
  "add_feature_distinct_call_element",
  "list_feature_classes",
  "get_feature_class",
  "update_feature_version",
  "add_fragment",
  "delete_fragment",
  "get_fragment",
  "list_fragments",
  "set_fragment",
  "add_comparison_function",
  "delete_comparison_function",
  "delete_comparison_function_cascade",
  "get_comparison_function",
  "list_comparison_functions",
  "set_comparison_function",
  "add_distinct_function",
  "delete_distinct_function",
  "get_distinct_function",
  "list_distinct_functions",
  "set_distinct_function",
  "add_expression_function",
  "delete_expression_function",
  "delete_expression_function_cascade",
  "get_expression_function",
  "list_expression_functions",
  "set_expression_function",
  "add_standardize_function",
  "delete_standardize_function",
  "delete_standardize_function_cascade",
  "get_standardize_function",
  "list_standardize_functions",
  "set_standardize_function",
  "clone_generic_plan",
  "delete_generic_plan",
  "list_generic_plans",
  "set_generic_plan",
  "add_rule",
  "delete_rule",
  "get_rule",
  "list_rules",
  "set_rule",
  "add_search_profile",
  "get_search_profile",
  "list_search_profiles",
  "delete_search_profile",
  "set_setting",
  "list_system_parameters",
  "set_system_parameter",
  "add_comparison_threshold",
  "delete_comparison_threshold",
  "set_comparison_threshold",
  "list_comparison_thresholds",
  "add_generic_threshold",
  "delete_generic_threshold",
  "set_generic_threshold",
  "list_generic_thresholds",
  "validate_generic_threshold",
  "validate_config",
  "get_version",
  "get_compatibility_version",
  "update_compatibility_version",
  "verify_compatibility_version",
];
