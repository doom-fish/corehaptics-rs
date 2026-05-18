//! Raw FFI declarations for the Swift `CoreHaptics` bridge.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod advanced_player;
mod capability;
mod core;
mod engine;
mod pattern;
mod player;

/// Raw advanced-player bridge symbols.
pub use advanced_player::*;
/// Raw capability bridge symbols.
pub use capability::*;
/// Raw core bridge symbols.
pub use core::*;
/// Raw engine bridge symbols.
pub use engine::*;
/// Raw pattern bridge symbols.
pub use pattern::*;
/// Raw player bridge symbols.
pub use player::*;
