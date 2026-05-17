#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Broad surface families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    /// A parametric surface.
    Parametric,
    /// An implicit surface.
    Implicit,
    /// A mesh-backed surface.
    Mesh,
}

/// A rectangular parameter-domain surface patch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfacePatch {
    kind: SurfaceKind,
    u_extent: f64,
    v_extent: f64,
}

impl SurfacePatch {
    /// Creates a surface patch with positive finite parameter extents.
    #[must_use]
    pub const fn new(kind: SurfaceKind, u_extent: f64, v_extent: f64) -> Option<Self> {
        if u_extent.is_finite() && v_extent.is_finite() && u_extent > 0.0 && v_extent > 0.0 {
            Some(Self {
                kind,
                u_extent,
                v_extent,
            })
        } else {
            None
        }
    }

    /// Returns the surface kind.
    #[must_use]
    pub const fn kind(self) -> SurfaceKind {
        self.kind
    }

    /// Returns the parameter-domain area.
    #[must_use]
    pub const fn parameter_area(self) -> f64 {
        self.u_extent * self.v_extent
    }
}

#[cfg(test)]
mod tests {
    use super::{SurfaceKind, SurfacePatch};

    #[test]
    fn stores_surface_patch_data() {
        let patch = SurfacePatch::new(SurfaceKind::Parametric, 2.0, 3.0).expect("valid patch");

        assert_eq!(patch.kind(), SurfaceKind::Parametric);
        assert_eq!(patch.parameter_area(), 6.0);
        assert_eq!(SurfacePatch::new(SurfaceKind::Mesh, 0.0, 3.0), None);
    }
}
