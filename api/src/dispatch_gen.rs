// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

use crate::args::Args;
use crate::output::Output;
use crate::ApiError;

/// Signature shared by every generated handler.
pub(crate) type Handler = fn(&str, &Args<'_>) -> Result<Output, ApiError>;

/// The generated manifest (functions, args, errors, paths) as JSON.
pub const MANIFEST_JSON: &str = include_str!("../manifest/generated/manifest.json");

/// Every function name `invoke` dispatches, in manifest order.
pub const FUNCTION_NAMES: &[&str] = &[
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

/// Resolve a function name to its handler.
pub(crate) fn lookup(name: &str) -> Option<Handler> {
    match name {
        "add_attribute" => Some(call_add_attribute),
        "delete_attribute" => Some(call_delete_attribute),
        "get_attribute" => Some(call_get_attribute),
        "list_attributes" => Some(call_list_attributes),
        "set_attribute" => Some(call_set_attribute),
        "add_behavior_override" => Some(call_add_behavior_override),
        "delete_behavior_override" => Some(call_delete_behavior_override),
        "get_behavior_override" => Some(call_get_behavior_override),
        "list_behavior_overrides" => Some(call_list_behavior_overrides),
        "list_behavior_overrides_resolved" => Some(call_list_behavior_overrides_resolved),
        "add_comparison_call" => Some(call_add_comparison_call),
        "delete_comparison_call" => Some(call_delete_comparison_call),
        "get_comparison_call" => Some(call_get_comparison_call),
        "list_comparison_calls" => Some(call_list_comparison_calls),
        "add_comparison_call_element" => Some(call_add_comparison_call_element),
        "delete_comparison_call_element" => Some(call_delete_comparison_call_element),
        "add_distinct_call" => Some(call_add_distinct_call),
        "delete_distinct_call" => Some(call_delete_distinct_call),
        "get_distinct_call" => Some(call_get_distinct_call),
        "list_distinct_calls" => Some(call_list_distinct_calls),
        "add_distinct_call_element" => Some(call_add_distinct_call_element),
        "delete_distinct_call_element" => Some(call_delete_distinct_call_element),
        "add_expression_call" => Some(call_add_expression_call),
        "delete_expression_call" => Some(call_delete_expression_call),
        "get_expression_call" => Some(call_get_expression_call),
        "list_expression_calls" => Some(call_list_expression_calls),
        "add_expression_call_element" => Some(call_add_expression_call_element),
        "delete_expression_call_element" => Some(call_delete_expression_call_element),
        "add_standardize_call" => Some(call_add_standardize_call),
        "delete_standardize_call" => Some(call_delete_standardize_call),
        "get_standardize_call" => Some(call_get_standardize_call),
        "list_standardize_calls" => Some(call_list_standardize_calls),
        "add_standardize_call_element" => Some(call_add_standardize_call_element),
        "delete_standardize_call_element" => Some(call_delete_standardize_call_element),
        "add_config_section" => Some(call_add_config_section),
        "remove_config_section" => Some(call_remove_config_section),
        "get_config_section" => Some(call_get_config_section),
        "config_section_is_empty" => Some(call_config_section_is_empty),
        "list_config_sections" => Some(call_list_config_sections),
        "remove_config_section_field" => Some(call_remove_config_section_field),
        "add_config_section_field" => Some(call_add_config_section_field),
        "add_data_source" => Some(call_add_data_source),
        "delete_data_source" => Some(call_delete_data_source),
        "get_data_source" => Some(call_get_data_source),
        "list_data_sources" => Some(call_list_data_sources),
        "set_data_source" => Some(call_set_data_source),
        "add_element" => Some(call_add_element),
        "delete_element" => Some(call_delete_element),
        "get_element" => Some(call_get_element),
        "list_elements" => Some(call_list_elements),
        "set_element" => Some(call_set_element),
        "set_feature_element" => Some(call_set_feature_element),
        "add_element_to_feature" => Some(call_add_element_to_feature),
        "delete_element_from_feature" => Some(call_delete_element_from_feature),
        "render_config" => Some(call_render_config),
        "add_feature" => Some(call_add_feature),
        "delete_feature" => Some(call_delete_feature),
        "get_feature" => Some(call_get_feature),
        "list_features" => Some(call_list_features),
        "set_feature" => Some(call_set_feature),
        "add_feature_comparison" => Some(call_add_feature_comparison),
        "delete_feature_comparison" => Some(call_delete_feature_comparison),
        "get_feature_comparison" => Some(call_get_feature_comparison),
        "list_feature_comparisons" => Some(call_list_feature_comparisons),
        "add_feature_distinct_call_element" => Some(call_add_feature_distinct_call_element),
        "list_feature_classes" => Some(call_list_feature_classes),
        "get_feature_class" => Some(call_get_feature_class),
        "update_feature_version" => Some(call_update_feature_version),
        "add_fragment" => Some(call_add_fragment),
        "delete_fragment" => Some(call_delete_fragment),
        "get_fragment" => Some(call_get_fragment),
        "list_fragments" => Some(call_list_fragments),
        "set_fragment" => Some(call_set_fragment),
        "add_comparison_function" => Some(call_add_comparison_function),
        "delete_comparison_function" => Some(call_delete_comparison_function),
        "delete_comparison_function_cascade" => Some(call_delete_comparison_function_cascade),
        "get_comparison_function" => Some(call_get_comparison_function),
        "list_comparison_functions" => Some(call_list_comparison_functions),
        "set_comparison_function" => Some(call_set_comparison_function),
        "add_distinct_function" => Some(call_add_distinct_function),
        "delete_distinct_function" => Some(call_delete_distinct_function),
        "get_distinct_function" => Some(call_get_distinct_function),
        "list_distinct_functions" => Some(call_list_distinct_functions),
        "set_distinct_function" => Some(call_set_distinct_function),
        "add_expression_function" => Some(call_add_expression_function),
        "delete_expression_function" => Some(call_delete_expression_function),
        "delete_expression_function_cascade" => Some(call_delete_expression_function_cascade),
        "get_expression_function" => Some(call_get_expression_function),
        "list_expression_functions" => Some(call_list_expression_functions),
        "set_expression_function" => Some(call_set_expression_function),
        "add_standardize_function" => Some(call_add_standardize_function),
        "delete_standardize_function" => Some(call_delete_standardize_function),
        "delete_standardize_function_cascade" => Some(call_delete_standardize_function_cascade),
        "get_standardize_function" => Some(call_get_standardize_function),
        "list_standardize_functions" => Some(call_list_standardize_functions),
        "set_standardize_function" => Some(call_set_standardize_function),
        "clone_generic_plan" => Some(call_clone_generic_plan),
        "delete_generic_plan" => Some(call_delete_generic_plan),
        "list_generic_plans" => Some(call_list_generic_plans),
        "set_generic_plan" => Some(call_set_generic_plan),
        "add_rule" => Some(call_add_rule),
        "delete_rule" => Some(call_delete_rule),
        "get_rule" => Some(call_get_rule),
        "list_rules" => Some(call_list_rules),
        "set_rule" => Some(call_set_rule),
        "add_search_profile" => Some(call_add_search_profile),
        "get_search_profile" => Some(call_get_search_profile),
        "list_search_profiles" => Some(call_list_search_profiles),
        "delete_search_profile" => Some(call_delete_search_profile),
        "set_setting" => Some(call_set_setting),
        "list_system_parameters" => Some(call_list_system_parameters),
        "set_system_parameter" => Some(call_set_system_parameter),
        "add_comparison_threshold" => Some(call_add_comparison_threshold),
        "delete_comparison_threshold" => Some(call_delete_comparison_threshold),
        "set_comparison_threshold" => Some(call_set_comparison_threshold),
        "list_comparison_thresholds" => Some(call_list_comparison_thresholds),
        "add_generic_threshold" => Some(call_add_generic_threshold),
        "delete_generic_threshold" => Some(call_delete_generic_threshold),
        "set_generic_threshold" => Some(call_set_generic_threshold),
        "list_generic_thresholds" => Some(call_list_generic_thresholds),
        "validate_generic_threshold" => Some(call_validate_generic_threshold),
        "validate_config" => Some(call_validate_config),
        "get_version" => Some(call_get_version),
        "get_compatibility_version" => Some(call_get_compatibility_version),
        "update_compatibility_version" => Some(call_update_compatibility_version),
        "verify_compatibility_version" => Some(call_verify_compatibility_version),
        _ => None,
    }
}

fn call_add_attribute(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["attribute", "feature", "element", "class", "default_value", "internal", "required", "id"])?;
    let result = sz_configtool_lib::attributes::add_attribute(
        config,
        sz_configtool_lib::attributes::AddAttributeParams {
            attribute: args.req_str("attribute")?,
            feature: args.req_str("feature")?,
            element: args.req_str("element")?,
            class: args.req_str("class")?,
            default_value: args.opt_str("default_value")?,
            internal: args.opt_str("internal")?,
            required: args.opt_str("required")?,
            id: args.opt_int("id")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_attribute(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::attributes::delete_attribute(
        config,
        args.req_str("code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_attribute(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::attributes::get_attribute(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_attributes(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::attributes::list_attributes(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_attribute(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["attribute", "internal", "required", "default_value"])?;
    let result = sz_configtool_lib::attributes::set_attribute(
        config,
        sz_configtool_lib::attributes::SetAttributeParams {
            attribute: args.req_str("attribute")?,
            internal: args.opt_str("internal")?,
            required: args.opt_str("required")?,
            default_value: args.opt_str("default_value")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_behavior_override(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature", "usage_type", "behavior"])?;
    let result = sz_configtool_lib::behavior_overrides::add_behavior_override(
        config,
        sz_configtool_lib::behavior_overrides::AddBehaviorOverrideParams {
            feature_code: args.req_str("feature")?,
            usage_type: args.req_str("usage_type")?,
            behavior: args.req_str("behavior")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_behavior_override(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature", "usage_type"])?;
    let result = sz_configtool_lib::behavior_overrides::delete_behavior_override(
        config,
        args.req_str("feature")?,
        args.req_str("usage_type")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_behavior_override(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature", "usage_type"])?;
    let result = sz_configtool_lib::behavior_overrides::get_behavior_override(
        config,
        args.req_str("feature")?,
        args.req_str("usage_type")?,
    )?;
    crate::output::json(result)
}

fn call_list_behavior_overrides(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::behavior_overrides::list_behavior_overrides(
        config,
    )?;
    crate::output::json(result)
}

fn call_list_behavior_overrides_resolved(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::behavior_overrides::list_behavior_overrides_resolved(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_comparison_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["ftype_code", "cfunc_code", "element_list", "id"])?;
    let result = sz_configtool_lib::calls::comparison::add_comparison_call(
        config,
        sz_configtool_lib::calls::comparison::AddComparisonCallParams {
            ftype_code: crate::convert::owned_str(args, "ftype_code")?,
            cfunc_code: crate::convert::owned_str(args, "cfunc_code")?,
            element_list: args.req_str_list("element_list")?,
            id: args.opt_int("id")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_comparison_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["cfcall_id"])?;
    let result = sz_configtool_lib::calls::comparison::delete_comparison_call(
        config,
        args.req_int("cfcall_id")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_comparison_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call"])?;
    let result = sz_configtool_lib::calls::comparison::get_comparison_call(
        config,
        crate::convert::call_selector(args, "call")?,
    )?;
    crate::output::json(result)
}

fn call_list_comparison_calls(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::calls::comparison::list_comparison_calls(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_comparison_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["cfcall_id", "ftype_id", "felem_id", "exec_order"])?;
    let result = sz_configtool_lib::calls::comparison::add_comparison_call_element(
        config,
        sz_configtool_lib::calls::comparison::AddComparisonCallElementParams {
            cfcall_id: args.req_int("cfcall_id")?,
            ftype_id: args.req_int("ftype_id")?,
            felem_id: args.req_int("felem_id")?,
            exec_order: args.opt_int("exec_order")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_comparison_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call", "element_code", "element_feature"])?;
    let result = sz_configtool_lib::calls::comparison::delete_comparison_call_element(
        config,
        crate::convert::call_selector(args, "call")?,
        args.req_str("element_code")?,
        args.opt_str("element_feature")?,
    )?;
    Ok(Output::Config(result))
}

fn call_add_distinct_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["ftype_code", "dfunc_code", "element_list"])?;
    let result = sz_configtool_lib::calls::distinct::add_distinct_call(
        config,
        sz_configtool_lib::calls::distinct::AddDistinctCallParams {
            ftype_code: crate::convert::owned_str(args, "ftype_code")?,
            dfunc_code: crate::convert::owned_str(args, "dfunc_code")?,
            element_list: args.req_str_list("element_list")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_distinct_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["dfcall_id"])?;
    let result = sz_configtool_lib::calls::distinct::delete_distinct_call(
        config,
        args.req_int("dfcall_id")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_distinct_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call"])?;
    let result = sz_configtool_lib::calls::distinct::get_distinct_call(
        config,
        crate::convert::call_selector(args, "call")?,
    )?;
    crate::output::json(result)
}

fn call_list_distinct_calls(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::calls::distinct::list_distinct_calls(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_distinct_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["dfcall_id", "ftype_id", "felem_id", "exec_order"])?;
    let result = sz_configtool_lib::calls::distinct::add_distinct_call_element(
        config,
        sz_configtool_lib::calls::distinct::AddDistinctCallElementParams {
            dfcall_id: args.req_int("dfcall_id")?,
            ftype_id: args.req_int("ftype_id")?,
            felem_id: args.req_int("felem_id")?,
            exec_order: args.opt_int("exec_order")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_distinct_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call", "element_code", "element_feature"])?;
    let result = sz_configtool_lib::calls::distinct::delete_distinct_call_element(
        config,
        crate::convert::call_selector(args, "call")?,
        args.req_str("element_code")?,
        args.opt_str("element_feature")?,
    )?;
    Ok(Output::Config(result))
}

fn call_add_expression_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["efunc_code", "element_list", "ftype_code", "felem_code", "exec_order", "expression_feature", "is_virtual"])?;
    let result = sz_configtool_lib::calls::expression::add_expression_call(
        config,
        sz_configtool_lib::calls::expression::AddExpressionCallParams {
            efunc_code: args.req_str("efunc_code")?,
            element_list: crate::convert::expression_element_list(args, "element_list")?,
            ftype_code: args.opt_str("ftype_code")?,
            felem_code: args.opt_str("felem_code")?,
            exec_order: args.opt_int("exec_order")?,
            expression_feature: args.opt_str("expression_feature")?,
            is_virtual: args.req_str("is_virtual")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_expression_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["efcall_id"])?;
    let result = sz_configtool_lib::calls::expression::delete_expression_call(
        config,
        args.req_int("efcall_id")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_expression_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call"])?;
    let result = sz_configtool_lib::calls::expression::get_expression_call(
        config,
        crate::convert::call_selector(args, "call")?,
    )?;
    crate::output::json(result)
}

fn call_list_expression_calls(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::calls::expression::list_expression_calls(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_expression_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["efcall_id", "ftype_id", "felem_id", "exec_order", "felem_req"])?;
    let result = sz_configtool_lib::calls::expression::add_expression_call_element(
        config,
        args.req_int("efcall_id")?,
        sz_configtool_lib::calls::expression::ExpressionCallElementParams {
            ftype_id: args.req_int("ftype_id")?,
            felem_id: args.req_int("felem_id")?,
            exec_order: args.opt_int("exec_order")?,
            felem_req: crate::convert::owned_str(args, "felem_req")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_expression_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call", "element_code", "element_feature"])?;
    let result = sz_configtool_lib::calls::expression::delete_expression_call_element(
        config,
        crate::convert::call_selector(args, "call")?,
        args.req_str("element_code")?,
        args.opt_str("element_feature")?,
    )?;
    Ok(Output::Config(result))
}

fn call_add_standardize_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["sfunc_code", "ftype_code", "felem_code", "exec_order"])?;
    let result = sz_configtool_lib::calls::standardize::add_standardize_call(
        config,
        sz_configtool_lib::calls::standardize::AddStandardizeCallParams {
            sfunc_code: args.req_str("sfunc_code")?,
            ftype_code: args.opt_str("ftype_code")?,
            felem_code: args.opt_str("felem_code")?,
            exec_order: args.opt_int("exec_order")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_standardize_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["sfcall_id"])?;
    let result = sz_configtool_lib::calls::standardize::delete_standardize_call(
        config,
        args.req_int("sfcall_id")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_standardize_call(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["call"])?;
    let result = sz_configtool_lib::calls::standardize::get_standardize_call(
        config,
        crate::convert::call_selector(args, "call")?,
    )?;
    crate::output::json(result)
}

fn call_list_standardize_calls(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::calls::standardize::list_standardize_calls(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_standardize_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["ftype_id", "sfunc_id", "felem_id", "exec_order"])?;
    let result = sz_configtool_lib::calls::standardize::add_standardize_call_element(
        config,
        sz_configtool_lib::calls::standardize::AddStandardizeCallElementParams {
            ftype_id: args.req_int("ftype_id")?,
            sfunc_id: args.req_int("sfunc_id")?,
            felem_id: args.opt_int("felem_id")?,
            exec_order: args.opt_int("exec_order")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_standardize_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["ftype_id", "sfunc_id", "felem_id"])?;
    let result = sz_configtool_lib::calls::standardize::delete_standardize_call_element(
        config,
        sz_configtool_lib::calls::standardize::DeleteStandardizeCallElementParams {
            ftype_id: args.req_int("ftype_id")?,
            sfunc_id: args.req_int("sfunc_id")?,
            felem_id: args.opt_int("felem_id")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_config_section(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name"])?;
    let result = sz_configtool_lib::config_sections::add_config_section(
        config,
        args.req_str("section_name")?,
    )?;
    Ok(Output::Config(result))
}

fn call_remove_config_section(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name"])?;
    let result = sz_configtool_lib::config_sections::remove_config_section(
        config,
        args.req_str("section_name")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_config_section(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name", "filter"])?;
    let result = sz_configtool_lib::config_sections::get_config_section(
        config,
        args.req_str("section_name")?,
        args.opt_str("filter")?,
    )?;
    crate::output::json(result)
}

fn call_config_section_is_empty(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name"])?;
    let result = sz_configtool_lib::config_sections::config_section_is_empty(
        config,
        args.req_str("section_name")?,
    )?;
    crate::output::json(result)
}

fn call_list_config_sections(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::config_sections::list_config_sections(
        config,
    )?;
    crate::output::json(result)
}

fn call_remove_config_section_field(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name", "field_name"])?;
    let result = sz_configtool_lib::config_sections::remove_config_section_field(
        config,
        args.req_str("section_name")?,
        args.req_str("field_name")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_add_config_section_field(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["section_name", "field_name", "field_value"])?;
    let result = sz_configtool_lib::config_sections::add_config_section_field(
        config,
        args.req_str("section_name")?,
        args.req_str("field_name")?,
        args.req_json("field_value")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_add_data_source(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "retention_level", "id"])?;
    let result = sz_configtool_lib::datasources::add_data_source(
        config,
        sz_configtool_lib::datasources::AddDataSourceParams {
            code: args.req_str("code")?,
            retention_level: args.opt_str("retention_level")?,
            id: args.opt_int("id")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_data_source(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::datasources::delete_data_source(
        config,
        args.req_str("code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_data_source(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::datasources::get_data_source(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_data_sources(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::datasources::list_data_sources(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_data_source(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "retention_level"])?;
    let result = sz_configtool_lib::datasources::set_data_source(
        config,
        sz_configtool_lib::datasources::SetDataSourceParams {
            code: args.req_str("code")?,
            retention_level: args.opt_str("retention_level")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "description", "data_type", "id"])?;
    let result = sz_configtool_lib::elements::add_element(
        config,
        sz_configtool_lib::elements::AddElementParams {
            code: args.req_str("code")?,
            description: args.opt_str("description")?,
            data_type: args.opt_str("data_type")?,
            id: args.opt_int("id")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::elements::delete_element(
        config,
        args.req_str("code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::elements::get_element(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_elements(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::elements::list_elements(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "description", "data_type"])?;
    let result = sz_configtool_lib::elements::set_element(
        config,
        sz_configtool_lib::elements::SetElementParams {
            code: args.req_str("code")?,
            description: args.opt_str("description")?,
            data_type: args.opt_str("data_type")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_set_feature_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code", "exec_order", "display_level", "display_delim", "derived"])?;
    let result = sz_configtool_lib::elements::set_feature_element(
        config,
        sz_configtool_lib::elements::SetFeatureElementParams {
            feature_code: crate::convert::required_str_as_some(args, "feature_code")?,
            element_code: crate::convert::required_str_as_some(args, "element_code")?,
            exec_order: args.opt_int("exec_order")?,
            display_level: args.opt_int("display_level")?,
            display_delim: args.opt_str("display_delim")?,
            derived: args.opt_str("derived")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_element_to_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code", "display_level", "display_delim", "derived"])?;
    let result = sz_configtool_lib::elements::add_element_to_feature(
        config,
        sz_configtool_lib::elements::AddElementToFeatureParams {
            feature_code: args.req_str("feature_code")?,
            element_code: args.req_str("element_code")?,
            display_level: args.opt_int("display_level")?,
            display_delim: args.opt_str("display_delim")?,
            derived: args.opt_str("derived")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_element_from_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code"])?;
    let result = sz_configtool_lib::elements::delete_element_from_feature(
        config,
        args.req_str("feature_code")?,
        args.req_str("element_code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_render_config(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["indent"])?;
    let result = sz_configtool_lib::export::render_config(
        config,
        crate::convert::req_usize(args, "indent")?,
    )?;
    crate::output::json(result)
}

fn call_add_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature", "element_list", "class", "behavior", "candidates", "anonymize", "derived", "history", "matchkey", "standardize", "expression", "comparison", "version", "rtype_id", "id"])?;
    let result = sz_configtool_lib::features::add_feature(
        config,
        sz_configtool_lib::features::AddFeatureParams {
            feature: args.req_str("feature")?,
            element_list: args.req_json("element_list")?,
            class: args.opt_str("class")?,
            behavior: args.opt_str("behavior")?,
            candidates: args.opt_str("candidates")?,
            anonymize: args.opt_str("anonymize")?,
            derived: args.opt_str("derived")?,
            history: args.opt_str("history")?,
            matchkey: args.opt_str("matchkey")?,
            standardize: args.opt_str("standardize")?,
            expression: args.opt_str("expression")?,
            comparison: args.opt_str("comparison")?,
            version: args.opt_int("version")?,
            rtype_id: args.opt_int("rtype_id")?,
            id: args.opt_int("id")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature"])?;
    let result = sz_configtool_lib::features::delete_feature(
        config,
        args.req_str("feature")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature"])?;
    let result = sz_configtool_lib::features::get_feature(
        config,
        args.req_str("feature")?,
    )?;
    crate::output::json(result)
}

fn call_list_features(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::features::list_features(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_feature(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature", "candidates", "anonymize", "derived", "history", "matchkey", "behavior", "class", "version", "rtype_id"])?;
    let result = sz_configtool_lib::features::set_feature(
        config,
        sz_configtool_lib::features::SetFeatureParams {
            feature: args.req_str("feature")?,
            candidates: args.opt_str("candidates")?,
            anonymize: args.opt_str("anonymize")?,
            derived: args.opt_str("derived")?,
            history: args.opt_str("history")?,
            matchkey: args.opt_str("matchkey")?,
            behavior: args.opt_str("behavior")?,
            class: args.opt_str("class")?,
            version: args.opt_int("version")?,
            rtype_id: args.opt_int("rtype_id")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_feature_comparison(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code", "exec_order", "display_level", "display_delim", "derived"])?;
    let result = sz_configtool_lib::features::add_feature_comparison(
        config,
        sz_configtool_lib::features::AddFeatureComparisonParams {
            feature_code: crate::convert::required_str_as_some(args, "feature_code")?,
            element_code: crate::convert::required_str_as_some(args, "element_code")?,
            exec_order: args.opt_int("exec_order")?,
            display_level: args.opt_int("display_level")?,
            display_delim: args.opt_str("display_delim")?,
            derived: args.opt_str("derived")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_feature_comparison(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code"])?;
    let result = sz_configtool_lib::features::delete_feature_comparison(
        config,
        args.req_str("feature_code")?,
        args.req_str("element_code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_feature_comparison(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "element_code"])?;
    let result = sz_configtool_lib::features::get_feature_comparison(
        config,
        sz_configtool_lib::features::GetFeatureComparisonParams {
            feature_code: crate::convert::required_str_as_some(args, "feature_code")?,
            element_code: crate::convert::required_str_as_some(args, "element_code")?,
        },
    )?;
    crate::output::json(result)
}

fn call_list_feature_comparisons(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::features::list_feature_comparisons(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_feature_distinct_call_element(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_code", "distinct_func_code", "element_code", "exec_order"])?;
    let result = sz_configtool_lib::features::add_feature_distinct_call_element(
        config,
        sz_configtool_lib::features::AddFeatureDistinctCallElementParams {
            feature_code: crate::convert::required_str_as_some(args, "feature_code")?,
            distinct_func_code: crate::convert::required_str_as_some(args, "distinct_func_code")?,
            element_code: args.opt_str("element_code")?,
            exec_order: args.opt_int("exec_order")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_list_feature_classes(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::features::list_feature_classes(
        config,
    )?;
    crate::output::json(result)
}

fn call_get_feature_class(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["feature_class"])?;
    let result = sz_configtool_lib::features::get_feature_class(
        config,
        args.req_str("feature_class")?,
    )?;
    crate::output::json(result)
}

fn call_update_feature_version(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["version"])?;
    let result = sz_configtool_lib::features::update_feature_version(
        config,
        args.req_str("version")?,
    )?;
    Ok(Output::Config(result))
}

fn call_add_fragment(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["fragment_config"])?;
    let result = sz_configtool_lib::fragments::add_fragment(
        config,
        args.req_json("fragment_config")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_fragment(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::fragments::delete_fragment(
        config,
        args.req_str("code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_fragment(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code_or_id"])?;
    let result = sz_configtool_lib::fragments::get_fragment(
        config,
        args.req_str("code_or_id")?,
    )?;
    crate::output::json(result)
}

fn call_list_fragments(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::fragments::list_fragments(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_fragment(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "source", "description"])?;
    let result = sz_configtool_lib::fragments::set_fragment(
        config,
        args.req_str("code")?,
        sz_configtool_lib::fragments::SetFragmentParams {
            source: args.tri_str("source")?,
            description: args.tri_str("description")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_comparison_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language", "anon_support"])?;
    let result = sz_configtool_lib::functions::comparison::add_comparison_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::comparison::AddComparisonFunctionParams {
            connect_str: args.opt_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
            anon_support: args.opt_str("anon_support")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_comparison_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::comparison::delete_comparison_function(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_comparison_function_cascade(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::comparison::delete_comparison_function_cascade(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_get_comparison_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::comparison::get_comparison_function(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_comparison_functions(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::functions::comparison::list_comparison_functions(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_comparison_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language", "anon_support"])?;
    let result = sz_configtool_lib::functions::comparison::set_comparison_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::comparison::SetComparisonFunctionParams {
            connect_str: args.tri_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
            anon_support: args.opt_str("anon_support")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_add_distinct_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language", "anon_support"])?;
    let result = sz_configtool_lib::functions::distinct::add_distinct_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::distinct::AddDistinctFunctionParams {
            connect_str: args.opt_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
            anon_support: args.opt_str("anon_support")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_distinct_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::distinct::delete_distinct_function(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_get_distinct_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::distinct::get_distinct_function(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_distinct_functions(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::functions::distinct::list_distinct_functions(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_distinct_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language", "anon_support"])?;
    let result = sz_configtool_lib::functions::distinct::set_distinct_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::distinct::SetDistinctFunctionParams {
            connect_str: args.tri_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
            anon_support: args.opt_str("anon_support")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_add_expression_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language"])?;
    let result = sz_configtool_lib::functions::expression::add_expression_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::expression::AddExpressionFunctionParams {
            connect_str: args.opt_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_expression_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::expression::delete_expression_function(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_expression_function_cascade(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::expression::delete_expression_function_cascade(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_get_expression_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::expression::get_expression_function(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_expression_functions(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::functions::expression::list_expression_functions(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_expression_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language"])?;
    let result = sz_configtool_lib::functions::expression::set_expression_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::expression::SetExpressionFunctionParams {
            connect_str: args.tri_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_add_standardize_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language"])?;
    let result = sz_configtool_lib::functions::standardize::add_standardize_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::standardize::AddStandardizeFunctionParams {
            connect_str: args.opt_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_standardize_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::standardize::delete_standardize_function(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_standardize_function_cascade(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::standardize::delete_standardize_function_cascade(
        config,
        args.req_str("code")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_get_standardize_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::functions::standardize::get_standardize_function(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_standardize_functions(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::functions::standardize::list_standardize_functions(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_standardize_function(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "connect_str", "description", "language"])?;
    let result = sz_configtool_lib::functions::standardize::set_standardize_function(
        config,
        args.req_str("code")?,
        sz_configtool_lib::functions::standardize::SetStandardizeFunctionParams {
            connect_str: args.tri_str("connect_str")?,
            description: args.opt_str("description")?,
            language: args.opt_str("language")?,
        },
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_clone_generic_plan(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["source_gplan_code", "new_gplan_code", "new_gplan_desc"])?;
    let result = sz_configtool_lib::generic_plans::clone_generic_plan(
        config,
        args.req_str("source_gplan_code")?,
        args.req_str("new_gplan_code")?,
        args.opt_str("new_gplan_desc")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_generic_plan(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["gplan_code"])?;
    let result = sz_configtool_lib::generic_plans::delete_generic_plan(
        config,
        args.req_str("gplan_code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_list_generic_plans(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["filter"])?;
    let result = sz_configtool_lib::generic_plans::list_generic_plans(
        config,
        args.opt_str("filter")?,
    )?;
    crate::output::json(result)
}

fn call_set_generic_plan(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["gplan_code", "gplan_desc"])?;
    let result = sz_configtool_lib::generic_plans::set_generic_plan(
        config,
        args.req_str("gplan_code")?,
        args.req_str("gplan_desc")?,
    )?;
    let (config, t0, t1) = result;
    crate::output::config_and_json(config, serde_json::json!({ "plan_id": t0, "was_created": t1 }))
}

fn call_add_rule(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["id", "rule_config"])?;
    let result = sz_configtool_lib::rules::add_rule(
        config,
        args.req_int("id")?,
        args.req_json("rule_config")?,
    )?;
    let (config, record) = result;
    crate::output::config_and_json(config, record)
}

fn call_delete_rule(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::rules::delete_rule(
        config,
        args.req_str("code")?,
    )?;
    Ok(Output::Config(result))
}

fn call_get_rule(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code_or_id"])?;
    let result = sz_configtool_lib::rules::get_rule(
        config,
        args.req_str("code_or_id")?,
    )?;
    crate::output::json(result)
}

fn call_list_rules(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::rules::list_rules(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_rule(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "resolve", "relate", "rtype_id", "fragment", "disqualifier", "tier"])?;
    let result = sz_configtool_lib::rules::set_rule(
        config,
        sz_configtool_lib::rules::SetRuleParams {
            code: args.req_str("code")?,
            resolve: args.opt_str("resolve")?,
            relate: args.opt_str("relate")?,
            rtype_id: args.opt_int("rtype_id")?,
            fragment: args.tri_str("fragment")?,
            disqualifier: args.tri_str("disqualifier")?,
            tier: args.tri_int("tier")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_add_search_profile(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code", "generic_plan", "candidates", "description", "elements"])?;
    let result = sz_configtool_lib::search_profiles::add_search_profile(
        config,
        sz_configtool_lib::search_profiles::AddSearchProfileParams {
            code: args.req_str("code")?,
            generic_plan: args.req_str("generic_plan")?,
            candidates: args.opt_str("candidates")?,
            description: args.opt_str("description")?,
            elements: crate::convert::search_profile_elements(args, "elements")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_get_search_profile(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["code"])?;
    let result = sz_configtool_lib::search_profiles::get_search_profile(
        config,
        args.req_str("code")?,
    )?;
    crate::output::json(result)
}

fn call_list_search_profiles(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["filter"])?;
    let result = sz_configtool_lib::search_profiles::list_search_profiles(
        config,
        args.opt_str("filter")?,
    )?;
    crate::output::json(result)
}

fn call_delete_search_profile(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["search_value"])?;
    let result = sz_configtool_lib::search_profiles::delete_search_profile(
        config,
        args.req_str("search_value")?,
    )?;
    Ok(Output::Config(result))
}

fn call_set_setting(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["name", "value"])?;
    let result = sz_configtool_lib::settings::set_setting(
        config,
        args.req_str("name")?,
        args.req_json_owned("value")?,
    )?;
    Ok(Output::Config(result))
}

fn call_list_system_parameters(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::system_params::list_system_parameters(
        config,
    )?;
    crate::output::json(result)
}

fn call_set_system_parameter(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["parameter_name", "parameter_value"])?;
    let result = sz_configtool_lib::system_params::set_system_parameter(
        config,
        args.req_str("parameter_name")?,
        args.req_json("parameter_value")?,
    )?;
    Ok(Output::Config(result))
}

fn call_add_comparison_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["cfunc_code", "ftype_code", "cfunc_rtnval", "exec_order", "same_score", "close_score", "likely_score", "plausible_score", "un_likely_score"])?;
    let result = sz_configtool_lib::thresholds::add_comparison_threshold(
        config,
        sz_configtool_lib::thresholds::AddComparisonThresholdParams {
            cfunc_code: args.opt_str("cfunc_code")?,
            ftype_code: args.opt_str("ftype_code")?,
            cfunc_rtnval: args.opt_str("cfunc_rtnval")?,
            exec_order: args.opt_int("exec_order")?,
            same_score: args.opt_int("same_score")?,
            close_score: args.opt_int("close_score")?,
            likely_score: args.opt_int("likely_score")?,
            plausible_score: args.opt_int("plausible_score")?,
            un_likely_score: args.opt_int("un_likely_score")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_comparison_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["cfunc_code", "ftype_code", "cfunc_rtnval"])?;
    let result = sz_configtool_lib::thresholds::delete_comparison_threshold(
        config,
        args.req_str("cfunc_code")?,
        args.req_str("ftype_code")?,
        args.req_str("cfunc_rtnval")?,
    )?;
    Ok(Output::Config(result))
}

fn call_set_comparison_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["cfunc_code", "ftype_code", "cfunc_rtnval", "exec_order", "same_score", "close_score", "likely_score", "plausible_score", "un_likely_score"])?;
    let result = sz_configtool_lib::thresholds::set_comparison_threshold(
        config,
        sz_configtool_lib::thresholds::SetComparisonThresholdParams {
            cfunc_code: args.opt_str("cfunc_code")?,
            ftype_code: args.opt_str("ftype_code")?,
            cfunc_rtnval: args.opt_str("cfunc_rtnval")?,
            exec_order: args.opt_int("exec_order")?,
            same_score: args.opt_int("same_score")?,
            close_score: args.opt_int("close_score")?,
            likely_score: args.opt_int("likely_score")?,
            plausible_score: args.opt_int("plausible_score")?,
            un_likely_score: args.opt_int("un_likely_score")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_list_comparison_thresholds(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::thresholds::list_comparison_thresholds(
        config,
    )?;
    crate::output::json(result)
}

fn call_add_generic_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["plan", "behavior", "scoring_cap", "candidate_cap", "send_to_redo", "feature"])?;
    let result = sz_configtool_lib::thresholds::add_generic_threshold(
        config,
        sz_configtool_lib::thresholds::AddGenericThresholdParams {
            plan: args.opt_str("plan")?,
            behavior: args.opt_str("behavior")?,
            scoring_cap: args.opt_int("scoring_cap")?,
            candidate_cap: args.opt_int("candidate_cap")?,
            send_to_redo: args.opt_str("send_to_redo")?,
            feature: args.opt_str("feature")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_delete_generic_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["plan", "behavior", "feature"])?;
    let result = sz_configtool_lib::thresholds::delete_generic_threshold(
        config,
        sz_configtool_lib::thresholds::DeleteGenericThresholdParams {
            plan: args.opt_str("plan")?,
            behavior: args.opt_str("behavior")?,
            feature: args.opt_str("feature")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_set_generic_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["plan", "behavior", "feature", "candidate_cap", "scoring_cap", "send_to_redo"])?;
    let result = sz_configtool_lib::thresholds::set_generic_threshold(
        config,
        sz_configtool_lib::thresholds::SetGenericThresholdParams {
            plan: args.opt_str("plan")?,
            behavior: args.opt_str("behavior")?,
            feature: args.opt_str("feature")?,
            candidate_cap: args.opt_int("candidate_cap")?,
            scoring_cap: args.opt_int("scoring_cap")?,
            send_to_redo: args.opt_str("send_to_redo")?,
        },
    )?;
    Ok(Output::Config(result))
}

fn call_list_generic_thresholds(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::thresholds::list_generic_thresholds(
        config,
    )?;
    crate::output::json(result)
}

fn call_validate_generic_threshold(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["plan", "behavior", "send_to_redo", "feature"])?;
    let result = sz_configtool_lib::thresholds::validate_generic_threshold(
        config,
        args.req_str("plan")?,
        args.req_str("behavior")?,
        args.req_str("send_to_redo")?,
        args.opt_str("feature")?,
    )?;
    crate::output::json(result)
}

fn call_validate_config(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    sz_configtool_lib::validation::validate_config(
        config,
    )?;
    Ok(Output::Unit)
}

fn call_get_version(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::versioning::get_version(
        config,
    )?;
    crate::output::json(result)
}

fn call_get_compatibility_version(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&[])?;
    let result = sz_configtool_lib::versioning::get_compatibility_version(
        config,
    )?;
    crate::output::json(result)
}

fn call_update_compatibility_version(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["new_version"])?;
    let result = sz_configtool_lib::versioning::update_compatibility_version(
        config,
        args.req_str("new_version")?,
    )?;
    Ok(Output::Config(result))
}

fn call_verify_compatibility_version(config: &str, args: &Args<'_>) -> Result<Output, ApiError> {
    args.check_known(&["expected_version"])?;
    let result = sz_configtool_lib::versioning::verify_compatibility_version(
        config,
        args.req_str("expected_version")?,
    )?;
    let (t0, t1) = result;
    crate::output::json(serde_json::json!({ "current_version": t0, "matches": t1 }))
}
