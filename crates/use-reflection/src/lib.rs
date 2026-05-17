#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// Axis-aligned 2D reflection choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisReflection2 {
    /// Reflect across the x axis.
    AcrossX,
    /// Reflect across the y axis.
    AcrossY,
    /// Reflect through the origin.
    ThroughOrigin,
}

impl AxisReflection2 {
    /// Reflects a point according to this reflection.
    #[must_use]
    pub const fn reflect(self, point: Point2) -> Point2 {
        match self {
            Self::AcrossX => Point2::new(point.x(), -point.y()),
            Self::AcrossY => Point2::new(-point.x(), point.y()),
            Self::ThroughOrigin => Point2::new(-point.x(), -point.y()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AxisReflection2;
    use use_point::Point2;

    #[test]
    fn reflects_across_axes() {
        let point = Point2::new(2.0, 3.0);

        assert_eq!(
            AxisReflection2::AcrossX.reflect(point),
            Point2::new(2.0, -3.0)
        );
        assert_eq!(
            AxisReflection2::AcrossY.reflect(point),
            Point2::new(-2.0, 3.0)
        );
        assert_eq!(
            AxisReflection2::ThroughOrigin.reflect(point),
            Point2::new(-2.0, -3.0)
        );
    }
}
