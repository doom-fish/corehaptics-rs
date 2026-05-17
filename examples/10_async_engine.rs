/// Example: Async engine operations using the `async_api` module.
///
/// This demonstrates how to use the async API to start and stop
/// a haptic engine asynchronously.
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use corehaptics::prelude::*;
    use corehaptics::async_api::AsyncHapticEngine;

    pollster::block_on(async {
        let capability = DeviceCapability::current()?;
        if !capability.supports_haptics() {
            println!("async engine unavailable on this hardware; skipping");
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

        // Keep the engine running for a short time
        std::thread::sleep(std::time::Duration::from_millis(100));

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
    eprintln!("Run with: cargo run --example 10_async_engine --features async");
    std::process::exit(1);
}
