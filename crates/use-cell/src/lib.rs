#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A cell used by geometric complex crates.
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

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn stores_cell_metadata() {
        let cell = Cell::new(2, 4).expect("valid cell");

        assert_eq!(cell.dimension(), 2);
        assert_eq!(cell.boundary_count(), 4);
        assert_eq!(Cell::new(0, 1), None);
    }
}
