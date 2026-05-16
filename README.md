# corehaptics

Safe Rust bindings for Apple's [CoreHaptics](https://developer.apple.com/documentation/corehaptics) framework on macOS — query hardware support, build patterns, start haptic engines, and create pattern players.

> **Status:** experimental. v0.1 ships hardware capability queries plus safe wrappers for `CHHapticEngine`, `CHHapticPattern`, and `CHHapticPatternPlayer`. The smoke example intentionally avoids starting a player, so it should not emit any user-visible haptic effect.

Swift bridge required — `CoreHaptics` is Objective-C / Swift-first and the safe Rust API is implemented on top of a static Swift helper library.

## Quick start

```rust,no_run
use corehaptics::prelude::*;

fn main() -> Result<()> {
    let capability = DeviceCapability::current()?;
    println!("supports haptics = {}", capability.supports_haptics());

    if capability.supports_haptics() {
        let engine = HapticEngine::new()?;
        engine.start()?;

        let pattern = HapticPattern::new(
            &[HapticEvent::haptic_transient(
                0.0,
                vec![HapticEventParameter::haptic_intensity(1.0)],
            )],
            &[],
        )?;
        let _player = engine.create_player(&pattern)?;
        engine.stop()?;
    }

    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

Example output (values vary by machine):

```text
supports haptics: false
supports audio: false
✅ corehaptics capability + engine OK
```

On Macs with supported hardware, the example also starts/stops an engine and prints the created pattern duration.

## Notes

- `DeviceCapability::current()` is a light-weight wrapper over `CHHapticEngine.capabilitiesForHardware()`.
- `HapticPattern::new` serializes Rust event / dynamic-parameter structs into JSON and lets the Swift bridge construct the native `CHHapticPattern` graph.
- `HapticEngine::stop()` uses a short `DispatchSemaphore` wait in the Swift bridge because the Objective-C API only exposes an asynchronous completion-handler variant.
- When `supports_haptics()` is `false`, engine creation/playback is skipped entirely; the smoke example still succeeds.
- The smoke example only creates the player; it does **not** call `PatternPlayer::start_*`.

## Roadmap

- [x] `DeviceCapability::{current, supports_haptics, supports_audio}`
- [x] `HapticPattern::new` + typed event / parameter builders
- [x] `HapticEngine::{new, start, stop, current_time, create_player}`
- [x] `PatternPlayer::{start, stop, cancel}`
- [ ] Advanced player wrapper (`CHHapticAdvancedPatternPlayer`)
- [ ] Engine stopped/reset callback closures
- [ ] Parameter-attribute queries from `CHHapticDeviceCapability`

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
