mod common;

use common::{supports_audio, supports_haptics};
use corehaptics::prelude::*;

#[test]
fn current_capability_reports_support_flags() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    let _ = capability.supports_haptics();
    let _ = capability.supports_audio();
    Ok(())
}

#[test]
fn capability_attributes_are_queryable_when_supported() -> corehaptics::Result<()> {
    let capability = DeviceCapability::current()?;
    if supports_haptics()? {
        let attributes = capability.event_parameter_attributes(
            HapticParameterId::HapticIntensity,
            HapticEventType::HapticTransient,
        )?;
        assert!(attributes.max_value() >= attributes.min_value());
        assert!(attributes.default_value() >= attributes.min_value());
    }
    if supports_audio()? || supports_haptics()? {
        let attributes =
            capability.dynamic_parameter_attributes(DynamicParameterId::HapticIntensityControl)?;
        assert!(attributes.max_value() >= attributes.min_value());
    }
    Ok(())
}
