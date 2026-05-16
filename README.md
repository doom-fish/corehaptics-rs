# corehaptics

Safe Rust bindings for Apple's [CoreHaptics](https://developer.apple.com/documentation/corehaptics) framework on macOS.

> **Status:** v0.2.1 reaches full audited safe coverage for the macOS CoreHaptics surface, including async engine lifecycle callbacks.

The crate uses a static Swift bridge because `CoreHaptics` is Objective-C / Swift-first. All public Rust APIs are safe wrappers over that bridge.

## Highlights

- Query hardware support and parameter ranges with `DeviceCapability`
- Build `HapticEvent`, `HapticEventParameter`, `DynamicParameter`, and `ParameterCurve` value graphs in pure Rust
- Create patterns from typed values, AHAP dictionaries, or `.ahap` files
- Start engines synchronously or asynchronously, create normal / advanced players, send live parameters, schedule curves, and register audio resources
- Install Rust closures for engine start/stop, engine finished, and advanced-player completion callbacks

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

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 06_pattern
cargo run --example 09_advanced_pattern_player
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

## Notes

- `HapticEngine::new()` fails with `HapticErrorCode::NotSupported` on Macs without internal haptics hardware.
- Examples and tests mute haptic/audio output wherever possible so they remain headless-safe.
- `HapticPattern::from_file` uses `CHHapticPattern(contentsOf:)`, which requires macOS 13.0+ at runtime.
- `CHHapticEngine.initWithAudioSession` is intentionally omitted because `AVAudioSession` is unavailable on macOS.

## Coverage

See [`COVERAGE.md`](COVERAGE.md) for the audited framework-to-crate mapping.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
