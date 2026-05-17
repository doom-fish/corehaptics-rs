# corehaptics-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 141
VERIFIED: 139
GAPS: 0
EXEMPT: 2
COVERAGE_PCT: 100

Audit methodology: Enumerated all public symbols from CoreHaptics.framework headers (CHHapticErrors.h, CHHapticEvent.h, CHHapticParameter.h, CHHapticPattern.h, CHHapticPatternPlayer.h, CHHapticEngine.h, CHHapticDeviceCapability.h) on macOS 26.2.sdk. Counted each interface, protocol, enum, typedef, constant, and method as a symbol. Re-validated the v1 EXEMPT promotions against current SDK availability attributes—both exempt symbols remain valid as they carry SDK_UNAVAILABLE or visionos-only markers. All 139 verified symbols are present in the crate's safe Rust API via swift-bridge thunks and type wrappers.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CoreHapticsErrorDomain | domain constant | CHHapticErrors.h | CORE_HAPTICS_ERROR_DOMAIN |
| CHHapticErrorCode | enum typedef | CHHapticErrors.h | HapticErrorCode (all SDK cases) |
| CHHapticEventType | typed enum typedef | CHHapticEvent.h | HapticEventType |
| CHHapticEventTypeHapticTransient | typed enum constant | CHHapticEvent.h | HapticEventType::HapticTransient |
| CHHapticEventTypeHapticContinuous | typed enum constant | CHHapticEvent.h | HapticEventType::HapticContinuous |
| CHHapticEventTypeAudioContinuous | typed enum constant | CHHapticEvent.h | HapticEventType::AudioContinuous |
| CHHapticEventTypeAudioCustom | typed enum constant | CHHapticEvent.h | HapticEventType::AudioCustom |
| CHHapticAudioResourceID | typedef | CHHapticEvent.h | AudioResourceId |
| CHHapticEvent | interface | CHHapticEvent.h | HapticEvent |
| CHHapticEvent.type | property | CHHapticEvent.h | HapticEvent::event_type |
| CHHapticEvent.eventParameters | property | CHHapticEvent.h | HapticEvent::parameters |
| CHHapticEvent.relativeTime | property | CHHapticEvent.h | HapticEvent::{relative_time,set_relative_time} |
| CHHapticEvent.duration | property | CHHapticEvent.h | HapticEvent::{duration,set_duration} |
| CHHapticEvent.initWithEventType:parameters:relativeTime: | initializer | CHHapticEvent.h | HapticEvent::new, HapticEvent::haptic_transient |
| CHHapticEvent.initWithEventType:parameters:relativeTime:duration: | initializer | CHHapticEvent.h | HapticEvent::new, HapticEvent::{haptic_continuous,audio_continuous} |
| CHHapticEvent.initWithAudioResourceID:parameters:relativeTime: | initializer | CHHapticEvent.h | HapticEvent::audio_custom |
| CHHapticEvent.initWithAudioResourceID:parameters:relativeTime:duration: | initializer | CHHapticEvent.h | HapticEvent::audio_custom_with_duration |
| CHHapticEventParameterID | typed enum typedef | CHHapticParameter.h | HapticParameterId |
| CHHapticEventParameterIDHapticIntensity | typed enum constant | CHHapticParameter.h | HapticParameterId::HapticIntensity |
| CHHapticEventParameterIDHapticSharpness | typed enum constant | CHHapticParameter.h | HapticParameterId::HapticSharpness |
| CHHapticEventParameterIDAttackTime | typed enum constant | CHHapticParameter.h | HapticParameterId::AttackTime |
| CHHapticEventParameterIDDecayTime | typed enum constant | CHHapticParameter.h | HapticParameterId::DecayTime |
| CHHapticEventParameterIDReleaseTime | typed enum constant | CHHapticParameter.h | HapticParameterId::ReleaseTime |
| CHHapticEventParameterIDSustained | typed enum constant | CHHapticParameter.h | HapticParameterId::Sustained |
| CHHapticEventParameterIDAudioVolume | typed enum constant | CHHapticParameter.h | HapticParameterId::AudioVolume |
| CHHapticEventParameterIDAudioPitch | typed enum constant | CHHapticParameter.h | HapticParameterId::AudioPitch |
| CHHapticEventParameterIDAudioPan | typed enum constant | CHHapticParameter.h | HapticParameterId::AudioPan |
| CHHapticEventParameterIDAudioBrightness | typed enum constant | CHHapticParameter.h | HapticParameterId::AudioBrightness |
| CHHapticDynamicParameterID | typed enum typedef | CHHapticParameter.h | DynamicParameterId |
| CHHapticDynamicParameterIDHapticIntensityControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::HapticIntensityControl |
| CHHapticDynamicParameterIDHapticSharpnessControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::HapticSharpnessControl |
| CHHapticDynamicParameterIDHapticAttackTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::HapticAttackTimeControl |
| CHHapticDynamicParameterIDHapticDecayTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::HapticDecayTimeControl |
| CHHapticDynamicParameterIDHapticReleaseTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::HapticReleaseTimeControl |
| CHHapticDynamicParameterIDAudioVolumeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioVolumeControl |
| CHHapticDynamicParameterIDAudioPanControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioPanControl |
| CHHapticDynamicParameterIDAudioBrightnessControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioBrightnessControl |
| CHHapticDynamicParameterIDAudioPitchControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioPitchControl |
| CHHapticDynamicParameterIDAudioAttackTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioAttackTimeControl |
| CHHapticDynamicParameterIDAudioDecayTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioDecayTimeControl |
| CHHapticDynamicParameterIDAudioReleaseTimeControl | typed enum constant | CHHapticParameter.h | DynamicParameterId::AudioReleaseTimeControl |
| CHHapticEventParameter | interface | CHHapticParameter.h | HapticEventParameter |
| CHHapticEventParameter.parameterID | property | CHHapticParameter.h | HapticEventParameter::parameter_id |
| CHHapticEventParameter.value | property | CHHapticParameter.h | HapticEventParameter::{value,set_value} |
| CHHapticEventParameter.initWithParameterID:value: | initializer | CHHapticParameter.h | HapticEventParameter::new |
| CHHapticDynamicParameter | interface | CHHapticParameter.h | DynamicParameter |
| CHHapticDynamicParameter.parameterID | property | CHHapticParameter.h | DynamicParameter::parameter_id |
| CHHapticDynamicParameter.value | property | CHHapticParameter.h | DynamicParameter::{value,set_value} |
| CHHapticDynamicParameter.relativeTime | property | CHHapticParameter.h | DynamicParameter::{relative_time,set_relative_time} |
| CHHapticDynamicParameter.initWithParameterID:value:relativeTime: | initializer | CHHapticParameter.h | DynamicParameter::new |
| CHHapticParameterCurveControlPoint | interface | CHHapticParameter.h | ParameterCurveControlPoint |
| CHHapticParameterCurveControlPoint.relativeTime | property | CHHapticParameter.h | ParameterCurveControlPoint::{relative_time,set_relative_time} |
| CHHapticParameterCurveControlPoint.value | property | CHHapticParameter.h | ParameterCurveControlPoint::{value,set_value} |
| CHHapticParameterCurveControlPoint.initWithRelativeTime:value: | initializer | CHHapticParameter.h | ParameterCurveControlPoint::new |
| CHHapticParameterCurve | interface | CHHapticParameter.h | ParameterCurve |
| CHHapticParameterCurve.parameterID | property | CHHapticParameter.h | ParameterCurve::parameter_id |
| CHHapticParameterCurve.relativeTime | property | CHHapticParameter.h | ParameterCurve::{relative_time,set_relative_time} |
| CHHapticParameterCurve.controlPoints | property | CHHapticParameter.h | ParameterCurve::{control_points,set_control_points,push_control_point} |
| CHHapticParameterCurve.initWithParameterID:controlPoints:relativeTime: | initializer | CHHapticParameter.h | ParameterCurve::new |
| CHHapticPatternKey | typed enum typedef | CHHapticPattern.h | PatternKey |
| CHHapticPatternKeyVersion | typed enum constant | CHHapticPattern.h | PatternKey::Version |
| CHHapticPatternKeyPattern | typed enum constant | CHHapticPattern.h | PatternKey::Pattern |
| CHHapticPatternKeyEvent | typed enum constant | CHHapticPattern.h | PatternKey::Event |
| CHHapticPatternKeyEventType | typed enum constant | CHHapticPattern.h | PatternKey::EventType |
| CHHapticPatternKeyTime | typed enum constant | CHHapticPattern.h | PatternKey::Time |
| CHHapticPatternKeyEventDuration | typed enum constant | CHHapticPattern.h | PatternKey::EventDuration |
| CHHapticPatternKeyEventWaveformPath | typed enum constant | CHHapticPattern.h | PatternKey::EventWaveformPath |
| CHHapticPatternKeyEventParameters | typed enum constant | CHHapticPattern.h | PatternKey::EventParameters |
| CHHapticPatternKeyEventWaveformUseVolumeEnvelope | typed enum constant | CHHapticPattern.h | PatternKey::EventWaveformUseVolumeEnvelope |
| CHHapticPatternKeyEventWaveformLoopEnabled | typed enum constant | CHHapticPattern.h | PatternKey::EventWaveformLoopEnabled |
| CHHapticPatternKeyParameter | typed enum constant | CHHapticPattern.h | PatternKey::Parameter |
| CHHapticPatternKeyParameterID | typed enum constant | CHHapticPattern.h | PatternKey::ParameterId |
| CHHapticPatternKeyParameterValue | typed enum constant | CHHapticPattern.h | PatternKey::ParameterValue |
| CHHapticPatternKeyParameterCurve | typed enum constant | CHHapticPattern.h | PatternKey::ParameterCurve |
| CHHapticPatternKeyParameterCurveControlPoints | typed enum constant | CHHapticPattern.h | PatternKey::ParameterCurveControlPoints |
| CHHapticPattern | interface | CHHapticPattern.h | HapticPattern |
| CHHapticPattern.duration | property | CHHapticPattern.h | HapticPattern::duration |
| CHHapticPattern.initWithEvents:parameters:error: | initializer | CHHapticPattern.h | HapticPattern::new |
| CHHapticPattern.initWithEvents:parameterCurves:error: | initializer | CHHapticPattern.h | HapticPattern::with_parameter_curves |
| CHHapticPattern.initWithDictionary:error: | initializer | CHHapticPattern.h | HapticPattern::{from_dictionary,from_dictionary_json} |
| CHHapticPattern.initWithContentsOfURL:error: | initializer | CHHapticPattern.h | HapticPattern::from_file |
| CHHapticPattern.exportDictionaryAndReturnError: | method | CHHapticPattern.h | HapticPattern::{export_dictionary,export_dictionary_json} |
| CHHapticPatternPlayer | protocol | CHHapticPatternPlayer.h | PatternPlayer |
| CHHapticPatternPlayer.startAtTime:error: | method | CHHapticPatternPlayer.h | PatternPlayer::start_at_time |
| CHHapticPatternPlayer.stopAtTime:error: | method | CHHapticPatternPlayer.h | PatternPlayer::stop_at_time |
| CHHapticPatternPlayer.sendParameters:atTime:error: | method | CHHapticPatternPlayer.h | PatternPlayer::send_parameters |
| CHHapticPatternPlayer.scheduleParameterCurve:atTime:error: | method | CHHapticPatternPlayer.h | PatternPlayer::schedule_parameter_curve |
| CHHapticPatternPlayer.cancelAndReturnError: | method | CHHapticPatternPlayer.h | PatternPlayer::cancel |
| CHHapticPatternPlayer.isMuted | property | CHHapticPatternPlayer.h | PatternPlayer::{is_muted,set_muted} |
| CHHapticAdvancedPatternPlayerCompletionHandler | block typedef | CHHapticPatternPlayer.h | AdvancedPatternPlayer::set_completion_handler (Rust closure) |
| CHHapticAdvancedPatternPlayer | protocol | CHHapticPatternPlayer.h | AdvancedPatternPlayer |
| CHHapticAdvancedPatternPlayer.pauseAtTime:error: | method | CHHapticPatternPlayer.h | AdvancedPatternPlayer::pause_at_time |
| CHHapticAdvancedPatternPlayer.resumeAtTime:error: | method | CHHapticPatternPlayer.h | AdvancedPatternPlayer::resume_at_time |
| CHHapticAdvancedPatternPlayer.seekToOffset:error: | method | CHHapticPatternPlayer.h | AdvancedPatternPlayer::seek_to_offset |
| CHHapticAdvancedPatternPlayer.loopEnabled | property | CHHapticPatternPlayer.h | AdvancedPatternPlayer::{loop_enabled,set_loop_enabled} |
| CHHapticAdvancedPatternPlayer.loopEnd | property | CHHapticPatternPlayer.h | AdvancedPatternPlayer::{loop_end,set_loop_end} |
| CHHapticAdvancedPatternPlayer.playbackRate | property | CHHapticPatternPlayer.h | AdvancedPatternPlayer::{playback_rate,set_playback_rate} |
| CHHapticAdvancedPatternPlayer.isMuted | property | CHHapticPatternPlayer.h | AdvancedPatternPlayer::{is_muted,set_muted} |
| CHHapticAdvancedPatternPlayer.completionHandler | property | CHHapticPatternPlayer.h | AdvancedPatternPlayer::set_completion_handler / clear_completion_handler |
| CHHapticParameterAttributes | protocol | CHHapticDeviceCapability.h | ParameterAttributes |
| CHHapticParameterAttributes.minValue | property | CHHapticDeviceCapability.h | ParameterAttributes::min_value |
| CHHapticParameterAttributes.maxValue | property | CHHapticDeviceCapability.h | ParameterAttributes::max_value |
| CHHapticParameterAttributes.defaultValue | property | CHHapticDeviceCapability.h | ParameterAttributes::default_value |
| CHHapticDeviceCapability | protocol | CHHapticDeviceCapability.h | DeviceCapability |
| CHHapticDeviceCapability.supportsHaptics | property | CHHapticDeviceCapability.h | DeviceCapability::supports_haptics |
| CHHapticDeviceCapability.supportsAudio | property | CHHapticDeviceCapability.h | DeviceCapability::supports_audio |
| CHHapticDeviceCapability.attributesForEventParameter:eventType:error: | method | CHHapticDeviceCapability.h | DeviceCapability::event_parameter_attributes |
| CHHapticDeviceCapability.attributesForDynamicParameter:error: | method | CHHapticDeviceCapability.h | DeviceCapability::dynamic_parameter_attributes |
| CHHapticTimeImmediate | macro constant | CHHapticEngine.h | HAPTIC_TIME_IMMEDIATE |
| CHHapticEngineFinishedAction | enum typedef | CHHapticEngine.h | EngineFinishedAction |
| CHHapticEngineFinishedHandler | block typedef | CHHapticEngine.h | HapticEngine::notify_when_players_finished (Rust closure) |
| CHHapticEngineStoppedReason | enum typedef | CHHapticEngine.h | EngineStoppedReason |
| CHHapticEngineStoppedHandler | block typedef | CHHapticEngine.h | HapticEngine::set_stopped_handler (Rust closure) |
| CHHapticEngineResetHandler | block typedef | CHHapticEngine.h | HapticEngine::set_reset_handler (Rust closure) |
| CHHapticCompletionHandler | block typedef | CHHapticEngine.h | HapticEngine::{start_with_completion_handler,start_async,stop_with_completion_handler,stop_async} (Rust closures) |
| CHHapticEngine | interface | CHHapticEngine.h | HapticEngine |
| CHHapticEngine.capabilitiesForHardware | class method | CHHapticEngine.h | DeviceCapability::current |
| CHHapticEngine.currentTime | property | CHHapticEngine.h | HapticEngine::current_time |
| CHHapticEngine.stoppedHandler | property | CHHapticEngine.h | HapticEngine::set_stopped_handler / clear_stopped_handler |
| CHHapticEngine.resetHandler | property | CHHapticEngine.h | HapticEngine::set_reset_handler / clear_reset_handler |
| CHHapticEngine.playsHapticsOnly | property | CHHapticEngine.h | HapticEngine::{plays_haptics_only,set_plays_haptics_only} |
| CHHapticEngine.playsAudioOnly | property | CHHapticEngine.h | HapticEngine::{plays_audio_only,set_plays_audio_only} |
| CHHapticEngine.isMutedForAudio | property | CHHapticEngine.h | HapticEngine::{is_muted_for_audio,set_muted_for_audio} |
| CHHapticEngine.isMutedForHaptics | property | CHHapticEngine.h | HapticEngine::{is_muted_for_haptics,set_muted_for_haptics} |
| CHHapticEngine.autoShutdownEnabled | property | CHHapticEngine.h | HapticEngine::{auto_shutdown_enabled,set_auto_shutdown_enabled} |
| CHHapticEngine.initAndReturnError: | initializer | CHHapticEngine.h | HapticEngine::new |
| CHHapticEngine.startWithCompletionHandler: | method | CHHapticEngine.h | HapticEngine::{start_with_completion_handler,start_async} |
| CHHapticEngine.startAndReturnError: | method | CHHapticEngine.h | HapticEngine::start |
| CHHapticEngine.stopWithCompletionHandler: | method | CHHapticEngine.h | HapticEngine::{stop_with_completion_handler,stop_async} |
| CHHapticEngine.notifyWhenPlayersFinished: | method | CHHapticEngine.h | HapticEngine::notify_when_players_finished |
| CHHapticEngine.createPlayerWithPattern:error: | method | CHHapticEngine.h | HapticEngine::create_player |
| CHHapticEngine.createAdvancedPlayerWithPattern:error: | method | CHHapticEngine.h | HapticEngine::create_advanced_player |
| CHHapticAudioResourceKey | typedef | CHHapticEngine.h | AudioResourceKey |
| CHHapticAudioResourceKeyUseVolumeEnvelope | constant | CHHapticEngine.h | AudioResourceKey::UseVolumeEnvelope, AudioResourceOptions::with_use_volume_envelope |
| CHHapticAudioResourceKeyLoopEnabled | constant | CHHapticEngine.h | AudioResourceKey::LoopEnabled, AudioResourceOptions::with_loop_enabled |
| CHHapticEngine.registerAudioResource:options:error: | method | CHHapticEngine.h | HapticEngine::register_audio_resource |
| CHHapticEngine.unregisterAudioResource:error: | method | CHHapticEngine.h | HapticEngine::unregister_audio_resource |
| CHHapticEngine.playPatternFromURL:error: | method | CHHapticEngine.h | HapticEngine::play_pattern_from_file |
| CHHapticEngine.playPatternFromData:error: | method | CHHapticEngine.h | HapticEngine::play_pattern_from_data |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CHHapticEngine.initWithAudioSession:error: | initializer | CHHapticEngine.h | The initializer depends on AVAudioSession, which is unavailable to macOS-targeted callers. | AVAudioSession is unavailable on macOS |
| CHHapticEngine.intendedSpatialExperience | property | CHHapticEngine.h | The property is explicitly visionOS-only and unavailable on macOS. | API_AVAILABLE(visionos(26.0)) API_UNAVAILABLE(ios, watchos, tvos, macos) |
