// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.
//
// Zod input schemas: `config` (opaque text) plus the function's arguments
// (camelCase). Strict: unknown keys are rejected, like the wire.

import { z } from "zod";

/** Input of `addAttribute`. */
export const addAttributeInput = z.strictObject({
  config: z.string(),
  attribute: z.string(),
  feature: z.string(),
  element: z.string(),
  class: z.string(),
  defaultValue: z.string().optional(),
  internal: z.string().optional(),
  required: z.string().optional(),
  id: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteAttribute`. */
export const deleteAttributeInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getAttribute`. */
export const getAttributeInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listAttributes`. */
export const listAttributesInput = z.strictObject({
  config: z.string(),
});

/** Input of `setAttribute`. */
export const setAttributeInput = z.strictObject({
  config: z.string(),
  attribute: z.string(),
  internal: z.string().optional(),
  required: z.string().optional(),
  defaultValue: z.string().optional(),
});

/** Input of `addBehaviorOverride`. */
export const addBehaviorOverrideInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
  usageType: z.string(),
  behavior: z.string(),
});

/** Input of `deleteBehaviorOverride`. */
export const deleteBehaviorOverrideInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
  usageType: z.string(),
});

/** Input of `getBehaviorOverride`. */
export const getBehaviorOverrideInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
  usageType: z.string(),
});

/** Input of `listBehaviorOverrides`. */
export const listBehaviorOverridesInput = z.strictObject({
  config: z.string(),
});

/** Input of `listBehaviorOverridesResolved`. */
export const listBehaviorOverridesResolvedInput = z.strictObject({
  config: z.string(),
});

/** Input of `addComparisonCall`. */
export const addComparisonCallInput = z.strictObject({
  config: z.string(),
  ftypeCode: z.string(),
  cfuncCode: z.string(),
  elementList: z.array(z.string()),
  id: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteComparisonCall`. */
export const deleteComparisonCallInput = z.strictObject({
  config: z.string(),
  cfcallId: z.union([z.int(), z.bigint()]),
});

/** Input of `getComparisonCall`. */
export const getComparisonCallInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
});

/** Input of `listComparisonCalls`. */
export const listComparisonCallsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addComparisonCallElement`. */
export const addComparisonCallElementInput = z.strictObject({
  config: z.string(),
  cfcallId: z.union([z.int(), z.bigint()]),
  ftypeId: z.union([z.int(), z.bigint()]),
  felemId: z.union([z.int(), z.bigint()]),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteComparisonCallElement`. */
export const deleteComparisonCallElementInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
  elementCode: z.string(),
  elementFeature: z.string().optional(),
});

/** Input of `addDistinctCall`. */
export const addDistinctCallInput = z.strictObject({
  config: z.string(),
  ftypeCode: z.string(),
  dfuncCode: z.string(),
  elementList: z.array(z.string()),
});

/** Input of `deleteDistinctCall`. */
export const deleteDistinctCallInput = z.strictObject({
  config: z.string(),
  dfcallId: z.union([z.int(), z.bigint()]),
});

/** Input of `getDistinctCall`. */
export const getDistinctCallInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
});

/** Input of `listDistinctCalls`. */
export const listDistinctCallsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addDistinctCallElement`. */
export const addDistinctCallElementInput = z.strictObject({
  config: z.string(),
  dfcallId: z.union([z.int(), z.bigint()]),
  ftypeId: z.union([z.int(), z.bigint()]),
  felemId: z.union([z.int(), z.bigint()]),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteDistinctCallElement`. */
export const deleteDistinctCallElementInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
  elementCode: z.string(),
  elementFeature: z.string().optional(),
});

/** Input of `addExpressionCall`. */
export const addExpressionCallInput = z.strictObject({
  config: z.string(),
  efuncCode: z.string(),
  elementList: z.json(),
  ftypeCode: z.string().optional(),
  felemCode: z.string().optional(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  expressionFeature: z.string().optional(),
  isVirtual: z.string(),
});

/** Input of `deleteExpressionCall`. */
export const deleteExpressionCallInput = z.strictObject({
  config: z.string(),
  efcallId: z.union([z.int(), z.bigint()]),
});

/** Input of `getExpressionCall`. */
export const getExpressionCallInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
});

