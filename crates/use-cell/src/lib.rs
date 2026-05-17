#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A cell in a cell complex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    dimension: usize,
    boundary_count: usize,
}

impl Cell {
    /// Creates a cell descriptor.
    #[must_use]
    pub const fn new(dimension: usize, boundary_count: usize) -> Option<Self> {
        if dimension > 0 {
            Some(Self {
                dimension,
                boundary_count,
            })
        } else {
            None
        }
    }

    /// Returns the cell dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the number of boundary elements.
    #[must_use]
    pub const fn boundary_count(self) -> usize {
        self.boundary_count
    }
}

/// A small collection of cells.
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

#[cfg(test)]
mod tests {
    use super::{Cell, CellComplex};

    #[test]
    fn stores_cell_complexes() {
        let cell = Cell::new(2, 4).expect("valid cell");
        let complex = CellComplex::new(vec![cell]);

        assert_eq!(cell.dimension(), 2);
        assert_eq!(cell.boundary_count(), 4);
        assert_eq!(complex.cell_count(), 1);
        assert_eq!(complex.cells(), &[cell]);
    }
}
