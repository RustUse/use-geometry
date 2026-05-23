#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_cell::Cell;

/// A general geometric complex count summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometricComplex {
    dimension: usize,
    cell_count: usize,
}

impl GeometricComplex {
    /// Creates a geometric complex descriptor.
    #[must_use]
    pub const fn new(dimension: usize, cell_count: usize) -> Option<Self> {
        if cell_count > 0 {
            Some(Self {
                dimension,
                cell_count,
            })
        } else {
            None
        }
    }

    /// Returns the maximum dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the cell count.
    #[must_use]
    pub const fn cell_count(self) -> usize {
        self.cell_count
    }
}

/// A collection of geometric cells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellComplex {
    cells: Vec<Cell>,
}

impl CellComplex {
    /// Creates a cell complex.
    #[must_use]
    pub const fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    /// Returns the cells.
    #[must_use]
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Returns the number of cells.
    #[must_use]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

/// A simplicial complex represented by simplex dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplicialComplex {
    simplex_dimensions: Vec<usize>,
}

impl SimplicialComplex {
    /// Creates a simplicial complex from simplex dimensions.
    #[must_use]
    pub fn new(simplex_dimensions: Vec<usize>) -> Option<Self> {
        if simplex_dimensions.iter().all(|dimension| *dimension > 0) {
            Some(Self { simplex_dimensions })
        } else {
            None
        }
    }

    /// Returns simplex dimensions.
    #[must_use]
    pub fn simplex_dimensions(&self) -> &[usize] {
        &self.simplex_dimensions
    }

    /// Returns the number of simplexes.
    #[must_use]
    pub fn simplex_count(&self) -> usize {
        self.simplex_dimensions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{CellComplex, GeometricComplex, SimplicialComplex};
    use use_cell::Cell;

    #[test]
    fn stores_geometric_complex_metadata() {
        let cell = Cell::new(2, 3).expect("valid cell");
        let cell_complex = CellComplex::new(vec![cell]);
        let geometric = GeometricComplex::new(2, 1).expect("valid complex");
        let simplicial = SimplicialComplex::new(vec![1, 2]).expect("valid simplicial complex");

        assert_eq!(cell_complex.cell_count(), 1);
        assert_eq!(cell_complex.cells(), &[cell]);
        assert_eq!(geometric.dimension(), 2);
        assert_eq!(simplicial.simplex_count(), 2);
        assert_eq!(SimplicialComplex::new(vec![0]), None);
    }
}
