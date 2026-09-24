# Changelog

All notable changes to `corehaptics` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Fixed

- The players-finished callback cleared the engine's stored handler box from the CoreHaptics queue while another thread could be replacing it, an unsynchronized swap of a strong reference. The swap is now locked, and a finished callback only clears the box it registered.
- `HapticEngine::stop()` waited for CoreHaptics without a timeout and wrote the result from the completion thread without synchronization. It now waits at most 5 seconds, returns `CoreHapticsError::Timeout` when CoreHaptics does not answer, and passes the result through lock-protected state.
- Paths that are not valid UTF-8 were silently rewritten with `to_string_lossy`, so `HapticPattern::from_file`, `HapticEngine::play_pattern_from_file` and `HapticEngine::register_audio_resource` could open a different file. Such paths are now rejected with `CoreHapticsError::InvalidArgument`; Foundation file URLs cannot represent them.
- The async API tests now assert on both the no-haptics path and the playback path.
- Handler contexts were dropped in `extern "C"` release callbacks without a panic guard, so a captured value whose destructor panics aborted the process. The contexts are now dropped inside `doom_fish_utils::panic_safe::catch_user_panic`, and the handler trampolines use the same helper instead of bare `catch_unwind`.
- `AsyncHapticEngine::notify_when_players_finished` registered its own handler directly with `CHHapticEngine`, which keeps a single players-finished handler. When a later registration replaced it, the handler was released without being called, so the future never resolved and its completion context leaked. The future now uses the same registration as `HapticEngine::notify_when_players_finished` and resolves with `CoreHapticsError::OperationFailed` when its handler is released before it runs.
- Audio events in `HapticPattern::new`, `HapticEngine::unregister_audio_resource` and `HapticEngine::register_audio_resource` converted resource IDs with Swift's trapping integer initializers, so a `HapticEvent::audio_custom` with an ID above `i64::MAX` crashed the process with `SIGTRAP`. `CHHapticAudioResourceID` is an `NSUInteger`, so the `u64` ID now crosses the bridge by bit pattern in both directions. The engine stopped reason is narrowed with clamping.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the link search path or rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's `libswift_Concurrency.tbd` for the whole binary, so the crate's own binaries linked the old copy and linking could break next to Swift bridges that use newer concurrency APIs.

### Changed

- `CoreHapticsError` has a new `Timeout` variant.
- The Swift bridge links `GameController`.
- Depends on `doom-fish-utils` `>=0.4.1, <0.5` for its panic guard, with or without the `async` feature.
- `rust-version` is now 1.82 (was 1.76).
- **Breaking:** `NotifyPlayersFinishedFuture` reports `CoreHaptics` errors as `CoreHapticsError::ObjectiveCError`, with the `NSError` code and domain, instead of `InvalidArgument` with the description.
- **Breaking:** the minimum macOS is 12 (was 10.15), and the Swift bridge targets macOS 12. `AsyncHapticEngine::start` and `stop` run on Swift concurrency, whose runtime ships with macOS from version 12; on macOS 10.15 and 11 they only worked where Xcode's back-deployment copy was on the rpath. A binary that uses them now links the concurrency runtime strongly, so it must target macOS 12 or later or have `/usr/lib/swift` as an rpath (see the README). The bridge's macOS 11 and 12 availability checks are gone.

### Added

- `HapticEngine::from_device_haptics` and `ControllerHapticsLocality`, which create an engine for a game controller from a raw `GCDeviceHaptics` pointer through `GCDeviceHaptics.createEngine(withLocality:)`.

### Removed

- The unused `CoreHapticsBridge.h` header, which no longer matched the bridge.
- **Breaking:** the raw `ffi::chrs_engine_notify_when_players_finished_async` bridge function.

## [0.3.4] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

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
