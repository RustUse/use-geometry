#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A Delaunay triangulation represented by indexed triangles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelaunayTriangulation {
    triangles: Vec<[usize; 3]>,
}

impl DelaunayTriangulation {
    /// Creates a Delaunay triangulation record.
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

    /// Returns a conservative upper bound for the number of triangle edges.
    #[must_use]
    pub fn edge_count_upper_bound(&self) -> usize {
        self.triangles.len() * 3
    }
}

#[cfg(test)]
mod tests {
    use super::DelaunayTriangulation;

    #[test]
    fn stores_delaunay_triangles() {
        let triangulation = DelaunayTriangulation::new(vec![[0, 1, 2]]);

        assert_eq!(triangulation.triangle_count(), 1);
        assert_eq!(triangulation.edge_count_upper_bound(), 3);
        assert_eq!(triangulation.triangles()[0], [0, 1, 2]);
    }
}
