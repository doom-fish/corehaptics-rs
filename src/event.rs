//! `CHHapticEvent` wrapper values.

use serde::{Deserialize, Serialize};

use crate::event_parameter::HapticEventParameter;

/// Event types from `CHHapticEvent.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HapticEventType {
    /// A transient haptic impulse.
    HapticTransient,
    /// A continuous haptic event.
    HapticContinuous,
    /// A continuous audio event.
    AudioContinuous,
    /// A custom audio-resource event.
    AudioCustom,
}

impl HapticEventType {
    /// Returns the AHAP event-type string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HapticTransient => "hapticTransient",
            Self::HapticContinuous => "hapticContinuous",
            Self::AudioContinuous => "audioContinuous",
            Self::AudioCustom => "audioCustom",
        }
    }
}

/// A registered `CHHapticAudioResourceID`.
pub type AudioResourceId = u64;

/// A single `CHHapticEvent` value object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HapticEvent {
    event_type: HapticEventType,
    relative_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    parameters: Vec<HapticEventParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_resource_id: Option<AudioResourceId>,
}

impl HapticEvent {
    /// Creates an event value.
    #[must_use]
    pub const fn new(
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
            audio_resource_id: None,
        }
    }

    /// Creates a custom-audio event with a registered resource identifier.
    #[must_use]
    pub const fn with_audio_resource(
        audio_resource_id: AudioResourceId,
        relative_time: f64,
        duration: Option<f64>,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self {
            event_type: HapticEventType::AudioCustom,
            relative_time,
            duration,
            parameters,
            audio_resource_id: Some(audio_resource_id),
        }
    }

    /// Returns the event type.
    #[must_use]
    pub const fn event_type(&self) -> HapticEventType {
        self.event_type
    }

    /// Sets the event type.
    pub fn set_event_type(&mut self, event_type: HapticEventType) {
        self.event_type = event_type;
    }

    /// Returns the event start time in seconds.
    #[must_use]
    pub const fn relative_time(&self) -> f64 {
        self.relative_time
    }

    /// Sets the event start time in seconds.
    pub fn set_relative_time(&mut self, relative_time: f64) {
        self.relative_time = relative_time;
    }

    /// Returns the optional event duration in seconds.
    #[must_use]
    pub const fn duration(&self) -> Option<f64> {
        self.duration
    }

    /// Sets the optional event duration in seconds.
    pub fn set_duration(&mut self, duration: Option<f64>) {
        self.duration = duration;
    }

    /// Returns the event parameters.
    #[must_use]
    pub fn parameters(&self) -> &[HapticEventParameter] {
        &self.parameters
    }

    /// Replaces the event parameters.
    pub fn set_parameters(&mut self, parameters: Vec<HapticEventParameter>) {
        self.parameters = parameters;
    }

    /// Returns the optional custom audio resource identifier.
    #[must_use]
    pub const fn audio_resource_id(&self) -> Option<AudioResourceId> {
        self.audio_resource_id
    }

    /// Sets the optional custom audio resource identifier.
    pub fn set_audio_resource_id(&mut self, audio_resource_id: Option<AudioResourceId>) {
        self.audio_resource_id = audio_resource_id;
    }

    /// Creates a transient haptic event.
    #[must_use]
    pub const fn haptic_transient(
        relative_time: f64,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self::new(
            HapticEventType::HapticTransient,
            relative_time,
            None,
            parameters,
        )
    }

    /// Creates a continuous haptic event.
    #[must_use]
    pub const fn haptic_continuous(
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

    /// Creates a continuous audio event.
    #[must_use]
    pub const fn audio_continuous(
        relative_time: f64,
        duration: f64,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self::new(
            HapticEventType::AudioContinuous,
            relative_time,
            Some(duration),
            parameters,
        )
    }

    /// Creates a custom audio event.
    #[must_use]
    pub const fn audio_custom(
        audio_resource_id: AudioResourceId,
        relative_time: f64,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self::with_audio_resource(audio_resource_id, relative_time, None, parameters)
    }

    /// Creates a custom audio event with an explicit duration.
    #[must_use]
    pub const fn audio_custom_with_duration(
        audio_resource_id: AudioResourceId,
        relative_time: f64,
        duration: f64,
        parameters: Vec<HapticEventParameter>,
    ) -> Self {
        Self::with_audio_resource(audio_resource_id, relative_time, Some(duration), parameters)
    }
}
