//! Raw FFI declarations for the Swift `CoreHaptics` bridge.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type Object = *mut c_void;

extern "C" {
    pub fn chrs_object_retain(obj: Object) -> Object;
    pub fn chrs_object_release(obj: Object);
    pub fn chrs_string_free(ptr: *mut c_char);

    pub fn chrs_error_code(error: Object) -> isize;
    pub fn chrs_error_domain(error: Object) -> *mut c_char;
    pub fn chrs_error_description(error: Object) -> *mut c_char;

    pub fn chrs_capabilities_for_hardware() -> Object;
    pub fn chrs_capability_supports_haptics(capability: Object) -> bool;
    pub fn chrs_capability_supports_audio(capability: Object) -> bool;

    pub fn chrs_engine_create(error_out: *mut Object) -> Object;
    pub fn chrs_engine_start(engine: Object, error_out: *mut Object) -> bool;
    pub fn chrs_engine_stop(engine: Object, error_out: *mut Object) -> bool;
    pub fn chrs_engine_current_time(engine: Object) -> f64;
    pub fn chrs_engine_auto_shutdown_enabled(engine: Object) -> bool;
    pub fn chrs_engine_set_auto_shutdown_enabled(engine: Object, enabled: bool);
    pub fn chrs_engine_create_player(engine: Object, pattern: Object, error_out: *mut Object)
        -> Object;

    pub fn chrs_pattern_create(pattern_json: *const c_char, error_out: *mut Object) -> Object;
    pub fn chrs_pattern_duration(pattern: Object) -> f64;

    pub fn chrs_player_start(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_player_stop(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_player_cancel(player: Object, error_out: *mut Object) -> bool;
}
