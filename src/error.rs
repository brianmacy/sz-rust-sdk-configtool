use std::fmt;

/// Custom error type for configuration operations.
///
/// # Stable, machine-readable surface
///
/// `SzConfigError` is the public error boundary for this crate. Its **variant
/// set** is a stable contract: callers should branch on the variant — or, more
/// conveniently, on the payload-free [`SzErrorKind`] discriminant returned by
/// [`SzConfigError::kind`], or the string returned by
/// [`SzConfigError::reason_code`] — rather than sniffing the human-facing
/// [`Display`](std::fmt::Display) text. The `Display` wording is **not** part of
/// the contract and may change between releases; the variant set,
/// [`SzErrorKind`], and [`reason_code`](SzConfigError::reason_code) are the
/// stable surface downstream code (notably the CLI adapter) should rely on.
///
/// The variant set will not be restructured without a version bump. The enum is
/// deliberately **not** `#[non_exhaustive]`, so downstream `match` expressions
/// can be exhaustive today. Under this crate's stability policy a new variant
/// may be introduced to split a previously-conflated sub-case out of an existing
/// one (as [`NotOnCall`](SzConfigError::NotOnCall) and
/// [`AlreadyPresent`](SzConfigError::AlreadyPresent) were carved out of the
/// broader "not found" / "already exists" families); such an addition is
/// released with a version bump and called out in the changelog so exhaustive
/// downstream matches can be updated.
///
/// Note that [`kind`](SzConfigError::kind) and
/// [`reason_code`](SzConfigError::reason_code) are variant-level only: two
/// distinct "not found" situations that share a variant both classify the same
/// way (e.g. two [`NotFound`](SzConfigError::NotFound) cases both report
/// [`SzErrorKind::NotFound`]). Where a sub-case is common enough to branch on it
/// is promoted to its own variant with its own stable
/// [`reason_code`](SzConfigError::reason_code); otherwise, finer discrimination
/// within a variant is not part of this surface.
#[derive(Debug)]
pub enum SzConfigError {
    /// JSON parsing error
    JsonParse(String),
    /// Item not found
    NotFound(String), // Generic not found with description
    /// A call-element delete targeted an element that is not on the given call.
    ///
    /// This is the benign single-call "not on call" sub-case carved out of the
    /// broader [`NotFound`](Self::NotFound) family: the call exists (or is
    /// addressed by id) but the requested feature/element is not one of its BOM
    /// rows, so there is nothing to delete. Its
    /// [`reason_code`](Self::reason_code) is `"NOT_ON_CALL"`.
    NotOnCall(String),
    /// A call-element operation named an element that is not part of the
    /// (element-)feature it was addressed under.
    ///
    /// This is the hard-error counterpart to [`NotOnCall`](Self::NotOnCall),
    /// distinguishing Python's two-tier check: when a call-element delete is
    /// addressed with an element feature, the element must first be an element of
    /// that feature (a `CFG_FBOM` member) — if it is not, that is a genuine error
    /// (mirroring Python's `"{element} is not an element of {feature}"`), not the
    /// benign "the element is valid but not on this call" ([`NotOnCall`](Self::NotOnCall)).
    /// Its [`reason_code`](Self::reason_code) is `"NOT_IN_FEATURE"`.
    NotInFeature(String),
    /// Item already exists
    AlreadyExists(String), // Generic already exists with description
    /// A call or call-element add targeted something that is already present.
    ///
    /// This is the benign single-call "already there" sub-case carved out of the
    /// broader [`AlreadyExists`](Self::AlreadyExists) family: a per-feature call
    /// is already set, or the feature/element is already a BOM row of the call,
    /// so the add is a no-op rather than a hard collision (contrast a taken
    /// explicit id or exec-order, which remain [`AlreadyExists`](Self::AlreadyExists)).
    /// Its [`reason_code`](Self::reason_code) is `"ALREADY_PRESENT"`.
    AlreadyPresent(String),
    /// Invalid input
    InvalidInput(String),
    /// Missing required section
    MissingSection(String),
    /// Invalid configuration structure
    InvalidStructure(String),
    /// Missing required field
    MissingField(String),
    /// Invalid configuration state
    InvalidConfig(String),
    /// Not implemented
    NotImplemented(String),
    /// One or more field-level validation failures collected together.
    ///
    /// This mirrors Python `validateGenericThreshold`'s aggregated `errorList`:
    /// rather than failing fast on the first bad field, every failure is
    /// collected into one [`ValidationFailure`] per offending field and reported
    /// together. Each failure carries the field tag and a machine-readable
    /// [`ValidationReason`] as **data**, so a caller (notably the CLI) can
    /// reproduce its own user-facing wording without sniffing prose. Its
    /// [`reason_code`](Self::reason_code) is `"VALIDATION_ERRORS"`; use
    /// [`validation_failures`](Self::validation_failures) to recover the vector.
    ///
    /// The `Display` text re-creates a `"; "`-joined summary for logs/FFI, but
    /// that wording is **not** part of the stable contract — branch on
    /// [`kind`](Self::kind)/[`validation_failures`](Self::validation_failures).
    ValidationErrors(Vec<ValidationFailure>),
}

