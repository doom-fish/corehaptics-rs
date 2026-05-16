//! `CHHapticDynamicParameter` wrapper values.

use serde::{Deserialize, Serialize};

/// Dynamic parameter identifiers from `CHHapticParameter.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl DynamicParameterId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HapticIntensityControl => "hapticIntensityControl",
            Self::HapticSharpnessControl => "hapticSharpnessControl",
            Self::HapticAttackTimeControl => "hapticAttackTimeControl",
            Self::HapticDecayTimeControl => "hapticDecayTimeControl",
            Self::HapticReleaseTimeControl => "hapticReleaseTimeControl",
            Self::AudioVolumeControl => "audioVolumeControl",
            Self::AudioPanControl => "audioPanControl",
            Self::AudioBrightnessControl => "audioBrightnessControl",
            Self::AudioPitchControl => "audioPitchControl",
            Self::AudioAttackTimeControl => "audioAttackTimeControl",
            Self::AudioDecayTimeControl => "audioDecayTimeControl",
            Self::AudioReleaseTimeControl => "audioReleaseTimeControl",
        }
    }
}

/// A single `CHHapticDynamicParameter` value object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicParameter {
    parameter_id: DynamicParameterId,
    value: f32,
    relative_time: f64,
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

    #[must_use]
    pub const fn parameter_id(&self) -> DynamicParameterId {
        self.parameter_id
    }

    #[must_use]
    pub const fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    #[must_use]
    pub const fn relative_time(&self) -> f64 {
        self.relative_time
    }

    pub fn set_relative_time(&mut self, relative_time: f64) {
        self.relative_time = relative_time;
    }

    #[must_use]
    pub const fn haptic_intensity_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::HapticIntensityControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn haptic_sharpness_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::HapticSharpnessControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn haptic_attack_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::HapticAttackTimeControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn haptic_decay_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::HapticDecayTimeControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn haptic_release_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::HapticReleaseTimeControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn audio_volume_control(value: f32, relative_time: f64) -> Self {
        Self::new(DynamicParameterId::AudioVolumeControl, value, relative_time)
    }

    #[must_use]
    pub const fn audio_pan_control(value: f32, relative_time: f64) -> Self {
        Self::new(DynamicParameterId::AudioPanControl, value, relative_time)
    }

    #[must_use]
    pub const fn audio_brightness_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::AudioBrightnessControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn audio_pitch_control(value: f32, relative_time: f64) -> Self {
        Self::new(DynamicParameterId::AudioPitchControl, value, relative_time)
    }

    #[must_use]
    pub const fn audio_attack_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::AudioAttackTimeControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn audio_decay_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::AudioDecayTimeControl,
            value,
            relative_time,
        )
    }

    #[must_use]
    pub const fn audio_release_time_control(value: f32, relative_time: f64) -> Self {
        Self::new(
            DynamicParameterId::AudioReleaseTimeControl,
            value,
            relative_time,
        )
    }
}
