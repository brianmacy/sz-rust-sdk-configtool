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

impl SzConfigError {
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
            Self::JsonParse(msg) => write!(f, "JSON parse error: {}", msg),
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::AlreadyExists(msg) => write!(f, "{}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::MissingSection(section) => write!(f, "Missing config section: {}", section),
            Self::InvalidStructure(msg) => write!(f, "Invalid config structure: {}", msg),
            Self::MissingField(field) => write!(f, "Missing required field: {}", field),
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
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
