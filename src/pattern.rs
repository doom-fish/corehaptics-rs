//! `CHHapticPattern` wrapper and JSON-backed pattern construction.

use crate::{object::{c_string, error_from_raw, RetainedObject}, types::{DynamicParameter, HapticEvent}};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct HapticPattern {
    obj: RetainedObject,
}

const fn slice_is_empty<T>(slice: &[T]) -> bool {
    slice.is_empty()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PatternEnvelope<'a> {
    events: &'a [HapticEvent],
    #[serde(default, skip_serializing_if = "slice_is_empty")]
    dynamic_parameters: &'a [DynamicParameter],
}

impl HapticPattern {
    /// Create a native `CHHapticPattern` from Rust event and dynamic-parameter definitions.
    ///
    /// # Errors
    ///
    /// Returns serialization errors or any `NSError` reported by the Swift bridge.
    pub fn new(events: &[HapticEvent], dynamic_parameters: &[DynamicParameter]) -> crate::Result<Self> {
        let envelope = PatternEnvelope {
            events,
            dynamic_parameters,
        };
        let json = serde_json::to_string(&envelope)?;
        let json = c_string(&json)?;
        let mut error = core::ptr::null_mut();
        let raw = unsafe { crate::ffi::chrs_pattern_create(json.as_ptr(), &mut error) };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticPattern init",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticPattern init", error) });
        }
        let Some(obj) = (unsafe { RetainedObject::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticPattern init",
            ));
        };
        Ok(Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { crate::ffi::chrs_pattern_duration(self.as_raw()) }
    }
}
