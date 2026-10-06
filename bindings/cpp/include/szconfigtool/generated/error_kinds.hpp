// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

#ifndef SZCONFIGTOOL_GENERATED_ERROR_KINDS_HPP
#define SZCONFIGTOOL_GENERATED_ERROR_KINDS_HPP

#include <array>
#include <string_view>
#include <utility>

namespace szconfigtool {

/// The wire error taxonomy (`project.yaml reason_codes`), plus
/// `Unknown` for a reason code this header does not know.
enum class ErrorKind {
    /// Reason code `JSON_PARSE`.
    JsonParse,
    /// Reason code `NOT_FOUND`.
    NotFound,
    /// Reason code `NOT_ON_CALL`.
    NotOnCall,
    /// Reason code `NOT_IN_FEATURE`.
    NotInFeature,
    /// Reason code `ALREADY_EXISTS`.
    AlreadyExists,
    /// Reason code `ALREADY_PRESENT`.
    AlreadyPresent,
    /// Reason code `INVALID_INPUT`.
    InvalidInput,
    /// Reason code `MISSING_SECTION`.
    MissingSection,
    /// Reason code `INVALID_STRUCTURE`.
    InvalidStructure,
    /// Reason code `MISSING_FIELD`.
    MissingField,
    /// Reason code `INVALID_CONFIG`.
    InvalidConfig,
    /// Reason code `NOT_IMPLEMENTED`.
    NotImplemented,
    /// Reason code `VALIDATION_ERRORS`.
    ValidationErrors,
    /// Reason code `INTERNAL`.
    Internal,
    /// Reason code not in this header's taxonomy.
    Unknown,
};

/// Every known reason code and its kind.
inline constexpr std::array<std::pair<std::string_view, ErrorKind>, 14> kReasonCodes{{
    {"JSON_PARSE", ErrorKind::JsonParse},
    {"NOT_FOUND", ErrorKind::NotFound},
    {"NOT_ON_CALL", ErrorKind::NotOnCall},
    {"NOT_IN_FEATURE", ErrorKind::NotInFeature},
    {"ALREADY_EXISTS", ErrorKind::AlreadyExists},
    {"ALREADY_PRESENT", ErrorKind::AlreadyPresent},
    {"INVALID_INPUT", ErrorKind::InvalidInput},
    {"MISSING_SECTION", ErrorKind::MissingSection},
    {"INVALID_STRUCTURE", ErrorKind::InvalidStructure},
    {"MISSING_FIELD", ErrorKind::MissingField},
    {"INVALID_CONFIG", ErrorKind::InvalidConfig},
    {"NOT_IMPLEMENTED", ErrorKind::NotImplemented},
    {"VALIDATION_ERRORS", ErrorKind::ValidationErrors},
    {"INTERNAL", ErrorKind::Internal},
}};

/// Map a reason code to its kind (`Unknown` if not in the taxonomy).
[[nodiscard]] constexpr ErrorKind ErrorKindFromReasonCode(std::string_view code) noexcept {
    for (const auto& [name, kind] : kReasonCodes) {
        if (name == code) {
            return kind;
        }
    }
    return ErrorKind::Unknown;
}

}  // namespace szconfigtool

#endif  // SZCONFIGTOOL_GENERATED_ERROR_KINDS_HPP
