#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// Common convex hull algorithm families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HullAlgorithm {
    /// The monotone-chain algorithm family.
    MonotoneChain,
    /// The gift-wrapping algorithm family.
    GiftWrapping,
    /// The quickhull algorithm family.
    QuickHull,
}

impl HullAlgorithm {
    /// Returns a stable algorithm name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::MonotoneChain => "monotone-chain",
            Self::GiftWrapping => "gift-wrapping",
            Self::QuickHull => "quickhull",
        }
    }
}

/// A 2D convex hull point set descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvexHull2 {
    points: Vec<Point2>,
}

impl ConvexHull2 {
    /// Creates a hull descriptor from input points.
    #[must_use]
    pub const fn new(points: Vec<Point2>) -> Self {
        Self { points }
    }

    /// Returns the input points.
    #[must_use]
    pub fn points(&self) -> &[Point2] {
        &self.points
    }

    /// Returns the number of input points.
    #[must_use]
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{ConvexHull2, HullAlgorithm};
    use use_point::Point2;

    #[test]
    fn stores_hull_inputs() {
        let hull = ConvexHull2::new(vec![Point2::origin(), Point2::new(1.0, 0.0)]);

        assert_eq!(hull.point_count(), 2);
        assert_eq!(hull.points()[1], Point2::new(1.0, 0.0));
        assert_eq!(HullAlgorithm::MonotoneChain.name(), "monotone-chain");
    }
}
