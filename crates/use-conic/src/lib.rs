#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_circle::Circle;
use use_point::Point2;

/// The high-level family of a conic section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicKind {
    /// A circle.
    Circle,
    /// An ellipse.
    Ellipse,
    /// A parabola.
    Parabola,
    /// A hyperbola.
    Hyperbola,
}

/// A small conic descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conic {
    kind: ConicKind,
}

impl Conic {
    /// Creates a conic descriptor.
    #[must_use]
    pub const fn new(kind: ConicKind) -> Self {
        Self { kind }
    }

    /// Returns the conic family.
    #[must_use]
    pub const fn kind(self) -> ConicKind {
        self.kind
    }
}

/// An ellipse represented by a center and two positive radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    center: Point2,
    major_radius: f64,
    minor_radius: f64,
}

impl Ellipse {
    /// Creates an ellipse with positive finite radii.
    #[must_use]
    pub fn new(center: Point2, major_radius: f64, minor_radius: f64) -> Option<Self> {
        if major_radius.is_finite()
            && minor_radius.is_finite()
            && major_radius > 0.0
            && minor_radius > 0.0
        {
            Some(Self {
                center,
                major_radius,
                minor_radius,
            })
        } else {
            None
        }
    }

    /// Returns the conic kind.
    #[must_use]
    pub const fn kind(self) -> ConicKind {
        ConicKind::Ellipse
    }

    /// Returns the center.
    #[must_use]
    pub const fn center(self) -> Point2 {
        self.center
    }

    /// Returns the major radius.
    #[must_use]
    pub const fn major_radius(self) -> f64 {
        self.major_radius
    }

    /// Returns the minor radius.
    #[must_use]
    pub const fn minor_radius(self) -> f64 {
        self.minor_radius
    }
}

impl From<Circle> for Ellipse {
    fn from(circle: Circle) -> Self {
        Self {
            center: circle.center(),
            major_radius: circle.radius(),
            minor_radius: circle.radius(),
        }
    }
}

/// A parabola descriptor using a vertex and focal parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Parabola {
    vertex: Point2,
    focal_parameter: f64,
}

impl Parabola {
    /// Creates a parabola with a finite, non-zero focal parameter.
    #[must_use]
    pub fn new(vertex: Point2, focal_parameter: f64) -> Option<Self> {
        if focal_parameter.is_finite() && focal_parameter != 0.0 {
            Some(Self {
                vertex,
                focal_parameter,
            })
        } else {
            None
        }
    }

    /// Returns the conic kind.
    #[must_use]
    pub const fn kind(self) -> ConicKind {
        ConicKind::Parabola
    }

    /// Returns the vertex.
    #[must_use]
    pub const fn vertex(self) -> Point2 {
        self.vertex
    }

    /// Returns the focal parameter.
    #[must_use]
    pub const fn focal_parameter(self) -> f64 {
        self.focal_parameter
    }
}

/// A hyperbola represented by a center and positive transverse/conjugate radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hyperbola {
    center: Point2,
    transverse_radius: f64,
    conjugate_radius: f64,
}

impl Hyperbola {
    /// Creates a hyperbola with positive finite radii.
    #[must_use]
    pub fn new(center: Point2, transverse_radius: f64, conjugate_radius: f64) -> Option<Self> {
        if transverse_radius.is_finite()
            && conjugate_radius.is_finite()
            && transverse_radius > 0.0
            && conjugate_radius > 0.0
        {
            Some(Self {
                center,
                transverse_radius,
                conjugate_radius,
            })
        } else {
            None
        }
    }

    /// Returns the conic kind.
    #[must_use]
    pub const fn kind(self) -> ConicKind {
        ConicKind::Hyperbola
    }

    /// Returns the center.
    #[must_use]
    pub const fn center(self) -> Point2 {
        self.center
    }

    /// Returns the transverse radius.
    #[must_use]
    pub const fn transverse_radius(self) -> f64 {
        self.transverse_radius
    }

    /// Returns the conjugate radius.
    #[must_use]
    pub const fn conjugate_radius(self) -> f64 {
        self.conjugate_radius
    }
}

#[cfg(test)]
mod tests {
    use super::{Conic, ConicKind, Ellipse, Hyperbola, Parabola};
    use use_circle::Circle;
    use use_point::Point2;

    #[test]
    fn creates_conic_descriptors() {
        assert_eq!(Conic::new(ConicKind::Ellipse).kind(), ConicKind::Ellipse);
    }

    #[test]
    fn creates_named_conics() {
        let ellipse = Ellipse::new(Point2::origin(), 4.0, 2.0).expect("valid ellipse");
        let parabola = Parabola::new(Point2::origin(), 1.0).expect("valid parabola");
        let hyperbola = Hyperbola::new(Point2::origin(), 3.0, 2.0).expect("valid hyperbola");
        let circle = Circle::try_new(Point2::origin(), 2.0).expect("valid circle");
        let circle_as_ellipse = Ellipse::from(circle);

        assert_eq!(ellipse.kind(), ConicKind::Ellipse);
        assert_eq!(ellipse.major_radius(), 4.0);
        assert_eq!(parabola.kind(), ConicKind::Parabola);
        assert_eq!(hyperbola.kind(), ConicKind::Hyperbola);
        assert_eq!(circle_as_ellipse.minor_radius(), 2.0);
        assert_eq!(Ellipse::new(Point2::origin(), 0.0, 2.0), None);
    }
}