/// Stable, DATA-only taxonomy of *why* a single field failed validation.
///
/// This is a machine-readable classifier, never user-facing wording: a consumer
/// (notably the CLI adapter) maps a `(field, reason_code)` pair to its own
/// display text. The enum is `#[non_exhaustive]` so future reason codes can be
/// added without a breaking change.
///
/// Only [`OutOfDomain`](Self::OutOfDomain) (a `sendToRedo` outside `[Yes, No]`)
/// and [`UnknownReferenceCode`](Self::UnknownReferenceCode) (a behaviour not in
/// the canonical set) are emitted today; the remaining codes are reserved for a
/// stable taxonomy (presence is caller-owned, plan/feature stay fatal-first
/// [`NotFound`](SzConfigError::NotFound), duplicates stay warning-success, and
/// caps are rejected strictly upstream as scalar [`InvalidInput`](SzConfigError::InvalidInput)).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationReason {
    /// A required field was absent (reserved — presence is caller-owned).
    Missing,
    /// A field was present but of the wrong scalar type (reserved — caps are
    /// rejected strictly upstream today).
    WrongType,
    /// A value fell outside a fixed enumerated domain (e.g. `sendToRedo` not in
    /// `[Yes, No]`).
    OutOfDomain,
    /// A value was not a member of a canonical reference-code set (e.g. a
    /// behaviour not in the canonical behaviour codes).
    UnknownReferenceCode,
    /// A referenced entity was not found (reserved — plan/feature lookups stay
    /// fatal-first [`NotFound`](SzConfigError::NotFound)).
    NotFound,
    /// A duplicate was detected (reserved — duplicates stay warning-success).
    Duplicate,
}

impl ValidationReason {
    /// Return the stable `SCREAMING_SNAKE_CASE` reason-code string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "MISSING",
            Self::WrongType => "WRONG_TYPE",
            Self::OutOfDomain => "OUT_OF_DOMAIN",
            Self::UnknownReferenceCode => "UNKNOWN_REFERENCE_CODE",
            Self::NotFound => "NOT_FOUND",
            Self::Duplicate => "DUPLICATE",
        }
    }
}

/// A single field-level validation failure, carried as DATA (never prose).
///
/// Aggregated inside [`SzConfigError::ValidationErrors`]. The struct is
/// `#[non_exhaustive]` so future fields (e.g. a row index) can be added without
/// a breaking change.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFailure {
    /// Canonical camelCase field tag, e.g. `"behavior"`, `"sendToRedo"` — the
    /// CLI/JSON attribute name, not the on-disk column.
    pub field: &'static str,
    /// The stable reason this field failed.
    pub reason_code: ValidationReason,
    /// The offending value verbatim, for echoing; `None` when the failure is not
    /// tied to a specific scalar value.
    pub offending_value: Option<String>,
}

impl ValidationFailure {
    /// Construct a validation failure from its parts.
    pub fn new(
        field: &'static str,
        reason_code: ValidationReason,
        offending_value: Option<String>,
    ) -> Self {
        Self {
            field,
            reason_code,
            offending_value,
        }
    }
}

