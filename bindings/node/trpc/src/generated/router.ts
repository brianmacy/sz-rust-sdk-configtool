// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.
//
// One procedure per typed function. Functions returning a config are
// mutations; read-only ones are queries (send them with POST: the config
// is ~300KB, far too large for a GET URL).

import * as api from "sz-configtool";

import { szCall } from "../sz-call.js";
import { t } from "../trpc.js";
import * as schemas from "./schemas.js";

/** The configuration-tool router. */
export const configToolRouter = t.router({
  /**
   * Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
   */
  addAttribute: t.procedure
    .input(schemas.addAttributeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addAttribute(config, options);
      }),
    ),
  /**
   * Delete an attribute by code.
   */
  deleteAttribute: t.procedure
    .input(schemas.deleteAttributeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteAttribute(config, options);
      }),
    ),
  /**
   * Get one attribute's raw CFG_ATTR row by code.
   */
  getAttribute: t.procedure
    .input(schemas.getAttributeInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getAttribute(config, options);
      }),
    ),
  /**
   * List all attributes as camelCase summaries.
   */
  listAttributes: t.procedure
    .input(schemas.listAttributesInput)
    .query(({ input }) =>
      szCall(() => api.listAttributes(input.config)),
    ),
  /**
   * Update an attribute's internal / required / default value.
   */
  setAttribute: t.procedure
    .input(schemas.setAttributeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setAttribute(config, options);
      }),
    ),
  /**
   * Add a behavior override (CFG_FBOVR row) for a feature and usage type.
   */
  addBehaviorOverride: t.procedure
    .input(schemas.addBehaviorOverrideInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addBehaviorOverride(config, options);
      }),
    ),
  /**
   * Delete the behavior override for a feature and usage type.
   */
  deleteBehaviorOverride: t.procedure
    .input(schemas.deleteBehaviorOverrideInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteBehaviorOverride(config, options);
      }),
    ),
  /**
   * Get the raw CFG_FBOVR row for a feature and usage type.
   */
  getBehaviorOverride: t.procedure
    .input(schemas.getBehaviorOverrideInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getBehaviorOverride(config, options);
      }),
    ),
  /**
   * List the raw CFG_FBOVR rows sorted by FTYPE_ID.
   */
  listBehaviorOverrides: t.procedure
    .input(schemas.listBehaviorOverridesInput)
    .query(({ input }) =>
      szCall(() => api.listBehaviorOverrides(input.config)),
    ),
  /**
   * List behavior overrides as {feature, usageType, behavior} display records.
   */
  listBehaviorOverridesResolved: t.procedure
    .input(schemas.listBehaviorOverridesResolvedInput)
    .query(({ input }) =>
      szCall(() => api.listBehaviorOverridesResolved(input.config)),
    ),
  /**
   * Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with its element list (CFG_CFBOM rows).
   */
  addComparisonCall: t.procedure
    .input(schemas.addComparisonCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addComparisonCall(config, options);
      }),
    ),
  /**
   * Delete a comparison call by CFCALL_ID, cascading to its CFG_CFBOM rows.
   */
  deleteComparisonCall: t.procedure
    .input(schemas.deleteComparisonCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteComparisonCall(config, options);
      }),
    ),
  /**
   * Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
   */
  getComparisonCall: t.procedure
    .input(schemas.getComparisonCallInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getComparisonCall(config, options);
      }),
    ),
  /**
   * List all comparison calls with feature/function codes resolved and their ordered element lists.
   */
  listComparisonCalls: t.procedure
    .input(schemas.listComparisonCallsInput)
    .query(({ input }) =>
      szCall(() => api.listComparisonCalls(input.config)),
    ),
  /**
   * Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
   */
  addComparisonCallElement: t.procedure
    .input(schemas.addComparisonCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addComparisonCallElement(config, options);
      }),
    ),
  /**
   * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or feature code plus element code.
   */
  deleteComparisonCallElement: t.procedure
    .input(schemas.deleteComparisonCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteComparisonCallElement(config, options);
      }),
    ),
  /**
   * Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its element list (CFG_DFBOM rows).
   */
  addDistinctCall: t.procedure
    .input(schemas.addDistinctCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addDistinctCall(config, options);
      }),
    ),
  /**
   * Delete a distinct call by DFCALL_ID, cascading to its CFG_DFBOM rows.
   */
  deleteDistinctCall: t.procedure
    .input(schemas.deleteDistinctCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteDistinctCall(config, options);
      }),
    ),
  /**
   * Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
   */
  getDistinctCall: t.procedure
    .input(schemas.getDistinctCallInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getDistinctCall(config, options);
      }),
    ),
  /**
   * List all distinct calls with feature/function codes resolved and their ordered element lists.
   */
  listDistinctCalls: t.procedure
    .input(schemas.listDistinctCallsInput)
    .query(({ input }) =>
      szCall(() => api.listDistinctCalls(input.config)),
    ),
  /**
   * Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
   */
  addDistinctCallElement: t.procedure
    .input(schemas.addDistinctCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addDistinctCallElement(config, options);
      }),
    ),
  /**
   * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature code plus element code.
   */
  deleteDistinctCallElement: t.procedure
    .input(schemas.deleteDistinctCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteDistinctCallElement(config, options);
      }),
    ),
  /**
   * Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
   */
  addExpressionCall: t.procedure
    .input(schemas.addExpressionCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addExpressionCall(config, options);
      }),
    ),
  /**
   * Delete an expression call by EFCALL_ID, cascading its CFG_EFBOM rows.
   */
  deleteExpressionCall: t.procedure
    .input(schemas.deleteExpressionCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteExpressionCall(config, options);
      }),
    ),
  /**
   * Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
   */
  getExpressionCall: t.procedure
    .input(schemas.getExpressionCallInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getExpressionCall(config, options);
      }),
    ),
  /**
   * List all expression calls with resolved codes and element lists.
   */
  listExpressionCalls: t.procedure
    .input(schemas.listExpressionCallsInput)
    .query(({ input }) =>
      szCall(() => api.listExpressionCalls(input.config)),
    ),
  /**
   * Add one CFG_EFBOM row to an expression call, addressed by raw ids.
   */
  addExpressionCallElement: t.procedure
    .input(schemas.addExpressionCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addExpressionCallElement(config, options);
      }),
    ),
  /**
   * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
   */
  deleteExpressionCallElement: t.procedure
    .input(schemas.deleteExpressionCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteExpressionCallElement(config, options);
      }),
    ),
  /**
   * Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or an element.
   */
  addStandardizeCall: t.procedure
    .input(schemas.addStandardizeCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addStandardizeCall(config, options);
      }),
    ),
  /**
   * Delete a standardize call by SFCALL_ID.
   */
  deleteStandardizeCall: t.procedure
    .input(schemas.deleteStandardizeCallInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteStandardizeCall(config, options);
      }),
    ),
  /**
   * Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
   */
  getStandardizeCall: t.procedure
    .input(schemas.getStandardizeCallInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getStandardizeCall(config, options);
      }),
    ),
  /**
   * List all standardize calls with resolved codes.
   */
  listStandardizeCalls: t.procedure
    .input(schemas.listStandardizeCallsInput)
    .query(({ input }) =>
      szCall(() => api.listStandardizeCalls(input.config)),
    ),
  /**
   * Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
   */
  addStandardizeCallElement: t.procedure
    .input(schemas.addStandardizeCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addStandardizeCallElement(config, options);
      }),
    ),
  /**
   * Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
   */
  deleteStandardizeCallElement: t.procedure
    .input(schemas.deleteStandardizeCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteStandardizeCallElement(config, options);
      }),
    ),
  /**
   * Add a new, empty top-level section (an empty array) to G2_CONFIG.
   */
  addConfigSection: t.procedure
    .input(schemas.addConfigSectionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addConfigSection(config, options);
      }),
    ),
  /**
   * Remove a top-level section from G2_CONFIG.
   */
  removeConfigSection: t.procedure
    .input(schemas.removeConfigSectionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.removeConfigSection(config, options);
      }),
    ),
  /**
   * Get the raw items of a top-level section, optionally filtered by a case-insensitive substring.
   */
  getConfigSection: t.procedure
    .input(schemas.getConfigSectionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getConfigSection(config, options);
      }),
    ),
  /**
   * Report whether a top-level section is empty (null or []).
   */
  configSectionIsEmpty: t.procedure
    .input(schemas.configSectionIsEmptyInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.configSectionIsEmpty(config, options);
      }),
    ),
  /**
   * List the names of all top-level G2_CONFIG keys.
   */
  listConfigSections: t.procedure
    .input(schemas.listConfigSectionsInput)
    .query(({ input }) =>
      szCall(() => api.listConfigSections(input.config)),
    ),
  /**
   * Remove a field from every item of an array section, returning how many items had it.
   */
  removeConfigSectionField: t.procedure
    .input(schemas.removeConfigSectionFieldInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.removeConfigSectionField(config, options);
      }),
    ),
  /**
   * Add a field to every item of an array section that lacks it, returning existed/updated counts.
   */
  addConfigSectionField: t.procedure
    .input(schemas.addConfigSectionFieldInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addConfigSectionField(config, options);
      }),
    ),
  /**
   * Add a data source (CFG_DSRC row) to the configuration.
   */
  addDataSource: t.procedure
    .input(schemas.addDataSourceInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addDataSource(config, options);
      }),
    ),
  /**
   * Delete a data source by code.
   */
  deleteDataSource: t.procedure
    .input(schemas.deleteDataSourceInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteDataSource(config, options);
      }),
    ),
  /**
   * Get one data source's raw CFG_DSRC row by code.
   */
  getDataSource: t.procedure
    .input(schemas.getDataSourceInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getDataSource(config, options);
      }),
    ),
  /**
   * List all data sources as {id, dataSource} summaries.
   */
  listDataSources: t.procedure
    .input(schemas.listDataSourcesInput)
    .query(({ input }) =>
      szCall(() => api.listDataSources(input.config)),
    ),
  /**
   * Update a data source's retention level.
   */
  setDataSource: t.procedure
    .input(schemas.setDataSourceInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setDataSource(config, options);
      }),
    ),
  /**
   * Add an element (CFG_FELEM row).
   */
  addElement: t.procedure
    .input(schemas.addElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addElement(config, options);
      }),
    ),
  /**
   * Delete an element that no feature uses.
   */
  deleteElement: t.procedure
    .input(schemas.deleteElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteElement(config, options);
      }),
    ),
  /**
   * Get one element as a display summary by code.
   */
  getElement: t.procedure
    .input(schemas.getElementInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getElement(config, options);
      }),
    ),
  /**
   * List all elements as display summaries, sorted by element code.
   */
  listElements: t.procedure
    .input(schemas.listElementsInput)
    .query(({ input }) =>
      szCall(() => api.listElements(input.config)),
    ),
  /**
   * Update an element's description and/or data type.
   */
  setElement: t.procedure
    .input(schemas.setElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setElement(config, options);
      }),
    ),
  /**
   * Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter or derived flag.
   */
  setFeatureElement: t.procedure
    .input(schemas.setFeatureElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setFeatureElement(config, options);
      }),
    ),
  /**
   * Map an existing element to a feature (append a CFG_FBOM row).
   */
  addElementToFeature: t.procedure
    .input(schemas.addElementToFeatureInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addElementToFeature(config, options);
      }),
    ),
  /**
   * Remove one feature-element (CFG_FBOM) mapping.
   */
  deleteElementFromFeature: t.procedure
    .input(schemas.deleteElementFromFeatureInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteElementFromFeature(config, options);
      }),
    ),
  /**
   * Render a config document in canonical export form (recursively key-sorted, pretty-printed).
   */
  renderConfig: t.procedure
    .input(schemas.renderConfigInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.renderConfig(config, options);
      }),
    ),
  /**
   * Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional standardize/expression/comparison calls.
   */
  addFeature: t.procedure
    .input(schemas.addFeatureInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addFeature(config, options);
      }),
    ),
  /**
   * Delete a feature and cascade-delete its FBOM rows, attributes and standardize/expression/comparison/distinct calls.
   */
  deleteFeature: t.procedure
    .input(schemas.deleteFeatureInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteFeature(config, options);
      }),
    ),
  /**
   * Get one feature as a display summary including its elementList.
   */
  getFeature: t.procedure
    .input(schemas.getFeatureInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getFeature(config, options);
      }),
    ),
  /**
   * List all features as display summaries, sorted by FTYPE_ID.
   */
  listFeatures: t.procedure
    .input(schemas.listFeaturesInput)
    .query(({ input }) =>
      szCall(() => api.listFeatures(input.config)),
    ),
  /**
   * Update a feature's flags, behavior, class, version or RTYPE_ID.
   */
  setFeature: t.procedure
    .input(schemas.setFeatureInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setFeature(config, options);
      }),
    ),
  /**
   * Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.
   */
  addFeatureComparison: t.procedure
    .input(schemas.addFeatureComparisonInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addFeatureComparison(config, options);
      }),
    ),
  /**
   * Delete one feature-element (CFG_FBOM) row.
   */
  deleteFeatureComparison: t.procedure
    .input(schemas.deleteFeatureComparisonInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteFeatureComparison(config, options);
      }),
    ),
  /**
   * Get one raw CFG_FBOM row by feature and element code.
   */
  getFeatureComparison: t.procedure
    .input(schemas.getFeatureComparisonInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getFeatureComparison(config, options);
      }),
    ),
  /**
   * List all raw CFG_FBOM rows sorted by (FTYPE_ID, EXEC_ORDER).
   */
  listFeatureComparisons: t.procedure
    .input(schemas.listFeatureComparisonsInput)
    .query(({ input }) =>
      szCall(() => api.listFeatureComparisons(input.config)),
    ),
  /**
   * Add a distinct-function call (CFG_DFCALL row) for a feature.
   */
  addFeatureDistinctCallElement: t.procedure
    .input(schemas.addFeatureDistinctCallElementInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addFeatureDistinctCallElement(config, options);
      }),
    ),
  /**
   * List all raw CFG_FCLASS rows sorted by FCLASS_ID.
   */
  listFeatureClasses: t.procedure
    .input(schemas.listFeatureClassesInput)
    .query(({ input }) =>
      szCall(() => api.listFeatureClasses(input.config)),
    ),
  /**
   * Get one raw CFG_FCLASS row by id or code.
   */
  getFeatureClass: t.procedure
    .input(schemas.getFeatureClassInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getFeatureClass(config, options);
      }),
    ),
  /**
   * Set G2_CONFIG.CONFIG_BASE_VERSION.COMPATIBILITY_VERSION.FEATURE_VERSION.
   */
  updateFeatureVersion: t.procedure
    .input(schemas.updateFeatureVersionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.updateFeatureVersion(config, options);
      }),
    ),
  /**
   * Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
   */
  addFragment: t.procedure
    .input(schemas.addFragmentInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addFragment(config, options);
      }),
    ),
  /**
   * Delete a fragment by code.
   */
  deleteFragment: t.procedure
    .input(schemas.deleteFragmentInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteFragment(config, options);
      }),
    ),
  /**
   * Get one fragment, by code or ERFRAG_ID, as a summary record.
   */
  getFragment: t.procedure
    .input(schemas.getFragmentInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getFragment(config, options);
      }),
    ),
  /**
   * List all fragments as summary records in config order.
   */
  listFragments: t.procedure
    .input(schemas.listFragmentsInput)
    .query(({ input }) =>
      szCall(() => api.listFragments(input.config)),
    ),
  /**
   * Update a fragment's source and/or description.
   */
  setFragment: t.procedure
    .input(schemas.setFragmentInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setFragment(config, options);
      }),
    ),
  /**
   * Add a comparison function (CFG_CFUNC row).
   */
  addComparisonFunction: t.procedure
    .input(schemas.addComparisonFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addComparisonFunction(config, options);
      }),
    ),
  /**
   * Delete a comparison function's CFG_CFUNC row only (no cascade).
   */
  deleteComparisonFunction: t.procedure
    .input(schemas.deleteComparisonFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteComparisonFunction(config, options);
      }),
    ),
  /**
   * Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
   */
  deleteComparisonFunctionCascade: t.procedure
    .input(schemas.deleteComparisonFunctionCascadeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteComparisonFunctionCascade(config, options);
      }),
    ),
  /**
   * Get one comparison function's raw CFG_CFUNC row by code.
   */
  getComparisonFunction: t.procedure
    .input(schemas.getComparisonFunctionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getComparisonFunction(config, options);
      }),
    ),
  /**
   * List all comparison functions as camelCase summaries.
   */
  listComparisonFunctions: t.procedure
    .input(schemas.listComparisonFunctionsInput)
    .query(({ input }) =>
      szCall(() => api.listComparisonFunctions(input.config)),
    ),
  /**
   * Update a comparison function's connect string / description / language / anon support.
   */
  setComparisonFunction: t.procedure
    .input(schemas.setComparisonFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setComparisonFunction(config, options);
      }),
    ),
  /**
   * Add a distinct function (CFG_DFUNC row) to the configuration.
   */
  addDistinctFunction: t.procedure
    .input(schemas.addDistinctFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addDistinctFunction(config, options);
      }),
    ),
  /**
   * Delete a distinct function by code.
   */
  deleteDistinctFunction: t.procedure
    .input(schemas.deleteDistinctFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteDistinctFunction(config, options);
      }),
    ),
  /**
   * Get one distinct function's raw CFG_DFUNC row by code.
   */
  getDistinctFunction: t.procedure
    .input(schemas.getDistinctFunctionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getDistinctFunction(config, options);
      }),
    ),
  /**
   * List all distinct functions as {id, function, connectStr, anonSupport, language} summaries.
   */
  listDistinctFunctions: t.procedure
    .input(schemas.listDistinctFunctionsInput)
    .query(({ input }) =>
      szCall(() => api.listDistinctFunctions(input.config)),
    ),
  /**
   * Update a distinct function's connect string, description, language or anon support.
   */
  setDistinctFunction: t.procedure
    .input(schemas.setDistinctFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setDistinctFunction(config, options);
      }),
    ),
  /**
   * Add an expression function (CFG_EFUNC row).
   */
  addExpressionFunction: t.procedure
    .input(schemas.addExpressionFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addExpressionFunction(config, options);
      }),
    ),
  /**
   * Delete an expression function's CFG_EFUNC row only (no cascade).
   */
  deleteExpressionFunction: t.procedure
    .input(schemas.deleteExpressionFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteExpressionFunction(config, options);
      }),
    ),
  /**
   * Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
   */
  deleteExpressionFunctionCascade: t.procedure
    .input(schemas.deleteExpressionFunctionCascadeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteExpressionFunctionCascade(config, options);
      }),
    ),
  /**
   * Get one expression function's raw CFG_EFUNC row by code.
   */
  getExpressionFunction: t.procedure
    .input(schemas.getExpressionFunctionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getExpressionFunction(config, options);
      }),
    ),
  /**
   * List all expression functions as camelCase summaries.
   */
  listExpressionFunctions: t.procedure
    .input(schemas.listExpressionFunctionsInput)
    .query(({ input }) =>
      szCall(() => api.listExpressionFunctions(input.config)),
    ),
  /**
   * Update an expression function's connect string / description / language.
   */
  setExpressionFunction: t.procedure
    .input(schemas.setExpressionFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setExpressionFunction(config, options);
      }),
    ),
  /**
   * Add a standardize function (CFG_SFUNC row).
   */
  addStandardizeFunction: t.procedure
    .input(schemas.addStandardizeFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addStandardizeFunction(config, options);
      }),
    ),
  /**
   * Delete a standardize function's CFG_SFUNC row only (no cascade).
   */
  deleteStandardizeFunction: t.procedure
    .input(schemas.deleteStandardizeFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteStandardizeFunction(config, options);
      }),
    ),
  /**
   * Delete a standardize function and its CFG_SFCALL rows.
   */
  deleteStandardizeFunctionCascade: t.procedure
    .input(schemas.deleteStandardizeFunctionCascadeInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteStandardizeFunctionCascade(config, options);
      }),
    ),
  /**
   * Get one standardize function's raw CFG_SFUNC row by code.
   */
  getStandardizeFunction: t.procedure
    .input(schemas.getStandardizeFunctionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getStandardizeFunction(config, options);
      }),
    ),
  /**
   * List all standardize functions as camelCase summaries.
   */
  listStandardizeFunctions: t.procedure
    .input(schemas.listStandardizeFunctionsInput)
    .query(({ input }) =>
      szCall(() => api.listStandardizeFunctions(input.config)),
    ),
  /**
   * Update a standardize function's connect string / description / language.
   */
  setStandardizeFunction: t.procedure
    .input(schemas.setStandardizeFunctionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setStandardizeFunction(config, options);
      }),
    ),
  /**
   * Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
   */
  cloneGenericPlan: t.procedure
    .input(schemas.cloneGenericPlanInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.cloneGenericPlan(config, options);
      }),
    ),
  /**
   * Delete a generic plan and all of its generic thresholds.
   */
  deleteGenericPlan: t.procedure
    .input(schemas.deleteGenericPlanInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteGenericPlan(config, options);
      }),
    ),
  /**
   * List generic plans as {id, plan, description}, optionally filtered.
   */
  listGenericPlans: t.procedure
    .input(schemas.listGenericPlansInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.listGenericPlans(config, options);
      }),
    ),
  /**
   * Create a generic plan, or update the description of an existing one (upsert).
   */
  setGenericPlan: t.procedure
    .input(schemas.setGenericPlanInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setGenericPlan(config, options);
      }),
    ),
  /**
   * Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
   */
  addRule: t.procedure
    .input(schemas.addRuleInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addRule(config, options);
      }),
    ),
  /**
   * Delete a rule by code.
   */
  deleteRule: t.procedure
    .input(schemas.deleteRuleInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteRule(config, options);
      }),
    ),
  /**
   * Get one rule, by code or ERRULE_ID, as a summary record.
   */
  getRule: t.procedure
    .input(schemas.getRuleInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getRule(config, options);
      }),
    ),
  /**
   * List all rules as summary records, sorted by ERRULE_ID.
   */
  listRules: t.procedure
    .input(schemas.listRulesInput)
    .query(({ input }) =>
      szCall(() => api.listRules(input.config)),
    ),
  /**
   * Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.
   */
  setRule: t.procedure
    .input(schemas.setRuleInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setRule(config, options);
      }),
    ),
  /**
   * Add a search profile (CFG_SPROFILE row) tying a generic plan and feature candidate overrides to a code.
   */
  addSearchProfile: t.procedure
    .input(schemas.addSearchProfileInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addSearchProfile(config, options);
      }),
    ),
  /**
   * Get one search profile's raw CFG_SPROFILE row by code.
   */
  getSearchProfile: t.procedure
    .input(schemas.getSearchProfileInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.getSearchProfile(config, options);
      }),
    ),
  /**
   * List search profiles as display records with ids resolved to codes, sorted by id.
   */
  listSearchProfiles: t.procedure
    .input(schemas.listSearchProfilesInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.listSearchProfiles(config, options);
      }),
    ),
  /**
   * Delete a search profile by code or SPROFILE_ID.
   */
  deleteSearchProfile: t.procedure
    .input(schemas.deleteSearchProfileInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteSearchProfile(config, options);
      }),
    ),
  /**
   * Create or overwrite a named setting in the G2_CONFIG.SETTINGS object.
   */
  setSetting: t.procedure
    .input(schemas.setSettingInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setSetting(config, options);
      }),
    ),
  /**
   * List system parameters as a name -> string-value map.
   */
  listSystemParameters: t.procedure
    .input(schemas.listSystemParametersInput)
    .query(({ input }) =>
      szCall(() => api.listSystemParameters(input.config)),
    ),
  /**
   * Set a system parameter (relationshipsBreakMatches).
   */
  setSystemParameter: t.procedure
    .input(schemas.setSystemParameterInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setSystemParameter(config, options);
      }),
    ),
  /**
   * Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and return value.
   */
  addComparisonThreshold: t.procedure
    .input(schemas.addComparisonThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addComparisonThreshold(config, options);
      }),
    ),
  /**
   * Delete a comparison threshold identified by (comparison function, feature, return value).
   */
  deleteComparisonThreshold: t.procedure
    .input(schemas.deleteComparisonThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteComparisonThreshold(config, options);
      }),
    ),
  /**
   * Update the exec order and/or scores of an existing comparison threshold.
   */
  setComparisonThreshold: t.procedure
    .input(schemas.setComparisonThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setComparisonThreshold(config, options);
      }),
    ),
  /**
   * List all comparison thresholds with resolved function and feature names.
   */
  listComparisonThresholds: t.procedure
    .input(schemas.listComparisonThresholdsInput)
    .query(({ input }) =>
      szCall(() => api.listComparisonThresholds(input.config)),
    ),
  /**
   * Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and optional feature.
   */
  addGenericThreshold: t.procedure
    .input(schemas.addGenericThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.addGenericThreshold(config, options);
      }),
    ),
  /**
   * Delete a generic threshold identified by (plan, behavior, feature).
   */
  deleteGenericThreshold: t.procedure
    .input(schemas.deleteGenericThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.deleteGenericThreshold(config, options);
      }),
    ),
  /**
   * Update the caps and/or send-to-redo flag of an existing generic threshold.
   */
  setGenericThreshold: t.procedure
    .input(schemas.setGenericThresholdInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.setGenericThreshold(config, options);
      }),
    ),
  /**
   * List all generic thresholds with resolved plan and feature names.
   */
  listGenericThresholds: t.procedure
    .input(schemas.listGenericThresholdsInput)
    .query(({ input }) =>
      szCall(() => api.listGenericThresholds(input.config)),
    ),
  /**
   * Stage the checks of a generic-threshold add without mutating the config, returning the outcome as data.
   */
  validateGenericThreshold: t.procedure
    .input(schemas.validateGenericThresholdInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.validateGenericThreshold(config, options);
      }),
    ),
  /**
   * Check that a document has the top-level shape of a config (structure only).
   */
  validateConfig: t.procedure
    .input(schemas.validateConfigInput)
    .query(({ input }) =>
      szCall(() => api.validateConfig(input.config)),
    ),
  /**
   * Get the configuration VERSION string (G2_CONFIG.CONFIG_BASE_VERSION.VERSION).
   */
  getVersion: t.procedure
    .input(schemas.getVersionInput)
    .query(({ input }) =>
      szCall(() => api.getVersion(input.config)),
    ),
  /**
   * Get COMPATIBILITY_VERSION.CONFIG_VERSION.
   */
  getCompatibilityVersion: t.procedure
    .input(schemas.getCompatibilityVersionInput)
    .query(({ input }) =>
      szCall(() => api.getCompatibilityVersion(input.config)),
    ),
  /**
   * Set COMPATIBILITY_VERSION.CONFIG_VERSION.
   */
  updateCompatibilityVersion: t.procedure
    .input(schemas.updateCompatibilityVersionInput)
    .mutation(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.updateCompatibilityVersion(config, options);
      }),
    ),
  /**
   * Compare COMPATIBILITY_VERSION.CONFIG_VERSION with an expected value.
   */
  verifyCompatibilityVersion: t.procedure
    .input(schemas.verifyCompatibilityVersionInput)
    .query(({ input }) =>
      szCall(() => {
        const { config, ...options } = input;
        return api.verifyCompatibilityVersion(config, options);
      }),
    ),
});

/** Type of {@link configToolRouter}, for typed clients. */
export type ConfigToolRouter = typeof configToolRouter;
