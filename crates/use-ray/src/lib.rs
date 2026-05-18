#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_coordinate::GeometryError;
use use_point::Point2;
use use_vector::Vector2;

fn validate_direction(direction: Vector2) -> Result<Vector2, GeometryError> {
    if !direction.x.is_finite() {
        return Err(GeometryError::non_finite_component(
            "Vector2",
            "x",
            direction.x,
        ));
    }

    if !direction.y.is_finite() {
        return Err(GeometryError::non_finite_component(
            "Vector2",
            "y",
            direction.y,
        ));
    }

    if direction.magnitude_squared() == 0.0 {
        return Err(GeometryError::ZeroDirectionVector);
    }

    Ok(direction)
}

/// A half-infinite 2D ray.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray2 {
    origin: Point2,
    direction: Vector2,
}

impl Ray2 {
    /// Creates a ray without validation.
    #[must_use]
    pub const fn new(origin: Point2, direction: Vector2) -> Self {
        Self { origin, direction }
    }

    /// Creates a ray with a finite origin and finite non-zero direction.
    ///
    /// # Errors
    ///
    /// Returns a [`GeometryError`] when the origin is non-finite or the direction is invalid.
    pub fn try_new(origin: Point2, direction: Vector2) -> Result<Self, GeometryError> {
        Ok(Self::new(
            origin.validate()?,
            validate_direction(direction)?,
        ))
    }

    /// Returns the ray origin.
    #[must_use]
    pub const fn origin(self) -> Point2 {
        self.origin
    }

    /// Returns the ray direction.
    #[must_use]
    pub const fn direction(self) -> Vector2 {
        self.direction
    }

    /// Returns the point at parameter `t` along the ray.
    #[must_use]
    pub fn point_at(self, t: f64) -> Point2 {
        self.origin + self.direction.scale(t)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray2;
    use use_coordinate::GeometryError;
    use use_point::Point2;
    use use_vector::Vector2;

    #[test]
    fn constructs_and_samples_rays() {
        let ray = Ray2::try_new(Point2::new(1.0, 2.0), Vector2::new(3.0, 0.0)).expect("valid ray");

        assert_eq!(ray.origin(), Point2::new(1.0, 2.0));
        assert_eq!(ray.direction(), Vector2::new(3.0, 0.0));
        assert_eq!(ray.point_at(2.0), Point2::new(7.0, 2.0));
    }

    #[test]
    fn rejects_zero_direction() {
        assert_eq!(
            Ray2::try_new(Point2::origin(), Vector2::ZERO),
            Err(GeometryError::ZeroDirectionVector)
        );
    }
}
