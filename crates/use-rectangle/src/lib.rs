#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_bounds::Aabb2;
use use_point::Point2;

/// An axis-aligned rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    bounds: Aabb2,
}

impl Rectangle {
    /// Creates a rectangle from any two corners.
    #[must_use]
    pub const fn from_corners(a: Point2, b: Point2) -> Self {
        Self {
            bounds: Aabb2::from_points(a, b),
        }
    }

    /// Creates a rectangle from bounds.
    #[must_use]
    pub const fn from_bounds(bounds: Aabb2) -> Self {
        Self { bounds }
    }

    /// Returns the underlying bounds.
    #[must_use]
    pub const fn bounds(self) -> Aabb2 {
        self.bounds
    }

    /// Returns the rectangle width.
    #[must_use]
    pub fn width(self) -> f64 {
        self.bounds.width()
    }

    /// Returns the rectangle height.
    #[must_use]
    pub fn height(self) -> f64 {
        self.bounds.height()
    }

    /// Returns the rectangle area.
    #[must_use]
    pub fn area(self) -> f64 {
        self.bounds.area()
    }

    /// Returns the rectangle center.
    #[must_use]
    pub const fn center(self) -> Point2 {
        self.bounds.center()
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use use_point::Point2;

    #[test]
    fn computes_rectangle_measurements() {
        let rectangle = Rectangle::from_corners(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0));

        assert_eq!(rectangle.width(), 4.0);
        assert_eq!(rectangle.height(), 2.0);
        assert_eq!(rectangle.area(), 8.0);
        assert_eq!(rectangle.center(), Point2::new(2.0, 1.0));
    }
}
