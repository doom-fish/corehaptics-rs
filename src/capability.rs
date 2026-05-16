//! `CHHapticDeviceCapability` wrapper.

#![allow(clippy::missing_errors_doc)]

use crate::{
    dynamic_parameter::DynamicParameterId,
    event::HapticEventType,
    event_parameter::HapticParameterId,
    object::{bool_result, c_string, RetainedObject},
};

/// Attribute ranges for a haptic event or dynamic parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterAttributes {
    min: f32,
    max: f32,
    default: f32,
}

impl ParameterAttributes {
    #[must_use]
    pub const fn new(min_value: f32, max_value: f32, default_value: f32) -> Self {
        Self {
            min: min_value,
            max: max_value,
            default: default_value,
        }
    }

    #[must_use]
    pub const fn min_value(&self) -> f32 {
        self.min
    }

    #[must_use]
    pub const fn max_value(&self) -> f32 {
        self.max
    }

    #[must_use]
    pub const fn default_value(&self) -> f32 {
        self.default
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCapability {
    obj: RetainedObject,
}

impl DeviceCapability {
    /// Return the current device capability snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error if `CoreHaptics` unexpectedly returns `nil`.
    pub fn current() -> crate::Result<Self> {
        unsafe {
            Self::from_owned_raw(crate::ffi::chrs_capabilities_for_hardware()).ok_or(
                crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticEngine.capabilitiesForHardware",
                ),
            )
        }
    }

    /// # Safety
    ///
    /// `raw` must be an owned `CHHapticDeviceCapability` bridge handle.
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        unsafe { RetainedObject::from_owned_raw(raw) }.map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn supports_haptics(&self) -> bool {
        unsafe { crate::ffi::chrs_capability_supports_haptics(self.as_raw()) }
    }

    #[must_use]
    pub fn supports_audio(&self) -> bool {
        unsafe { crate::ffi::chrs_capability_supports_audio(self.as_raw()) }
    }

    /// Query attributes for an event parameter / event-type pair.
    pub fn event_parameter_attributes(
        &self,
        parameter_id: HapticParameterId,
        event_type: HapticEventType,
    ) -> crate::Result<ParameterAttributes> {
        let parameter_id = c_string(parameter_id.as_str())?;
        let event_type = c_string(event_type.as_str())?;
        let mut min_value = 0.0;
        let mut max_value = 0.0;
        let mut default_value = 0.0;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_capability_event_parameter_attributes(
                self.as_raw(),
                parameter_id.as_ptr(),
                event_type.as_ptr(),
                &mut min_value,
                &mut max_value,
                &mut default_value,
                &mut error,
            )
        };
        unsafe {
            bool_result(
                ok,
                error,
                "CHHapticDeviceCapability.attributes(forEventParameter:eventType:)",
            )?;
        };
        Ok(ParameterAttributes::new(
            min_value,
            max_value,
            default_value,
        ))
    }

    /// Query attributes for a dynamic parameter.
    pub fn dynamic_parameter_attributes(
        &self,
        parameter_id: DynamicParameterId,
    ) -> crate::Result<ParameterAttributes> {
        let parameter_id = c_string(parameter_id.as_str())?;
        let mut min_value = 0.0;
        let mut max_value = 0.0;
        let mut default_value = 0.0;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_capability_dynamic_parameter_attributes(
                self.as_raw(),
                parameter_id.as_ptr(),
                &mut min_value,
                &mut max_value,
                &mut default_value,
                &mut error,
            )
        };
        unsafe {
            bool_result(
                ok,
                error,
                "CHHapticDeviceCapability.attributes(forDynamicParameter:)",
            )?;
        };
        Ok(ParameterAttributes::new(
            min_value,
            max_value,
            default_value,
        ))
    }
}
