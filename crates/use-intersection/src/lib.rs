#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A minimal two-dimensional intersection result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intersection2 {
    /// No intersection was found.
    None,
    /// A single point intersection.
    Point(Point2),
    /// A finite overlapping region or segment.
    Overlap,
    /// Infinitely many intersections, such as coincident infinite primitives.
    Infinite,
}

impl Intersection2 {
    /// Returns `true` when this is [`Intersection2::None`].
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::None)
    }
}

/// A trait for values that can report whether they intersect `Rhs`.
pub trait Intersects<Rhs = Self> {
    /// Returns `true` when `self` intersects `rhs`.
    fn intersects(&self, rhs: &Rhs) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Intersection2;
    use use_point::Point2;

    #[test]
    fn classifies_empty_and_point_intersections() {
        assert!(Intersection2::None.is_empty());
        assert!(!Intersection2::Point(Point2::new(1.0, 2.0)).is_empty());
    }
}
