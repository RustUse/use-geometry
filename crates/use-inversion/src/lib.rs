#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_geometry_core::GeometryError;
use use_point::Point2;

/// A planar geometric inversion represented by center and radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inversion {
    center: Point2,
    radius: f64,
}

impl Inversion {
    /// Creates an inversion with a finite center and positive finite radius.
    ///
    /// # Errors
    ///
    /// Returns a [`GeometryError`] when the center is non-finite or the radius is invalid.
    pub fn try_new(center: Point2, radius: f64) -> Result<Self, GeometryError> {
        let center = center.validate()?;

        if !radius.is_finite() {
            return Err(GeometryError::NonFiniteRadius(radius));
        }

        if radius <= 0.0 {
            return Err(GeometryError::NegativeRadius(radius));
        }

        Ok(Self { center, radius })
    }

    /// Returns the inversion center.
    #[must_use]
    pub const fn center(self) -> Point2 {
        self.center
    }

    /// Returns the inversion radius.
    #[must_use]
    pub const fn radius(self) -> f64 {
        self.radius
    }

    /// Returns the squared inversion radius.
    #[must_use]
    pub const fn radius_squared(self) -> f64 {
        self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::Inversion;
    use use_geometry_core::GeometryError;
    use use_point::Point2;

    #[test]
    fn validates_inversion_radius() {
        let inversion = Inversion::try_new(Point2::origin(), 2.0).expect("valid inversion");

        assert_eq!(inversion.center(), Point2::origin());
        assert_eq!(inversion.radius(), 2.0);
        assert_eq!(inversion.radius_squared(), 4.0);
        assert_eq!(
            Inversion::try_new(Point2::origin(), 0.0),
            Err(GeometryError::NegativeRadius(0.0))
        );
    }
}
