// conformance_support.hpp -- test-side helpers shared by the generated typed
// dispatch table (tests/generated/typed_dispatch.hpp) and the test suites.
#ifndef SZCONFIGTOOL_TESTS_CONFORMANCE_SUPPORT_HPP
#define SZCONFIGTOOL_TESTS_CONFORMANCE_SUPPORT_HPP

#include <charconv>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <functional>
#include <initializer_list>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <variant>
#include <vector>

#include "szconfigtool/szconfigtool.hpp"

namespace szconfigtool_test {

namespace json = szconfigtool::detail::json;
using JType = json::Value::Type;

/// Absolute path of a workspace-relative path (root from CMake).
inline std::filesystem::path WorkspacePath(std::string_view rel) {
    return std::filesystem::path(SZCONFIGTOOL_TEST_WORKSPACE_ROOT) / rel;
}

inline std::string ReadFile(const std::filesystem::path& path) {
    std::ifstream in(path, std::ios::binary);
    if (!in) {
        throw std::runtime_error("cannot read " + path.string());
    }
    std::ostringstream ss;
    ss << in.rdbuf();
    return ss.str();
}

/// Raw JSON text of a parsed value.
inline std::string Raw(std::string_view source, const json::Value& v) {
    return std::string(source.substr(v.begin, v.end - v.begin));
}

/// Conformance `args` object, decoded for the typed call.
class TestArgs {
public:
    TestArgs(const json::Value& obj, std::string_view source) : obj_(obj), source_(source) {}

    [[nodiscard]] bool Has(std::string_view key) const { return obj_.Find(key) != nullptr; }
    [[nodiscard]] bool IsNull(std::string_view key) const {
        return Get(key).type == JType::Null;
    }
    [[nodiscard]] std::string Str(std::string_view key) const {
        return Typed(key, JType::String).text;
    }
    [[nodiscard]] std::int64_t Int(std::string_view key) const {
        const std::string& t = Typed(key, JType::Number).text;
        std::int64_t v = 0;
        auto [ptr, ec] = std::from_chars(t.data(), t.data() + t.size(), v);
        if (ec != std::errc() || ptr != t.data() + t.size()) {
            throw std::logic_error("arg '" + std::string(key) + "' is not an integer");
        }
        return v;
    }
    [[nodiscard]] bool Bool(std::string_view key) const { return Typed(key, JType::Bool).boolean; }
    [[nodiscard]] std::string Json(std::string_view key) const { return Raw(source_, Get(key)); }
    /// An `int_or_str` arg: its JSON type (integer / string) picks the overload.
    [[nodiscard]] std::variant<std::int64_t, std::string> IntOrStr(std::string_view key) const {
        if (Get(key).type == JType::String) {
            return Str(key);
        }
        return Int(key);
    }
    [[nodiscard]] std::vector<std::string> StrList(std::string_view key) const {
        std::vector<std::string> out;
        for (const auto& item : Typed(key, JType::Array).items) {
            if (item.type != JType::String) {
                throw std::logic_error("arg '" + std::string(key) + "' has a non-string item");
            }
            out.push_back(item.text);
        }
        return out;
    }
    /// Every conformance arg must be a typed parameter (nothing dropped).
    void CheckKnown(std::initializer_list<std::string_view> known) const {
        for (const auto& m : obj_.members) {
            bool found = false;
            for (auto k : known) {
                found = found || k == m.key;
            }
            if (!found) {
                throw std::logic_error("conformance arg '" + m.key + "' is not a typed parameter");
            }
        }
    }

private:
    const json::Value& obj_;
    std::string_view source_;

    [[nodiscard]] const json::Value& Get(std::string_view key) const {
        const json::Value* v = obj_.Find(key);
        if (v == nullptr) {
            throw std::logic_error("missing arg '" + std::string(key) + "'");
        }
        return *v;
    }
    [[nodiscard]] const json::Value& Typed(std::string_view key, JType t) const {
        const json::Value& v = Get(key);
        if (v.type != t) {
            throw std::logic_error("arg '" + std::string(key) + "' has the wrong JSON type");
        }
        return v;
    }
};

/// Outcome of one call, typed or raw, in wire terms.
struct Outcome {
    std::string kind;
    std::optional<std::string> config;
    std::optional<std::string> result;

