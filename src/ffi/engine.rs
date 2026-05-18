use core::ffi::{c_char, c_void};

use super::{ContextDrop, Object};

/// Callback invoked when the engine stops.
pub type EngineStoppedHandler = Option<unsafe extern "C" fn(*mut c_void, i32)>;
/// Callback invoked when the engine resets.
pub type EngineResetHandler = Option<unsafe extern "C" fn(*mut c_void)>;
/// Callback invoked when all players finish.
pub type EngineFinishedHandler = Option<unsafe extern "C" fn(*mut c_void, Object) -> i32>;
/// Callback invoked when an engine start or stop completes.
pub type EngineCompletionHandler = Option<unsafe extern "C" fn(*mut c_void, Object)>;

/// Callback used by async completion helpers.
pub type AsyncCompletionCallback = unsafe extern "C" fn(*const c_void, *const c_char, *mut c_void);

unsafe extern "C" {
    /// Creates a new engine instance.
    pub fn chrs_engine_create(error_out: *mut Object) -> Object;
    /// Starts the engine.
    pub fn chrs_engine_start(engine: Object, error_out: *mut Object) -> bool;
    /// Starts the engine with a completion callback.
    pub fn chrs_engine_start_with_completion_handler(
        engine: Object,
        callback: EngineCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    /// Stops the engine.
    pub fn chrs_engine_stop(engine: Object, error_out: *mut Object) -> bool;
    /// Stops the engine with a completion callback.
    pub fn chrs_engine_stop_with_completion_handler(
        engine: Object,
        callback: EngineCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    /// Returns the current engine time in seconds.
    pub fn chrs_engine_current_time(engine: Object) -> f64;

    /// Returns whether the engine plays only haptics.
    pub fn chrs_engine_plays_haptics_only(engine: Object) -> bool;
    /// Configures whether the engine plays only haptics.
    pub fn chrs_engine_set_plays_haptics_only(engine: Object, enabled: bool);
    /// Returns whether the engine plays only audio.
    pub fn chrs_engine_plays_audio_only(engine: Object) -> bool;
    /// Configures whether the engine plays only audio.
    pub fn chrs_engine_set_plays_audio_only(engine: Object, enabled: bool);
    /// Returns whether audio output is muted.
    pub fn chrs_engine_is_muted_for_audio(engine: Object) -> bool;
    /// Configures whether audio output is muted.
    pub fn chrs_engine_set_muted_for_audio(engine: Object, enabled: bool);
    /// Returns whether haptic output is muted.
    pub fn chrs_engine_is_muted_for_haptics(engine: Object) -> bool;
    /// Configures whether haptic output is muted.
    pub fn chrs_engine_set_muted_for_haptics(engine: Object, enabled: bool);
    /// Returns whether automatic shutdown is enabled.
    pub fn chrs_engine_auto_shutdown_enabled(engine: Object) -> bool;
    /// Configures whether automatic shutdown is enabled.
    pub fn chrs_engine_set_auto_shutdown_enabled(engine: Object, enabled: bool);

    /// Creates a basic pattern player.
    pub fn chrs_engine_create_player(
        engine: Object,
        pattern: Object,
        error_out: *mut Object,
    ) -> Object;
    /// Creates an advanced pattern player.
    pub fn chrs_engine_create_advanced_player(
        engine: Object,
        pattern: Object,
        error_out: *mut Object,
    ) -> Object;

    /// Registers an audio resource.
    pub fn chrs_engine_register_audio_resource(
        engine: Object,
        path: *const c_char,
        options_json: *const c_char,
        out_resource_id: *mut u64,
        error_out: *mut Object,
    ) -> bool;
    /// Unregisters a previously registered audio resource.
    pub fn chrs_engine_unregister_audio_resource(
        engine: Object,
        resource_id: u64,
        error_out: *mut Object,
    ) -> bool;
    /// Plays a pattern from a file URL.
    pub fn chrs_engine_play_pattern_from_url(
        engine: Object,
        path: *const c_char,
        error_out: *mut Object,
    ) -> bool;
    /// Plays a pattern from raw data.
    pub fn chrs_engine_play_pattern_from_data(
        engine: Object,
        bytes: *const u8,
        length: usize,
        error_out: *mut Object,
    ) -> bool;

    /// Registers a stopped handler.
    pub fn chrs_engine_set_stopped_handler(
        engine: Object,
        callback: EngineStoppedHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    /// Clears the stopped handler.
    pub fn chrs_engine_clear_stopped_handler(engine: Object);
    /// Registers a reset handler.
    pub fn chrs_engine_set_reset_handler(
        engine: Object,
        callback: EngineResetHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    /// Clears the reset handler.
    pub fn chrs_engine_clear_reset_handler(engine: Object);
    /// Registers a players-finished handler.
    pub fn chrs_engine_notify_when_players_finished(
        engine: Object,
        callback: EngineFinishedHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );

    /// Starts the engine with an async completion callback.
    pub fn chrs_engine_start_async(engine: Object, cb: AsyncCompletionCallback, ctx: *mut c_void);
    /// Stops the engine with an async completion callback.
    pub fn chrs_engine_stop_async(engine: Object, cb: AsyncCompletionCallback, ctx: *mut c_void);
    /// Waits for players to finish with an async completion callback.
    pub fn chrs_engine_notify_when_players_finished_async(
        engine: Object,
        cb: AsyncCompletionCallback,
        ctx: *mut c_void,
    );
}
