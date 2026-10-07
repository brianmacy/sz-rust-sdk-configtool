// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

#ifndef SZCONFIGTOOL_TESTS_GENERATED_TYPED_DISPATCH_HPP
#define SZCONFIGTOOL_TESTS_GENERATED_TYPED_DISPATCH_HPP

#include <map>
#include <optional>
#include <string>
#include <variant>
#include "conformance_support.hpp"
#include "szconfigtool/szconfigtool.hpp"

namespace szconfigtool_test {

/// Workspace-relative inputs (from project.yaml `paths`).
inline constexpr const char* kManifestJson = "api/manifest/generated/manifest.json";
inline constexpr const char* kConformanceJson = "api/manifest/generated/conformance.json";

/// Wire name -> typed call, for every IMPLEMENTED manifest function.
inline const std::map<std::string, TypedCall>& TypedFunctions() {
    static const std::map<std::string, TypedCall> table = {
    {"add_attribute", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"attribute", "feature", "element", "class", "default_value", "internal", "required", "id"});
        szconfigtool::AddAttributeOptions options;
        if (args.Has("default_value")) {
            options.default_value = args.Str("default_value");
        }
        if (args.Has("internal")) {
            options.internal = args.Str("internal");
        }
        if (args.Has("required")) {
            options.required = args.Str("required");
        }
        if (args.Has("id")) {
            options.id = args.Int("id");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddAttribute(config, args.Str("attribute"), args.Str("feature"), args.Str("element"), args.Str("class"), options));
    }},
    {"delete_attribute", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfig(szconfigtool::DeleteAttribute(config, args.Str("code")));
    }},
    {"get_attribute", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetAttribute(config, args.Str("code")));
    }},
    {"list_attributes", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListAttributes(config));
    }},
    {"set_attribute", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"attribute", "internal", "required", "default_value"});
        szconfigtool::SetAttributeOptions options;
        if (args.Has("internal")) {
            options.internal = args.Str("internal");
        }
        if (args.Has("required")) {
            options.required = args.Str("required");
        }
        if (args.Has("default_value")) {
            options.default_value = args.Str("default_value");
        }
        return Outcome::FromConfig(szconfigtool::SetAttribute(config, args.Str("attribute"), options));
    }},
    {"add_behavior_override", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature", "usage_type", "behavior"});
        return Outcome::FromConfig(szconfigtool::AddBehaviorOverride(config, args.Str("feature"), args.Str("usage_type"), args.Str("behavior")));
    }},
    {"delete_behavior_override", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature", "usage_type"});
        return Outcome::FromConfig(szconfigtool::DeleteBehaviorOverride(config, args.Str("feature"), args.Str("usage_type")));
    }},
    {"get_behavior_override", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature", "usage_type"});
        return Outcome::FromJson(szconfigtool::GetBehaviorOverride(config, args.Str("feature"), args.Str("usage_type")));
    }},
    {"list_behavior_overrides", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListBehaviorOverrides(config));
    }},
    {"list_behavior_overrides_resolved", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListBehaviorOverridesResolved(config));
    }},
    {"add_comparison_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"ftype_code", "cfunc_code", "element_list", "id"});
        szconfigtool::AddComparisonCallOptions options;
        if (args.Has("id")) {
            options.id = args.Int("id");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddComparisonCall(config, args.Str("ftype_code"), args.Str("cfunc_code"), args.StrList("element_list"), options));
    }},
    {"delete_comparison_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"cfcall_id"});
        return Outcome::FromConfig(szconfigtool::DeleteComparisonCall(config, args.Int("cfcall_id")));
    }},
    {"get_comparison_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call"});
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromJson(szconfigtool::GetComparisonCall(config, sz_v_call));
        }, args.IntOrStr("call"));
    }},
    {"list_comparison_calls", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListComparisonCalls(config));
    }},
    {"add_comparison_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"cfcall_id", "ftype_id", "felem_id", "exec_order"});
        szconfigtool::AddComparisonCallElementOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddComparisonCallElement(config, args.Int("cfcall_id"), args.Int("ftype_id"), args.Int("felem_id"), options));
    }},
    {"delete_comparison_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call", "element_code", "element_feature"});
        szconfigtool::DeleteComparisonCallElementOptions options;
        if (args.Has("element_feature")) {
            options.element_feature = args.Str("element_feature");
        }
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromConfig(szconfigtool::DeleteComparisonCallElement(config, sz_v_call, args.Str("element_code"), options));
        }, args.IntOrStr("call"));
    }},
    {"add_distinct_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"ftype_code", "dfunc_code", "element_list"});
        return Outcome::FromConfigAndJson(szconfigtool::AddDistinctCall(config, args.Str("ftype_code"), args.Str("dfunc_code"), args.StrList("element_list")));
    }},
    {"delete_distinct_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"dfcall_id"});
        return Outcome::FromConfig(szconfigtool::DeleteDistinctCall(config, args.Int("dfcall_id")));
    }},
    {"get_distinct_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call"});
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromJson(szconfigtool::GetDistinctCall(config, sz_v_call));
        }, args.IntOrStr("call"));
    }},
    {"list_distinct_calls", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListDistinctCalls(config));
    }},
    {"add_distinct_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"dfcall_id", "ftype_id", "felem_id", "exec_order"});
        szconfigtool::AddDistinctCallElementOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddDistinctCallElement(config, args.Int("dfcall_id"), args.Int("ftype_id"), args.Int("felem_id"), options));
    }},
    {"delete_distinct_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call", "element_code", "element_feature"});
        szconfigtool::DeleteDistinctCallElementOptions options;
        if (args.Has("element_feature")) {
            options.element_feature = args.Str("element_feature");
        }
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromConfig(szconfigtool::DeleteDistinctCallElement(config, sz_v_call, args.Str("element_code"), options));
        }, args.IntOrStr("call"));
    }},
    {"add_expression_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"efunc_code", "element_list", "ftype_code", "felem_code", "exec_order", "expression_feature", "is_virtual"});
        szconfigtool::AddExpressionCallOptions options;
        if (args.Has("ftype_code")) {
            options.ftype_code = args.Str("ftype_code");
        }
        if (args.Has("felem_code")) {
            options.felem_code = args.Str("felem_code");
        }
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        if (args.Has("expression_feature")) {
            options.expression_feature = args.Str("expression_feature");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddExpressionCall(config, args.Str("efunc_code"), args.Json("element_list"), args.Str("is_virtual"), options));
    }},
    {"delete_expression_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"efcall_id"});
        return Outcome::FromConfig(szconfigtool::DeleteExpressionCall(config, args.Int("efcall_id")));
    }},
    {"get_expression_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call"});
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromJson(szconfigtool::GetExpressionCall(config, sz_v_call));
        }, args.IntOrStr("call"));
    }},
    {"list_expression_calls", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListExpressionCalls(config));
    }},
    {"add_expression_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"efcall_id", "ftype_id", "felem_id", "exec_order", "felem_req"});
        szconfigtool::AddExpressionCallElementOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddExpressionCallElement(config, args.Int("efcall_id"), args.Int("ftype_id"), args.Int("felem_id"), args.Str("felem_req"), options));
    }},
    {"delete_expression_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call", "element_code", "element_feature"});
        szconfigtool::DeleteExpressionCallElementOptions options;
        if (args.Has("element_feature")) {
            options.element_feature = args.Str("element_feature");
        }
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromConfig(szconfigtool::DeleteExpressionCallElement(config, sz_v_call, args.Str("element_code"), options));
        }, args.IntOrStr("call"));
    }},
    {"add_standardize_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"sfunc_code", "ftype_code", "felem_code", "exec_order"});
        szconfigtool::AddStandardizeCallOptions options;
        if (args.Has("ftype_code")) {
            options.ftype_code = args.Str("ftype_code");
        }
        if (args.Has("felem_code")) {
            options.felem_code = args.Str("felem_code");
        }
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddStandardizeCall(config, args.Str("sfunc_code"), options));
    }},
    {"delete_standardize_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"sfcall_id"});
        return Outcome::FromConfig(szconfigtool::DeleteStandardizeCall(config, args.Int("sfcall_id")));
    }},
    {"get_standardize_call", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"call"});
        return std::visit([&](const auto& sz_v_call) -> Outcome {
            return Outcome::FromJson(szconfigtool::GetStandardizeCall(config, sz_v_call));
        }, args.IntOrStr("call"));
    }},
    {"list_standardize_calls", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListStandardizeCalls(config));
    }},
    {"add_standardize_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"ftype_id", "sfunc_id", "felem_id", "exec_order"});
        szconfigtool::AddStandardizeCallElementOptions options;
        if (args.Has("felem_id")) {
            options.felem_id = args.Int("felem_id");
        }
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddStandardizeCallElement(config, args.Int("ftype_id"), args.Int("sfunc_id"), options));
    }},
    {"delete_standardize_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"ftype_id", "sfunc_id", "felem_id"});
        szconfigtool::DeleteStandardizeCallElementOptions options;
        if (args.Has("felem_id")) {
            options.felem_id = args.Int("felem_id");
        }
        return Outcome::FromConfig(szconfigtool::DeleteStandardizeCallElement(config, args.Int("ftype_id"), args.Int("sfunc_id"), options));
    }},
    {"add_config_section", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name"});
        return Outcome::FromConfig(szconfigtool::AddConfigSection(config, args.Str("section_name")));
    }},
    {"remove_config_section", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name"});
        return Outcome::FromConfig(szconfigtool::RemoveConfigSection(config, args.Str("section_name")));
    }},
    {"get_config_section", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name", "filter"});
        szconfigtool::GetConfigSectionOptions options;
        if (args.Has("filter")) {
            options.filter = args.Str("filter");
        }
        return Outcome::FromJson(szconfigtool::GetConfigSection(config, args.Str("section_name"), options));
    }},
    {"config_section_is_empty", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name"});
        return Outcome::FromJson(szconfigtool::ConfigSectionIsEmpty(config, args.Str("section_name")));
    }},
    {"list_config_sections", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListConfigSections(config));
    }},
    {"remove_config_section_field", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name", "field_name"});
        return Outcome::FromConfigAndJson(szconfigtool::RemoveConfigSectionField(config, args.Str("section_name"), args.Str("field_name")));
    }},
    {"add_config_section_field", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"section_name", "field_name", "field_value"});
        return Outcome::FromConfigAndJson(szconfigtool::AddConfigSectionField(config, args.Str("section_name"), args.Str("field_name"), args.Json("field_value")));
    }},
    {"add_data_source", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "retention_level", "id"});
        szconfigtool::AddDataSourceOptions options;
        if (args.Has("retention_level")) {
            options.retention_level = args.Str("retention_level");
        }
        if (args.Has("id")) {
            options.id = args.Int("id");
        }
        return Outcome::FromConfig(szconfigtool::AddDataSource(config, args.Str("code"), options));
    }},
    {"delete_data_source", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfig(szconfigtool::DeleteDataSource(config, args.Str("code")));
    }},
    {"get_data_source", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetDataSource(config, args.Str("code")));
    }},
    {"list_data_sources", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListDataSources(config));
    }},
    {"set_data_source", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "retention_level"});
        szconfigtool::SetDataSourceOptions options;
        if (args.Has("retention_level")) {
            options.retention_level = args.Str("retention_level");
        }
        return Outcome::FromConfig(szconfigtool::SetDataSource(config, args.Str("code"), options));
    }},
    {"add_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "description", "data_type", "id"});
        szconfigtool::AddElementOptions options;
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("data_type")) {
            options.data_type = args.Str("data_type");
        }
        if (args.Has("id")) {
            options.id = args.Int("id");
        }
        return Outcome::FromConfig(szconfigtool::AddElement(config, args.Str("code"), options));
    }},
    {"delete_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfig(szconfigtool::DeleteElement(config, args.Str("code")));
    }},
    {"get_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetElement(config, args.Str("code")));
    }},
    {"list_elements", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListElements(config));
    }},
    {"set_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "description", "data_type"});
        szconfigtool::SetElementOptions options;
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("data_type")) {
            options.data_type = args.Str("data_type");
        }
        return Outcome::FromConfig(szconfigtool::SetElement(config, args.Str("code"), options));
    }},
    {"set_feature_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code", "exec_order", "display_level", "display_delim", "derived"});
        szconfigtool::SetFeatureElementOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        if (args.Has("display_level")) {
            options.display_level = args.Int("display_level");
        }
        if (args.Has("display_delim")) {
            options.display_delim = args.Str("display_delim");
        }
        if (args.Has("derived")) {
            options.derived = args.Str("derived");
        }
        return Outcome::FromConfig(szconfigtool::SetFeatureElement(config, args.Str("feature_code"), args.Str("element_code"), options));
    }},
    {"add_element_to_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code", "display_level", "display_delim", "derived"});
        szconfigtool::AddElementToFeatureOptions options;
        if (args.Has("display_level")) {
            options.display_level = args.Int("display_level");
        }
        if (args.Has("display_delim")) {
            options.display_delim = args.Str("display_delim");
        }
        if (args.Has("derived")) {
            options.derived = args.Str("derived");
        }
        return Outcome::FromConfig(szconfigtool::AddElementToFeature(config, args.Str("feature_code"), args.Str("element_code"), options));
    }},
    {"delete_element_from_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code"});
        return Outcome::FromConfig(szconfigtool::DeleteElementFromFeature(config, args.Str("feature_code"), args.Str("element_code")));
    }},
    {"render_config", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"indent"});
        return Outcome::FromJson(szconfigtool::RenderConfig(config, args.Int("indent")));
    }},
    {"add_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature", "element_list", "class", "behavior", "candidates", "anonymize", "derived", "history", "matchkey", "standardize", "expression", "comparison", "version", "rtype_id", "id"});
        szconfigtool::AddFeatureOptions options;
        if (args.Has("class")) {
            options.class_ = args.Str("class");
        }
        if (args.Has("behavior")) {
            options.behavior = args.Str("behavior");
        }
        if (args.Has("candidates")) {
            options.candidates = args.Str("candidates");
        }
        if (args.Has("anonymize")) {
            options.anonymize = args.Str("anonymize");
        }
        if (args.Has("derived")) {
            options.derived = args.Str("derived");
        }
        if (args.Has("history")) {
            options.history = args.Str("history");
        }
        if (args.Has("matchkey")) {
            options.matchkey = args.Str("matchkey");
        }
        if (args.Has("standardize")) {
            options.standardize = args.Str("standardize");
        }
        if (args.Has("expression")) {
            options.expression = args.Str("expression");
        }
        if (args.Has("comparison")) {
            options.comparison = args.Str("comparison");
        }
        if (args.Has("version")) {
            options.version = args.Int("version");
        }
        if (args.Has("rtype_id")) {
            options.rtype_id = args.Int("rtype_id");
        }
        if (args.Has("id")) {
            options.id = args.Int("id");
        }
        return Outcome::FromConfig(szconfigtool::AddFeature(config, args.Str("feature"), args.Json("element_list"), options));
    }},
    {"delete_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature"});
        return Outcome::FromConfig(szconfigtool::DeleteFeature(config, args.Str("feature")));
    }},
    {"get_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature"});
        return Outcome::FromJson(szconfigtool::GetFeature(config, args.Str("feature")));
    }},
    {"list_features", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListFeatures(config));
    }},
    {"set_feature", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature", "candidates", "anonymize", "derived", "history", "matchkey", "behavior", "class", "version", "rtype_id"});
        szconfigtool::SetFeatureOptions options;
        if (args.Has("candidates")) {
            options.candidates = args.Str("candidates");
        }
        if (args.Has("anonymize")) {
            options.anonymize = args.Str("anonymize");
        }
        if (args.Has("derived")) {
            options.derived = args.Str("derived");
        }
        if (args.Has("history")) {
            options.history = args.Str("history");
        }
        if (args.Has("matchkey")) {
            options.matchkey = args.Str("matchkey");
        }
        if (args.Has("behavior")) {
            options.behavior = args.Str("behavior");
        }
        if (args.Has("class")) {
            options.class_ = args.Str("class");
        }
        if (args.Has("version")) {
            options.version = args.Int("version");
        }
        if (args.Has("rtype_id")) {
            options.rtype_id = args.Int("rtype_id");
        }
        return Outcome::FromConfig(szconfigtool::SetFeature(config, args.Str("feature"), options));
    }},
    {"add_feature_comparison", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code", "exec_order", "display_level", "display_delim", "derived"});
        szconfigtool::AddFeatureComparisonOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        if (args.Has("display_level")) {
            options.display_level = args.Int("display_level");
        }
        if (args.Has("display_delim")) {
            options.display_delim = args.Str("display_delim");
        }
        if (args.Has("derived")) {
            options.derived = args.Str("derived");
        }
        return Outcome::FromConfig(szconfigtool::AddFeatureComparison(config, args.Str("feature_code"), args.Str("element_code"), options));
    }},
    {"delete_feature_comparison", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code"});
        return Outcome::FromConfig(szconfigtool::DeleteFeatureComparison(config, args.Str("feature_code"), args.Str("element_code")));
    }},
    {"get_feature_comparison", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "element_code"});
        return Outcome::FromJson(szconfigtool::GetFeatureComparison(config, args.Str("feature_code"), args.Str("element_code")));
    }},
    {"list_feature_comparisons", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListFeatureComparisons(config));
    }},
    {"add_feature_distinct_call_element", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_code", "distinct_func_code", "element_code", "exec_order"});
        szconfigtool::AddFeatureDistinctCallElementOptions options;
        if (args.Has("element_code")) {
            options.element_code = args.Str("element_code");
        }
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        return Outcome::FromConfig(szconfigtool::AddFeatureDistinctCallElement(config, args.Str("feature_code"), args.Str("distinct_func_code"), options));
    }},
    {"list_feature_classes", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListFeatureClasses(config));
    }},
    {"get_feature_class", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"feature_class"});
        return Outcome::FromJson(szconfigtool::GetFeatureClass(config, args.Str("feature_class")));
    }},
    {"update_feature_version", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"version"});
        return Outcome::FromConfig(szconfigtool::UpdateFeatureVersion(config, args.Str("version")));
    }},
    {"add_fragment", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"fragment_config"});
        return Outcome::FromConfigAndJson(szconfigtool::AddFragment(config, args.Json("fragment_config")));
    }},
    {"delete_fragment", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfig(szconfigtool::DeleteFragment(config, args.Str("code")));
    }},
    {"get_fragment", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code_or_id"});
        return Outcome::FromJson(szconfigtool::GetFragment(config, args.Str("code_or_id")));
    }},
    {"list_fragments", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListFragments(config));
    }},
    {"set_fragment", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "source", "description"});
        szconfigtool::SetFragmentOptions options;
        if (args.Has("source")) {
            options.source = args.IsNull("source") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("source"));
        }
        if (args.Has("description")) {
            options.description = args.IsNull("description") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("description"));
        }
        return Outcome::FromConfig(szconfigtool::SetFragment(config, args.Str("code"), options));
    }},
    {"add_comparison_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language", "anon_support"});
        szconfigtool::AddComparisonFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.Str("connect_str");
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        if (args.Has("anon_support")) {
            options.anon_support = args.Str("anon_support");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddComparisonFunction(config, args.Str("code"), options));
    }},
    {"delete_comparison_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteComparisonFunction(config, args.Str("code")));
    }},
    {"delete_comparison_function_cascade", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteComparisonFunctionCascade(config, args.Str("code")));
    }},
    {"get_comparison_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetComparisonFunction(config, args.Str("code")));
    }},
    {"list_comparison_functions", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListComparisonFunctions(config));
    }},
    {"set_comparison_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language", "anon_support"});
        szconfigtool::SetComparisonFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.IsNull("connect_str") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("connect_str"));
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        if (args.Has("anon_support")) {
            options.anon_support = args.Str("anon_support");
        }
        return Outcome::FromConfigAndJson(szconfigtool::SetComparisonFunction(config, args.Str("code"), options));
    }},
    {"add_distinct_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language", "anon_support"});
        szconfigtool::AddDistinctFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.Str("connect_str");
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        if (args.Has("anon_support")) {
            options.anon_support = args.Str("anon_support");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddDistinctFunction(config, args.Str("code"), options));
    }},
    {"delete_distinct_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteDistinctFunction(config, args.Str("code")));
    }},
    {"get_distinct_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetDistinctFunction(config, args.Str("code")));
    }},
    {"list_distinct_functions", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListDistinctFunctions(config));
    }},
    {"set_distinct_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language", "anon_support"});
        szconfigtool::SetDistinctFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.IsNull("connect_str") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("connect_str"));
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        if (args.Has("anon_support")) {
            options.anon_support = args.Str("anon_support");
        }
        return Outcome::FromConfigAndJson(szconfigtool::SetDistinctFunction(config, args.Str("code"), options));
    }},
    {"add_expression_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language"});
        szconfigtool::AddExpressionFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.Str("connect_str");
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddExpressionFunction(config, args.Str("code"), options));
    }},
    {"delete_expression_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteExpressionFunction(config, args.Str("code")));
    }},
    {"delete_expression_function_cascade", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteExpressionFunctionCascade(config, args.Str("code")));
    }},
    {"get_expression_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetExpressionFunction(config, args.Str("code")));
    }},
    {"list_expression_functions", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListExpressionFunctions(config));
    }},
    {"set_expression_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language"});
        szconfigtool::SetExpressionFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.IsNull("connect_str") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("connect_str"));
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        return Outcome::FromConfigAndJson(szconfigtool::SetExpressionFunction(config, args.Str("code"), options));
    }},
    {"add_standardize_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language"});
        szconfigtool::AddStandardizeFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.Str("connect_str");
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        return Outcome::FromConfigAndJson(szconfigtool::AddStandardizeFunction(config, args.Str("code"), options));
    }},
    {"delete_standardize_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteStandardizeFunction(config, args.Str("code")));
    }},
    {"delete_standardize_function_cascade", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfigAndJson(szconfigtool::DeleteStandardizeFunctionCascade(config, args.Str("code")));
    }},
    {"get_standardize_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetStandardizeFunction(config, args.Str("code")));
    }},
    {"list_standardize_functions", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListStandardizeFunctions(config));
    }},
    {"set_standardize_function", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "connect_str", "description", "language"});
        szconfigtool::SetStandardizeFunctionOptions options;
        if (args.Has("connect_str")) {
            options.connect_str = args.IsNull("connect_str") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("connect_str"));
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("language")) {
            options.language = args.Str("language");
        }
        return Outcome::FromConfigAndJson(szconfigtool::SetStandardizeFunction(config, args.Str("code"), options));
    }},
    {"clone_generic_plan", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"source_gplan_code", "new_gplan_code", "new_gplan_desc"});
        szconfigtool::CloneGenericPlanOptions options;
        if (args.Has("new_gplan_desc")) {
            options.new_gplan_desc = args.Str("new_gplan_desc");
        }
        return Outcome::FromConfigAndJson(szconfigtool::CloneGenericPlan(config, args.Str("source_gplan_code"), args.Str("new_gplan_code"), options));
    }},
    {"delete_generic_plan", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"gplan_code"});
        return Outcome::FromConfig(szconfigtool::DeleteGenericPlan(config, args.Str("gplan_code")));
    }},
    {"list_generic_plans", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"filter"});
        szconfigtool::ListGenericPlansOptions options;
        if (args.Has("filter")) {
            options.filter = args.Str("filter");
        }
        return Outcome::FromJson(szconfigtool::ListGenericPlans(config, options));
    }},
    {"set_generic_plan", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"gplan_code", "gplan_desc"});
        const auto sz_r = szconfigtool::SetGenericPlan(config, args.Str("gplan_code"), args.Str("gplan_desc"));
        return Outcome::FromRecord("config_and_json", std::optional<std::string>(sz_r.config), {{"plan_id", sz_r.plan_id}, {"was_created", sz_r.was_created}});
    }},
    {"add_rule", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"id", "rule_config"});
        return Outcome::FromConfigAndJson(szconfigtool::AddRule(config, args.Int("id"), args.Json("rule_config")));
    }},
    {"delete_rule", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromConfig(szconfigtool::DeleteRule(config, args.Str("code")));
    }},
    {"get_rule", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code_or_id"});
        return Outcome::FromJson(szconfigtool::GetRule(config, args.Str("code_or_id")));
    }},
    {"list_rules", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListRules(config));
    }},
    {"set_rule", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "resolve", "relate", "rtype_id", "fragment", "disqualifier", "tier"});
        szconfigtool::SetRuleOptions options;
        if (args.Has("resolve")) {
            options.resolve = args.Str("resolve");
        }
        if (args.Has("relate")) {
            options.relate = args.Str("relate");
        }
        if (args.Has("rtype_id")) {
            options.rtype_id = args.Int("rtype_id");
        }
        if (args.Has("fragment")) {
            options.fragment = args.IsNull("fragment") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("fragment"));
        }
        if (args.Has("disqualifier")) {
            options.disqualifier = args.IsNull("disqualifier") ? szconfigtool::FieldUpdate<std::string>::Clear() : szconfigtool::FieldUpdate<std::string>::Set(args.Str("disqualifier"));
        }
        if (args.Has("tier")) {
            options.tier = args.IsNull("tier") ? szconfigtool::FieldUpdate<std::int64_t>::Clear() : szconfigtool::FieldUpdate<std::int64_t>::Set(args.Int("tier"));
        }
        return Outcome::FromConfig(szconfigtool::SetRule(config, args.Str("code"), options));
    }},
    {"add_search_profile", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code", "generic_plan", "candidates", "description", "elements"});
        szconfigtool::AddSearchProfileOptions options;
        if (args.Has("candidates")) {
            options.candidates = args.Str("candidates");
        }
        if (args.Has("description")) {
            options.description = args.Str("description");
        }
        if (args.Has("elements")) {
            options.elements = args.Json("elements");
        }
        return Outcome::FromConfig(szconfigtool::AddSearchProfile(config, args.Str("code"), args.Str("generic_plan"), options));
    }},
    {"get_search_profile", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"code"});
        return Outcome::FromJson(szconfigtool::GetSearchProfile(config, args.Str("code")));
    }},
    {"list_search_profiles", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"filter"});
        szconfigtool::ListSearchProfilesOptions options;
        if (args.Has("filter")) {
            options.filter = args.Str("filter");
        }
        return Outcome::FromJson(szconfigtool::ListSearchProfiles(config, options));
    }},
    {"delete_search_profile", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"search_value"});
        return Outcome::FromConfig(szconfigtool::DeleteSearchProfile(config, args.Str("search_value")));
    }},
    {"set_setting", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"name", "value"});
        return Outcome::FromConfig(szconfigtool::SetSetting(config, args.Str("name"), args.Json("value")));
    }},
    {"list_system_parameters", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListSystemParameters(config));
    }},
    {"set_system_parameter", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"parameter_name", "parameter_value"});
        return Outcome::FromConfig(szconfigtool::SetSystemParameter(config, args.Str("parameter_name"), args.Json("parameter_value")));
    }},
    {"add_comparison_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"cfunc_code", "ftype_code", "cfunc_rtnval", "exec_order", "same_score", "close_score", "likely_score", "plausible_score", "un_likely_score"});
        szconfigtool::AddComparisonThresholdOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        if (args.Has("same_score")) {
            options.same_score = args.Int("same_score");
        }
        if (args.Has("close_score")) {
            options.close_score = args.Int("close_score");
        }
        if (args.Has("likely_score")) {
            options.likely_score = args.Int("likely_score");
        }
        if (args.Has("plausible_score")) {
            options.plausible_score = args.Int("plausible_score");
        }
        if (args.Has("un_likely_score")) {
            options.un_likely_score = args.Int("un_likely_score");
        }
        return Outcome::FromConfig(szconfigtool::AddComparisonThreshold(config, args.Str("cfunc_code"), args.Str("ftype_code"), args.Str("cfunc_rtnval"), options));
    }},
    {"delete_comparison_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"cfunc_code", "ftype_code", "cfunc_rtnval"});
        return Outcome::FromConfig(szconfigtool::DeleteComparisonThreshold(config, args.Str("cfunc_code"), args.Str("ftype_code"), args.Str("cfunc_rtnval")));
    }},
    {"set_comparison_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"cfunc_code", "ftype_code", "cfunc_rtnval", "exec_order", "same_score", "close_score", "likely_score", "plausible_score", "un_likely_score"});
        szconfigtool::SetComparisonThresholdOptions options;
        if (args.Has("exec_order")) {
            options.exec_order = args.Int("exec_order");
        }
        if (args.Has("same_score")) {
            options.same_score = args.Int("same_score");
        }
        if (args.Has("close_score")) {
            options.close_score = args.Int("close_score");
        }
        if (args.Has("likely_score")) {
            options.likely_score = args.Int("likely_score");
        }
        if (args.Has("plausible_score")) {
            options.plausible_score = args.Int("plausible_score");
        }
        if (args.Has("un_likely_score")) {
            options.un_likely_score = args.Int("un_likely_score");
        }
        return Outcome::FromConfig(szconfigtool::SetComparisonThreshold(config, args.Str("cfunc_code"), args.Str("ftype_code"), args.Str("cfunc_rtnval"), options));
    }},
    {"list_comparison_thresholds", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListComparisonThresholds(config));
    }},
    {"add_generic_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"plan", "behavior", "scoring_cap", "candidate_cap", "send_to_redo", "feature"});
        szconfigtool::AddGenericThresholdOptions options;
        if (args.Has("feature")) {
            options.feature = args.Str("feature");
        }
        return Outcome::FromConfig(szconfigtool::AddGenericThreshold(config, args.Str("plan"), args.Str("behavior"), args.Int("scoring_cap"), args.Int("candidate_cap"), args.Str("send_to_redo"), options));
    }},
    {"delete_generic_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"plan", "behavior", "feature"});
        szconfigtool::DeleteGenericThresholdOptions options;
        if (args.Has("feature")) {
            options.feature = args.Str("feature");
        }
        return Outcome::FromConfig(szconfigtool::DeleteGenericThreshold(config, args.Str("plan"), args.Str("behavior"), options));
    }},
    {"set_generic_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"plan", "behavior", "feature", "candidate_cap", "scoring_cap", "send_to_redo"});
        szconfigtool::SetGenericThresholdOptions options;
        if (args.Has("feature")) {
            options.feature = args.Str("feature");
        }
        if (args.Has("candidate_cap")) {
            options.candidate_cap = args.Int("candidate_cap");
        }
        if (args.Has("scoring_cap")) {
            options.scoring_cap = args.Int("scoring_cap");
        }
        if (args.Has("send_to_redo")) {
            options.send_to_redo = args.Str("send_to_redo");
        }
        return Outcome::FromConfig(szconfigtool::SetGenericThreshold(config, args.Str("plan"), args.Str("behavior"), options));
    }},
    {"list_generic_thresholds", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::ListGenericThresholds(config));
    }},
    {"validate_generic_threshold", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"plan", "behavior", "send_to_redo", "feature"});
        szconfigtool::ValidateGenericThresholdOptions options;
        if (args.Has("feature")) {
            options.feature = args.Str("feature");
        }
        return Outcome::FromJson(szconfigtool::ValidateGenericThreshold(config, args.Str("plan"), args.Str("behavior"), args.Str("send_to_redo"), options));
    }},
    {"validate_config", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        szconfigtool::ValidateConfig(config);
        return Outcome::Unit();
    }},
    {"get_version", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::GetVersion(config));
    }},
    {"get_compatibility_version", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({});
        return Outcome::FromJson(szconfigtool::GetCompatibilityVersion(config));
    }},
    {"update_compatibility_version", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"new_version"});
        return Outcome::FromConfig(szconfigtool::UpdateCompatibilityVersion(config, args.Str("new_version")));
    }},
    {"verify_compatibility_version", [](const std::string& config, const TestArgs& args) -> Outcome {
        args.CheckKnown({"expected_version"});
        const auto sz_r = szconfigtool::VerifyCompatibilityVersion(config, args.Str("expected_version"));
        return Outcome::FromRecord("json", std::nullopt, {{"current_version", sz_r.current_version}, {"matches", sz_r.matches}});
    }},
    };
    return table;
}

}  // namespace szconfigtool_test

#endif  // SZCONFIGTOOL_TESTS_GENERATED_TYPED_DISPATCH_HPP
