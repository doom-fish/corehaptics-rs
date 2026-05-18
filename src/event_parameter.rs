//! `CHHapticEventParameter` wrapper values.

use serde::{Deserialize, Serialize};

/// Event parameter identifiers from `CHHapticParameter.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HapticParameterId {
    /// Controls haptic intensity.
    HapticIntensity,
    /// Controls haptic sharpness.
    HapticSharpness,
    /// Controls attack time.
    AttackTime,
    /// Controls decay time.
    DecayTime,
    /// Controls release time.
    ReleaseTime,
    /// Enables or disables sustain.
    Sustained,
    /// Controls audio volume.
    AudioVolume,
    /// Controls audio pitch.
    AudioPitch,
    /// Controls audio pan.
    AudioPan,
    /// Controls audio brightness.
    AudioBrightness,
}

impl HapticParameterId {
    /// Returns the AHAP parameter identifier string.
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
    /// Creates an event parameter value.
    #[must_use]
    pub const fn new(parameter_id: HapticParameterId, value: f32) -> Self {
        Self {
            parameter_id,
            value,
        }
    }

    /// Returns the parameter identifier.
    #[must_use]
    pub const fn parameter_id(&self) -> HapticParameterId {
        self.parameter_id
    }

    /// Returns the parameter value.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.value
    }

    /// Sets the parameter value.
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    /// Creates a haptic intensity parameter.
    #[must_use]
    pub const fn haptic_intensity(value: f32) -> Self {
        Self::new(HapticParameterId::HapticIntensity, value)
    }

    /// Creates a haptic sharpness parameter.
    #[must_use]
    pub const fn haptic_sharpness(value: f32) -> Self {
        Self::new(HapticParameterId::HapticSharpness, value)
    }

    /// Creates an attack-time parameter.
    #[must_use]
    pub const fn attack_time(value: f32) -> Self {
        Self::new(HapticParameterId::AttackTime, value)
    }

    /// Creates a decay-time parameter.
    #[must_use]
    pub const fn decay_time(value: f32) -> Self {
        Self::new(HapticParameterId::DecayTime, value)
    }

    /// Creates a release-time parameter.
    #[must_use]
    pub const fn release_time(value: f32) -> Self {
        Self::new(HapticParameterId::ReleaseTime, value)
    }

    /// Creates a sustained flag parameter.
    #[must_use]
    pub const fn sustained(enabled: bool) -> Self {
        Self::new(
            HapticParameterId::Sustained,
            if enabled { 1.0 } else { 0.0 },
        )
    }

    /// Creates an audio volume parameter.
    #[must_use]
    pub const fn audio_volume(value: f32) -> Self {
        Self::new(HapticParameterId::AudioVolume, value)
    }

    /// Creates an audio pitch parameter.
    #[must_use]
    pub const fn audio_pitch(value: f32) -> Self {
        Self::new(HapticParameterId::AudioPitch, value)
    }

    /// Creates an audio pan parameter.
    #[must_use]
    pub const fn audio_pan(value: f32) -> Self {
        Self::new(HapticParameterId::AudioPan, value)
    }

    /// Creates an audio brightness parameter.
    #[must_use]
    pub const fn audio_brightness(value: f32) -> Self {
        Self::new(HapticParameterId::AudioBrightness, value)
    }
}
