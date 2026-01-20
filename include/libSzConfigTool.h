#ifndef LIBSZCONFIGTOOL_H
#define LIBSZCONFIGTOOL_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Result structure for operations that return modified configuration JSON
 */
typedef struct SzConfigTool_result {
  /**
   * Modified configuration JSON (caller must free with SzConfigTool_free)
   */
  char *response;
  /**
   * Return code: 0 = success, negative = error
   */
  int64_t return_code;
} SzConfigTool_result;

/**
 * Free memory allocated by this library
 *
 * # Safety
 * ptr must be a valid pointer previously returned by this library, or null
 */
void SzConfigTool_free(char *ptr);

/**
 * Get the last error message
 *
 * # Returns
 * Pointer to error string (do not free), or null if no error
 */
const char *SzConfigTool_getLastError(void);

/**
 * Get the last error code
 *
 * # Returns
 * Error code (0 = no error, negative = error
 */
int64_t SzConfigTool_getLastErrorCode(void);

/**
 * Clear the last error
 */
void SzConfigTool_clearLastError(void);

/* ============================================================================
 * Data Source Functions
 * ============================================================================ */

/**
 * Add a data source to the configuration
 *
 * # Safety
 * configJson and dataSourceCode must be valid null-terminated C strings
 */
struct SzConfigTool_result SzConfigTool_addDataSource(const char *config_json,
                                                             const char *data_source_code);

/**
 * Delete a data source from the configuration
 *
 * # Safety
 * configJson and dataSourceCode must be valid null-terminated C strings
 */
struct SzConfigTool_result SzConfigTool_deleteDataSource(const char *config_json,
                                                                const char *data_source_code);

/**
 * List all data sources in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
struct SzConfigTool_result SzConfigTool_listDataSources(const char *config_json);

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
struct SzConfigTool_result SzConfigTool_addAttribute(const char *config_json,
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
struct SzConfigTool_result SzConfigTool_deleteAttribute(const char *config_json,
                                                               const char *attribute_code);

/**
 * Get an attribute from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and attributeCode must be valid null-terminated C strings
 */
struct SzConfigTool_result SzConfigTool_getAttribute(const char *config_json,
                                                            const char *attribute_code);

/**
 * List all attributes in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
struct SzConfigTool_result SzConfigTool_listAttributes(const char *config_json);

/* ============================================================================
 * Feature Functions (Phase 1: read-only)
 * ============================================================================ */

/**
 * Get a feature from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and featureCode must be valid null-terminated C strings
 */
struct SzConfigTool_result SzConfigTool_getFeature(const char *config_json,
                                                          const char *feature_code);

/**
 * List all features in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
struct SzConfigTool_result SzConfigTool_listFeatures(const char *config_json);

/* ============================================================================
 * Element Functions (Phase 1: read-only)
 * ============================================================================ */

/**
 * Get an element from the configuration (returns JSON object string)
 *
 * # Safety
 * configJson and elementCode must be valid null-terminated C strings
 */
struct SzConfigTool_result SzConfigTool_getElement(const char *config_json,
                                                          const char *element_code);

/**
 * List all elements in the configuration (returns JSON array string)
 *
 * # Safety
 * configJson must be a valid null-terminated C string
 */
struct SzConfigTool_result SzConfigTool_listElements(const char *config_json);

/* ============================================================================
 * Standardize Function Operations
 * ============================================================================ */

/**
 * List all standardize functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listStandardizeFunctions(const char *config_json);

/**
 * Get a standardize function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getStandardizeFunction(const char *config_json,
                                                               const char *sfunc_code);

/**
 * Set/update a standardize function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setStandardizeFunctionWithJson(const char *config_json,
                                                                        const char *sfunc_code,
                                                                        const char *updates_json);

/* ============================================================================
 * Expression Function Operations
 * ============================================================================ */

