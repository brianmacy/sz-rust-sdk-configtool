// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

#ifndef SZCONFIGTOOL_GENERATED_API_HPP
#define SZCONFIGTOOL_GENERATED_API_HPP

#include <cstdint>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <variant>
#include <vector>
#include "szconfigtool/core.hpp"

namespace szconfigtool {

/// Optional arguments of AddAttribute(); omitted fields are not sent.
struct AddAttributeOptions {
    /// Wire arg `default_value` (str). Absent stores DEFAULT_VALUE null; any string (including "") is stored verbatim.
    std::optional<std::string> default_value{};
    /// Wire arg `internal` (str). Case-insensitive; normalized to Yes or No, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> internal{};
    /// Wire arg `required` (str). Case-insensitive; normalized to Yes, No, Any or Desired (stored in FELEM_REQ), else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> required{};
    /// Wire arg `id` (int). Requested ATTR_ID. Absent OR \<= 0 means auto-allocate (max existing + 1, floor 1000). A taken id \> 0 is ALREADY_EXISTS.
    std::optional<std::int64_t> id{};
};

/// @brief Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
///
/// Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param attribute Wire arg `attribute` (str). Uppercased before storage and duplicate check.
/// @param feature Wire arg `feature` (str). Must name an existing CFG_FTYPE (case-insensitive) or NOT_FOUND; stored uppercased in FTYPE_CODE.
/// @param element Wire arg `element` (str). Must name an existing CFG_FELEM (case-insensitive) or NOT_FOUND; stored uppercased in FELEM_CODE.
/// @param class_ Wire arg `class` (str). CASE-SENSITIVE (not uppercased): must be exactly one of NAME, ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
/// @param options Optional arguments (see AddAttributeOptions).
/// @return The modified configuration JSON. AddAttributeResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND.
[[nodiscard]] inline std::string AddAttribute(const std::string& config_json, std::string_view attribute, std::string_view feature, std::string_view element, std::string_view class_, const AddAttributeOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("attribute", attribute);
    sz_args.Str("feature", feature);
    sz_args.Str("element", element);
    sz_args.Str("class", class_);
    if (options.default_value) {
        sz_args.Str("default_value", *options.default_value);
    }
    if (options.internal) {
        sz_args.Str("internal", *options.internal);
    }
    if (options.required) {
        sz_args.Str("required", *options.required);
    }
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_attribute", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddAttribute(): same arguments and operation, but returns the record instead of the configuration. Operation: Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
///
/// Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param attribute Wire arg `attribute` (str). Uppercased before storage and duplicate check.
/// @param feature Wire arg `feature` (str). Must name an existing CFG_FTYPE (case-insensitive) or NOT_FOUND; stored uppercased in FTYPE_CODE.
/// @param element Wire arg `element` (str). Must name an existing CFG_FELEM (case-insensitive) or NOT_FOUND; stored uppercased in FELEM_CODE.
/// @param class_ Wire arg `class` (str). CASE-SENSITIVE (not uppercased): must be exactly one of NAME, ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
/// @param options Optional arguments (see AddAttributeOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND.
[[nodiscard]] inline std::string AddAttributeResult(const std::string& config_json, std::string_view attribute, std::string_view feature, std::string_view element, std::string_view class_, const AddAttributeOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("attribute", attribute);
    sz_args.Str("feature", feature);
    sz_args.Str("element", element);
    sz_args.Str("class", class_);
    if (options.default_value) {
        sz_args.Str("default_value", *options.default_value);
    }
    if (options.internal) {
        sz_args.Str("internal", *options.internal);
    }
    if (options.required) {
        sz_args.Str("required", *options.required);
    }
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_attribute", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete an attribute by code.
///
/// No dependency or system-attribute protection; any attribute can be deleted.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string DeleteAttribute(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_attribute", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one attribute's raw CFG_ATTR row by code.
///
/// Result uses on-disk keys (ATTR_ID, ATTR_CODE, ATTR_CLASS, FTYPE_CODE, FELEM_CODE, FELEM_REQ, DEFAULT_VALUE, INTERNAL).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string GetAttribute(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_attribute", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all attributes as camelCase summaries.
///
/// Result is an array of {id, attribute, class, feature, element, required, default, internal} in config order; feature/element/default may be null.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListAttributes(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_attributes", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetAttribute(); omitted fields are not sent.
struct SetAttributeOptions {
    /// Wire arg `internal` (str). Absent leaves INTERNAL unchanged; else case-insensitive, normalized to Yes or No, else INVALID_INPUT.
    std::optional<std::string> internal{};
    /// Wire arg `required` (str). Absent leaves FELEM_REQ unchanged; else normalized to Yes, No, Any or Desired, else INVALID_INPUT.
    std::optional<std::string> required{};
    /// Wire arg `default_value` (str). Absent leaves DEFAULT_VALUE unchanged; a string is stored verbatim. NOT tri-state: there is no way to clear DEFAULT_VALUE back to null.
    std::optional<std::string> default_value{};
};

/// @brief Update an attribute's internal / required / default value.
///
/// Validation is interleaved with mutation but the input config is never modified on error (a new config string is returned only on success).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param attribute Wire arg `attribute` (str). Attribute code; uppercased before lookup.
/// @param options Optional arguments (see SetAttributeOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string SetAttribute(const std::string& config_json, std::string_view attribute, const SetAttributeOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("attribute", attribute);
    if (options.internal) {
        sz_args.Str("internal", *options.internal);
    }
    if (options.required) {
        sz_args.Str("required", *options.required);
    }
    if (options.default_value) {
        sz_args.Str("default_value", *options.default_value);
    }
    auto sz_env = detail::Call("set_attribute", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Add a behavior override (CFG_FBOVR row) for a feature and usage type.
///
/// Validation order: feature, behavior, CFG_FBOVR present (MISSING_SECTION), duplicate (same FTYPE_ID + uppercased UTYPE_CODE, ALREADY_EXISTS). The row always carries FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Feature code, matched case-insensitively against CFG_FTYPE (NOT_FOUND); stored as its FTYPE_ID.
/// @param usage_type Wire arg `usage_type` (str). Uppercased; stored as UTYPE_CODE. Any string is accepted (no domain check).
/// @param behavior Wire arg `behavior` (str). Behavior code, case-insensitive: a frequency A1, F1, FF, FM, FVM (with optional E = exclusive and/or S = stable letters in any order) or the bare NAME / NONE; anything else is INVALID_INPUT. Split into FTYPE_FREQ, FTYPE_EXCL (Yes/No), FTYPE_STAB (Yes/No).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, MISSING_SECTION, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddBehaviorOverride(const std::string& config_json, std::string_view feature, std::string_view usage_type, std::string_view behavior) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    sz_args.Str("usage_type", usage_type);
    sz_args.Str("behavior", behavior);
    auto sz_env = detail::Call("add_behavior_override", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete the behavior override for a feature and usage type.
///
/// No override for the pair is NOT_FOUND; a config without CFG_FBOVR is MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Feature code, case-insensitive; unknown is NOT_FOUND.
/// @param usage_type Wire arg `usage_type` (str). Uppercased, then matched exactly against UTYPE_CODE (a stored lowercase UTYPE_CODE cannot be matched).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION.
[[nodiscard]] inline std::string DeleteBehaviorOverride(const std::string& config_json, std::string_view feature, std::string_view usage_type) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    sz_args.Str("usage_type", usage_type);
    auto sz_env = detail::Call("delete_behavior_override", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get the raw CFG_FBOVR row for a feature and usage type.
///
/// Result is the stored row with on-disk keys (FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Feature code, case-insensitive; unknown is NOT_FOUND.
/// @param usage_type Wire arg `usage_type` (str). Uppercased, then matched exactly against UTYPE_CODE.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION.
[[nodiscard]] inline std::string GetBehaviorOverride(const std::string& config_json, std::string_view feature, std::string_view usage_type) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    sz_args.Str("usage_type", usage_type);
    auto sz_env = detail::Call("get_behavior_override", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List the raw CFG_FBOVR rows sorted by FTYPE_ID.
///
/// Result is an array of stored rows (on-disk keys), stable-sorted by FTYPE_ID only (rows sharing a FTYPE_ID keep config order). A config without CFG_FBOVR is MISSING_SECTION (not []).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListBehaviorOverrides(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_behavior_overrides", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List behavior overrides as {feature, usageType, behavior} display records.
///
/// Result is an array of {feature, usageType, behavior} sorted by (FTYPE_ID, UTYPE_CODE). feature is the FTYPE_CODE resolved from FTYPE_ID, or the id as a string when no CFG_FTYPE row matches; behavior is FTYPE_FREQ plus E when FTYPE_EXCL and S when FTYPE_STAB is Y/YES/1 (case-insensitive). A missing G2_CONFIG or CFG_FBOVR is MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListBehaviorOverridesResolved(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_behavior_overrides_resolved", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddComparisonCall(); omitted fields are not sent.
struct AddComparisonCallOptions {
    /// Wire arg `id` (int). Requested CFCALL_ID. Absent OR \<= 0 means auto-allocate (max existing + 1, floor 1000). A taken id \> 0 is ALREADY_EXISTS (checked before any lookup).
    std::optional<std::int64_t> id{};
};

/// @brief Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with its element list (CFG_CFBOM rows).
///
/// Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array; ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function, empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to the feature is not checked.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_code Wire arg `ftype_code` (str). Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
/// @param cfunc_code Wire arg `cfunc_code` (str). Comparison function code; case-insensitive lookup in CFG_CFUNC, else NOT_FOUND.
/// @param element_list Wire arg `element_list` (str_list). Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position (outside the exec-order allocation policy). Duplicate items are not rejected.
/// @param options Optional arguments (see AddComparisonCallOptions).
/// @return The modified configuration JSON. AddComparisonCallResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT.
[[nodiscard]] inline std::string AddComparisonCall(const std::string& config_json, std::string_view ftype_code, std::string_view cfunc_code, const std::vector<std::string>& element_list, const AddComparisonCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("cfunc_code", cfunc_code);
    sz_args.StrList("element_list", element_list);
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_comparison_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddComparisonCall(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with its element list (CFG_CFBOM rows).
///
/// Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array; ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function, empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to the feature is not checked.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_code Wire arg `ftype_code` (str). Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
/// @param cfunc_code Wire arg `cfunc_code` (str). Comparison function code; case-insensitive lookup in CFG_CFUNC, else NOT_FOUND.
/// @param element_list Wire arg `element_list` (str_list). Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position (outside the exec-order allocation policy). Duplicate items are not rejected.
/// @param options Optional arguments (see AddComparisonCallOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT.
[[nodiscard]] inline std::string AddComparisonCallResult(const std::string& config_json, std::string_view ftype_code, std::string_view cfunc_code, const std::vector<std::string>& element_list, const AddComparisonCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("cfunc_code", cfunc_code);
    sz_args.StrList("element_list", element_list);
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_comparison_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a comparison call by CFCALL_ID, cascading to its CFG_CFBOM rows.
///
/// Removes the CFG_CFCALL row and every CFG_CFBOM row with that CFCALL_ID. No dependency or system-call protection: template calls (ids \< 1000) can be deleted.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfcall_id Wire arg `cfcall_id` (int). CFCALL_ID; must exist in CFG_CFCALL else NOT_FOUND (an absent section is also NOT_FOUND).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteComparisonCall(const std::string& config_json, std::int64_t cfcall_id) {
    detail::ArgsWriter sz_args;
    sz_args.Int("cfcall_id", cfcall_id);
    auto sz_env = detail::Call("delete_comparison_call", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
///
/// Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it does not include the CFBOM elements (codes: list_comparison_calls; raw rows: get_config_section("CFG_CFBOM")).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by CFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_CFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetComparisonCall(const std::string& config_json, std::int64_t call) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    auto sz_env = detail::Call("get_comparison_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
///
/// Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it does not include the CFBOM elements (codes: list_comparison_calls; raw rows: get_config_section("CFG_CFBOM")).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by CFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_CFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetComparisonCall(const std::string& config_json, std::string_view call) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    auto sz_env = detail::Call("get_comparison_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all comparison calls with feature/function codes resolved and their ordered element lists.
///
/// Result is an array of {id, feature, function, elementList} sorted by (FTYPE_ID, CFCALL_ID) — not config order. elementList is the call's CFG_CFBOM element codes ordered by EXEC_ORDER. Unresolvable ids render as the string "unknown". Missing sections are treated as empty (never MISSING_SECTION). LIMITATION: elementList omits the stored CFG_CFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows with get_config_section("CFG_CFBOM").
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListComparisonCalls(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_comparison_calls", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddComparisonCallElement(); omitted fields are not sent.
struct AddComparisonCallElementOptions {
    /// Wire arg `exec_order` (int). Allocated per CFCALL_ID. Absent OR \<= 0 means auto-allocate (max EXEC_ORDER on this call + 1, seed 0 so an empty call starts at 1). A taken order \> 0 on the same call is ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -\> ALREADY_PRESENT. Order of checks: ftype_id \< 0, duplicate, exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added under a different ftype_id, which makes a later feature-less delete_comparison_call_element ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfcall_id Wire arg `cfcall_id` (int). CFCALL_ID written verbatim. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id written to the BOM row's FTYPE_ID. Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
/// @param felem_id Wire arg `felem_id` (int). FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
/// @param options Optional arguments (see AddComparisonCallElementOptions).
/// @return The modified configuration JSON. AddComparisonCallElementResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddComparisonCallElement(const std::string& config_json, std::int64_t cfcall_id, std::int64_t ftype_id, std::int64_t felem_id, const AddComparisonCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("cfcall_id", cfcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_comparison_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddComparisonCallElement(): same arguments and operation, but returns the record instead of the configuration. Operation: Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -\> ALREADY_PRESENT. Order of checks: ftype_id \< 0, duplicate, exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added under a different ftype_id, which makes a later feature-less delete_comparison_call_element ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfcall_id Wire arg `cfcall_id` (int). CFCALL_ID written verbatim. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id written to the BOM row's FTYPE_ID. Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
/// @param felem_id Wire arg `felem_id` (int). FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
/// @param options Optional arguments (see AddComparisonCallElementOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddComparisonCallElementResult(const std::string& config_json, std::int64_t cfcall_id, std::int64_t ftype_id, std::int64_t felem_id, const AddComparisonCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("cfcall_id", cfcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_comparison_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of DeleteComparisonCallElement(); omitted fields are not sent.
struct DeleteComparisonCallElementOptions {
    /// Wire arg `element_feature` (str). The element's feature code (case-insensitive; unknown is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
    std::optional<std::string> element_feature{};
};

/// @brief Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or feature code plus element code.
///
/// EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no renumbering).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by CFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
/// @param element_code Wire arg `element_code` (str). Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
/// @param options Optional arguments (see DeleteComparisonCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteComparisonCallElement(const std::string& config_json, std::int64_t call, std::string_view element_code, const DeleteComparisonCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_comparison_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or feature code plus element code.
///
/// EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no renumbering).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by CFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
/// @param element_code Wire arg `element_code` (str). Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
/// @param options Optional arguments (see DeleteComparisonCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteComparisonCallElement(const std::string& config_json, std::string_view call, std::string_view element_code, const DeleteComparisonCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_comparison_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its element list (CFG_DFBOM rows).
///
/// Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike add_comparison_call there is no `id` parameter. Validation order: empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_code Wire arg `ftype_code` (str). Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one distinct call per feature: if any CFG_DFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
/// @param dfunc_code Wire arg `dfunc_code` (str). Distinct function code; case-insensitive lookup in CFG_DFUNC, else NOT_FOUND.
/// @param element_list Wire arg `element_list` (str_list). Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT (checked before anything else). One CFG_DFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position. Duplicate items are not rejected.
/// @return The modified configuration JSON. AddDistinctCallResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT.
[[nodiscard]] inline std::string AddDistinctCall(const std::string& config_json, std::string_view ftype_code, std::string_view dfunc_code, const std::vector<std::string>& element_list) {
    detail::ArgsWriter sz_args;
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("dfunc_code", dfunc_code);
    sz_args.StrList("element_list", element_list);
    auto sz_env = detail::Call("add_distinct_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddDistinctCall(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its element list (CFG_DFBOM rows).
///
/// Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike add_comparison_call there is no `id` parameter. Validation order: empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_code Wire arg `ftype_code` (str). Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND. Only one distinct call per feature: if any CFG_DFCALL row already has this FTYPE_ID the call fails with ALREADY_PRESENT.
/// @param dfunc_code Wire arg `dfunc_code` (str). Distinct function code; case-insensitive lookup in CFG_DFUNC, else NOT_FOUND.
/// @param element_list Wire arg `element_list` (str_list). Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty list or a blank/whitespace-only item is INVALID_INPUT (checked before anything else). One CFG_DFBOM row is written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list position. Duplicate items are not rejected.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT.
[[nodiscard]] inline std::string AddDistinctCallResult(const std::string& config_json, std::string_view ftype_code, std::string_view dfunc_code, const std::vector<std::string>& element_list) {
    detail::ArgsWriter sz_args;
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("dfunc_code", dfunc_code);
    sz_args.StrList("element_list", element_list);
    auto sz_env = detail::Call("add_distinct_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a distinct call by DFCALL_ID, cascading to its CFG_DFBOM rows.
///
/// Removes the CFG_DFCALL row and every CFG_DFBOM row with that DFCALL_ID. No dependency or system-call protection.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param dfcall_id Wire arg `dfcall_id` (int). DFCALL_ID; must exist in CFG_DFCALL else NOT_FOUND (an absent section is also NOT_FOUND).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteDistinctCall(const std::string& config_json, std::int64_t dfcall_id) {
    detail::ArgsWriter sz_args;
    sz_args.Int("dfcall_id", dfcall_id);
    auto sz_env = detail::Call("delete_distinct_call", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
///
/// Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it does not include the DFBOM elements (codes: list_distinct_calls; raw rows: get_config_section("CFG_DFBOM")).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by DFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_DFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no distinct call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetDistinctCall(const std::string& config_json, std::int64_t call) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    auto sz_env = detail::Call("get_distinct_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
///
/// Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it does not include the DFBOM elements (codes: list_distinct_calls; raw rows: get_config_section("CFG_DFBOM")).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by DFCALL_ID; a JSON string selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_DFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no distinct call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetDistinctCall(const std::string& config_json, std::string_view call) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    auto sz_env = detail::Call("get_distinct_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all distinct calls with feature/function codes resolved and their ordered element lists.
///
/// Result is an array of {id, feature, function, execOrder, elementList} sorted by (FTYPE_ID, DFCALL_ID) — not config order. execOrder is the CFG_DFCALL row's EXEC_ORDER, which the v4 schema (and every template / add_distinct_call row) lacks, so it is 1. elementList is the call's CFG_DFBOM element codes ordered by EXEC_ORDER. Unresolvable ids render as "unknown". Missing sections are treated as empty. LIMITATION: elementList omits the stored CFG_DFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows with get_config_section("CFG_DFBOM").
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListDistinctCalls(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_distinct_calls", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddDistinctCallElement(); omitted fields are not sent.
struct AddDistinctCallElementOptions {
    /// Wire arg `exec_order` (int). Allocated per DFCALL_ID. Absent OR \<= 0 means auto-allocate (max EXEC_ORDER on this call + 1, seed 0). A taken order \> 0 on the same call is ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -\> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then MISSING_SECTION if CFG_DFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param dfcall_id Wire arg `dfcall_id` (int). DFCALL_ID written verbatim. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id written to the BOM row's FTYPE_ID. NOT validated at all — unlike add_comparison_call_element, a negative id is accepted and stored.
/// @param felem_id Wire arg `felem_id` (int). FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
/// @param options Optional arguments (see AddDistinctCallElementOptions).
/// @return The modified configuration JSON. AddDistinctCallElementResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddDistinctCallElement(const std::string& config_json, std::int64_t dfcall_id, std::int64_t ftype_id, std::int64_t felem_id, const AddDistinctCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("dfcall_id", dfcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_distinct_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddDistinctCallElement(): same arguments and operation, but returns the record instead of the configuration. Operation: Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of EXEC_ORDER -\> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then MISSING_SECTION if CFG_DFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param dfcall_id Wire arg `dfcall_id` (int). DFCALL_ID written verbatim. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id written to the BOM row's FTYPE_ID. NOT validated at all — unlike add_comparison_call_element, a negative id is accepted and stored.
/// @param felem_id Wire arg `felem_id` (int). FELEM_ID written verbatim. NOT validated against CFG_FELEM or CFG_FBOM.
/// @param options Optional arguments (see AddDistinctCallElementOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddDistinctCallElementResult(const std::string& config_json, std::int64_t dfcall_id, std::int64_t ftype_id, std::int64_t felem_id, const AddDistinctCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("dfcall_id", dfcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_distinct_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of DeleteDistinctCallElement(); omitted fields are not sent.
struct DeleteDistinctCallElementOptions {
    /// Wire arg `element_feature` (str). The element's feature code (case-insensitive; unknown is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
    std::optional<std::string> element_feature{};
};

/// @brief Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature code plus element code.
///
/// EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; no renumbering.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by DFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
/// @param element_code Wire arg `element_code` (str). Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
/// @param options Optional arguments (see DeleteDistinctCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteDistinctCallElement(const std::string& config_json, std::int64_t call, std::string_view element_code, const DeleteDistinctCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_distinct_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature code plus element code.
///
/// EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed; no renumbering.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). A JSON integer selects by DFCALL_ID (must exist, else NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is INVALID_INPUT.
/// @param element_code Wire arg `element_code` (str). Element code, case-insensitive. Without element_feature: global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
/// @param options Optional arguments (see DeleteDistinctCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteDistinctCallElement(const std::string& config_json, std::string_view call, std::string_view element_code, const DeleteDistinctCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_distinct_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddExpressionCall(); omitted fields are not sent.
struct AddExpressionCallOptions {
    /// Wire arg `ftype_code` (str). Feature code (case-insensitive) or NOT_FOUND; "ALL" (case-insensitive) = absent; absent stores FTYPE_ID -1.
    std::optional<std::string> ftype_code{};
    /// Wire arg `felem_code` (str). Element code (case-insensitive) or NOT_FOUND; "N/A" (case-insensitive) = absent; absent stores FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else INVALID_INPUT.
    std::optional<std::string> felem_code{};
    /// Wire arg `exec_order` (int). CFG_EFCALL EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or \<= 0 = auto-allocate (max in scope + 1); \> 0 and free = verbatim; \> 0 and taken = ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
    /// Wire arg `expression_feature` (str). Feature code stored as EFEAT_FTYPE_ID (case-insensitive) or NOT_FOUND; absent or "N/A" (case-insensitive) stores -1.
    std::optional<std::string> expression_feature{};
};

/// @brief Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
///
/// Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order, expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any feature. The BOM-feature column is not rendered by get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param efunc_code Wire arg `efunc_code` (str). Expression function code (CFG_EFUNC, case-insensitive) or NOT_FOUND.
/// @param element_list Wire arg `element_list` (json). JSON array of {"element": str, "required": str, "feature"?: str} objects (unknown keys, non-objects, non-string values = INVALID_INPUT; missing element/required = MISSING_FIELD). One CFG_EFBOM row per item, EXEC_ORDER = 1-based list position. element: global CFG_FELEM lookup (case-insensitive) or NOT_FOUND. required: stored verbatim in FELEM_REQ (not validated or normalized). feature: absent stores BOM FTYPE_ID -1 (G2 WILDCARDED_FTYPE: any feature in the record carrying the element); "PARENT" (case-insensitive) stores BOM FTYPE_ID 0 (G2 PARENT_FEATURE_LINKED_FTYPE: the feature that triggered the call); otherwise a feature code (case-insensitive) or NOT_FOUND. The element is NOT checked for membership in that feature. [] is allowed. Shape: `[{element: string, required: string, feature?: string}]`.
/// @param is_virtual Wire arg `is_virtual` (str). Stored verbatim in IS_VIRTUAL (not validated or normalized; the Rust `new()` default is "No").
/// @param options Optional arguments (see AddExpressionCallOptions).
/// @return The modified configuration JSON. AddExpressionCallResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD.
[[nodiscard]] inline std::string AddExpressionCall(const std::string& config_json, std::string_view efunc_code, std::string_view element_list, std::string_view is_virtual, const AddExpressionCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("efunc_code", efunc_code);
    sz_args.Json("element_list", element_list);
    if (options.ftype_code) {
        sz_args.Str("ftype_code", *options.ftype_code);
    }
    if (options.felem_code) {
        sz_args.Str("felem_code", *options.felem_code);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.expression_feature) {
        sz_args.Str("expression_feature", *options.expression_feature);
    }
    sz_args.Str("is_virtual", is_virtual);
    auto sz_env = detail::Call("add_expression_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddExpressionCall(): same arguments and operation, but returns the record instead of the configuration. Operation: Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
///
/// Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order, expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any feature. The BOM-feature column is not rendered by get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param efunc_code Wire arg `efunc_code` (str). Expression function code (CFG_EFUNC, case-insensitive) or NOT_FOUND.
/// @param element_list Wire arg `element_list` (json). JSON array of {"element": str, "required": str, "feature"?: str} objects (unknown keys, non-objects, non-string values = INVALID_INPUT; missing element/required = MISSING_FIELD). One CFG_EFBOM row per item, EXEC_ORDER = 1-based list position. element: global CFG_FELEM lookup (case-insensitive) or NOT_FOUND. required: stored verbatim in FELEM_REQ (not validated or normalized). feature: absent stores BOM FTYPE_ID -1 (G2 WILDCARDED_FTYPE: any feature in the record carrying the element); "PARENT" (case-insensitive) stores BOM FTYPE_ID 0 (G2 PARENT_FEATURE_LINKED_FTYPE: the feature that triggered the call); otherwise a feature code (case-insensitive) or NOT_FOUND. The element is NOT checked for membership in that feature. [] is allowed. Shape: `[{element: string, required: string, feature?: string}]`.
/// @param is_virtual Wire arg `is_virtual` (str). Stored verbatim in IS_VIRTUAL (not validated or normalized; the Rust `new()` default is "No").
/// @param options Optional arguments (see AddExpressionCallOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD.
[[nodiscard]] inline std::string AddExpressionCallResult(const std::string& config_json, std::string_view efunc_code, std::string_view element_list, std::string_view is_virtual, const AddExpressionCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("efunc_code", efunc_code);
    sz_args.Json("element_list", element_list);
    if (options.ftype_code) {
        sz_args.Str("ftype_code", *options.ftype_code);
    }
    if (options.felem_code) {
        sz_args.Str("felem_code", *options.felem_code);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.expression_feature) {
        sz_args.Str("expression_feature", *options.expression_feature);
    }
    sz_args.Str("is_virtual", is_virtual);
    auto sz_env = detail::Call("add_expression_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete an expression call by EFCALL_ID, cascading its CFG_EFBOM rows.
///
/// Removes the CFG_EFCALL row(s) with that id and every CFG_EFBOM row with that EFCALL_ID.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param efcall_id Wire arg `efcall_id` (int). Must match an existing EFCALL_ID, else NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteExpressionCall(const std::string& config_json, std::int64_t efcall_id) {
    detail::ArgsWriter sz_args;
    sz_args.Int("efcall_id", efcall_id);
    auto sz_env = detail::Call("delete_expression_call", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
///
/// Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows: get_config_section("CFG_EFBOM")). Expression calls are many-per-feature (template NAME has 7), so by-feature is often ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = EFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetExpressionCall(const std::string& config_json, std::int64_t call) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    auto sz_env = detail::Call("get_expression_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
///
/// Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows: get_config_section("CFG_EFBOM")). Expression calls are many-per-feature (template NAME has 7), so by-feature is often ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = EFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetExpressionCall(const std::string& config_json, std::string_view call) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    auto sz_env = detail::Call("get_expression_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all expression calls with resolved codes and element lists.
///
/// Array of {id, feature, element, execOrder, function, isVirtual, expressionFeature, elementList}, stably sorted by (FTYPE_ID, FELEM_ID, EXEC_ORDER). feature "all" / element "n/a" / function "unknown" when the id is \<= 0 or unresolved; expressionFeature "n/a" when EFEAT_FTYPE_ID \<= 0. elementList is the BOM element codes ordered by BOM EXEC_ORDER. Missing sections yield []. LIMITATION: elementList omits the stored CFG_EFBOM columns FTYPE_ID (0 = parent feature, -1 = any feature), EXEC_ORDER and FELEM_REQ, which the engine uses; read the raw rows with get_config_section("CFG_EFBOM").
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListExpressionCalls(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_expression_calls", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddExpressionCallElement(); omitted fields are not sent.
struct AddExpressionCallElementOptions {
    /// Wire arg `exec_order` (int). BOM EXEC_ORDER scoped per EFCALL_ID. Absent or \<= 0 = auto-allocate (max on the call + 1); \> 0 and free = verbatim; \> 0 and taken = ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add one CFG_EFBOM row to an expression call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER, FELEM_REQ}). Check order: ftype_id \< 0, ALREADY_PRESENT when (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then MISSING_SECTION if CFG_EFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param efcall_id Wire arg `efcall_id` (int). Stored as EFCALL_ID. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID. \< 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT validated against CFG_FTYPE.
/// @param felem_id Wire arg `felem_id` (int). Stored verbatim as FELEM_ID. NOT validated against CFG_FELEM.
/// @param felem_req Wire arg `felem_req` (str). Stored verbatim in FELEM_REQ (not validated or normalized).
/// @param options Optional arguments (see AddExpressionCallElementOptions).
/// @return The modified configuration JSON. AddExpressionCallElementResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddExpressionCallElement(const std::string& config_json, std::int64_t efcall_id, std::int64_t ftype_id, std::int64_t felem_id, std::string_view felem_req, const AddExpressionCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("efcall_id", efcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    sz_args.Str("felem_req", felem_req);
    auto sz_env = detail::Call("add_expression_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddExpressionCallElement(): same arguments and operation, but returns the record instead of the configuration. Operation: Add one CFG_EFBOM row to an expression call, addressed by raw ids.
///
/// Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID, EXEC_ORDER, FELEM_REQ}). Check order: ftype_id \< 0, ALREADY_PRESENT when (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then MISSING_SECTION if CFG_EFBOM is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param efcall_id Wire arg `efcall_id` (int). Stored as EFCALL_ID. NOT validated — the call need not exist.
/// @param ftype_id Wire arg `ftype_id` (int). The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID. \< 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT validated against CFG_FTYPE.
/// @param felem_id Wire arg `felem_id` (int). Stored verbatim as FELEM_ID. NOT validated against CFG_FELEM.
/// @param felem_req Wire arg `felem_req` (str). Stored verbatim in FELEM_REQ (not validated or normalized).
/// @param options Optional arguments (see AddExpressionCallElementOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION.
[[nodiscard]] inline std::string AddExpressionCallElementResult(const std::string& config_json, std::int64_t efcall_id, std::int64_t ftype_id, std::int64_t felem_id, std::string_view felem_req, const AddExpressionCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("efcall_id", efcall_id);
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("felem_id", felem_id);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    sz_args.Str("felem_req", felem_req);
    auto sz_env = detail::Call("add_expression_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of DeleteExpressionCallElement(); omitted fields are not sent.
struct DeleteExpressionCallElementOptions {
    /// Wire arg `element_feature` (str). The element's feature code (case-insensitive; unknown = NOT_FOUND). Narrows the BOM match to that FTYPE_ID. Without it, an element present under more than one feature on the call is INVALID_INPUT (ambiguous) — e.g. template EFCALL 97 carries TOKENIZED_NM under GROUP_ASSOCIATION and EMPLOYER.
    std::optional<std::string> element_feature{};
};

/// @brief Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
///
/// The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check order: call selector, call existence, element_feature, element, BOM row.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = EFCALL_ID; a string = feature code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call = INVALID_INPUT). A call id that does not exist = NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Element code (case-insensitive). Without element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call = NOT_ON_CALL.
/// @param options Optional arguments (see DeleteExpressionCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteExpressionCallElement(const std::string& config_json, std::int64_t call, std::string_view element_code, const DeleteExpressionCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_expression_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
///
/// The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check order: call selector, call existence, element_feature, element, BOM row.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = EFCALL_ID; a string = feature code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call = INVALID_INPUT). A call id that does not exist = NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Element code (case-insensitive). Without element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call = NOT_ON_CALL.
/// @param options Optional arguments (see DeleteExpressionCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL.
[[nodiscard]] inline std::string DeleteExpressionCallElement(const std::string& config_json, std::string_view call, std::string_view element_code, const DeleteExpressionCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    sz_args.Str("element_code", element_code);
    if (options.element_feature) {
        sz_args.Str("element_feature", *options.element_feature);
    }
    auto sz_env = detail::Call("delete_expression_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddStandardizeCall(); omitted fields are not sent.
struct AddStandardizeCallOptions {
    /// Wire arg `ftype_code` (str). Feature code (case-insensitive) or NOT_FOUND. "ALL" (case-insensitive) is treated as absent. Stored as FTYPE_ID; absent stores FTYPE_ID -1.
    std::optional<std::string> ftype_code{};
    /// Wire arg `felem_code` (str). Element code (case-insensitive) or NOT_FOUND. "N/A" (case-insensitive) is treated as absent. Stored as FELEM_ID; absent stores FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else INVALID_INPUT.
    std::optional<std::string> felem_code{};
    /// Wire arg `exec_order` (int). EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID) of the new row (the -1 sentinel is part of the scope). Absent or \<= 0 = auto-allocate (max in scope + 1, 1 for an empty scope); \> 0 and free = used verbatim; \> 0 and taken = ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or an element.
///
/// Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not include SFUNC_ID, so a second call on the same feature continues that feature's order sequence.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param sfunc_code Wire arg `sfunc_code` (str). Standardize function code (CFG_SFUNC, case-insensitive) or NOT_FOUND. Looked up before the feature/element.
/// @param options Optional arguments (see AddStandardizeCallOptions).
/// @return The modified configuration JSON. AddStandardizeCallResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddStandardizeCall(const std::string& config_json, std::string_view sfunc_code, const AddStandardizeCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("sfunc_code", sfunc_code);
    if (options.ftype_code) {
        sz_args.Str("ftype_code", *options.ftype_code);
    }
    if (options.felem_code) {
        sz_args.Str("felem_code", *options.felem_code);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_standardize_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddStandardizeCall(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or an element.
///
/// Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not include SFUNC_ID, so a second call on the same feature continues that feature's order sequence.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param sfunc_code Wire arg `sfunc_code` (str). Standardize function code (CFG_SFUNC, case-insensitive) or NOT_FOUND. Looked up before the feature/element.
/// @param options Optional arguments (see AddStandardizeCallOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddStandardizeCallResult(const std::string& config_json, std::string_view sfunc_code, const AddStandardizeCallOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("sfunc_code", sfunc_code);
    if (options.ftype_code) {
        sz_args.Str("ftype_code", *options.ftype_code);
    }
    if (options.felem_code) {
        sz_args.Str("felem_code", *options.felem_code);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_standardize_call", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a standardize call by SFCALL_ID.
///
/// Removes every CFG_SFCALL row with that id; no dependency checks (there is no standardize BOM).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param sfcall_id Wire arg `sfcall_id` (int). Must match an existing SFCALL_ID, else NOT_FOUND (also NOT_FOUND when CFG_SFCALL is absent).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteStandardizeCall(const std::string& config_json, std::int64_t sfcall_id) {
    detail::ArgsWriter sz_args;
    sz_args.Int("sfcall_id", sfcall_id);
    auto sz_env = detail::Call("delete_standardize_call", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
///
/// Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER). Element-bound calls (FTYPE_ID -1) are never found by feature. In the template NAME has two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME is ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = SFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetStandardizeCall(const std::string& config_json, std::int64_t call) {
    detail::ArgsWriter sz_args;
    sz_args.Int("call", call);
    auto sz_env = detail::Call("get_standardize_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
///
/// Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER). Element-bound calls (FTYPE_ID -1) are never found by feature. In the template NAME has two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME is ambiguous.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param call Wire arg `call` (int_or_str). Call selector: an integer = SFCALL_ID (NOT_FOUND if absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT — address such calls by id). Any other JSON type = INVALID_INPUT.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string GetStandardizeCall(const std::string& config_json, std::string_view call) {
    detail::ArgsWriter sz_args;
    sz_args.Str("call", call);
    auto sz_env = detail::Call("get_standardize_call", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all standardize calls with resolved codes.
///
/// Array of {id, feature, element, execOrder, function}, stably sorted by (FTYPE_ID, EXEC_ORDER). feature is "all" when FTYPE_ID \<= 0 or unresolved; element is "n/a" when FELEM_ID \<= 0 or unresolved; function is "unknown" when unresolved. No elementList key. Missing sections yield [] (no MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListStandardizeCalls(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_standardize_calls", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddStandardizeCallElement(); omitted fields are not sent.
struct AddStandardizeCallElementOptions {
    /// Wire arg `felem_id` (int). Stored as FELEM_ID; absent = -1. NOT validated against CFG_FELEM.
    std::optional<std::int64_t> felem_id{};
    /// Wire arg `exec_order` (int). EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or \<= 0 = auto-allocate (max in scope + 1); \> 0 and free = verbatim; \> 0 and taken = ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
///
/// Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike add_standardize_call there is no feature-xor-element rule and no id validation.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_id Wire arg `ftype_id` (int). Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE (use -1 for an element-bound row).
/// @param sfunc_id Wire arg `sfunc_id` (int). Stored verbatim as SFUNC_ID. NOT validated against CFG_SFUNC.
/// @param options Optional arguments (see AddStandardizeCallElementOptions).
/// @return The modified configuration JSON. AddStandardizeCallElementResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddStandardizeCallElement(const std::string& config_json, std::int64_t ftype_id, std::int64_t sfunc_id, const AddStandardizeCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("sfunc_id", sfunc_id);
    if (options.felem_id) {
        sz_args.Int("felem_id", *options.felem_id);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_standardize_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddStandardizeCallElement(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
///
/// Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike add_standardize_call there is no feature-xor-element rule and no id validation.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_id Wire arg `ftype_id` (int). Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE (use -1 for an element-bound row).
/// @param sfunc_id Wire arg `sfunc_id` (int). Stored verbatim as SFUNC_ID. NOT validated against CFG_SFUNC.
/// @param options Optional arguments (see AddStandardizeCallElementOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddStandardizeCallElementResult(const std::string& config_json, std::int64_t ftype_id, std::int64_t sfunc_id, const AddStandardizeCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("sfunc_id", sfunc_id);
    if (options.felem_id) {
        sz_args.Int("felem_id", *options.felem_id);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_standardize_call_element", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of DeleteStandardizeCallElement(); omitted fields are not sent.
struct DeleteStandardizeCallElementOptions {
    /// Wire arg `felem_id` (int). Matched against FELEM_ID; absent = -1.
    std::optional<std::int64_t> felem_id{};
};

/// @brief Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
///
/// NOT_FOUND (not NOT_ON_CALL) when no row matches. Removes EVERY matching row.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param ftype_id Wire arg `ftype_id` (int). Matched against FTYPE_ID.
/// @param sfunc_id Wire arg `sfunc_id` (int). Matched against SFUNC_ID.
/// @param options Optional arguments (see DeleteStandardizeCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteStandardizeCallElement(const std::string& config_json, std::int64_t ftype_id, std::int64_t sfunc_id, const DeleteStandardizeCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Int("ftype_id", ftype_id);
    sz_args.Int("sfunc_id", sfunc_id);
    if (options.felem_id) {
        sz_args.Int("felem_id", *options.felem_id);
    }
    auto sz_env = detail::Call("delete_standardize_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Add a new, empty top-level section (an empty array) to G2_CONFIG.
///
/// An existing key of the same (uppercased) name, of ANY JSON type, is ALREADY_EXISTS. A config without a G2_CONFIG key is NOT_FOUND (not MISSING_SECTION). No validation of the name (any string, e.g. not CFG_*, is accepted).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before the duplicate check and storage. The new section is always an empty JSON array.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, ALREADY_EXISTS, NOT_FOUND.
[[nodiscard]] inline std::string AddConfigSection(const std::string& config_json, std::string_view section_name) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    auto sz_env = detail::Call("add_config_section", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Remove a top-level section from G2_CONFIG.
///
/// TRAP: no protection or dependency check; any section (including core sections such as CFG_DSRC, SETTINGS) can be removed. A missing section or a missing G2_CONFIG is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before lookup.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string RemoveConfigSection(const std::string& config_json, std::string_view section_name) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    auto sz_env = detail::Call("remove_config_section", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of GetConfigSection(); omitted fields are not sent.
struct GetConfigSectionOptions {
    /// Wire arg `filter` (str). Absent returns every item. Otherwise keeps items whose json.dumps-spaced rendering (`{"K": 1, "J": null}`, crate::filter::to_json_dumps_string) contains the filter, case-insensitively.
    std::optional<std::string> filter{};
};

/// @brief Get the raw items of a top-level section, optionally filtered by a case-insensitive substring.
///
/// Result is an array. An array section returns its rows; a null or empty section returns []; a non-array, non-null section (e.g. SETTINGS, CONFIG_BASE_VERSION objects) returns a one-element array holding the value. An empty result does not distinguish "section empty" from "filter matched nothing": use config_section_is_empty.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). TRAP: matched EXACTLY (case-sensitive, NOT uppercased, unlike add/remove_config_section). Unknown name is NOT_FOUND.
/// @param options Optional arguments (see GetConfigSectionOptions).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetConfigSection(const std::string& config_json, std::string_view section_name, const GetConfigSectionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    if (options.filter) {
        sz_args.Str("filter", *options.filter);
    }
    auto sz_env = detail::Call("get_config_section", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Report whether a top-level section is empty (null or []).
///
/// Result is a JSON boolean: true for null or [], false otherwise (any non-array, non-null value such as an object counts as non-empty).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Matched EXACTLY (case-sensitive, not uppercased). Unknown name is NOT_FOUND.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string ConfigSectionIsEmpty(const std::string& config_json, std::string_view section_name) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    auto sz_env = detail::Call("config_section_is_empty", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List the names of all top-level G2_CONFIG keys.
///
/// Result is an array of key names in config order, including non-CFG keys (SETTINGS, SYS_OOM, CONFIG_BASE_VERSION). A missing or non-object G2_CONFIG yields [] rather than an error.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListConfigSections(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_config_sections", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Remove a field from every item of an array section, returning how many items had it.
///
/// Record is the integer count of items the field was removed from (0 when no item had it; the config is still returned). Non-object items are skipped. A config with no G2_CONFIG key succeeds unchanged with count 0.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
/// @param field_name Wire arg `field_name` (str). Uppercased before removal (a lowercase key in a row can never be removed).
/// @return The modified configuration JSON. RemoveConfigSectionFieldResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string RemoveConfigSectionField(const std::string& config_json, std::string_view section_name, std::string_view field_name) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    sz_args.Str("field_name", field_name);
    auto sz_env = detail::Call("remove_config_section_field", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of RemoveConfigSectionField(): same arguments and operation, but returns the record instead of the configuration. Operation: Remove a field from every item of an array section, returning how many items had it.
///
/// Record is the integer count of items the field was removed from (0 when no item had it; the config is still returned). Non-object items are skipped. A config with no G2_CONFIG key succeeds unchanged with count 0.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
/// @param field_name Wire arg `field_name` (str). Uppercased before removal (a lowercase key in a row can never be removed).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string RemoveConfigSectionFieldResult(const std::string& config_json, std::string_view section_name, std::string_view field_name) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    sz_args.Str("field_name", field_name);
    auto sz_env = detail::Call("remove_config_section_field", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Add a field to every item of an array section that lacks it, returning existed/updated counts.
///
/// Record is {"existed": n, "updated": n}: items that already had the field (value preserved, never overwritten) vs. items it was inserted into. Non-object items are skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with both counts 0.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
/// @param field_name Wire arg `field_name` (str). Uppercased before insertion.
/// @param field_value Wire arg `field_value` (json). Any JSON value, stored verbatim (cloned) into each item that lacks the field.
/// @return The modified configuration JSON. AddConfigSectionFieldResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string AddConfigSectionField(const std::string& config_json, std::string_view section_name, std::string_view field_name, std::string_view field_value) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    sz_args.Str("field_name", field_name);
    sz_args.Json("field_value", field_value);
    auto sz_env = detail::Call("add_config_section_field", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddConfigSectionField(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a field to every item of an array section that lacks it, returning existed/updated counts.
///
/// Record is {"existed": n, "updated": n}: items that already had the field (value preserved, never overwritten) vs. items it was inserted into. Non-object items are skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with both counts 0.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param section_name Wire arg `section_name` (str). Uppercased before lookup. Must name an ARRAY section, else NOT_FOUND.
/// @param field_name Wire arg `field_name` (str). Uppercased before insertion.
/// @param field_value Wire arg `field_value` (json). Any JSON value, stored verbatim (cloned) into each item that lacks the field.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string AddConfigSectionFieldResult(const std::string& config_json, std::string_view section_name, std::string_view field_name, std::string_view field_value) {
    detail::ArgsWriter sz_args;
    sz_args.Str("section_name", section_name);
    sz_args.Str("field_name", field_name);
    sz_args.Json("field_value", field_value);
    auto sz_env = detail::Call("add_config_section_field", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of AddDataSource(); omitted fields are not sent.
struct AddDataSourceOptions {
    /// Wire arg `retention_level` (str). Case-insensitive; normalized to Remember or Forget. Any other value is INVALID_INPUT. Library default when absent: "Remember".
    std::optional<std::string> retention_level{};
    /// Wire arg `id` (int). Requested DSRC_ID. Absent OR \<= 0 means auto-allocate (next free id, floor 1000). A taken id \> 0 is ALREADY_EXISTS.
    std::optional<std::int64_t> id{};
};

/// @brief Add a data source (CFG_DSRC row) to the configuration.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before storage and duplicate check; DSRC_DESC is set to the same uppercased code.
/// @param options Optional arguments (see AddDataSourceOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT.
[[nodiscard]] inline std::string AddDataSource(const std::string& config_json, std::string_view code, const AddDataSourceOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.retention_level) {
        sz_args.Str("retention_level", *options.retention_level);
    }
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_data_source", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete a data source by code.
///
/// System data sources (DSRC_ID \<= 2, e.g. TEST and SEARCH in the template) are protected and fail with INVALID_INPUT. No dependency check is made against other sections.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string DeleteDataSource(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_data_source", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one data source's raw CFG_DSRC row by code.
///
/// Result is the stored row with on-disk keys (DSRC_ID, DSRC_CODE, DSRC_DESC, RETENTION_LEVEL).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string GetDataSource(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_data_source", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all data sources as {id, dataSource} summaries.
///
/// Result is an array of {"id": DSRC_ID, "dataSource": DSRC_CODE} in config order.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListDataSources(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_data_sources", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetDataSource(); omitted fields are not sent.
struct SetDataSourceOptions {
    /// Wire arg `retention_level` (str). Absent leaves RETENTION_LEVEL unchanged. TRAP: unlike add_data_source the value is written VERBATIM, with no domain validation or case normalization.
    std::optional<std::string> retention_level{};
};

/// @brief Update a data source's retention level.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetDataSourceOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string SetDataSource(const std::string& config_json, std::string_view code, const SetDataSourceOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.retention_level) {
        sz_args.Str("retention_level", *options.retention_level);
    }
    auto sz_env = detail::Call("set_data_source", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddElement(); omitted fields are not sent.
struct AddElementOptions {
    /// Wire arg `description` (str). FELEM_DESC, stored verbatim; absent defaults to the uppercased code.
    std::optional<std::string> description{};
    /// Wire arg `data_type` (str). Case-insensitive; normalized to lowercase string, number, date, datetime or json, else INVALID_INPUT. Library default when absent: "string".
    std::optional<std::string> data_type{};
    /// Wire arg `id` (int). Requested FELEM_ID. Absent OR \<= 0 means auto-allocate (max existing + 1, floor 1000). A taken id \> 0 is ALREADY_EXISTS.
    std::optional<std::int64_t> id{};
};

/// @brief Add an element (CFG_FELEM row).
///
/// Validation order: duplicate code, id, data_type.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased; duplicate (exact match on the uppercased code) is ALREADY_EXISTS.
/// @param options Optional arguments (see AddElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT.
[[nodiscard]] inline std::string AddElement(const std::string& config_json, std::string_view code, const AddElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.data_type) {
        sz_args.Str("data_type", *options.data_type);
    }
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete an element that no feature uses.
///
/// INVALID_INPUT when any CFG_FBOM row maps the element to an existing feature ("Element linked to the following feature(s): ..."). TRAP: CFG_ATTR rows naming the element are NOT checked and are left dangling. MISSING_SECTION for an absent CFG_FELEM or CFG_FBOM; MISSING_FIELD when the matched row's FELEM_ID is not an integer.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup; unknown is NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteElement(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one element as a display summary by code.
///
/// Result is exactly {id, element, datatype}; FELEM_DESC is NOT included.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup; unknown is NOT_FOUND.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string GetElement(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_element", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all elements as display summaries, sorted by element code.
///
/// Array of {id, element, datatype}, sorted alphabetically by element code (not by id).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListElements(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_elements", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetElement(); omitted fields are not sent.
struct SetElementOptions {
    /// Wire arg `description` (str). Absent leaves FELEM_DESC; else stored verbatim.
    std::optional<std::string> description{};
    /// Wire arg `data_type` (str). TRAP — absent leaves DATA_TYPE; else stored VERBATIM with no validation or normalization (unlike add_element).
    std::optional<std::string> data_type{};
};

/// @brief Update an element's description and/or data type.
///
/// Succeeds unchanged when no update args are given.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup; unknown is NOT_FOUND.
/// @param options Optional arguments (see SetElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string SetElement(const std::string& config_json, std::string_view code, const SetElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.data_type) {
        sz_args.Str("data_type", *options.data_type);
    }
    auto sz_env = detail::Call("set_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of SetFeatureElement(); omitted fields are not sent.
struct SetFeatureElementOptions {
    /// Wire arg `exec_order` (int). TRAP — stored verbatim; no uniqueness check and no \<= 0 auto-allocation.
    std::optional<std::int64_t> exec_order{};
    /// Wire arg `display_level` (int). Must be \>= 0, else INVALID_INPUT.
    std::optional<std::int64_t> display_level{};
    /// Wire arg `display_delim` (str). Stored verbatim. Not tri-state; cannot be cleared back to null.
    std::optional<std::string> display_delim{};
    /// Wire arg `derived` (str). Case-insensitive Yes/No, normalized, else INVALID_INPUT.
    std::optional<std::string> derived{};
};

/// @brief Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter or derived flag.
///
/// NOT_FOUND when the (feature, element) mapping is absent. Succeeds unchanged when no update args are given. A missing CFG_FTYPE/CFG_FELEM surfaces as NOT_FOUND (code lookup).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Required. Case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Required. Case-insensitive; unknown is NOT_FOUND.
/// @param options Optional arguments (see SetFeatureElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string SetFeatureElement(const std::string& config_json, std::string_view feature_code, std::string_view element_code, const SetFeatureElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.display_level) {
        sz_args.Int("display_level", *options.display_level);
    }
    if (options.display_delim) {
        sz_args.Str("display_delim", *options.display_delim);
    }
    if (options.derived) {
        sz_args.Str("derived", *options.derived);
    }
    auto sz_env = detail::Call("set_feature_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddElementToFeature(); omitted fields are not sent.
struct AddElementToFeatureOptions {
    /// Wire arg `display_level` (int). Must be \>= 0, else INVALID_INPUT. Library default when absent: 1.
    std::optional<std::int64_t> display_level{};
    /// Wire arg `display_delim` (str). Stored verbatim; absent stores null.
    std::optional<std::string> display_delim{};
    /// Wire arg `derived` (str). Case-insensitive Yes/No, normalized, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> derived{};
};

/// @brief Map an existing element to a feature (append a CFG_FBOM row).
///
/// EXEC_ORDER is always auto-allocated as max(EXEC_ORDER over the WHOLE CFG_FBOM table) + 1 and cannot be requested (use features.add_feature_comparison for an explicit order). Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. Validation order: feature, element, display_level, derived, CFG_FBOM section, duplicate.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Case-insensitive; unknown is NOT_FOUND (the element is never auto-created).
/// @param options Optional arguments (see AddElementToFeatureOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddElementToFeature(const std::string& config_json, std::string_view feature_code, std::string_view element_code, const AddElementToFeatureOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    if (options.display_level) {
        sz_args.Int("display_level", *options.display_level);
    }
    if (options.display_delim) {
        sz_args.Str("display_delim", *options.display_delim);
    }
    if (options.derived) {
        sz_args.Str("derived", *options.derived);
    }
    auto sz_env = detail::Call("add_element_to_feature", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Remove one feature-element (CFG_FBOM) mapping.
///
/// NOT_FOUND when the mapping is absent. The CFG_FELEM row is kept.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Case-insensitive; unknown is NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string DeleteElementFromFeature(const std::string& config_json, std::string_view feature_code, std::string_view element_code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    auto sz_env = detail::Call("delete_element_from_feature", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Render a config document in canonical export form (recursively key-sorted, pretty-printed).
///
/// Result is a JSON STRING holding the rendered text (not the working config envelope): every object's keys sorted recursively (Python json.dumps(sort_keys=True)), array order kept, no trailing newline. The text is itself a valid config and may be passed back in. Any JSON document is accepted (no G2_CONFIG check).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param indent Wire arg `indent` (int). Spaces per indentation level, applied verbatim (2 = CLI on-disk form, 4 = Python parity; 0 allowed). Negative is INVALID_INPUT (raised by the req_usize converter, before the library is called).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT.
[[nodiscard]] inline std::string RenderConfig(const std::string& config_json, std::int64_t indent) {
    detail::ArgsWriter sz_args;
    sz_args.Int("indent", indent);
    auto sz_env = detail::Call("render_config", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddFeature(); omitted fields are not sent.
struct AddFeatureOptions {
    /// Wire arg `class` (str). CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND. Library default when absent: "OTHER".
    std::optional<std::string> class_{};
    /// Wire arg `behavior` (str). Behavior code (A1, F1, FF, FM, FVM, NONE, NAME; E/S suffixes set FTYPE_EXCL/FTYPE_STAB), case-insensitive; otherwise INVALID_INPUT. Library default when absent: "FM".
    std::optional<std::string> behavior{};
    /// Wire arg `candidates` (str). USED_FOR_CAND; case-insensitive Yes/No, normalized, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> candidates{};
    /// Wire arg `anonymize` (str). ANONYMIZE; case-insensitive Yes/No, normalized, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> anonymize{};
    /// Wire arg `derived` (str). DERIVED; case-insensitive Yes/No, normalized, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> derived{};
    /// Wire arg `history` (str). PERSIST_HISTORY; case-insensitive Yes/No, normalized, else INVALID_INPUT. Library default when absent: "Yes".
    std::optional<std::string> history{};
    /// Wire arg `matchkey` (str). SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial, normalized, else INVALID_INPUT. Absent defaults to Yes when `comparison` is given, else No.
    std::optional<std::string> matchkey{};
    /// Wire arg `standardize` (str). CFG_SFUNC code (case-insensitive) or NOT_FOUND (an empty string is looked up too, so "" is NOT_FOUND). Creates a CFG_SFCALL row (SFCALL_ID max+1 floor 1000, EXEC_ORDER 1, FELEM_ID -1).
    std::optional<std::string> standardize{};
    /// Wire arg `expression` (str). CFG_EFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND). Requires at least one element_list item with expressed=yes, else INVALID_INPUT. Creates a CFG_EFCALL row (EFCALL_ID max+1 floor 1000, EXEC_ORDER 1) and a CFG_EFBOM row (FELEM_REQ Yes) per expressed element.
    std::optional<std::string> expression{};
    /// Wire arg `comparison` (str). CFG_CFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND). Requires at least one element_list item with compared=yes, else INVALID_INPUT. Creates a CFG_CFCALL row (CFCALL_ID max+1 floor 1000) and a CFG_CFBOM row per compared element.
    std::optional<std::string> comparison{};
    /// Wire arg `version` (int). Stored verbatim in VERSION. Library default when absent: 1.
    std::optional<std::int64_t> version{};
    /// Wire arg `rtype_id` (int). Stored verbatim in RTYPE_ID; not validated against CFG_RTYPE. Library default when absent: 0.
    std::optional<std::int64_t> rtype_id{};
    /// Wire arg `id` (int). Requested FTYPE_ID. Absent OR \<= 0 means auto-allocate (max existing + 1, floor 1000). A taken id \> 0 is ALREADY_EXISTS.
    std::optional<std::int64_t> id{};
};

/// @brief Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional standardize/expression/comparison calls.
///
/// Validation order: CFG_FTYPE section, duplicate code, element_list shape, candidates/anonymize/derived/history/matchkey domains, id, behavior, class, function codes, expressed/compared counts, then per-element (display level, derived). The input config is never modified on error.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Uppercased; stored as FTYPE_CODE and FTYPE_DESC. Duplicate (exact match on the uppercased code) is ALREADY_EXISTS.
/// @param element_list Wire arg `element_list` (json). Must be a non-empty JSON array, else INVALID_INPUT. Each item is either an element-code string, or an object with `element` (or `ELEMENT`, required, else INVALID_INPUT) and optional `expressed`/`EXPRESSED`, `compared`/`COMPARED` ("yes" case-insensitive = true), `display`/`DISPLAY` ("yes" = DISPLAY_LEVEL 1, anything else 0) or `displaylevel`/`DISPLAYLEVEL`/`display_level` (int, default 1, negative = INVALID_INPUT), `displaydelim`/`DISPLAYDELIM`/`display_delim`, `derived`/`DERIVED` (Yes/No case-insensitive, else INVALID_INPUT; default No). Any other item type is INVALID_INPUT. Element codes are uppercased; a code not in CFG_FELEM is AUTO-CREATED (FELEM_ID max+1 floor 1000, DATA_TYPE string, FELEM_DESC = code). The FBOM EXEC_ORDER is the item's 1-based position (per feature, not whole-table). Shape: `[string | {element?: string, ELEMENT?: string, expressed?: string, EXPRESSED?: string, compared?: string, COMPARED?: string, display?: string, DISPLAY?: string, displaylevel?: int, DISPLAYLEVEL?: int, display_level?: int, displaydelim?: string, DISPLAYDELIM?: string, display_delim?: string, derived?: string, DERIVED?: string}]`.
/// @param options Optional arguments (see AddFeatureOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND, INVALID_STRUCTURE.
[[nodiscard]] inline std::string AddFeature(const std::string& config_json, std::string_view feature, std::string_view element_list, const AddFeatureOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    sz_args.Json("element_list", element_list);
    if (options.class_) {
        sz_args.Str("class", *options.class_);
    }
    if (options.behavior) {
        sz_args.Str("behavior", *options.behavior);
    }
    if (options.candidates) {
        sz_args.Str("candidates", *options.candidates);
    }
    if (options.anonymize) {
        sz_args.Str("anonymize", *options.anonymize);
    }
    if (options.derived) {
        sz_args.Str("derived", *options.derived);
    }
    if (options.history) {
        sz_args.Str("history", *options.history);
    }
    if (options.matchkey) {
        sz_args.Str("matchkey", *options.matchkey);
    }
    if (options.standardize) {
        sz_args.Str("standardize", *options.standardize);
    }
    if (options.expression) {
        sz_args.Str("expression", *options.expression);
    }
    if (options.comparison) {
        sz_args.Str("comparison", *options.comparison);
    }
    if (options.version) {
        sz_args.Int("version", *options.version);
    }
    if (options.rtype_id) {
        sz_args.Int("rtype_id", *options.rtype_id);
    }
    if (options.id) {
        sz_args.Int("id", *options.id);
    }
    auto sz_env = detail::Call("add_feature", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete a feature and cascade-delete its FBOM rows, attributes and standardize/expression/comparison/distinct calls.
///
/// Locked features NAME, ADDRESS, PHONE, DOB, REL_LINK, REL_ANCHOR, REL_POINTER are INVALID_INPUT. Cascade removes the feature's CFG_FBOM rows, CFG_ATTR rows whose FTYPE_CODE matches, CFG_SFCALL, CFG_EFCALL (+ their CFG_EFBOM), CFG_CFCALL (+ CFG_CFBOM) and CFG_DFCALL (+ CFG_DFBOM) rows; CFG_FELEM rows are kept. A missing CFG_FTYPE is MISSING_SECTION for an id but NOT_FOUND for a code.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). A string that parses (after trim) as an integer is an FTYPE_ID; otherwise a feature code, uppercased. Unknown is NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string DeleteFeature(const std::string& config_json, std::string_view feature) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    auto sz_env = detail::Call("delete_feature", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one feature as a display summary including its elementList.
///
/// Result {id, feature, class, behavior, anonymize, candidates, standardize, expression, comparison, matchKey, version, elementList}; elementList items {element, expressed, compared, derived, display} sorted by EXEC_ORDER, display = "No" iff DISPLAY_LEVEL is 0. standardize/expression/comparison are "" when absent. DERIVED, PERSIST_HISTORY and RTYPE_ID of the feature are NOT in the result. A missing CFG_FTYPE section is NOT_FOUND (never MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetFeature(const std::string& config_json, std::string_view feature) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    auto sz_env = detail::Call("get_feature", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all features as display summaries, sorted by FTYPE_ID.
///
/// Array of the same objects get_feature returns.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListFeatures(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_features", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetFeature(); omitted fields are not sent.
struct SetFeatureOptions {
    /// Wire arg `candidates` (str). USED_FOR_CAND; case-insensitive Yes/No, normalized, else INVALID_INPUT.
    std::optional<std::string> candidates{};
    /// Wire arg `anonymize` (str). TRAP — stored VERBATIM in ANONYMIZE (no validation or normalization, unlike add_feature).
    std::optional<std::string> anonymize{};
    /// Wire arg `derived` (str). TRAP — stored VERBATIM in DERIVED (no validation or normalization).
    std::optional<std::string> derived{};
    /// Wire arg `history` (str). TRAP — stored VERBATIM in PERSIST_HISTORY (no validation or normalization).
    std::optional<std::string> history{};
    /// Wire arg `matchkey` (str). SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial, normalized, else INVALID_INPUT.
    std::optional<std::string> matchkey{};
    /// Wire arg `behavior` (str). Behavior code (case-insensitive) parsed into FTYPE_FREQ/FTYPE_EXCL/FTYPE_STAB, else INVALID_INPUT.
    std::optional<std::string> behavior{};
    /// Wire arg `class` (str). CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND.
    std::optional<std::string> class_{};
    /// Wire arg `version` (int). Stored verbatim in VERSION.
    std::optional<std::int64_t> version{};
    /// Wire arg `rtype_id` (int). Stored verbatim in RTYPE_ID; not validated.
    std::optional<std::int64_t> rtype_id{};
};

/// @brief Update a feature's flags, behavior, class, version or RTYPE_ID.
///
/// If no supplied value differs from the stored row (including when no update args are given) the call fails with INVALID_INPUT "No changes detected". The input config is never modified on error.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature Wire arg `feature` (str). Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased. Unknown is NOT_FOUND.
/// @param options Optional arguments (see SetFeatureOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string SetFeature(const std::string& config_json, std::string_view feature, const SetFeatureOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature", feature);
    if (options.candidates) {
        sz_args.Str("candidates", *options.candidates);
    }
    if (options.anonymize) {
        sz_args.Str("anonymize", *options.anonymize);
    }
    if (options.derived) {
        sz_args.Str("derived", *options.derived);
    }
    if (options.history) {
        sz_args.Str("history", *options.history);
    }
    if (options.matchkey) {
        sz_args.Str("matchkey", *options.matchkey);
    }
    if (options.behavior) {
        sz_args.Str("behavior", *options.behavior);
    }
    if (options.class_) {
        sz_args.Str("class", *options.class_);
    }
    if (options.version) {
        sz_args.Int("version", *options.version);
    }
    if (options.rtype_id) {
        sz_args.Int("rtype_id", *options.rtype_id);
    }
    auto sz_env = detail::Call("set_feature", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddFeatureComparison(); omitted fields are not sent.
struct AddFeatureComparisonOptions {
    /// Wire arg `exec_order` (int). WHOLE-TABLE scope: absent or \<= 0 auto-allocates max(EXEC_ORDER over all of CFG_FBOM) + 1; a value \> 0 already used by ANY CFG_FBOM row is ALREADY_EXISTS.
    std::optional<std::int64_t> exec_order{};
    /// Wire arg `display_level` (int). TRAP — stored verbatim, NOT validated, and absent stores DISPLAY_LEVEL null (no default 1, unlike add_element_to_feature).
    std::optional<std::int64_t> display_level{};
    /// Wire arg `display_delim` (str). Stored verbatim; absent stores null.
    std::optional<std::string> display_delim{};
    /// Wire arg `derived` (str). TRAP — stored verbatim, NOT validated; absent stores DERIVED null.
    std::optional<std::string> derived{};
};

/// @brief Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.
///
/// Writes the same table as elements.add_element_to_feature. Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. A missing CFG_FTYPE/CFG_FELEM section surfaces as NOT_FOUND (code lookup); a missing CFG_FBOM as MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Required. Feature code, case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Required. Element code, case-insensitive; unknown is NOT_FOUND.
/// @param options Optional arguments (see AddFeatureComparisonOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddFeatureComparison(const std::string& config_json, std::string_view feature_code, std::string_view element_code, const AddFeatureComparisonOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.display_level) {
        sz_args.Int("display_level", *options.display_level);
    }
    if (options.display_delim) {
        sz_args.Str("display_delim", *options.display_delim);
    }
    if (options.derived) {
        sz_args.Str("derived", *options.derived);
    }
    auto sz_env = detail::Call("add_feature_comparison", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete one feature-element (CFG_FBOM) row.
///
/// Same effect as elements.delete_element_from_feature, except a missing CFG_FBOM section is NOT_FOUND here (MISSING_SECTION there). An absent mapping is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Case-insensitive; unknown is NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteFeatureComparison(const std::string& config_json, std::string_view feature_code, std::string_view element_code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    auto sz_env = detail::Call("delete_feature_comparison", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one raw CFG_FBOM row by feature and element code.
///
/// Result uses on-disk keys (FTYPE_ID, FELEM_ID, EXEC_ORDER, DISPLAY_LEVEL, DISPLAY_DELIM, DERIVED).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Required. Case-insensitive; unknown is NOT_FOUND.
/// @param element_code Wire arg `element_code` (str). Required. Case-insensitive; unknown is NOT_FOUND.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string GetFeatureComparison(const std::string& config_json, std::string_view feature_code, std::string_view element_code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("element_code", element_code);
    auto sz_env = detail::Call("get_feature_comparison", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all raw CFG_FBOM rows sorted by (FTYPE_ID, EXEC_ORDER).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListFeatureComparisons(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_feature_comparisons", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddFeatureDistinctCallElement(); omitted fields are not sent.
struct AddFeatureDistinctCallElementOptions {
    /// Wire arg `element_code` (str). TRAP — only validated (unknown is NOT_FOUND); it is NOT stored (CFG_DFCALL has no FELEM_ID column).
    std::optional<std::string> element_code{};
    /// Wire arg `exec_order` (int). TRAP — IGNORED entirely (CFG_DFCALL has no EXEC_ORDER column).
    std::optional<std::int64_t> exec_order{};
};

/// @brief Add a distinct-function call (CFG_DFCALL row) for a feature.
///
/// Duplicate (FTYPE_ID, DFUNC_ID) is ALREADY_EXISTS. New row is exactly {DFCALL_ID (max+1 floor 1000), FTYPE_ID, DFUNC_ID}; no CFG_DFBOM rows are written.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_code Wire arg `feature_code` (str). Required. Case-insensitive; unknown is NOT_FOUND.
/// @param distinct_func_code Wire arg `distinct_func_code` (str). Required. CFG_DFUNC code, case-insensitive; unknown is NOT_FOUND.
/// @param options Optional arguments (see AddFeatureDistinctCallElementOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddFeatureDistinctCallElement(const std::string& config_json, std::string_view feature_code, std::string_view distinct_func_code, const AddFeatureDistinctCallElementOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_code", feature_code);
    sz_args.Str("distinct_func_code", distinct_func_code);
    if (options.element_code) {
        sz_args.Str("element_code", *options.element_code);
    }
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    auto sz_env = detail::Call("add_feature_distinct_call_element", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief List all raw CFG_FCLASS rows sorted by FCLASS_ID.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListFeatureClasses(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_feature_classes", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get one raw CFG_FCLASS row by id or code.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param feature_class Wire arg `feature_class` (str). Integer string (after trim) = FCLASS_ID; otherwise a code, uppercased. Unknown is NOT_FOUND.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND.
[[nodiscard]] inline std::string GetFeatureClass(const std::string& config_json, std::string_view feature_class) {
    detail::ArgsWriter sz_args;
    sz_args.Str("feature_class", feature_class);
    auto sz_env = detail::Call("get_feature_class", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Set G2_CONFIG.CONFIG_BASE_VERSION.COMPATIBILITY_VERSION.FEATURE_VERSION.
///
/// MISSING_SECTION when COMPATIBILITY_VERSION is absent or not an object. No manifest function reads FEATURE_VERSION back (versioning reads CONFIG_VERSION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param version Wire arg `version` (str). Stored verbatim as a string (inserted or overwritten); not validated.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string UpdateFeatureVersion(const std::string& config_json, std::string_view version) {
    detail::ArgsWriter sz_args;
    sz_args.Str("version", version);
    auto sz_env = detail::Call("update_feature_version", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
///
/// Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param fragment_config Wire arg `fragment_config` (json). Object with on-disk keys. ERFRAG_CODE (string, required, else MISSING_FIELD) is uppercased for storage and the duplicate check (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is stored verbatim; every name referenced inside a FRAGMENT[...] clause (e.g. "./FRAGMENT[./SAME_NAME\>0 and ./SAME_STAB\>0]") must be an existing ERFRAG_CODE matched EXACTLY (case-sensitive), else INVALID_INPUT. A source without FRAGMENT[ (including "") is accepted unvalidated. ERFRAG_ID (integer, optional): absent or \<= 0 auto-allocates (max + 1, floor 1, so 1000 on the template); a taken id \> 0 is ALREADY_EXISTS. Any ERFRAG_DESC key is IGNORED. Shape: `{ERFRAG_CODE: string, ERFRAG_SOURCE: string, ERFRAG_ID?: int, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}`.
/// @return The modified configuration JSON. AddFragmentResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION.
[[nodiscard]] inline std::string AddFragment(const std::string& config_json, std::string_view fragment_config) {
    detail::ArgsWriter sz_args;
    sz_args.Json("fragment_config", fragment_config);
    auto sz_env = detail::Call("add_fragment", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddFragment(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
///
/// Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param fragment_config Wire arg `fragment_config` (json). Object with on-disk keys. ERFRAG_CODE (string, required, else MISSING_FIELD) is uppercased for storage and the duplicate check (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is stored verbatim; every name referenced inside a FRAGMENT[...] clause (e.g. "./FRAGMENT[./SAME_NAME\>0 and ./SAME_STAB\>0]") must be an existing ERFRAG_CODE matched EXACTLY (case-sensitive), else INVALID_INPUT. A source without FRAGMENT[ (including "") is accepted unvalidated. ERFRAG_ID (integer, optional): absent or \<= 0 auto-allocates (max + 1, floor 1, so 1000 on the template); a taken id \> 0 is ALREADY_EXISTS. Any ERFRAG_DESC key is IGNORED. Shape: `{ERFRAG_CODE: string, ERFRAG_SOURCE: string, ERFRAG_ID?: int, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}`.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION.
[[nodiscard]] inline std::string AddFragmentResult(const std::string& config_json, std::string_view fragment_config) {
    detail::ArgsWriter sz_args;
    sz_args.Json("fragment_config", fragment_config);
    auto sz_env = detail::Call("add_fragment", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a fragment by code.
///
/// TRAP: no dependency check; a fragment still referenced by a rule (QUAL_ERFRAG_CODE / DISQ_ERFRAG_CODE) or by another fragment's ERFRAG_DEPENDS is deleted anyway, leaving dangling references. A config without CFG_ERFRAG is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased, then matched exactly against ERFRAG_CODE. Not an id.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteFragment(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_fragment", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one fragment, by code or ERFRAG_ID, as a summary record.
///
/// Result is {id, fragment, source, depends}: id/source/depends are null-preserving projections of ERFRAG_ID/ERFRAG_SOURCE/ERFRAG_DEPENDS; fragment is ERFRAG_CODE or "" when absent. ERFRAG_DESC is not reported.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code_or_id Wire arg `code_or_id` (str). Uppercased, then matched exactly against ERFRAG_CODE first, then numerically against ERFRAG_ID (e.g. "11").
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetFragment(const std::string& config_json, std::string_view code_or_id) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code_or_id", code_or_id);
    auto sz_env = detail::Call("get_fragment", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all fragments as summary records in config order.
///
/// Result is an array of the get_fragment record shape, in config order (NOT sorted). A missing CFG_ERFRAG or G2_CONFIG yields [].
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListFragments(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_fragments", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetFragment(); omitted fields are not sent.
struct SetFragmentOptions {
    /// Wire arg `source` (str). Tri-state: Leave (absent) / Clear (null) / Set. ERFRAG_SOURCE. Absent = keep source AND ERFRAG_DEPENDS; null = clear BOTH source and ERFRAG_DEPENDS to null; a string is validated exactly as in add_fragment (INVALID_INPUT) and ERFRAG_DEPENDS is recomputed.
    FieldUpdate<std::string> source{};
    /// Wire arg `description` (str). Tri-state: Leave (absent) / Clear (null) / Set. ERFRAG_DESC. Absent = keep; null = clear to null; a string is stored verbatim.
    FieldUpdate<std::string> description{};
};

/// @brief Update a fragment's source and/or description.
///
/// The row is rewritten with every CFG_ERFRAG key; ERFRAG_ID is preserved.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Fragment code; uppercased, then matched exactly. Unknown is NOT_FOUND.
/// @param options Optional arguments (see SetFragmentOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string SetFragment(const std::string& config_json, std::string_view code, const SetFragmentOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.source.IsSet()) {
        sz_args.Str("source", options.source.Value());
    } else if (options.source.IsClear()) {
        sz_args.Null("source");
    }
    if (options.description.IsSet()) {
        sz_args.Str("description", options.description.Value());
    } else if (options.description.IsClear()) {
        sz_args.Null("description");
    }
    auto sz_env = detail::Call("set_fragment", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddComparisonFunction(); omitted fields are not sent.
struct AddComparisonFunctionOptions {
    /// Wire arg `connect_str` (str). Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
    std::optional<std::string> connect_str{};
    /// Wire arg `description` (str). Absent stores CFUNC_DESC null; any string is stored verbatim.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent stores LANGUAGE null; any string is stored verbatim.
    std::optional<std::string> language{};
    /// Wire arg `anon_support` (str). Case-insensitive; normalized to Yes or No, else INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> anon_support{};
};

/// @brief Add a comparison function (CFG_CFUNC row).
///
/// Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE, CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code, anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (CFUNC_CODE).
/// @param options Optional arguments (see AddComparisonFunctionOptions).
/// @return The modified configuration JSON. AddComparisonFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddComparisonFunction(const std::string& config_json, std::string_view code, const AddComparisonFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("add_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddComparisonFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a comparison function (CFG_CFUNC row).
///
/// Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE, CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code, anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (CFUNC_CODE).
/// @param options Optional arguments (see AddComparisonFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddComparisonFunctionResult(const std::string& config_json, std::string_view code, const AddComparisonFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("add_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a comparison function's CFG_CFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row; CFG_CFCALL rows referencing it are left dangling (use delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteComparisonFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteComparisonFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteComparisonFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete a comparison function's CFG_CFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row; CFG_CFCALL rows referencing it are left dangling (use delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteComparisonFunctionResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
///
/// Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteComparisonFunctionCascadeResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteComparisonFunctionCascade(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_comparison_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteComparisonFunctionCascade(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
///
/// Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteComparisonFunctionCascadeResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_comparison_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Get one comparison function's raw CFG_CFUNC row by code.
///
/// Result uses on-disk keys (CFUNC_ID, CFUNC_CODE, CFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetComparisonFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_comparison_function", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all comparison functions as camelCase summaries.
///
/// Result is an array of {id, function, description, connectStr, anonSupport, language} in config order; all but function are null-preserving. A missing CFG_CFUNC section yields [] (no error).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListComparisonFunctions(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_comparison_functions", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetComparisonFunction(); omitted fields are not sent.
struct SetComparisonFunctionOptions {
    /// Wire arg `connect_str` (str). Tri-state: Leave (absent) / Clear (null) / Set. Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
    FieldUpdate<std::string> connect_str{};
    /// Wire arg `description` (str). Absent leaves CFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> language{};
    /// Wire arg `anon_support` (str). Absent leaves ANON_SUPPORT. TRAP: unlike add, NOT validated or normalized; any string is stored verbatim.
    std::optional<std::string> anon_support{};
};

/// @brief Update a comparison function's connect string / description / language / anon support.
///
/// Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetComparisonFunctionOptions).
/// @return The modified configuration JSON. SetComparisonFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetComparisonFunction(const std::string& config_json, std::string_view code, const SetComparisonFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("set_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of SetComparisonFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Update a comparison function's connect string / description / language / anon support.
///
/// Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetComparisonFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetComparisonFunctionResult(const std::string& config_json, std::string_view code, const SetComparisonFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("set_comparison_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of AddDistinctFunction(); omitted fields are not sent.
struct AddDistinctFunctionOptions {
    /// Wire arg `connect_str` (str). Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
    std::optional<std::string> connect_str{};
    /// Wire arg `description` (str). Stored verbatim in DFUNC_DESC; absent stores null (NOT defaulted to the code).
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Stored verbatim in LANGUAGE; absent stores null.
    std::optional<std::string> language{};
    /// Wire arg `anon_support` (str). Case-insensitive; normalized to Yes or No, any other value is INVALID_INPUT. Library default when absent: "No".
    std::optional<std::string> anon_support{};
};

/// @brief Add a distinct function (CFG_DFUNC row) to the configuration.
///
/// Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE, DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (DFUNC_CODE).
/// @param options Optional arguments (see AddDistinctFunctionOptions).
/// @return The modified configuration JSON. AddDistinctFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddDistinctFunction(const std::string& config_json, std::string_view code, const AddDistinctFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("add_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddDistinctFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a distinct function (CFG_DFUNC row) to the configuration.
///
/// Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE, DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (DFUNC_CODE).
/// @param options Optional arguments (see AddDistinctFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddDistinctFunctionResult(const std::string& config_json, std::string_view code, const AddDistinctFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("add_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a distinct function by code.
///
/// Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteDistinctFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteDistinctFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteDistinctFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete a distinct function by code.
///
/// Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteDistinctFunctionResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Get one distinct function's raw CFG_DFUNC row by code.
///
/// Result is the stored row with on-disk keys. A config without CFG_DFUNC is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetDistinctFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_distinct_function", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all distinct functions as {id, function, connectStr, anonSupport, language} summaries.
///
/// Result is an array in config order; id/connectStr/anonSupport/language are null-preserving (stored null stays null), DFUNC_DESC is not included. A config without G2_CONFIG.CFG_DFUNC yields [] (no MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListDistinctFunctions(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_distinct_functions", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetDistinctFunction(); omitted fields are not sent.
struct SetDistinctFunctionOptions {
    /// Wire arg `connect_str` (str). Tri-state: Leave (absent) / Clear (null) / Set. Absent leaves CONNECT_STR; null writes null; a string (including "") is written.
    FieldUpdate<std::string> connect_str{};
    /// Wire arg `description` (str). Absent leaves DFUNC_DESC; a string is written verbatim.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent leaves LANGUAGE; a string is written verbatim.
    std::optional<std::string> language{};
    /// Wire arg `anon_support` (str). Absent leaves ANON_SUPPORT. TRAP: unlike add_distinct_function the value is written VERBATIM, with no Yes/No validation or case normalization.
    std::optional<std::string> anon_support{};
};

/// @brief Update a distinct function's connect string, description, language or anon support.
///
/// Returns (modified config, the updated CFG_DFUNC row). The row is removed and re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without CFG_DFUNC is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetDistinctFunctionOptions).
/// @return The modified configuration JSON. SetDistinctFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetDistinctFunction(const std::string& config_json, std::string_view code, const SetDistinctFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("set_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of SetDistinctFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Update a distinct function's connect string, description, language or anon support.
///
/// Returns (modified config, the updated CFG_DFUNC row). The row is removed and re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without CFG_DFUNC is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetDistinctFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetDistinctFunctionResult(const std::string& config_json, std::string_view code, const SetDistinctFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    if (options.anon_support) {
        sz_args.Str("anon_support", *options.anon_support);
    }
    auto sz_env = detail::Call("set_distinct_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of AddExpressionFunction(); omitted fields are not sent.
struct AddExpressionFunctionOptions {
    /// Wire arg `connect_str` (str). Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
    std::optional<std::string> connect_str{};
    /// Wire arg `description` (str). Absent stores EFUNC_DESC null; any string is stored verbatim.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent stores LANGUAGE null; any string is stored verbatim.
    std::optional<std::string> language{};
};

/// @brief Add an expression function (CFG_EFUNC row).
///
/// Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE, CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (EFUNC_CODE).
/// @param options Optional arguments (see AddExpressionFunctionOptions).
/// @return The modified configuration JSON. AddExpressionFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddExpressionFunction(const std::string& config_json, std::string_view code, const AddExpressionFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("add_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddExpressionFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Add an expression function (CFG_EFUNC row).
///
/// Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE, CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (EFUNC_CODE).
/// @param options Optional arguments (see AddExpressionFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddExpressionFunctionResult(const std::string& config_json, std::string_view code, const AddExpressionFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("add_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete an expression function's CFG_EFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row; CFG_EFCALL rows referencing it are left dangling (use delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteExpressionFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteExpressionFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteExpressionFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete an expression function's CFG_EFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row; CFG_EFCALL rows referencing it are left dangling (use delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteExpressionFunctionResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
///
/// Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteExpressionFunctionCascadeResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteExpressionFunctionCascade(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_expression_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteExpressionFunctionCascade(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
///
/// Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteExpressionFunctionCascadeResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_expression_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Get one expression function's raw CFG_EFUNC row by code.
///
/// Result uses on-disk keys (EFUNC_ID, EFUNC_CODE, EFUNC_DESC, CONNECT_STR, LANGUAGE). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetExpressionFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_expression_function", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all expression functions as camelCase summaries.
///
/// Result is an array of {id, function, connectStr, language} in config order (description is NOT included); connectStr/language are null-preserving. A missing CFG_EFUNC section yields [] (no error).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListExpressionFunctions(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_expression_functions", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetExpressionFunction(); omitted fields are not sent.
struct SetExpressionFunctionOptions {
    /// Wire arg `connect_str` (str). Tri-state: Leave (absent) / Clear (null) / Set. Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
    FieldUpdate<std::string> connect_str{};
    /// Wire arg `description` (str). Absent leaves EFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> language{};
};

/// @brief Update an expression function's connect string / description / language.
///
/// Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetExpressionFunctionOptions).
/// @return The modified configuration JSON. SetExpressionFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetExpressionFunction(const std::string& config_json, std::string_view code, const SetExpressionFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("set_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of SetExpressionFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Update an expression function's connect string / description / language.
///
/// Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetExpressionFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetExpressionFunctionResult(const std::string& config_json, std::string_view code, const SetExpressionFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("set_expression_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of AddStandardizeFunction(); omitted fields are not sent.
struct AddStandardizeFunctionOptions {
    /// Wire arg `connect_str` (str). Absent stores CONNECT_STR null; any string (including "") is stored verbatim.
    std::optional<std::string> connect_str{};
    /// Wire arg `description` (str). Absent stores SFUNC_DESC null; any string is stored verbatim.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent stores LANGUAGE null; any string is stored verbatim.
    std::optional<std::string> language{};
};

/// @brief Add a standardize function (CFG_SFUNC row).
///
/// Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE, CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (SFUNC_CODE).
/// @param options Optional arguments (see AddStandardizeFunctionOptions).
/// @return The modified configuration JSON. AddStandardizeFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddStandardizeFunction(const std::string& config_json, std::string_view code, const AddStandardizeFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("add_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddStandardizeFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Add a standardize function (CFG_SFUNC row).
///
/// Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE, CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is absent.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before the duplicate check and storage (SFUNC_CODE).
/// @param options Optional arguments (see AddStandardizeFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT.
[[nodiscard]] inline std::string AddStandardizeFunctionResult(const std::string& config_json, std::string_view code, const AddStandardizeFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str) {
        sz_args.Str("connect_str", *options.connect_str);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("add_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a standardize function's CFG_SFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row; CFG_SFCALL rows referencing it are left dangling (use delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteStandardizeFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteStandardizeFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteStandardizeFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete a standardize function's CFG_SFUNC row only (no cascade).
///
/// Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row; CFG_SFCALL rows referencing it are left dangling (use delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteStandardizeFunctionResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a standardize function and its CFG_SFCALL rows.
///
/// Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The modified configuration JSON. DeleteStandardizeFunctionCascadeResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteStandardizeFunctionCascade(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_standardize_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of DeleteStandardizeFunctionCascade(): same arguments and operation, but returns the record instead of the configuration. Operation: Delete a standardize function and its CFG_SFCALL rows.
///
/// Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD.
[[nodiscard]] inline std::string DeleteStandardizeFunctionCascadeResult(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_standardize_function_cascade", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Get one standardize function's raw CFG_SFUNC row by code.
///
/// Result uses on-disk keys (SFUNC_ID, SFUNC_CODE, SFUNC_DESC, CONNECT_STR, LANGUAGE). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetStandardizeFunction(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_standardize_function", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all standardize functions as camelCase summaries.
///
/// Result is an array of {id, function, connectStr, language} in config order (description is NOT included); connectStr/language are null-preserving. A missing CFG_SFUNC section yields [] (no error).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListStandardizeFunctions(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_standardize_functions", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetStandardizeFunction(); omitted fields are not sent.
struct SetStandardizeFunctionOptions {
    /// Wire arg `connect_str` (str). Tri-state: Leave (absent) / Clear (null) / Set. Absent leaves CONNECT_STR; null clears it to null; a string (including "") sets it.
    FieldUpdate<std::string> connect_str{};
    /// Wire arg `description` (str). Absent leaves SFUNC_DESC; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> description{};
    /// Wire arg `language` (str). Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared to null.
    std::optional<std::string> language{};
};

/// @brief Update a standardize function's connect string / description / language.
///
/// Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetStandardizeFunctionOptions).
/// @return The modified configuration JSON. SetStandardizeFunctionResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetStandardizeFunction(const std::string& config_json, std::string_view code, const SetStandardizeFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("set_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of SetStandardizeFunction(): same arguments and operation, but returns the record instead of the configuration. Operation: Update a standardize function's connect string / description / language.
///
/// Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased before lookup.
/// @param options Optional arguments (see SetStandardizeFunctionOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string SetStandardizeFunctionResult(const std::string& config_json, std::string_view code, const SetStandardizeFunctionOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.connect_str.IsSet()) {
        sz_args.Str("connect_str", options.connect_str.Value());
    } else if (options.connect_str.IsClear()) {
        sz_args.Null("connect_str");
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.language) {
        sz_args.Str("language", *options.language);
    }
    auto sz_env = detail::Call("set_standardize_function", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// Optional arguments of CloneGenericPlan(); omitted fields are not sent.
struct CloneGenericPlanOptions {
    /// Wire arg `new_gplan_desc` (str). Stored verbatim in GPLAN_DESC; absent = the uppercased new code.
    std::optional<std::string> new_gplan_desc{};
};

/// @brief Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
///
/// Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's GPLAN_ID is not an integer.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param source_gplan_code Wire arg `source_gplan_code` (str). Uppercased, then matched exactly against GPLAN_CODE; unknown = NOT_FOUND.
/// @param new_gplan_code Wire arg `new_gplan_code` (str). Uppercased before the duplicate check and storage; an existing code = ALREADY_EXISTS.
/// @param options Optional arguments (see CloneGenericPlanOptions).
/// @return The modified configuration JSON. CloneGenericPlanResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG.
[[nodiscard]] inline std::string CloneGenericPlan(const std::string& config_json, std::string_view source_gplan_code, std::string_view new_gplan_code, const CloneGenericPlanOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("source_gplan_code", source_gplan_code);
    sz_args.Str("new_gplan_code", new_gplan_code);
    if (options.new_gplan_desc) {
        sz_args.Str("new_gplan_desc", *options.new_gplan_desc);
    }
    auto sz_env = detail::Call("clone_generic_plan", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of CloneGenericPlan(): same arguments and operation, but returns the record instead of the configuration. Operation: Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
///
/// Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's GPLAN_ID is not an integer.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param source_gplan_code Wire arg `source_gplan_code` (str). Uppercased, then matched exactly against GPLAN_CODE; unknown = NOT_FOUND.
/// @param new_gplan_code Wire arg `new_gplan_code` (str). Uppercased before the duplicate check and storage; an existing code = ALREADY_EXISTS.
/// @param options Optional arguments (see CloneGenericPlanOptions).
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG.
[[nodiscard]] inline std::string CloneGenericPlanResult(const std::string& config_json, std::string_view source_gplan_code, std::string_view new_gplan_code, const CloneGenericPlanOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("source_gplan_code", source_gplan_code);
    sz_args.Str("new_gplan_code", new_gplan_code);
    if (options.new_gplan_desc) {
        sz_args.Str("new_gplan_desc", *options.new_gplan_desc);
    }
    auto sz_env = detail::Call("clone_generic_plan", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a generic plan and all of its generic thresholds.
///
/// System plans (GPLAN_ID \<= 2, i.e. INGEST and SEARCH in the template) are protected: INVALID_INPUT. Removes the CFG_GPLAN row and every CFG_GENERIC_THRESHOLD row with that GPLAN_ID. An absent CFG_GPLAN section is NOT_FOUND (not MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param gplan_code Wire arg `gplan_code` (str). Uppercased, then matched exactly against GPLAN_CODE; unknown = NOT_FOUND.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, INVALID_CONFIG.
[[nodiscard]] inline std::string DeleteGenericPlan(const std::string& config_json, std::string_view gplan_code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("gplan_code", gplan_code);
    auto sz_env = detail::Call("delete_generic_plan", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of ListGenericPlans(); omitted fields are not sent.
struct ListGenericPlansOptions {
    /// Wire arg `filter` (str). Case-insensitive SUBSTRING match against the raw row serialized as JSON text — keys and numbers included (so e.g. "gplan" matches every row). Absent = no filtering.
    std::optional<std::string> filter{};
};

/// @brief List generic plans as {id, plan, description}, optionally filtered.
///
/// Array of {id (GPLAN_ID), plan (GPLAN_CODE), description (GPLAN_DESC)} sorted by id; missing values become 0 / "". An absent CFG_GPLAN (or G2_CONFIG) yields an empty array, never MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param options Optional arguments (see ListGenericPlansOptions).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListGenericPlans(const std::string& config_json, const ListGenericPlansOptions& options = {}) {
    detail::ArgsWriter sz_args;
    if (options.filter) {
        sz_args.Str("filter", *options.filter);
    }
    auto sz_env = detail::Call("list_generic_plans", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Result of SetGenericPlanResult(): each named record field as JSON text (exactly the
/// record member's JSON, e.g. `1001`, `true`, `"4.0.0"`).
struct SetGenericPlanRecord {
    /// Record field `plan_id` (JSON text).
    std::string plan_id{};
    /// Record field `was_created` (JSON text).
    std::string was_created{};
};

/// @brief Create a generic plan, or update the description of an existing one (upsert).
///
/// Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this create path.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param gplan_code Wire arg `gplan_code` (str). Uppercased, then matched exactly against GPLAN_CODE.
/// @param gplan_desc Wire arg `gplan_desc` (str). Written verbatim to GPLAN_DESC.
/// @return The modified configuration JSON. SetGenericPlanResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string SetGenericPlan(const std::string& config_json, std::string_view gplan_code, std::string_view gplan_desc) {
    detail::ArgsWriter sz_args;
    sz_args.Str("gplan_code", gplan_code);
    sz_args.Str("gplan_desc", gplan_desc);
    auto sz_env = detail::Call("set_generic_plan", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of SetGenericPlan(): same arguments and operation, but returns the record instead of the configuration. Operation: Create a generic plan, or update the description of an existing one (upsert).
///
/// Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this create path.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param gplan_code Wire arg `gplan_code` (str). Uppercased, then matched exactly against GPLAN_CODE.
/// @param gplan_desc Wire arg `gplan_desc` (str). Written verbatim to GPLAN_DESC.
/// @return SetGenericPlanRecord (fields: plan_id, was_created).
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline SetGenericPlanRecord SetGenericPlanResult(const std::string& config_json, std::string_view gplan_code, std::string_view gplan_desc) {
    detail::ArgsWriter sz_args;
    sz_args.Str("gplan_code", gplan_code);
    sz_args.Str("gplan_desc", gplan_desc);
    auto sz_env = detail::Call("set_generic_plan", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    const detail::Record sz_rec(std::move(sz_env.result));
    return SetGenericPlanRecord{.plan_id = sz_rec.Member("plan_id"), .was_created = sz_rec.Member("was_created")};
}

/// @brief Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
///
/// Record is the assigned ERRULE_ID (integer). The written row always carries every CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order: fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param id Wire arg `id` (int). Requested ERRULE_ID. 0 or any negative value means auto-allocate (max existing + 1, floor 1000, so 1000 on the template). A taken id \> 0 is ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
/// @param rule_config Wire arg `rule_config` (json). Object with on-disk keys. ERRULE_CODE (string) is required, else MISSING_FIELD; uppercased for storage and the case-insensitive duplicate check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required: absent/non-string is MISSING_FIELD, "" or an unknown code is NOT_FOUND (existence is case-insensitive). DISQ_ERFRAG_CODE is optional: "" is accepted and stored as "", an unknown code is NOT_FOUND. TRAP: both fragment codes are stored VERBATIM (not uppercased). RESOLVE / RELATE default "No", must be Yes/No case-insensitively (stored title-case) else INVALID_INPUT, and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires RTYPE_ID in 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER defaults to null. A non-string / non-integer value for any of these keys is treated as absent. Shape: `{ERRULE_CODE: string, QUAL_ERFRAG_CODE: string, DISQ_ERFRAG_CODE?: string, RESOLVE?: string, RELATE?: string, RTYPE_ID?: int, ERRULE_TIER?: int, ERRULE_ID?: int}`.
/// @return The modified configuration JSON. AddRuleResult() (same arguments) returns the record this operation produces.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION.
[[nodiscard]] inline std::string AddRule(const std::string& config_json, std::int64_t id, std::string_view rule_config) {
    detail::ArgsWriter sz_args;
    sz_args.Int("id", id);
    sz_args.Json("rule_config", rule_config);
    auto sz_env = detail::Call("add_rule", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.config);
}

/// @brief The record (row / ids) of AddRule(): same arguments and operation, but returns the record instead of the configuration. Operation: Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
///
/// Record is the assigned ERRULE_ID (integer). The written row always carries every CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order: fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param id Wire arg `id` (int). Requested ERRULE_ID. 0 or any negative value means auto-allocate (max existing + 1, floor 1000, so 1000 on the template). A taken id \> 0 is ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
/// @param rule_config Wire arg `rule_config` (json). Object with on-disk keys. ERRULE_CODE (string) is required, else MISSING_FIELD; uppercased for storage and the case-insensitive duplicate check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required: absent/non-string is MISSING_FIELD, "" or an unknown code is NOT_FOUND (existence is case-insensitive). DISQ_ERFRAG_CODE is optional: "" is accepted and stored as "", an unknown code is NOT_FOUND. TRAP: both fragment codes are stored VERBATIM (not uppercased). RESOLVE / RELATE default "No", must be Yes/No case-insensitively (stored title-case) else INVALID_INPUT, and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires RTYPE_ID in 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER defaults to null. A non-string / non-integer value for any of these keys is treated as absent. Shape: `{ERRULE_CODE: string, QUAL_ERFRAG_CODE: string, DISQ_ERFRAG_CODE?: string, RESOLVE?: string, RELATE?: string, RTYPE_ID?: int, ERRULE_TIER?: int, ERRULE_ID?: int}`.
/// @return The record (e.g. the created row or ids) as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION.
[[nodiscard]] inline std::string AddRuleResult(const std::string& config_json, std::int64_t id, std::string_view rule_config) {
    detail::ArgsWriter sz_args;
    sz_args.Int("id", id);
    sz_args.Json("rule_config", rule_config);
    auto sz_env = detail::Call("add_rule", config_json, sz_args.Finish(), ResultKind::ConfigAndJson);
    return std::move(sz_env.result);
}

/// @brief Delete a rule by code.
///
/// No dependency or system-rule protection; any rule can be deleted. A config without CFG_ERRULE is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Uppercased, then matched EXACTLY against ERRULE_CODE (a stored lowercase code cannot be deleted). Not an id.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string DeleteRule(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("delete_rule", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one rule, by code or ERRULE_ID, as a summary record.
///
/// Result is {id, rule, resolve, relate, rtype_id, fragment, disqualifier, tier} projected from ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE (null-preserving). TRAP: `tier` is the stored ERRULE_TIER only when RESOLVE is exactly "Yes", otherwise null.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code_or_id Wire arg `code_or_id` (str). Uppercased, then matched exactly against ERRULE_CODE first, then numerically against ERRULE_ID (e.g. "100").
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetRule(const std::string& config_json, std::string_view code_or_id) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code_or_id", code_or_id);
    auto sz_env = detail::Call("get_rule", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief List all rules as summary records, sorted by ERRULE_ID.
///
/// Result is an array of the get_rule record shape, sorted by id ascending (null/absent id sorts as 0). A missing CFG_ERRULE or G2_CONFIG yields [].
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListRules(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_rules", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of SetRule(); omitted fields are not sent.
struct SetRuleOptions {
    /// Wire arg `resolve` (str). Absent keeps the stored RESOLVE. Must be Yes/No case-insensitively (stored title-case), else INVALID_INPUT.
    std::optional<std::string> resolve{};
    /// Wire arg `relate` (str). Absent keeps the stored RELATE. Must be Yes/No case-insensitively, else INVALID_INPUT.
    std::optional<std::string> relate{};
    /// Wire arg `rtype_id` (int). Absent keeps the stored RTYPE_ID. Forced to 1 when the merged RESOLVE is Yes; must be 2, 3 or 4 when RELATE is Yes.
    std::optional<std::int64_t> rtype_id{};
    /// Wire arg `fragment` (str). Tri-state: Leave (absent) / Clear (null) / Set. QUAL_ERFRAG_CODE. Absent = keep (never re-validated); null = clear to null (TRAP: allowed here although add_rule requires a fragment); a string must name an existing fragment (case-insensitive; "" is NOT_FOUND) and is stored UPPERCASED (unlike add_rule).
    FieldUpdate<std::string> fragment{};
    /// Wire arg `disqualifier` (str). Tri-state: Leave (absent) / Clear (null) / Set. DISQ_ERFRAG_CODE. Absent = keep; null = clear to null; "" is accepted and stored ""; another string must name an existing fragment (NOT_FOUND) and is stored uppercased.
    FieldUpdate<std::string> disqualifier{};
    /// Wire arg `tier` (int). Tri-state: Leave (absent) / Clear (null) / Set. ERRULE_TIER. Absent = keep; null = clear; value = set. The merged rule with RESOLVE=Yes must have a non-zero tier, else INVALID_INPUT.
    FieldUpdate<std::int64_t> tier{};
};

/// @brief Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.
///
/// Merges the update into the stored row and re-applies add_rule's RESOLVE/RELATE/exclusivity/tier/RTYPE_ID rules to the merged row (no duplicate-code check); the row is rewritten with every CFG_ERRULE key and ERRULE_ID preserved.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Rule code; uppercased, then matched exactly. Unknown is NOT_FOUND.
/// @param options Optional arguments (see SetRuleOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string SetRule(const std::string& config_json, std::string_view code, const SetRuleOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    if (options.resolve) {
        sz_args.Str("resolve", *options.resolve);
    }
    if (options.relate) {
        sz_args.Str("relate", *options.relate);
    }
    if (options.rtype_id) {
        sz_args.Int("rtype_id", *options.rtype_id);
    }
    if (options.fragment.IsSet()) {
        sz_args.Str("fragment", options.fragment.Value());
    } else if (options.fragment.IsClear()) {
        sz_args.Null("fragment");
    }
    if (options.disqualifier.IsSet()) {
        sz_args.Str("disqualifier", options.disqualifier.Value());
    } else if (options.disqualifier.IsClear()) {
        sz_args.Null("disqualifier");
    }
    if (options.tier.IsSet()) {
        sz_args.Int("tier", options.tier.Value());
    } else if (options.tier.IsClear()) {
        sz_args.Null("tier");
    }
    auto sz_env = detail::Call("set_rule", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddSearchProfile(); omitted fields are not sent.
struct AddSearchProfileOptions {
    /// Wire arg `candidates` (str). DEFAULT_USED_FOR_CAND; trimmed, case-insensitive, normalized to Normal or Off. Absent or blank = Normal. Anything else is VALIDATION_ERRORS (field "candidates", OUT_OF_DOMAIN). Library default when absent: "Normal".
    std::optional<std::string> candidates{};
    /// Wire arg `description` (str). SPROFILE_DESC, stored verbatim; absent = "". Library default when absent: "".
    std::optional<std::string> description{};
    /// Wire arg `elements` (json). Feature candidate overrides: an array of {"feature": FTYPE_CODE, "flag": Yes|No|Y|N} objects (only those two keys, both strings, else INVALID_INPUT; a missing key = MISSING_FIELD); absent = none. Each feature is resolved case-insensitively against CFG_FTYPE (NOT_FOUND); a feature listed twice is VALIDATION_ERRORS (field "overrides", DUPLICATE); a flag other than Yes/Y/No/N (trimmed, case-insensitive) is VALIDATION_ERRORS (field "overrides", OUT_OF_DOMAIN). Stored in FTYPE_OVERRIDES as "[{\<ftypeId\>,\<Y|N\>},...]" sorted by FTYPE_ID, or "[]". Shape: `[{feature: string, flag: "Yes"|"No"|"Y"|"N"}]`.
    std::optional<std::string> elements{};
};

/// @brief Add a search profile (CFG_SPROFILE row) tying a generic plan and feature candidate overrides to a code.
///
/// SPROFILE_ID is always auto-allocated (max + 1, floor 1; 3 on the template, whose only profile is SEARCH = 2); no explicit id can be requested. CFG_SPROFILE is created when absent. Validation order: code, candidates, generic plan, overrides (per element: feature, duplicate, flag), duplicate code. A config without G2_CONFIG fails the generic-plan lookup (NOT_FOUND), so the library's MISSING_SECTION branch is unreachable; a non-array CFG_SPROFILE is INVALID_STRUCTURE.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). SPROFILE_CODE; trimmed and uppercased. Empty after trimming is INVALID_INPUT; an existing code (case-insensitive) is ALREADY_EXISTS.
/// @param generic_plan Wire arg `generic_plan` (str). GPLAN_CODE, matched case-insensitively against CFG_GPLAN (NOT_FOUND); stored as GPLAN_ID.
/// @param options Optional arguments (see AddSearchProfileOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_INPUT, VALIDATION_ERRORS, NOT_FOUND, ALREADY_EXISTS, INVALID_STRUCTURE, MISSING_FIELD.
[[nodiscard]] inline std::string AddSearchProfile(const std::string& config_json, std::string_view code, std::string_view generic_plan, const AddSearchProfileOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    sz_args.Str("generic_plan", generic_plan);
    if (options.candidates) {
        sz_args.Str("candidates", *options.candidates);
    }
    if (options.description) {
        sz_args.Str("description", *options.description);
    }
    if (options.elements) {
        sz_args.Json("elements", *options.elements);
    }
    auto sz_env = detail::Call("add_search_profile", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Get one search profile's raw CFG_SPROFILE row by code.
///
/// Result is the stored row (SPROFILE_ID, SPROFILE_CODE, SPROFILE_DESC, GPLAN_ID, DEFAULT_USED_FOR_CAND, FTYPE_OVERRIDES). A missing section is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param code Wire arg `code` (str). Case-insensitive match against SPROFILE_CODE. Not an id.
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetSearchProfile(const std::string& config_json, std::string_view code) {
    detail::ArgsWriter sz_args;
    sz_args.Str("code", code);
    auto sz_env = detail::Call("get_search_profile", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of ListSearchProfiles(); omitted fields are not sent.
struct ListSearchProfilesOptions {
    /// Wire arg `filter` (str). Absent returns all. Otherwise keeps rows whose COMPACT raw-row JSON (serde_json::to_string, no spaces, on-disk keys and ids, e.g. "GPLAN_ID":2) contains the filter case-insensitively; it is applied to the raw row, not the projected record.
    std::optional<std::string> filter{};
};

/// @brief List search profiles as display records with ids resolved to codes, sorted by id.
///
/// Result is an array of {id, profile, description, genericPlan, candidates, overrides: [{feature, flag: Yes|No}], overridesRaw} sorted by id. genericPlan / feature fall back to the numeric id as a string when unresolvable. A missing CFG_SPROFILE yields [].
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param options Optional arguments (see ListSearchProfilesOptions).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListSearchProfiles(const std::string& config_json, const ListSearchProfilesOptions& options = {}) {
    detail::ArgsWriter sz_args;
    if (options.filter) {
        sz_args.Str("filter", *options.filter);
    }
    auto sz_env = detail::Call("list_search_profiles", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Delete a search profile by code or SPROFILE_ID.
///
/// The shipped profiles INGEST and SEARCH are protected: deleting one that exists is INVALID_INPUT. Existence is checked FIRST, so an absent reserved code (INGEST is not in the template) is NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param search_value Wire arg `search_value` (str). Matched case-insensitively against SPROFILE_CODE, or (when it parses as an integer after trimming) against SPROFILE_ID.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT.
[[nodiscard]] inline std::string DeleteSearchProfile(const std::string& config_json, std::string_view search_value) {
    detail::ArgsWriter sz_args;
    sz_args.Str("search_value", search_value);
    auto sz_env = detail::Call("delete_search_profile", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Create or overwrite a named setting in the G2_CONFIG.SETTINGS object.
///
/// SETTINGS is created when absent, and a non-object SETTINGS value (null, string, array) is REPLACED by a fresh object (prior content lost). A missing or non-object G2_CONFIG is MISSING_SECTION.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param name Wire arg `name` (str). Uppercased; an existing setting of that name is overwritten silently.
/// @param value Wire arg `value` (json). Stored VERBATIM as its typed JSON value (an integer stays an integer, a string a string); no validation.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string SetSetting(const std::string& config_json, std::string_view name, std::string_view value) {
    detail::ArgsWriter sz_args;
    sz_args.Str("name", name);
    sz_args.Json("value", value);
    auto sz_env = detail::Call("set_setting", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief List system parameters as a name -\> string-value map.
///
/// Result is an object. The only parameter is relationshipsBreakMatches, read from BREAK_RES of the FIRST CFG_RTYPE row with RCLASS_ID 2 and reported as a decimal STRING. TRAP: it is reported only when BREAK_RES is a JSON integer; the template's DISCLOSED row stores the string "No", so the template yields {}. A missing CFG_RTYPE/G2_CONFIG also yields {}.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ListSystemParameters(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_system_parameters", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Set a system parameter (relationshipsBreakMatches).
///
/// NOT_FOUND when no CFG_RTYPE row has RCLASS_ID 2 (or CFG_RTYPE/G2_CONFIG is absent). The parameter name is checked AFTER the config is parsed.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param parameter_name Wire arg `parameter_name` (str). Case-insensitive; only relationshipsBreakMatches (or relationships_break_matches) is known. Any other name is INVALID_CONFIG (not INVALID_INPUT).
/// @param parameter_value Wire arg `parameter_value` (json). Written VERBATIM (any JSON value, no validation) to BREAK_RES of the first CFG_RTYPE row with RCLASS_ID 2. Only an integer value is visible to list_system_parameters afterwards.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, INVALID_CONFIG, NOT_FOUND.
[[nodiscard]] inline std::string SetSystemParameter(const std::string& config_json, std::string_view parameter_name, std::string_view parameter_value) {
    detail::ArgsWriter sz_args;
    sz_args.Str("parameter_name", parameter_name);
    sz_args.Json("parameter_value", parameter_value);
    auto sz_env = detail::Call("set_system_parameter", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of AddComparisonThreshold(); omitted fields are not sent.
struct AddComparisonThresholdOptions {
    /// Wire arg `exec_order` (int). Resolved in three steps: (1) if an all-features (FTYPE_ID 0) row already exists for this (cfunc, rtnval), its EXEC_ORDER is REUSED and this arg is ignored; (2) else a value \> 0 is honoured, or ALREADY_EXISTS if taken within (CFUNC_ID, FTYPE_ID 0); (3) else (absent or \<= 0) the next order within (CFUNC_ID, FTYPE_ID 0) is auto-allocated (max + 1). Never null.
    std::optional<std::int64_t> exec_order{};
    /// Wire arg `same_score` (int). Stored verbatim (no range check); absent stores SAME_SCORE null.
    std::optional<std::int64_t> same_score{};
    /// Wire arg `close_score` (int). Stored verbatim; absent stores CLOSE_SCORE null.
    std::optional<std::int64_t> close_score{};
    /// Wire arg `likely_score` (int). Stored verbatim; absent stores LIKELY_SCORE null.
    std::optional<std::int64_t> likely_score{};
    /// Wire arg `plausible_score` (int). Stored verbatim; absent stores PLAUSIBLE_SCORE null.
    std::optional<std::int64_t> plausible_score{};
    /// Wire arg `un_likely_score` (int). Stored verbatim; absent stores UN_LIKELY_SCORE null.
    std::optional<std::int64_t> un_likely_score{};
};

/// @brief Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and return value.
///
/// CFRTN_ID is always auto-allocated (max existing + 1, no floor); there is no id arg. Duplicate key is (CFUNC_ID, FTYPE_ID, uppercased rtnval) = ALREADY_EXISTS. Order: missing fields, cfunc lookup, feature lookup, then CFG_CFRTN section, duplicate, exec order.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfunc_code REQUIRED. Wire arg `cfunc_code` (str). REQUIRED by the library (absent = MISSING_FIELD). Comparison function code (CFG_CFUNC), matched case-insensitively; unknown = NOT_FOUND.
/// @param ftype_code REQUIRED. Wire arg `ftype_code` (str). REQUIRED by the library (absent = MISSING_FIELD). Feature code matched case-insensitively (unknown = NOT_FOUND), or "all" (any case) for the all-features FTYPE_ID 0 sentinel.
/// @param cfunc_rtnval REQUIRED. Wire arg `cfunc_rtnval` (str). REQUIRED by the library (absent = MISSING_FIELD). Return value / score name; uppercased before storage and duplicate check.
/// @param options Optional arguments (see AddComparisonThresholdOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, ALREADY_EXISTS.
[[nodiscard]] inline std::string AddComparisonThreshold(const std::string& config_json, std::string_view cfunc_code, std::string_view ftype_code, std::string_view cfunc_rtnval, const AddComparisonThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("cfunc_code", cfunc_code);
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("cfunc_rtnval", cfunc_rtnval);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.same_score) {
        sz_args.Int("same_score", *options.same_score);
    }
    if (options.close_score) {
        sz_args.Int("close_score", *options.close_score);
    }
    if (options.likely_score) {
        sz_args.Int("likely_score", *options.likely_score);
    }
    if (options.plausible_score) {
        sz_args.Int("plausible_score", *options.plausible_score);
    }
    if (options.un_likely_score) {
        sz_args.Int("un_likely_score", *options.un_likely_score);
    }
    auto sz_env = detail::Call("add_comparison_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief Delete a comparison threshold identified by (comparison function, feature, return value).
///
/// No matching row = NOT_FOUND. No tier/dependency protection: deleting the all-features tier row leaves per-feature rows in place.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfunc_code Wire arg `cfunc_code` (str). Comparison function code, case-insensitive; unknown = NOT_FOUND.
/// @param ftype_code Wire arg `ftype_code` (str). Feature code (case-insensitive) or "all" for FTYPE_ID 0; unknown = NOT_FOUND.
/// @param cfunc_rtnval Wire arg `cfunc_rtnval` (str). Return value, matched case-insensitively.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION.
[[nodiscard]] inline std::string DeleteComparisonThreshold(const std::string& config_json, std::string_view cfunc_code, std::string_view ftype_code, std::string_view cfunc_rtnval) {
    detail::ArgsWriter sz_args;
    sz_args.Str("cfunc_code", cfunc_code);
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("cfunc_rtnval", cfunc_rtnval);
    auto sz_env = detail::Call("delete_comparison_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of SetComparisonThreshold(); omitted fields are not sent.
struct SetComparisonThresholdOptions {
    /// Wire arg `exec_order` (int). Absent leaves EXEC_ORDER unchanged; a value is written VERBATIM (no uniqueness / tier check, any integer incl. \<= 0).
    std::optional<std::int64_t> exec_order{};
    /// Wire arg `same_score` (int). Absent leaves unchanged; else written verbatim.
    std::optional<std::int64_t> same_score{};
    /// Wire arg `close_score` (int). Absent leaves unchanged; else written verbatim.
    std::optional<std::int64_t> close_score{};
    /// Wire arg `likely_score` (int). Absent leaves unchanged; else written verbatim.
    std::optional<std::int64_t> likely_score{};
    /// Wire arg `plausible_score` (int). Absent leaves unchanged; else written verbatim.
    std::optional<std::int64_t> plausible_score{};
    /// Wire arg `un_likely_score` (int). Absent leaves unchanged; else written verbatim.
    std::optional<std::int64_t> un_likely_score{};
};

/// @brief Update the exec order and/or scores of an existing comparison threshold.
///
/// NOT tri-state: a score cannot be cleared back to null.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param cfunc_code REQUIRED. Wire arg `cfunc_code` (str). REQUIRED by the library (absent = MISSING_FIELD). Case-insensitive lookup; unknown = NOT_FOUND.
/// @param ftype_code REQUIRED. Wire arg `ftype_code` (str). REQUIRED (absent = MISSING_FIELD). Feature code (case-insensitive) or "all" for FTYPE_ID 0.
/// @param cfunc_rtnval REQUIRED. Wire arg `cfunc_rtnval` (str). REQUIRED (absent = MISSING_FIELD). Matched case-insensitively; no row = NOT_FOUND.
/// @param options Optional arguments (see SetComparisonThresholdOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION.
[[nodiscard]] inline std::string SetComparisonThreshold(const std::string& config_json, std::string_view cfunc_code, std::string_view ftype_code, std::string_view cfunc_rtnval, const SetComparisonThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("cfunc_code", cfunc_code);
    sz_args.Str("ftype_code", ftype_code);
    sz_args.Str("cfunc_rtnval", cfunc_rtnval);
    if (options.exec_order) {
        sz_args.Int("exec_order", *options.exec_order);
    }
    if (options.same_score) {
        sz_args.Int("same_score", *options.same_score);
    }
    if (options.close_score) {
        sz_args.Int("close_score", *options.close_score);
    }
    if (options.likely_score) {
        sz_args.Int("likely_score", *options.likely_score);
    }
    if (options.plausible_score) {
        sz_args.Int("plausible_score", *options.plausible_score);
    }
    if (options.un_likely_score) {
        sz_args.Int("un_likely_score", *options.un_likely_score);
    }
    auto sz_env = detail::Call("set_comparison_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief List all comparison thresholds with resolved function and feature names.
///
/// Array of {id (CFRTN_ID), function (CFUNC_CODE or "unknown"), returnOrder (EXEC_ORDER), scoreName, feature ("all" for FTYPE_ID 0, else FTYPE_CODE or "unknown"), sameScore, closeScore, likelyScore, plausibleScore, unlikelyScore}, sorted by (CFUNC_ID, CFRTN_ID). TRAP: null/absent EXEC_ORDER and scores are reported as 0, not null. Requires CFG_CFRTN, CFG_CFUNC and CFG_FTYPE (else MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListComparisonThresholds(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_comparison_thresholds", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of AddGenericThreshold(); omitted fields are not sent.
struct AddGenericThresholdOptions {
    /// Wire arg `feature` (str). Absent or "all" (any case) = FTYPE_ID 0. Else uppercased and matched EXACTLY against FTYPE_CODE; unknown = NOT_FOUND. Library default when absent: "ALL".
    std::optional<std::string> feature{};
};

/// @brief Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and optional feature.
///
/// All absent required fields are reported in ONE MISSING_FIELD (order plan, behavior, scoring_cap, candidate_cap, send_to_redo), checked before the config is parsed. Then: plan lookup, feature lookup, duplicate (plan, behavior, feature) = ALREADY_EXISTS (Python treats it as a warning-success; the root library does not), then behavior + sendToRedo are validated TOGETHER into one VALIDATION_ERRORS (details schema sz-configtool.validation-errors/v1, order [behavior, sendToRedo]).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param plan REQUIRED. Wire arg `plan` (str). REQUIRED (absent = MISSING_FIELD). Uppercased, then matched EXACTLY against GPLAN_CODE (so effectively case-insensitive for upper-case stored codes); unknown = NOT_FOUND.
/// @param behavior REQUIRED. Wire arg `behavior` (str). REQUIRED (absent = MISSING_FIELD). Uppercased; must be a canonical behavior code (behavior_domain, e.g. NAME, F1, FM, A1) else VALIDATION_ERRORS (field "behavior", UnknownReferenceCode).
/// @param scoring_cap REQUIRED. Wire arg `scoring_cap` (int). REQUIRED (absent = MISSING_FIELD). Stored verbatim (e.g. -1).
/// @param candidate_cap REQUIRED. Wire arg `candidate_cap` (int). REQUIRED (absent = MISSING_FIELD). Stored verbatim.
/// @param send_to_redo REQUIRED. Wire arg `send_to_redo` (str). REQUIRED (absent = MISSING_FIELD). Case-insensitive Yes/No, stored canonical "Yes"/"No"; else VALIDATION_ERRORS (field "sendToRedo", OutOfDomain).
/// @param options Optional arguments (see AddGenericThresholdOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS, VALIDATION_ERRORS.
[[nodiscard]] inline std::string AddGenericThreshold(const std::string& config_json, std::string_view plan, std::string_view behavior, std::int64_t scoring_cap, std::int64_t candidate_cap, std::string_view send_to_redo, const AddGenericThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("plan", plan);
    sz_args.Str("behavior", behavior);
    sz_args.Int("scoring_cap", scoring_cap);
    sz_args.Int("candidate_cap", candidate_cap);
    sz_args.Str("send_to_redo", send_to_redo);
    if (options.feature) {
        sz_args.Str("feature", *options.feature);
    }
    auto sz_env = detail::Call("add_generic_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of DeleteGenericThreshold(); omitted fields are not sent.
struct DeleteGenericThresholdOptions {
    /// Wire arg `feature` (str). Absent or "all" = FTYPE_ID 0; else uppercased exact FTYPE_CODE match, unknown = NOT_FOUND. Library default when absent: "ALL".
    std::optional<std::string> feature{};
};

/// @brief Delete a generic threshold identified by (plan, behavior, feature).
///
/// No matching row = NOT_FOUND (also when CFG_GENERIC_THRESHOLD is absent). (Before the Unreleased fix it ignored `plan` and always deleted from INGEST.)
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param plan REQUIRED. Wire arg `plan` (str). REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
/// @param behavior REQUIRED. Wire arg `behavior` (str). REQUIRED (absent = MISSING_FIELD). Uppercased and matched exactly; NOT validated against the behavior domain (no row = NOT_FOUND).
/// @param options Optional arguments (see DeleteGenericThresholdOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION.
[[nodiscard]] inline std::string DeleteGenericThreshold(const std::string& config_json, std::string_view plan, std::string_view behavior, const DeleteGenericThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("plan", plan);
    sz_args.Str("behavior", behavior);
    if (options.feature) {
        sz_args.Str("feature", *options.feature);
    }
    auto sz_env = detail::Call("delete_generic_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Optional arguments of SetGenericThreshold(); omitted fields are not sent.
struct SetGenericThresholdOptions {
    /// Wire arg `feature` (str). Lookup KEY selecting the per-feature row (never written). Absent or "all" = FTYPE_ID 0; else case-insensitive feature lookup, unknown = NOT_FOUND. Library default when absent: "ALL".
    std::optional<std::string> feature{};
    /// Wire arg `candidate_cap` (int). Absent leaves CANDIDATE_CAP unchanged; else written verbatim.
    std::optional<std::int64_t> candidate_cap{};
    /// Wire arg `scoring_cap` (int). Absent leaves SCORING_CAP unchanged; else written verbatim.
    std::optional<std::int64_t> scoring_cap{};
    /// Wire arg `send_to_redo` (str). Absent leaves unchanged; else case-insensitive Yes/No stored canonical, otherwise VALIDATION_ERRORS (field "sendToRedo", OutOfDomain) — checked AFTER the row lookup, so a missing row wins.
    std::optional<std::string> send_to_redo{};
};

/// @brief Update the caps and/or send-to-redo flag of an existing generic threshold.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param plan REQUIRED. Wire arg `plan` (str). REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
/// @param behavior REQUIRED. Wire arg `behavior` (str). REQUIRED (absent = MISSING_FIELD). Lookup KEY only: uppercased and matched exactly, never validated against the domain (no row = NOT_FOUND).
/// @param options Optional arguments (see SetGenericThresholdOptions).
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, VALIDATION_ERRORS.
[[nodiscard]] inline std::string SetGenericThreshold(const std::string& config_json, std::string_view plan, std::string_view behavior, const SetGenericThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("plan", plan);
    sz_args.Str("behavior", behavior);
    if (options.feature) {
        sz_args.Str("feature", *options.feature);
    }
    if (options.candidate_cap) {
        sz_args.Int("candidate_cap", *options.candidate_cap);
    }
    if (options.scoring_cap) {
        sz_args.Int("scoring_cap", *options.scoring_cap);
    }
    if (options.send_to_redo) {
        sz_args.Str("send_to_redo", *options.send_to_redo);
    }
    auto sz_env = detail::Call("set_generic_threshold", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// @brief List all generic thresholds with resolved plan and feature names.
///
/// Array of {id (GPLAN_ID), plan, behavior, feature ("all" for FTYPE_ID 0), candidateCap, scoringCap, sendToRedo}, sorted by (GPLAN_ID, canonical behavior position; unknown behaviors last), stable within ties. Requires CFG_GENERIC_THRESHOLD, CFG_GPLAN and CFG_FTYPE (else MISSING_SECTION).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION.
[[nodiscard]] inline std::string ListGenericThresholds(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("list_generic_thresholds", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// Optional arguments of ValidateGenericThreshold(); omitted fields are not sent.
struct ValidateGenericThresholdOptions {
    /// Wire arg `feature` (str). Absent or "all" (any case) = FTYPE_ID 0. Else uppercased exact FTYPE_CODE match; unknown = result notFound/feature. Library default when absent: "ALL".
    std::optional<std::string> feature{};
};

/// @brief Stage the checks of a generic-threshold add without mutating the config, returning the outcome as data.
///
/// Result is the versioned object (schema sz-configtool.generic-threshold-check/v1, root-library Serialize for GenericThresholdCheck): {"schema", "result": "ok" | "duplicate" | "notFound" (+ "which": "plan"|"feature", "value": uppercased code) | "invalid" (+ "failures": [{"field", "reasonCode", "offendingValue"}], order [behavior, sendToRedo])}. Stages stop at the first hit: plan, feature, duplicate (plan, behavior, feature), then field validation. Missing sections are NOT errors (a missing CFG_GPLAN is notFound/plan). Caps are not taken.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param plan Wire arg `plan` (str). Uppercased, matched exactly against GPLAN_CODE; unknown = result notFound/plan (data, not an error).
/// @param behavior Wire arg `behavior` (str). Uppercased; checked against the canonical behavior codes only in the last stage.
/// @param send_to_redo Wire arg `send_to_redo` (str). Case-insensitive Yes/No; checked only in the last stage.
/// @param options Optional arguments (see ValidateGenericThresholdOptions).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE.
[[nodiscard]] inline std::string ValidateGenericThreshold(const std::string& config_json, std::string_view plan, std::string_view behavior, std::string_view send_to_redo, const ValidateGenericThresholdOptions& options = {}) {
    detail::ArgsWriter sz_args;
    sz_args.Str("plan", plan);
    sz_args.Str("behavior", behavior);
    sz_args.Str("send_to_redo", send_to_redo);
    if (options.feature) {
        sz_args.Str("feature", *options.feature);
    }
    auto sz_env = detail::Call("validate_generic_threshold", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Check that a document has the top-level shape of a config (structure only).
///
/// Checks, in order: valid JSON (JSON_PARSE); a G2_CONFIG key (MISSING_SECTION) that is an object (INVALID_STRUCTURE); every recognised CFG_* section (validation::EXPECTED_SECTIONS, 27 names) that is PRESENT is an array (INVALID_STRUCTURE). Absent sections, other keys and cross-references are not checked.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_STRUCTURE.
inline void ValidateConfig(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    (void)detail::Call("validate_config", config_json, sz_args.Finish(), ResultKind::Unit);
}

/// @brief Get the configuration VERSION string (G2_CONFIG.CONFIG_BASE_VERSION.VERSION).
///
/// Result is a JSON string (e.g. "4.4.0" in the template). Absent or non-string VERSION (or any missing parent) = NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetVersion(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("get_version", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Get COMPATIBILITY_VERSION.CONFIG_VERSION.
///
/// Result is a JSON string (e.g. "11" in the template). Absent or non-string CONFIG_VERSION (or any missing parent) = NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @return The result as JSON text.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline std::string GetCompatibilityVersion(const std::string& config_json) {
    detail::ArgsWriter sz_args;
    auto sz_env = detail::Call("get_compatibility_version", config_json, sz_args.Finish(), ResultKind::Json);
    return std::move(sz_env.result);
}

/// @brief Set COMPATIBILITY_VERSION.CONFIG_VERSION.
///
/// Absent CONFIG_BASE_VERSION or COMPATIBILITY_VERSION = NOT_FOUND; COMPATIBILITY_VERSION not an object = INVALID_CONFIG. TRAP: an absent G2_CONFIG is NOT an error — the config is returned unchanged (re-serialized).
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param new_version Wire arg `new_version` (str). Stored verbatim as a JSON string; no format validation.
/// @return The modified configuration JSON.
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND, INVALID_CONFIG.
[[nodiscard]] inline std::string UpdateCompatibilityVersion(const std::string& config_json, std::string_view new_version) {
    detail::ArgsWriter sz_args;
    sz_args.Str("new_version", new_version);
    auto sz_env = detail::Call("update_compatibility_version", config_json, sz_args.Finish(), ResultKind::Config);
    return std::move(sz_env.config);
}

/// Result of VerifyCompatibilityVersion(): each named record field as JSON text (exactly the
/// record member's JSON, e.g. `1001`, `true`, `"4.0.0"`).
struct VerifyCompatibilityVersionRecord {
    /// Record field `current_version` (JSON text).
    std::string current_version{};
    /// Record field `matches` (JSON text).
    std::string matches{};
};

/// @brief Compare COMPATIBILITY_VERSION.CONFIG_VERSION with an expected value.
///
/// Rust returns (current_version, matches) where the String is NOT a config, so it is `json`; tuple_names makes the result the object {"current_version": "11", "matches": true}. Absent CONFIG_VERSION = NOT_FOUND.
///
/// @param config_json Configuration JSON (opaque; passed byte-exact).
/// @param expected_version Wire arg `expected_version` (str). Compared by exact, case-sensitive string equality.
/// @return VerifyCompatibilityVersionRecord (fields: current_version, matches).
/// @throws SzConfigToolException Library reason codes: JSON_PARSE, NOT_FOUND.
[[nodiscard]] inline VerifyCompatibilityVersionRecord VerifyCompatibilityVersion(const std::string& config_json, std::string_view expected_version) {
    detail::ArgsWriter sz_args;
    sz_args.Str("expected_version", expected_version);
    auto sz_env = detail::Call("verify_compatibility_version", config_json, sz_args.Finish(), ResultKind::Json);
    const detail::Record sz_rec(std::move(sz_env.result));
    return VerifyCompatibilityVersionRecord{.current_version = sz_rec.Member("current_version"), .matches = sz_rec.Member("matches")};
}

}  // namespace szconfigtool

#endif  // SZCONFIGTOOL_GENERATED_API_HPP