/** Input of `listExpressionCalls`. */
export const listExpressionCallsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addExpressionCallElement`. */
export const addExpressionCallElementInput = z.strictObject({
  config: z.string(),
  efcallId: z.union([z.int(), z.bigint()]),
  ftypeId: z.union([z.int(), z.bigint()]),
  felemId: z.union([z.int(), z.bigint()]),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  felemReq: z.string(),
});

/** Input of `deleteExpressionCallElement`. */
export const deleteExpressionCallElementInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
  elementCode: z.string(),
  elementFeature: z.string().optional(),
});

/** Input of `addStandardizeCall`. */
export const addStandardizeCallInput = z.strictObject({
  config: z.string(),
  sfuncCode: z.string(),
  ftypeCode: z.string().optional(),
  felemCode: z.string().optional(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteStandardizeCall`. */
export const deleteStandardizeCallInput = z.strictObject({
  config: z.string(),
  sfcallId: z.union([z.int(), z.bigint()]),
});

/** Input of `getStandardizeCall`. */
export const getStandardizeCallInput = z.strictObject({
  config: z.string(),
  call: z.union([z.int(), z.bigint(), z.string()]),
});

/** Input of `listStandardizeCalls`. */
export const listStandardizeCallsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addStandardizeCallElement`. */
export const addStandardizeCallElementInput = z.strictObject({
  config: z.string(),
  ftypeId: z.union([z.int(), z.bigint()]),
  sfuncId: z.union([z.int(), z.bigint()]),
  felemId: z.union([z.int(), z.bigint()]).optional(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteStandardizeCallElement`. */
export const deleteStandardizeCallElementInput = z.strictObject({
  config: z.string(),
  ftypeId: z.union([z.int(), z.bigint()]),
  sfuncId: z.union([z.int(), z.bigint()]),
  felemId: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `addConfigSection`. */
export const addConfigSectionInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
});

/** Input of `removeConfigSection`. */
export const removeConfigSectionInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
});

/** Input of `getConfigSection`. */
export const getConfigSectionInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
  filter: z.string().optional(),
});

/** Input of `configSectionIsEmpty`. */
export const configSectionIsEmptyInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
});

/** Input of `listConfigSections`. */
export const listConfigSectionsInput = z.strictObject({
  config: z.string(),
});

/** Input of `removeConfigSectionField`. */
export const removeConfigSectionFieldInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
  fieldName: z.string(),
});

/** Input of `addConfigSectionField`. */
export const addConfigSectionFieldInput = z.strictObject({
  config: z.string(),
  sectionName: z.string(),
  fieldName: z.string(),
  fieldValue: z.json(),
});

/** Input of `addDataSource`. */
export const addDataSourceInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  retentionLevel: z.string().optional(),
  id: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteDataSource`. */
export const deleteDataSourceInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getDataSource`. */
export const getDataSourceInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listDataSources`. */
export const listDataSourcesInput = z.strictObject({
  config: z.string(),
});

/** Input of `setDataSource`. */
export const setDataSourceInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  retentionLevel: z.string().optional(),
});

/** Input of `addElement`. */
export const addElementInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  description: z.string().optional(),
  dataType: z.string().optional(),
  id: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteElement`. */
