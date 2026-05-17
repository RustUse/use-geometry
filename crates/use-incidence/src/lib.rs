#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A point-line incidence pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IncidencePair {
    point: usize,
    line: usize,
}

impl IncidencePair {
    /// Creates an incidence pair.
    #[must_use]
    pub const fn new(point: usize, line: usize) -> Self {
        Self { point, line }
    }

    /// Returns the point index.
    #[must_use]
    pub const fn point(self) -> usize {
        self.point
    }

    /// Returns the line index.
    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }
}

/// A dense incidence matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidenceMatrix {
    point_count: usize,
    line_count: usize,
    entries: Vec<bool>,
}

impl IncidenceMatrix {
    /// Creates a dense incidence matrix when the entry count matches `point_count * line_count`.
    #[must_use]
    pub fn new(point_count: usize, line_count: usize, entries: Vec<bool>) -> Option<Self> {
        if point_count > 0 && line_count > 0 && entries.len() == point_count * line_count {
            Some(Self {
                point_count,
                line_count,
                entries,
            })
        } else {
            None
        }
    }

    /// Returns the point count.
    #[must_use]
    pub const fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns the line count.
    #[must_use]
    pub const fn line_count(&self) -> usize {
        self.line_count
    }

    /// Returns whether a point is incident with a line.
    #[must_use]
    pub fn is_incident(&self, point: usize, line: usize) -> Option<bool> {
        if point < self.point_count && line < self.line_count {
            Some(self.entries[(point * self.line_count) + line])
        } else {
            None
        }
    }
}

/// A sparse incidence structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidenceStructure {
    point_count: usize,
    line_count: usize,
    pairs: Vec<IncidencePair>,
}

impl IncidenceStructure {
    /// Creates an incidence structure when all pairs are in range.
    #[must_use]
    pub fn new(point_count: usize, line_count: usize, pairs: Vec<IncidencePair>) -> Option<Self> {
        if point_count > 0
            && line_count > 0
            && pairs
                .iter()
                .all(|pair| pair.point() < point_count && pair.line() < line_count)
        {
            Some(Self {
                point_count,
                line_count,
                pairs,
            })
        } else {
            None
        }
    }

    /// Returns the point count.
    #[must_use]
    pub const fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns the line count.
    #[must_use]
    pub const fn line_count(&self) -> usize {
        self.line_count
    }

    /// Returns the incidence pairs.
    #[must_use]
    pub fn pairs(&self) -> &[IncidencePair] {
        &self.pairs
    }

    /// Returns the incidence pair count.
    #[must_use]
    pub fn incidence_count(&self) -> usize {
        self.pairs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{IncidenceMatrix, IncidencePair, IncidenceStructure};

    #[test]
    fn validates_incidence_structures() {
        let pair = IncidencePair::new(1, 0);
        let structure = IncidenceStructure::new(2, 1, vec![pair]).expect("valid structure");
        let matrix = IncidenceMatrix::new(2, 1, vec![false, true]).expect("valid matrix");

        assert_eq!(structure.point_count(), 2);
        assert_eq!(structure.line_count(), 1);
        assert_eq!(structure.incidence_count(), 1);
        assert_eq!(matrix.is_incident(1, 0), Some(true));
        assert_eq!(IncidenceStructure::new(1, 1, vec![pair]), None);
    }
}
