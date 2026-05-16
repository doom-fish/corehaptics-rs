use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use corehaptics::prelude::*;

fn main() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    if !capability.supports_haptics() {
        println!("engine unavailable on this hardware; skipping");
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_plays_haptics_only(true);
    engine.set_muted_for_haptics(true);
    engine.set_muted_for_audio(true);
    engine.set_auto_shutdown_enabled(true);

    let notified = Arc::new(AtomicBool::new(false));
    let notified_clone = notified.clone();
    engine.notify_when_players_finished(move |error| {
        if let Some(error) = error {
            eprintln!("notify_when_players_finished error: {error}");
        }
        notified_clone.store(true, Ordering::SeqCst);
        EngineFinishedAction::LeaveEngineRunning
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    println!(
        "plays_haptics_only={} current_time={:.3} notified={}",
        engine.plays_haptics_only(),
        engine.current_time(),
        notified.load(Ordering::SeqCst)
    );
    Ok(())
}
