#![allow(dead_code)]

use std::path::PathBuf;

use corehaptics::prelude::*;

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

pub fn sample_pattern() -> corehaptics::Result<HapticPattern> {
    HapticPattern::new(
        &[HapticEvent::haptic_transient(
            0.0,
            vec![
                HapticEventParameter::haptic_intensity(1.0),
                HapticEventParameter::haptic_sharpness(0.5),
            ],
        )],
        &[],
    )
}

pub fn sample_continuous_pattern() -> corehaptics::Result<HapticPattern> {
    HapticPattern::new(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.15,
            vec![
                HapticEventParameter::haptic_intensity(0.75),
                HapticEventParameter::haptic_sharpness(0.4),
            ],
        )],
        &[],
    )
}

pub fn sample_curve() -> ParameterCurve {
    ParameterCurve::new(
        DynamicParameterId::HapticIntensityControl,
        vec![
            ParameterCurveControlPoint::new(0.0, 0.25),
            ParameterCurveControlPoint::new(0.05, 0.75),
        ],
        0.0,
    )
}

pub fn supports_haptics() -> corehaptics::Result<bool> {
    Ok(DeviceCapability::current()?.supports_haptics())
}

pub fn supports_audio() -> corehaptics::Result<bool> {
    Ok(DeviceCapability::current()?.supports_audio())
}
