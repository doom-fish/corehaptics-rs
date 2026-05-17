#![cfg(feature = "async")]
/// Tests for the `async_api` module.
use corehaptics::prelude::*;
use corehaptics::async_api::AsyncHapticEngine;

#[test]
fn test_async_engine_start_and_stop() {
    pollster::block_on(async {
        let capability = DeviceCapability::current().expect("failed to query capability");
        if !capability.supports_haptics() {
            println!("skipping test on non-haptic hardware");
            return;
        }

        let engine = HapticEngine::new().expect("failed to create engine");
        engine.set_muted_for_haptics(true);
        
        // Test starting the engine
        AsyncHapticEngine::start(&engine)
            .await
            .expect("failed to start engine");

        // Give it a moment to actually start
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Test stopping the engine
        AsyncHapticEngine::stop(&engine)
            .await
            .expect("failed to stop engine");
    });
}

#[test]
fn test_async_engine_multiple_starts_and_stops() {
    pollster::block_on(async {
        let capability = DeviceCapability::current().expect("failed to query capability");
        if !capability.supports_haptics() {
            println!("skipping test on non-haptic hardware");
            return;
        }

        let engine = HapticEngine::new().expect("failed to create engine");
        engine.set_muted_for_haptics(true);
        
        // Test multiple start/stop cycles
        for i in 0..3 {
            println!("Cycle {}", i + 1);
            
            AsyncHapticEngine::start(&engine)
                .await
                .expect("failed to start engine");

            std::thread::sleep(std::time::Duration::from_millis(10));

            AsyncHapticEngine::stop(&engine)
                .await
                .expect("failed to stop engine");
        }
    });
}

#[test]
fn test_async_notify_players_finished() {
    pollster::block_on(async {
        let capability = DeviceCapability::current().expect("failed to query capability");
        if !capability.supports_haptics() {
            println!("skipping test on non-haptic hardware");
            return;
        }

        let engine = HapticEngine::new().expect("failed to create engine");
        engine.set_muted_for_haptics(true);
        engine.set_muted_for_audio(true);
        
        AsyncHapticEngine::start(&engine)
            .await
            .expect("failed to start engine");

        // Create a simple pattern
        let pattern = HapticPattern::new(
            &[HapticEvent::haptic_continuous(
                0.0,
                0.05,
                vec![HapticEventParameter::haptic_intensity(0.5)],
            )],
            &[],
        ).expect("failed to create pattern");

        // Create player and play
        let player = engine.create_player(&pattern)
            .expect("failed to create player");
        player.set_muted(true);
        player.start_immediately()
            .expect("failed to start player");

        // Wait for player to finish
        AsyncHapticEngine::notify_when_players_finished(&engine)
            .await
            .expect("failed to wait for players");

        AsyncHapticEngine::stop(&engine)
            .await
            .expect("failed to stop engine");
    });
}
