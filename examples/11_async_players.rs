/// Example: Waiting for players to finish asynchronously.
///
/// This demonstrates how to use the `async_api` to wait for pattern
/// players to finish playing patterns.
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use corehaptics::prelude::*;
    use corehaptics::async_api::AsyncHapticEngine;

    pollster::block_on(async {
        let capability = DeviceCapability::current()?;
        if !capability.supports_haptics() {
            println!("async players unavailable on this hardware; skipping");
            return Ok(());
        }

        // Create a haptic engine
        let engine = HapticEngine::new()?;
        engine.set_muted_for_haptics(true);
        engine.set_muted_for_audio(true);

        // Start the engine asynchronously
        println!("Starting engine...");
        AsyncHapticEngine::start(&engine).await?;
        println!("Engine started!");

        // Create a simple haptic pattern
        let pattern = HapticPattern::new(
            &[HapticEvent::haptic_continuous(
                0.0,
                0.1,
                vec![HapticEventParameter::haptic_intensity(0.5)],
            )],
            &[],
        )?;

        // Create a player and play the pattern
        let player = engine.create_player(&pattern)?;
        player.set_muted(true);
        player.start_immediately()?;

        // Wait for all players to finish
        println!("Waiting for players to finish...");
        AsyncHapticEngine::notify_when_players_finished(&engine).await?;
        println!("All players finished!");

        // Stop the engine asynchronously
        println!("Stopping engine...");
        AsyncHapticEngine::stop(&engine).await?;
        println!("Engine stopped!");

        Ok(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the 'async' feature to be enabled.");
    eprintln!("Run with: cargo run --example 11_async_players --features async");
    std::process::exit(1);
}
