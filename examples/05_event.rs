use corehaptics::prelude::*;

fn main() {
    let transient =
        HapticEvent::haptic_transient(0.0, vec![HapticEventParameter::haptic_intensity(1.0)]);
    let audio_custom =
        HapticEvent::audio_custom(7, 0.25, vec![HapticEventParameter::audio_volume(0.5)]);
    println!(
        "event {} at {:.2}s, params={}; custom resource={:?}",
        transient.event_type().as_str(),
        transient.relative_time(),
        transient.parameters().len(),
        audio_custom.audio_resource_id()
    );
}
