# CoreHaptics coverage audit (`corehaptics` v0.2.1)

Audited against:

- `CHHapticDeviceCapability.h`
- `CHHapticErrors.h`
- `CHHapticEngine.h`
- `CHHapticEvent.h`
- `CHHapticParameter.h`
- `CHHapticPattern.h`
- `CHHapticPatternPlayer.h`

Status legend:

- ✅ implemented
- ⏭️ skipped for macOS crate scope

## `CHHapticErrors.h`

| API | Status | Notes |
| --- | --- | --- |
| `CoreHapticsErrorDomain` | ✅ | `CORE_HAPTICS_ERROR_DOMAIN` |
| `CHHapticErrorCode` | ✅ | `HapticErrorCode` enum + error decoding |

## `CHHapticEvent.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticEventType*` constants | ✅ | `HapticEventType` |
| `CHHapticAudioResourceID` | ✅ | `AudioResourceId` |
| `CHHapticEvent` properties (`type`, `eventParameters`, `relativeTime`, `duration`) | ✅ | `HapticEvent` getters/setters |
| `initWithEventType:parameters:relativeTime:` | ✅ | `HapticEvent::new`, `haptic_transient` |
| `initWithEventType:parameters:relativeTime:duration:` | ✅ | `HapticEvent::new`, `haptic_continuous`, `audio_continuous` |
| `initWithAudioResourceID:parameters:relativeTime:` | ✅ | `HapticEvent::audio_custom` |
| `initWithAudioResourceID:parameters:relativeTime:duration:` | ✅ | `HapticEvent::audio_custom_with_duration` |

## `CHHapticParameter.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticEventParameterID*` constants | ✅ | `HapticParameterId` |
| `CHHapticDynamicParameterID*` constants | ✅ | `DynamicParameterId` |
| `CHHapticEventParameter` | ✅ | `HapticEventParameter` value type + builders |
| `CHHapticDynamicParameter` | ✅ | `DynamicParameter` value type + builders |
| `CHHapticParameterCurveControlPoint` | ✅ | `ParameterCurveControlPoint` |
| `CHHapticParameterCurve` | ✅ | `ParameterCurve` |

## `CHHapticPattern.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticPatternKey*` constants | ✅ | `PatternKey` |
| `CHHapticPattern.duration` | ✅ | `HapticPattern::duration` |
| `initWithEvents:parameters:error` | ✅ | `HapticPattern::new` |
| `initWithEvents:parameterCurves:error` | ✅ | `HapticPattern::with_parameter_curves` |
| `initWithDictionary:error` | ✅ | `HapticPattern::from_dictionary`, `from_dictionary_json` |
| `initWithContentsOfURL:error` | ✅ | `HapticPattern::from_file` |
| `exportDictionaryAndReturnError:` | ✅ | `HapticPattern::export_dictionary`, `export_dictionary_json` |

## `CHHapticPatternPlayer.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticPatternPlayer.startAtTime:error` | ✅ | `PatternPlayer::start_at_time`, `start_immediately` |
| `CHHapticPatternPlayer.stopAtTime:error` | ✅ | `PatternPlayer::stop_at_time`, `stop_immediately` |
| `sendParameters:atTime:error` | ✅ | `PatternPlayer::send_parameters`, `send_parameters_immediately` |
| `scheduleParameterCurve:atTime:error` | ✅ | `PatternPlayer::schedule_parameter_curve`, `schedule_parameter_curve_immediately` |
| `cancelAndReturnError:` | ✅ | `PatternPlayer::cancel` |
| `isMuted` | ✅ | `PatternPlayer::{is_muted,set_muted}` |
| `CHHapticAdvancedPatternPlayer.pauseAtTime:error` | ✅ | `AdvancedPatternPlayer::pause_at_time`, `pause_immediately` |
| `resumeAtTime:error` | ✅ | `AdvancedPatternPlayer::resume_at_time`, `resume_immediately` |
| `seekToOffset:error` | ✅ | `AdvancedPatternPlayer::seek_to_offset` |
| `loopEnabled` | ✅ | `AdvancedPatternPlayer::{loop_enabled,set_loop_enabled}` |
| `loopEnd` | ✅ | `AdvancedPatternPlayer::{loop_end,set_loop_end}` |
| `playbackRate` | ✅ | `AdvancedPatternPlayer::{playback_rate,set_playback_rate}` |
| `completionHandler` | ✅ | `AdvancedPatternPlayer::{set_completion_handler,clear_completion_handler}` |

