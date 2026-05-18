# Changelog

## [0.3.3] - 2026-05-18

- Completed a public API doc pass across non-generated `src/` modules and raised rustdoc coverage for non-generated source to 100.0% (360/360 items).

## [0.3.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## 0.3.1

- Fixed async callback safety: wrapped `engine_start_callback`, `engine_stop_callback`, and `notify_players_finished_callback` with `catch_user_panic` to prevent UB if closure panics across the FFI boundary.
- Added explicit SAFETY comments to all unsafe blocks in async callbacks and async API functions.

## 0.3.0

- Added `async_api` module (gated behind `async` feature) with `AsyncHapticEngine` for `CHHapticEngine.start()`, `CHHapticEngine.stop()`, and `CHHapticEngine.notifyWhenPlayersFinished()`.
- Added `EngineFuture` and `NotifyPlayersFinishedFuture` types for Tier 1 async operations.
- Added two async examples (`10_async_engine.rs`, `11_async_players.rs`) and async integration tests.
- Async API uses `doom-fish-utils` completion pattern for runtime-agnostic futures.

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
