#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A simple inside, boundary, or outside classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Containment {
    /// Strictly inside the containing value.
    Inside,
    /// On the boundary of the containing value.
    Boundary,
    /// Outside the containing value.
    Outside,
}

impl Containment {
    /// Returns `true` for inside or boundary classifications.
    #[must_use]
    pub const fn is_contained(self) -> bool {
        matches!(self, Self::Inside | Self::Boundary)
    }

    /// Returns `true` when the classification is strictly outside.
    #[must_use]
    pub const fn is_outside(self) -> bool {
        matches!(self, Self::Outside)
    }
}

/// A trait for values that can classify containment of `T`.
pub trait Contains<T> {
    /// Returns the containment classification for `value`.
    fn contains_value(&self, value: &T) -> Containment;
}

#[cfg(test)]
mod tests {
    use super::Containment;

    #[test]
    fn classifies_containment() {
        assert!(Containment::Inside.is_contained());
        assert!(Containment::Boundary.is_contained());
        assert!(!Containment::Outside.is_contained());
        assert!(Containment::Outside.is_outside());
    }
}