/**
 * List all expression functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listExpressionFunctions(const char *config_json);

/**
 * Get an expression function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getExpressionFunction(const char *config_json,
                                                              const char *efunc_code);

/**
 * Set/update an expression function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setExpressionFunctionWithJson(const char *config_json,
                                                                       const char *efunc_code,
                                                                       const char *updates_json);

/* ============================================================================
 * Comparison Function Operations
 * ============================================================================ */

/**
 * List all comparison functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listComparisonFunctions(const char *config_json);

/**
 * Get a comparison function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getComparisonFunction(const char *config_json,
                                                              const char *cfunc_code);

/**
 * Set/update a comparison function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setComparisonFunctionWithJson(const char *config_json,
                                                                       const char *cfunc_code,
                                                                       const char *updates_json);

/* ============================================================================
 * Matching Function Operations
 * ============================================================================ */

/**
 * List all matching functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listMatchingFunctions(const char *config_json);

/**
 * Get a matching function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getMatchingFunction(const char *config_json,
                                                            const char *mfunc_code);

/**
 * Set/update a matching function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setMatchingFunctionWithJson(const char *config_json,
                                                                     const char *mfunc_code,
                                                                     const char *updates_json);

/* ============================================================================
 * Distinct Function Operations
 * ============================================================================ */

/**
 * List all distinct functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listDistinctFunctions(const char *config_json);

/**
 * Get a distinct function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getDistinctFunction(const char *config_json,
                                                            const char *dfunc_code);

/**
 * Set/update a distinct function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setDistinctFunctionWithJson(const char *config_json,
                                                                     const char *dfunc_code,
                                                                     const char *updates_json);

/* ============================================================================
 * Candidate Function Operations
 * ============================================================================ */

/**
 * List all candidate functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listCandidateFunctions(const char *config_json);

/**
 * Get a candidate function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getCandidateFunction(const char *config_json,
                                                             const char *rtype_code);

/**
 * Set/update a candidate function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setCandidateFunctionWithJson(const char *config_json,
                                                                      const char *rtype_code,
                                                                      const char *updates_json);

/* ============================================================================
 * Validation Function Operations
 * ============================================================================ */

/**
 * List all validation functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listValidationFunctions(const char *config_json);

/**
 * Get a validation function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getValidationFunction(const char *config_json,
                                                              const char *attr_code);

/**
 * Set/update a validation function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setValidationFunctionWithJson(const char *config_json,
                                                                       const char *attr_code,
                                                                       const char *updates_json);

/* ============================================================================
 * Scoring Function Operations
 * ============================================================================ */

/**
 * List all scoring functions (returns JSON array string)
 */
struct SzConfigTool_result SzConfigTool_listScoringFunctions(const char *config_json);

/**
 * Get a scoring function (returns JSON object string)
 */
struct SzConfigTool_result SzConfigTool_getScoringFunction(const char *config_json,
                                                           const char *rtype_code);

/**
 * Set/update a scoring function with JSON parameters
 */
struct SzConfigTool_result SzConfigTool_setScoringFunctionWithJson(const char *config_json,
                                                                    const char *rtype_code,
                                                                    const char *updates_json);

/* ============================================================================
 * Batch 1-4: System, Generic Plans, Rules, Config Sections
 * ============================================================================ */

