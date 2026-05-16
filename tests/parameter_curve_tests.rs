use corehaptics::prelude::*;

#[test]
fn parameter_curve_control_points_are_mutable() {
    let mut curve = ParameterCurve::new(
        DynamicParameterId::HapticIntensityControl,
        vec![
            ParameterCurveControlPoint::new(0.0, 0.1),
            ParameterCurveControlPoint::new(0.1, 0.9),
        ],
        0.0,
    );
    assert_eq!(curve.control_points().len(), 2);
    assert_eq!(
        curve.parameter_id(),
        DynamicParameterId::HapticIntensityControl
    );

    curve.push_control_point(ParameterCurveControlPoint::new(0.2, 0.3));
    curve.set_relative_time(0.05);
    assert_eq!(curve.control_points().len(), 3);
    assert!((curve.relative_time() - 0.05).abs() < f64::EPSILON);
}
