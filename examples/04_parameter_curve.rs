use corehaptics::prelude::*;

fn main() {
    let curve = ParameterCurve::new(
        DynamicParameterId::HapticIntensityControl,
        vec![
            ParameterCurveControlPoint::new(0.0, 0.25),
            ParameterCurveControlPoint::new(0.1, 1.0),
        ],
        0.0,
    );
    println!(
        "curve {} has {} control points",
        curve.parameter_id().as_str(),
        curve.control_points().len()
    );
}
