// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

package io.github.brianmacy.szconfigtool;

import java.util.Map;

/** Routes conformance steps through the generated typed methods. */
final class TypedDispatch {
    private TypedDispatch() {
    }

    /**
     * Call {@code fn} through its typed wrapper.
     *
     * @return {kind, config, result}, or {@code null} when {@code fn} has no typed wrapper
     */
    static String[] call(String fn, String config, Map<String, Object> in)
            throws SzConfigToolException {
        return switch (fn) {
            case "add_attribute" -> addAttribute(config, in);
            case "delete_attribute" -> deleteAttribute(config, in);
            case "get_attribute" -> getAttribute(config, in);
            case "list_attributes" -> listAttributes(config, in);
            case "set_attribute" -> setAttribute(config, in);
            case "add_behavior_override" -> addBehaviorOverride(config, in);
            case "delete_behavior_override" -> deleteBehaviorOverride(config, in);
            case "get_behavior_override" -> getBehaviorOverride(config, in);
            case "list_behavior_overrides" -> listBehaviorOverrides(config, in);
            case "list_behavior_overrides_resolved" -> listBehaviorOverridesResolved(config, in);
            case "add_comparison_call" -> addComparisonCall(config, in);
            case "delete_comparison_call" -> deleteComparisonCall(config, in);
            case "get_comparison_call" -> getComparisonCall(config, in);
            case "list_comparison_calls" -> listComparisonCalls(config, in);
            case "add_comparison_call_element" -> addComparisonCallElement(config, in);
            case "delete_comparison_call_element" -> deleteComparisonCallElement(config, in);
            case "add_distinct_call" -> addDistinctCall(config, in);
            case "delete_distinct_call" -> deleteDistinctCall(config, in);
            case "get_distinct_call" -> getDistinctCall(config, in);
            case "list_distinct_calls" -> listDistinctCalls(config, in);
            case "add_distinct_call_element" -> addDistinctCallElement(config, in);
            case "delete_distinct_call_element" -> deleteDistinctCallElement(config, in);
            case "add_expression_call" -> addExpressionCall(config, in);
            case "delete_expression_call" -> deleteExpressionCall(config, in);
            case "get_expression_call" -> getExpressionCall(config, in);
            case "list_expression_calls" -> listExpressionCalls(config, in);
            case "add_expression_call_element" -> addExpressionCallElement(config, in);
            case "delete_expression_call_element" -> deleteExpressionCallElement(config, in);
            case "add_standardize_call" -> addStandardizeCall(config, in);
            case "delete_standardize_call" -> deleteStandardizeCall(config, in);
            case "get_standardize_call" -> getStandardizeCall(config, in);
            case "list_standardize_calls" -> listStandardizeCalls(config, in);
            case "add_standardize_call_element" -> addStandardizeCallElement(config, in);
            case "delete_standardize_call_element" -> deleteStandardizeCallElement(config, in);
            case "add_config_section" -> addConfigSection(config, in);
            case "remove_config_section" -> removeConfigSection(config, in);
            case "get_config_section" -> getConfigSection(config, in);
            case "config_section_is_empty" -> configSectionIsEmpty(config, in);
            case "list_config_sections" -> listConfigSections(config, in);
            case "remove_config_section_field" -> removeConfigSectionField(config, in);
            case "add_config_section_field" -> addConfigSectionField(config, in);
            case "add_data_source" -> addDataSource(config, in);
            case "delete_data_source" -> deleteDataSource(config, in);
            case "get_data_source" -> getDataSource(config, in);
            case "list_data_sources" -> listDataSources(config, in);
            case "set_data_source" -> setDataSource(config, in);
            case "add_element" -> addElement(config, in);
            case "delete_element" -> deleteElement(config, in);
            case "get_element" -> getElement(config, in);
            case "list_elements" -> listElements(config, in);
            case "set_element" -> setElement(config, in);
            case "set_feature_element" -> setFeatureElement(config, in);
            case "add_element_to_feature" -> addElementToFeature(config, in);
            case "delete_element_from_feature" -> deleteElementFromFeature(config, in);
            case "render_config" -> renderConfig(config, in);
            case "add_feature" -> addFeature(config, in);
            case "delete_feature" -> deleteFeature(config, in);
            case "get_feature" -> getFeature(config, in);
            case "list_features" -> listFeatures(config, in);
            case "set_feature" -> setFeature(config, in);
            case "add_feature_comparison" -> addFeatureComparison(config, in);
            case "delete_feature_comparison" -> deleteFeatureComparison(config, in);
            case "get_feature_comparison" -> getFeatureComparison(config, in);
            case "list_feature_comparisons" -> listFeatureComparisons(config, in);
            case "add_feature_distinct_call_element" -> addFeatureDistinctCallElement(config, in);
            case "list_feature_classes" -> listFeatureClasses(config, in);
            case "get_feature_class" -> getFeatureClass(config, in);
            case "update_feature_version" -> updateFeatureVersion(config, in);
            case "add_fragment" -> addFragment(config, in);
            case "delete_fragment" -> deleteFragment(config, in);
            case "get_fragment" -> getFragment(config, in);
            case "list_fragments" -> listFragments(config, in);
            case "set_fragment" -> setFragment(config, in);
            case "add_comparison_function" -> addComparisonFunction(config, in);
            case "delete_comparison_function" -> deleteComparisonFunction(config, in);
            case "delete_comparison_function_cascade" -> deleteComparisonFunctionCascade(config, in);
            case "get_comparison_function" -> getComparisonFunction(config, in);
            case "list_comparison_functions" -> listComparisonFunctions(config, in);
            case "set_comparison_function" -> setComparisonFunction(config, in);
            case "add_distinct_function" -> addDistinctFunction(config, in);
            case "delete_distinct_function" -> deleteDistinctFunction(config, in);
            case "get_distinct_function" -> getDistinctFunction(config, in);
            case "list_distinct_functions" -> listDistinctFunctions(config, in);
            case "set_distinct_function" -> setDistinctFunction(config, in);
            case "add_expression_function" -> addExpressionFunction(config, in);
            case "delete_expression_function" -> deleteExpressionFunction(config, in);
            case "delete_expression_function_cascade" -> deleteExpressionFunctionCascade(config, in);
            case "get_expression_function" -> getExpressionFunction(config, in);
            case "list_expression_functions" -> listExpressionFunctions(config, in);
            case "set_expression_function" -> setExpressionFunction(config, in);
            case "add_standardize_function" -> addStandardizeFunction(config, in);
            case "delete_standardize_function" -> deleteStandardizeFunction(config, in);
            case "delete_standardize_function_cascade" -> deleteStandardizeFunctionCascade(config, in);
            case "get_standardize_function" -> getStandardizeFunction(config, in);
            case "list_standardize_functions" -> listStandardizeFunctions(config, in);
            case "set_standardize_function" -> setStandardizeFunction(config, in);
            case "clone_generic_plan" -> cloneGenericPlan(config, in);
            case "delete_generic_plan" -> deleteGenericPlan(config, in);
            case "list_generic_plans" -> listGenericPlans(config, in);
            case "set_generic_plan" -> setGenericPlan(config, in);
            case "add_rule" -> addRule(config, in);
            case "delete_rule" -> deleteRule(config, in);
            case "get_rule" -> getRule(config, in);
            case "list_rules" -> listRules(config, in);
            case "set_rule" -> setRule(config, in);
            case "add_search_profile" -> addSearchProfile(config, in);
            case "get_search_profile" -> getSearchProfile(config, in);
            case "list_search_profiles" -> listSearchProfiles(config, in);
            case "delete_search_profile" -> deleteSearchProfile(config, in);
            case "set_setting" -> setSetting(config, in);
            case "list_system_parameters" -> listSystemParameters(config, in);
            case "set_system_parameter" -> setSystemParameter(config, in);
            case "add_comparison_threshold" -> addComparisonThreshold(config, in);
            case "delete_comparison_threshold" -> deleteComparisonThreshold(config, in);
            case "set_comparison_threshold" -> setComparisonThreshold(config, in);
            case "list_comparison_thresholds" -> listComparisonThresholds(config, in);
            case "add_generic_threshold" -> addGenericThreshold(config, in);
            case "delete_generic_threshold" -> deleteGenericThreshold(config, in);
            case "set_generic_threshold" -> setGenericThreshold(config, in);
            case "list_generic_thresholds" -> listGenericThresholds(config, in);
            case "validate_generic_threshold" -> validateGenericThreshold(config, in);
            case "validate_config" -> validateConfig(config, in);
            case "get_version" -> getVersion(config, in);
            case "get_compatibility_version" -> getCompatibilityVersion(config, in);
            case "update_compatibility_version" -> updateCompatibilityVersion(config, in);
            case "verify_compatibility_version" -> verifyCompatibilityVersion(config, in);
            default -> null;
        };
    }

