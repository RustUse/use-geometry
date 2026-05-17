#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

fn valid_homogeneous_coordinates(coordinates: &[f64]) -> bool {
    !coordinates.is_empty()
        && coordinates.iter().all(|coordinate| coordinate.is_finite())
        && coordinates.iter().any(|coordinate| *coordinate != 0.0)
}

/// A projective point represented by homogeneous coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectivePoint {
    coordinates: Vec<f64>,
}

impl ProjectivePoint {
    /// Creates a projective point from nonzero finite homogeneous coordinates.
    #[must_use]
    pub fn new(coordinates: Vec<f64>) -> Option<Self> {
        if valid_homogeneous_coordinates(&coordinates) {
            Some(Self { coordinates })
        } else {
            None
        }
    }

    /// Returns the homogeneous coordinates.
    #[must_use]
    pub fn coordinates(&self) -> &[f64] {
        &self.coordinates
    }

    /// Returns the projective dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.coordinates.len() - 1
    }
}

/// A projective line represented by homogeneous coefficients.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectiveLine {
    coefficients: Vec<f64>,
}

impl ProjectiveLine {
    /// Creates a projective line from nonzero finite homogeneous coefficients.
    #[must_use]
    pub fn new(coefficients: Vec<f64>) -> Option<Self> {
        if valid_homogeneous_coordinates(&coefficients) {
            Some(Self { coefficients })
        } else {
            None
        }
    }

    /// Returns the line coefficients.
    #[must_use]
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }
}

/// A projective plane descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectivePlane {
    order: Option<usize>,
}

impl ProjectivePlane {
    /// Creates a projective plane descriptor with optional finite order.
    #[must_use]
    pub const fn new(order: Option<usize>) -> Self {
        Self { order }
    }

    /// Returns the finite order, when known.
    #[must_use]
    pub const fn order(self) -> Option<usize> {
        self.order
    }
}

#[cfg(test)]
mod tests {
    use super::{ProjectiveLine, ProjectivePlane, ProjectivePoint};

    #[test]
    fn validates_projective_coordinates() {
        let point = ProjectivePoint::new(vec![1.0, 2.0, 1.0]).expect("valid point");
        let line = ProjectiveLine::new(vec![1.0, -1.0, 0.0]).expect("valid line");
        let plane = ProjectivePlane::new(Some(2));

        assert_eq!(point.dimension(), 2);
        assert_eq!(point.coordinates(), &[1.0, 2.0, 1.0]);
        assert_eq!(line.coefficients(), &[1.0, -1.0, 0.0]);
        assert_eq!(plane.order(), Some(2));
        assert_eq!(ProjectivePoint::new(vec![0.0, 0.0]), None);
    }
}
