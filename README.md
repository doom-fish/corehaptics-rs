# corehaptics

Safe Rust bindings for Apple's [CoreHaptics](https://developer.apple.com/documentation/corehaptics) framework on macOS 12 or later.

> **Most Macs have no haptics hardware.** On a Mac without internal haptics, `DeviceCapability::current()?.supports_haptics()` is `false` and `HapticEngine::new()` fails with `HapticErrorCode::NotSupported`. On macOS, haptics usually come from game controllers: create an engine for one with `HapticEngine::from_device_haptics` (see below).

The crate uses a static Swift bridge because `CoreHaptics` is Objective-C / Swift-first. The public Rust APIs are safe wrappers over that bridge, except `HapticEngine::from_device_haptics`, which takes a raw Objective-C pointer, and the raw `ffi` module.

## Installation

```toml
[dependencies]
corehaptics = "0.4"
```

## Highlights

- Query hardware support and parameter ranges with `DeviceCapability`
- Build `HapticEvent`, `HapticEventParameter`, `DynamicParameter`, and `ParameterCurve` value graphs in pure Rust
- Create patterns from typed values, AHAP dictionaries, or `.ahap` files
- Start engines synchronously or asynchronously, create normal / advanced players, send live parameters, schedule curves, and register audio resources
- Install Rust closures for engine start/stop, engine finished, and advanced-player completion callbacks
- **Async API**: Executor-agnostic futures for `CHHapticEngine.start()`, `stop()`, and `notifyWhenPlayersFinished()` (gated behind `async` feature)

## Quick start

```rust,no_run
use corehaptics::prelude::*;

fn main() -> Result<()> {
    let capability = DeviceCapability::current()?;
    println!("supports haptics = {}", capability.supports_haptics());

    if capability.supports_haptics() {
        let engine = HapticEngine::new()?;
        engine.set_muted_for_haptics(true);
        engine.start()?;

        let pattern = HapticPattern::new(
            &[HapticEvent::haptic_transient(
                0.0,
                vec![HapticEventParameter::haptic_intensity(1.0)],
            )],
            &[],
        )?;

        let player = engine.create_player(&pattern)?;
        player.set_muted(true);
        player.start_immediately()?;
        player.stop_immediately()?;
        engine.stop()?;
    }

    Ok(())
}
```

## Game controller haptics

`HapticEngine::from_device_haptics(device_haptics, locality)` wraps `GCDeviceHaptics.createEngine(withLocality:)`. `device_haptics` is a non-null pointer to a controller's `GCDeviceHaptics` object, for example `GCController.haptics` obtained through another `GameController` binding; it must point to a live `GCDeviceHaptics` for the duration of the call, which is why the function is `unsafe`. The call fails when the controller has no actuator for the requested `ControllerHapticsLocality`.

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 06_pattern
cargo run --example 09_advanced_pattern_player
cargo run --example 10_async_engine --features async
cargo run --example 11_async_players --features async
```

The numbered examples in `examples/` cover every logical area:

1. smoke / capability
2. event parameters
3. dynamic parameters
4. parameter curves
5. events
6. patterns + AHAP import/export
7. engine lifecycle + sync/async callbacks
8. pattern players
9. advanced pattern players
10. async engine operations (requires `async` feature)
11. async player operations (requires `async` feature)

## Async API

The `async` feature provides executor-agnostic futures for common engine operations:

```rust,no_run
use corehaptics::prelude::*;
use corehaptics::async_api::AsyncHapticEngine;

async fn example() -> Result<()> {
    let capability = DeviceCapability::current()?;
    if !capability.supports_haptics() {
        return Ok(());
    }
    let engine = HapticEngine::new()?;
    AsyncHapticEngine::start(&engine).await?;
    // ... play patterns ...
    AsyncHapticEngine::stop(&engine).await?;
    Ok(())
}
```

See [`async_api`](src/async_api.rs) for documentation on all available futures.

`AsyncHapticEngine::start` and `stop` run on Swift concurrency, whose runtime is part of macOS 12 and later. A binary that uses them must target macOS 12 or later (for example with `MACOSX_DEPLOYMENT_TARGET=12.0`) or have `/usr/lib/swift` as an rpath (`-Wl,-rpath,/usr/lib/swift`). Rust's default deployment target is older, and then the linker records `@rpath/libswift_Concurrency.dylib`, which dyld cannot load without that rpath.

## Notes

- `HapticEngine::new()` fails with `HapticErrorCode::NotSupported` on Macs without internal haptics hardware, which is most Macs.
- `HapticEngine::stop()` waits at most 5 seconds for `CoreHaptics` to confirm and returns `CoreHapticsError::Timeout` otherwise.
- An engine keeps one players-finished handler. Each `notify_when_players_finished` call, sync or async, replaces the previous one; a replaced `AsyncHapticEngine::notify_when_players_finished` future resolves with `CoreHapticsError::OperationFailed`.
- File paths passed to `HapticPattern::from_file`, `HapticEngine::play_pattern_from_file`, and `HapticEngine::register_audio_resource` must be valid UTF-8, because `CoreHaptics` takes Foundation file URLs, which cannot represent other byte sequences. Other paths are rejected with `CoreHapticsError::InvalidArgument` instead of being rewritten.
- Examples and tests mute haptic/audio output wherever possible so they remain headless-safe.
- `HapticPattern::from_file` uses `CHHapticPattern(contentsOf:)`, which requires macOS 13.0+ at runtime.
- `CHHapticEngine.initWithAudioSession` is intentionally omitted because `AVAudioSession` is unavailable on macOS.

## Coverage

See [`COVERAGE.md`](COVERAGE.md) for the audited framework-to-crate mapping.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