/// Stable, variant-level discriminant for [`SzConfigError`].
///
/// This mirrors the set of [`SzConfigError`] variants without carrying any of
/// their payloads. It lets callers (notably the CLI) classify an error by
/// category and branch on it in a `match` without string-sniffing the
/// `Display` output.
///
/// # Note on granularity
///
/// This is a **variant-level** discriminant only. It intentionally does not
/// distinguish sub-cases *within* a variant — for example, two different
/// "not found" situations that share the [`NotFound`](SzConfigError::NotFound)
/// variant both report [`SzErrorKind::NotFound`]. Where a sub-case is worth
/// branching on it is promoted to its own variant with its own discriminant
/// (see [`NotOnCall`](SzErrorKind::NotOnCall) and
/// [`AlreadyPresent`](SzErrorKind::AlreadyPresent)); callers needing finer
/// discrimination than the variant set provides must still inspect the message,
/// and this method does not discharge that need.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SzErrorKind {
    /// JSON could not be parsed. Corresponds to [`SzConfigError::JsonParse`].
    JsonParse,
    /// A requested item was not found. Corresponds to [`SzConfigError::NotFound`].
    NotFound,
    /// A call-element delete targeted an element not on the call. Corresponds to [`SzConfigError::NotOnCall`].
    NotOnCall,
    /// A call-element operation named an element not in the addressed feature. Corresponds to [`SzConfigError::NotInFeature`].
    NotInFeature,
    /// An item already exists. Corresponds to [`SzConfigError::AlreadyExists`].
    AlreadyExists,
    /// A call or call-element add targeted something already present. Corresponds to [`SzConfigError::AlreadyPresent`].
    AlreadyPresent,
    /// Input failed validation. Corresponds to [`SzConfigError::InvalidInput`].
    InvalidInput,
    /// A required config section is missing. Corresponds to [`SzConfigError::MissingSection`].
    MissingSection,
    /// The config structure is invalid. Corresponds to [`SzConfigError::InvalidStructure`].
    InvalidStructure,
    /// A required field is missing. Corresponds to [`SzConfigError::MissingField`].
    MissingField,
    /// The configuration state is invalid. Corresponds to [`SzConfigError::InvalidConfig`].
    InvalidConfig,
    /// The operation is not implemented. Corresponds to [`SzConfigError::NotImplemented`].
    NotImplemented,
    /// One or more field-level validation failures were aggregated. Corresponds
    /// to [`SzConfigError::ValidationErrors`].
    ValidationErrors,
}

impl SzConfigError {
    /// Return the variant-level [`SzErrorKind`] discriminant for this error.
    ///
    /// This is a non-allocating classifier that lets callers branch on the
    /// category of an error without matching on the payload-carrying variants
    /// or sniffing `Display` text.
    ///
    /// # Example
    ///
    /// ```
    /// use sz_configtool_lib::{SzConfigError, error::SzErrorKind};
    ///
    /// let err = SzConfigError::not_found("Rule not found: FOO");
    /// assert_eq!(err.kind(), SzErrorKind::NotFound);
    /// ```
    pub fn kind(&self) -> SzErrorKind {
        match self {
            Self::JsonParse(_) => SzErrorKind::JsonParse,
            Self::NotFound(_) => SzErrorKind::NotFound,
            Self::NotOnCall(_) => SzErrorKind::NotOnCall,
            Self::NotInFeature(_) => SzErrorKind::NotInFeature,
            Self::AlreadyExists(_) => SzErrorKind::AlreadyExists,
            Self::AlreadyPresent(_) => SzErrorKind::AlreadyPresent,
            Self::InvalidInput(_) => SzErrorKind::InvalidInput,
            Self::MissingSection(_) => SzErrorKind::MissingSection,
            Self::InvalidStructure(_) => SzErrorKind::InvalidStructure,
            Self::MissingField(_) => SzErrorKind::MissingField,
            Self::InvalidConfig(_) => SzErrorKind::InvalidConfig,
            Self::NotImplemented(_) => SzErrorKind::NotImplemented,
            Self::ValidationErrors(_) => SzErrorKind::ValidationErrors,
        }
    }

