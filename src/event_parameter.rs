//! `CHHapticEventParameter` wrapper values.

use serde::{Deserialize, Serialize};

/// Event parameter identifiers from `CHHapticParameter.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl HapticParameterId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HapticIntensity => "hapticIntensity",
            Self::HapticSharpness => "hapticSharpness",
            Self::AttackTime => "attackTime",
            Self::DecayTime => "decayTime",
            Self::ReleaseTime => "releaseTime",
            Self::Sustained => "sustained",
            Self::AudioVolume => "audioVolume",
            Self::AudioPitch => "audioPitch",
            Self::AudioPan => "audioPan",
            Self::AudioBrightness => "audioBrightness",
        }
    }
}

/// A single `CHHapticEventParameter` value object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticEventParameter {
    parameter_id: HapticParameterId,
    value: f32,
}

impl HapticEventParameter {
    #[must_use]
    pub const fn new(parameter_id: HapticParameterId, value: f32) -> Self {
        Self {
            parameter_id,
            value,
        }
    }

    #[must_use]
    pub const fn parameter_id(&self) -> HapticParameterId {
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
    pub const fn haptic_intensity(value: f32) -> Self {
        Self::new(HapticParameterId::HapticIntensity, value)
    }

    #[must_use]
    pub const fn haptic_sharpness(value: f32) -> Self {
        Self::new(HapticParameterId::HapticSharpness, value)
    }

    #[must_use]
    pub const fn attack_time(value: f32) -> Self {
        Self::new(HapticParameterId::AttackTime, value)
    }

    #[must_use]
    pub const fn decay_time(value: f32) -> Self {
        Self::new(HapticParameterId::DecayTime, value)
    }

    #[must_use]
    pub const fn release_time(value: f32) -> Self {
        Self::new(HapticParameterId::ReleaseTime, value)
    }

    #[must_use]
    pub const fn sustained(enabled: bool) -> Self {
        Self::new(
            HapticParameterId::Sustained,
            if enabled { 1.0 } else { 0.0 },
        )
    }

    #[must_use]
    pub const fn audio_volume(value: f32) -> Self {
        Self::new(HapticParameterId::AudioVolume, value)
    }

    #[must_use]
    pub const fn audio_pitch(value: f32) -> Self {
        Self::new(HapticParameterId::AudioPitch, value)
    }

    #[must_use]
    pub const fn audio_pan(value: f32) -> Self {
        Self::new(HapticParameterId::AudioPan, value)
    }

    #[must_use]
    pub const fn audio_brightness(value: f32) -> Self {
        Self::new(HapticParameterId::AudioBrightness, value)
    }
}
