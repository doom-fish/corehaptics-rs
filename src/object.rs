//! Internal helpers for retained bridge objects and `NSError` conversion.

use crate::{error::CoreHapticsError, ffi};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct RetainedObject {
    raw: ffi::Object,
}

impl RetainedObject {
    pub unsafe fn from_owned_raw(raw: ffi::Object) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub const fn as_raw(&self) -> ffi::Object {
        self.raw
    }
}

impl Clone for RetainedObject {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::chrs_object_retain(self.raw) };
        Self { raw }
    }
}

impl Drop for RetainedObject {
    fn drop(&mut self) {
        unsafe { ffi::chrs_object_release(self.raw) };
    }
}

pub unsafe fn take_c_string(raw: *mut c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    let value = CStr::from_ptr(raw).to_string_lossy().into_owned();
    ffi::chrs_string_free(raw);
    Some(value)
}

pub unsafe fn error_from_raw(
    operation: &'static str,
    error: ffi::Object,
) -> CoreHapticsError {
    let code = ffi::chrs_error_code(error);
    let domain = take_c_string(ffi::chrs_error_domain(error))
        .unwrap_or_else(|| "NSCocoaErrorDomain".to_owned());
    let description = take_c_string(ffi::chrs_error_description(error))
        .unwrap_or_else(|| operation.to_owned());
    ffi::chrs_object_release(error);
    CoreHapticsError::ObjectiveCError {
        operation,
        code,
        domain,
        description,
    }
}

pub unsafe fn bool_result(
    ok: bool,
    error: ffi::Object,
    operation: &'static str,
) -> crate::Result<()> {
    if ok {
        if !error.is_null() {
            ffi::chrs_object_release(error);
        }
        return Ok(());
    }
    if error.is_null() {
        return Err(CoreHapticsError::OperationFailed(operation));
    }
    Err(error_from_raw(operation, error))
}

pub fn c_string(value: &str) -> crate::Result<CString> {
    CString::new(value).map_err(|_| {
        CoreHapticsError::InvalidArgument(format!("string contains interior NUL byte: {value:?}"))
    })
}
