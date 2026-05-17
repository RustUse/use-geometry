#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Triangulation method families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulationMethod {
    /// Ear-clipping polygon triangulation.
    EarClipping,
    /// Delaunay triangulation.
    Delaunay,
    /// Constrained triangulation.
    Constrained,
}

impl TriangulationMethod {
    /// Returns a stable method name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::EarClipping => "ear-clipping",
            Self::Delaunay => "delaunay",
            Self::Constrained => "constrained",
        }
    }
}

/// A 2D triangulation represented by indexed triangles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triangulation2 {
    triangles: Vec<[usize; 3]>,
}

impl Triangulation2 {
    /// Creates a triangulation from indexed triangles.
    #[must_use]
    pub const fn new(triangles: Vec<[usize; 3]>) -> Self {
        Self { triangles }
    }

    /// Returns the indexed triangles.
    #[must_use]
    pub fn triangles(&self) -> &[[usize; 3]] {
        &self.triangles
    }

    /// Returns the number of triangles.
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Triangulation2, TriangulationMethod};

    #[test]
    fn stores_indexed_triangles() {
        let triangulation = Triangulation2::new(vec![[0, 1, 2], [0, 2, 3]]);

        assert_eq!(triangulation.triangle_count(), 2);
        assert_eq!(triangulation.triangles()[0], [0, 1, 2]);
        assert_eq!(TriangulationMethod::EarClipping.name(), "ear-clipping");
    }
}
