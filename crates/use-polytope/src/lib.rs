#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A small n-dimensional polytope descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polytope {
    dimension: usize,
    vertex_count: usize,
    facet_count: usize,
}

impl Polytope {
    /// Creates a polytope descriptor with positive dimension and counts.
    #[must_use]
    pub const fn new(dimension: usize, vertex_count: usize, facet_count: usize) -> Option<Self> {
        if dimension > 0 && vertex_count > 0 && facet_count > 0 {
            Some(Self {
                dimension,
                vertex_count,
                facet_count,
            })
        } else {
            None
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the vertex count.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        self.vertex_count
    }

    /// Returns the facet count.
    #[must_use]
    pub const fn facet_count(self) -> usize {
        self.facet_count
    }
}

#[cfg(test)]
mod tests {
    use super::Polytope;

    #[test]
    fn stores_polytope_counts() {
        let polytope = Polytope::new(4, 8, 16).expect("valid polytope");

        assert_eq!(polytope.dimension(), 4);
        assert_eq!(polytope.vertex_count(), 8);
        assert_eq!(polytope.facet_count(), 16);
        assert_eq!(Polytope::new(0, 8, 16), None);
    }
}
