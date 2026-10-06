#ifndef LIBSZCONFIGTOOL_H
#define LIBSZCONFIGTOOL_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/*
 * SZCONFIGTOOL_API marks every exported function.
 *  - Static linking (libSzConfigTool.a / static .lib): define SZCONFIGTOOL_STATIC
 *    before including this header.
 *  - Windows DLL: consumers get __declspec(dllimport); define
 *    SZCONFIGTOOL_BUILDING_DLL only when building the DLL itself.
 *  - ELF/Mach-O: default visibility, so the symbols stay exported even when the
 *    consumer compiles with -fvisibility=hidden.
 * Predefine SZCONFIGTOOL_API to override all of the above.
 */
#ifndef SZCONFIGTOOL_API
#  if defined(SZCONFIGTOOL_STATIC)
#    define SZCONFIGTOOL_API
#  elif defined(_WIN32)
#    if defined(SZCONFIGTOOL_BUILDING_DLL)
#      define SZCONFIGTOOL_API __declspec(dllexport)
#    else
#      define SZCONFIGTOOL_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__) || defined(__clang__)
#    define SZCONFIGTOOL_API __attribute__((visibility("default")))
#  else
#    define SZCONFIGTOOL_API
#  endif
#endif

/*
 * ABI version of this header; compare with SzConfigTool_getAbiVersion().
 * Bumped only for incompatible changes to existing declarations.
 */
#define SZCONFIGTOOL_ABI_VERSION 2

