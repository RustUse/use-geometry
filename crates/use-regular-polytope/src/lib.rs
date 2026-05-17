#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_schlafli::SchlafliSymbol;

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

    /// Returns the Schlafli symbol `{p}`.
    #[must_use]
    pub fn schlafli_symbol(self) -> SchlafliSymbol {
        SchlafliSymbol::polygon(self.side_count).expect("regular polygon side count is valid")
    }
}

/// Two-dimensional regular polytopes are regular polygons.
pub type RegularPolytope2 = RegularPolygon;

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
    /// Returns the common lowercase name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Tetrahedron => "tetrahedron",
            Self::Cube => "cube",
            Self::Octahedron => "octahedron",
            Self::Dodecahedron => "dodecahedron",
            Self::Icosahedron => "icosahedron",
        }
    }

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

    /// Returns the Schlafli symbol.
    #[must_use]
    pub fn schlafli_symbol(self) -> SchlafliSymbol {
        let (p, q) = match self {
            Self::Tetrahedron => (3, 3),
            Self::Cube => (4, 3),
            Self::Octahedron => (3, 4),
            Self::Dodecahedron => (5, 3),
            Self::Icosahedron => (3, 5),
        };

        SchlafliSymbol::polyhedron(p, q).expect("Platonic solid Schlafli entries are valid")
    }
}

/// Three-dimensional regular polytopes are the Platonic solids.
pub type RegularPolytope3 = PlatonicSolid;

/// Marker for the 5-cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiveCell;

/// Marker for the tesseract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tesseract;

/// Marker for the 16-cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixteenCell;

/// Marker for the 24-cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwentyFourCell;

/// Marker for the 600-cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixHundredCell;

/// Marker for the 120-cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneHundredTwentyCell;

/// The six convex regular four-dimensional polytopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularPolytope4 {
    /// The 5-cell.
    FiveCell,
    /// The tesseract.
    Tesseract,
    /// The 16-cell.
    SixteenCell,
    /// The 24-cell.
    TwentyFourCell,
    /// The 600-cell.
    SixHundredCell,
    /// The 120-cell.
    OneHundredTwentyCell,
}

impl RegularPolytope4 {
    /// Returns the common lowercase name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::FiveCell => "5-cell",
            Self::Tesseract => "tesseract",
            Self::SixteenCell => "16-cell",
            Self::TwentyFourCell => "24-cell",
            Self::SixHundredCell => "600-cell",
            Self::OneHundredTwentyCell => "120-cell",
        }
    }

    /// Returns the vertex count.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        match self {
            Self::FiveCell => 5,
            Self::Tesseract => 16,
            Self::SixteenCell => 8,
            Self::TwentyFourCell => 24,
            Self::SixHundredCell => 120,
            Self::OneHundredTwentyCell => 600,
        }
    }

    /// Returns the edge count.
    #[must_use]
    pub const fn edge_count(self) -> usize {
        match self {
            Self::FiveCell => 10,
            Self::Tesseract => 32,
            Self::SixteenCell => 24,
            Self::TwentyFourCell => 96,
            Self::SixHundredCell => 720,
            Self::OneHundredTwentyCell => 1200,
        }
    }

    /// Returns the two-dimensional face count.
    #[must_use]
    pub const fn face_count(self) -> usize {
        match self {
            Self::FiveCell => 10,
            Self::Tesseract => 24,
            Self::SixteenCell => 32,
            Self::TwentyFourCell => 96,
            Self::SixHundredCell => 1200,
            Self::OneHundredTwentyCell => 720,
        }
    }

    /// Returns the three-dimensional cell count.
    #[must_use]
    pub const fn cell_count(self) -> usize {
        match self {
            Self::FiveCell => 5,
            Self::Tesseract => 8,
            Self::SixteenCell => 16,
            Self::TwentyFourCell => 24,
            Self::SixHundredCell => 600,
            Self::OneHundredTwentyCell => 120,
        }
    }

    /// Returns the Schlafli symbol.
    #[must_use]
    pub fn schlafli_symbol(self) -> SchlafliSymbol {
        let (p, q, r) = match self {
            Self::FiveCell => (3, 3, 3),
            Self::Tesseract => (4, 3, 3),
            Self::SixteenCell => (3, 3, 4),
            Self::TwentyFourCell => (3, 4, 3),
            Self::SixHundredCell => (3, 3, 5),
            Self::OneHundredTwentyCell => (5, 3, 3),
        };

        SchlafliSymbol::polychoron(p, q, r).expect("regular 4-polytope entries are valid")
    }
}

