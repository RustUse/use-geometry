#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_geometry_core::GeometryError;
use use_vector::Vector3;

fn validate_vector3(name: &'static str, vector: Vector3) -> Result<Vector3, GeometryError> {
    if !vector.x.is_finite() {
        return Err(GeometryError::non_finite_component(name, "x", vector.x));
    }
    if !vector.y.is_finite() {
        return Err(GeometryError::non_finite_component(name, "y", vector.y));
    }
    if !vector.z.is_finite() {
        return Err(GeometryError::non_finite_component(name, "z", vector.z));
    }
    Ok(vector)
}

/// A 3D plane represented by `normal.dot(point) + offset = 0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane3 {
    normal: Vector3,
    offset: f64,
}

impl Plane3 {
    /// Creates a validated plane from a finite non-zero normal and finite offset.
    ///
    /// # Errors
    ///
    /// Returns a [`GeometryError`] when the normal or offset is invalid.
    pub fn try_new(normal: Vector3, offset: f64) -> Result<Self, GeometryError> {
        let normal = validate_vector3("Plane3", normal)?;
        if normal.magnitude_squared() == 0.0 {
            return Err(GeometryError::ZeroDirectionVector);
        }
        if !offset.is_finite() {
            return Err(GeometryError::non_finite_component(
                "Plane3", "offset", offset,
            ));
        }
        Ok(Self { normal, offset })
    }

    /// Returns the plane normal.
    #[must_use]
    pub const fn normal(self) -> Vector3 {
        self.normal
    }

    /// Returns the plane offset.
    #[must_use]
    pub const fn offset(self) -> f64 {
        self.offset
    }

    /// Evaluates the plane equation at `point`.
    #[must_use]
    pub fn evaluate(self, point: Vector3) -> f64 {
        self.normal.dot(point) + self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::Plane3;
    use use_geometry_core::GeometryError;
    use use_vector::Vector3;

    #[test]
    fn evaluates_plane_equation() {
        let plane = Plane3::try_new(Vector3::new(0.0, 0.0, 1.0), -2.0).expect("valid plane");

        assert_eq!(plane.evaluate(Vector3::new(0.0, 0.0, 2.0)), 0.0);
        assert_eq!(plane.normal(), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(plane.offset(), -2.0);
    }

    #[test]
    fn rejects_zero_normals() {
        assert_eq!(
            Plane3::try_new(Vector3::ZERO, 0.0),
            Err(GeometryError::ZeroDirectionVector)
        );
    }
}
