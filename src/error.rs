//! Errors returned by the `corehaptics` crate.

use core::fmt;

/// A crate-local result type for `CoreHaptics` operations.
pub type Result<T> = std::result::Result<T, CoreHapticsError>;

/// The `CoreHaptics` `NSErrorDomain`.
pub const CORE_HAPTICS_ERROR_DOMAIN: &str = "com.apple.CoreHaptics";

/// Typed `CoreHaptics` error codes from `CHHapticErrors.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum HapticErrorCode {
    /// The engine is not running.
    EngineNotRunning = -4805,
    /// The requested operation is not permitted.
    OperationNotPermitted = -4806,
    /// Starting the engine timed out.
    EngineStartTimeout = -4808,
    /// The current device does not support the requested feature.
    NotSupported = -4809,
    /// The haptics server failed to initialize.
    ServerInitFailed = -4810,
    /// The haptics server was interrupted.
    ServerInterrupted = -4811,
    /// The pattern player handle is invalid.
    InvalidPatternPlayer = -4812,
    /// The pattern data is invalid.
    InvalidPatternData = -4813,
    /// The pattern dictionary is invalid.
    InvalidPatternDictionary = -4814,
    /// The audio session is invalid.
    InvalidAudioSession = -4815,
    /// An engine parameter is invalid.
    InvalidEngineParameter = -4816,
    /// A parameter type is invalid.
    InvalidParameterType = -4820,
    /// An event type is invalid.
    InvalidEventType = -4821,
    /// An event time is invalid.
    InvalidEventTime = -4822,
    /// An event duration is invalid.
    InvalidEventDuration = -4823,
    /// An audio resource is invalid.
    InvalidAudioResource = -4824,
    /// A requested resource is unavailable.
    ResourceNotAvailable = -4825,
    /// An event entry is malformed.
    BadEventEntry = -4830,
    /// A parameter entry is malformed.
    BadParameterEntry = -4831,
    /// A time value is invalid.
    InvalidTime = -4840,
    /// A referenced file could not be found.
    FileNotFound = -4851,
    /// The device has insufficient power.
    InsufficientPower = -4897,
    /// The framework reported an unknown error.
    UnknownError = -4898,
    /// The framework reported a memory error.
    MemoryError = -4899,
}

impl HapticErrorCode {
    /// Converts a raw `CoreHaptics` error code into a typed variant.
    #[must_use]
    pub const fn from_code(code: isize) -> Option<Self> {
        match code {
            -4805 => Some(Self::EngineNotRunning),
            -4806 => Some(Self::OperationNotPermitted),
            -4808 => Some(Self::EngineStartTimeout),
            -4809 => Some(Self::NotSupported),
            -4810 => Some(Self::ServerInitFailed),
            -4811 => Some(Self::ServerInterrupted),
            -4812 => Some(Self::InvalidPatternPlayer),
            -4813 => Some(Self::InvalidPatternData),
            -4814 => Some(Self::InvalidPatternDictionary),
            -4815 => Some(Self::InvalidAudioSession),
            -4816 => Some(Self::InvalidEngineParameter),
            -4820 => Some(Self::InvalidParameterType),
            -4821 => Some(Self::InvalidEventType),
            -4822 => Some(Self::InvalidEventTime),
            -4823 => Some(Self::InvalidEventDuration),
            -4824 => Some(Self::InvalidAudioResource),
            -4825 => Some(Self::ResourceNotAvailable),
            -4830 => Some(Self::BadEventEntry),
            -4831 => Some(Self::BadParameterEntry),
            -4840 => Some(Self::InvalidTime),
            -4851 => Some(Self::FileNotFound),
            -4897 => Some(Self::InsufficientPower),
            -4898 => Some(Self::UnknownError),
            -4899 => Some(Self::MemoryError),
            _ => None,
        }
    }
}

impl fmt::Display for HapticErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::EngineNotRunning => "engine not running",
            Self::OperationNotPermitted => "operation not permitted",
            Self::EngineStartTimeout => "engine start timeout",
            Self::NotSupported => "not supported",
            Self::ServerInitFailed => "server init failed",
            Self::ServerInterrupted => "server interrupted",
            Self::InvalidPatternPlayer => "invalid pattern player",
            Self::InvalidPatternData => "invalid pattern data",
            Self::InvalidPatternDictionary => "invalid pattern dictionary",
            Self::InvalidAudioSession => "invalid audio session",
            Self::InvalidEngineParameter => "invalid engine parameter",
            Self::InvalidParameterType => "invalid parameter type",
            Self::InvalidEventType => "invalid event type",
            Self::InvalidEventTime => "invalid event time",
            Self::InvalidEventDuration => "invalid event duration",
            Self::InvalidAudioResource => "invalid audio resource",
            Self::ResourceNotAvailable => "resource not available",
            Self::BadEventEntry => "bad event entry",
            Self::BadParameterEntry => "bad parameter entry",
            Self::InvalidTime => "invalid time",
            Self::FileNotFound => "file not found",
            Self::InsufficientPower => "insufficient power",
            Self::UnknownError => "unknown error",
            Self::MemoryError => "memory error",
        };
        write!(f, "{name}")
    }
}

/// Errors returned by safe `CoreHaptics` wrapper APIs.
#[derive(Debug)]
#[non_exhaustive]
pub enum CoreHapticsError {
    /// A bridge call unexpectedly returned `NULL`.
    UnexpectedNull(&'static str),
    /// An operation failed without an accompanying `NSError`.
    OperationFailed(&'static str),
    /// An Objective-C operation returned an `NSError`.
    ObjectiveCError {
        /// The operation that failed.
        operation: &'static str,
        /// The raw `NSError` code.
        code: isize,
        /// The `NSError` domain.
        domain: String,
        /// The localized error description.
        description: String,
        /// The typed `CoreHaptics` error code when available.
        haptic_error_code: Option<HapticErrorCode>,
    },
    /// An argument failed local validation.
    InvalidArgument(String),
    /// File I/O failed.
    Io(std::io::Error),
    /// JSON serialization or deserialization failed.
    Json(serde_json::Error),
}

impl CoreHapticsError {
    /// Returns the typed `CoreHaptics` error code when one is available.
    #[must_use]
    pub const fn haptic_error_code(&self) -> Option<HapticErrorCode> {
        match self {
            Self::ObjectiveCError {
                haptic_error_code, ..
            } => *haptic_error_code,
            _ => None,
        }
    }
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
                haptic_error_code,
            } => {
                if let Some(haptic_error_code) = haptic_error_code {
                    write!(
                        f,
                        "{operation} failed: {domain} ({code}, {haptic_error_code}) — {description}"
                    )
                } else {
                    write!(f, "{operation} failed: {domain} ({code}) — {description}")
                }
            }
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Io(error) => write!(f, "I/O failed: {error}"),
            Self::Json(error) => write!(f, "json serialization failed: {error}"),
        }
    }
}

impl std::error::Error for CoreHapticsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CoreHapticsError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for CoreHapticsError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