impl FiveCell {
    /// Returns the 5-cell family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::FiveCell
    }
}

impl Tesseract {
    /// Returns the tesseract family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::Tesseract
    }
}

impl SixteenCell {
    /// Returns the 16-cell family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::SixteenCell
    }
}

impl TwentyFourCell {
    /// Returns the 24-cell family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::TwentyFourCell
    }
}

impl SixHundredCell {
    /// Returns the 600-cell family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::SixHundredCell
    }
}

impl OneHundredTwentyCell {
    /// Returns the 120-cell family enum value.
    #[must_use]
    pub const fn family() -> RegularPolytope4 {
        RegularPolytope4::OneHundredTwentyCell
    }
}

/// A regular polytope descriptor across common dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegularPolytope {
    /// A regular polygon.
    Polygon(RegularPolytope2),
    /// A Platonic solid.
    PlatonicSolid(RegularPolytope3),
    /// A convex regular four-dimensional polytope.
    Polytope4(RegularPolytope4),
}

impl RegularPolytope {
    /// Returns the polytope dimension.
    #[must_use]
    pub const fn dimension(self) -> usize {
        match self {
            Self::Polygon(_) => 2,
            Self::PlatonicSolid(_) => 3,
            Self::Polytope4(_) => 4,
        }
    }

    /// Returns the Schlafli symbol.
    #[must_use]
    pub fn schlafli_symbol(self) -> SchlafliSymbol {
        match self {
            Self::Polygon(polygon) => polygon.schlafli_symbol(),
            Self::PlatonicSolid(solid) => solid.schlafli_symbol(),
            Self::Polytope4(polytope) => polytope.schlafli_symbol(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PlatonicSolid, RegularPolygon, RegularPolytope, RegularPolytope4};

    #[test]
    fn stores_regular_polygon_data() {
        let polygon = RegularPolygon::new(6, 2.0).expect("valid polygon");

        assert_eq!(polygon.side_count(), 6);
        assert_eq!(polygon.circumradius(), 2.0);
        assert_eq!(polygon.schlafli_symbol().to_string(), "{6}");
        assert_eq!(RegularPolygon::new(2, 2.0), None);
    }

    #[test]
    fn exposes_platonic_solid_counts() {
        assert_eq!(PlatonicSolid::Icosahedron.face_count(), 20);
        assert_eq!(PlatonicSolid::Cube.edge_count(), 12);
        assert_eq!(PlatonicSolid::Dodecahedron.vertex_count(), 20);
        assert_eq!(
            PlatonicSolid::Icosahedron.schlafli_symbol().to_string(),
            "{3, 5}"
        );
    }

    #[test]
    fn exposes_four_dimensional_regular_polytope_metadata() {
        assert_eq!(RegularPolytope4::TwentyFourCell.name(), "24-cell");
        assert_eq!(RegularPolytope4::TwentyFourCell.vertex_count(), 24);
        assert_eq!(RegularPolytope4::SixHundredCell.cell_count(), 600);
        assert_eq!(RegularPolytope4::OneHundredTwentyCell.face_count(), 720);
        assert_eq!(
            RegularPolytope4::TwentyFourCell
                .schlafli_symbol()
                .to_string(),
            "{3, 4, 3}"
        );
    }

    #[test]
    fn wraps_regular_polytope_families() {
        let polytope = RegularPolytope::Polytope4(RegularPolytope4::Tesseract);

        assert_eq!(polytope.dimension(), 4);
        assert_eq!(polytope.schlafli_symbol().to_string(), "{4, 3, 3}");
    }
}