export const deleteElementInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getElement`. */
export const getElementInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listElements`. */
export const listElementsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setElement`. */
export const setElementInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  description: z.string().optional(),
  dataType: z.string().optional(),
});

/** Input of `setFeatureElement`. */
export const setFeatureElementInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  displayLevel: z.union([z.int(), z.bigint()]).optional(),
  displayDelim: z.string().optional(),
  derived: z.string().optional(),
});

/** Input of `addElementToFeature`. */
export const addElementToFeatureInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
  displayLevel: z.union([z.int(), z.bigint()]).optional(),
  displayDelim: z.string().optional(),
  derived: z.string().optional(),
});

/** Input of `deleteElementFromFeature`. */
export const deleteElementFromFeatureInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
});

/** Input of `renderConfig`. */
export const renderConfigInput = z.strictObject({
  config: z.string(),
  indent: z.union([z.int(), z.bigint()]),
});

/** Input of `addFeature`. */
export const addFeatureInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
  elementList: z.json(),
  class: z.string().optional(),
  behavior: z.string().optional(),
  candidates: z.string().optional(),
  anonymize: z.string().optional(),
  derived: z.string().optional(),
  history: z.string().optional(),
  matchkey: z.string().optional(),
  standardize: z.string().optional(),
  expression: z.string().optional(),
  comparison: z.string().optional(),
  version: z.union([z.int(), z.bigint()]).optional(),
  rtypeId: z.union([z.int(), z.bigint()]).optional(),
  id: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteFeature`. */
export const deleteFeatureInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
});

/** Input of `getFeature`. */
export const getFeatureInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
});

/** Input of `listFeatures`. */
export const listFeaturesInput = z.strictObject({
  config: z.string(),
});

/** Input of `setFeature`. */
export const setFeatureInput = z.strictObject({
  config: z.string(),
  feature: z.string(),
  candidates: z.string().optional(),
  anonymize: z.string().optional(),
  derived: z.string().optional(),
  history: z.string().optional(),
  matchkey: z.string().optional(),
  behavior: z.string().optional(),
  class: z.string().optional(),
  version: z.union([z.int(), z.bigint()]).optional(),
  rtypeId: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `addFeatureComparison`. */
export const addFeatureComparisonInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  displayLevel: z.union([z.int(), z.bigint()]).optional(),
  displayDelim: z.string().optional(),
  derived: z.string().optional(),
});

/** Input of `deleteFeatureComparison`. */
export const deleteFeatureComparisonInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
});

/** Input of `getFeatureComparison`. */
export const getFeatureComparisonInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  elementCode: z.string(),
});

/** Input of `listFeatureComparisons`. */
export const listFeatureComparisonsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addFeatureDistinctCallElement`. */
export const addFeatureDistinctCallElementInput = z.strictObject({
  config: z.string(),
  featureCode: z.string(),
  distinctFuncCode: z.string(),
  elementCode: z.string().optional(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `listFeatureClasses`. */
export const listFeatureClassesInput = z.strictObject({
  config: z.string(),
});

/** Input of `getFeatureClass`. */
export const getFeatureClassInput = z.strictObject({
  config: z.string(),
  featureClass: z.string(),
});

/** Input of `updateFeatureVersion`. */
export const updateFeatureVersionInput = z.strictObject({
  config: z.string(),
  version: z.string(),
});

/** Input of `addFragment`. */
export const addFragmentInput = z.strictObject({
  config: z.string(),
  fragmentConfig: z.json(),
});

/** Input of `deleteFragment`. */
export const deleteFragmentInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getFragment`. */
export const getFragmentInput = z.strictObject({
  config: z.string(),
  codeOrId: z.string(),
});

/** Input of `listFragments`. */
export const listFragmentsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setFragment`. */
export const setFragmentInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  source: z.string().nullable().optional(),
  description: z.string().nullable().optional(),
});

/** Input of `addComparisonFunction`. */
export const addComparisonFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
  anonSupport: z.string().optional(),
});

/** Input of `deleteComparisonFunction`. */
export const deleteComparisonFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `deleteComparisonFunctionCascade`. */
export const deleteComparisonFunctionCascadeInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getComparisonFunction`. */
export const getComparisonFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listComparisonFunctions`. */
export const listComparisonFunctionsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setComparisonFunction`. */
export const setComparisonFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().nullable().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
  anonSupport: z.string().optional(),
});

