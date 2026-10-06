// core.hpp -- hand-written runtime of the header-only C++20 binding of
// libSzConfigTool. The typed functions in generated/api.hpp call into this
// file; see bindings/CONTRACT.md for the wire contract.
//
// Everything goes through ONE C export, SzConfigTool_invoke(name, config,
// args_json). The configuration is OPAQUE: it is passed to the library as-is
// and recovered from the envelope by decoding a JSON string, never parsed as a
// document or re-serialized.
#ifndef SZCONFIGTOOL_CORE_HPP
#define SZCONFIGTOOL_CORE_HPP

#include <charconv>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <string_view>
#include <system_error>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include <libSzConfigTool.h>

#include "szconfigtool/generated/error_kinds.hpp"

namespace szconfigtool {

/// Error raised by every binding function.
///
/// The error KIND is the reason code: `ReasonCode()` is the stable wire reason
/// code (one of `kReasonCodes`) and `Kind()` is the SAME identity as the
/// generated `ErrorKind` enum constant (`Unknown` only for a code this header
/// does not know). `Details()` holds the versioned validation
/// details JSON (`sz-configtool.validation-errors/v1`) when the reason code is
/// `VALIDATION_ERRORS`. `ReturnCode()` is the raw C return code (-1 boundary,
/// -2 library); it is NOT the taxonomy.
class SzConfigToolException : public std::runtime_error {
public:
    SzConfigToolException(std::string reason_code, const std::string& message,
                          std::optional<std::string> details = std::nullopt,
                          std::int64_t return_code = -2)
        : std::runtime_error(message),
          reasonCode_(std::move(reason_code)),
          details_(std::move(details)),
          returnCode_(return_code) {}

    [[nodiscard]] ErrorKind Kind() const noexcept {
        return ErrorKindFromReasonCode(reasonCode_);
    }
    [[nodiscard]] const std::string& ReasonCode() const noexcept { return reasonCode_; }
    [[nodiscard]] const std::optional<std::string>& Details() const noexcept {
        return details_;
    }
    [[nodiscard]] std::int64_t ReturnCode() const noexcept { return returnCode_; }

private:
    std::string reasonCode_;
    std::optional<std::string> details_;
    std::int64_t returnCode_;
};

/// Tri-state update of a field: Leave (default; not sent), Clear (sent as
/// JSON null) or Set (sent as the value). A value converts implicitly to Set,
/// so `{.tier = 3}` works in designated initializers.
template <class T>
class FieldUpdate {
public:
    FieldUpdate() = default;

    template <class U>
        requires std::constructible_from<T, U&&> &&
                 (!std::same_as<std::remove_cvref_t<U>, FieldUpdate>) &&
                 (!std::same_as<std::remove_cvref_t<U>, std::nullptr_t>)
    FieldUpdate(U&& value)  // NOLINT(google-explicit-constructor): Set shorthand
        : state_(State::Set), value_(std::forward<U>(value)) {}

    [[nodiscard]] static FieldUpdate Leave() { return FieldUpdate{}; }
    [[nodiscard]] static FieldUpdate Clear() {
        FieldUpdate u;
        u.state_ = State::Clear;
        return u;
    }
    [[nodiscard]] static FieldUpdate Set(T value) { return FieldUpdate(std::move(value)); }

