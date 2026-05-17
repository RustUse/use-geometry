#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A quadratic Bezier curve in 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadraticBezier2 {
    start: Point2,
    control: Point2,
    end: Point2,
}

impl QuadraticBezier2 {
    /// Creates a quadratic Bezier curve.
    #[must_use]
    pub const fn new(start: Point2, control: Point2, end: Point2) -> Self {
        Self {
            start,
            control,
            end,
        }
    }

    /// Evaluates the curve at `t`.
    #[must_use]
    pub const fn point_at(self, t: f64) -> Point2 {
        let left = self.start.lerp(self.control, t);
        let right = self.control.lerp(self.end, t);
        left.lerp(right, t)
    }
}

/// A cubic Bezier curve in 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier2 {
    start: Point2,
    control_a: Point2,
    control_b: Point2,
    end: Point2,
}

impl CubicBezier2 {
    /// Creates a cubic Bezier curve.
    #[must_use]
    pub const fn new(start: Point2, control_a: Point2, control_b: Point2, end: Point2) -> Self {
        Self {
            start,
            control_a,
            control_b,
            end,
        }
    }

    /// Evaluates the curve at `t`.
    #[must_use]
    pub const fn point_at(self, t: f64) -> Point2 {
        let left = self.start.lerp(self.control_a, t);
        let middle = self.control_a.lerp(self.control_b, t);
        let right = self.control_b.lerp(self.end, t);
        let left = left.lerp(middle, t);
        let right = middle.lerp(right, t);
        left.lerp(right, t)
    }
}

#[cfg(test)]
mod tests {
    use super::{CubicBezier2, QuadraticBezier2};
    use use_point::Point2;

    #[test]
    fn evaluates_quadratic_curves() {
        let curve = QuadraticBezier2::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 2.0),
            Point2::new(2.0, 0.0),
        );

        assert_eq!(curve.point_at(0.5), Point2::new(1.0, 1.0));
    }

    #[test]
    fn evaluates_cubic_endpoints() {
        let curve = CubicBezier2::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 2.0),
            Point2::new(2.0, 2.0),
            Point2::new(3.0, 0.0),
        );

        assert_eq!(curve.point_at(0.0), Point2::new(0.0, 0.0));
        assert_eq!(curve.point_at(1.0), Point2::new(3.0, 0.0));
    }
}
