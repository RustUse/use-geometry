#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::f64::consts::{PI, TAU};

/// An angle stored in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    /// A zero angle.
    pub const ZERO: Self = Self { radians: 0.0 };
    /// A half turn.
    pub const HALF_TURN: Self = Self { radians: PI };
    /// A full turn.
    pub const FULL_TURN: Self = Self { radians: TAU };

    /// Creates an angle from radians.
    #[must_use]
    pub const fn from_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an angle from degrees.
    #[must_use]
    pub fn from_degrees(degrees: f64) -> Self {
        Self::from_radians(degrees.to_radians())
    }

    /// Returns the angle in radians.
    #[must_use]
    pub const fn radians(self) -> f64 {
        self.radians
    }

    /// Returns the angle in degrees.
    #[must_use]
    pub fn degrees(self) -> f64 {
        self.radians.to_degrees()
    }

    /// Returns the equivalent angle in `[0, 2pi)`.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self::from_radians(self.radians.rem_euclid(TAU))
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use super::Angle;

    #[test]
    fn converts_degrees_and_radians() {
        assert_eq!(Angle::from_degrees(180.0).radians(), PI);
        assert_eq!(Angle::from_radians(PI).degrees(), 180.0);
        assert_eq!(Angle::ZERO.radians(), 0.0);
        assert_eq!(Angle::HALF_TURN.radians(), PI);
    }

    #[test]
    fn normalizes_angles() {
        assert_eq!(Angle::from_degrees(450.0).normalized().degrees(), 90.0);
    }
}
