#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// An n-dimensional orthotope represented by positive edge lengths.
#[derive(Debug, Clone, PartialEq)]
pub struct Orthotope {
    edge_lengths: Vec<f64>,
}

impl Orthotope {
    /// Creates an orthotope with positive finite edge lengths.
    #[must_use]
    pub fn new(edge_lengths: Vec<f64>) -> Option<Self> {
        if !edge_lengths.is_empty()
            && edge_lengths
                .iter()
                .all(|length| length.is_finite() && *length > 0.0)
        {
            Some(Self { edge_lengths })
        } else {
            None
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.edge_lengths.len()
    }

    /// Returns the edge lengths.
    #[must_use]
    pub fn edge_lengths(&self) -> &[f64] {
        &self.edge_lengths
    }

    /// Returns the hypervolume.
    #[must_use]
    pub fn measure(&self) -> f64 {
        self.edge_lengths.iter().product()
    }
}

/// A rectangular cuboid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cuboid {
    width: f64,
    height: f64,
    depth: f64,
}

impl Cuboid {
    /// Creates a cuboid with positive finite dimensions.
    #[must_use]
    pub const fn new(width: f64, height: f64, depth: f64) -> Option<Self> {
        if width.is_finite()
            && height.is_finite()
            && depth.is_finite()
            && width > 0.0
            && height > 0.0
            && depth > 0.0
        {
            Some(Self {
                width,
                height,
                depth,
            })
        } else {
            None
        }
    }

    /// Returns the volume.
    #[must_use]
    pub const fn volume(self) -> f64 {
        self.width * self.height * self.depth
    }
}

/// A cube.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    edge: f64,
}

impl Cube {
    /// Creates a cube with a positive finite edge length.
    #[must_use]
    pub const fn new(edge: f64) -> Option<Self> {
        if edge.is_finite() && edge > 0.0 {
            Some(Self { edge })
        } else {
            None
        }
    }

    /// Returns the edge length.
    #[must_use]
    pub const fn edge(self) -> f64 {
        self.edge
    }

    /// Returns the volume.
    #[must_use]
    pub const fn volume(self) -> f64 {
        self.edge * self.edge * self.edge
    }
}

#[cfg(test)]
mod tests {
    use super::{Cube, Cuboid, Orthotope};

    #[test]
    fn computes_orthotope_measures() {
        let orthotope = Orthotope::new(vec![2.0, 3.0, 4.0]).expect("valid orthotope");
        let cuboid = Cuboid::new(2.0, 3.0, 4.0).expect("valid cuboid");
        let cube = Cube::new(3.0).expect("valid cube");

        assert_eq!(orthotope.dimension(), 3);
        assert_eq!(orthotope.measure(), 24.0);
        assert_eq!(cuboid.volume(), 24.0);
        assert_eq!(cube.edge(), 3.0);
        assert_eq!(cube.volume(), 27.0);
    }
}