/*
 * Error state
 * -----------
 * Every function records its outcome in a per-thread last-error slot: on
 * success the slot is cleared, on failure it holds a message, a code, a stable
 * reason code and (for validation errors) JSON details. The slot is
 * thread-local, so concurrent calls on different threads never see each
 * other's errors. Pointers returned by SzConfigTool_getLastError* are
 * NUL-terminated, owned by the library (do not free) and valid until the next
 * SzConfigTool_* call on the same thread.
 *
 * Return codes (SzConfigTool_result.returnCode)
 * ----------------------------------------------
 * 0 is success; every failure is negative and also sets the last-error slot
 * (message + SzConfigTool_getLastErrorCode() = the same code). The typed
 * exports were added in waves and do NOT share one numbering; this section
 * describes what they actually do (ffi/tests/return_codes.rs checks the lists
 * against the source). SzConfigTool_invoke is the precise interface: it
 * returns only 0, -1 (NULL or invalid UTF-8 argument) and -2, and every -2
 * from a library error carries a stable reason code
 * (SzConfigTool_getLastErrorReasonCode) and, for VALIDATION_ERRORS, details.
 *
 *  -1  A required pointer is NULL (every export). Invalid UTF-8 in an input
 *      for the exports in List A. SzConfigTool_setAttribute also returns -1
 *      when updates_json is not valid JSON. (Also: the result could not be
 *      converted to a C string; JSON output never contains a raw NUL, so this
 *      does not occur in practice.)
 *  -2  Library error WITH a reason code, for every export not in List B or
 *      List C. Library error WITHOUT a reason code (message only) for List C.
 *      Invalid UTF-8 in an input for every export not in List A. An internal
 *      panic ("internal panic in <function>: ...", no reason code), any export.
 *  -3  An argument that must be JSON (updates_json, rule_json, feature_json,
 *      element_list_json, options_json, ...) does not parse or has the wrong
 *      shape (no reason code). Also: serializing a result failed (does not
 *      occur in practice).
 *  -4  SzConfigTool_setGenericThreshold: gplan_id does not resolve to a plan
 *      (or config_json is not JSON), message only. Otherwise: the result could
 *      not be converted to a C string (does not occur in practice).
 *  -5  Library error WITHOUT a reason code (message only) for List B.
 *
 * List A (invalid UTF-8 => -1; for the three *CallElement deletes only in
 * element_feature, their other inputs => -2):
 *   SzConfigTool_addAttribute SzConfigTool_addDataSource
 *   SzConfigTool_deleteAttribute SzConfigTool_deleteComparisonCallElement
 *   SzConfigTool_deleteDataSource SzConfigTool_deleteDistinctCallElement
 *   SzConfigTool_deleteExpressionCallElement SzConfigTool_getAttribute
 *   SzConfigTool_getElement SzConfigTool_getFeature SzConfigTool_invoke
 *   SzConfigTool_listAttributes SzConfigTool_listDataSources
 *   SzConfigTool_listElements SzConfigTool_listFeatures
 *   SzConfigTool_setAttribute
 *
 * List B (library error => -5, no reason code):
 *   SzConfigTool_addCandidateFunction SzConfigTool_addComparisonCall
 *   SzConfigTool_addComparisonFunction SzConfigTool_addConfigSectionField
 *   SzConfigTool_addDistinctCall SzConfigTool_addDistinctFunction
 *   SzConfigTool_addExpressionCall SzConfigTool_addExpressionFunction
 *   SzConfigTool_addFeature SzConfigTool_addFragment
 *   SzConfigTool_addMatchingFunction SzConfigTool_addRule
 *   SzConfigTool_addScoringFunction SzConfigTool_addStandardizeCall
 *   SzConfigTool_addStandardizeFunction SzConfigTool_addValidationFunction
 *   SzConfigTool_cloneGenericPlan SzConfigTool_configSectionIsEmpty
 *   SzConfigTool_deleteCandidateFunction SzConfigTool_deleteComparisonFunction
 *   SzConfigTool_deleteDistinctCall SzConfigTool_deleteDistinctFunction
 *   SzConfigTool_deleteExpressionFunction SzConfigTool_deleteMatchingFunction
 *   SzConfigTool_deleteScoringFunction SzConfigTool_deleteStandardizeFunction
 *   SzConfigTool_deleteValidationFunction SzConfigTool_getCandidateFunction
 *   SzConfigTool_getComparisonCall SzConfigTool_getComparisonFunction
 *   SzConfigTool_getCompatibilityVersion SzConfigTool_getConfigSection
 *   SzConfigTool_getDataSource SzConfigTool_getDistinctCall
 *   SzConfigTool_getDistinctFunction SzConfigTool_getExpressionCall
 *   SzConfigTool_getExpressionFunction SzConfigTool_getFragment
 *   SzConfigTool_getMatchingFunction SzConfigTool_getRule
 *   SzConfigTool_getScoringFunction SzConfigTool_getStandardizeCall
 *   SzConfigTool_getStandardizeFunction SzConfigTool_getThreshold
 *   SzConfigTool_getValidationFunction SzConfigTool_getVersion
 *   SzConfigTool_listCandidateFunctions SzConfigTool_listComparisonCalls
 *   SzConfigTool_listComparisonFunctions SzConfigTool_listComparisonThresholds
 *   SzConfigTool_listConfigSections SzConfigTool_listDistinctCalls
 *   SzConfigTool_listDistinctFunctions SzConfigTool_listExpressionCalls
 *   SzConfigTool_listExpressionFunctions SzConfigTool_listFragments
 *   SzConfigTool_listGenericPlans SzConfigTool_listGenericThresholds
 *   SzConfigTool_listMatchingFunctions SzConfigTool_listRules
 *   SzConfigTool_listScoringFunctions SzConfigTool_listStandardizeCalls
 *   SzConfigTool_listStandardizeFunctions SzConfigTool_listSystemParameters
 *   SzConfigTool_listValidationFunctions SzConfigTool_removeConfigSectionField
 *   SzConfigTool_setCandidateFunction SzConfigTool_setComparisonFunction
 *   SzConfigTool_setDistinctCall SzConfigTool_setDistinctFunction
 *   SzConfigTool_setExpressionFunction SzConfigTool_setGenericPlan
 *   SzConfigTool_setMatchingFunction SzConfigTool_setScoringFunction
 *   SzConfigTool_setStandardizeFunction SzConfigTool_setValidationFunction
 *   SzConfigTool_verifyCompatibilityVersion
 *
 * List C (library error => -2, no reason code):
 *   SzConfigTool_getComparisonCallByFeature
 *   SzConfigTool_getDistinctCallByFeature
 *   SzConfigTool_getExpressionCallByFeature
 *   SzConfigTool_getStandardizeCallByFeature
 */

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Result structure for operations that return modified configuration JSON
 * Matches SzHelpers pattern for C compatibility
 */
typedef struct SzConfigTool_result {
  /**
   * Modified configuration JSON (caller must free with SzConfigTool_free)
   */
  char *response;
  /**
   * Return code: 0 = success, negative = error
   */
  int64_t returnCode;
} SzConfigTool_result;

/**
 * Free memory allocated by this library
 *
 * # Safety
 * ptr must be a valid pointer previously returned by this library, or null
 */
SZCONFIGTOOL_API void SzConfigTool_free(char *ptr);

/**
 * Get the last error message recorded on the calling thread
 *
 * # Returns
 * Pointer to a NUL-terminated error string (do not free), or null if no error.
 * See "Error state" above for the pointer lifetime.
 */