/** Input of `addDistinctFunction`. */
export const addDistinctFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
  anonSupport: z.string().optional(),
});

/** Input of `deleteDistinctFunction`. */
export const deleteDistinctFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getDistinctFunction`. */
export const getDistinctFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listDistinctFunctions`. */
export const listDistinctFunctionsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setDistinctFunction`. */
export const setDistinctFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().nullable().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
  anonSupport: z.string().optional(),
});

/** Input of `addExpressionFunction`. */
export const addExpressionFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
});

/** Input of `deleteExpressionFunction`. */
export const deleteExpressionFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `deleteExpressionFunctionCascade`. */
export const deleteExpressionFunctionCascadeInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getExpressionFunction`. */
export const getExpressionFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listExpressionFunctions`. */
export const listExpressionFunctionsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setExpressionFunction`. */
export const setExpressionFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().nullable().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
});

/** Input of `addStandardizeFunction`. */
export const addStandardizeFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
});

/** Input of `deleteStandardizeFunction`. */
export const deleteStandardizeFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `deleteStandardizeFunctionCascade`. */
export const deleteStandardizeFunctionCascadeInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getStandardizeFunction`. */
export const getStandardizeFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listStandardizeFunctions`. */
export const listStandardizeFunctionsInput = z.strictObject({
  config: z.string(),
});

/** Input of `setStandardizeFunction`. */
export const setStandardizeFunctionInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  connectStr: z.string().nullable().optional(),
  description: z.string().optional(),
  language: z.string().optional(),
});

/** Input of `cloneGenericPlan`. */
export const cloneGenericPlanInput = z.strictObject({
  config: z.string(),
  sourceGplanCode: z.string(),
  newGplanCode: z.string(),
  newGplanDesc: z.string().optional(),
});

/** Input of `deleteGenericPlan`. */
export const deleteGenericPlanInput = z.strictObject({
  config: z.string(),
  gplanCode: z.string(),
});

/** Input of `listGenericPlans`. */
export const listGenericPlansInput = z.strictObject({
  config: z.string(),
  filter: z.string().optional(),
});

/** Input of `setGenericPlan`. */
export const setGenericPlanInput = z.strictObject({
  config: z.string(),
  gplanCode: z.string(),
  gplanDesc: z.string(),
});

/** Input of `addRule`. */
export const addRuleInput = z.strictObject({
  config: z.string(),
  id: z.union([z.int(), z.bigint()]),
  ruleConfig: z.json(),
});

/** Input of `deleteRule`. */
export const deleteRuleInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `getRule`. */
export const getRuleInput = z.strictObject({
  config: z.string(),
  codeOrId: z.string(),
});

/** Input of `listRules`. */
export const listRulesInput = z.strictObject({
  config: z.string(),
});

/** Input of `setRule`. */
export const setRuleInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  resolve: z.string().optional(),
  relate: z.string().optional(),
  rtypeId: z.union([z.int(), z.bigint()]).optional(),
  fragment: z.string().nullable().optional(),
  disqualifier: z.string().nullable().optional(),
  tier: z.union([z.int(), z.bigint()]).nullable().optional(),
});

/** Input of `addSearchProfile`. */
export const addSearchProfileInput = z.strictObject({
  config: z.string(),
  code: z.string(),
  genericPlan: z.string(),
  candidates: z.string().optional(),
  description: z.string().optional(),
  elements: z.json().optional(),
});

/** Input of `getSearchProfile`. */
export const getSearchProfileInput = z.strictObject({
  config: z.string(),
  code: z.string(),
});

/** Input of `listSearchProfiles`. */
export const listSearchProfilesInput = z.strictObject({
  config: z.string(),
  filter: z.string().optional(),
});

/** Input of `deleteSearchProfile`. */
export const deleteSearchProfileInput = z.strictObject({
  config: z.string(),
  searchValue: z.string(),
});

/** Input of `setSetting`. */
export const setSettingInput = z.strictObject({
  config: z.string(),
  name: z.string(),
  value: z.json(),
});

