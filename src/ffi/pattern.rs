use core::ffi::c_char;

use super::Object;

unsafe extern "C" {
    pub fn chrs_pattern_create(pattern_json: *const c_char, error_out: *mut Object) -> Object;
    pub fn chrs_pattern_create_from_dictionary_json(
        pattern_json: *const c_char,
        error_out: *mut Object,
    ) -> Object;
    pub fn chrs_pattern_create_from_ahap_file(
        path: *const c_char,
        error_out: *mut Object,
    ) -> Object;
    pub fn chrs_pattern_export_dictionary_json(
        pattern: Object,
        error_out: *mut Object,
    ) -> *mut c_char;
    pub fn chrs_pattern_duration(pattern: Object) -> f64;
}
