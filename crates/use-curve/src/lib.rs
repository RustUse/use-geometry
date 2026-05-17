#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A normalized curve parameter in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveParameter {
    value: f64,
}

impl CurveParameter {
    /// Creates a normalized curve parameter.
    #[must_use]
    pub const fn new(value: f64) -> Option<Self> {
        if value.is_finite() && value >= 0.0 && value <= 1.0 {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Returns the parameter value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

/// A sampled 2D curve point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveSample2 {
    parameter: CurveParameter,
    point: Point2,
}

impl CurveSample2 {
    /// Creates a curve sample.
    #[must_use]
    pub const fn new(parameter: CurveParameter, point: Point2) -> Self {
        Self { parameter, point }
    }

    /// Returns the parameter.
    #[must_use]
    pub const fn parameter(self) -> CurveParameter {
        self.parameter
    }

    /// Returns the sampled point.
    #[must_use]
    pub const fn point(self) -> Point2 {
        self.point
    }
}

#[cfg(test)]
mod tests {
    use super::{CurveParameter, CurveSample2};
    use use_point::Point2;

    #[test]
    fn creates_curve_samples() {
        let parameter = CurveParameter::new(0.5).expect("valid parameter");
        let sample = CurveSample2::new(parameter, Point2::new(1.0, 2.0));

        assert_eq!(sample.parameter().value(), 0.5);
        assert_eq!(sample.point(), Point2::new(1.0, 2.0));
        assert_eq!(CurveParameter::new(1.5), None);
    }
}
