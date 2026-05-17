#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// The thirteen Archimedean solids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchimedeanSolid {
    /// Truncated tetrahedron.
    TruncatedTetrahedron,
    /// Cuboctahedron.
    Cuboctahedron,
    /// Truncated cube.
    TruncatedCube,
    /// Truncated octahedron.
    TruncatedOctahedron,
    /// Rhombicuboctahedron.
    Rhombicuboctahedron,
    /// Truncated cuboctahedron.
    TruncatedCuboctahedron,
    /// Snub cube.
    SnubCube,
    /// Icosidodecahedron.
    Icosidodecahedron,
    /// Truncated dodecahedron.
    TruncatedDodecahedron,
    /// Truncated icosahedron.
    TruncatedIcosahedron,
    /// Rhombicosidodecahedron.
    Rhombicosidodecahedron,
    /// Truncated icosidodecahedron.
    TruncatedIcosidodecahedron,
    /// Snub dodecahedron.
    SnubDodecahedron,
}

impl ArchimedeanSolid {
    /// Returns the common lowercase name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::TruncatedTetrahedron => "truncated tetrahedron",
            Self::Cuboctahedron => "cuboctahedron",
            Self::TruncatedCube => "truncated cube",
            Self::TruncatedOctahedron => "truncated octahedron",
            Self::Rhombicuboctahedron => "rhombicuboctahedron",
            Self::TruncatedCuboctahedron => "truncated cuboctahedron",
            Self::SnubCube => "snub cube",
            Self::Icosidodecahedron => "icosidodecahedron",
            Self::TruncatedDodecahedron => "truncated dodecahedron",
            Self::TruncatedIcosahedron => "truncated icosahedron",
            Self::Rhombicosidodecahedron => "rhombicosidodecahedron",
            Self::TruncatedIcosidodecahedron => "truncated icosidodecahedron",
            Self::SnubDodecahedron => "snub dodecahedron",
        }
    }

    /// Returns a compact vertex face-configuration string.
    #[must_use]
    pub const fn face_configuration(self) -> &'static str {
        match self {
            Self::TruncatedTetrahedron => "3.6.6",
            Self::Cuboctahedron => "3.4.3.4",
            Self::TruncatedCube => "3.8.8",
            Self::TruncatedOctahedron => "4.6.6",
            Self::Rhombicuboctahedron => "3.4.4.4",
            Self::TruncatedCuboctahedron => "4.6.8",
            Self::SnubCube => "3.3.3.3.4",
            Self::Icosidodecahedron => "3.5.3.5",
            Self::TruncatedDodecahedron => "3.10.10",
            Self::TruncatedIcosahedron => "5.6.6",
            Self::Rhombicosidodecahedron => "3.4.5.4",
            Self::TruncatedIcosidodecahedron => "4.6.10",
            Self::SnubDodecahedron => "3.3.3.3.5",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArchimedeanSolid;

    #[test]
    fn exposes_cuboctahedron_metadata() {
        let solid = ArchimedeanSolid::Cuboctahedron;

        assert_eq!(solid.name(), "cuboctahedron");
        assert_eq!(solid.face_configuration(), "3.4.3.4");
        assert_eq!(
            ArchimedeanSolid::TruncatedIcosahedron.face_configuration(),
            "5.6.6"
        );
    }
}
