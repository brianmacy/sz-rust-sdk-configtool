// Unit tests of the hand-written runtime (core.hpp): JSON reader/writer,
// FieldUpdate, the exception and the generated ErrorKind table.
#include <gtest/gtest.h>

#include <set>
#include <string>

#include "conformance_support.hpp"
#include "generated/typed_dispatch.hpp"

namespace szconfigtool_test {
namespace {

using szconfigtool::ErrorKind;
using szconfigtool::FieldUpdate;
using szconfigtool::SzConfigToolException;

TEST(Json, DecodesEveryEscape) {
    const json::Value v = json::Parse(R"("q\" b\\ s\/ \b\f\n\r\t \u00e9 \ud83d\ude00 \u0001")");
    EXPECT_EQ(v.text, "q\" b\\ s/ \b\f\n\r\t \xC3\xA9 \xF0\x9F\x98\x80 \x01");
}

TEST(Json, KeepsRawSpans) {
    const std::string src = R"({"a": [1, {"b": null}], "c": "x"})";
    const json::Value v = json::Parse(src);
    EXPECT_EQ(Raw(src, *v.Find("a")), R"([1, {"b": null}])");
    EXPECT_EQ(Raw(src, *v.Find("c")), R"("x")");
}

TEST(Json, RejectsMalformedInput) {
    for (const char* bad : {"", "{", "[1,]", "\"abc", "tru", "{\"a\" 1}", "1 2", "\"\\ud800\"",
                            "\"\\x\""}) {
        try {
            (void)json::Parse(bad);
            ADD_FAILURE() << "accepted: " << bad;
        } catch (const SzConfigToolException& e) {
            EXPECT_EQ(e.Kind(), ErrorKind::Internal) << bad;
        }
    }
}

TEST(Json, ReportsEachMalformation) {
    const std::pair<const char*, const char*> cases[] = {
        {"{1:2}", "invalid JSON at byte 1: expected key"},
        {R"({"a":1,2})", "invalid JSON at byte 7: expected key"},
        // (Plain literals: the compiler may expand \u inside raw strings.)
        {"\"\\u12\"", "invalid JSON at byte 3: short \\u escape"},
        {"\"\\uZZZZ\"", "invalid JSON at byte 3: bad \\u escape"},
        {"\"\\u12G4\"", "invalid JSON at byte 3: bad \\u escape"},
        {"\"\\udc00\\udc00\"", "invalid JSON at byte 7: lone surrogate"},
        {"\"\\ud800x\"", "invalid JSON at byte 7: lone surrogate"},
        {"\"\\ud800\\u0041\"", "invalid JSON at byte 13: bad low surrogate"},
        {"\"\\ud800\\ue000\"", "invalid JSON at byte 13: bad low surrogate"},
        {"\"\\", "invalid JSON at byte 2: unterminated escape"},
    };
    for (const auto& [bad, message] : cases) {
        try {
            (void)json::Parse(bad);
            ADD_FAILURE() << "accepted: " << bad;
        } catch (const SzConfigToolException& e) {
            EXPECT_EQ(e.ReasonCode(), "INTERNAL") << bad;
            EXPECT_STREQ(e.what(), message) << bad;
        }
    }
}

TEST(Json, DecodesEveryUtf8LengthAndWhitespace) {
    EXPECT_EQ(json::Parse("\"\\u0041\\u00e9\\u20ac\\uffff\\ud83d\\ude00\"").text,
              "A\xC3\xA9\xE2\x82\xAC\xEF\xBF\xBF\xF0\x9F\x98\x80");
    const json::Value v = json::Parse(" \t\n\r[\r1 ,\tfalse\n]\r\n");
    ASSERT_EQ(v.items.size(), 2U);
    EXPECT_EQ(v.items[0].text, "1");
    EXPECT_EQ(v.items[1].type, json::Value::Type::Bool);
    EXPECT_FALSE(v.items[1].boolean);
}

TEST(Json, RejectsExcessiveNesting) {
    const std::string deep(2000, '[');
    EXPECT_THROW((void)json::Parse(deep), SzConfigToolException);
}

TEST(Json, QuotedRoundTripsThroughParser) {
    std::string raw = "a\"b\\c\n\t\x01\x1f\x7f\xC3\xA9";
    raw.push_back('\0');
    std::string quoted;
    json::AppendQuoted(quoted, raw);
    EXPECT_EQ(json::Parse(quoted).text, raw);
}

TEST(ArgsWriter, WritesEveryType) {
    szconfigtool::detail::ArgsWriter w;
    w.Str("s", "a\"b");
    w.Int("i", -42);
    w.Bool("b", true);
    w.Json("j", R"({"k":[1]})");
    w.Null("n");
    w.StrList("l", {"x", "y"});
    EXPECT_EQ(w.Finish(), R"({"s":"a\"b","i":-42,"b":true,"j":{"k":[1]},"n":null,"l":["x","y"]})");
    EXPECT_EQ(szconfigtool::detail::ArgsWriter{}.Finish(), "{}");
}

TEST(ArgsWriter, WritesFalseCarriageReturnAndIntOrStr) {
    szconfigtool::detail::ArgsWriter w;
    w.Bool("f", false);
    w.Str("r", "a\rb");
    w.IntOrStr("id", std::int64_t{-7});
    w.IntOrStr("code", std::string("PHONE"));
    EXPECT_EQ(w.Finish(), R"({"f":false,"r":"a\rb","id":-7,"code":"PHONE"})");
}

TEST(Runtime, WireNameOfEveryResultKind) {
    using szconfigtool::ResultKind;
    using szconfigtool::detail::WireName;
    EXPECT_EQ(WireName(ResultKind::Config), "config");
    EXPECT_EQ(WireName(ResultKind::Json), "json");
    EXPECT_EQ(WireName(ResultKind::ConfigAndJson), "config_and_json");
    EXPECT_EQ(WireName(ResultKind::Int), "int");
    EXPECT_EQ(WireName(ResultKind::Unit), "unit");
    // A value outside the enumerators (valid for a scoped enum) has no name.
    EXPECT_EQ(WireName(static_cast<ResultKind>(99)), "");
}

TEST(Runtime, ParseIntAcceptsOnlyAWholeInteger) {
    using szconfigtool::detail::ParseInt;
    EXPECT_EQ(ParseInt("42"), 42);
    EXPECT_EQ(ParseInt("-9223372036854775808"), INT64_MIN);
    for (const char* bad : {"", "x", "1.5", "99999999999999999999"}) {
        try {
            (void)ParseInt(bad);
            ADD_FAILURE() << "accepted: " << bad;
        } catch (const SzConfigToolException& e) {
            EXPECT_EQ(e.ReasonCode(), "INTERNAL") << bad;
            EXPECT_EQ(std::string(e.what()), std::string("expected an integer result, got '") + bad + "'");
        }
    }
}

TEST(Runtime, RecordMemberRequiresAnObjectWithTheField) {
    const szconfigtool::detail::Record rec(R"({"ID": 1001, "V": "4.0.0"})");
    EXPECT_EQ(rec.Member("ID"), "1001");
    EXPECT_EQ(rec.Member("V"), R"("4.0.0")");
    for (const char* text : {R"({"ID": 1})", "[1]"}) {
        const szconfigtool::detail::Record r(text);
        try {
            (void)r.Member("MISSING");
            ADD_FAILURE() << "found MISSING in " << text;
        } catch (const SzConfigToolException& e) {
            EXPECT_EQ(e.ReasonCode(), "INTERNAL");
            EXPECT_STREQ(e.what(), "record lacks field 'MISSING'");
        }
    }
}

TEST(FieldUpdate, DefaultIsLeave) {
    const FieldUpdate<std::string> u;
    EXPECT_TRUE(u.IsLeave());
    EXPECT_FALSE(u.IsSet());
    EXPECT_FALSE(u.IsClear());
    EXPECT_THROW((void)u.Value(), std::bad_optional_access);
}

TEST(FieldUpdate, FactoriesAndImplicitSet) {
    EXPECT_TRUE(FieldUpdate<std::int64_t>::Clear().IsClear());
    EXPECT_TRUE(FieldUpdate<std::int64_t>::Leave().IsLeave());
    const auto s = FieldUpdate<std::int64_t>::Set(7);
    EXPECT_TRUE(s.IsSet());
    EXPECT_EQ(s.Value(), 7);
    const FieldUpdate<std::string> implicit = "text";
    EXPECT_TRUE(implicit.IsSet());
    EXPECT_EQ(implicit.Value(), "text");
    static_assert(!std::is_convertible_v<std::nullptr_t, FieldUpdate<std::string>>);
}

TEST(ErrorKinds, EveryManifestReasonCodeMaps) {
    const std::string text = ReadFile(WorkspacePath(kManifestJson));
    const json::Value manifest = json::Parse(text);
    const auto& codes = manifest.Find("reason_codes")->items;
    ASSERT_EQ(codes.size(), szconfigtool::kReasonCodes.size());
    std::set<ErrorKind> kinds;
    for (const auto& c : codes) {
        const ErrorKind k = szconfigtool::ErrorKindFromReasonCode(c.text);
        EXPECT_NE(k, ErrorKind::Unknown) << c.text;
        kinds.insert(k);
    }
    EXPECT_EQ(kinds.size(), codes.size());
    EXPECT_EQ(szconfigtool::ErrorKindFromReasonCode("NO_SUCH_CODE"), ErrorKind::Unknown);
}

TEST(Exception, Accessors) {
    const SzConfigToolException e("VALIDATION_ERRORS", "bad", std::string("{}"), -2);
    const std::runtime_error& base = e;
    EXPECT_STREQ(base.what(), "bad");
    EXPECT_EQ(e.Kind(), ErrorKind::ValidationErrors);
    EXPECT_EQ(e.ReasonCode(), "VALIDATION_ERRORS");
    EXPECT_EQ(e.Details(), std::optional<std::string>("{}"));
    EXPECT_EQ(e.ReturnCode(), -2);
}

}  // namespace
}  // namespace szconfigtool_test
