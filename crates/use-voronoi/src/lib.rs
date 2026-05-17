#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A Voronoi cell represented by its site index and boundary vertices.
#[derive(Debug, Clone, PartialEq)]
pub struct VoronoiCell {
    site_index: usize,
    boundary_vertices: Vec<Point2>,
}

impl VoronoiCell {
    /// Creates a Voronoi cell record.
    #[must_use]
    pub const fn new(site_index: usize, boundary_vertices: Vec<Point2>) -> Self {
        Self {
            site_index,
            boundary_vertices,
        }
    }

    /// Returns the site index.
    #[must_use]
    pub const fn site_index(&self) -> usize {
        self.site_index
    }

    /// Returns the boundary vertices.
    #[must_use]
    pub fn boundary_vertices(&self) -> &[Point2] {
        &self.boundary_vertices
    }
}

/// A Voronoi diagram record.
#[derive(Debug, Clone, PartialEq)]
pub struct VoronoiDiagram {
    cells: Vec<VoronoiCell>,
}

impl VoronoiDiagram {
    /// Creates a Voronoi diagram from cells.
    #[must_use]
    pub const fn new(cells: Vec<VoronoiCell>) -> Self {
        Self { cells }
    }

    /// Returns the cells.
    #[must_use]
    pub fn cells(&self) -> &[VoronoiCell] {
        &self.cells
    }

    /// Returns the number of cells.
    #[must_use]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{VoronoiCell, VoronoiDiagram};
    use use_point::Point2;

    #[test]
    fn stores_voronoi_cells() {
        let cell = VoronoiCell::new(0, vec![Point2::origin()]);
        let diagram = VoronoiDiagram::new(vec![cell]);

        assert_eq!(diagram.cell_count(), 1);
        assert_eq!(diagram.cells()[0].site_index(), 0);
        assert_eq!(diagram.cells()[0].boundary_vertices(), &[Point2::origin()]);
    }
}
