use core::ffi::c_char;

use super::Object;

unsafe extern "C" {
    pub fn chrs_player_start(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_player_stop(player: Object, time: f64, error_out: *mut Object) -> bool;
    pub fn chrs_player_cancel(player: Object, error_out: *mut Object) -> bool;
    pub fn chrs_player_send_parameters(
        player: Object,
        parameters_json: *const c_char,
        time: f64,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_player_schedule_parameter_curve(
        player: Object,
        parameter_curve_json: *const c_char,
        time: f64,
        error_out: *mut Object,
    ) -> bool;
    pub fn chrs_player_is_muted(player: Object) -> bool;
    pub fn chrs_player_set_muted(player: Object, muted: bool);
}
