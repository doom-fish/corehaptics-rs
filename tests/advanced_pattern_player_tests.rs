mod common;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use common::{sample_continuous_pattern, supports_haptics};
use corehaptics::prelude::*;

#[test]
fn advanced_pattern_player_supports_extended_controls() -> corehaptics::Result<()> {
    if !supports_haptics()? {
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_muted_for_haptics(true);
    engine.start()?;

    let pattern = sample_continuous_pattern()?;
    let player = engine.create_advanced_player(&pattern)?;
    player.set_muted(true);
    player.set_loop_enabled(true);
    player.set_loop_end(pattern.duration());
    player.set_playback_rate(1.25);
    assert!(player.loop_enabled());
    assert!((player.loop_end() - pattern.duration()).abs() < f64::EPSILON);
    assert!((player.playback_rate() - 1.25).abs() < f32::EPSILON);

    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();
    player.set_completion_handler(move |_| {
        completed_clone.store(true, Ordering::SeqCst);
    });

    player.start_immediately()?;
    player.pause_immediately()?;
    player.resume_immediately()?;
    player.seek_to_offset(0.0)?;
    player.set_loop_enabled(false);
    std::thread::sleep(Duration::from_millis(150));
    player.stop_immediately()?;
    player.clear_completion_handler();
    engine.stop()?;

    let _ = completed.load(Ordering::SeqCst);
    Ok(())
}
