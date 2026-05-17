#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_angle::Angle;

/// A geometric translation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Translation {
    x: f64,
    y: f64,
    z: f64,
}

impl Translation {
    /// Creates a translation.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the x offset.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Returns the y offset.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Returns the z offset.
    #[must_use]
    pub const fn z(self) -> f64 {
        self.z
    }
}

/// A geometric rotation represented by one angle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    angle: Angle,
}

impl Rotation {
    /// Creates a rotation.
    #[must_use]
    pub const fn new(angle: Angle) -> Self {
        Self { angle }
    }

    /// Returns the rotation angle.
    #[must_use]
    pub const fn angle(self) -> Angle {
        self.angle
    }
}

/// A geometric scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

impl Scale {
    /// Creates a scale.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the x scale.
    #[must_use]
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Returns the y scale.
    #[must_use]
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Returns the z scale.
    #[must_use]
    pub const fn z(self) -> f64 {
        self.z
    }
}

/// A homogeneous 2D transform matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2 {
    matrix: [[f64; 3]; 3],
}

impl Transform2 {
    /// Creates a transform from a 3x3 homogeneous matrix.
    #[must_use]
    pub const fn new(matrix: [[f64; 3]; 3]) -> Self {
        Self { matrix }
    }

    /// Returns the identity transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    /// Creates a 2D translation transform.
    #[must_use]
    pub const fn translation(translation: Translation) -> Self {
        Self::new([
            [1.0, 0.0, translation.x()],
            [0.0, 1.0, translation.y()],
            [0.0, 0.0, 1.0],
        ])
    }

    /// Creates a 2D scale transform.
    #[must_use]
    pub const fn scale(scale: Scale) -> Self {
        Self::new([
            [scale.x(), 0.0, 0.0],
            [0.0, scale.y(), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    /// Creates a 2D rotation transform.
    #[must_use]
    pub fn rotation(rotation: Rotation) -> Self {
        let sin = rotation.angle().radians().sin();
        let cos = rotation.angle().radians().cos();
        Self::new([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]])
    }

    /// Returns the underlying matrix.
    #[must_use]
    pub const fn matrix(self) -> [[f64; 3]; 3] {
        self.matrix
    }

    /// Applies the transform to a 2D point tuple.
    #[must_use]
    pub fn apply_point(self, point: (f64, f64)) -> (f64, f64) {
        let x = self.matrix[0][0].mul_add(point.0, self.matrix[0][1] * point.1) + self.matrix[0][2];
        let y = self.matrix[1][0].mul_add(point.0, self.matrix[1][1] * point.1) + self.matrix[1][2];
        (x, y)
    }
}

/// A homogeneous 3D transform matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3 {
    matrix: [[f64; 4]; 4],
}

impl Transform3 {
    /// Creates a transform from a 4x4 homogeneous matrix.
    #[must_use]
    pub const fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { matrix }
    }

    /// Returns the identity transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns the underlying matrix.
    #[must_use]
    pub const fn matrix(self) -> [[f64; 4]; 4] {
        self.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::{Rotation, Scale, Transform2, Transform3, Translation};
    use use_angle::Angle;

    #[test]
    fn translates_points() {
        let transform = Transform2::translation(Translation::new(2.0, 3.0, 0.0));

        assert_eq!(transform.apply_point((1.0, 1.0)), (3.0, 4.0));
    }

    #[test]
    fn scales_points() {
        let transform = Transform2::scale(Scale::new(2.0, 3.0, 1.0));

        assert_eq!(transform.apply_point((2.0, 3.0)), (4.0, 9.0));
        assert_eq!(Transform3::identity().matrix()[3][3], 1.0);
    }

    #[test]
    fn rotates_points() {
        let transform = Transform2::rotation(Rotation::new(Angle::from_degrees(90.0)));
        let rotated = transform.apply_point((1.0, 0.0));

        assert!(rotated.0.abs() < 1.0e-10);
        assert!((rotated.1 - 1.0).abs() < 1.0e-10);
    }
}
