use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use corehaptics::prelude::*;

fn main() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    if !capability.supports_haptics() {
        println!("advanced player unavailable on this hardware; skipping");
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_muted_for_haptics(true);
    engine.start()?;

    let pattern = HapticPattern::new(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.12,
            vec![HapticEventParameter::haptic_intensity(0.4)],
        )],
        &[],
    )?;
    let player = engine.create_advanced_player(&pattern)?;
    player.set_muted(true);
    player.set_loop_enabled(false);
    player.set_loop_end(pattern.duration());
    player.set_playback_rate(1.0);

    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();
    player.set_completion_handler(move |error| {
        if let Some(error) = error {
            eprintln!("advanced player completion error: {error}");
        }
        completed_clone.store(true, Ordering::SeqCst);
    });

    player.start_immediately()?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    player.clear_completion_handler();
    engine.stop()?;

    println!(
        "advanced player rate {:.1}, loop_end {:.3}, completion_called={}",
        player.playback_rate(),
        player.loop_end(),
        completed.load(Ordering::SeqCst)
    );
    Ok(())
}
