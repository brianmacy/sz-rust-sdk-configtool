use std::fmt;

/// Custom error type for configuration operations
#[derive(Debug)]
pub enum SzConfigError {
    /// JSON parsing error
    JsonParse(String),
    /// Item not found
    NotFound(String), // Generic not found with description
    /// Item already exists
    AlreadyExists(String), // Generic already exists with description
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
/// "not found" situations both report [`SzErrorKind::NotFound`]. Callers that
/// need to tell those apart must still inspect the message (or a future
/// structured field); this method does not discharge that need.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SzErrorKind {
    /// JSON could not be parsed. Corresponds to [`SzConfigError::JsonParse`].
    JsonParse,
    /// A requested item was not found. Corresponds to [`SzConfigError::NotFound`].
    NotFound,
    /// An item already exists. Corresponds to [`SzConfigError::AlreadyExists`].
    AlreadyExists,
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
            Self::AlreadyExists(_) => SzErrorKind::AlreadyExists,
            Self::InvalidInput(_) => SzErrorKind::InvalidInput,
            Self::MissingSection(_) => SzErrorKind::MissingSection,
            Self::InvalidStructure(_) => SzErrorKind::InvalidStructure,
            Self::MissingField(_) => SzErrorKind::MissingField,
            Self::InvalidConfig(_) => SzErrorKind::InvalidConfig,
            Self::NotImplemented(_) => SzErrorKind::NotImplemented,
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
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::MissingSection(_) => "MISSING_SECTION",
            Self::InvalidStructure(_) => "INVALID_STRUCTURE",
            Self::MissingField(_) => "MISSING_FIELD",
            Self::InvalidConfig(_) => "INVALID_CONFIG",
            Self::NotImplemented(_) => "NOT_IMPLEMENTED",
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

    /// Create an already exists error
    pub fn already_exists<S: Into<String>>(msg: S) -> Self {
        Self::AlreadyExists(msg.into())
    }

    /// Create an invalid input error (validation)
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Create a not implemented error
    pub fn not_implemented<S: Into<String>>(msg: S) -> Self {
        Self::NotImplemented(msg.into())
    }
}

impl fmt::Display for SzConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::JsonParse(msg) => write!(f, "JSON parse error: {msg}"),
            Self::NotFound(msg) => write!(f, "{msg}"),
            Self::AlreadyExists(msg) => write!(f, "{msg}"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            Self::MissingSection(section) => write!(f, "Missing config section: {section}"),
            Self::InvalidStructure(msg) => write!(f, "Invalid config structure: {msg}"),
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {msg}"),
            Self::NotImplemented(msg) => write!(f, "Not implemented: {msg}"),
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
        let cases: [(SzConfigError, SzErrorKind, &str); 9] = [
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
                SzConfigError::AlreadyExists("x".into()),
                SzErrorKind::AlreadyExists,
                "ALREADY_EXISTS",
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
        ];

        for (err, kind, code) in cases {
            assert_eq!(err.kind(), kind, "kind mismatch for {err:?}");
            assert_eq!(err.reason_code(), code, "reason_code mismatch for {err:?}");
        }
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
