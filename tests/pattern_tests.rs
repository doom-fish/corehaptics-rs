mod common;

use std::fs;

use common::{fixture_path, sample_curve, sample_pattern};
use corehaptics::prelude::*;

#[test]
fn pattern_round_trips_through_dictionary_json() -> corehaptics::Result<()> {
    let pattern = sample_pattern()?;
    let exported = pattern.export_dictionary_json()?;
    let imported = HapticPattern::from_dictionary_json(&exported)?;
    assert!(imported.duration() >= 0.0);
    assert!(exported.contains(PatternKey::Pattern.as_str()));
    Ok(())
}

#[test]
fn pattern_supports_parameter_curves_and_file_loading() -> corehaptics::Result<()> {
    let curve_pattern = HapticPattern::with_parameter_curves(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.1,
            vec![HapticEventParameter::haptic_intensity(0.5)],
        )],
        &[sample_curve()],
    )?;
    assert!(curve_pattern.duration() >= 0.1);

    let raw = fs::read_to_string(fixture_path("minimal.ahap"))?;
    let from_json = HapticPattern::from_dictionary_json(&raw)?;
    let from_file = HapticPattern::from_file(fixture_path("minimal.ahap"))?;
    assert_eq!(
        from_json.export_dictionary()?,
        from_file.export_dictionary()?
    );
    Ok(())
}
