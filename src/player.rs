//! `CHHapticPatternPlayer` wrapper.

use crate::object::{bool_result, RetainedObject};

#[derive(Debug, Clone)]
pub struct PatternPlayer {
    obj: RetainedObject,
}

impl PatternPlayer {
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        RetainedObject::from_owned_raw(raw).map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    /// Start the player immediately.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreHaptics`.
    pub fn start_immediately(&self) -> crate::Result<()> {
        self.start_at_time(0.0)
    }

    /// Start the player at the specified engine time.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreHaptics`.
    pub fn start_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_start(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.start") }
    }

    /// Stop the player immediately.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreHaptics`.
    pub fn stop_immediately(&self) -> crate::Result<()> {
        self.stop_at_time(0.0)
    }

    /// Stop the player at the specified engine time.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreHaptics`.
    pub fn stop_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_stop(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.stop") }
    }

    /// Cancel the player and clear any queued commands.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` reported by `CoreHaptics`.
    pub fn cancel(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_cancel(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.cancel") }
    }
}
