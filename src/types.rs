//! Strongly-typed Rust builders for `CoreHaptics` events and parameters.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HapticEventType {
    HapticTransient,
    HapticContinuous,
    AudioContinuous,
    AudioCustom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HapticParameterId {
    HapticIntensity,
    HapticSharpness,
    AttackTime,
    DecayTime,
    ReleaseTime,
    Sustained,
    AudioVolume,
    AudioPitch,
    AudioPan,
    AudioBrightness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DynamicParameterId {
    HapticIntensityControl,
    HapticSharpnessControl,
    HapticAttackTimeControl,
    HapticDecayTimeControl,
    HapticReleaseTimeControl,
    AudioVolumeControl,
    AudioPanControl,
    AudioBrightnessControl,
    AudioPitchControl,
    AudioAttackTimeControl,
    AudioDecayTimeControl,
    AudioReleaseTimeControl,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticEventParameter {
    pub parameter_id: HapticParameterId,
    pub value: f32,
}

impl HapticEventParameter {
    #[must_use]
    pub const fn new(parameter_id: HapticParameterId, value: f32) -> Self {
        Self { parameter_id, value }
    }

    #[must_use]
    pub const fn haptic_intensity(value: f32) -> Self {
        Self::new(HapticParameterId::HapticIntensity, value)
    }

    #[must_use]
    pub const fn haptic_sharpness(value: f32) -> Self {
        Self::new(HapticParameterId::HapticSharpness, value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicParameter {
    pub parameter_id: DynamicParameterId,
    pub value: f32,
    pub relative_time: f64,
}

impl DynamicParameter {
    #[must_use]
    pub const fn new(parameter_id: DynamicParameterId, value: f32, relative_time: f64) -> Self {
        Self {
            parameter_id,
            value,
            relative_time,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticEvent {
    pub event_type: HapticEventType,
    pub relative_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<HapticEventParameter>,
}

impl HapticEvent {
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn new(
        event_type: HapticEventType,
        relative_time: f64,
        duration: Option<f64>,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self {
            event_type,
            relative_time,
            duration,
            parameters,
        }
    }

    #[must_use]
    pub fn haptic_transient(relative_time: f64, parameters: Vec<HapticEventParameter>) -> Self {
        Self::new(HapticEventType::HapticTransient, relative_time, None, parameters)
    }

    #[must_use]
    pub fn haptic_continuous(
        relative_time: f64,
        duration: f64,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self::new(
            HapticEventType::HapticContinuous,
            relative_time,
            Some(duration),
            parameters,
        )
    }
}
