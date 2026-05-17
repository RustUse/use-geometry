#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Axis labels for two-dimensional coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis2 {
    /// Horizontal axis.
    X,
    /// Vertical axis.
    Y,
}

/// Axis labels for three-dimensional coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis3 {
    /// X axis.
    X,
    /// Y axis.
    Y,
    /// Z axis.
    Z,
}

/// A raw two-dimensional coordinate pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate2 {
    x: f64,
    y: f64,
}

impl Coordinate2 {
    /// Creates a two-dimensional coordinate pair.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the origin coordinate.
    #[must_use]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Returns the x component.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Returns the y component.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Returns the component selected by `axis`.
    #[must_use]
    pub const fn component(self, axis: Axis2) -> f64 {
        match axis {
            Axis2::X => self.x,
            Axis2::Y => self.y,
        }
    }

    /// Returns `(x, y)`.
    #[must_use]
    pub const fn as_tuple(self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Returns `true` when both components are finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

/// A raw three-dimensional coordinate triple.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate3 {
    /// Creates a three-dimensional coordinate triple.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the origin coordinate.
    #[must_use]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the x component.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Returns the y component.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Returns the z component.
    #[must_use]
    pub const fn z(self) -> f64 {
        self.z
    }

    /// Returns the component selected by `axis`.
    #[must_use]
    pub const fn component(self, axis: Axis3) -> f64 {
        match axis {
            Axis3::X => self.x,
            Axis3::Y => self.y,
            Axis3::Z => self.z,
        }
    }

    /// Returns `(x, y, z)`.
    #[must_use]
    pub const fn as_tuple(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Returns `true` when all components are finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::{Axis2, Axis3, Coordinate2, Coordinate3};

    #[test]
    fn exposes_coordinate_components() {
        let coordinate = Coordinate2::new(2.0, 3.0);

        assert_eq!(coordinate.component(Axis2::X), 2.0);
        assert_eq!(coordinate.component(Axis2::Y), 3.0);
        assert_eq!(coordinate.as_tuple(), (2.0, 3.0));
        assert!(coordinate.is_finite());
        assert_eq!(Coordinate2::origin(), Coordinate2::new(0.0, 0.0));
    }

    #[test]
    fn exposes_three_dimensional_components() {
        let coordinate = Coordinate3::new(2.0, 3.0, 5.0);

        assert_eq!(coordinate.component(Axis3::Z), 5.0);
        assert_eq!(coordinate.as_tuple(), (2.0, 3.0, 5.0));
        assert!(coordinate.is_finite());
        assert_eq!(Coordinate3::origin(), Coordinate3::new(0.0, 0.0, 0.0));
    }
}