struct SzConfigTool_result SzConfigTool_setFragmentWithJson(const char *config_json, const char *fragment_code, const char *updates_json);
struct SzConfigTool_result SzConfigTool_cloneGenericPlan(const char *config_json, const char *source_code, const char *new_code, const char *new_desc);
struct SzConfigTool_result SzConfigTool_setGenericPlan(const char *config_json, const char *gplan_code, const char *gplan_desc, const char *updates_json);
struct SzConfigTool_result SzConfigTool_listGenericPlans(const char *config_json, const char *filter_code);
struct SzConfigTool_result SzConfigTool_addToSsnLast4Hash(const char *config_json, const char *name);
struct SzConfigTool_result SzConfigTool_deleteFromSsnLast4Hash(const char *config_json, const char *name);
struct SzConfigTool_result SzConfigTool_getThreshold(const char *config_json, int64_t threshold_id);
struct SzConfigTool_result SzConfigTool_listSystemParameters(const char *config_json);
struct SzConfigTool_result SzConfigTool_setSystemParameterWithJson(const char *config_json, const char *param_name, const char *param_value_json);
struct SzConfigTool_result SzConfigTool_getVersion(const char *config_json);
struct SzConfigTool_result SzConfigTool_getCompatibilityVersion(const char *config_json);
struct SzConfigTool_result SzConfigTool_updateCompatibilityVersion(const char *config_json, int64_t new_version);
struct SzConfigTool_result SzConfigTool_updateFeatureVersion(const char *config_json, int64_t new_version);
struct SzConfigTool_result SzConfigTool_verifyCompatibilityVersion(const char *config_json, int64_t required_version);
struct SzConfigTool_result SzConfigTool_addConfigSection(const char *config_json, const char *section_name, const char *section_json);
struct SzConfigTool_result SzConfigTool_removeConfigSection(const char *config_json, const char *section_name);
struct SzConfigTool_result SzConfigTool_getConfigSection(const char *config_json, const char *section_name, const char *filter_json);
struct SzConfigTool_result SzConfigTool_listConfigSections(const char *config_json);
struct SzConfigTool_result SzConfigTool_addConfigSectionField(const char *config_json, const char *section_name, const char *field_name, const char *field_value_json);
struct SzConfigTool_result SzConfigTool_removeConfigSectionField(const char *config_json, const char *section_name, const char *field_name);
struct SzConfigTool_result SzConfigTool_addRule(const char *config_json, const char *rule_json);
struct SzConfigTool_result SzConfigTool_deleteRule(const char *config_json, const char *rule_code);
struct SzConfigTool_result SzConfigTool_getRule(const char *config_json, const char *code_or_id);
struct SzConfigTool_result SzConfigTool_listRules(const char *config_json);
struct SzConfigTool_result SzConfigTool_setRule(const char *config_json, const char *rule_code, const char *rule_json);


/* ============================================================================
 * Comparison Function Operations (Batch 5c)
 * ============================================================================ */

struct SzConfigTool_result SzConfigTool_addComparisonFunction(const char *config_json, const char *cfunc_code, const char *connect_str, const char *cfunc_desc, const char *language, const char *anon_support);
struct SzConfigTool_result SzConfigTool_deleteComparisonFunction(const char *config_json, const char *cfunc_code);
struct SzConfigTool_result SzConfigTool_getComparisonFunction(const char *config_json, const char *cfunc_code);
struct SzConfigTool_result SzConfigTool_listComparisonFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setComparisonFunction(const char *config_json, const char *cfunc_code, const char *connect_str, const char *cfunc_desc, const char *language, const char *anon_support);

/* ============================================================================
 * Standardize Call Operations (Batch 6a)
 * ============================================================================ */

struct SzConfigTool_result SzConfigTool_addStandardizeCall(const char *config_json, const char *ftype_code, const char *felem_code, int64_t exec_order, const char *sfunc_code);
struct SzConfigTool_result SzConfigTool_deleteStandardizeCall(const char *config_json, int64_t sfcall_id);
struct SzConfigTool_result SzConfigTool_getStandardizeCall(const char *config_json, int64_t sfcall_id);
struct SzConfigTool_result SzConfigTool_listStandardizeCalls(const char *config_json, const char *ftype_code, const char *felem_code);
struct SzConfigTool_result SzConfigTool_setStandardizeCall(const char *config_json, int64_t sfcall_id, const char *updates_json);

/* ============================================================================
 * Threshold Operations (Batch 7)
 * ============================================================================ */

