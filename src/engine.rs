//! `CHHapticEngine` wrapper.

use crate::{object::{bool_result, error_from_raw, RetainedObject}, pattern::HapticPattern, player::PatternPlayer};

#[derive(Debug, Clone)]
pub struct HapticEngine {
    obj: RetainedObject,
}

impl HapticEngine {
    /// Create a new `CHHapticEngine`.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` emitted by `CoreHaptics`.
    pub fn new() -> crate::Result<Self> {
        let mut error = core::ptr::null_mut();
        let raw = unsafe { crate::ffi::chrs_engine_create(&mut error) };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticEngine init",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticEngine init", error) });
        }
        let Some(obj) = (unsafe { RetainedObject::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticEngine init",
            ));
        };
        Ok(Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    /// Start the engine synchronously.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` emitted by `CoreHaptics`.
    pub fn start(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_engine_start(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticEngine.start") }
    }

    /// Stop the engine synchronously.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` emitted by `CoreHaptics`.
    pub fn stop(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_engine_stop(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticEngine.stop") }
    }

    #[must_use]
    pub fn current_time(&self) -> f64 {
        unsafe { crate::ffi::chrs_engine_current_time(self.as_raw()) }
    }

    #[must_use]
    pub fn auto_shutdown_enabled(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_auto_shutdown_enabled(self.as_raw()) }
    }

    pub fn set_auto_shutdown_enabled(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_auto_shutdown_enabled(self.as_raw(), enabled) };
    }

    /// Create a pattern player for the provided pattern.
    ///
    /// # Errors
    ///
    /// Returns any `NSError` emitted by `CoreHaptics`.
    pub fn create_player(&self, pattern: &HapticPattern) -> crate::Result<PatternPlayer> {
        let mut error = core::ptr::null_mut();
        let raw = unsafe {
            crate::ffi::chrs_engine_create_player(self.as_raw(), pattern.as_raw(), &mut error)
        };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticEngine.createPlayer",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticEngine.createPlayer", error) });
        }
        let Some(player) = (unsafe { PatternPlayer::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticEngine.createPlayer",
            ));
        };
        Ok(player)
    }
}
