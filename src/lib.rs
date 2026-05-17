#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `CoreHaptics` framework on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod object;

pub mod advanced_player;
pub mod capability;
pub mod dynamic_parameter;
pub mod engine;
pub mod error;
pub mod event;
pub mod event_parameter;
pub mod ffi;
pub mod parameter_curve;
pub mod pattern;
pub mod player;
pub mod types;

#[cfg(feature = "async")]
pub mod async_api;

pub use advanced_player::AdvancedPatternPlayer;
pub use capability::{DeviceCapability, ParameterAttributes};
pub use dynamic_parameter::{DynamicParameter, DynamicParameterId};
pub use engine::{
    AudioResourceKey, AudioResourceOptions, EngineFinishedAction, EngineStoppedReason,
    HapticEngine, HAPTIC_TIME_IMMEDIATE,
};
pub use error::{CoreHapticsError, HapticErrorCode, Result, CORE_HAPTICS_ERROR_DOMAIN};
pub use event::{AudioResourceId, HapticEvent, HapticEventType};
pub use event_parameter::{HapticEventParameter, HapticParameterId};
pub use parameter_curve::{ParameterCurve, ParameterCurveControlPoint};
pub use pattern::{HapticPattern, PatternKey};
pub use player::PatternPlayer;

/// Common imports.
pub mod prelude {
    pub use crate::advanced_player::AdvancedPatternPlayer;
    pub use crate::capability::{DeviceCapability, ParameterAttributes};
    pub use crate::dynamic_parameter::{DynamicParameter, DynamicParameterId};
    pub use crate::engine::{
        AudioResourceKey, AudioResourceOptions, EngineFinishedAction, EngineStoppedReason,
        HapticEngine, HAPTIC_TIME_IMMEDIATE,
    };
    pub use crate::error::{CoreHapticsError, HapticErrorCode, Result, CORE_HAPTICS_ERROR_DOMAIN};
    pub use crate::event::{AudioResourceId, HapticEvent, HapticEventType};
    pub use crate::event_parameter::{HapticEventParameter, HapticParameterId};
    pub use crate::parameter_curve::{ParameterCurve, ParameterCurveControlPoint};
    pub use crate::pattern::{HapticPattern, PatternKey};
    pub use crate::player::PatternPlayer;
}
