//! `CHHapticDeviceCapability` wrapper.

use crate::object::RetainedObject;

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
            Self::from_owned_raw(crate::ffi::chrs_capabilities_for_hardware())
                .ok_or(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticEngine.capabilitiesForHardware",
                ))
        }
    }

    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
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
}
