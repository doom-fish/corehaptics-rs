use corehaptics::prelude::*;

#[test]
fn dynamic_parameter_round_trip() {
    let mut parameter = DynamicParameter::audio_pitch_control(-0.25, 0.1);
    assert_eq!(
        parameter.parameter_id(),
        DynamicParameterId::AudioPitchControl
    );
    assert!((parameter.relative_time() - 0.1).abs() < f64::EPSILON);
    assert!((parameter.value() + 0.25).abs() < f32::EPSILON);

    parameter.set_relative_time(0.2);
    parameter.set_value(0.0);
    assert!((parameter.relative_time() - 0.2).abs() < f64::EPSILON);
    assert!(parameter.value().abs() < f32::EPSILON);
}
