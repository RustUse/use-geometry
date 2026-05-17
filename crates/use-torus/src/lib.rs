#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::f64::consts::PI;

/// A torus represented by major and minor radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Torus {
    major_radius: f64,
    minor_radius: f64,
}

impl Torus {
    /// Creates a torus with positive finite radii.
    #[must_use]
    pub const fn new(major_radius: f64, minor_radius: f64) -> Option<Self> {
        if major_radius.is_finite()
            && minor_radius.is_finite()
            && major_radius > 0.0
            && minor_radius > 0.0
        {
            Some(Self {
                major_radius,
                minor_radius,
            })
        } else {
            None
        }
    }

    /// Returns the major radius.
    #[must_use]
    pub const fn major_radius(self) -> f64 {
        self.major_radius
    }

    /// Returns the minor radius.
    #[must_use]
    pub const fn minor_radius(self) -> f64 {
        self.minor_radius
    }

    /// Returns the surface area.
    #[must_use]
    pub fn surface_area(self) -> f64 {
        4.0 * PI * PI * self.major_radius * self.minor_radius
    }

    /// Returns the enclosed volume.
    #[must_use]
    pub fn volume(self) -> f64 {
        2.0 * PI * PI * self.major_radius * self.minor_radius * self.minor_radius
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use super::Torus;

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn computes_torus_measurements() {
        let torus = Torus::new(3.0, 1.0).expect("valid torus");

        assert_eq!(torus.major_radius(), 3.0);
        assert_eq!(torus.minor_radius(), 1.0);
        assert!(approx_eq(torus.surface_area(), 12.0 * PI * PI));
        assert!(approx_eq(torus.volume(), 6.0 * PI * PI));
        assert_eq!(Torus::new(3.0, 0.0), None);
    }
}
