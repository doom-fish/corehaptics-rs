use core::ffi::c_char;

use super::Object;

unsafe extern "C" {
    /// Starts a player at the specified engine time.
    pub fn chrs_player_start(player: Object, time: f64, error_out: *mut Object) -> bool;
    /// Stops a player at the specified engine time.
    pub fn chrs_player_stop(player: Object, time: f64, error_out: *mut Object) -> bool;
    /// Cancels a player and clears queued commands.
    pub fn chrs_player_cancel(player: Object, error_out: *mut Object) -> bool;
    /// Sends dynamic parameters to a player.
    pub fn chrs_player_send_parameters(
        player: Object,
        parameters_json: *const c_char,
        time: f64,
        error_out: *mut Object,
    ) -> bool;
    /// Schedules a parameter curve on a player.
    pub fn chrs_player_schedule_parameter_curve(
        player: Object,
        parameter_curve_json: *const c_char,
        time: f64,
        error_out: *mut Object,
    ) -> bool;
    /// Returns whether a player is muted.
    pub fn chrs_player_is_muted(player: Object) -> bool;
    /// Configures whether a player is muted.
    pub fn chrs_player_set_muted(player: Object, muted: bool);
}