    [[nodiscard]] bool IsLeave() const noexcept { return state_ == State::Leave; }
    [[nodiscard]] bool IsClear() const noexcept { return state_ == State::Clear; }
    [[nodiscard]] bool IsSet() const noexcept { return state_ == State::Set; }
    /// The Set value; throws std::bad_optional_access unless IsSet().
    [[nodiscard]] const T& Value() const { return value_.value(); }

private:
    enum class State { Leave, Clear, Set };
    State state_ = State::Leave;
    std::optional<T> value_;
};

/// Result of a `config_and_json` function: the modified config and the
/// record as JSON text.
struct ConfigAndJson {
    std::string config;
    std::string json;
};

/// Wire result kinds (`returns` in the manifest).
enum class ResultKind { Config, Json, ConfigAndJson, Int, Unit };

/// Decoded success envelope of Invoke(): `config` is present for config
/// kinds, `result` (JSON text) for json / config_and_json / int.
struct InvokeResult {
    std::string kind;
    std::optional<std::string> config;
    std::optional<std::string> result;
};

namespace detail {

[[nodiscard]] constexpr std::string_view WireName(ResultKind k) noexcept {
    switch (k) {
        case ResultKind::Config: return "config";
        case ResultKind::Json: return "json";
        case ResultKind::ConfigAndJson: return "config_and_json";
        case ResultKind::Int: return "int";
        case ResultKind::Unit: return "unit";
    }
    return "";
}

[[noreturn]] inline void ThrowInternal(const std::string& message) {
    throw SzConfigToolException("INTERNAL", message);
}

// ---------------------------------------------------------------------------
// Minimal JSON reader (dependency-free). Values keep their byte span in the
// source so callers can return sub-documents verbatim (no re-serialization).
// ---------------------------------------------------------------------------
namespace json {

struct Member;

struct Value {
    enum class Type { Null, Bool, Number, String, Array, Object };
    Type type = Type::Null;
    bool boolean = false;
    std::string text;  ///< decoded string, or the number literal
    std::vector<Value> items;
    std::vector<Member> members;
    std::size_t begin = 0;  ///< span in the parsed source
    std::size_t end = 0;

    [[nodiscard]] const Value* Find(std::string_view key) const;
};

struct Member {
    std::string key;
    Value value;
};

inline const Value* Value::Find(std::string_view key) const {
    for (const auto& m : members) {
        if (m.key == key) {
            return &m.value;
        }
    }
    return nullptr;
}

class Parser {
public:
    explicit Parser(std::string_view src) : src_(src) {}

    Value Document() {
        Value v = ParseValue(0);
        SkipWs();
        if (pos_ != src_.size()) {
            Fail("trailing characters");
        }
        return v;
    }

private:
    static constexpr int kMaxDepth = 512;
    std::string_view src_;
    std::size_t pos_ = 0;

    [[noreturn]] void Fail(const char* what) const {
        ThrowInternal(std::string("invalid JSON at byte ") + std::to_string(pos_) + ": " + what);
    }
    void SkipWs() {
        while (pos_ < src_.size() && (src_[pos_] == ' ' || src_[pos_] == '\t' ||
                                      src_[pos_] == '\n' || src_[pos_] == '\r')) {
            ++pos_;
        }
    }
    char Peek() {
        SkipWs();
        if (pos_ >= src_.size()) {
            Fail("unexpected end");
        }
        return src_[pos_];
    }
    void Expect(char c) {
        if (Peek() != c) {
            Fail("unexpected character");
        }
        ++pos_;
    }
    void Literal(std::string_view word) {
        if (src_.substr(pos_, word.size()) != word) {
            Fail("bad literal");
        }
        pos_ += word.size();
    }

    Value ParseValue(int depth) {
        if (depth > kMaxDepth) {
            Fail("nesting too deep");
        }
        Value v;
        const char c = Peek();
        v.begin = pos_;
        switch (c) {
            case '{': ParseObject(v, depth); break;
            case '[': ParseArray(v, depth); break;
            case '"': v.type = Value::Type::String; v.text = ParseString(); break;
            case 't': Literal("true"); v.type = Value::Type::Bool; v.boolean = true; break;
            case 'f': Literal("false"); v.type = Value::Type::Bool; break;
            case 'n': Literal("null"); break;
            default: ParseNumber(v); break;
        }
        v.end = pos_;
        return v;
    }

    void ParseObject(Value& v, int depth) {
        v.type = Value::Type::Object;
        ++pos_;
        if (Peek() == '}') {
            ++pos_;
            return;
        }
        while (true) {
            if (Peek() != '"') {
                Fail("expected key");
            }
            std::string key = ParseString();
            Expect(':');
            v.members.push_back(Member{std::move(key), ParseValue(depth + 1)});
            if (Peek() == '}') {
                ++pos_;
                return;
            }
            Expect(',');
        }
    }

