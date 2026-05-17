#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// An adjacency edge in a polyhedral net.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetEdge {
    first_face: usize,
    second_face: usize,
}

impl NetEdge {
    /// Creates a net edge.
    #[must_use]
    pub const fn new(first_face: usize, second_face: usize) -> Self {
        Self {
            first_face,
            second_face,
        }
    }

    /// Returns the adjacent face indices.
    #[must_use]
    pub const fn faces(self) -> (usize, usize) {
        (self.first_face, self.second_face)
    }
}

/// A polyhedral net descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyhedralNet {
    face_count: usize,
    edges: Vec<NetEdge>,
}

impl PolyhedralNet {
    /// Creates a polyhedral net with at least one face.
    #[must_use]
    pub fn new(face_count: usize, edges: Vec<NetEdge>) -> Option<Self> {
        if face_count > 0 {
            Some(Self { face_count, edges })
        } else {
            None
        }
    }

    /// Returns the face count.
    #[must_use]
    pub const fn face_count(&self) -> usize {
        self.face_count
    }

    /// Returns the adjacency edges.
    #[must_use]
    pub fn edges(&self) -> &[NetEdge] {
        &self.edges
    }

    /// Returns the adjacency count.
    #[must_use]
    pub fn adjacency_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{NetEdge, PolyhedralNet};

    #[test]
    fn stores_polyhedral_nets() {
        let edge = NetEdge::new(0, 1);
        let net = PolyhedralNet::new(6, vec![edge]).expect("valid net");

        assert_eq!(edge.faces(), (0, 1));
        assert_eq!(net.face_count(), 6);
        assert_eq!(net.adjacency_count(), 1);
        assert_eq!(net.edges(), &[edge]);
    }
}
