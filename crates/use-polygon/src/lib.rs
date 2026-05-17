#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// An ordered planar polygon boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    vertices: Vec<Point2>,
}

impl Polygon {
    /// Creates a polygon boundary from ordered vertices.
    #[must_use]
    pub const fn new(vertices: Vec<Point2>) -> Self {
        Self { vertices }
    }

    /// Returns the vertices.
    #[must_use]
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }

    /// Returns the vertex count.
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns `true` when there are no vertices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Returns twice the signed area by the shoelace formula.
    #[must_use]
    pub fn twice_signed_area(&self) -> f64 {
        if self.vertices.len() < 3 {
            return 0.0;
        }

        self.vertices
            .iter()
            .zip(self.vertices.iter().cycle().skip(1))
            .take(self.vertices.len())
            .map(|(a, b)| a.x() * b.y() - a.y() * b.x())
            .sum()
    }

    /// Returns the unsigned polygon area.
    #[must_use]
    pub fn area(&self) -> f64 {
        self.twice_signed_area().abs() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::Polygon;
    use use_point::Point2;

    #[test]
    fn computes_polygon_area() {
        let polygon = Polygon::new(vec![
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(0.0, 3.0),
        ]);

        assert_eq!(polygon.vertex_count(), 3);
        assert_eq!(polygon.area(), 6.0);
        assert!(!polygon.is_empty());
    }
}