    private static String[] addAttribute(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("default_value") && !in.containsKey("internal") && !in.containsKey("required") && !in.containsKey("id")) {
            return Conv.configAndJson(() -> SzConfigTool.addAttribute(config, Conv.str(in.get("attribute")), Conv.str(in.get("feature")), Conv.str(in.get("element")), Conv.str(in.get("class"))),
                () -> SzConfigTool.addAttributeResult(config, Conv.str(in.get("attribute")), Conv.str(in.get("feature")), Conv.str(in.get("element")), Conv.str(in.get("class"))));
        }
        SzConfigTool.AddAttributeOptions o = new SzConfigTool.AddAttributeOptions();
        if (in.containsKey("default_value")) {
            o.defaultValue(Conv.str(in.get("default_value")));
        }
        if (in.containsKey("internal")) {
            o.internal(Conv.str(in.get("internal")));
        }
        if (in.containsKey("required")) {
            o.required(Conv.str(in.get("required")));
        }
        if (in.containsKey("id")) {
            o.id(Conv.lng(in.get("id")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addAttribute(config, Conv.str(in.get("attribute")), Conv.str(in.get("feature")), Conv.str(in.get("element")), Conv.str(in.get("class")), o),
            () -> SzConfigTool.addAttributeResult(config, Conv.str(in.get("attribute")), Conv.str(in.get("feature")), Conv.str(in.get("element")), Conv.str(in.get("class")), o));
    }

    private static String[] deleteAttribute(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteAttribute(config, Conv.str(in.get("code"))));
    }

