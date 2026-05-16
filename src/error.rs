//! Errors returned by the `corehaptics` crate.

use core::fmt;

pub type Result<T> = std::result::Result<T, CoreHapticsError>;

/// The `CoreHaptics` `NSErrorDomain`.
pub const CORE_HAPTICS_ERROR_DOMAIN: &str = "com.apple.CoreHaptics";

/// Typed `CoreHaptics` error codes from `CHHapticErrors.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum HapticErrorCode {
    EngineNotRunning = -4805,
    OperationNotPermitted = -4806,
    EngineStartTimeout = -4808,
    NotSupported = -4809,
    ServerInitFailed = -4810,
    ServerInterrupted = -4811,
    InvalidPatternPlayer = -4812,
    InvalidPatternData = -4813,
    InvalidPatternDictionary = -4814,
    InvalidAudioSession = -4815,
    InvalidEngineParameter = -4816,
    InvalidParameterType = -4820,
    InvalidEventType = -4821,
    InvalidEventTime = -4822,
    InvalidEventDuration = -4823,
    InvalidAudioResource = -4824,
    ResourceNotAvailable = -4825,
    BadEventEntry = -4830,
    BadParameterEntry = -4831,
    InvalidTime = -4840,
    FileNotFound = -4851,
    InsufficientPower = -4897,
    UnknownError = -4898,
    MemoryError = -4899,
}

impl HapticErrorCode {
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
        haptic_error_code: Option<HapticErrorCode>,
    },
    InvalidArgument(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl CoreHapticsError {
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