SZCONFIGTOOL_API const char *SzConfigTool_getLastError(void);

/**
 * Get the last error code recorded on the calling thread
 *
 * # Returns
 * Error code (0 = no error, negative = error)
 */
SZCONFIGTOOL_API int64_t SzConfigTool_getLastErrorCode(void);

/**
 * Get the last error's stable reason code (e.g. "VALIDATION_ERRORS").
 *
 * Callers should discriminate the error KIND on this string FIRST, and only
 * fetch structured details (below) when it is "VALIDATION_ERRORS".
 *
 * # Returns
 * Pointer to the reason-code string (do not free), or null if there is no
 * error or no classified reason.
 */
SZCONFIGTOOL_API const char *SzConfigTool_getLastErrorReasonCode(void);

/**
 * Get versioned, namespaced JSON details for the last error.
 *
 * Populated only when the last error is a validation-errors aggregate (reason
 * code "VALIDATION_ERRORS"); null otherwise. Payload shape:
 *   {"schema":"sz-configtool.validation-errors/v1",
 *    "failures":[{"field":...,"reasonCode":...,"offendingValue":...}]}
 * The "schema" string is the stability contract: a future field addition bumps
 * it to /v2 rather than silently breaking the parse.
 *
 * # Returns
 * Pointer to the JSON string (do not free), or null if the last error carries
 * no structured details.
 */
SZCONFIGTOOL_API const char *SzConfigTool_getLastErrorDetails(void);

/**
 * Library version string, e.g. "4.4.0-1" (NUL-terminated, static storage; do
 * not free). Equals the sz_configtool_lib crate version it was built from.
 */
SZCONFIGTOOL_API const char *SzConfigTool_getLibraryVersion(void);

/**
 * ABI version of this C interface. Compare against SZCONFIGTOOL_ABI_VERSION
 * from the header you compiled with; a mismatch means the header and the
 * loaded library are incompatible.
 */
SZCONFIGTOOL_API int32_t SzConfigTool_getAbiVersion(void);

/**
 * Clear the last error
 */
SZCONFIGTOOL_API void SzConfigTool_clearLastError(void);

/* ============================================================================
 * Dynamic Invoke
 * ============================================================================ */

/**
 * Call any manifest function by name (see api/manifest in the source tree;
 * names are the manifest's snake_case names, e.g. "add_data_source").
 *
 * args_json is a JSON object keyed by the manifest argument names: a key that
 * is absent leaves the field / uses the library default, null clears a
 * tri-state field (an error for any other argument), and a value sets it.
 * Unknown keys are rejected. NULL args_json means "no arguments".
 *
 * On success (returnCode 0) the response is a JSON envelope:
 *   {"kind":"config"|"json"|"config_and_json"|"int"|"unit",
 *    "config":"<modified config, as a JSON string>",   (config kinds only)
 *    "result":<value>}                                (json/record/int kinds)
 * Decode "config" as a JSON string to recover the library's exact bytes.
 *
 * Errors: -1 for a NULL / non-UTF-8 name or config_json (no reason code);
 * otherwise -2 with SzConfigTool_getLastErrorReasonCode() set: INVALID_INPUT
 * for an unknown function, unparsable or non-object args_json, or unknown /
 * mistyped arguments; MISSING_FIELD for a missing required argument; the
 * library's own reason code for library errors; INTERNAL for internal faults.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_invoke(const char *name,
                                                                const char *config_json,
                                                                const char *args_json);

/* ============================================================================
 * Data Source Functions
 * ============================================================================ */

/**
 * Add a data source to the configuration
 *
 * # Safety
 * configJson and dataSourceCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addDataSource(const char *config_json,
                                                             const char *data_source_code);

/**
 * Delete a data source from the configuration
 *
 * # Safety
 * configJson and dataSourceCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteDataSource(const char *config_json,
                                                                const char *data_source_code);

/**
 * List all data sources in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listDataSources(const char *config_json);

/* ============================================================================
 * Attribute Functions
 * ============================================================================ */

