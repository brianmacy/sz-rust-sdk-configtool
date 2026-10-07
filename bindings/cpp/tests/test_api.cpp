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
    EXPECT_EQ(Manifest().Find("functions")->items.size(), typed.size());  // no not_implemented functions
}

TEST(Naming, PascalCaseFunctionsSnakeCaseArgs) {
    // Compile-time evidence: PascalCase names, snake_case option fields, and
    // the C++-keyword arg `class` exposed as `class_`.
    const sz::AddAttributeOptions opts{.default_value = "d", .internal = "No", .id = 0};
    const std::string row = sz::AddAttributeResult(Fixture(), "MY_ATTR", "NAME", "FULL_NAME",
                                                   /*class_=*/"OTHER", opts);
    EXPECT_EQ(ParseText(row).Find("ATTR_CODE")->text, "MY_ATTR");
    EXPECT_EQ(ParseText(row).Find("DEFAULT_VALUE")->text, "d");
}

// ---- every config-changing function returns the config text (issue #75) ----

TEST(Chaining, ConfigAndJsonFunctionsHaveCompanionsInTheDispatch) {
    // The generated dispatch routes a config_and_json step through the primary
    // (statically required to return std::string) AND its <Name>Result
    // companion; TypedCompanions() lists the wire names it does that for.
    std::set<std::string> paired;
    for (const auto& f : Manifest().Find("functions")->items) {
        if (f.Find("status")->text == "implemented" && f.Find("returns")->text == "config_and_json") {
            paired.insert(f.Find("name")->text);
        }
    }
    EXPECT_FALSE(paired.empty());
    EXPECT_EQ(TypedCompanions(), paired);
}

TEST(Chaining, SevenDifferentConfigChangingFunctionsChain) {
    static_assert(std::is_same_v<decltype(sz::AddAttribute(std::string{}, "", "", "", "")), std::string>);
    static_assert(std::is_same_v<decltype(sz::AddAttributeResult(std::string{}, "", "", "", "")), std::string>);
    std::string cfg = Fixture();
    cfg = sz::AddElement(cfg, "DEMO_EL", {.data_type = "string"});
    cfg = sz::AddFeature(cfg, "DEMO_FEAT", R"(["DEMO_EL"])");
    cfg = sz::AddAttribute(cfg, "DEMO_ATTR", "DEMO_FEAT", "DEMO_EL", "OTHER");
    cfg = sz::AddFragment(cfg, R"({"ERFRAG_CODE":"DEMO_FRAG","ERFRAG_SOURCE":"./FRAGMENT[./SAME_NAME>0]"})");
    cfg = sz::AddComparisonCall(cfg, "DEMO_FEAT", "EXACT_COMP", {"DEMO_EL"});
    cfg = sz::AddComparisonFunction(cfg, "DEMO_COMP");
    cfg = sz::AddDataSource(cfg, "DEMO_DS");
    EXPECT_EQ(ParseText(sz::GetAttribute(cfg, "DEMO_ATTR")).Find("ATTR_CODE")->text, "DEMO_ATTR");
    EXPECT_NE(sz::GetFragment(cfg, "DEMO_FRAG").find("DEMO_FRAG"), std::string::npos);
    EXPECT_NE(sz::GetComparisonFunction(cfg, "DEMO_COMP").find("DEMO_COMP"), std::string::npos);
    EXPECT_NE(sz::ListDataSources(cfg).find("DEMO_DS"), std::string::npos);
}