/** Input of `listSystemParameters`. */
export const listSystemParametersInput = z.strictObject({
  config: z.string(),
});

/** Input of `setSystemParameter`. */
export const setSystemParameterInput = z.strictObject({
  config: z.string(),
  parameterName: z.string(),
  parameterValue: z.json(),
});

/** Input of `addComparisonThreshold`. */
export const addComparisonThresholdInput = z.strictObject({
  config: z.string(),
  cfuncCode: z.string(),
  ftypeCode: z.string(),
  cfuncRtnval: z.string(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  sameScore: z.union([z.int(), z.bigint()]).optional(),
  closeScore: z.union([z.int(), z.bigint()]).optional(),
  likelyScore: z.union([z.int(), z.bigint()]).optional(),
  plausibleScore: z.union([z.int(), z.bigint()]).optional(),
  unLikelyScore: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `deleteComparisonThreshold`. */
export const deleteComparisonThresholdInput = z.strictObject({
  config: z.string(),
  cfuncCode: z.string(),
  ftypeCode: z.string(),
  cfuncRtnval: z.string(),
});

/** Input of `setComparisonThreshold`. */
export const setComparisonThresholdInput = z.strictObject({
  config: z.string(),
  cfuncCode: z.string(),
  ftypeCode: z.string(),
  cfuncRtnval: z.string(),
  execOrder: z.union([z.int(), z.bigint()]).optional(),
  sameScore: z.union([z.int(), z.bigint()]).optional(),
  closeScore: z.union([z.int(), z.bigint()]).optional(),
  likelyScore: z.union([z.int(), z.bigint()]).optional(),
  plausibleScore: z.union([z.int(), z.bigint()]).optional(),
  unLikelyScore: z.union([z.int(), z.bigint()]).optional(),
});

/** Input of `listComparisonThresholds`. */
export const listComparisonThresholdsInput = z.strictObject({
  config: z.string(),
});

/** Input of `addGenericThreshold`. */
export const addGenericThresholdInput = z.strictObject({
  config: z.string(),
  plan: z.string(),
  behavior: z.string(),
  scoringCap: z.union([z.int(), z.bigint()]),
  candidateCap: z.union([z.int(), z.bigint()]),
  sendToRedo: z.string(),
  feature: z.string().optional(),
});

/** Input of `deleteGenericThreshold`. */
export const deleteGenericThresholdInput = z.strictObject({
  config: z.string(),
  plan: z.string(),
  behavior: z.string(),
  feature: z.string().optional(),
});

/** Input of `setGenericThreshold`. */
export const setGenericThresholdInput = z.strictObject({
  config: z.string(),
  plan: z.string(),
  behavior: z.string(),
  feature: z.string().optional(),
  candidateCap: z.union([z.int(), z.bigint()]).optional(),
  scoringCap: z.union([z.int(), z.bigint()]).optional(),
  sendToRedo: z.string().optional(),
});

/** Input of `listGenericThresholds`. */
export const listGenericThresholdsInput = z.strictObject({
  config: z.string(),
});

/** Input of `validateGenericThreshold`. */
export const validateGenericThresholdInput = z.strictObject({
  config: z.string(),
  plan: z.string(),
  behavior: z.string(),
  sendToRedo: z.string(),
  feature: z.string().optional(),
});

/** Input of `validateConfig`. */
export const validateConfigInput = z.strictObject({
  config: z.string(),
});

/** Input of `getVersion`. */
export const getVersionInput = z.strictObject({
  config: z.string(),
});

/** Input of `getCompatibilityVersion`. */
export const getCompatibilityVersionInput = z.strictObject({
  config: z.string(),
});

/** Input of `updateCompatibilityVersion`. */
export const updateCompatibilityVersionInput = z.strictObject({
  config: z.string(),
  newVersion: z.string(),
});

/** Input of `verifyCompatibilityVersion`. */
export const verifyCompatibilityVersionInput = z.strictObject({
  config: z.string(),
  expectedVersion: z.string(),
});

