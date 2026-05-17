#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// Broad projection families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionKind {
    /// Orthographic projection.
    Orthographic,
    /// Perspective projection.
    Perspective,
}

/// A tiny 2D projection descriptor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection2 {
    kind: ProjectionKind,
    scale: f64,
}

impl Projection2 {
    /// Creates a projection with a positive finite scale.
    #[must_use]
    pub const fn new(kind: ProjectionKind, scale: f64) -> Option<Self> {
        if scale.is_finite() && scale > 0.0 {
            Some(Self { kind, scale })
        } else {
            None
        }
    }

    /// Returns the projection kind.
    #[must_use]
    pub const fn kind(self) -> ProjectionKind {
        self.kind
    }

    /// Returns the projection scale.
    #[must_use]
    pub const fn scale(self) -> f64 {
        self.scale
    }

    /// Projects a point onto the x axis and applies the projection scale.
    #[must_use]
    pub fn project_x_axis(self, point: Point2) -> f64 {
        point.x() * self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::{Projection2, ProjectionKind};
    use use_point::Point2;

    #[test]
    fn projects_points_to_axis() {
        let projection = Projection2::new(ProjectionKind::Orthographic, 2.0).expect("valid");

        assert_eq!(projection.kind(), ProjectionKind::Orthographic);
        assert_eq!(projection.scale(), 2.0);
        assert_eq!(projection.project_x_axis(Point2::new(3.0, 4.0)), 6.0);
        assert_eq!(Projection2::new(ProjectionKind::Perspective, 0.0), None);
    }
}
