// Typed-API tests against the REAL libSzConfigTool: naming / coverage,
// optional + tri-state + required args, error mapping, config opacity.
#include <gtest/gtest.h>

#include <cstdint>
#include <set>
#include <string>
#include <string_view>
#include <type_traits>

#include "conformance_support.hpp"
#include "generated/typed_dispatch.hpp"

namespace szconfigtool_test {
namespace {

namespace sz = szconfigtool;
using sz::ErrorKind;
using sz::SzConfigToolException;

const json::Value& Manifest() {
    static const std::string text = ReadFile(WorkspacePath(kManifestJson));
    static const json::Value root = json::Parse(text);
    return root;
}

std::string Fixture() {
    static const std::string text = [] {
        const std::string conf = ReadFile(WorkspacePath(kConformanceJson));
        return ReadFile(WorkspacePath(json::Parse(conf).Find("fixture")->text));
    }();
    return text;
}

json::Value ParseText(const std::string& text) { return json::Parse(text); }

/// Run `fn`, returning the exception it throws (fails if none).
template <class F>
SzConfigToolException Throws(F&& fn) {
    try {
        fn();
    } catch (const SzConfigToolException& e) {
        return e;
    }
    ADD_FAILURE() << "no SzConfigToolException";
    return SzConfigToolException("", "");
}

// ---- naming / coverage ----

TEST(Naming, TypedApiCoversExactlyTheImplementedFunctions) {
    std::set<std::string> implemented;
    for (const auto& f : Manifest().Find("functions")->items) {
        if (f.Find("status")->text == "implemented") {
            implemented.insert(f.Find("name")->text);
        }
    }
    std::set<std::string> typed;
    for (const auto& [name, call] : TypedFunctions()) {
        typed.insert(name);
    }
    EXPECT_EQ(typed, implemented);
    EXPECT_GT(Manifest().Find("functions")->items.size(), typed.size());
}

TEST(Naming, PascalCaseFunctionsSnakeCaseArgs) {
    // Compile-time evidence: PascalCase names, snake_case option fields, and
    // the C++-keyword arg `class` exposed as `class_`.
    const sz::AddAttributeOptions opts{.default_value = "d", .internal = "No", .id = 0};
    const sz::ConfigAndJson r = sz::AddAttribute(Fixture(), "MY_ATTR", "NAME", "FULL_NAME",
                                                 /*class_=*/"OTHER", opts);
    EXPECT_EQ(ParseText(r.json).Find("ATTR_CODE")->text, "MY_ATTR");
    EXPECT_EQ(ParseText(r.json).Find("DEFAULT_VALUE")->text, "d");
}

// ---- optional / required / tri-state ----

TEST(Args, OptionalArgsUseDesignatedInitializers) {
    const std::string cfg = sz::AddDataSource(Fixture(), "crm", {.id = 4242});
    const json::Value ds = ParseText(sz::GetDataSource(cfg, "CRM"));
    EXPECT_EQ(ds.Find("DSRC_ID")->text, "4242");
    // Omitted optional = library default (auto-allocated id).
    const std::string cfg2 = sz::AddDataSource(cfg, "web");
    EXPECT_NE(ParseText(sz::GetDataSource(cfg2, "WEB")).Find("DSRC_ID")->text, "4242");
}

template <class S>
concept CallsWithPlanOnly =
    requires(const S& s) { sz::DeleteGenericThreshold(s, std::string_view{}); };
template <class S>
concept CallsWithPlanAndBehavior = requires(const S& s) {
    sz::DeleteGenericThreshold(s, std::string_view{}, std::string_view{});
};

TEST(Args, RequiredOptionalArgsArePositional) {
    // `plan` and `behavior` are Rust Option<T> with `required: true`.
    static_assert(!CallsWithPlanOnly<std::string>);
    static_assert(CallsWithPlanAndBehavior<std::string>);
    // Over the raw wire, omitting one is MISSING_FIELD (what the typed
    // signature prevents).
    const auto e = Throws([] { (void)sz::Invoke("delete_generic_threshold", Fixture(), R"({"plan":"INGEST"})"); });
    EXPECT_EQ(e.Kind(), ErrorKind::MissingField);
}

std::string FragmentField(const std::string& cfg, const char* field) {
    const std::string text = sz::GetFragment(cfg, "SNAME_SSTAB");
    return Raw(text, *ParseText(text).Find(field));
}

TEST(Args, FieldUpdateLeaveClearSet) {
    using U = sz::FieldUpdate<std::string>;
    const std::string original = FragmentField(Fixture(), "source");
    const std::string set = sz::SetFragment(Fixture(), "SNAME_SSTAB", {.description = "new desc"});
    EXPECT_EQ(FragmentField(set, "source"), original);  // Leave keeps it
    const std::string cleared = sz::SetFragment(set, "SNAME_SSTAB", {.source = U::Clear()});
    EXPECT_EQ(FragmentField(cleared, "source"), "null");
    const std::string reset =
        sz::SetFragment(cleared, "SNAME_SSTAB", {.source = U::Set(json::Parse(original).text)});
    EXPECT_EQ(FragmentField(reset, "source"), original);
}

// ---- returns ----

TEST(Returns, ConfigAndJsonTupleNamesBecomeFields) {
    // Named fields, no inherited ConfigAndJson / `json` member.
    static_assert(!std::is_base_of_v<sz::ConfigAndJson, sz::SetGenericPlanResult>);
    const auto [config, plan_id, was_created] = sz::SetGenericPlan(Fixture(), "new_plan", "New Plan");
    EXPECT_EQ(plan_id, "3");  // JSON text of each record member
    EXPECT_EQ(was_created, "true");
    EXPECT_NE(sz::ListGenericPlans(config).find("NEW_PLAN"), std::string::npos);
    const sz::SetGenericPlanResult upd = sz::SetGenericPlan(Fixture(), "search", "Updated");
    EXPECT_EQ(upd.plan_id, "2");
    EXPECT_EQ(upd.was_created, "false");
}

TEST(Returns, JsonTupleNamesBecomeFields) {
    const sz::VerifyCompatibilityVersionResult match = sz::VerifyCompatibilityVersion(Fixture(), "11");
    EXPECT_EQ(match.current_version, R"("11")");  // a JSON string keeps its quotes
    EXPECT_EQ(match.matches, "true");
    const auto [current, matches] = sz::VerifyCompatibilityVersion(Fixture(), "11.0");
    EXPECT_EQ(current, R"("11")");
    EXPECT_EQ(matches, "false");
}

// ---- int_or_str call selectors ----

template <class A>
concept SelectsCall = requires(const std::string& c, A a) { sz::GetComparisonCall(c, a); };

TEST(IntOrStr, OverloadsAcceptIntegersAndStringsUnambiguously) {
    static_assert(SelectsCall<int>);
    static_assert(SelectsCall<long long>);
    static_assert(SelectsCall<std::int64_t>);
    static_assert(SelectsCall<const char*>);
    static_assert(SelectsCall<std::string>);
    static_assert(SelectsCall<std::string_view>);
    static_assert(!SelectsCall<double*>);
    // A literal 0 is an int (not a null const char*), and picks the id overload.
    EXPECT_EQ(Throws([] { (void)sz::GetComparisonCall(Fixture(), 0); }).Kind(), ErrorKind::NotFound);
}

TEST(IntOrStr, ByIdAndByFeatureCodeSelectTheSameCall) {
    const std::string by_id = sz::GetComparisonCall(Fixture(), 34);
    EXPECT_EQ(ParseText(by_id).Find("CFCALL_ID")->text, "34");
    EXPECT_EQ(sz::GetComparisonCall(Fixture(), "tax_id"), by_id);
    EXPECT_EQ(sz::GetComparisonCall(Fixture(), std::string("TAX_ID")), by_id);
    EXPECT_EQ(sz::GetComparisonCall(Fixture(), std::int64_t{34}), by_id);
    EXPECT_EQ(Throws([] { (void)sz::GetComparisonCall(Fixture(), "NO_SUCH_FEATURE"); }).Kind(),
              ErrorKind::NotFound);
}

TEST(IntOrStr, DeleteElementByFeatureAndById) {
    const std::string by_feature = sz::DeleteDistinctCallElement(Fixture(), "login_id", "login_domain");
    const std::string by_id = sz::DeleteDistinctCallElement(Fixture(), 13, std::string("LOGIN_DOMAIN"));
    EXPECT_EQ(by_feature, by_id);
    EXPECT_NE(by_id, Fixture());
}

TEST(Returns, UnitAndInvoke) {
    sz::ValidateConfig(Fixture());
    const sz::InvokeResult r = sz::Invoke("list_data_sources", Fixture());
    EXPECT_EQ(r.kind, "json");
    EXPECT_FALSE(r.config.has_value());
    EXPECT_EQ(ParseText(*r.result).type, JType::Array);
    EXPECT_EQ(*r.result, sz::ListDataSources(Fixture()));
}

TEST(Library, VersionAndAbi) {
    EXPECT_EQ(sz::LibraryVersion(), SZCONFIGTOOL_TEST_PROJECT_VERSION);
    EXPECT_TRUE(sz::AbiCompatible());
    EXPECT_EQ(sz::AbiVersion(), 2);
    EXPECT_EQ(sz::AbiVersion(), SZCONFIGTOOL_ABI_VERSION);
}

// ---- error mapping ----

TEST(Errors, KindHasTheReasonCodesIdentity) {
    for (const auto& [code, kind] : sz::kReasonCodes) {
        const SzConfigToolException e{std::string(code), "m"};
        EXPECT_EQ(e.ReasonCode(), code);
        EXPECT_EQ(e.Kind(), kind);
        EXPECT_EQ(e.Kind(), sz::ErrorKindFromReasonCode(e.ReasonCode()));
    }
}

TEST(Errors, LibraryReasonCodesMapToKinds) {
    EXPECT_EQ(Throws([] { (void)sz::GetDataSource(Fixture(), "NOPE"); }).Kind(), ErrorKind::NotFound);
    EXPECT_EQ(Throws([] { (void)sz::AddDataSource(Fixture(), "TEST"); }).Kind(),
              ErrorKind::AlreadyExists);
    const auto parse = Throws([] { (void)sz::ListDataSources("not json"); });
    EXPECT_EQ(parse.Kind(), ErrorKind::JsonParse);
    EXPECT_EQ(parse.ReasonCode(), "JSON_PARSE");
    EXPECT_EQ(parse.ReturnCode(), -2);
    EXPECT_FALSE(parse.Details().has_value());
}

TEST(Errors, WireErrorsAreInvalidInput) {
    EXPECT_EQ(Throws([] { (void)sz::Invoke("no_such_fn", Fixture()); }).Kind(), ErrorKind::InvalidInput);
    EXPECT_EQ(Throws([] { (void)sz::Invoke("list_data_sources", Fixture(), "[]"); }).Kind(),
              ErrorKind::InvalidInput);
    EXPECT_EQ(Throws([] { (void)sz::Invoke("get_data_source", Fixture(), R"({"code":1})"); }).Kind(),
              ErrorKind::InvalidInput);
}

TEST(Errors, NotImplementedStaysReachableThroughInvoke) {
    for (const auto& f : Manifest().Find("functions")->items) {
        if (f.Find("status")->text != "not_implemented") {
            continue;
        }
        const std::string& name = f.Find("name")->text;
        EXPECT_FALSE(TypedFunctions().contains(name)) << name;
        std::string args = "{";
        for (const auto& a : f.Find("args")->items) {
            if (!a.Find("optional")->boolean && !a.Find("tristate")->boolean) {
                const bool is_int = a.Find("type")->text == "int";
                args += (args.size() > 1 ? "," : "") + std::string("\"") + a.Find("name")->text +
                        "\":" + (is_int ? "1" : "\"X\"");
            }
        }
        args += "}";
        EXPECT_EQ(Throws([&] { (void)sz::Invoke(name, Fixture(), args); }).Kind(),
                  ErrorKind::NotImplemented)
            << name << " " << args;
    }
}

TEST(Errors, ValidationErrorsCarryDetails) {
    const auto e = Throws([] {
        (void)sz::AddSearchProfile(Fixture(), "P2", "SEARCH", {.candidates = "Sometimes"});
    });
    ASSERT_EQ(e.Kind(), ErrorKind::ValidationErrors);
    ASSERT_TRUE(e.Details().has_value());
    const json::Value d = json::Parse(*e.Details());
    EXPECT_EQ(d.Find("schema")->text, "sz-configtool.validation-errors/v1");
    EXPECT_FALSE(d.Find("failures")->items.empty());
}

TEST(Errors, EmbeddedNulIsRejectedBeforeTheCall) {
    const std::string cfg = Fixture() + std::string(1, '\0');
    const auto e = Throws([&] { (void)sz::ListDataSources(cfg); });
    EXPECT_EQ(e.Kind(), ErrorKind::InvalidInput);
    EXPECT_EQ(e.ReturnCode(), -1);
}

TEST(Errors, NonUtf8IsInvalidInputFromTheBoundary) {
    const auto e = Throws([] { (void)sz::GetDataSource(Fixture(), "\xff\xfe"); });
    EXPECT_EQ(e.Kind(), ErrorKind::InvalidInput);
    EXPECT_EQ(e.ReturnCode(), -1);
}

// ---- config opacity ----

/// A config whose bytes exercise every envelope escape path.
std::string ExoticConfig() {
    return R"({"G2_CONFIG": {"CFG_DSRC": [{"DSRC_ID": 1, "DSRC_CODE": "Q\"\\\n\t\u0001 é 😀 \/"}],)"
           R"( "CFG_DFUNC": []}})";
}

