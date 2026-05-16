use corehaptics::prelude::*;

fn main() {
    let mut intensity = HapticEventParameter::haptic_intensity(0.75);
    println!(
        "parameter {} = {:.2}",
        intensity.parameter_id().as_str(),
        intensity.value()
    );
    intensity.set_value(1.0);
    let sustained = HapticEventParameter::sustained(true);
    println!(
        "updated {} = {:.2}; sustained={} => {:.1}",
        intensity.parameter_id().as_str(),
        intensity.value(),
        sustained.parameter_id().as_str(),
        sustained.value()
    );
}
