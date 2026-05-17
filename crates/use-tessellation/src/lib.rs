#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Broad tessellation families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TessellationKind {
    /// Regular tessellation.
    Regular,
    /// Semi-regular tessellation.
    SemiRegular,
    /// Irregular tessellation.
    Irregular,
}

/// A tessellation summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tessellation {
    kind: TessellationKind,
    tile_count: usize,
}

impl Tessellation {
    /// Creates a tessellation with at least one tile.
    #[must_use]
    pub const fn new(kind: TessellationKind, tile_count: usize) -> Option<Self> {
        if tile_count > 0 {
            Some(Self { kind, tile_count })
        } else {
            None
        }
    }

    /// Returns the tessellation kind.
    #[must_use]
    pub const fn kind(self) -> TessellationKind {
        self.kind
    }

    /// Returns the tile count.
    #[must_use]
    pub const fn tile_count(self) -> usize {
        self.tile_count
    }
}

/// A tessellation tile identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileId(pub usize);

#[cfg(test)]
mod tests {
    use super::{Tessellation, TessellationKind, TileId};

    #[test]
    fn stores_tessellation_counts() {
        let tessellation = Tessellation::new(TessellationKind::Regular, 12).expect("valid");

        assert_eq!(tessellation.kind(), TessellationKind::Regular);
        assert_eq!(tessellation.tile_count(), 12);
        assert_eq!(TileId(3).0, 3);
    }
}
