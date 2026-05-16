mod common;

use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::Duration,
};

use common::{fixture_path, supports_audio, supports_haptics};
use corehaptics::prelude::*;

#[test]
fn engine_creation_reflects_hardware_support() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    match HapticEngine::new() {
        Ok(engine) => {
            assert!(capability.supports_haptics());
            engine.stop().ok();
        }
        Err(error) => {
            assert!(!capability.supports_haptics());
            assert_eq!(
                error.haptic_error_code(),
                Some(HapticErrorCode::NotSupported)
            );
        }
    }
    Ok(())
}

#[test]
fn engine_properties_callbacks_and_pattern_playback_work() -> corehaptics::Result<()> {
    if !supports_haptics()? {
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    engine.set_plays_haptics_only(true);
    engine.set_muted_for_haptics(true);
    engine.set_muted_for_audio(true);
    engine.set_auto_shutdown_enabled(true);
    assert!(engine.plays_haptics_only());
    assert!(engine.is_muted_for_haptics());
    assert!(engine.is_muted_for_audio());
    assert!(engine.auto_shutdown_enabled());

    let notified = Arc::new(AtomicBool::new(false));
    let notified_clone = notified.clone();
    engine.notify_when_players_finished(move |error| {
        assert!(error.is_none());
        notified_clone.store(true, Ordering::SeqCst);
        EngineFinishedAction::LeaveEngineRunning
    });
    std::thread::sleep(Duration::from_millis(100));
    assert!(notified.load(Ordering::SeqCst));

    let ahap_bytes = fs::read(fixture_path("minimal.ahap"))?;
    engine.start()?;
    engine.play_pattern_from_data(&ahap_bytes)?;
    engine.play_pattern_from_file(fixture_path("minimal.ahap"))?;
    engine.stop()?;
    Ok(())
}

#[test]
fn engine_start_and_stop_completion_handlers_report_success() -> corehaptics::Result<()> {
    if !supports_haptics()? {
        return Ok(());
    }

    let engine = HapticEngine::new()?;

    let (start_tx, start_rx) = mpsc::channel();
    engine.start_with_completion_handler(move |error| {
        let _ = start_tx.send(error.map_or(Ok(()), |error| Err(error.to_string())));
    });
    assert_eq!(
        start_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("engine start completion should arrive"),
        Ok(())
    );

    let (stop_tx, stop_rx) = mpsc::channel();
    engine.stop_with_completion_handler(move |error| {
        let _ = stop_tx.send(error.map_or(Ok(()), |error| Err(error.to_string())));
    });
    assert_eq!(
        stop_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("engine stop completion should arrive"),
        Ok(())
    );
    Ok(())
}

#[test]
fn engine_registers_audio_resources_when_audio_is_supported() -> corehaptics::Result<()> {
    if !supports_haptics()? || !supports_audio()? {
        return Ok(());
    }

    let engine = HapticEngine::new()?;
    let resource_id = engine.register_audio_resource(
        fixture_path("silence.wav"),
        &AudioResourceOptions::new().with_use_volume_envelope(true),
    )?;
    assert!(resource_id > 0);
    engine.unregister_audio_resource(resource_id)?;
    Ok(())
}
