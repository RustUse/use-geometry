#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// An n-sphere represented by its topological dimension and radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hypersphere<const N: usize> {
    radius: f64,
}

impl<const N: usize> Hypersphere<N> {
    /// Creates an n-sphere with a positive finite radius.
    #[must_use]
    pub const fn new(radius: f64) -> Option<Self> {
        if radius.is_finite() && radius > 0.0 {
            Some(Self { radius })
        } else {
            None
        }
    }

    /// Returns the n-sphere dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        N
    }

    /// Returns the ambient Euclidean dimension.
    #[must_use]
    pub const fn ambient_dimension(self) -> usize {
        N + 1
    }

    /// Returns the radius.
    #[must_use]
    pub const fn radius(self) -> f64 {
        self.radius
    }
}

/// A 3-sphere, the boundary of a four-dimensional ball.
pub type ThreeSphere = Hypersphere<3>;

#[cfg(test)]
mod tests {
    use super::{Hypersphere, ThreeSphere};

    #[test]
    fn stores_hypersphere_metadata() {
        let hypersphere = Hypersphere::<4>::new(2.0).expect("valid hypersphere");
        let three_sphere = ThreeSphere::new(1.5).expect("valid 3-sphere");

        assert_eq!(hypersphere.dimension(), 4);
        assert_eq!(hypersphere.ambient_dimension(), 5);
        assert_eq!(three_sphere.dimension(), 3);
        assert_eq!(three_sphere.radius(), 1.5);
        assert_eq!(Hypersphere::<2>::new(f64::NAN), None);
    }
}