TEST(Opacity, ConfigBytesEqualTheLegacyCExport) {
    for (const std::string& input : {Fixture(), ExoticConfig()}) {
        const sz::detail::OwnedCString ref_holder = [&] {
            const SzConfigTool_result r = SzConfigTool_addDataSource(input.c_str(), "CRM");
            EXPECT_EQ(r.returnCode, 0);
            return sz::detail::OwnedCString(r.response);
        }();
        ASSERT_NE(ref_holder, nullptr);
        EXPECT_EQ(sz::AddDataSource(input, "CRM"), std::string(ref_holder.get()));
    }
}

TEST(Opacity, ReadsDoNotTouchTheConfig) {
    const std::string cfg = ExoticConfig();
    const std::string copy = cfg;
    const std::string text = sz::ListDataSources(cfg);
    const json::Value list = ParseText(text);
    EXPECT_EQ(cfg, copy);
    ASSERT_EQ(list.items.size(), 1U) << text;
    const json::Value* code = list.items[0].Find("dataSource");
    ASSERT_NE(code, nullptr) << text;
    // The decoded code survives the envelope exactly.
    EXPECT_EQ(code->text, "Q\"\\\n\t\x01 \xC3\xA9 \xF0\x9F\x98\x80 /");
}

}  // namespace
}  // namespace szconfigtool_test
