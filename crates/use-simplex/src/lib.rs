#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_dimension::Dimension;
use use_vector::Vector3;

/// A general simplex descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simplex {
    dimension: Dimension,
}

impl Simplex {
    /// Creates a simplex descriptor for a positive dimension.
    #[must_use]
    pub const fn new(dimension: usize) -> Option<Self> {
        match Dimension::new(dimension) {
            Some(dimension) => Some(Self { dimension }),
            None => None,
        }
    }

    /// Returns the simplex dimension.
    #[must_use]
    pub const fn dimension(self) -> Dimension {
        self.dimension
    }

    /// Returns the number of vertices in this simplex family.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        self.dimension.value() + 1
    }
}

/// A 3-simplex in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tetrahedron {
    vertices: [Vector3; 4],
}

impl Tetrahedron {
    /// Creates a tetrahedron from four vertices.
    #[must_use]
    pub const fn new(vertices: [Vector3; 4]) -> Self {
        Self { vertices }
    }

    /// Returns the vertices.
    #[must_use]
    pub const fn vertices(self) -> [Vector3; 4] {
        self.vertices
    }

    /// Returns the vertex count.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::{Simplex, Tetrahedron};
    use use_vector::Vector3;

    #[test]
    fn counts_simplex_vertices() {
        let simplex = Simplex::new(3).expect("valid simplex");
        let tetrahedron = Tetrahedron::new([
            Vector3::ZERO,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ]);

        assert_eq!(simplex.vertex_count(), 4);
        assert_eq!(tetrahedron.vertex_count(), 4);
        assert_eq!(Simplex::new(0), None);
    }
}
