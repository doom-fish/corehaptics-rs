# Changelog

## 0.2.1

- Added `HapticEngine::{start_with_completion_handler,start_async,stop_with_completion_handler,stop_async}` so `CHHapticCompletionHandler` is covered by safe Rust closures.
- Added engine lifecycle example/test coverage for async start/stop completion callbacks.
- Refreshed coverage docs to reflect full audited macOS coverage.

## 0.2.0

- Added safe wrappers for `CHHapticEvent`, `CHHapticEventParameter`, `CHHapticDynamicParameter`, and `CHHapticParameterCurve`.
- Added `CHHapticAdvancedPatternPlayer` coverage, completion callbacks, and additional `CHHapticPatternPlayer` controls.
- Extended `CHHapticEngine` with muting, callback handlers, audio-resource registration, AHAP playback helpers, and advanced-player creation.
- Added `DeviceCapability` parameter-attribute queries plus typed `CHHapticErrorCode` coverage.
- Added `COVERAGE.md`, nine numbered examples, fixture-backed integration tests, and refreshed documentation.

## 0.1.0

- Initial release.
- Safe wrappers for hardware capabilities, `CHHapticEngine`, `CHHapticPattern`, and `CHHapticPatternPlayer`.
- Strongly-typed Rust event/parameter builders serialized into a Swift bridge.
- Smoke example that queries capabilities, starts/stops an engine when supported, and creates a player without starting playback.
