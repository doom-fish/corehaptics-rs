use core::ffi::c_char;

use super::Object;

unsafe extern "C" {
    /// Returns the current device capability object.
    pub fn chrs_capabilities_for_hardware() -> Object;
    /// Returns whether the device supports haptics.
    pub fn chrs_capability_supports_haptics(capability: Object) -> bool;
    /// Returns whether the device supports audio playback.
    pub fn chrs_capability_supports_audio(capability: Object) -> bool;
    /// Queries attributes for an event-parameter and event-type pair.
    pub fn chrs_capability_event_parameter_attributes(
        capability: Object,
        parameter_id: *const c_char,
        event_type: *const c_char,
        out_min: *mut f32,
        out_max: *mut f32,
        out_default: *mut f32,
        error_out: *mut Object,
    ) -> bool;
    /// Queries attributes for a dynamic parameter.
    pub fn chrs_capability_dynamic_parameter_attributes(
        capability: Object,
        parameter_id: *const c_char,
        out_min: *mut f32,
        out_max: *mut f32,
        out_default: *mut f32,
        error_out: *mut Object,
    ) -> bool;
}