    /// Return a stable, machine-readable reason code string for this error.
    ///
    /// The returned code is a `SCREAMING_SNAKE_CASE` identifier that is stable
    /// across releases (unlike the human-facing `Display` message). It is
    /// suitable for logging, telemetry, or crossing an FFI boundary where a
    /// compact classifier is preferred over a Rust enum.
    ///
    /// Like [`SzConfigError::kind`], this is variant-level only and does not
    /// distinguish sub-cases within a variant.
    ///
    /// # Example
    ///
    /// ```
    /// use sz_configtool_lib::SzConfigError;
    ///
    /// let err = SzConfigError::already_exists("Rule 'FOO' already exists");
    /// assert_eq!(err.reason_code(), "ALREADY_EXISTS");
    /// ```
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::JsonParse(_) => "JSON_PARSE",
            Self::NotFound(_) => "NOT_FOUND",
            Self::NotOnCall(_) => "NOT_ON_CALL",
            Self::NotInFeature(_) => "NOT_IN_FEATURE",
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::AlreadyPresent(_) => "ALREADY_PRESENT",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::MissingSection(_) => "MISSING_SECTION",
            Self::InvalidStructure(_) => "INVALID_STRUCTURE",
            Self::MissingField(_) => "MISSING_FIELD",
            Self::InvalidConfig(_) => "INVALID_CONFIG",
            Self::NotImplemented(_) => "NOT_IMPLEMENTED",
            Self::ValidationErrors(_) => "VALIDATION_ERRORS",
        }
    }

    /// Create a JSON parse error
    pub fn json_parse<S: Into<String>>(msg: S) -> Self {
        Self::JsonParse(msg.into())
    }

    /// Create a not found error
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a not-on-call error (call-element delete found nothing to remove)
    pub fn not_on_call<S: Into<String>>(msg: S) -> Self {
        Self::NotOnCall(msg.into())
    }

    /// Create a not-in-feature error (element is not a member of the addressed feature)
    pub fn not_in_feature<S: Into<String>>(msg: S) -> Self {
        Self::NotInFeature(msg.into())
    }

    /// Create an already exists error
    pub fn already_exists<S: Into<String>>(msg: S) -> Self {
        Self::AlreadyExists(msg.into())
    }

    /// Create an already-present error (call or call-element add is a no-op)
    pub fn already_present<S: Into<String>>(msg: S) -> Self {
        Self::AlreadyPresent(msg.into())
    }

    /// Create an invalid input error (validation)
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Create a not implemented error
    pub fn not_implemented<S: Into<String>>(msg: S) -> Self {
        Self::NotImplemented(msg.into())
    }

    /// Return the bare inner message payload carried by this error.
    ///
    /// Every variant wraps a single `String`; this returns that string directly,
    /// without any of the category prefixes that
    /// [`Display`](std::fmt::Display) prepends for some variants (e.g. the
    /// `"Invalid input: "` in front of an [`InvalidInput`](Self::InvalidInput)).
    /// It is the complement of [`kind`](Self::kind)/[`reason_code`](Self::reason_code):
    /// those classify the error, this recovers its human-readable detail without
    /// re-parsing the formatted string.
    ///
    /// # Example
    ///
    /// ```
    /// use sz_configtool_lib::SzConfigError;
    ///
    /// let err = SzConfigError::validation("DISPLAY_LEVEL must be >= 0");
    /// // Display prefixes the category; message() is the bare payload.
    /// assert_eq!(err.to_string(), "Invalid input: DISPLAY_LEVEL must be >= 0");
    /// assert_eq!(err.message(), "DISPLAY_LEVEL must be >= 0");
    /// ```
    pub fn message(&self) -> &str {
        match self {
            Self::JsonParse(msg)
            | Self::NotFound(msg)
            | Self::NotOnCall(msg)
            | Self::NotInFeature(msg)
            | Self::AlreadyExists(msg)
            | Self::AlreadyPresent(msg)
            | Self::InvalidInput(msg)
            | Self::MissingSection(msg)
            | Self::InvalidStructure(msg)
            | Self::MissingField(msg)
            | Self::InvalidConfig(msg)
            | Self::NotImplemented(msg) => msg,
            // ValidationErrors carries a Vec, not a single String payload, so it
            // cannot join into a borrowed &str. Return a fixed summary; callers
            // wanting detail use validation_failures() or Display.
            Self::ValidationErrors(_) => "one or more validation failures",
        }
    }

    /// Return the aggregated [`ValidationFailure`]s when this is a
    /// [`ValidationErrors`](Self::ValidationErrors), else `None`.
    ///
    /// This lets Rust callers recover the structured, DATA-only failures without
    /// matching on the variant or parsing the `Display` text.
    ///
    /// # Example
    ///
    /// ```
    /// use sz_configtool_lib::{SzConfigError, error::{ValidationFailure, ValidationReason}};
    ///
    /// let err = SzConfigError::ValidationErrors(vec![ValidationFailure::new(
    ///     "sendToRedo",
    ///     ValidationReason::OutOfDomain,
    ///     Some("perhaps".into()),
    /// )]);
    /// let failures = err.validation_failures().unwrap();
    /// assert_eq!(failures[0].field, "sendToRedo");
    /// assert_eq!(failures[0].reason_code, ValidationReason::OutOfDomain);
    /// ```
    pub fn validation_failures(&self) -> Option<&[ValidationFailure]> {
        match self {
            Self::ValidationErrors(failures) => Some(failures),
            _ => None,
        }
    }
}