/**
 * Add an attribute to the configuration
 *
 * # Safety
 * All string parameters must be valid null-terminated C strings
 * Optional parameters can be null
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addAttribute(const char *config_json,
                                                            const char *attribute_code,
                                                            const char *feature_code,
                                                            const char *element_code,
                                                            const char *attr_class,
                                                            const char *default_value,
                                                            const char *internal,
                                                            const char *required);

/**
 * Delete an attribute from the configuration
 *
 * # Safety
 * configJson and attributeCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteAttribute(const char *config_json,
                                                               const char *attribute_code);

/**
 * Get an attribute from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and attributeCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getAttribute(const char *config_json,
                                                            const char *attribute_code);

/**
 * List all attributes in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listAttributes(const char *config_json);

/**
 * Set (update) an attribute's properties
 *
 * # Safety
 * configJson, attributeCode, and updatesJson must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setAttribute(const char *config_json,
                                                       const char *attribute_code,
                                                       const char *updates_json);

/* ============================================================================
 * Feature Functions (Phase 1: read-only)
 * ============================================================================ */

/**
 * Get a feature from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and featureCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getFeature(const char *config_json,
                                                          const char *feature_code);

/**
 * List all features in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listFeatures(const char *config_json);

/* ============================================================================
 * Element Functions (Phase 1: read-only)
 * ============================================================================ */

/**
 * Get an element from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and elementCode must be valid null-terminated C strings
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getElement(const char *config_json,
                                                          const char *element_code);

/**
 * List all elements in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listElements(const char *config_json);

/* ============================================================================
 * Standardize Function Operations
 * ============================================================================ */

/**
 * List all standardize functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listStandardizeFunctions(const char *config_json);

/**
 * Delete a standardize function (fails if any standardize call still uses it;
 * see SzConfigTool_deleteStandardizeFunctionCascade)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteStandardizeFunction(const char *config_json,
                                                                  const char *sfunc_code);

/**
 * Get a standardize function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getStandardizeFunction(const char *config_json,
                                                               const char *sfunc_code);

/**
 * Set/update a standardize function from a JSON object with keys "CONNECT_STR", "SFUNC_DESC", "LANGUAGE"
 * (camelCase aliases connectStr/description/language accepted).
 * CONNECT_STR is tri-state: absent leaves it, JSON null clears it (writes
 * null), a string sets it. For the other keys a null or absent value leaves
 * the stored value untouched. Give ONE spelling per field (key or alias).
 * A non-object, an unknown key (ANON_SUPPORT is not accepted here), a non-string non-null value,
 * or both spellings of one field fails with returnCode -2 and reason code
 * INVALID_INPUT; unparsable updates_json is returnCode -3.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setStandardizeFunctionWithJson(const char *config_json,
                                                                        const char *sfunc_code,
                                                                        const char *updates_json);

/* ============================================================================
 * Expression Function Operations
 * ============================================================================ */

/**
 * List all expression functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listExpressionFunctions(const char *config_json);

/**
 * Delete an expression function (fails if any expression call still uses it;
 * see SzConfigTool_deleteExpressionFunctionCascade)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteExpressionFunction(const char *config_json,
                                                                 const char *efunc_code);

/**
 * Get an expression function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getExpressionFunction(const char *config_json,
                                                              const char *efunc_code);

/**
 * Set/update an expression function from a JSON object with keys "CONNECT_STR", "EFUNC_DESC", "LANGUAGE"
 * (camelCase aliases connectStr/description/language accepted).
 * CONNECT_STR is tri-state: absent leaves it, JSON null clears it (writes
 * null), a string sets it. For the other keys a null or absent value leaves
 * the stored value untouched. Give ONE spelling per field (key or alias).
 * A non-object, an unknown key (ANON_SUPPORT is not accepted here), a non-string non-null value,
 * or both spellings of one field fails with returnCode -2 and reason code
 * INVALID_INPUT; unparsable updates_json is returnCode -3.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setExpressionFunctionWithJson(const char *config_json,
                                                                       const char *efunc_code,
                                                                       const char *updates_json);

/* ============================================================================
 * Comparison Function Operations
 * ============================================================================ */

/**
 * List all comparison functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listComparisonFunctions(const char *config_json);

/**
 * Get a comparison function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getComparisonFunction(const char *config_json,
                                                              const char *cfunc_code);

/**
 * Set/update a comparison function from a JSON object with keys "CONNECT_STR", "CFUNC_DESC", "LANGUAGE", "ANON_SUPPORT"
 * (camelCase aliases connectStr/description/language/anonSupport accepted).
 * CONNECT_STR is tri-state: absent leaves it, JSON null clears it (writes
 * null), a string sets it. For the other keys a null or absent value leaves
 * the stored value untouched. Give ONE spelling per field (key or alias).
 * A non-object, an unknown key, a non-string non-null value,
 * or both spellings of one field fails with returnCode -2 and reason code
 * INVALID_INPUT; unparsable updates_json is returnCode -3.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setComparisonFunctionWithJson(const char *config_json,
                                                                       const char *cfunc_code,
                                                                       const char *updates_json);

/* ============================================================================
 * Matching Function Operations
 * ============================================================================ */

