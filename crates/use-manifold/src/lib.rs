#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A non-negative manifold dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ManifoldDimension(usize);

impl ManifoldDimension {
    /// Creates a manifold dimension.
    #[must_use]
    pub const fn new(value: usize) -> Option<Self> {
        Some(Self(value))
    }

    /// Returns the numeric dimension.
    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }
}

/// Boundary metadata for a manifold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryKind {
    /// The manifold has no boundary.
    WithoutBoundary,
    /// The manifold has a boundary.
    WithBoundary,
    /// Boundary information is intentionally unspecified.
    Unspecified,
}

/// A lightweight manifold descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Manifold {
    dimension: ManifoldDimension,
    boundary_kind: BoundaryKind,
}

impl Manifold {
    /// Creates a manifold descriptor.
    #[must_use]
    pub const fn new(dimension: ManifoldDimension, boundary_kind: BoundaryKind) -> Self {
        Self {
            dimension,
            boundary_kind,
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub const fn dimension(self) -> ManifoldDimension {
        self.dimension
    }

    /// Returns the boundary kind.
    #[must_use]
    pub const fn boundary_kind(self) -> BoundaryKind {
        self.boundary_kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundaryKind, Manifold, ManifoldDimension};

    #[test]
    fn stores_manifold_metadata() {
        let dimension = ManifoldDimension::new(2).expect("valid dimension");
        let manifold = Manifold::new(dimension, BoundaryKind::WithoutBoundary);

        assert_eq!(dimension.value(), 2);
        assert_eq!(manifold.dimension(), dimension);
        assert_eq!(manifold.boundary_kind(), BoundaryKind::WithoutBoundary);
    }
}
