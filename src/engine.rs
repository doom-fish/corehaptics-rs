//! `CHHapticEngine` wrapper.

#![allow(clippy::missing_errors_doc)]

use core::ffi::c_void;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    path::Path,
    ptr::NonNull,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

use crate::{
    advanced_player::AdvancedPatternPlayer,
    error::CoreHapticsError,
    object::{bool_result, error_from_raw, path_c_string, RetainedObject},
    pattern::HapticPattern,
    player::PatternPlayer,
};

/// `CHHapticTimeImmediate`.
pub const HAPTIC_TIME_IMMEDIATE: f64 = 0.0;

/// Keys used by `CHHapticEngine.registerAudioResource(_:options:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioResourceKey {
    UseVolumeEnvelope,
    LoopEnabled,
}

impl AudioResourceKey {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseVolumeEnvelope => "UseVolumeEnvelope",
            Self::LoopEnabled => "LoopEnabled",
        }
    }
}

/// Options for `register_audio_resource`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioResourceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_volume_envelope: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loop_enabled: Option<bool>,
}

impl AudioResourceOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            use_volume_envelope: None,
            loop_enabled: None,
        }
    }

    #[must_use]
    pub const fn with_use_volume_envelope(mut self, enabled: bool) -> Self {
        self.use_volume_envelope = Some(enabled);
        self
    }

    #[must_use]
    pub const fn with_loop_enabled(mut self, enabled: bool) -> Self {
        self.loop_enabled = Some(enabled);
        self
    }

    #[must_use]
    pub const fn use_volume_envelope(&self) -> Option<bool> {
        self.use_volume_envelope
    }

    #[must_use]
    pub const fn loop_enabled(&self) -> Option<bool> {
        self.loop_enabled
    }
}

/// Actions returned by `notify_when_players_finished`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineFinishedAction {
    StopEngine,
    LeaveEngineRunning,
}

impl EngineFinishedAction {
    const fn as_raw(self) -> i32 {
        match self {
            Self::StopEngine => 1,
            Self::LeaveEngineRunning => 2,
        }
    }
}

/// Stop reasons delivered to the engine stopped handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineStoppedReason {
    AudioSessionInterrupt,
    ApplicationSuspended,
    IdleTimeout,
    NotifyWhenFinished,
    EngineDestroyed,
    GameControllerDisconnect,
    SystemError,
    Unknown(i32),
}

impl EngineStoppedReason {
    const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::AudioSessionInterrupt,
            2 => Self::ApplicationSuspended,
            3 => Self::IdleTimeout,
            4 => Self::NotifyWhenFinished,
            5 => Self::EngineDestroyed,
            6 => Self::GameControllerDisconnect,
            -1 => Self::SystemError,
            other => Self::Unknown(other),
        }
    }
}

type StoppedHandlerFn = dyn Fn(EngineStoppedReason) + Send + Sync + 'static;
type ResetHandlerFn = dyn Fn() + Send + Sync + 'static;
type FinishedHandlerFn =
    dyn Fn(Option<CoreHapticsError>) -> EngineFinishedAction + Send + Sync + 'static;
type CompletionHandlerFn = dyn FnOnce(Option<CoreHapticsError>) + Send + 'static;
type CompletionRegistrar = unsafe extern "C" fn(
    crate::ffi::Object,
    crate::ffi::EngineCompletionHandler,
    *mut c_void,
    crate::ffi::ContextDrop,
);

struct StoppedHandlerContext {
    callback: Box<StoppedHandlerFn>,
}

struct ResetHandlerContext {
    callback: Box<ResetHandlerFn>,
}

struct FinishedHandlerContext {
    callback: Box<FinishedHandlerFn>,
}

struct CompletionHandlerContext {
    operation: &'static str,
    callback: Mutex<Option<Box<CompletionHandlerFn>>>,
}

