use corehaptics::prelude::*;

fn main() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    println!("supports haptics: {}", capability.supports_haptics());
    println!("supports audio: {}", capability.supports_audio());

    if capability.supports_haptics() {
        let engine = HapticEngine::new()?;
        engine.start()?;

        let pattern = HapticPattern::new(
            &[HapticEvent::haptic_transient(
                0.0,
                vec![
                    HapticEventParameter::haptic_intensity(1.0),
                    HapticEventParameter::haptic_sharpness(0.5),
                ],
            )],
            &[],
        )?;
        let _player = engine.create_player(&pattern)?;
        println!("pattern duration: {:.1}s", pattern.duration());
        engine.stop()?;
    }

    println!("✅ corehaptics capability + engine OK");
    Ok(())
}
