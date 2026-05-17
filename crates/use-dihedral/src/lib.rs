#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_angle::Angle;

/// A dihedral angle stored as an `Angle`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DihedralAngle {
    angle: Angle,
}

impl DihedralAngle {
    /// Creates a dihedral angle from an existing angle.
    #[must_use]
    pub const fn new(angle: Angle) -> Self {
        Self { angle }
    }

    /// Creates a dihedral angle from radians.
    #[must_use]
    pub const fn from_radians(radians: f64) -> Self {
        Self::new(Angle::from_radians(radians))
    }

    /// Creates a dihedral angle from degrees.
    #[must_use]
    pub fn from_degrees(degrees: f64) -> Self {
        Self::new(Angle::from_degrees(degrees))
    }

    /// Returns the underlying angle.
    #[must_use]
    pub const fn angle(self) -> Angle {
        self.angle
    }

    /// Returns the angle in radians.
    #[must_use]
    pub const fn radians(self) -> f64 {
        self.angle.radians()
    }

    /// Returns the angle in degrees.
    #[must_use]
    pub fn degrees(self) -> f64 {
        self.angle.degrees()
    }

    /// Returns the equivalent dihedral angle in `[0, 2pi)`.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self::new(self.angle.normalized())
    }
}

#[cfg(test)]
mod tests {
    use super::DihedralAngle;

    #[test]
    fn converts_and_normalizes_dihedral_angles() {
        let angle = DihedralAngle::from_degrees(450.0).normalized();

        assert_eq!(angle.degrees(), 90.0);
        assert_eq!(
            DihedralAngle::from_radians(core::f64::consts::PI).degrees(),
            180.0
        );
    }
}
