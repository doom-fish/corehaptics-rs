use core::ffi::c_void;

use super::{ContextDrop, Object};

/// Callback invoked when an advanced player finishes playback.
pub type AdvancedPlayerCompletionHandler = Option<unsafe extern "C" fn(*mut c_void, Object)>;

unsafe extern "C" {
    /// Pauses an advanced player at the specified engine time.
    pub fn chrs_advanced_player_pause(player: Object, time: f64, error_out: *mut Object) -> bool;
    /// Resumes an advanced player at the specified engine time.
    pub fn chrs_advanced_player_resume(player: Object, time: f64, error_out: *mut Object) -> bool;
    /// Seeks an advanced player to the specified offset.
    pub fn chrs_advanced_player_seek_to_offset(
        player: Object,
        offset: f64,
        error_out: *mut Object,
    ) -> bool;
    /// Returns whether looping is enabled.
    pub fn chrs_advanced_player_loop_enabled(player: Object) -> bool;
    /// Configures whether looping is enabled.
    pub fn chrs_advanced_player_set_loop_enabled(player: Object, enabled: bool);
    /// Returns the loop end time in seconds.
    pub fn chrs_advanced_player_loop_end(player: Object) -> f64;
    /// Sets the loop end time in seconds.
    pub fn chrs_advanced_player_set_loop_end(player: Object, loop_end: f64);
    /// Returns the playback rate multiplier.
    pub fn chrs_advanced_player_playback_rate(player: Object) -> f32;
    /// Sets the playback rate multiplier.
    pub fn chrs_advanced_player_set_playback_rate(player: Object, playback_rate: f32);
    /// Registers a completion handler for an advanced player.
    pub fn chrs_advanced_player_set_completion_handler(
        player: Object,
        callback: AdvancedPlayerCompletionHandler,
        context: *mut c_void,
        drop_context: ContextDrop,
    );
    /// Clears the advanced-player completion handler.
    pub fn chrs_advanced_player_clear_completion_handler(player: Object);
}
