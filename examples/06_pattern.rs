use std::{fs, path::PathBuf};

use corehaptics::prelude::*;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn main() -> corehaptics::Result<()> {
    let pattern = HapticPattern::with_parameter_curves(
        &[HapticEvent::haptic_continuous(
            0.0,
            0.1,
            vec![HapticEventParameter::haptic_intensity(0.5)],
        )],
        &[ParameterCurve::new(
            DynamicParameterId::HapticIntensityControl,
            vec![
                ParameterCurveControlPoint::new(0.0, 0.2),
                ParameterCurveControlPoint::new(0.05, 0.8),
            ],
            0.0,
        )],
    )?;
    let exported = pattern.export_dictionary_json()?;
    let imported = HapticPattern::from_dictionary_json(&exported)?;
    let from_file = HapticPattern::from_file(fixture_path("minimal.ahap"))?;
    let raw = fs::read_to_string(fixture_path("minimal.ahap"))?;
    let _ = HapticPattern::from_dictionary_json(&raw)?;
    println!(
        "exported {} bytes; imported duration {:.3}s; fixture duration {:.3}s",
        exported.len(),
        imported.duration(),
        from_file.duration()
    );
    Ok(())
}
