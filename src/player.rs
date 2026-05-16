//! `CHHapticPatternPlayer` wrapper.

#![allow(clippy::missing_errors_doc)]

use crate::{
    dynamic_parameter::DynamicParameter,
    object::{bool_result, c_string, RetainedObject},
    parameter_curve::ParameterCurve,
};

#[derive(Debug, Clone)]
pub struct PatternPlayer {
    obj: RetainedObject,
}

impl PatternPlayer {
    /// # Safety
    ///
    /// `raw` must be an owned `CHHapticPatternPlayer` bridge handle.
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        unsafe { RetainedObject::from_owned_raw(raw) }.map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    /// Start the player immediately.
    pub fn start_immediately(&self) -> crate::Result<()> {
        self.start_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    /// Start the player at the specified engine time.
    pub fn start_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_start(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.start") }
    }

    /// Stop the player immediately.
    pub fn stop_immediately(&self) -> crate::Result<()> {
        self.stop_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    /// Stop the player at the specified engine time.
    pub fn stop_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_stop(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.stop") }
    }

    /// Cancel the player and clear any queued commands.
    pub fn cancel(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_cancel(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.cancel") }
    }

    /// Send dynamic parameters at the specified engine time.
    pub fn send_parameters(&self, parameters: &[DynamicParameter], time: f64) -> crate::Result<()> {
        let parameters = serde_json::to_string(parameters)?;
        let parameters = c_string(&parameters)?;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_player_send_parameters(
                self.as_raw(),
                parameters.as_ptr(),
                time,
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.sendParameters") }
    }

    /// Send dynamic parameters as soon as possible.
    pub fn send_parameters_immediately(
        &self,
        parameters: &[DynamicParameter],
    ) -> crate::Result<()> {
        self.send_parameters(parameters, crate::HAPTIC_TIME_IMMEDIATE)
    }

    /// Schedule a parameter curve at the specified engine time.
    pub fn schedule_parameter_curve(
        &self,
        parameter_curve: &ParameterCurve,
        time: f64,
    ) -> crate::Result<()> {
        let parameter_curve = serde_json::to_string(parameter_curve)?;
        let parameter_curve = c_string(&parameter_curve)?;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_player_schedule_parameter_curve(
                self.as_raw(),
                parameter_curve.as_ptr(),
                time,
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.scheduleParameterCurve") }
    }

    /// Schedule a parameter curve as soon as possible.
    pub fn schedule_parameter_curve_immediately(
        &self,
        parameter_curve: &ParameterCurve,
    ) -> crate::Result<()> {
        self.schedule_parameter_curve(parameter_curve, crate::HAPTIC_TIME_IMMEDIATE)
    }

    #[must_use]
    pub fn is_muted(&self) -> bool {
        unsafe { crate::ffi::chrs_player_is_muted(self.as_raw()) }
    }

    pub fn set_muted(&self, muted: bool) {
        unsafe { crate::ffi::chrs_player_set_muted(self.as_raw(), muted) };
    }
}
