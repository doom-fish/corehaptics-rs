use core::ffi::{c_char, c_void};

/// An opaque retained object pointer returned by the Swift bridge.
pub type Object = *mut c_void;
/// An optional callback that drops a bridge context pointer.
pub type ContextDrop = Option<unsafe extern "C" fn(*mut c_void)>;

unsafe extern "C" {
    /// Retains a bridge object and returns the retained handle.
    pub fn chrs_object_retain(obj: Object) -> Object;
    /// Releases a bridge object handle.
    pub fn chrs_object_release(obj: Object);
    /// Frees a heap-allocated C string returned by the bridge.
    pub fn chrs_string_free(ptr: *mut c_char);

    /// Returns the numeric code from an `NSError` bridge object.
    pub fn chrs_error_code(error: Object) -> isize;
    /// Returns the domain string from an `NSError` bridge object.
    pub fn chrs_error_domain(error: Object) -> *mut c_char;
    /// Returns the localized description from an `NSError` bridge object.
    pub fn chrs_error_description(error: Object) -> *mut c_char;
}
