mod common;

use std::time::Duration;

use common::{sample_continuous_pattern, sample_curve, supports_haptics};
use corehaptics::prelude::*;

#[test]
fn pattern_player_supports_commands_when_haptics_are_available() -> corehaptics::Result<()> {
    if !supports_haptics()? {
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_muted_for_haptics(true);
    engine.start()?;

    let player = engine.create_player(&sample_continuous_pattern()?)?;
    player.set_muted(true);
    assert!(player.is_muted());
    player.start_immediately()?;
    player.send_parameters_immediately(&[DynamicParameter::haptic_intensity_control(0.6, 0.0)])?;
    player.schedule_parameter_curve_immediately(&sample_curve())?;
    std::thread::sleep(Duration::from_millis(75));
    player.stop_immediately()?;
    player.cancel()?;
    engine.stop()?;
    Ok(())
}