/**
 * List all matching functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listMatchingFunctions(const char *config_json);

/**
 * Get a matching function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getMatchingFunction(const char *config_json,
                                                            const char *mfunc_code);


/* ============================================================================
 * Distinct Function Operations
 * ============================================================================ */

/**
 * List all distinct functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listDistinctFunctions(const char *config_json);

/**
 * Get a distinct function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getDistinctFunction(const char *config_json,
                                                            const char *dfunc_code);

/**
 * Set/update a distinct function from a JSON object with keys "CONNECT_STR", "DFUNC_DESC", "LANGUAGE", "ANON_SUPPORT"
 * (camelCase aliases connectStr/description/language/anonSupport accepted).
 * CONNECT_STR is tri-state: absent leaves it, JSON null clears it (writes
 * null), a string sets it. For the other keys a null or absent value leaves
 * the stored value untouched. Give ONE spelling per field (key or alias).
 * A non-object, an unknown key, a non-string non-null value,
 * or both spellings of one field fails with returnCode -2 and reason code
 * INVALID_INPUT; unparsable updates_json is returnCode -3.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setDistinctFunctionWithJson(const char *config_json,
                                                                     const char *dfunc_code,
                                                                     const char *updates_json);

/* ============================================================================
 * Candidate Function Operations
 * ============================================================================ */

/**
 * List all candidate functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listCandidateFunctions(const char *config_json);

/**
 * Get a candidate function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getCandidateFunction(const char *config_json,
                                                             const char *rtype_code);


/* ============================================================================
 * Validation Function Operations
 * ============================================================================ */

/**
 * List all validation functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listValidationFunctions(const char *config_json);

/**
 * Get a validation function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getValidationFunction(const char *config_json,
                                                              const char *attr_code);


/* ============================================================================
 * Scoring Function Operations
 * ============================================================================ */

/**
 * List all scoring functions (returns JSON array string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listScoringFunctions(const char *config_json);

/**
 * Get a scoring function (returns JSON object string)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getScoringFunction(const char *config_json,
                                                           const char *rtype_code);


/* ============================================================================
 * Batch 1-4: System, Generic Plans, Rules, Config Sections
 * ============================================================================ */

