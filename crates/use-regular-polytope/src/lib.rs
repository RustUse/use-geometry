#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A minimal Schläfli symbol descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchlafliSymbol {
    p: usize,
    q: Option<usize>,
}

impl SchlafliSymbol {
    /// Creates a polygon symbol `{p}`.
    #[must_use]
    pub const fn polygon(p: usize) -> Option<Self> {
        if p >= 3 {
            Some(Self { p, q: None })
        } else {
            None
        }
    }

    /// Creates a polyhedron symbol `{p, q}`.
    #[must_use]
    pub const fn polyhedron(p: usize, q: usize) -> Option<Self> {
        if p >= 3 && q >= 3 {
            Some(Self { p, q: Some(q) })
        } else {
            None
        }
    }

    /// Returns the first symbol entry.
    #[must_use]
    pub const fn p(self) -> usize {
        self.p
    }

    /// Returns the optional second symbol entry.
    #[must_use]
    pub const fn q(self) -> Option<usize> {
        self.q
    }
}

/// A regular polygon descriptor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegularPolygon {
    side_count: usize,
    circumradius: f64,
}

impl RegularPolygon {
    /// Creates a regular polygon with at least three sides and positive finite circumradius.
    #[must_use]
    pub const fn new(side_count: usize, circumradius: f64) -> Option<Self> {
        if side_count >= 3 && circumradius.is_finite() && circumradius > 0.0 {
            Some(Self {
                side_count,
                circumradius,
            })
        } else {
            None
        }
    }

    /// Returns the side count.
    #[must_use]
    pub const fn side_count(self) -> usize {
        self.side_count
    }

    /// Returns the circumradius.
    #[must_use]
    pub const fn circumradius(self) -> f64 {
        self.circumradius
    }
}

/// The five Platonic solids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatonicSolid {
    /// Four triangular faces.
    Tetrahedron,
    /// Six square faces.
    Cube,
    /// Eight triangular faces.
    Octahedron,
    /// Twelve pentagonal faces.
    Dodecahedron,
    /// Twenty triangular faces.
    Icosahedron,
}

impl PlatonicSolid {
    /// Returns the number of faces.
    #[must_use]
    pub const fn face_count(self) -> usize {
        match self {
            Self::Tetrahedron => 4,
            Self::Cube => 6,
            Self::Octahedron => 8,
            Self::Dodecahedron => 12,
            Self::Icosahedron => 20,
        }
    }

    /// Returns the number of edges.
    #[must_use]
    pub const fn edge_count(self) -> usize {
        match self {
            Self::Tetrahedron => 6,
            Self::Cube => 12,
            Self::Octahedron => 12,
            Self::Dodecahedron => 30,
            Self::Icosahedron => 30,
        }
    }

    /// Returns the number of vertices.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        match self {
            Self::Tetrahedron => 4,
            Self::Cube => 8,
            Self::Octahedron => 6,
            Self::Dodecahedron => 20,
            Self::Icosahedron => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, RegularPolygon, SchlafliSymbol};

    #[test]
    fn stores_regular_polygon_data() {
        let polygon = RegularPolygon::new(6, 2.0).expect("valid polygon");

        assert_eq!(polygon.side_count(), 6);
        assert_eq!(polygon.circumradius(), 2.0);
        assert_eq!(RegularPolygon::new(2, 2.0), None);
    }

    #[test]
    fn exposes_platonic_solid_counts() {
        assert_eq!(PlatonicSolid::Icosahedron.face_count(), 20);
        assert_eq!(PlatonicSolid::Cube.edge_count(), 12);
        assert_eq!(PlatonicSolid::Dodecahedron.vertex_count(), 20);
        assert_eq!(
            SchlafliSymbol::polyhedron(3, 5).expect("valid").q(),
            Some(5)
        );
    }
}
