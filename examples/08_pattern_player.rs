use corehaptics::prelude::*;

fn main() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    if !capability.supports_haptics() {
        println!("pattern player unavailable on this hardware; skipping");
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_muted_for_haptics(true);
    engine.start()?;

    let pattern = HapticPattern::new(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.2,
            vec![HapticEventParameter::haptic_intensity(0.5)],
        )],
        &[],
    )?;
    let player = engine.create_player(&pattern)?;
    player.set_muted(true);
    player.start_immediately()?;
    player.send_parameters_immediately(&[DynamicParameter::haptic_intensity_control(0.8, 0.0)])?;
    player.schedule_parameter_curve_immediately(&ParameterCurve::new(
        DynamicParameterId::HapticIntensityControl,
        vec![
            ParameterCurveControlPoint::new(0.0, 0.2),
            ParameterCurveControlPoint::new(0.05, 0.7),
        ],
        0.0,
    ))?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    player.stop_immediately()?;
    engine.stop()?;
    println!(
        "pattern player example complete; muted={}",
        player.is_muted()
    );
    Ok(())
}
