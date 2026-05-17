#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A positive spatial dimension count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dimension(usize);

impl Dimension {
    /// One-dimensional space.
    pub const ONE: Self = Self(1);
    /// Two-dimensional space.
    pub const TWO: Self = Self(2);
    /// Three-dimensional space.
    pub const THREE: Self = Self(3);

    /// Creates a positive dimension count.
    #[must_use]
    pub const fn new(value: usize) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    /// Creates a dimension count without checking whether it is positive.
    #[must_use]
    pub const fn new_unchecked(value: usize) -> Self {
        Self(value)
    }

    /// Returns the numeric dimension count.
    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }

    /// Returns `true` for dimensions commonly used as spatial geometry.
    #[must_use]
    pub const fn is_spatial(self) -> bool {
        self.0 <= 3
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    fn validates_positive_dimensions() {
        assert_eq!(Dimension::new(0), None);
        assert_eq!(Dimension::new(2), Some(Dimension::TWO));
        assert_eq!(Dimension::THREE.value(), 3);
        assert!(Dimension::THREE.is_spatial());
        assert!(!Dimension::new_unchecked(4).is_spatial());
    }
}
