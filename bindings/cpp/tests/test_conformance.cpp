// Runs api/manifest/generated/conformance.json against the REAL
// libSzConfigTool through the TYPED C++ functions (generated dispatch table);
// `wire_only` steps and `not_implemented` functions (absent from the typed
// API by contract) go through szconfigtool::Invoke.
#include <gtest/gtest.h>

#include <cctype>
#include <iostream>
#include <map>
#include <set>
#include <string>

#include "conformance_support.hpp"
#include "generated/typed_dispatch.hpp"

namespace szconfigtool_test {
namespace {

struct Document {
    std::string text;
    json::Value root;
};

const Document& Load(std::string_view rel) {
    // One document per distinct path; both inputs are loaded once per process.
    static std::map<std::string, Document, std::less<>> cache;
    auto it = cache.find(rel);
    if (it == cache.end()) {
        Document d{ReadFile(WorkspacePath(rel)), {}};
        d.root = json::Parse(d.text);
        it = cache.emplace(std::string(rel), std::move(d)).first;
    }
    return it->second;
}

const Document& Conformance() { return Load(kConformanceJson); }

const std::vector<json::Value>& Cases() { return Conformance().root.Find("cases")->items; }

const std::string& Fixture() {
    static const std::string text =
        ReadFile(WorkspacePath(Conformance().root.Find("fixture")->text));
    return text;
}

std::set<std::string> NotImplemented() {
    std::set<std::string> out;
    for (const auto& f : Load(kManifestJson).root.Find("functions")->items) {
        if (f.Find("status")->text == "not_implemented") {
            out.insert(f.Find("name")->text);
        }
    }
    return out;
}

bool WireOnly(const json::Value& step) {
    const json::Value* w = step.Find("wire_only");
    return w != nullptr && w->boolean;
}

bool RunsTyped(const json::Value& step) {
    return !WireOnly(step) && TypedFunctions().contains(step.Find("fn")->text);
}

Outcome Execute(const json::Value& step, const std::string& config) {
    const std::string& name = step.Find("fn")->text;
    const json::Value& args = *step.Find("args");
    const std::string_view src = Conformance().text;
    if (RunsTyped(step)) {
        return TypedFunctions().at(name)(config, TestArgs(args, src));
    }
    szconfigtool::InvokeResult r = szconfigtool::Invoke(name, config, Raw(src, args));
    return Outcome{r.kind, r.config, r.result};
}

void CheckSuccess(const json::Value& expect, const Outcome& out, const std::string& where) {
    if (const auto* k = expect.Find("kind")) {
        EXPECT_EQ(out.kind, k->text) << where;
    }
    const json::Value* result = expect.Find("result");
    if (result == nullptr && expect.Find("contains") == nullptr && expect.Find("excludes") == nullptr &&
        expect.Find("len") == nullptr) {
        return;
    }
    ASSERT_TRUE(out.result.has_value()) << where << ": no result";
    const json::Value actual = json::Parse(*out.result);
    if (result != nullptr) {
        EXPECT_TRUE(SubsetMatch(*result, actual)) << where << ": result " << *out.result;
    }
    for (const auto& problem : ArrayCheckProblems(expect, actual)) {
        ADD_FAILURE() << where << ": " << problem << " (result " << *out.result << ")";
    }
}

TEST(ConformanceMatcher, ArrayChecksFailOnNonArrayResults) {
    // An object/int result must never satisfy len/contains/excludes vacuously.
    for (const char* expect : {R"({"len":0})", R"({"excludes":[{"a":1}]})", R"({"contains":[{"a":1}]})"}) {
        const json::Value e = json::Parse(expect);
        EXPECT_FALSE(ArrayCheckProblems(e, json::Parse(R"({"a":1})")).empty()) << expect;
        EXPECT_FALSE(ArrayCheckProblems(e, json::Parse("3")).empty()) << expect;
    }
    EXPECT_TRUE(ArrayCheckProblems(json::Parse(R"({"len":1,"excludes":[{"a":2}]})"),
                                   json::Parse(R"([{"a":1}])"))
                    .empty());
}

class ConformanceCase : public ::testing::TestWithParam<std::size_t> {};

TEST_P(ConformanceCase, Case) {
    const json::Value& c = Cases().at(GetParam());
    std::string config = Fixture();
    std::size_t index = 0;
    for (const auto& step : c.Find("steps")->items) {
        const std::string where = c.Find("name")->text + " step " + std::to_string(index++) +
                                  " (" + step.Find("fn")->text + ")";
        const json::Value* literal = step.Find("config_literal");
        const std::string input = literal != nullptr ? literal->text : config;
        const json::Value& expect = *step.Find("expect");
        if (const json::Value* err = expect.Find("error")) {
            try {
                (void)Execute(step, input);
                ADD_FAILURE() << where << ": expected " << err->text;
            } catch (const szconfigtool::SzConfigToolException& e) {
                EXPECT_EQ(e.ReasonCode(), err->text) << where << ": " << e.what();
                EXPECT_EQ(e.Kind(), szconfigtool::ErrorKindFromReasonCode(err->text)) << where;
                EXPECT_NE(e.Kind(), szconfigtool::ErrorKind::Unknown) << where;
            }
            continue;
        }
        Outcome out;
        try {
            out = Execute(step, input);
        } catch (const szconfigtool::SzConfigToolException& e) {
            FAIL() << where << ": unexpected " << e.ReasonCode() << ": " << e.what();
        }
        CheckSuccess(expect, out, where);
        if (out.config) {
            config = *out.config;
        }
    }
}

std::string CaseName(const ::testing::TestParamInfo<std::size_t>& info) {
    const json::Value& c = Cases().at(info.param);
    std::string name = c.Find("group")->text + "_" + c.Find("name")->text;
    for (char& ch : name) {
        if (std::isalnum(static_cast<unsigned char>(ch)) == 0) {
            ch = '_';
        }
    }
    return name;
}

INSTANTIATE_TEST_SUITE_P(All, ConformanceCase, ::testing::Range<std::size_t>(0, Cases().size()),
                         CaseName);

// Every step that a typed binding CAN express runs typed: the only raw-invoke
// steps are wire_only ones and calls to not_implemented functions.
TEST(ConformanceCoverage, StepsRouteThroughTypedApi) {
    const std::set<std::string> stubs = NotImplemented();
    std::size_t typed = 0;
    std::size_t wire = 0;
    for (const auto& c : Cases()) {
        for (const auto& step : c.Find("steps")->items) {
            const std::string& name = step.Find("fn")->text;
            if (RunsTyped(step)) {
                ++typed;
                continue;
            }
            ++wire;
            EXPECT_TRUE(WireOnly(step) || stubs.contains(name)) << name << " has no typed function";
        }
    }
    EXPECT_GT(typed, wire);
    std::cout << "conformance steps: typed=" << typed << " invoke=" << wire << "\n";
}

}  // namespace
}  // namespace szconfigtool_test
