#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_archimedean::ArchimedeanSolid;

/// The thirteen Catalan solids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalanSolid {
    /// Triakis tetrahedron.
    TriakisTetrahedron,
    /// Rhombic dodecahedron.
    RhombicDodecahedron,
    /// Triakis octahedron.
    TriakisOctahedron,
    /// Tetrakis hexahedron.
    TetrakisHexahedron,
    /// Deltoidal icositetrahedron.
    DeltoidalIcositetrahedron,
    /// Disdyakis dodecahedron.
    DisdyakisDodecahedron,
    /// Pentagonal icositetrahedron.
    PentagonalIcositetrahedron,
    /// Rhombic triacontahedron.
    RhombicTriacontahedron,
    /// Triakis icosahedron.
    TriakisIcosahedron,
    /// Pentakis dodecahedron.
    PentakisDodecahedron,
    /// Deltoidal hexecontahedron.
    DeltoidalHexecontahedron,
    /// Disdyakis triacontahedron.
    DisdyakisTriacontahedron,
    /// Pentagonal hexecontahedron.
    PentagonalHexecontahedron,
}

impl CatalanSolid {
    /// Returns the common lowercase name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::TriakisTetrahedron => "triakis tetrahedron",
            Self::RhombicDodecahedron => "rhombic dodecahedron",
            Self::TriakisOctahedron => "triakis octahedron",
            Self::TetrakisHexahedron => "tetrakis hexahedron",
            Self::DeltoidalIcositetrahedron => "deltoidal icositetrahedron",
            Self::DisdyakisDodecahedron => "disdyakis dodecahedron",
            Self::PentagonalIcositetrahedron => "pentagonal icositetrahedron",
            Self::RhombicTriacontahedron => "rhombic triacontahedron",
            Self::TriakisIcosahedron => "triakis icosahedron",
            Self::PentakisDodecahedron => "pentakis dodecahedron",
            Self::DeltoidalHexecontahedron => "deltoidal hexecontahedron",
            Self::DisdyakisTriacontahedron => "disdyakis triacontahedron",
            Self::PentagonalHexecontahedron => "pentagonal hexecontahedron",
        }
    }

    /// Returns the Archimedean solid dual to this Catalan solid.
    #[must_use]
    pub const fn dual_archimedean(self) -> ArchimedeanSolid {
        match self {
            Self::TriakisTetrahedron => ArchimedeanSolid::TruncatedTetrahedron,
            Self::RhombicDodecahedron => ArchimedeanSolid::Cuboctahedron,
            Self::TriakisOctahedron => ArchimedeanSolid::TruncatedCube,
            Self::TetrakisHexahedron => ArchimedeanSolid::TruncatedOctahedron,
            Self::DeltoidalIcositetrahedron => ArchimedeanSolid::Rhombicuboctahedron,
            Self::DisdyakisDodecahedron => ArchimedeanSolid::TruncatedCuboctahedron,
            Self::PentagonalIcositetrahedron => ArchimedeanSolid::SnubCube,
            Self::RhombicTriacontahedron => ArchimedeanSolid::Icosidodecahedron,
            Self::TriakisIcosahedron => ArchimedeanSolid::TruncatedDodecahedron,
            Self::PentakisDodecahedron => ArchimedeanSolid::TruncatedIcosahedron,
            Self::DeltoidalHexecontahedron => ArchimedeanSolid::Rhombicosidodecahedron,
            Self::DisdyakisTriacontahedron => ArchimedeanSolid::TruncatedIcosidodecahedron,
            Self::PentagonalHexecontahedron => ArchimedeanSolid::SnubDodecahedron,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CatalanSolid;
    use use_archimedean::ArchimedeanSolid;

    #[test]
    fn exposes_dual_archimedean_metadata() {
        let solid = CatalanSolid::RhombicDodecahedron;

        assert_eq!(solid.name(), "rhombic dodecahedron");
        assert_eq!(solid.dual_archimedean(), ArchimedeanSolid::Cuboctahedron);
    }
}