unsafe extern "C" fn release_stopped_handler_context(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<StoppedHandlerContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

unsafe extern "C" fn release_reset_handler_context(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<ResetHandlerContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

unsafe extern "C" fn release_finished_handler_context(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<FinishedHandlerContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

unsafe extern "C" fn release_completion_handler_context(context: *mut c_void) {
    if let Some(context) = NonNull::new(context.cast::<CompletionHandlerContext>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

unsafe extern "C" fn stopped_handler_trampoline(context: *mut c_void, reason: i32) {
    let Some(context) = NonNull::new(context.cast::<StoppedHandlerContext>()) else {
        return;
    };
    let state = unsafe { context.as_ref() };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        (state.callback)(EngineStoppedReason::from_raw(reason));
    }));
}

unsafe extern "C" fn reset_handler_trampoline(context: *mut c_void) {
    let Some(context) = NonNull::new(context.cast::<ResetHandlerContext>()) else {
        return;
    };
    let state = unsafe { context.as_ref() };
    let _ = catch_unwind(AssertUnwindSafe(|| (state.callback)()));
}

unsafe extern "C" fn finished_handler_trampoline(
    context: *mut c_void,
    error: crate::ffi::Object,
) -> i32 {
    let Some(context) = NonNull::new(context.cast::<FinishedHandlerContext>()) else {
        return EngineFinishedAction::LeaveEngineRunning.as_raw();
    };
    let state = unsafe { context.as_ref() };
    let error = if error.is_null() {
        None
    } else {
        Some(unsafe { error_from_raw("CHHapticEngine.notifyWhenPlayersFinished", error) })
    };
    catch_unwind(AssertUnwindSafe(|| (state.callback)(error))).map_or_else(
        |_| EngineFinishedAction::LeaveEngineRunning.as_raw(),
        EngineFinishedAction::as_raw,
    )
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
        Some(unsafe { error_from_raw(state.operation, error) })
    };
    let mut callback = match state.callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callback.take() {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(error)));
    }
}

#[derive(Debug, Clone)]
pub struct HapticEngine {
    obj: RetainedObject,
}

impl HapticEngine {
    /// Create a new `CHHapticEngine`.
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

    fn with_completion_handler<F>(
        &self,
        operation: &'static str,
        registrar: CompletionRegistrar,
        handler: F,
    ) where
        F: FnOnce(Option<CoreHapticsError>) + Send + 'static,
    {
        let context = Box::new(CompletionHandlerContext {
            operation,
            callback: Mutex::new(Some(Box::new(handler))),
        });
        unsafe {
            registrar(
                self.as_raw(),
                Some(completion_handler_trampoline),
                Box::into_raw(context).cast(),
                Some(release_completion_handler_context),
            );
        }
    }

    pub fn start(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_engine_start(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticEngine.start") }
    }

    pub fn start_with_completion_handler<F>(&self, handler: F)
    where
        F: FnOnce(Option<CoreHapticsError>) + Send + 'static,
    {
        self.with_completion_handler(
            "CHHapticEngine.startWithCompletionHandler",
            crate::ffi::chrs_engine_start_with_completion_handler,
            handler,
        );
    }

    pub fn start_async<F>(&self, handler: F)
    where
        F: FnOnce(Option<CoreHapticsError>) + Send + 'static,
    {
        self.start_with_completion_handler(handler);
    }

    pub fn stop(&self) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe { crate::ffi::chrs_engine_stop(self.as_raw(), &mut error) };
        unsafe { bool_result(ok, error, "CHHapticEngine.stop") }
    }

    pub fn stop_with_completion_handler<F>(&self, handler: F)
    where
        F: FnOnce(Option<CoreHapticsError>) + Send + 'static,
    {
        self.with_completion_handler(
            "CHHapticEngine.stopWithCompletionHandler",
            crate::ffi::chrs_engine_stop_with_completion_handler,
            handler,
        );
    }

    pub fn stop_async<F>(&self, handler: F)
    where
        F: FnOnce(Option<CoreHapticsError>) + Send + 'static,
    {
        self.stop_with_completion_handler(handler);
    }

    #[must_use]
    pub fn current_time(&self) -> f64 {
        unsafe { crate::ffi::chrs_engine_current_time(self.as_raw()) }
    }

