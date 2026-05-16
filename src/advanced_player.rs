//! `CHHapticAdvancedPatternPlayer` wrapper.

#![allow(clippy::missing_errors_doc)]

use core::ffi::c_void;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
};

use crate::{
    dynamic_parameter::DynamicParameter,
    error::CoreHapticsError,
    object::{bool_result, c_string, error_from_raw, RetainedObject},
    parameter_curve::ParameterCurve,
};

#[derive(Debug, Clone)]
pub struct AdvancedPatternPlayer {
    obj: RetainedObject,
}

type CompletionHandlerFn = dyn Fn(Option<CoreHapticsError>) + Send + Sync + 'static;

struct CompletionHandlerContext {
    callback: Box<CompletionHandlerFn>,
}

unsafe extern "C" fn release_completion_handler_context(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<CompletionHandlerContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

unsafe extern "C" fn completion_handler_trampoline(
    context: *mut c_void,
    error: crate::ffi::Object,
) {
    let Some(context) = NonNull::new(context.cast::<CompletionHandlerContext>()) else {
        return;
    };
    let state = unsafe { context.as_ref() };
    let error = if error.is_null() {
        None
    } else {
        Some(unsafe { error_from_raw("CHHapticAdvancedPatternPlayer.completionHandler", error) })
    };
    let _ = catch_unwind(AssertUnwindSafe(|| (state.callback)(error)));
}

impl AdvancedPatternPlayer {
    /// # Safety
    ///
    /// `raw` must be an owned `CHHapticAdvancedPatternPlayer` bridge handle.
    pub(crate) unsafe fn from_owned_raw(raw: crate::ffi::Object) -> Option<Self> {
        unsafe { RetainedObject::from_owned_raw(raw) }.map(|obj| Self { obj })
    }

    pub(crate) const fn as_raw(&self) -> crate::ffi::Object {
        self.obj.as_raw()
    }

    pub fn start_immediately(&self) -> crate::Result<()> {
        self.start_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    pub fn start_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_start(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.start") }
    }

    pub fn stop_immediately(&self) -> crate::Result<()> {
        self.stop_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    pub fn stop_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_stop(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.stop") }
    }

    pub fn cancel(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_player_cancel(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticPatternPlayer.cancel") }
    }

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

    pub fn send_parameters_immediately(
        &self,
        parameters: &[DynamicParameter],
    ) -> crate::Result<()> {
        self.send_parameters(parameters, crate::HAPTIC_TIME_IMMEDIATE)
    }

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

    pub fn pause_immediately(&self) -> crate::Result<()> {
        self.pause_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    pub fn pause_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_advanced_player_pause(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticAdvancedPatternPlayer.pause") }
    }

    pub fn resume_immediately(&self) -> crate::Result<()> {
        self.resume_at_time(crate::HAPTIC_TIME_IMMEDIATE)
    }

    pub fn resume_at_time(&self, time: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok =
            unsafe { crate::ffi::chrs_advanced_player_resume(self.as_raw(), time, &mut error) };
        unsafe { bool_result(ok, error, "CHHapticAdvancedPatternPlayer.resume") }
    }

    pub fn seek_to_offset(&self, offset: f64) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_advanced_player_seek_to_offset(self.as_raw(), offset, &mut error)
        };
        unsafe { bool_result(ok, error, "CHHapticAdvancedPatternPlayer.seekToOffset") }
    }

    #[must_use]
    pub fn loop_enabled(&self) -> bool {
        unsafe { crate::ffi::chrs_advanced_player_loop_enabled(self.as_raw()) }
    }

    pub fn set_loop_enabled(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_advanced_player_set_loop_enabled(self.as_raw(), enabled) };
    }

    #[must_use]
    pub fn loop_end(&self) -> f64 {
        unsafe { crate::ffi::chrs_advanced_player_loop_end(self.as_raw()) }
    }

    pub fn set_loop_end(&self, loop_end: f64) {
        unsafe { crate::ffi::chrs_advanced_player_set_loop_end(self.as_raw(), loop_end) };
    }

    #[must_use]
    pub fn playback_rate(&self) -> f32 {
        unsafe { crate::ffi::chrs_advanced_player_playback_rate(self.as_raw()) }
    }

    pub fn set_playback_rate(&self, playback_rate: f32) {
        unsafe { crate::ffi::chrs_advanced_player_set_playback_rate(self.as_raw(), playback_rate) };
    }

    pub fn set_completion_handler<F>(&self, handler: F)
    where
        F: Fn(Option<CoreHapticsError>) + Send + Sync + 'static,
    {
        let context = Box::new(CompletionHandlerContext {
            callback: Box::new(handler),
        });
        unsafe {
            crate::ffi::chrs_advanced_player_set_completion_handler(
                self.as_raw(),
                Some(completion_handler_trampoline),
                Box::into_raw(context).cast(),
                Some(release_completion_handler_context),
            );
        }
    }

    pub fn clear_completion_handler(&self) {
        unsafe { crate::ffi::chrs_advanced_player_clear_completion_handler(self.as_raw()) };
    }
}
