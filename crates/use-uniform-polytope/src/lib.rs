#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Broad uniform polytope families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformPolytopeKind {
    /// Regular polytopes.
    Regular,
    /// Quasiregular polytopes.
    Quasiregular,
    /// Archimedean solids and higher-dimensional analogues.
    Archimedean,
    /// Prismatic uniform families.
    Prismatic,
    /// Other named or not-yet-classified uniform families.
    Other,
}

/// A lightweight uniform polytope descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformPolytope {
    dimension: usize,
    kind: UniformPolytopeKind,
}

impl UniformPolytope {
    /// Creates a uniform polytope descriptor with positive dimension.
    #[must_use]
    pub const fn new(dimension: usize, kind: UniformPolytopeKind) -> Option<Self> {
        if dimension > 0 {
            Some(Self { dimension, kind })
        } else {
            None
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the classification kind.
    #[must_use]
    pub const fn kind(self) -> UniformPolytopeKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{UniformPolytope, UniformPolytopeKind};

    #[test]
    fn stores_uniform_polytope_metadata() {
        let polytope = UniformPolytope::new(3, UniformPolytopeKind::Archimedean).expect("valid");

        assert_eq!(polytope.dimension(), 3);
        assert_eq!(polytope.kind(), UniformPolytopeKind::Archimedean);
        assert_eq!(UniformPolytope::new(0, UniformPolytopeKind::Regular), None);
    }
}
