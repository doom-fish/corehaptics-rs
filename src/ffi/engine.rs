use core::ffi::{c_char, c_void};

use super::{ContextDrop, Object};

pub type EngineStoppedHandler = Option<unsafe extern "C" fn(*mut c_void, i32)>;
pub type EngineResetHandler = Option<unsafe extern "C" fn(*mut c_void)>;
pub type EngineFinishedHandler = Option<unsafe extern "C" fn(*mut c_void, Object) -> i32>;
pub type EngineCompletionHandler = Option<unsafe extern "C" fn(*mut c_void, Object)>;

unsafe extern "C" {
    pub fn chrs_engine_create(error_out: *mut Object) -> Object;
    pub fn chrs_engine_start(engine: Object, error_out: *mut Object) -> bool;
    pub fn chrs_engine_start_with_completion_handler(
        engine: Object,
        callback: EngineCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    pub fn chrs_engine_stop(engine: Object, error_out: *mut Object) -> bool;
    pub fn chrs_engine_stop_with_completion_handler(
        engine: Object,
        callback: EngineCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    pub fn chrs_engine_current_time(engine: Object) -> f64;

    pub fn chrs_engine_plays_haptics_only(engine: Object) -> bool;
    pub fn chrs_engine_set_plays_haptics_only(engine: Object, enabled: bool);
    pub fn chrs_engine_plays_audio_only(engine: Object) -> bool;
    pub fn chrs_engine_set_plays_audio_only(engine: Object, enabled: bool);
    pub fn chrs_engine_is_muted_for_audio(engine: Object) -> bool;
    pub fn chrs_engine_set_muted_for_audio(engine: Object, enabled: bool);
    pub fn chrs_engine_is_muted_for_haptics(engine: Object) -> bool;
    pub fn chrs_engine_set_muted_for_haptics(engine: Object, enabled: bool);
    pub fn chrs_engine_auto_shutdown_enabled(engine: Object) -> bool;
    pub fn chrs_engine_set_auto_shutdown_enabled(engine: Object, enabled: bool);

    pub fn chrs_engine_create_player(
        engine: Object,
        pattern: Object,
        error_out: *mut Object,
    ) -> Object;
    pub fn chrs_engine_create_advanced_player(
        engine: Object,
        pattern: Object,
        error_out: *mut Object,
    ) -> Object;

    pub fn chrs_engine_register_audio_resource(
        engine: Object,
        path: *const c_char,
        options_json: *const c_char,
        out_resource_id: *mut u64,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_engine_unregister_audio_resource(
        engine: Object,
        resource_id: u64,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_engine_play_pattern_from_url(
        engine: Object,
        path: *const c_char,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_engine_play_pattern_from_data(
        engine: Object,
        bytes: *const u8,
        length: usize,
        error_out: *mut Object,
    ) -> bool;

    pub fn chrs_engine_set_stopped_handler(
        engine: Object,
        callback: EngineStoppedHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    pub fn chrs_engine_clear_stopped_handler(engine: Object);
    pub fn chrs_engine_set_reset_handler(
        engine: Object,
        callback: EngineResetHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    pub fn chrs_engine_clear_reset_handler(engine: Object);
    pub fn chrs_engine_notify_when_players_finished(
        engine: Object,
        callback: EngineFinishedHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
}
