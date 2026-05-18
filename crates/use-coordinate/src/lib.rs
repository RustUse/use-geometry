#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// Errors returned by validated geometry constructors and tolerance-aware helpers.
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    /// A geometry component must be finite.
    ///
    /// The type name, component name, and invalid value are included for
    /// diagnostics.
    NonFiniteComponent {
        /// The type that rejected the invalid component.
        type_name: &'static str,
        /// The component that rejected the invalid value.
        component: &'static str,
        /// The invalid component value.
        value: f64,
    },
    /// A circle radius cannot be negative.
    NegativeRadius(f64),
    /// A circle radius must be finite.
    NonFiniteRadius(f64),
    /// A tolerance must be finite.
    NonFiniteTolerance(f64),
    /// A tolerance cannot be negative.
    NegativeTolerance(f64),
    /// Distinct points were required but identical points were supplied.
    IdenticalPoints,
    /// A non-zero direction vector was required.
    ZeroDirectionVector,
    /// An axis-aligned bounding box corner ordering was invalid.
    InvalidBounds {
        /// The minimum x coordinate.
        min_x: f64,
        /// The minimum y coordinate.
        min_y: f64,
        /// The maximum x coordinate.
        max_x: f64,
        /// The maximum y coordinate.
        max_y: f64,
    },
}

impl GeometryError {
    /// Builds a non-finite component error for a named public type and field.
    #[must_use]
    pub const fn non_finite_component(
        type_name: &'static str,
        component: &'static str,
        value: f64,
    ) -> Self {
        Self::NonFiniteComponent {
            type_name,
            component,
            value,
        }
    }

    /// Validates a tolerance used by geometry APIs.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::NonFiniteTolerance`] when `tolerance` is `NaN`
    /// or infinite.
    ///
    /// Returns [`GeometryError::NegativeTolerance`] when `tolerance` is negative.
    pub const fn validate_tolerance(tolerance: f64) -> Result<f64, Self> {
        if !tolerance.is_finite() {
            return Err(Self::NonFiniteTolerance(tolerance));
        }

        if tolerance < 0.0 {
            return Err(Self::NegativeTolerance(tolerance));
        }

        Ok(tolerance)
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteComponent {
                type_name,
                component,
                value,
            } => write!(
                formatter,
                "{type_name} {component} component must be finite, got {value}"
            ),
            Self::NegativeRadius(value) => {
                write!(formatter, "circle radius must be non-negative, got {value}")
            },
            Self::NonFiniteRadius(value) => {
                write!(formatter, "circle radius must be finite, got {value}")
            },
            Self::NonFiniteTolerance(value) => {
                write!(formatter, "tolerance must be finite, got {value}")
            },
            Self::NegativeTolerance(value) => {
                write!(formatter, "tolerance must be non-negative, got {value}")
            },
            Self::IdenticalPoints => write!(formatter, "points must be distinct"),
            Self::ZeroDirectionVector => write!(formatter, "direction vector must be non-zero"),
            Self::InvalidBounds {
                min_x,
                min_y,
                max_x,
                max_y,
            } => write!(
                formatter,
                "aabb min must not exceed max, got min=({min_x}, {min_y}) and max=({max_x}, {max_y})"
            ),
        }
    }
}

impl Error for GeometryError {}

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
    use super::{Axis2, Axis3, Coordinate2, Coordinate3, GeometryError};

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

    #[test]
    fn validates_tolerance_values() {
        assert_eq!(GeometryError::validate_tolerance(0.25), Ok(0.25));
        assert_eq!(
            GeometryError::validate_tolerance(-0.25),
            Err(GeometryError::NegativeTolerance(-0.25))
        );
        assert!(matches!(
            GeometryError::validate_tolerance(f64::NAN),
            Err(GeometryError::NonFiniteTolerance(value)) if value.is_nan()
        ));
    }
}
