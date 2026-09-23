#![cfg(feature = "async")]

use corehaptics::async_api::AsyncHapticEngine;
use corehaptics::prelude::*;

const fn assert_send<T: Send>(_: &T) {}

fn muted_engine() -> Option<HapticEngine> {
    let capability = DeviceCapability::current().expect("failed to query capability");
    match HapticEngine::new() {
        Ok(engine) => {
            assert!(capability.supports_haptics());
            engine.set_muted_for_haptics(true);
            engine.set_muted_for_audio(true);
            Some(engine)
        }
        Err(error) => {
            assert!(!capability.supports_haptics());
            assert_eq!(
                error.haptic_error_code(),
                Some(HapticErrorCode::NotSupported)
            );
            None
        }
    }
}

#[test]
fn async_engine_start_and_stop() {
    let Some(engine) = muted_engine() else {
        return;
    };
    let start = AsyncHapticEngine::start(&engine);
    assert_send(&start);
    assert!(pollster::block_on(start).is_ok());

    let stop = AsyncHapticEngine::stop(&engine);
    assert_send(&stop);
    assert!(pollster::block_on(stop).is_ok());
}

#[test]
fn async_engine_restarts_repeatedly() {
    let Some(engine) = muted_engine() else {
        return;
    };
    for _ in 0..3 {
        assert!(pollster::block_on(AsyncHapticEngine::start(&engine)).is_ok());
        assert!(pollster::block_on(AsyncHapticEngine::stop(&engine)).is_ok());
    }
}

#[test]
fn async_notify_players_finished_resolves_after_playback() {
    let Some(engine) = muted_engine() else {
        return;
    };
    assert!(pollster::block_on(AsyncHapticEngine::start(&engine)).is_ok());

    let pattern = HapticPattern::new(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.05,
            vec![HapticEventParameter::haptic_intensity(0.5)],
        )],
        &[],
    )
    .expect("failed to create pattern");
    let player = engine
        .create_player(&pattern)
        .expect("failed to create player");
    player.set_muted(true);
    assert!(player.start_immediately().is_ok());

    let finished = AsyncHapticEngine::notify_when_players_finished(&engine);
    assert_send(&finished);
    assert!(pollster::block_on(finished).is_ok());
    assert!(pollster::block_on(AsyncHapticEngine::stop(&engine)).is_ok());
}
