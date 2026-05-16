use corehaptics::prelude::*;

#[test]
fn event_parameter_builders_cover_core_ids() {
    let mut parameter = HapticEventParameter::haptic_intensity(0.5);
    assert_eq!(parameter.parameter_id(), HapticParameterId::HapticIntensity);
    assert!((parameter.value() - 0.5).abs() < f32::EPSILON);

    parameter.set_value(0.75);
    assert!((parameter.value() - 0.75).abs() < f32::EPSILON);
    assert_eq!(
        HapticParameterId::AudioBrightness.as_str(),
        "audioBrightness"
    );
    assert!((HapticEventParameter::sustained(true).value() - 1.0).abs() < f32::EPSILON);
}
