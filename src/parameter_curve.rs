//! `CHHapticParameterCurve` wrapper values.

#![allow(clippy::missing_const_for_fn)]

use serde::{Deserialize, Serialize};

use crate::dynamic_parameter::DynamicParameterId;

/// A single `CHHapticParameterCurveControlPoint` value object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterCurveControlPoint {
    relative_time: f64,
    value: f32,
}

impl ParameterCurveControlPoint {
    /// Creates a curve control point.
    #[must_use]
    pub const fn new(relative_time: f64, value: f32) -> Self {
        Self {
            relative_time,
            value,
        }
    }

    /// Returns the control point time in seconds.
    #[must_use]
    pub const fn relative_time(&self) -> f64 {
        self.relative_time
    }

    /// Sets the control point time in seconds.
    pub fn set_relative_time(&mut self, relative_time: f64) {
        self.relative_time = relative_time;
    }

    /// Returns the control point value.
    #[must_use]
    pub const fn value(&self) -> f32 {
        self.value
    }

    /// Sets the control point value.
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

/// A single `CHHapticParameterCurve` value object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterCurve {
    parameter_id: DynamicParameterId,
    relative_time: f64,
    control_points: Vec<ParameterCurveControlPoint>,
}

impl ParameterCurve {
    /// Creates a parameter curve.
    #[must_use]
    pub fn new(
        parameter_id: DynamicParameterId,
        control_points: Vec<ParameterCurveControlPoint>,
        relative_time: f64,
    ) -> Self {
        Self {
            parameter_id,
            relative_time,
            control_points,
        }
    }

    /// Returns the parameter identifier.
    #[must_use]
    pub const fn parameter_id(&self) -> DynamicParameterId {
        self.parameter_id
    }

    /// Returns the curve start time in seconds.
    #[must_use]
    pub const fn relative_time(&self) -> f64 {
        self.relative_time
    }

    /// Sets the curve start time in seconds.
    pub fn set_relative_time(&mut self, relative_time: f64) {
        self.relative_time = relative_time;
    }

    /// Returns the control points in playback order.
    #[must_use]
    pub fn control_points(&self) -> &[ParameterCurveControlPoint] {
        &self.control_points
    }

    /// Replaces the control points in playback order.
    pub fn set_control_points(&mut self, control_points: Vec<ParameterCurveControlPoint>) {
        self.control_points = control_points;
    }

    /// Appends a control point to the curve.
    pub fn push_control_point(&mut self, control_point: ParameterCurveControlPoint) {
        self.control_points.push(control_point);
    }
}
