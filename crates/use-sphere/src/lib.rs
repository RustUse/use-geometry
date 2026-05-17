#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::f64::consts::PI;

/// A three-dimensional Euclidean sphere represented by radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    /// Creates a sphere with a positive finite radius.
    #[must_use]
    pub const fn new(radius: f64) -> Option<Self> {
        if radius.is_finite() && radius > 0.0 {
            Some(Self { radius })
        } else {
            None
        }
    }

    /// Returns the radius.
    #[must_use]
    pub const fn radius(self) -> f64 {
        self.radius
    }

    /// Returns the diameter.
    #[must_use]
    pub const fn diameter(self) -> f64 {
        self.radius * 2.0
    }

    /// Returns the surface area.
    #[must_use]
    pub fn surface_area(self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    /// Returns the enclosed volume.
    #[must_use]
    pub fn volume(self) -> f64 {
        (4.0 / 3.0) * PI * self.radius * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use super::Sphere;

    fn approx_eq(left: f64, right: f64) -> bool {
        (left - right).abs() < 1.0e-10
    }

    #[test]
    fn computes_sphere_measurements() {
        let sphere = Sphere::new(3.0).expect("valid sphere");

        assert_eq!(sphere.radius(), 3.0);
        assert_eq!(sphere.diameter(), 6.0);
        assert!(approx_eq(sphere.surface_area(), 36.0 * PI));
        assert!(approx_eq(sphere.volume(), 36.0 * PI));
        assert_eq!(Sphere::new(0.0), None);
    }
}