    private static String[] getAttribute(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getAttribute(config, Conv.str(in.get("code"))));
    }

    private static String[] listAttributes(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listAttributes(config));
    }

    private static String[] setAttribute(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("internal") && !in.containsKey("required") && !in.containsKey("default_value")) {
            return Conv.config(SzConfigTool.setAttribute(config, Conv.str(in.get("attribute"))));
        }
        SzConfigTool.SetAttributeOptions o = new SzConfigTool.SetAttributeOptions();
        if (in.containsKey("internal")) {
            o.internal(Conv.str(in.get("internal")));
        }
        if (in.containsKey("required")) {
            o.required(Conv.str(in.get("required")));
        }
        if (in.containsKey("default_value")) {
            o.defaultValue(Conv.str(in.get("default_value")));
        }
        return Conv.config(SzConfigTool.setAttribute(config, Conv.str(in.get("attribute")), o));
    }

    private static String[] addBehaviorOverride(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.addBehaviorOverride(config, Conv.str(in.get("feature")), Conv.str(in.get("usage_type")), Conv.str(in.get("behavior"))));
    }

    private static String[] deleteBehaviorOverride(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteBehaviorOverride(config, Conv.str(in.get("feature")), Conv.str(in.get("usage_type"))));
    }

    private static String[] getBehaviorOverride(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getBehaviorOverride(config, Conv.str(in.get("feature")), Conv.str(in.get("usage_type"))));
    }

    private static String[] listBehaviorOverrides(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listBehaviorOverrides(config));
    }

    private static String[] listBehaviorOverridesResolved(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listBehaviorOverridesResolved(config));
    }

    private static String[] addComparisonCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("id")) {
            return Conv.configAndJson(() -> SzConfigTool.addComparisonCall(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_code")), Conv.strList(in.get("element_list"))),
                () -> SzConfigTool.addComparisonCallResult(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_code")), Conv.strList(in.get("element_list"))));
        }
        SzConfigTool.AddComparisonCallOptions o = new SzConfigTool.AddComparisonCallOptions();
        if (in.containsKey("id")) {
            o.id(Conv.lng(in.get("id")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addComparisonCall(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_code")), Conv.strList(in.get("element_list")), o),
            () -> SzConfigTool.addComparisonCallResult(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_code")), Conv.strList(in.get("element_list")), o));
    }

    private static String[] deleteComparisonCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteComparisonCall(config, Conv.lng(in.get("cfcall_id"))));
    }

    private static String[] getComparisonCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if ((in.get("call") instanceof Long)) {
            return Conv.json(SzConfigTool.getComparisonCall(config, Conv.lng(in.get("call"))));
        }
        return Conv.json(SzConfigTool.getComparisonCall(config, Conv.str(in.get("call"))));
    }

    private static String[] listComparisonCalls(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listComparisonCalls(config));
    }

    private static String[] addComparisonCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order")) {
            return Conv.configAndJson(() -> SzConfigTool.addComparisonCallElement(config, Conv.lng(in.get("cfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id"))),
                () -> SzConfigTool.addComparisonCallElementResult(config, Conv.lng(in.get("cfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id"))));
        }
        SzConfigTool.AddComparisonCallElementOptions o = new SzConfigTool.AddComparisonCallElementOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addComparisonCallElement(config, Conv.lng(in.get("cfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), o),
            () -> SzConfigTool.addComparisonCallElementResult(config, Conv.lng(in.get("cfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), o));
    }

    private static String[] deleteComparisonCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("element_feature")) {
            if ((in.get("call") instanceof Long)) {
                return Conv.config(SzConfigTool.deleteComparisonCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code"))));
            }
            return Conv.config(SzConfigTool.deleteComparisonCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.DeleteComparisonCallElementOptions o = new SzConfigTool.DeleteComparisonCallElementOptions();
        if (in.containsKey("element_feature")) {
            o.elementFeature(Conv.str(in.get("element_feature")));
        }
        if ((in.get("call") instanceof Long)) {
            return Conv.config(SzConfigTool.deleteComparisonCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code")), o));
        }
        return Conv.config(SzConfigTool.deleteComparisonCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code")), o));
    }

    private static String[] addDistinctCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.addDistinctCall(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("dfunc_code")), Conv.strList(in.get("element_list"))),
            () -> SzConfigTool.addDistinctCallResult(config, Conv.str(in.get("ftype_code")), Conv.str(in.get("dfunc_code")), Conv.strList(in.get("element_list"))));
    }

    private static String[] deleteDistinctCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteDistinctCall(config, Conv.lng(in.get("dfcall_id"))));
    }

    private static String[] getDistinctCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if ((in.get("call") instanceof Long)) {
            return Conv.json(SzConfigTool.getDistinctCall(config, Conv.lng(in.get("call"))));
        }
        return Conv.json(SzConfigTool.getDistinctCall(config, Conv.str(in.get("call"))));
    }

    private static String[] listDistinctCalls(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listDistinctCalls(config));
    }

    private static String[] addDistinctCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order")) {
            return Conv.configAndJson(() -> SzConfigTool.addDistinctCallElement(config, Conv.lng(in.get("dfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id"))),
                () -> SzConfigTool.addDistinctCallElementResult(config, Conv.lng(in.get("dfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id"))));
        }
        SzConfigTool.AddDistinctCallElementOptions o = new SzConfigTool.AddDistinctCallElementOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addDistinctCallElement(config, Conv.lng(in.get("dfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), o),
            () -> SzConfigTool.addDistinctCallElementResult(config, Conv.lng(in.get("dfcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), o));
    }

    private static String[] deleteDistinctCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("element_feature")) {
            if ((in.get("call") instanceof Long)) {
                return Conv.config(SzConfigTool.deleteDistinctCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code"))));
            }
            return Conv.config(SzConfigTool.deleteDistinctCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.DeleteDistinctCallElementOptions o = new SzConfigTool.DeleteDistinctCallElementOptions();
        if (in.containsKey("element_feature")) {
            o.elementFeature(Conv.str(in.get("element_feature")));
        }
        if ((in.get("call") instanceof Long)) {
            return Conv.config(SzConfigTool.deleteDistinctCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code")), o));
        }
        return Conv.config(SzConfigTool.deleteDistinctCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code")), o));
    }

    private static String[] addExpressionCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        SzConfigTool.AddExpressionCallOptions o = new SzConfigTool.AddExpressionCallOptions();
        if (in.containsKey("ftype_code")) {
            o.ftypeCode(Conv.str(in.get("ftype_code")));
        }
        if (in.containsKey("felem_code")) {
            o.felemCode(Conv.str(in.get("felem_code")));
        }
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        if (in.containsKey("expression_feature")) {
            o.expressionFeature(Conv.str(in.get("expression_feature")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addExpressionCall(config, Conv.str(in.get("efunc_code")), Conv.json(in.get("element_list")), Conv.str(in.get("is_virtual")), o),
            () -> SzConfigTool.addExpressionCallResult(config, Conv.str(in.get("efunc_code")), Conv.json(in.get("element_list")), Conv.str(in.get("is_virtual")), o));
    }

    private static String[] deleteExpressionCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteExpressionCall(config, Conv.lng(in.get("efcall_id"))));
    }

    private static String[] getExpressionCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if ((in.get("call") instanceof Long)) {
            return Conv.json(SzConfigTool.getExpressionCall(config, Conv.lng(in.get("call"))));
        }
        return Conv.json(SzConfigTool.getExpressionCall(config, Conv.str(in.get("call"))));
    }

    private static String[] listExpressionCalls(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listExpressionCalls(config));
    }

    private static String[] addExpressionCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order")) {
            return Conv.configAndJson(() -> SzConfigTool.addExpressionCallElement(config, Conv.lng(in.get("efcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), Conv.str(in.get("felem_req"))),
                () -> SzConfigTool.addExpressionCallElementResult(config, Conv.lng(in.get("efcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), Conv.str(in.get("felem_req"))));
        }
        SzConfigTool.AddExpressionCallElementOptions o = new SzConfigTool.AddExpressionCallElementOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addExpressionCallElement(config, Conv.lng(in.get("efcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), Conv.str(in.get("felem_req")), o),
            () -> SzConfigTool.addExpressionCallElementResult(config, Conv.lng(in.get("efcall_id")), Conv.lng(in.get("ftype_id")), Conv.lng(in.get("felem_id")), Conv.str(in.get("felem_req")), o));
    }

    private static String[] deleteExpressionCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("element_feature")) {
            if ((in.get("call") instanceof Long)) {
                return Conv.config(SzConfigTool.deleteExpressionCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code"))));
            }
            return Conv.config(SzConfigTool.deleteExpressionCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.DeleteExpressionCallElementOptions o = new SzConfigTool.DeleteExpressionCallElementOptions();
        if (in.containsKey("element_feature")) {
            o.elementFeature(Conv.str(in.get("element_feature")));
        }
        if ((in.get("call") instanceof Long)) {
            return Conv.config(SzConfigTool.deleteExpressionCallElement(config, Conv.lng(in.get("call")), Conv.str(in.get("element_code")), o));
        }
        return Conv.config(SzConfigTool.deleteExpressionCallElement(config, Conv.str(in.get("call")), Conv.str(in.get("element_code")), o));
    }

    private static String[] addStandardizeCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        SzConfigTool.AddStandardizeCallOptions o = new SzConfigTool.AddStandardizeCallOptions();
        if (in.containsKey("ftype_code")) {
            o.ftypeCode(Conv.str(in.get("ftype_code")));
        }
        if (in.containsKey("felem_code")) {
            o.felemCode(Conv.str(in.get("felem_code")));
        }
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addStandardizeCall(config, Conv.str(in.get("sfunc_code")), o),
            () -> SzConfigTool.addStandardizeCallResult(config, Conv.str(in.get("sfunc_code")), o));
    }

    private static String[] deleteStandardizeCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteStandardizeCall(config, Conv.lng(in.get("sfcall_id"))));
    }

    private static String[] getStandardizeCall(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if ((in.get("call") instanceof Long)) {
            return Conv.json(SzConfigTool.getStandardizeCall(config, Conv.lng(in.get("call"))));
        }
        return Conv.json(SzConfigTool.getStandardizeCall(config, Conv.str(in.get("call"))));
    }

    private static String[] listStandardizeCalls(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listStandardizeCalls(config));
    }

    private static String[] addStandardizeCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("felem_id") && !in.containsKey("exec_order")) {
            return Conv.configAndJson(() -> SzConfigTool.addStandardizeCallElement(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id"))),
                () -> SzConfigTool.addStandardizeCallElementResult(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id"))));
        }
        SzConfigTool.AddStandardizeCallElementOptions o = new SzConfigTool.AddStandardizeCallElementOptions();
        if (in.containsKey("felem_id")) {
            o.felemId(Conv.lng(in.get("felem_id")));
        }
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addStandardizeCallElement(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id")), o),
            () -> SzConfigTool.addStandardizeCallElementResult(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id")), o));
    }

    private static String[] deleteStandardizeCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("felem_id")) {
            return Conv.config(SzConfigTool.deleteStandardizeCallElement(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id"))));
        }
        SzConfigTool.DeleteStandardizeCallElementOptions o = new SzConfigTool.DeleteStandardizeCallElementOptions();
        if (in.containsKey("felem_id")) {
            o.felemId(Conv.lng(in.get("felem_id")));
        }
        return Conv.config(SzConfigTool.deleteStandardizeCallElement(config, Conv.lng(in.get("ftype_id")), Conv.lng(in.get("sfunc_id")), o));
    }

    private static String[] addConfigSection(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.addConfigSection(config, Conv.str(in.get("section_name"))));
    }

    private static String[] removeConfigSection(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.removeConfigSection(config, Conv.str(in.get("section_name"))));
    }

    private static String[] getConfigSection(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("filter")) {
            return Conv.json(SzConfigTool.getConfigSection(config, Conv.str(in.get("section_name"))));
        }
        SzConfigTool.GetConfigSectionOptions o = new SzConfigTool.GetConfigSectionOptions();
        if (in.containsKey("filter")) {
            o.filter(Conv.str(in.get("filter")));
        }
        return Conv.json(SzConfigTool.getConfigSection(config, Conv.str(in.get("section_name")), o));
    }

    private static String[] configSectionIsEmpty(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.configSectionIsEmpty(config, Conv.str(in.get("section_name"))));
    }

    private static String[] listConfigSections(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listConfigSections(config));
    }

    private static String[] removeConfigSectionField(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.removeConfigSectionField(config, Conv.str(in.get("section_name")), Conv.str(in.get("field_name"))),
            () -> SzConfigTool.removeConfigSectionFieldResult(config, Conv.str(in.get("section_name")), Conv.str(in.get("field_name"))));
    }

    private static String[] addConfigSectionField(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.addConfigSectionField(config, Conv.str(in.get("section_name")), Conv.str(in.get("field_name")), Conv.json(in.get("field_value"))),
            () -> SzConfigTool.addConfigSectionFieldResult(config, Conv.str(in.get("section_name")), Conv.str(in.get("field_name")), Conv.json(in.get("field_value"))));
    }

    private static String[] addDataSource(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("retention_level") && !in.containsKey("id")) {
            return Conv.config(SzConfigTool.addDataSource(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddDataSourceOptions o = new SzConfigTool.AddDataSourceOptions();
        if (in.containsKey("retention_level")) {
            o.retentionLevel(Conv.str(in.get("retention_level")));
        }
        if (in.containsKey("id")) {
            o.id(Conv.lng(in.get("id")));
        }
        return Conv.config(SzConfigTool.addDataSource(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteDataSource(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteDataSource(config, Conv.str(in.get("code"))));
    }

    private static String[] getDataSource(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getDataSource(config, Conv.str(in.get("code"))));
    }

    private static String[] listDataSources(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listDataSources(config));
    }

    private static String[] setDataSource(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("retention_level")) {
            return Conv.config(SzConfigTool.setDataSource(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetDataSourceOptions o = new SzConfigTool.SetDataSourceOptions();
        if (in.containsKey("retention_level")) {
            o.retentionLevel(Conv.str(in.get("retention_level")));
        }
        return Conv.config(SzConfigTool.setDataSource(config, Conv.str(in.get("code")), o));
    }

    private static String[] addElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("description") && !in.containsKey("data_type") && !in.containsKey("id")) {
            return Conv.config(SzConfigTool.addElement(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddElementOptions o = new SzConfigTool.AddElementOptions();
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("data_type")) {
            o.dataType(Conv.str(in.get("data_type")));
        }
        if (in.containsKey("id")) {
            o.id(Conv.lng(in.get("id")));
        }
        return Conv.config(SzConfigTool.addElement(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteElement(config, Conv.str(in.get("code"))));
    }

    private static String[] getElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getElement(config, Conv.str(in.get("code"))));
    }

    private static String[] listElements(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listElements(config));
    }

    private static String[] setElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("description") && !in.containsKey("data_type")) {
            return Conv.config(SzConfigTool.setElement(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetElementOptions o = new SzConfigTool.SetElementOptions();
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("data_type")) {
            o.dataType(Conv.str(in.get("data_type")));
        }
        return Conv.config(SzConfigTool.setElement(config, Conv.str(in.get("code")), o));
    }

    private static String[] setFeatureElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order") && !in.containsKey("display_level") && !in.containsKey("display_delim") && !in.containsKey("derived")) {
            return Conv.config(SzConfigTool.setFeatureElement(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.SetFeatureElementOptions o = new SzConfigTool.SetFeatureElementOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        if (in.containsKey("display_level")) {
            o.displayLevel(Conv.lng(in.get("display_level")));
        }
        if (in.containsKey("display_delim")) {
            o.displayDelim(Conv.str(in.get("display_delim")));
        }
        if (in.containsKey("derived")) {
            o.derived(Conv.str(in.get("derived")));
        }
        return Conv.config(SzConfigTool.setFeatureElement(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code")), o));
    }

    private static String[] addElementToFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("display_level") && !in.containsKey("display_delim") && !in.containsKey("derived")) {
            return Conv.config(SzConfigTool.addElementToFeature(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.AddElementToFeatureOptions o = new SzConfigTool.AddElementToFeatureOptions();
        if (in.containsKey("display_level")) {
            o.displayLevel(Conv.lng(in.get("display_level")));
        }
        if (in.containsKey("display_delim")) {
            o.displayDelim(Conv.str(in.get("display_delim")));
        }
        if (in.containsKey("derived")) {
            o.derived(Conv.str(in.get("derived")));
        }
        return Conv.config(SzConfigTool.addElementToFeature(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code")), o));
    }

    private static String[] deleteElementFromFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteElementFromFeature(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
    }

    private static String[] renderConfig(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.renderConfig(config, Conv.lng(in.get("indent"))));
    }

    private static String[] addFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("class") && !in.containsKey("behavior") && !in.containsKey("candidates") && !in.containsKey("anonymize") && !in.containsKey("derived") && !in.containsKey("history") && !in.containsKey("matchkey") && !in.containsKey("standardize") && !in.containsKey("expression") && !in.containsKey("comparison") && !in.containsKey("version") && !in.containsKey("rtype_id") && !in.containsKey("id")) {
            return Conv.config(SzConfigTool.addFeature(config, Conv.str(in.get("feature")), Conv.json(in.get("element_list"))));
        }
        SzConfigTool.AddFeatureOptions o = new SzConfigTool.AddFeatureOptions();
        if (in.containsKey("class")) {
            o.classValue(Conv.str(in.get("class")));
        }
        if (in.containsKey("behavior")) {
            o.behavior(Conv.str(in.get("behavior")));
        }
        if (in.containsKey("candidates")) {
            o.candidates(Conv.str(in.get("candidates")));
        }
        if (in.containsKey("anonymize")) {
            o.anonymize(Conv.str(in.get("anonymize")));
        }
        if (in.containsKey("derived")) {
            o.derived(Conv.str(in.get("derived")));
        }
        if (in.containsKey("history")) {
            o.history(Conv.str(in.get("history")));
        }
        if (in.containsKey("matchkey")) {
            o.matchkey(Conv.str(in.get("matchkey")));
        }
        if (in.containsKey("standardize")) {
            o.standardize(Conv.str(in.get("standardize")));
        }
        if (in.containsKey("expression")) {
            o.expression(Conv.str(in.get("expression")));
        }
        if (in.containsKey("comparison")) {
            o.comparison(Conv.str(in.get("comparison")));
        }
        if (in.containsKey("version")) {
            o.version(Conv.lng(in.get("version")));
        }
        if (in.containsKey("rtype_id")) {
            o.rtypeId(Conv.lng(in.get("rtype_id")));
        }
        if (in.containsKey("id")) {
            o.id(Conv.lng(in.get("id")));
        }
        return Conv.config(SzConfigTool.addFeature(config, Conv.str(in.get("feature")), Conv.json(in.get("element_list")), o));
    }

    private static String[] deleteFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteFeature(config, Conv.str(in.get("feature"))));
    }

    private static String[] getFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getFeature(config, Conv.str(in.get("feature"))));
    }

    private static String[] listFeatures(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listFeatures(config));
    }

    private static String[] setFeature(String config, Map<String, Object> in)
            throws SzConfigToolException {
        SzConfigTool.SetFeatureOptions o = new SzConfigTool.SetFeatureOptions();
        if (in.containsKey("candidates")) {
            o.candidates(Conv.str(in.get("candidates")));
        }
        if (in.containsKey("anonymize")) {
            o.anonymize(Conv.str(in.get("anonymize")));
        }
        if (in.containsKey("derived")) {
            o.derived(Conv.str(in.get("derived")));
        }
        if (in.containsKey("history")) {
            o.history(Conv.str(in.get("history")));
        }
        if (in.containsKey("matchkey")) {
            o.matchkey(Conv.str(in.get("matchkey")));
        }
        if (in.containsKey("behavior")) {
            o.behavior(Conv.str(in.get("behavior")));
        }
        if (in.containsKey("class")) {
            o.classValue(Conv.str(in.get("class")));
        }
        if (in.containsKey("version")) {
            o.version(Conv.lng(in.get("version")));
        }
        if (in.containsKey("rtype_id")) {
            o.rtypeId(Conv.lng(in.get("rtype_id")));
        }
        return Conv.config(SzConfigTool.setFeature(config, Conv.str(in.get("feature")), o));
    }

    private static String[] addFeatureComparison(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order") && !in.containsKey("display_level") && !in.containsKey("display_delim") && !in.containsKey("derived")) {
            return Conv.config(SzConfigTool.addFeatureComparison(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
        }
        SzConfigTool.AddFeatureComparisonOptions o = new SzConfigTool.AddFeatureComparisonOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        if (in.containsKey("display_level")) {
            o.displayLevel(Conv.lng(in.get("display_level")));
        }
        if (in.containsKey("display_delim")) {
            o.displayDelim(Conv.str(in.get("display_delim")));
        }
        if (in.containsKey("derived")) {
            o.derived(Conv.str(in.get("derived")));
        }
        return Conv.config(SzConfigTool.addFeatureComparison(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code")), o));
    }

    private static String[] deleteFeatureComparison(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteFeatureComparison(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
    }

    private static String[] getFeatureComparison(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getFeatureComparison(config, Conv.str(in.get("feature_code")), Conv.str(in.get("element_code"))));
    }

    private static String[] listFeatureComparisons(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listFeatureComparisons(config));
    }

    private static String[] addFeatureDistinctCallElement(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("element_code") && !in.containsKey("exec_order")) {
            return Conv.config(SzConfigTool.addFeatureDistinctCallElement(config, Conv.str(in.get("feature_code")), Conv.str(in.get("distinct_func_code"))));
        }
        SzConfigTool.AddFeatureDistinctCallElementOptions o = new SzConfigTool.AddFeatureDistinctCallElementOptions();
        if (in.containsKey("element_code")) {
            o.elementCode(Conv.str(in.get("element_code")));
        }
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        return Conv.config(SzConfigTool.addFeatureDistinctCallElement(config, Conv.str(in.get("feature_code")), Conv.str(in.get("distinct_func_code")), o));
    }

    private static String[] listFeatureClasses(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listFeatureClasses(config));
    }

    private static String[] getFeatureClass(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getFeatureClass(config, Conv.str(in.get("feature_class"))));
    }

    private static String[] updateFeatureVersion(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.updateFeatureVersion(config, Conv.str(in.get("version"))));
    }

    private static String[] addFragment(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.addFragment(config, Conv.json(in.get("fragment_config"))),
            () -> SzConfigTool.addFragmentResult(config, Conv.json(in.get("fragment_config"))));
    }

    private static String[] deleteFragment(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteFragment(config, Conv.str(in.get("code"))));
    }

    private static String[] getFragment(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getFragment(config, Conv.str(in.get("code_or_id"))));
    }

    private static String[] listFragments(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listFragments(config));
    }

    private static String[] setFragment(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("source") && !in.containsKey("description")) {
            return Conv.config(SzConfigTool.setFragment(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetFragmentOptions o = new SzConfigTool.SetFragmentOptions();
        if (in.containsKey("source")) {
            o.source(Conv.strUpdate(in.get("source")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.strUpdate(in.get("description")));
        }
        return Conv.config(SzConfigTool.setFragment(config, Conv.str(in.get("code")), o));
    }

    private static String[] addComparisonFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language") && !in.containsKey("anon_support")) {
            return Conv.configAndJson(() -> SzConfigTool.addComparisonFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.addComparisonFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddComparisonFunctionOptions o = new SzConfigTool.AddComparisonFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.str(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        if (in.containsKey("anon_support")) {
            o.anonSupport(Conv.str(in.get("anon_support")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addComparisonFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.addComparisonFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteComparisonFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteComparisonFunction(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteComparisonFunctionResult(config, Conv.str(in.get("code"))));
    }

    private static String[] deleteComparisonFunctionCascade(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteComparisonFunctionCascade(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteComparisonFunctionCascadeResult(config, Conv.str(in.get("code"))));
    }

    private static String[] getComparisonFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getComparisonFunction(config, Conv.str(in.get("code"))));
    }

    private static String[] listComparisonFunctions(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listComparisonFunctions(config));
    }

    private static String[] setComparisonFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language") && !in.containsKey("anon_support")) {
            return Conv.configAndJson(() -> SzConfigTool.setComparisonFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.setComparisonFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetComparisonFunctionOptions o = new SzConfigTool.SetComparisonFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.strUpdate(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        if (in.containsKey("anon_support")) {
            o.anonSupport(Conv.str(in.get("anon_support")));
        }
        return Conv.configAndJson(() -> SzConfigTool.setComparisonFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.setComparisonFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] addDistinctFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language") && !in.containsKey("anon_support")) {
            return Conv.configAndJson(() -> SzConfigTool.addDistinctFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.addDistinctFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddDistinctFunctionOptions o = new SzConfigTool.AddDistinctFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.str(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        if (in.containsKey("anon_support")) {
            o.anonSupport(Conv.str(in.get("anon_support")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addDistinctFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.addDistinctFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteDistinctFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteDistinctFunction(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteDistinctFunctionResult(config, Conv.str(in.get("code"))));
    }

    private static String[] getDistinctFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getDistinctFunction(config, Conv.str(in.get("code"))));
    }

    private static String[] listDistinctFunctions(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listDistinctFunctions(config));
    }

    private static String[] setDistinctFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language") && !in.containsKey("anon_support")) {
            return Conv.configAndJson(() -> SzConfigTool.setDistinctFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.setDistinctFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetDistinctFunctionOptions o = new SzConfigTool.SetDistinctFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.strUpdate(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        if (in.containsKey("anon_support")) {
            o.anonSupport(Conv.str(in.get("anon_support")));
        }
        return Conv.configAndJson(() -> SzConfigTool.setDistinctFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.setDistinctFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] addExpressionFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language")) {
            return Conv.configAndJson(() -> SzConfigTool.addExpressionFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.addExpressionFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddExpressionFunctionOptions o = new SzConfigTool.AddExpressionFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.str(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addExpressionFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.addExpressionFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteExpressionFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteExpressionFunction(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteExpressionFunctionResult(config, Conv.str(in.get("code"))));
    }

    private static String[] deleteExpressionFunctionCascade(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteExpressionFunctionCascade(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteExpressionFunctionCascadeResult(config, Conv.str(in.get("code"))));
    }

    private static String[] getExpressionFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getExpressionFunction(config, Conv.str(in.get("code"))));
    }

    private static String[] listExpressionFunctions(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listExpressionFunctions(config));
    }

    private static String[] setExpressionFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language")) {
            return Conv.configAndJson(() -> SzConfigTool.setExpressionFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.setExpressionFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetExpressionFunctionOptions o = new SzConfigTool.SetExpressionFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.strUpdate(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        return Conv.configAndJson(() -> SzConfigTool.setExpressionFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.setExpressionFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] addStandardizeFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language")) {
            return Conv.configAndJson(() -> SzConfigTool.addStandardizeFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.addStandardizeFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.AddStandardizeFunctionOptions o = new SzConfigTool.AddStandardizeFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.str(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        return Conv.configAndJson(() -> SzConfigTool.addStandardizeFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.addStandardizeFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] deleteStandardizeFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteStandardizeFunction(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteStandardizeFunctionResult(config, Conv.str(in.get("code"))));
    }

    private static String[] deleteStandardizeFunctionCascade(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.deleteStandardizeFunctionCascade(config, Conv.str(in.get("code"))),
            () -> SzConfigTool.deleteStandardizeFunctionCascadeResult(config, Conv.str(in.get("code"))));
    }

    private static String[] getStandardizeFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getStandardizeFunction(config, Conv.str(in.get("code"))));
    }

    private static String[] listStandardizeFunctions(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listStandardizeFunctions(config));
    }

    private static String[] setStandardizeFunction(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("connect_str") && !in.containsKey("description") && !in.containsKey("language")) {
            return Conv.configAndJson(() -> SzConfigTool.setStandardizeFunction(config, Conv.str(in.get("code"))),
                () -> SzConfigTool.setStandardizeFunctionResult(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetStandardizeFunctionOptions o = new SzConfigTool.SetStandardizeFunctionOptions();
        if (in.containsKey("connect_str")) {
            o.connectStr(Conv.strUpdate(in.get("connect_str")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("language")) {
            o.language(Conv.str(in.get("language")));
        }
        return Conv.configAndJson(() -> SzConfigTool.setStandardizeFunction(config, Conv.str(in.get("code")), o),
            () -> SzConfigTool.setStandardizeFunctionResult(config, Conv.str(in.get("code")), o));
    }

    private static String[] cloneGenericPlan(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("new_gplan_desc")) {
            return Conv.configAndJson(() -> SzConfigTool.cloneGenericPlan(config, Conv.str(in.get("source_gplan_code")), Conv.str(in.get("new_gplan_code"))),
                () -> SzConfigTool.cloneGenericPlanResult(config, Conv.str(in.get("source_gplan_code")), Conv.str(in.get("new_gplan_code"))));
        }
        SzConfigTool.CloneGenericPlanOptions o = new SzConfigTool.CloneGenericPlanOptions();
        if (in.containsKey("new_gplan_desc")) {
            o.newGplanDesc(Conv.str(in.get("new_gplan_desc")));
        }
        return Conv.configAndJson(() -> SzConfigTool.cloneGenericPlan(config, Conv.str(in.get("source_gplan_code")), Conv.str(in.get("new_gplan_code")), o),
            () -> SzConfigTool.cloneGenericPlanResult(config, Conv.str(in.get("source_gplan_code")), Conv.str(in.get("new_gplan_code")), o));
    }

    private static String[] deleteGenericPlan(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteGenericPlan(config, Conv.str(in.get("gplan_code"))));
    }

    private static String[] listGenericPlans(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("filter")) {
            return Conv.json(SzConfigTool.listGenericPlans(config));
        }
        SzConfigTool.ListGenericPlansOptions o = new SzConfigTool.ListGenericPlansOptions();
        if (in.containsKey("filter")) {
            o.filter(Conv.str(in.get("filter")));
        }
        return Conv.json(SzConfigTool.listGenericPlans(config, o));
    }

    private static String[] setGenericPlan(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.setGenericPlan(config, Conv.str(in.get("gplan_code")), Conv.str(in.get("gplan_desc"))),
            () -> {
                var r = SzConfigTool.setGenericPlanResult(config, Conv.str(in.get("gplan_code")), Conv.str(in.get("gplan_desc")));
                return Conv.record(new String[] {"plan_id", "was_created"}, r.planId(), r.wasCreated());
            });
    }

    private static String[] addRule(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.configAndJson(() -> SzConfigTool.addRule(config, Conv.lng(in.get("id")), Conv.json(in.get("rule_config"))),
            () -> SzConfigTool.addRuleResult(config, Conv.lng(in.get("id")), Conv.json(in.get("rule_config"))));
    }

    private static String[] deleteRule(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteRule(config, Conv.str(in.get("code"))));
    }

    private static String[] getRule(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getRule(config, Conv.str(in.get("code_or_id"))));
    }

    private static String[] listRules(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listRules(config));
    }

    private static String[] setRule(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("resolve") && !in.containsKey("relate") && !in.containsKey("rtype_id") && !in.containsKey("fragment") && !in.containsKey("disqualifier") && !in.containsKey("tier")) {
            return Conv.config(SzConfigTool.setRule(config, Conv.str(in.get("code"))));
        }
        SzConfigTool.SetRuleOptions o = new SzConfigTool.SetRuleOptions();
        if (in.containsKey("resolve")) {
            o.resolve(Conv.str(in.get("resolve")));
        }
        if (in.containsKey("relate")) {
            o.relate(Conv.str(in.get("relate")));
        }
        if (in.containsKey("rtype_id")) {
            o.rtypeId(Conv.lng(in.get("rtype_id")));
        }
        if (in.containsKey("fragment")) {
            o.fragment(Conv.strUpdate(in.get("fragment")));
        }
        if (in.containsKey("disqualifier")) {
            o.disqualifier(Conv.strUpdate(in.get("disqualifier")));
        }
        if (in.containsKey("tier")) {
            o.tier(Conv.intUpdate(in.get("tier")));
        }
        return Conv.config(SzConfigTool.setRule(config, Conv.str(in.get("code")), o));
    }

    private static String[] addSearchProfile(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("candidates") && !in.containsKey("description") && !in.containsKey("elements")) {
            return Conv.config(SzConfigTool.addSearchProfile(config, Conv.str(in.get("code")), Conv.str(in.get("generic_plan"))));
        }
        SzConfigTool.AddSearchProfileOptions o = new SzConfigTool.AddSearchProfileOptions();
        if (in.containsKey("candidates")) {
            o.candidates(Conv.str(in.get("candidates")));
        }
        if (in.containsKey("description")) {
            o.description(Conv.str(in.get("description")));
        }
        if (in.containsKey("elements")) {
            o.elements(Conv.json(in.get("elements")));
        }
        return Conv.config(SzConfigTool.addSearchProfile(config, Conv.str(in.get("code")), Conv.str(in.get("generic_plan")), o));
    }

    private static String[] getSearchProfile(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getSearchProfile(config, Conv.str(in.get("code"))));
    }

    private static String[] listSearchProfiles(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("filter")) {
            return Conv.json(SzConfigTool.listSearchProfiles(config));
        }
        SzConfigTool.ListSearchProfilesOptions o = new SzConfigTool.ListSearchProfilesOptions();
        if (in.containsKey("filter")) {
            o.filter(Conv.str(in.get("filter")));
        }
        return Conv.json(SzConfigTool.listSearchProfiles(config, o));
    }

    private static String[] deleteSearchProfile(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteSearchProfile(config, Conv.str(in.get("search_value"))));
    }

    private static String[] setSetting(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.setSetting(config, Conv.str(in.get("name")), Conv.json(in.get("value"))));
    }

    private static String[] listSystemParameters(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listSystemParameters(config));
    }

    private static String[] setSystemParameter(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.setSystemParameter(config, Conv.str(in.get("parameter_name")), Conv.json(in.get("parameter_value"))));
    }

    private static String[] addComparisonThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order") && !in.containsKey("same_score") && !in.containsKey("close_score") && !in.containsKey("likely_score") && !in.containsKey("plausible_score") && !in.containsKey("un_likely_score")) {
            return Conv.config(SzConfigTool.addComparisonThreshold(config, Conv.str(in.get("cfunc_code")), Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_rtnval"))));
        }
        SzConfigTool.AddComparisonThresholdOptions o = new SzConfigTool.AddComparisonThresholdOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        if (in.containsKey("same_score")) {
            o.sameScore(Conv.lng(in.get("same_score")));
        }
        if (in.containsKey("close_score")) {
            o.closeScore(Conv.lng(in.get("close_score")));
        }
        if (in.containsKey("likely_score")) {
            o.likelyScore(Conv.lng(in.get("likely_score")));
        }
        if (in.containsKey("plausible_score")) {
            o.plausibleScore(Conv.lng(in.get("plausible_score")));
        }
        if (in.containsKey("un_likely_score")) {
            o.unLikelyScore(Conv.lng(in.get("un_likely_score")));
        }
        return Conv.config(SzConfigTool.addComparisonThreshold(config, Conv.str(in.get("cfunc_code")), Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_rtnval")), o));
    }

    private static String[] deleteComparisonThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.deleteComparisonThreshold(config, Conv.str(in.get("cfunc_code")), Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_rtnval"))));
    }

    private static String[] setComparisonThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("exec_order") && !in.containsKey("same_score") && !in.containsKey("close_score") && !in.containsKey("likely_score") && !in.containsKey("plausible_score") && !in.containsKey("un_likely_score")) {
            return Conv.config(SzConfigTool.setComparisonThreshold(config, Conv.str(in.get("cfunc_code")), Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_rtnval"))));
        }
        SzConfigTool.SetComparisonThresholdOptions o = new SzConfigTool.SetComparisonThresholdOptions();
        if (in.containsKey("exec_order")) {
            o.execOrder(Conv.lng(in.get("exec_order")));
        }
        if (in.containsKey("same_score")) {
            o.sameScore(Conv.lng(in.get("same_score")));
        }
        if (in.containsKey("close_score")) {
            o.closeScore(Conv.lng(in.get("close_score")));
        }
        if (in.containsKey("likely_score")) {
            o.likelyScore(Conv.lng(in.get("likely_score")));
        }
        if (in.containsKey("plausible_score")) {
            o.plausibleScore(Conv.lng(in.get("plausible_score")));
        }
        if (in.containsKey("un_likely_score")) {
            o.unLikelyScore(Conv.lng(in.get("un_likely_score")));
        }
        return Conv.config(SzConfigTool.setComparisonThreshold(config, Conv.str(in.get("cfunc_code")), Conv.str(in.get("ftype_code")), Conv.str(in.get("cfunc_rtnval")), o));
    }

    private static String[] listComparisonThresholds(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listComparisonThresholds(config));
    }

    private static String[] addGenericThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("feature")) {
            return Conv.config(SzConfigTool.addGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), Conv.lng(in.get("scoring_cap")), Conv.lng(in.get("candidate_cap")), Conv.str(in.get("send_to_redo"))));
        }
        SzConfigTool.AddGenericThresholdOptions o = new SzConfigTool.AddGenericThresholdOptions();
        if (in.containsKey("feature")) {
            o.feature(Conv.str(in.get("feature")));
        }
        return Conv.config(SzConfigTool.addGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), Conv.lng(in.get("scoring_cap")), Conv.lng(in.get("candidate_cap")), Conv.str(in.get("send_to_redo")), o));
    }

    private static String[] deleteGenericThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("feature")) {
            return Conv.config(SzConfigTool.deleteGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior"))));
        }
        SzConfigTool.DeleteGenericThresholdOptions o = new SzConfigTool.DeleteGenericThresholdOptions();
        if (in.containsKey("feature")) {
            o.feature(Conv.str(in.get("feature")));
        }
        return Conv.config(SzConfigTool.deleteGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), o));
    }

    private static String[] setGenericThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("feature") && !in.containsKey("candidate_cap") && !in.containsKey("scoring_cap") && !in.containsKey("send_to_redo")) {
            return Conv.config(SzConfigTool.setGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior"))));
        }
        SzConfigTool.SetGenericThresholdOptions o = new SzConfigTool.SetGenericThresholdOptions();
        if (in.containsKey("feature")) {
            o.feature(Conv.str(in.get("feature")));
        }
        if (in.containsKey("candidate_cap")) {
            o.candidateCap(Conv.lng(in.get("candidate_cap")));
        }
        if (in.containsKey("scoring_cap")) {
            o.scoringCap(Conv.lng(in.get("scoring_cap")));
        }
        if (in.containsKey("send_to_redo")) {
            o.sendToRedo(Conv.str(in.get("send_to_redo")));
        }
        return Conv.config(SzConfigTool.setGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), o));
    }

    private static String[] listGenericThresholds(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.listGenericThresholds(config));
    }

    private static String[] validateGenericThreshold(String config, Map<String, Object> in)
            throws SzConfigToolException {
        if (!in.containsKey("feature")) {
            return Conv.json(SzConfigTool.validateGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), Conv.str(in.get("send_to_redo"))));
        }
        SzConfigTool.ValidateGenericThresholdOptions o = new SzConfigTool.ValidateGenericThresholdOptions();
        if (in.containsKey("feature")) {
            o.feature(Conv.str(in.get("feature")));
        }
        return Conv.json(SzConfigTool.validateGenericThreshold(config, Conv.str(in.get("plan")), Conv.str(in.get("behavior")), Conv.str(in.get("send_to_redo")), o));
    }

    private static String[] validateConfig(String config, Map<String, Object> in)
            throws SzConfigToolException {
        SzConfigTool.validateConfig(config);
        return Conv.unit();
    }

    private static String[] getVersion(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getVersion(config));
    }

    private static String[] getCompatibilityVersion(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.json(SzConfigTool.getCompatibilityVersion(config));
    }

    private static String[] updateCompatibilityVersion(String config, Map<String, Object> in)
            throws SzConfigToolException {
        return Conv.config(SzConfigTool.updateCompatibilityVersion(config, Conv.str(in.get("new_version"))));
    }

    private static String[] verifyCompatibilityVersion(String config, Map<String, Object> in)
            throws SzConfigToolException {
        var r = SzConfigTool.verifyCompatibilityVersion(config, Conv.str(in.get("expected_version")));
        return Conv.jsonResult(Conv.record(new String[] {"current_version", "matches"}, r.currentVersion(), r.matches()));
    }
}