    void ParseArray(Value& v, int depth) {
        v.type = Value::Type::Array;
        ++pos_;
        if (Peek() == ']') {
            ++pos_;
            return;
        }
        while (true) {
            v.items.push_back(ParseValue(depth + 1));
            if (Peek() == ']') {
                ++pos_;
                return;
            }
            Expect(',');
        }
    }

    void ParseNumber(Value& v) {
        const std::size_t start = pos_;
        while (pos_ < src_.size() && std::string_view("+-0123456789.eE").find(src_[pos_]) !=
                                         std::string_view::npos) {
            ++pos_;
        }
        if (pos_ == start) {
            Fail("unexpected character");
        }
        v.type = Value::Type::Number;
        v.text = std::string(src_.substr(start, pos_ - start));
    }

    unsigned Hex4() {
        if (pos_ + 4 > src_.size()) {
            Fail("short \\u escape");
        }
        unsigned cp = 0;
        const auto* first = src_.data() + pos_;
        auto [ptr, ec] = std::from_chars(first, first + 4, cp, 16);
        if (ec != std::errc() || ptr != first + 4) {
            Fail("bad \\u escape");
        }
        pos_ += 4;
        return cp;
    }

    static void AppendUtf8(std::string& out, unsigned cp) {
        if (cp < 0x80) {
            out.push_back(static_cast<char>(cp));
        } else if (cp < 0x800) {
            out.push_back(static_cast<char>(0xC0 | (cp >> 6)));
            out.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        } else if (cp < 0x10000) {
            out.push_back(static_cast<char>(0xE0 | (cp >> 12)));
            out.push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
            out.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        } else {
            out.push_back(static_cast<char>(0xF0 | (cp >> 18)));
            out.push_back(static_cast<char>(0x80 | ((cp >> 12) & 0x3F)));
            out.push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
            out.push_back(static_cast<char>(0x80 | (cp & 0x3F)));
        }
    }

    unsigned CodePoint() {
        const unsigned hi = Hex4();
        if (hi < 0xD800 || hi > 0xDFFF) {
            return hi;
        }
        if (hi > 0xDBFF || src_.substr(pos_, 2) != "\\u") {
            Fail("lone surrogate");
        }
        pos_ += 2;
        const unsigned lo = Hex4();
        if (lo < 0xDC00 || lo > 0xDFFF) {
            Fail("bad low surrogate");
        }
        return 0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00);
    }

    void Escape(std::string& out) {
        if (pos_ >= src_.size()) {
            Fail("unterminated escape");
        }
        const char e = src_[pos_++];
        switch (e) {
            case '"': out.push_back('"'); break;
            case '\\': out.push_back('\\'); break;
            case '/': out.push_back('/'); break;
            case 'b': out.push_back('\b'); break;
            case 'f': out.push_back('\f'); break;
            case 'n': out.push_back('\n'); break;
            case 'r': out.push_back('\r'); break;
            case 't': out.push_back('\t'); break;
            case 'u': AppendUtf8(out, CodePoint()); break;
            default: Fail("bad escape");
        }
    }

