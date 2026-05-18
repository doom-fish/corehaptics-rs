//! `CHHapticPattern` wrapper and JSON-backed pattern construction.

#![allow(clippy::missing_errors_doc)]

use std::path::Path;

use serde::Serialize;

use crate::{
    dynamic_parameter::DynamicParameter,
    event::HapticEvent,
    object::{c_string, error_from_raw, path_c_string, take_c_string, RetainedObject},
    parameter_curve::ParameterCurve,
};

/// A native `CHHapticPattern` built from Rust or AHAP definitions.
#[derive(Debug, Clone)]
pub struct HapticPattern {
    obj: RetainedObject,
}

const fn slice_is_empty<T>(slice: &[T]) -> bool {
    slice.is_empty()
}

/// String keys used by AHAP pattern dictionaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternKey {
    /// The top-level AHAP version key.
    Version,
    /// The top-level pattern array key.
    Pattern,
    /// An event entry key.
    Event,
    /// The event-type key.
    EventType,
    /// The event start-time key.
    Time,
    /// The event duration key.
    EventDuration,
    /// The custom audio waveform path key.
    EventWaveformPath,
    /// The event-parameters array key.
    EventParameters,
    /// The waveform volume-envelope option key.
    EventWaveformUseVolumeEnvelope,
    /// The waveform loop-enabled option key.
    EventWaveformLoopEnabled,
    /// A dynamic-parameter entry key.
    Parameter,
    /// The parameter identifier key.
    ParameterId,
    /// The parameter value key.
    ParameterValue,
    /// A parameter-curve entry key.
    ParameterCurve,
    /// The parameter-curve control-points key.
    ParameterCurveControlPoints,
}

impl PatternKey {
    /// Returns the AHAP dictionary key string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Version => "Version",
            Self::Pattern => "Pattern",
            Self::Event => "Event",
            Self::EventType => "EventType",
            Self::Time => "Time",
            Self::EventDuration => "EventDuration",
            Self::EventWaveformPath => "EventWaveformPath",
            Self::EventParameters => "EventParameters",
            Self::EventWaveformUseVolumeEnvelope => "EventWaveformUseVolumeEnvelope",
            Self::EventWaveformLoopEnabled => "EventWaveformLoopEnabled",
            Self::Parameter => "Parameter",
            Self::ParameterId => "ParameterID",
            Self::ParameterValue => "ParameterValue",
            Self::ParameterCurve => "ParameterCurve",
            Self::ParameterCurveControlPoints => "ParameterCurveControlPoints",
        }
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
struct PatternEnvelope<'a> {
    events: &'a [HapticEvent],
    #[serde(default, skip_serializing_if = "slice_is_empty")]
    dynamic_parameters: &'a [DynamicParameter],
    #[serde(default, skip_serializing_if = "slice_is_empty")]
    parameter_curves: &'a [ParameterCurve],
}

impl HapticPattern {
    /// Create a native `CHHapticPattern` from Rust event and dynamic-parameter definitions.
    ///
    /// # Errors
    ///
    /// Returns serialization errors or any `NSError` reported by the Swift bridge.
    pub fn new(
        events: &[HapticEvent],
        dynamic_parameters: &[DynamicParameter],
    ) -> crate::Result<Self> {
        Self::from_envelope(&PatternEnvelope {
            events,
            dynamic_parameters,
            parameter_curves: &[],
        })
    }

    /// Create a native `CHHapticPattern` from Rust event and parameter-curve definitions.
    ///
    /// # Errors
    ///
    /// Returns serialization errors or any `NSError` reported by the Swift bridge.
    pub fn with_parameter_curves(
        events: &[HapticEvent],
        parameter_curves: &[ParameterCurve],
    ) -> crate::Result<Self> {
        Self::from_envelope(&PatternEnvelope {
            events,
            dynamic_parameters: &[],
            parameter_curves,
        })
    }

    fn from_envelope(envelope: &PatternEnvelope<'_>) -> crate::Result<Self> {
        let json = serde_json::to_string(envelope)?;
        let json = c_string(&json)?;
        let mut error = core::ptr::null_mut();
        let raw = unsafe { crate::ffi::chrs_pattern_create(json.as_ptr(), &mut error) };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticPattern init",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticPattern init", error) });
        }
        let Some(obj) = (unsafe { RetainedObject::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticPattern init",
            ));
        };
        Ok(Self { obj })
    }

    /// Create a pattern from an AHAP-style JSON dictionary.
    pub fn from_dictionary_json(json: &str) -> crate::Result<Self> {
        let json = c_string(json)?;
        let mut error = core::ptr::null_mut();
        let raw = unsafe {
            crate::ffi::chrs_pattern_create_from_dictionary_json(json.as_ptr(), &mut error)
        };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticPattern initWithDictionary",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticPattern initWithDictionary", error) });
        }
        let Some(obj) = (unsafe { RetainedObject::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticPattern initWithDictionary",
            ));
        };
        Ok(Self { obj })
    }

    /// Create a pattern from a `serde_json::Value` AHAP dictionary.
    pub fn from_dictionary(value: &serde_json::Value) -> crate::Result<Self> {
        Self::from_dictionary_json(&serde_json::to_string(value)?)
    }

    /// Create a pattern from an `.ahap` file path.
    pub fn from_file(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref();
        let path = path_c_string(path)?;
        let mut error = core::ptr::null_mut();
        let raw =
            unsafe { crate::ffi::chrs_pattern_create_from_ahap_file(path.as_ptr(), &mut error) };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticPattern initWithContentsOfURL",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticPattern initWithContentsOfURL", error) });
        }
        let Some(obj) = (unsafe { RetainedObject::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticPattern initWithContentsOfURL",
            ));
        };
        Ok(Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    /// Returns the pattern duration in seconds.
    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { crate::ffi::chrs_pattern_duration(self.as_raw()) }
    }

    /// Export the pattern as an AHAP dictionary JSON string.
    pub fn export_dictionary_json(&self) -> crate::Result<String> {
        let mut error = core::ptr::null_mut();
        let json =
            unsafe { crate::ffi::chrs_pattern_export_dictionary_json(self.as_raw(), &mut error) };
        if json.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticPattern.exportDictionary",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticPattern.exportDictionary", error) });
        }
        unsafe { take_c_string(json) }.ok_or(crate::error::CoreHapticsError::UnexpectedNull(
            "CHHapticPattern.exportDictionary JSON",
        ))
    }

    /// Export the pattern as a `serde_json::Value` AHAP dictionary.
    pub fn export_dictionary(&self) -> crate::Result<serde_json::Value> {
        Ok(serde_json::from_str(&self.export_dictionary_json()?)?)
    }
}
