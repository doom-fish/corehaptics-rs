use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::Duration,
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

    let (start_tx, start_rx) = mpsc::channel();
    engine.start_async(move |error| {
        let success = error.is_none();
        if let Some(error) = error {
            eprintln!("start_async error: {error}");
        }
        let _ = start_tx.send(success);
    });
    let start_ok = start_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("engine start completion should arrive");

    let (stop_tx, stop_rx) = mpsc::channel();
    engine.stop_async(move |error| {
        let success = error.is_none();
        if let Some(error) = error {
            eprintln!("stop_async error: {error}");
        }
        let _ = stop_tx.send(success);
    });
    let stop_ok = stop_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("engine stop completion should arrive");

    std::thread::sleep(Duration::from_millis(50));
    println!(
        "plays_haptics_only={} current_time={:.3} notified={} start_ok={} stop_ok={}",
        engine.plays_haptics_only(),
        engine.current_time(),
        notified.load(Ordering::SeqCst),
        start_ok,
        stop_ok,
    );
    Ok(())
}
