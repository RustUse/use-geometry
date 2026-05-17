#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A pair of affine weights for two points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AffinePair {
    first: f64,
    second: f64,
}

impl AffinePair {
    /// Creates affine weights that sum to one.
    #[must_use]
    pub fn new(first: f64, second: f64) -> Option<Self> {
        if first.is_finite() && second.is_finite() && (first + second - 1.0).abs() <= 1.0e-12 {
            Some(Self { first, second })
        } else {
            None
        }
    }

    /// Returns the first weight.
    #[must_use]
    pub const fn first(self) -> f64 {
        self.first
    }

    /// Returns the second weight.
    #[must_use]
    pub const fn second(self) -> f64 {
        self.second
    }
}

/// Returns the affine interpolation between two 2D points.
#[must_use]
pub const fn affine_combination_2d(a: Point2, b: Point2, t: f64) -> Point2 {
    a.lerp(b, t)
}

#[cfg(test)]
mod tests {
    use super::{AffinePair, affine_combination_2d};
    use use_point::Point2;

    #[test]
    fn computes_affine_combinations() {
        let midpoint = affine_combination_2d(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0), 0.5);

        assert_eq!(midpoint, Point2::new(2.0, 1.0));
        assert_eq!(AffinePair::new(0.25, 0.75).expect("valid").first(), 0.25);
        assert_eq!(AffinePair::new(0.25, 0.70), None);
    }
}
