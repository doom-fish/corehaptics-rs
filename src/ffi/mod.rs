//! Raw FFI declarations for the Swift `CoreHaptics` bridge.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

mod advanced_player;
mod capability;
mod core;
mod engine;
mod pattern;
mod player;

pub use advanced_player::*;
pub use capability::*;
pub use core::*;
pub use engine::*;
pub use pattern::*;
pub use player::*;
