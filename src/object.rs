//! Internal helpers for retained bridge objects and `NSError` conversion.

use crate::{
    error::{CoreHapticsError, HapticErrorCode, CORE_HAPTICS_ERROR_DOMAIN},
    ffi,
};
use core::ffi::c_char;
use std::{
    ffi::{CStr, CString},
    path::Path,
};

#[derive(Debug)]
pub struct RetainedObject {
    raw: ffi::Object,
}

impl RetainedObject {
    /// # Safety
    ///
    /// `raw` must be an owned `CoreHaptics` bridge handle returned by the Swift bridge.
    pub(crate) unsafe fn from_owned_raw(raw: ffi::Object) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub(crate) const fn as_raw(&self) -> ffi::Object {
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

/// # Safety
///
/// `raw` must point to a NUL-terminated string allocated by the Swift bridge.
pub unsafe fn take_c_string(raw: *mut c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::chrs_string_free(raw) };
    Some(value)
}

/// # Safety
///
/// `error` must be an owned `NSError` bridge handle returned by the Swift bridge.
pub unsafe fn error_from_raw(operation: &'static str, error: ffi::Object) -> CoreHapticsError {
    let code = unsafe { ffi::chrs_error_code(error) };
    let domain = unsafe { take_c_string(ffi::chrs_error_domain(error)) }
        .unwrap_or_else(|| "NSCocoaErrorDomain".to_owned());
    let description = unsafe { take_c_string(ffi::chrs_error_description(error)) }
        .unwrap_or_else(|| operation.to_owned());
    unsafe { ffi::chrs_object_release(error) };
    let haptic_error_code = if domain == CORE_HAPTICS_ERROR_DOMAIN {
        HapticErrorCode::from_code(code)
    } else {
        None
    };
    CoreHapticsError::ObjectiveCError {
        operation,
        code,
        domain,
        description,
        haptic_error_code,
    }
}

/// # Safety
///
/// `error` must either be null or an owned `NSError` bridge handle returned by the Swift bridge.
pub unsafe fn bool_result(
    ok: bool,
    error: ffi::Object,
    operation: &'static str,
) -> crate::Result<()> {
    if ok {
        if !error.is_null() {
            unsafe { ffi::chrs_object_release(error) };
        }
        return Ok(());
    }
    if error.is_null() {
        return Err(CoreHapticsError::OperationFailed(operation));
    }
    Err(unsafe { error_from_raw(operation, error) })
}

pub fn c_string(value: &str) -> crate::Result<CString> {
    CString::new(value).map_err(|_| {
        CoreHapticsError::InvalidArgument(format!("string contains interior NUL byte: {value:?}"))
    })
}

pub fn path_c_string(path: &Path) -> crate::Result<CString> {
    let Some(utf8) = path.to_str() else {
        return Err(CoreHapticsError::InvalidArgument(format!(
            "path is not valid UTF-8 and cannot be passed to a file URL: {}",
            path.display()
        )));
    };
    c_string(utf8)
}

#[cfg(test)]
mod tests {
    use super::path_c_string;
    use crate::CoreHapticsError;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    #[test]
    fn utf8_path_bytes_are_preserved() {
        let path = Path::new("patterns/haptic \u{e9}v\u{e9}nement.ahap");
        let converted = path_c_string(path).expect("UTF-8 path without NUL");
        assert_eq!(converted.as_bytes(), path.as_os_str().as_bytes());
    }

    #[test]
    fn non_utf8_paths_are_rejected_instead_of_rewritten() {
        let path = Path::new(OsStr::from_bytes(b"patterns/haptic-\xff.ahap"));
        assert!(matches!(
            path_c_string(path),
            Err(CoreHapticsError::InvalidArgument(_))
        ));
    }

    #[test]
    fn interior_nul_paths_are_rejected() {
        let path = Path::new(OsStr::from_bytes(b"bad\0path.ahap"));
        assert!(matches!(
            path_c_string(path),
            Err(CoreHapticsError::InvalidArgument(_))
        ));
    }
}
