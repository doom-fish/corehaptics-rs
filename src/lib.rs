#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `CoreHaptics` framework on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod object;

pub mod capability;
pub mod engine;
pub mod error;
pub mod ffi;
pub mod pattern;
pub mod player;
pub mod types;

pub use capability::DeviceCapability;
pub use engine::HapticEngine;
pub use error::{CoreHapticsError, Result};
pub use pattern::HapticPattern;
pub use player::PatternPlayer;
pub use types::{
    DynamicParameter, DynamicParameterId, HapticEvent, HapticEventParameter, HapticEventType,
    HapticParameterId,
};

/// Common imports.
pub mod prelude {
    pub use crate::capability::DeviceCapability;
    pub use crate::engine::HapticEngine;
    pub use crate::error::{CoreHapticsError, Result};
    pub use crate::pattern::HapticPattern;
    pub use crate::player::PatternPlayer;
    pub use crate::types::{
        DynamicParameter, DynamicParameterId, HapticEvent, HapticEventParameter,
        HapticEventType, HapticParameterId,
    };
}
