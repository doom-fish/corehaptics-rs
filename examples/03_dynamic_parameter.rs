use corehaptics::prelude::*;

fn main() {
    let mut parameter = DynamicParameter::haptic_intensity_control(0.3, 0.05);
    println!(
        "dynamic {} at {:.2}s = {:.2}",
        parameter.parameter_id().as_str(),
        parameter.relative_time(),
        parameter.value()
    );
    parameter.set_value(0.8);
    parameter.set_relative_time(0.1);
    println!(
        "updated {} at {:.2}s = {:.2}",
        parameter.parameter_id().as_str(),
        parameter.relative_time(),
        parameter.value()
    );
}
