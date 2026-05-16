use corehaptics::prelude::*;

#[test]
fn event_builders_capture_audio_resource_ids() {
    let mut event = HapticEvent::haptic_continuous(
        0.0,
        0.25,
        vec![HapticEventParameter::haptic_intensity(1.0)],
    );
    assert_eq!(event.event_type(), HapticEventType::HapticContinuous);
    assert_eq!(event.duration(), Some(0.25));
    assert_eq!(event.parameters().len(), 1);

    event.set_duration(Some(0.5));
    event.set_event_type(HapticEventType::AudioContinuous);
    assert_eq!(event.event_type().as_str(), "audioContinuous");
    assert_eq!(event.duration(), Some(0.5));

    let audio_custom = HapticEvent::audio_custom_with_duration(
        99,
        0.1,
        0.25,
        vec![HapticEventParameter::audio_volume(0.5)],
    );
    assert_eq!(audio_custom.event_type(), HapticEventType::AudioCustom);
    assert_eq!(audio_custom.audio_resource_id(), Some(99));
}