impl fmt::Display for SzConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::JsonParse(msg) => write!(f, "JSON parse error: {msg}"),
            Self::NotFound(msg) => write!(f, "{msg}"),
            Self::NotOnCall(msg) => write!(f, "{msg}"),
            Self::NotInFeature(msg) => write!(f, "{msg}"),
            Self::AlreadyExists(msg) => write!(f, "{msg}"),
            Self::AlreadyPresent(msg) => write!(f, "{msg}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::MissingSection(section) => write!(f, "Missing config section: {section}"),
            Self::InvalidStructure(msg) => write!(f, "Invalid config structure: {msg}"),
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {msg}"),
            Self::NotImplemented(msg) => write!(f, "Not implemented: {msg}"),
            // Re-create the old `"; "`-joined summary so logs/FFI stay readable.
            // This wording is explicitly NOT part of the stable contract.
            Self::ValidationErrors(failures) => {
                let joined = failures
                    .iter()
                    .map(|fail| match &fail.offending_value {
                        Some(v) => format!("{} {} '{v}'", fail.field, fail.reason_code.as_str()),
                        None => format!("{} {}", fail.field, fail.reason_code.as_str()),
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                write!(f, "Invalid input: {joined}")
            }
        }
    }
}

impl std::error::Error for SzConfigError {}

impl From<serde_json::Error> for SzConfigError {
    fn from(err: serde_json::Error) -> Self {
        SzConfigError::JsonParse(err.to_string())
    }
}