// Comparison Thresholds
struct SzConfigTool_result SzConfigTool_addComparisonThreshold(
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
struct SzConfigTool_result SzConfigTool_deleteComparisonThreshold(const char *config_json, int64_t cfrtn_id);
struct SzConfigTool_result SzConfigTool_setComparisonThreshold(const char *config_json, int64_t cfrtn_id, const char *updates_json);
struct SzConfigTool_result SzConfigTool_listComparisonThresholds(const char *config_json);

// Generic Thresholds
struct SzConfigTool_result SzConfigTool_addGenericThreshold(
    const char *config_json,
    const char *plan,
    const char *behavior,
    int64_t scoring_cap,
    int64_t candidate_cap,
    const char *send_to_redo,
    const char *feature  // NULL = "ALL"
);
struct SzConfigTool_result SzConfigTool_deleteGenericThreshold(const char *config_json, const char *plan, const char *behavior, const char *feature);
struct SzConfigTool_result SzConfigTool_setGenericThreshold(const char *config_json, int64_t gplan_id, const char *behavior, const char *updates_json);
struct SzConfigTool_result SzConfigTool_listGenericThresholds(const char *config_json);


/* ============================================================================
 * Fragment & Data Source Operations (Batch 8)
 * ============================================================================ */

// Fragment Operations
struct SzConfigTool_result SzConfigTool_getFragment(const char *config_json, const char *code_or_id);
struct SzConfigTool_result SzConfigTool_listFragments(const char *config_json);
struct SzConfigTool_result SzConfigTool_addFragment(const char *config_json, const char *fragment_json);
struct SzConfigTool_result SzConfigTool_deleteFragment(const char *config_json, const char *fragment_code);

// Data Source Operations
struct SzConfigTool_result SzConfigTool_getDataSource(const char *config_json, const char *code);
struct SzConfigTool_result SzConfigTool_setDataSource(const char *config_json, const char *code, const char *updates_json);


/* ============================================================================
 * Feature & Element Operations (Batch 9)
 * ============================================================================ */

// Feature Operations
struct SzConfigTool_result SzConfigTool_addFeature(const char *config_json, const char *feature_code, const char *feature_json);
struct SzConfigTool_result SzConfigTool_deleteFeature(const char *config_json, const char *feature_code_or_id);
struct SzConfigTool_result SzConfigTool_setFeature(const char *config_json, const char *feature_code_or_id, const char *updates_json);

// Element Operations
struct SzConfigTool_result SzConfigTool_addElement(const char *config_json, const char *element_code, const char *element_json);
struct SzConfigTool_result SzConfigTool_deleteElement(const char *config_json, const char *element_code);
struct SzConfigTool_result SzConfigTool_setElement(const char *config_json, const char *element_code, const char *updates_json);


/* ============================================================================
 * Call Operations (Batch 10 & 11)
 * ============================================================================ */

// Expression Call Operations (Batch 10)
struct SzConfigTool_result SzConfigTool_addExpressionCall(const char *config_json,
                                                          const char *ftype_code,
                                                          const char *felem_code,
                                                          int64_t exec_order,
                                                          const char *efunc_code,
                                                          const char *element_list_json,
                                                          const char *expression_feature,
                                                          const char *is_virtual);
struct SzConfigTool_result SzConfigTool_deleteExpressionCall(const char *config_json, int64_t efcall_id);
struct SzConfigTool_result SzConfigTool_getExpressionCall(const char *config_json, int64_t efcall_id);
struct SzConfigTool_result SzConfigTool_listExpressionCalls(const char *config_json);
struct SzConfigTool_result SzConfigTool_setExpressionCall(const char *config_json, int64_t efcall_id, const char *updates_json);

// Comparison Call Operations (Batch 10)
struct SzConfigTool_result SzConfigTool_addComparisonCall(const char *config_json,
                                                          const char *ftype_code,
                                                          const char *cfunc_code,
                                                          const char *element_list_json);
struct SzConfigTool_result SzConfigTool_deleteComparisonCall(const char *config_json, int64_t cfcall_id);
struct SzConfigTool_result SzConfigTool_getComparisonCall(const char *config_json, int64_t cfcall_id);
struct SzConfigTool_result SzConfigTool_listComparisonCalls(const char *config_json);
struct SzConfigTool_result SzConfigTool_setComparisonCall(const char *config_json, int64_t cfcall_id, const char *updates_json);

// Distinct Call Operations (Batch 11)
struct SzConfigTool_result SzConfigTool_addDistinctCall(const char *config_json,
                                                        const char *ftype_code,
                                                        const char *dfunc_code,
                                                        const char *element_list_json);
struct SzConfigTool_result SzConfigTool_deleteDistinctCall(const char *config_json, int64_t dfcall_id);
struct SzConfigTool_result SzConfigTool_getDistinctCall(const char *config_json, int64_t dfcall_id);
struct SzConfigTool_result SzConfigTool_listDistinctCalls(const char *config_json);
struct SzConfigTool_result SzConfigTool_setDistinctCall(const char *config_json, int64_t dfcall_id, const char *updates_json);


/* ============================================================================
 * Function Type Operations (Batch 12, 13, 14)
 * ============================================================================ */

// Matching Function Operations (Batch 12 - Placeholders)
struct SzConfigTool_result SzConfigTool_addMatchingFunction(const char *config_json,
                                                            const char *rtype_code,
                                                            const char *matching_func);
struct SzConfigTool_result SzConfigTool_deleteMatchingFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_getMatchingFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_listMatchingFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setMatchingFunction(const char *config_json,
                                                            const char *rtype_code,
                                                            const char *matching_func);

// Distinct Function Operations (Batch 12)
struct SzConfigTool_result SzConfigTool_addDistinctFunction(const char *config_json,
                                                            const char *dfunc_code,
                                                            const char *connect_str,
                                                            const char *dfunc_desc,
                                                            const char *language);
struct SzConfigTool_result SzConfigTool_deleteDistinctFunction(const char *config_json, const char *dfunc_code);
struct SzConfigTool_result SzConfigTool_getDistinctFunction(const char *config_json, const char *dfunc_code);
struct SzConfigTool_result SzConfigTool_listDistinctFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setDistinctFunction(const char *config_json,
                                                            const char *dfunc_code,
                                                            const char *connect_str,
                                                            const char *dfunc_desc,
                                                            const char *language);

// Candidate Function Operations (Batch 13 - Placeholders)
struct SzConfigTool_result SzConfigTool_addCandidateFunction(const char *config_json,
                                                             const char *rtype_code,
                                                             const char *candidate_func);
struct SzConfigTool_result SzConfigTool_deleteCandidateFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_getCandidateFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_listCandidateFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setCandidateFunction(const char *config_json,
                                                             const char *rtype_code,
                                                             const char *candidate_func);

// Validation Function Operations (Batch 13 - Placeholders)
struct SzConfigTool_result SzConfigTool_addValidationFunction(const char *config_json,
                                                              const char *attr_code,
                                                              const char *validation_func);
struct SzConfigTool_result SzConfigTool_deleteValidationFunction(const char *config_json, const char *attr_code);
struct SzConfigTool_result SzConfigTool_getValidationFunction(const char *config_json, const char *attr_code);
struct SzConfigTool_result SzConfigTool_listValidationFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setValidationFunction(const char *config_json,
                                                              const char *attr_code,
                                                              const char *validation_func);

// Scoring Function Operations (Batch 14 - Placeholders)
struct SzConfigTool_result SzConfigTool_addScoringFunction(const char *config_json,
                                                           const char *rtype_code,
                                                           const char *scoring_func);
struct SzConfigTool_result SzConfigTool_deleteScoringFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_getScoringFunction(const char *config_json, const char *rtype_code);
struct SzConfigTool_result SzConfigTool_listScoringFunctions(const char *config_json);
struct SzConfigTool_result SzConfigTool_setScoringFunction(const char *config_json,
                                                           const char *rtype_code,
                                                           const char *scoring_func);

#endif /* LIBSZCONFIGTOOL_H */