    std::string ParseString() {
        ++pos_;  // opening quote
        std::string out;
        while (true) {
            const std::size_t run = pos_;
            while (pos_ < src_.size() && src_[pos_] != '"' && src_[pos_] != '\\') {
                ++pos_;
            }
            out.append(src_.substr(run, pos_ - run));
            if (pos_ >= src_.size()) {
                Fail("unterminated string");
            }
            if (src_[pos_++] == '"') {
                return out;
            }
            Escape(out);
        }
    }
};

/// Parse a complete JSON document (throws SzConfigToolException INTERNAL).
[[nodiscard]] inline Value Parse(std::string_view src) { return Parser(src).Document(); }

/// Append `s` as a JSON string literal.
inline void AppendQuoted(std::string& out, std::string_view s) {
    static constexpr char kHex[] = "0123456789abcdef";
    out.push_back('"');
    for (const char c : s) {
        const auto u = static_cast<unsigned char>(c);
        switch (c) {
            case '"': out += "\\\""; break;
            case '\\': out += "\\\\"; break;
            case '\n': out += "\\n"; break;
            case '\r': out += "\\r"; break;
            case '\t': out += "\\t"; break;
            default:
                if (u < 0x20) {
                    out += "\\u00";
                    out.push_back(kHex[u >> 4]);
                    out.push_back(kHex[u & 0xF]);
                } else {
                    out.push_back(c);
                }
        }
    }
    out.push_back('"');
}

}  // namespace json

// ---------------------------------------------------------------------------
// args_json writer
// ---------------------------------------------------------------------------
class ArgsWriter {
public:
    void Str(std::string_view key, std::string_view v) { json::AppendQuoted(Key(key), v); }
    void Int(std::string_view key, std::int64_t v) { Key(key) += std::to_string(v); }
    void Bool(std::string_view key, bool v) { Key(key) += v ? "true" : "false"; }
    /// `v` is JSON text, inserted verbatim (the library validates it).
    void Json(std::string_view key, std::string_view v) { Key(key) += v; }
    void Null(std::string_view key) { Key(key) += "null"; }
    /// An `int_or_str` value: a JSON integer or a JSON string.
    void IntOrStr(std::string_view key, const std::variant<std::int64_t, std::string>& v) {
        if (const auto* i = std::get_if<std::int64_t>(&v)) {
            Int(key, *i);
        } else {
            Str(key, std::get<std::string>(v));
        }
    }
    void StrList(std::string_view key, const std::vector<std::string>& v) {
        std::string& out = Key(key);
        out.push_back('[');
        for (std::size_t i = 0; i < v.size(); ++i) {
            if (i != 0) {
                out.push_back(',');
            }
            json::AppendQuoted(out, v[i]);
        }
        out.push_back(']');
    }
    [[nodiscard]] std::string Finish() const { return buf_ + "}"; }

private:
    std::string buf_ = "{";
    std::string& Key(std::string_view key) {
        if (buf_.size() > 1) {
            buf_.push_back(',');
        }
        json::AppendQuoted(buf_, key);
        buf_.push_back(':');
        return buf_;
    }
};

// ---------------------------------------------------------------------------
// C boundary
// ---------------------------------------------------------------------------
struct CFree {
    void operator()(char* p) const noexcept { SzConfigTool_free(p); }
};
/// Owner of a library-allocated string (released with SzConfigTool_free).
using OwnedCString = std::unique_ptr<char, CFree>;

inline std::string CopyOr(const char* p, std::string fallback) {
    return p != nullptr ? std::string(p) : std::move(fallback);
}

/// Build the exception from this thread's last-error slot (read at once:
/// the pointers are valid only until the next SzConfigTool_* call). A missing
/// reason code is INVALID_INPUT for -1 (null / non-UTF-8 argument) and
/// INTERNAL otherwise (e.g. a caught panic).
[[nodiscard]] inline SzConfigToolException LastError(std::int64_t return_code) {
    const std::string fallback = return_code == -1 ? "INVALID_INPUT" : "INTERNAL";
    std::string reason = CopyOr(SzConfigTool_getLastErrorReasonCode(), fallback);
    std::string message = CopyOr(SzConfigTool_getLastError(),
                                 "SzConfigTool_invoke failed with return code " +
                                     std::to_string(return_code));
    const char* details = SzConfigTool_getLastErrorDetails();
    std::optional<std::string> det;
    if (details != nullptr) {
        det = std::string(details);
    }
    return SzConfigToolException(std::move(reason), message, std::move(det), return_code);
}

inline void RequireNoNul(std::string_view s, const char* what) {
    if (s.find('\0') != std::string_view::npos) {
        throw SzConfigToolException("INVALID_INPUT",
                                    std::string(what) + " contains an embedded NUL byte", std::nullopt,
                                    -1);
    }
}

/// Call SzConfigTool_invoke and decode the success envelope.
[[nodiscard]] inline InvokeResult InvokeRaw(const std::string& name, const std::string& config_json,
                                            const std::string& args_json) {
    RequireNoNul(name, "name");
    RequireNoNul(config_json, "config_json");
    RequireNoNul(args_json, "args_json");
    const SzConfigTool_result r =
        SzConfigTool_invoke(name.c_str(), config_json.c_str(), args_json.c_str());
    const OwnedCString response(r.response);
    if (r.returnCode != 0 || response == nullptr) {
        throw LastError(r.returnCode);
    }
    const std::string_view text(response.get());
    const json::Value env = json::Parse(text);
    const json::Value* kind = env.Find("kind");
    if (kind == nullptr || kind->type != json::Value::Type::String) {
        ThrowInternal("envelope without kind");
    }
    InvokeResult out{kind->text, std::nullopt, std::nullopt};
    if (const json::Value* c = env.Find("config"); c != nullptr) {
        out.config = c->text;
    }
    if (const json::Value* res = env.Find("result"); res != nullptr) {
        out.result = std::string(text.substr(res->begin, res->end - res->begin));
    }
    return out;
}

/// Decoded envelope of a typed call (fields empty when absent for the kind).
struct Envelope {
    std::string config;
    std::string result;
};

[[nodiscard]] inline Envelope Call(const char* name, const std::string& config_json,
                                   const std::string& args_json, ResultKind expected) {
    InvokeResult r = InvokeRaw(name, config_json, args_json);
    if (r.kind != WireName(expected)) {
        ThrowInternal(std::string(name) + ": unexpected envelope kind '" + r.kind + "'");
    }
    return Envelope{std::move(r.config).value_or(std::string()),
                    std::move(r.result).value_or(std::string())};
}

[[nodiscard]] inline std::int64_t ParseInt(std::string_view text) {
    std::int64_t v = 0;
    auto [ptr, ec] = std::from_chars(text.data(), text.data() + text.size(), v);
    if (ec != std::errc() || ptr != text.data() + text.size()) {
        ThrowInternal("expected an integer result, got '" + std::string(text) + "'");
    }
    return v;
}

/// A result record parsed once; Member() returns a member's verbatim JSON
/// text (`1001`, `true`, `"4.0.0"` including the quotes).
class Record {
public:
    explicit Record(std::string text) : text_(std::move(text)), root_(json::Parse(text_)) {}