/// Result type for configuration operations
pub type Result<T> = std::result::Result<T, SzConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_and_reason_code_cover_all_variants() {
        let cases: [(SzConfigError, SzErrorKind, &str); 13] = [
            (
                SzConfigError::JsonParse("x".into()),
                SzErrorKind::JsonParse,
                "JSON_PARSE",
            ),
            (
                SzConfigError::NotFound("x".into()),
                SzErrorKind::NotFound,
                "NOT_FOUND",
            ),
            (
                SzConfigError::NotOnCall("x".into()),
                SzErrorKind::NotOnCall,
                "NOT_ON_CALL",
            ),
            (
                SzConfigError::NotInFeature("x".into()),
                SzErrorKind::NotInFeature,
                "NOT_IN_FEATURE",
            ),
            (
                SzConfigError::AlreadyExists("x".into()),
                SzErrorKind::AlreadyExists,
                "ALREADY_EXISTS",
            ),
            (
                SzConfigError::AlreadyPresent("x".into()),
                SzErrorKind::AlreadyPresent,
                "ALREADY_PRESENT",
            ),
            (
                SzConfigError::InvalidInput("x".into()),
                SzErrorKind::InvalidInput,
                "INVALID_INPUT",
            ),
            (
                SzConfigError::MissingSection("x".into()),
                SzErrorKind::MissingSection,
                "MISSING_SECTION",
            ),
            (
                SzConfigError::InvalidStructure("x".into()),
                SzErrorKind::InvalidStructure,
                "INVALID_STRUCTURE",
            ),
            (
                SzConfigError::MissingField("x".into()),
                SzErrorKind::MissingField,
                "MISSING_FIELD",
            ),
            (
                SzConfigError::InvalidConfig("x".into()),
                SzErrorKind::InvalidConfig,
                "INVALID_CONFIG",
            ),
            (
                SzConfigError::NotImplemented("x".into()),
                SzErrorKind::NotImplemented,
                "NOT_IMPLEMENTED",
            ),
            (
                SzConfigError::ValidationErrors(vec![ValidationFailure::new(
                    "behavior",
                    ValidationReason::UnknownReferenceCode,
                    Some("BOGUS".into()),
                )]),
                SzErrorKind::ValidationErrors,
                "VALIDATION_ERRORS",
            ),
        ];

        for (err, kind, code) in cases {
            assert_eq!(err.kind(), kind, "kind mismatch for {err:?}");
            assert_eq!(err.reason_code(), code, "reason_code mismatch for {err:?}");
        }
    }

    #[test]
    fn test_message_returns_bare_payload_for_all_variants() {
        // message() is the raw inner payload for every single-String variant,
        // with none of the category prefixes Display prepends. ValidationErrors
        // carries a Vec (no bare payload) and is asserted separately below.
        let variants: [SzConfigError; 11] = [
            SzConfigError::JsonParse("p".into()),
            SzConfigError::NotFound("p".into()),
            SzConfigError::NotOnCall("p".into()),
            SzConfigError::AlreadyExists("p".into()),
            SzConfigError::AlreadyPresent("p".into()),
            SzConfigError::InvalidInput("p".into()),
            SzConfigError::MissingSection("p".into()),
            SzConfigError::InvalidStructure("p".into()),
            SzConfigError::MissingField("p".into()),
            SzConfigError::InvalidConfig("p".into()),
            SzConfigError::NotImplemented("p".into()),
        ];
        for err in &variants {
            assert_eq!(err.message(), "p", "message mismatch for {err:?}");
        }
    }

    #[test]
    fn test_display_wording_unchanged_for_new_variants() {
        // The new sub-case variants render the BARE message, exactly like their
        // parent NotFound/AlreadyExists variants (no added prefix).
        assert_eq!(
            SzConfigError::NotOnCall("Comparison call element not found".into()).to_string(),
            "Comparison call element not found"
        );
        assert_eq!(
            SzConfigError::AlreadyPresent("Feature/element already exists for call".into())
                .to_string(),
            "Feature/element already exists for call"
        );
    }

    #[test]
    fn test_validation_errors_accessor_and_display_join() {
        let err = SzConfigError::ValidationErrors(vec![
            ValidationFailure::new(
                "behavior",
                ValidationReason::UnknownReferenceCode,
                Some("BOGUS".into()),
            ),
            ValidationFailure::new(
                "sendToRedo",
                ValidationReason::OutOfDomain,
                Some("perhaps".into()),
            ),
        ]);

        // validation_failures() recovers the structured DATA in canonical order.
        let failures = err.validation_failures().expect("is ValidationErrors");
        assert_eq!(failures.len(), 2);
        assert_eq!(failures[0].field, "behavior");
        assert_eq!(
            failures[0].reason_code,
            ValidationReason::UnknownReferenceCode
        );
        assert_eq!(failures[0].offending_value.as_deref(), Some("BOGUS"));
        assert_eq!(failures[1].field, "sendToRedo");
        assert_eq!(failures[1].reason_code, ValidationReason::OutOfDomain);

        // Display re-creates a "; "-joined summary (wording is non-contract).
        let text = err.to_string();
        assert!(
            text.contains("behavior UNKNOWN_REFERENCE_CODE 'BOGUS'"),
            "{text}"
        );
        assert!(text.contains("; "), "failures should be joined: {text}");
        assert!(
            text.contains("sendToRedo OUT_OF_DOMAIN 'perhaps'"),
            "{text}"
        );

        // message() is a fixed summary; other variants return None from the accessor.
        assert_eq!(err.message(), "one or more validation failures");
        assert!(
            SzConfigError::not_found("x")
                .validation_failures()
                .is_none()
        );
    }

    #[test]
    fn test_validation_reason_as_str_stable() {
        assert_eq!(ValidationReason::Missing.as_str(), "MISSING");
        assert_eq!(ValidationReason::WrongType.as_str(), "WRONG_TYPE");
        assert_eq!(ValidationReason::OutOfDomain.as_str(), "OUT_OF_DOMAIN");
        assert_eq!(
            ValidationReason::UnknownReferenceCode.as_str(),
            "UNKNOWN_REFERENCE_CODE"
        );
        assert_eq!(ValidationReason::NotFound.as_str(), "NOT_FOUND");
        assert_eq!(ValidationReason::Duplicate.as_str(), "DUPLICATE");
    }

    #[test]
    fn test_kind_is_copy_and_comparable() {
        let err = SzConfigError::not_found("nope");
        let k = err.kind();
        // Copy semantics: using k twice must compile and compare equal.
        assert_eq!(k, k);
        assert_ne!(k, SzErrorKind::AlreadyExists);
    }
}
