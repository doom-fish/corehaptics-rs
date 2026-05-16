use core::ffi::c_void;

use super::{ContextDrop, Object};

pub type AdvancedPlayerCompletionHandler = Option<unsafe extern "C" fn(*mut c_void, Object)>;

unsafe extern "C" {
    pub fn chrs_advanced_player_pause(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_advanced_player_resume(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_advanced_player_seek_to_offset(
        player: Object,
        offset: f64,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_advanced_player_loop_enabled(player: Object) -> bool;
    pub fn chrs_advanced_player_set_loop_enabled(player: Object, enabled: bool);
    pub fn chrs_advanced_player_loop_end(player: Object) -> f64;
    pub fn chrs_advanced_player_set_loop_end(player: Object, loop_end: f64);
    pub fn chrs_advanced_player_playback_rate(player: Object) -> f32;
    pub fn chrs_advanced_player_set_playback_rate(player: Object, playback_rate: f32);
    pub fn chrs_advanced_player_set_completion_handler(
        player: Object,
        callback: AdvancedPlayerCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    pub fn chrs_advanced_player_clear_completion_handler(player: Object);
}