    [[nodiscard]] std::string Member(std::string_view key) const {
        const json::Value* m = root_.Find(key);
        if (root_.type != json::Value::Type::Object || m == nullptr) {
            ThrowInternal("record lacks field '" + std::string(key) + "'");
        }
        return text_.substr(m->begin, m->end - m->begin);
    }

private:
    std::string text_;
    json::Value root_;
};

}  // namespace detail

/// Call any manifest function by its snake_case wire name (including
/// `not_implemented` placeholders, which always fail NOT_IMPLEMENTED).
/// `args_json` is a JSON object keyed by the manifest arg names.
[[nodiscard]] inline InvokeResult Invoke(std::string_view name, const std::string& config_json,
                                         std::string_view args_json = "{}") {
    return detail::InvokeRaw(std::string(name), config_json, std::string(args_json));
}

/// Version of the loaded libSzConfigTool (e.g. "4.4.0-1").
[[nodiscard]] inline std::string LibraryVersion() {
    return detail::CopyOr(SzConfigTool_getLibraryVersion(), std::string());
}

/// C ABI version of the loaded library (`SZCONFIGTOOL_ABI_VERSION` it was built with).
[[nodiscard]] inline int AbiVersion() noexcept {
    return SzConfigTool_getAbiVersion();
}

/// True when the loaded library's ABI matches the header compiled against.
[[nodiscard]] inline bool AbiCompatible() noexcept {
    return SzConfigTool_getAbiVersion() == SZCONFIGTOOL_ABI_VERSION;
}

}  // namespace szconfigtool

#endif  // SZCONFIGTOOL_CORE_HPP
