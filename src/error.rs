//! Errors returned by the `corehaptics` crate.

use core::fmt;

pub type Result<T> = std::result::Result<T, CoreHapticsError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum CoreHapticsError {
    UnexpectedNull(&'static str),
    OperationFailed(&'static str),
    ObjectiveCError {
        operation: &'static str,
        code: isize,
        domain: String,
        description: String,
    },
    InvalidArgument(String),
    Json(serde_json::Error),
}

impl fmt::Display for CoreHapticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedNull(what) => write!(f, "{what} returned NULL"),
            Self::OperationFailed(op) => write!(f, "{op} failed"),
            Self::ObjectiveCError {
                operation,
                code,
                domain,
                description,
            } => write!(
                f,
                "{operation} failed: {domain} ({code}) — {description}"
            ),
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Json(err) => write!(f, "json serialization failed: {err}"),
        }
    }
}

impl std::error::Error for CoreHapticsError {}

impl From<serde_json::Error> for CoreHapticsError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
