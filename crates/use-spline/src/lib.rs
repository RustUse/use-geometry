#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A 2D spline control-point sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Spline2 {
    control_points: Vec<Point2>,
}

impl Spline2 {
    /// Creates a spline with at least two control points.
    #[must_use]
    pub fn new(control_points: Vec<Point2>) -> Option<Self> {
        if control_points.len() >= 2 {
            Some(Self { control_points })
        } else {
            None
        }
    }

    /// Returns the control points.
    #[must_use]
    pub fn control_points(&self) -> &[Point2] {
        &self.control_points
    }

    /// Returns the number of control points.
    #[must_use]
    pub fn control_point_count(&self) -> usize {
        self.control_points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Spline2;
    use use_point::Point2;

    #[test]
    fn validates_control_point_count() {
        assert_eq!(Spline2::new(vec![Point2::origin()]), None);

        let spline =
            Spline2::new(vec![Point2::origin(), Point2::new(1.0, 1.0)]).expect("valid spline");
        assert_eq!(spline.control_point_count(), 2);
        assert_eq!(spline.control_points()[1], Point2::new(1.0, 1.0));
    }
}