/*
 * Set/update a fragment from JSON. The updates object is tri-state per field
 * (ERFRAG_SOURCE, ERFRAG_DESC): an absent key leaves the stored value untouched,
 * an explicit JSON null clears it (writes null), and a string sets it. Clearing
 * ERFRAG_SOURCE also clears ERFRAG_DEPENDS to null.
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setFragmentWithJson(const char *config_json, const char *fragment_code, const char *updates_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_cloneGenericPlan(const char *config_json, const char *source_code, const char *new_code, const char *new_desc);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setGenericPlan(const char *config_json, const char *gplan_code, const char *gplan_desc);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listGenericPlans(const char *config_json, const char *filter_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getThreshold(const char *config_json, int64_t threshold_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listSystemParameters(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setSystemParameterWithJson(const char *config_json, const char *param_name, const char *param_value_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getVersion(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getCompatibilityVersion(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_updateCompatibilityVersion(const char *config_json, const char *new_version);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_updateFeatureVersion(const char *config_json, const char *version);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_verifyCompatibilityVersion(const char *config_json, const char *expected_version);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addConfigSection(const char *config_json, const char *section_name);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_removeConfigSection(const char *config_json, const char *section_name);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getConfigSection(const char *config_json, const char *section_name, const char *filter_json);
/* Returns JSON boolean "true"/"false": whether the section is empty (null or []); errors if the section is missing. */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_configSectionIsEmpty(const char *config_json, const char *section_name);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listConfigSections(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addConfigSectionField(const char *config_json, const char *section_name, const char *field_name, const char *field_value_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_removeConfigSectionField(const char *config_json, const char *section_name, const char *field_name);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addRule(const char *config_json, const char *rule_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteRule(const char *config_json, const char *rule_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getRule(const char *config_json, const char *code_or_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listRules(const char *config_json);
/*
 * Set/update a rule from JSON. The fragment, disqualifier and tier fields are
 * tri-state: an absent key leaves the stored value untouched, an explicit JSON
 * null clears the column (writes null), and a value sets it. (The direct-arg
 * function wrappers below cannot express a null-clear; this JSON API can.)
 */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setRule(const char *config_json, const char *rule_code, const char *rule_json);


/* ============================================================================
 * Comparison Function Operations (Batch 5c)
 * ============================================================================ */

/* Direct-arg add: connect_str NULL stores JSON null; a non-null pointer
 * (including "") stores that value. */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addComparisonFunction(const char *config_json, const char *cfunc_code, const char *connect_str, const char *cfunc_desc, const char *language, const char *anon_support);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteComparisonFunction(const char *config_json, const char *cfunc_code);
/* Direct-arg set: connect_str NULL leaves the stored value untouched; a non-null
 * pointer (including "") sets it. This form cannot clear a value to null (use
 * SzConfigTool_set*FunctionWithJson for that). */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setComparisonFunction(const char *config_json, const char *cfunc_code, const char *connect_str, const char *cfunc_desc, const char *language, const char *anon_support);

/* ----------------------------------------------------------------------------
 * Standardize / Expression Function Operations (direct-arg forms)
 *
 * Direct-arg add: connect_str NULL stores JSON null; a non-null pointer
 *   (including "") stores that value.
 * Direct-arg set: connect_str NULL leaves the stored value untouched; a non-null
 *   pointer (including "") sets it. This form cannot clear a value to null (use
 *   SzConfigTool_set*FunctionWithJson for that).
 * ---------------------------------------------------------------------------- */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addStandardizeFunction(const char *config_json, const char *sfunc_code, const char *connect_str, const char *sfunc_desc, const char *language);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setStandardizeFunction(const char *config_json, const char *sfunc_code, const char *connect_str, const char *sfunc_desc, const char *language);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addExpressionFunction(const char *config_json, const char *efunc_code, const char *connect_str, const char *efunc_desc, const char *language);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setExpressionFunction(const char *config_json, const char *efunc_code, const char *connect_str, const char *efunc_desc, const char *language);

/* ============================================================================
 * Standardize Call Operations (Batch 6a)
 * ============================================================================ */

SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addStandardizeCall(const char *config_json, const char *ftype_code, const char *felem_code, int64_t exec_order, const char *sfunc_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteStandardizeCall(const char *config_json, int64_t sfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getStandardizeCall(const char *config_json, int64_t sfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listStandardizeCalls(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setStandardizeCall(const char *config_json, int64_t sfcall_id, const char *updates_json);

/* ============================================================================
 * Threshold Operations (Batch 7)
 * ============================================================================ */

// Comparison Thresholds
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addComparisonThreshold(
    const char *config_json, 
    int64_t cfunc_id, 
    const char *cfunc_rtnval, 
    int64_t ftype_id,       // Negative = None
    int64_t exec_order,     // Negative = None
    int64_t same_score,     // Negative = None
    int64_t close_score,    // Negative = None
    int64_t likely_score,   // Negative = None
    int64_t plausible_score,  // Negative = None
    int64_t un_likely_score   // Negative = None
);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteComparisonThreshold(const char *config_json, int64_t cfrtn_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setComparisonThreshold(const char *config_json, int64_t cfrtn_id, const char *updates_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listComparisonThresholds(const char *config_json);

// Generic Thresholds
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addGenericThreshold(
    const char *config_json,
    const char *plan,
    const char *behavior,
    int64_t scoring_cap,
    int64_t candidate_cap,
    const char *send_to_redo,
    const char *feature  // NULL = "ALL"
);
// Validate a generic-threshold ADD without mutating the config. Returns the
// staged check as versioned JSON (schema "sz-configtool.generic-threshold-check/v1")
// in `response` with returnCode 0; returnCode is negative only for a
// boundary/internal error (null/invalid-UTF-8 argument or unparseable config).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_validateGenericThreshold(
    const char *config_json,
    const char *plan,
    const char *behavior,
    const char *send_to_redo,
    const char *feature  // NULL = "ALL"
);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteGenericThreshold(const char *config_json, const char *plan, const char *behavior, const char *feature);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setGenericThreshold(const char *config_json, int64_t gplan_id, const char *behavior, const char *updates_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listGenericThresholds(const char *config_json);


/* ============================================================================
 * Fragment & Data Source Operations (Batch 8)
 * ============================================================================ */

// Fragment Operations
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getFragment(const char *config_json, const char *code_or_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listFragments(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addFragment(const char *config_json, const char *fragment_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteFragment(const char *config_json, const char *fragment_code);

// Data Source Operations
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getDataSource(const char *config_json, const char *code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setDataSource(const char *config_json, const char *code, const char *updates_json);


/* ============================================================================
 * Feature & Element Operations (Batch 9)
 * ============================================================================ */

// Feature Operations
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addFeature(const char *config_json, const char *feature_code, const char *feature_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteFeature(const char *config_json, const char *feature_code_or_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setFeature(const char *config_json, const char *feature_code_or_id, const char *updates_json);

// Element Operations
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addElement(const char *config_json, const char *element_code, const char *element_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteElement(const char *config_json, const char *element_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setElement(const char *config_json, const char *element_code, const char *updates_json);


/* ============================================================================
 * Call Operations (Batch 10 & 11)
 * ============================================================================ */

// Expression Call Operations (Batch 10)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addExpressionCall(const char *config_json,
                                                          const char *ftype_code,
                                                          const char *felem_code,
                                                          int64_t exec_order,
                                                          const char *efunc_code,
                                                          const char *element_list_json,
                                                          const char *expression_feature,
                                                          const char *is_virtual);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteExpressionCall(const char *config_json, int64_t efcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getExpressionCall(const char *config_json, int64_t efcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listExpressionCalls(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setExpressionCall(const char *config_json, int64_t efcall_id, const char *updates_json);

// Comparison Call Operations (Batch 10)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addComparisonCall(const char *config_json,
                                                          const char *ftype_code,
                                                          const char *cfunc_code,
                                                          const char *element_list_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteComparisonCall(const char *config_json, int64_t cfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getComparisonCall(const char *config_json, int64_t cfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listComparisonCalls(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setComparisonCall(const char *config_json, int64_t cfcall_id, const char *updates_json);

// Distinct Call Operations (Batch 11)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addDistinctCall(const char *config_json,
                                                        const char *ftype_code,
                                                        const char *dfunc_code,
                                                        const char *element_list_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteDistinctCall(const char *config_json, int64_t dfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getDistinctCall(const char *config_json, int64_t dfcall_id);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listDistinctCalls(const char *config_json);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setDistinctCall(const char *config_json, int64_t dfcall_id, const char *updates_json);


/* ============================================================================
 * Function Type Operations (Batch 12, 13, 14)
 * ============================================================================ */

// Matching Function Operations (Batch 12 - Placeholders)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addMatchingFunction(const char *config_json,
                                                            const char *rtype_code,
                                                            const char *matching_func);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteMatchingFunction(const char *config_json, const char *rtype_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setMatchingFunction(const char *config_json,
                                                            const char *rtype_code,
                                                            const char *matching_func);

// Distinct Function Operations (Batch 12)
/* Direct-arg add: connect_str NULL stores JSON null; a non-null pointer
 * (including "") stores that value. A blank connect_str is accepted. */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addDistinctFunction(const char *config_json,
                                                            const char *dfunc_code,
                                                            const char *connect_str,
                                                            const char *dfunc_desc,
                                                            const char *language);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteDistinctFunction(const char *config_json, const char *dfunc_code);
/* Direct-arg set: connect_str NULL leaves the stored value untouched; a non-null
 * pointer (including "") sets it. This form cannot clear a value to null (use
 * SzConfigTool_set*FunctionWithJson for that). */
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setDistinctFunction(const char *config_json,
                                                            const char *dfunc_code,
                                                            const char *connect_str,
                                                            const char *dfunc_desc,
                                                            const char *language);

// Candidate Function Operations (Batch 13 - Placeholders)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addCandidateFunction(const char *config_json,
                                                             const char *rtype_code,
                                                             const char *candidate_func);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteCandidateFunction(const char *config_json, const char *rtype_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setCandidateFunction(const char *config_json,
                                                             const char *rtype_code,
                                                             const char *candidate_func);

// Validation Function Operations (Batch 13 - Placeholders)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addValidationFunction(const char *config_json,
                                                              const char *attr_code,
                                                              const char *validation_func);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteValidationFunction(const char *config_json, const char *attr_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setValidationFunction(const char *config_json,
                                                              const char *attr_code,
                                                              const char *validation_func);

// Scoring Function Operations (Batch 14 - Placeholders)
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addScoringFunction(const char *config_json,
                                                           const char *rtype_code,
                                                           const char *scoring_func);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteScoringFunction(const char *config_json, const char *rtype_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setScoringFunction(const char *config_json,
                                                           const char *rtype_code,
                                                           const char *scoring_func);

// Wave 4A additions (#38): feature-element mutators, settings, cascade deletes

// Append a feature-element mapping (CFG_FBOM row). options_json may be NULL, or a
// JSON object carrying displayLevel (int), displayDelim (string), derived (Yes/No).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addElementToFeature(const char *config_json,
                                                            const char *feature_code,
                                                            const char *element_code,
                                                            const char *options_json);

// Remove a single feature-element mapping (CFG_FBOM row).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteElementFromFeature(const char *config_json,
                                                                 const char *feature_code,
                                                                 const char *element_code);

// Create or overwrite a named setting under G2_CONFIG.SETTINGS (name is uppercased).
// The value param is a JSON-encoded value: it is parsed and stored verbatim as its
// typed value (e.g. "3" is stored as the number 3). A bare string must be quoted JSON
// (e.g. "\"hello\""). Invalid JSON returns an error (no string fallback).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_setSetting(const char *config_json,
                                                   const char *name,
                                                   const char *value);

// Delete a function and all of its dependent rows (cascade).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteComparisonFunctionCascade(const char *config_json,
                                                                        const char *cfunc_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteExpressionFunctionCascade(const char *config_json,
                                                                        const char *efunc_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteStandardizeFunctionCascade(const char *config_json,
                                                                         const char *sfunc_code);

// Wave 4B additions (#40): by-feature call gets and code-addressed call-element deletes.
//
// Get the call bound to a feature (scans CFG_*CALL by FTYPE_ID). This replaces
// the previous, incorrect practice of using the feature id directly as a call
// id. Standardize/expression error if the feature has more than one such call
// (address those by id via the existing get*Call functions).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getComparisonCallByFeature(const char *config_json,
                                                                   const char *feature_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getDistinctCallByFeature(const char *config_json,
                                                                 const char *feature_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getStandardizeCallByFeature(const char *config_json,
                                                                    const char *feature_code);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_getExpressionCallByFeature(const char *config_json,
                                                                   const char *feature_code);

// Delete a call element addressed by code; EXEC_ORDER is derived internally, so
// (unlike the removed Rust exec_order argument) no execution order is passed.
// NOTE: this is NEW FFI surface — there were no prior call-element delete
// wrappers — so it does not break an existing ABI. Comparison/distinct address
// the call by feature code (0-or-1 call per feature); expression addresses the
// call by its EFCALL_ID because expression calls are many-per-feature.
// element_feature is OPTIONAL (may be NULL): when one call carries the same
// element under multiple element-features, pass the element's feature code to
// disambiguate to the correct BOM row (matches Python). Pass NULL when the
// (call, element) pair is unambiguous.
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteComparisonCallElement(const char *config_json,
                                                                    const char *feature_code,
                                                                    const char *element_code,
                                                                    const char *element_feature);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteDistinctCallElement(const char *config_json,
                                                                  const char *feature_code,
                                                                  const char *element_code,
                                                                  const char *element_feature);
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteExpressionCallElement(const char *config_json,
                                                                    int64_t efcall_id,
                                                                    const char *element_code,
                                                                    const char *element_feature);

// Wave 6 additions (#42, #43): API-surface helpers. All additive.
//
// Add a behavior override (CFG_FBOVR row) for a feature and usage type.
// `behavior` is a behavior code such as "F1E" or "NAME".
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_addBehaviorOverride(const char *config_json,
                                                            const char *feature_code,
                                                            const char *usage_type,
                                                            const char *behavior);

// Delete the behavior override for a feature and usage type.
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_deleteBehaviorOverride(const char *config_json,
                                                               const char *feature_code,
                                                               const char *usage_type);

// List behavior overrides as raw CFG_FBOVR rows (thin projection).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listBehaviorOverrides(const char *config_json);

// List behavior overrides in the resolved display shape: a JSON array of
// { "feature", "usageType", "behavior" } objects, sorted by (FTYPE_ID,
// UTYPE_CODE). Richer than SzConfigTool_listBehaviorOverrides (raw rows).
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_listBehaviorOverridesResolved(const char *config_json);

// Validate the top-level structure of a config document (structure-only:
// G2_CONFIG must be an object; any present CFG_* section must be an array).
// Returns "OK" with return code 0 on success, or an error result otherwise.
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_validateConfig(const char *config_json);

// Render a config document to its canonical export form: recursive object-key
// sort (Python sort_keys=True semantics) then pretty-print at `indent` spaces
// per level. `indent` is required; a negative value is treated as 0.
SZCONFIGTOOL_API struct SzConfigTool_result SzConfigTool_renderConfig(const char *config_json,
                                                     int64_t indent);

#ifdef __cplusplus
}
#endif

#endif /* LIBSZCONFIGTOOL_H */