TEST(Chaining, CompanionReturnsTheRowOfTheSameOperation) {
    const std::string row = sz::AddAttributeResult(Fixture(), "X_ATTR", "NAME", "FULL_NAME", "OTHER");
    const sz::InvokeResult wire = sz::Invoke("add_attribute", Fixture(),
        R"({"attribute":"X_ATTR","feature":"NAME","element":"FULL_NAME","class":"OTHER"})");
    EXPECT_EQ(wire.result, std::optional<std::string>(row));
    EXPECT_EQ(wire.config, std::optional<std::string>(
        sz::AddAttribute(Fixture(), "X_ATTR", "NAME", "FULL_NAME", "OTHER")));
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

TEST(Returns, ConfigAndJsonTupleNamesBecomeCompanionFields) {
    // The primary returns the config; the companion the named fields only.
    static_assert(std::is_same_v<decltype(sz::SetGenericPlan(std::string{}, "", "")), std::string>);
    const auto [plan_id, was_created] = sz::SetGenericPlanResult(Fixture(), "new_plan", "New Plan");
    EXPECT_EQ(plan_id, "3");  // JSON text of each record member
    EXPECT_EQ(was_created, "true");
    const std::string config = sz::SetGenericPlan(Fixture(), "new_plan", "New Plan");
    EXPECT_NE(sz::ListGenericPlans(config).find("NEW_PLAN"), std::string::npos);
    const sz::SetGenericPlanRecord upd = sz::SetGenericPlanResult(Fixture(), "search", "Updated");
    EXPECT_EQ(upd.plan_id, "2");
    EXPECT_EQ(upd.was_created, "false");
}

TEST(Returns, JsonTupleNamesBecomeFields) {
    const sz::VerifyCompatibilityVersionRecord match = sz::VerifyCompatibilityVersion(Fixture(), "11");
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

TEST(IntOrStr, DeleteElementByFeatureSelectorWithElementFeature) {
    const std::string without = sz::DeleteExpressionCallElement(Fixture(), "PHONE", "PHONE_LAST_10");
    const std::string with = sz::DeleteExpressionCallElement(Fixture(), "phone", "phone_last_10",
                                                            {.element_feature = "PHONE"});
    EXPECT_EQ(with, without);
    EXPECT_NE(with, Fixture());
}

// The one CFG_CFRTN row whose rendering contains `rtnval`.
json::Value ThresholdRow(const std::string& cfg, const std::string& rtnval) {
    const json::Value rows = ParseText(sz::GetConfigSection(cfg, "CFG_CFRTN", {.filter = rtnval}));
    EXPECT_EQ(rows.items.size(), 1U);
    return rows.items.at(0);
}

void ExpectScores(const json::Value& row, const char* exec, const char* same, const char* close,
                  const char* likely, const char* plausible, const char* unlikely) {
    EXPECT_EQ(row.Find("EXEC_ORDER")->text, exec);
    EXPECT_EQ(row.Find("SAME_SCORE")->text, same);
    EXPECT_EQ(row.Find("CLOSE_SCORE")->text, close);
    EXPECT_EQ(row.Find("LIKELY_SCORE")->text, likely);
    EXPECT_EQ(row.Find("PLAUSIBLE_SCORE")->text, plausible);
    EXPECT_EQ(row.Find("UN_LIKELY_SCORE")->text, unlikely);
}

TEST(Args, ComparisonThresholdEveryScoreOption) {
    const std::string added = sz::AddComparisonThreshold(
        Fixture(), "GNR_COMP", "all", "cpp_rtn",
        {.exec_order = 20, .same_score = 90, .close_score = 80, .likely_score = 70,
         .plausible_score = 60, .un_likely_score = 50});
    ExpectScores(ThresholdRow(added, "CPP_RTN"), "20", "90", "80", "70", "60", "50");

    const std::string set = sz::SetComparisonThreshold(
        added, "gnr_comp", "ALL", "cpp_rtn",
        {.exec_order = 21, .same_score = 91, .close_score = 81, .likely_score = 71,
         .plausible_score = 61, .un_likely_score = -1});
    ExpectScores(ThresholdRow(set, "CPP_RTN"), "21", "91", "81", "71", "61", "-1");
}

TEST(Returns, UnitAndInvoke) {
    sz::ValidateConfig(Fixture());
    const sz::InvokeResult r = sz::Invoke("list_data_sources", Fixture());
    EXPECT_EQ(r.kind, "json");
    EXPECT_FALSE(r.config.has_value());
    EXPECT_EQ(ParseText(*r.result).type, JType::Array);
    EXPECT_EQ(*r.result, sz::ListDataSources(Fixture()));
}

TEST(Returns, EnvelopeKindMismatchIsInternal) {
    // A real `json` function read through each other typed result kind.
    for (const auto kind : {sz::ResultKind::Config, sz::ResultKind::ConfigAndJson,
                            sz::ResultKind::Int, sz::ResultKind::Unit}) {
        const SzConfigToolException e = Throws([kind] {
            (void)sz::detail::Call("list_data_sources", Fixture(), "{}", kind);
        });
        EXPECT_EQ(e.Kind(), ErrorKind::Internal);
        EXPECT_STREQ(e.what(), "list_data_sources: unexpected envelope kind 'json'");
    }
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
