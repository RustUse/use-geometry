#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A face boundary represented by vertex indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
    vertices: Vec<usize>,
}

impl Face {
    /// Creates a face with at least three vertex indices.
    #[must_use]
    pub fn new(vertices: Vec<usize>) -> Option<Self> {
        if vertices.len() >= 3 {
            Some(Self { vertices })
        } else {
            None
        }
    }

    /// Returns the vertex indices.
    #[must_use]
    pub fn vertices(&self) -> &[usize] {
        &self.vertices
    }

    /// Returns the edge count.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.vertices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Face;

    #[test]
    fn validates_face_boundaries() {
        let face = Face::new(vec![0, 1, 2]).expect("valid face");

        assert_eq!(face.vertices(), &[0, 1, 2]);
        assert_eq!(face.edge_count(), 3);
        assert_eq!(Face::new(vec![0, 1]), None);
    }
}