    #[must_use]
    pub fn plays_haptics_only(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_plays_haptics_only(self.as_raw()) }
    }

    pub fn set_plays_haptics_only(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_plays_haptics_only(self.as_raw(), enabled) };
    }

    #[must_use]
    pub fn plays_audio_only(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_plays_audio_only(self.as_raw()) }
    }

    pub fn set_plays_audio_only(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_plays_audio_only(self.as_raw(), enabled) };
    }

    #[must_use]
    pub fn is_muted_for_audio(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_is_muted_for_audio(self.as_raw()) }
    }

    pub fn set_muted_for_audio(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_muted_for_audio(self.as_raw(), enabled) };
    }

    #[must_use]
    pub fn is_muted_for_haptics(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_is_muted_for_haptics(self.as_raw()) }
    }

    pub fn set_muted_for_haptics(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_muted_for_haptics(self.as_raw(), enabled) };
    }

    #[must_use]
    pub fn auto_shutdown_enabled(&self) -> bool {
        unsafe { crate::ffi::chrs_engine_auto_shutdown_enabled(self.as_raw()) }
    }

    pub fn set_auto_shutdown_enabled(&self, enabled: bool) {
        unsafe { crate::ffi::chrs_engine_set_auto_shutdown_enabled(self.as_raw(), enabled) };
    }

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

    pub fn create_advanced_player(
        &self,
        pattern: &HapticPattern,
    ) -> crate::Result<AdvancedPatternPlayer> {
        let mut error = core::ptr::null_mut();
        let raw = unsafe {
            crate::ffi::chrs_engine_create_advanced_player(
                self.as_raw(),
                pattern.as_raw(),
                &mut error,
            )
        };
        if raw.is_null() {
            if error.is_null() {
                return Err(crate::error::CoreHapticsError::UnexpectedNull(
                    "CHHapticEngine.createAdvancedPlayer",
                ));
            }
            return Err(unsafe { error_from_raw("CHHapticEngine.createAdvancedPlayer", error) });
        }
        let Some(player) = (unsafe { AdvancedPatternPlayer::from_owned_raw(raw) }) else {
            return Err(crate::error::CoreHapticsError::UnexpectedNull(
                "CHHapticEngine.createAdvancedPlayer",
            ));
        };
        Ok(player)
    }

    pub fn register_audio_resource(
        &self,
        path: impl AsRef<Path>,
        options: &AudioResourceOptions,
    ) -> crate::Result<crate::AudioResourceId> {
        let path = path_c_string(path.as_ref())?;
        let options = serde_json::to_string(options)?;
        let options = crate::object::c_string(&options)?;
        let mut resource_id = 0_u64;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_engine_register_audio_resource(
                self.as_raw(),
                path.as_ptr(),
                options.as_ptr(),
                &mut resource_id,
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "CHHapticEngine.registerAudioResource")? };
        Ok(resource_id)
    }

    pub fn unregister_audio_resource(
        &self,
        resource_id: crate::AudioResourceId,
    ) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_engine_unregister_audio_resource(
                self.as_raw(),
                resource_id,
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "CHHapticEngine.unregisterAudioResource") }
    }

    pub fn play_pattern_from_file(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        let path = path_c_string(path.as_ref())?;
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_engine_play_pattern_from_url(self.as_raw(), path.as_ptr(), &mut error)
        };
        unsafe { bool_result(ok, error, "CHHapticEngine.playPattern(from: URL)") }
    }

    pub fn play_pattern_from_data(&self, data: &[u8]) -> crate::Result<()> {
        let mut error = core::ptr::null_mut();
        let ok = unsafe {
            crate::ffi::chrs_engine_play_pattern_from_data(
                self.as_raw(),
                data.as_ptr(),
                data.len(),
                &mut error,
            )
        };
        unsafe { bool_result(ok, error, "CHHapticEngine.playPattern(from: Data)") }
    }

    pub fn set_stopped_handler<F>(&self, handler: F)
    where
        F: Fn(EngineStoppedReason) + Send + Sync + 'static,
    {
        let context = Box::new(StoppedHandlerContext {
            callback: Box::new(handler),
        });
        unsafe {
            crate::ffi::chrs_engine_set_stopped_handler(
                self.as_raw(),
                Some(stopped_handler_trampoline),
                Box::into_raw(context).cast(),
                Some(release_stopped_handler_context),
            );
        }
    }

    pub fn clear_stopped_handler(&self) {
        unsafe { crate::ffi::chrs_engine_clear_stopped_handler(self.as_raw()) };
    }

    pub fn set_reset_handler<F>(&self, handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let context = Box::new(ResetHandlerContext {
            callback: Box::new(handler),
        });
        unsafe {
            crate::ffi::chrs_engine_set_reset_handler(
                self.as_raw(),
                Some(reset_handler_trampoline),
                Box::into_raw(context).cast(),
                Some(release_reset_handler_context),
            );
        }
    }

    pub fn clear_reset_handler(&self) {
        unsafe { crate::ffi::chrs_engine_clear_reset_handler(self.as_raw()) };
    }

    pub fn notify_when_players_finished<F>(&self, handler: F)
    where
        F: Fn(Option<CoreHapticsError>) -> EngineFinishedAction + Send + Sync + 'static,
    {
        let context = Box::new(FinishedHandlerContext {
            callback: Box::new(handler),
        });
        unsafe {
            crate::ffi::chrs_engine_notify_when_players_finished(
                self.as_raw(),
                Some(finished_handler_trampoline),
                Box::into_raw(context).cast(),
                Some(release_finished_handler_context),
            );
        }
    }
}