    static Outcome FromConfig(std::string c) { return {"config", std::move(c), std::nullopt}; }
    static Outcome FromJson(std::string j) { return {"json", std::nullopt, std::move(j)}; }
    static Outcome FromConfigAndJson(szconfigtool::ConfigAndJson r) {
        return {"config_and_json", std::move(r.config), std::move(r.json)};
    }
    /// A named-field result: rebuilds the record `{"name": <json text>, ...}`.
    static Outcome FromRecord(std::string kind, std::optional<std::string> config,
                              std::initializer_list<std::pair<std::string_view, std::string_view>> fields) {
        std::string rec = "{";
        for (const auto& [name, text] : fields) {
            if (rec.size() > 1) {
                rec.push_back(',');
            }
            szconfigtool::detail::json::AppendQuoted(rec, name);
            rec.push_back(':');
            rec.append(text);
        }
        rec.push_back('}');
        return {std::move(kind), std::move(config), std::move(rec)};
    }
    static Outcome FromInt(std::int64_t v) { return {"int", std::nullopt, std::to_string(v)}; }
    static Outcome Unit() { return {"unit", std::nullopt, std::nullopt}; }
};

using TypedCall = std::function<Outcome(const std::string&, const TestArgs&)>;

// ---- JSON comparison (schema.md "Subset match") ----

inline bool NumbersEqual(const std::string& a, const std::string& b) {
    const bool ai = a.find_first_of(".eE") == std::string::npos;
    const bool bi = b.find_first_of(".eE") == std::string::npos;
    if (ai && bi) {
        return a == b;
    }
    return ai == bi && std::stod(a) == std::stod(b);
}

inline bool SubsetMatch(const json::Value& expected, const json::Value& actual) {
    if (expected.type != actual.type) {
        return false;
    }
    switch (expected.type) {
        case JType::Null: return true;
        case JType::Bool: return expected.boolean == actual.boolean;
        case JType::String: return expected.text == actual.text;
        case JType::Number: return NumbersEqual(expected.text, actual.text);
        case JType::Array:
            if (expected.items.size() != actual.items.size()) {
                return false;
            }
            for (std::size_t i = 0; i < expected.items.size(); ++i) {
                if (!SubsetMatch(expected.items[i], actual.items[i])) {
                    return false;
                }
            }
            return true;
        case JType::Object:
            for (const auto& m : expected.members) {
                const json::Value* a = actual.Find(m.key);
                if (a == nullptr || !SubsetMatch(m.value, *a)) {
                    return false;
                }
            }
            return true;
    }
    return false;
}

inline bool AnyMatches(const json::Value& expected, const json::Value& array) {
    for (const auto& item : array.items) {
        if (SubsetMatch(expected, item)) {
            return true;
        }
    }
    return false;
}

/// Problems with the `len` / `contains` / `excludes` expectations of `expect`
/// against `actual` (empty = pass).
inline std::vector<std::string> ArrayCheckProblems(const json::Value& expect, const json::Value& actual) {
    std::vector<std::string> problems;
    const json::Value* contains = expect.Find("contains");
    const json::Value* excludes = expect.Find("excludes");
    const json::Value* len = expect.Find("len");
    if (contains == nullptr && excludes == nullptr && len == nullptr) {
        return problems;
    }
    if (actual.type != JType::Array) {
        problems.emplace_back("len/contains/excludes need an array result");
        return problems;
    }
    if (contains != nullptr) {
        for (const auto& e : contains->items) {
            if (!AnyMatches(e, actual)) {
                problems.emplace_back("a contains entry matches no element");
            }
        }
    }
    if (excludes != nullptr) {
        for (const auto& e : excludes->items) {
            if (AnyMatches(e, actual)) {
                problems.emplace_back("an element matches an excludes entry");
            }
        }
    }
    if (len != nullptr && std::to_string(actual.items.size()) != len->text) {
        problems.emplace_back("len " + std::to_string(actual.items.size()) + " != " + len->text);
    }
    return problems;
}

}  // namespace szconfigtool_test

#endif  // SZCONFIGTOOL_TESTS_CONFORMANCE_SUPPORT_HPP
