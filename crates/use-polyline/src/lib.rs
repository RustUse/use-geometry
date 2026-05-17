#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_distance::distance_2d;
use use_point::Point2;

/// An ordered 2D polyline.
#[derive(Debug, Clone, PartialEq)]
pub struct Polyline2 {
    vertices: Vec<Point2>,
}

impl Polyline2 {
    /// Creates a polyline from ordered vertices.
    #[must_use]
    pub const fn new(vertices: Vec<Point2>) -> Self {
        Self { vertices }
    }

    /// Returns the vertices.
    #[must_use]
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }

    /// Returns the number of vertices.
    #[must_use]
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Returns `true` when there are no vertices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Returns the total segment length.
    #[must_use]
    pub fn length(&self) -> f64 {
        self.vertices
            .windows(2)
            .map(|pair| distance_2d(pair[0], pair[1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Polyline2;
    use use_point::Point2;

    #[test]
    fn computes_polyline_lengths() {
        let polyline = Polyline2::new(vec![Point2::new(0.0, 0.0), Point2::new(3.0, 4.0)]);

        assert_eq!(polyline.len(), 2);
        assert!(!polyline.is_empty());
        assert_eq!(polyline.length(), 5.0);
    }
}