## `CHHapticDeviceCapability.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticParameterAttributes` | ✅ | `ParameterAttributes` |
| `supportsHaptics` | ✅ | `DeviceCapability::supports_haptics` |
| `supportsAudio` | ✅ | `DeviceCapability::supports_audio` |
| `attributesForEventParameter:eventType:error` | ✅ | `DeviceCapability::event_parameter_attributes` |
| `attributesForDynamicParameter:error` | ✅ | `DeviceCapability::dynamic_parameter_attributes` |

## `CHHapticEngine.h`

| API | Status | Notes |
| --- | --- | --- |
| `CHHapticTimeImmediate` | ✅ | `HAPTIC_TIME_IMMEDIATE` |
| `capabilitiesForHardware` | ✅ | `DeviceCapability::current` |
| `currentTime` | ✅ | `HapticEngine::current_time` |
| `stoppedHandler` | ✅ | `HapticEngine::{set_stopped_handler,clear_stopped_handler}` |
| `resetHandler` | ✅ | `HapticEngine::{set_reset_handler,clear_reset_handler}` |
| `playsHapticsOnly` | ✅ | `HapticEngine::{plays_haptics_only,set_plays_haptics_only}` |
| `playsAudioOnly` | ✅ | `HapticEngine::{plays_audio_only,set_plays_audio_only}` |
| `isMutedForAudio` | ✅ | `HapticEngine::{is_muted_for_audio,set_muted_for_audio}` |
| `isMutedForHaptics` | ✅ | `HapticEngine::{is_muted_for_haptics,set_muted_for_haptics}` |
| `autoShutdownEnabled` | ✅ | `HapticEngine::{auto_shutdown_enabled,set_auto_shutdown_enabled}` |
| `initAndReturnError:` | ✅ | `HapticEngine::new` |
| `initWithAudioSession:error` | ⏭️ | `AVAudioSession` is unavailable on macOS |
| `CHHapticCompletionHandler` | ✅ | Rust closures via `HapticEngine::{start_with_completion_handler,start_async,stop_with_completion_handler,stop_async}` |
| `startWithCompletionHandler:` | ✅ | `HapticEngine::{start_with_completion_handler,start_async}` |
| `startAndReturnError:` | ✅ | `HapticEngine::start` |
| `stopWithCompletionHandler:` | ✅ | `HapticEngine::{stop_with_completion_handler,stop_async}` (plus blocking `HapticEngine::stop`) |
| `notifyWhenPlayersFinished:` | ✅ | `HapticEngine::notify_when_players_finished` |
| `createPlayerWithPattern:error` | ✅ | `HapticEngine::create_player` |
| `createAdvancedPlayerWithPattern:error` | ✅ | `HapticEngine::create_advanced_player` |
| `CHHapticAudioResourceKeyUseVolumeEnvelope` | ✅ | `AudioResourceKey::UseVolumeEnvelope`, `AudioResourceOptions` |
| `CHHapticAudioResourceKeyLoopEnabled` | ✅ | `AudioResourceKey::LoopEnabled`, `AudioResourceOptions` |
| `registerAudioResource:options:error` | ✅ | `HapticEngine::register_audio_resource` |
| `unregisterAudioResource:error` | ✅ | `HapticEngine::unregister_audio_resource` |
| `playPatternFromURL:error` | ✅ | `HapticEngine::play_pattern_from_file` |
| `playPatternFromData:error` | ✅ | `HapticEngine::play_pattern_from_data` |
| `intendedSpatialExperience` | ⏭️ | visionOS-only (`API_UNAVAILABLE(macos)`) |

## Deferred / skipped count

- 2 skipped APIs: `initWithAudioSession:error` (macOS-unavailable `AVAudioSession`) and `intendedSpatialExperience` (visionOS-only)
