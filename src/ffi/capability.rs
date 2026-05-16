use core::ffi::c_char;

use super::Object;

unsafe extern "C" {
    pub fn chrs_capabilities_for_hardware() -> Object;
    pub fn chrs_capability_supports_haptics(capability: Object) -> bool;
    pub fn chrs_capability_supports_audio(capability: Object) -> bool;
    pub fn chrs_capability_event_parameter_attributes(
        capability: Object,
        parameter_id: *const c_char,
        event_type: *const c_char,
        out_min: *mut f32,
        out_max: *mut f32,
        out_default: *mut f32,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_capability_dynamic_parameter_attributes(
        capability: Object,
        parameter_id: *const c_char,
        out_min: *mut f32,
        out_max: *mut f32,
        out_default: *mut f32,
        error_out: *mut Object,
    ) -> bool;
}
