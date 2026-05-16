use core::ffi::{c_char, c_void};

pub type Object = *mut c_void;
pub type ContextDrop = Option<unsafe extern "C" fn(*mut c_void)>;

unsafe extern "C" {
    pub fn chrs_object_retain(obj: Object) -> Object;
    pub fn chrs_object_release(obj: Object);
    pub fn chrs_string_free(ptr: *mut c_char);

    pub fn chrs_error_code(error: Object) -> isize;
    pub fn chrs_error_domain(error: Object) -> *mut c_char;
    pub fn chrs_error_description(error: Object) -> *mut c_char;
}
