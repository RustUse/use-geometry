#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A mesh vertex index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshVertexIndex(pub usize);

/// A mesh face index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshFaceIndex(pub usize);

/// A lightweight mesh count summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mesh {
    vertex_count: usize,
    edge_count: usize,
    face_count: usize,
}

impl Mesh {
    /// Creates a mesh summary with positive counts.
    #[must_use]
    pub const fn new(vertex_count: usize, edge_count: usize, face_count: usize) -> Option<Self> {
        if vertex_count > 0 && edge_count > 0 && face_count > 0 {
            Some(Self {
                vertex_count,
                edge_count,
                face_count,
            })
        } else {
            None
        }
    }

    /// Returns the vertex count.
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        self.vertex_count
    }

    /// Returns the edge count.
    #[must_use]
    pub const fn edge_count(self) -> usize {
        self.edge_count
    }

    /// Returns the face count.
    #[must_use]
    pub const fn face_count(self) -> usize {
        self.face_count
    }
}

#[cfg(test)]
mod tests {
    use super::{Mesh, MeshFaceIndex, MeshVertexIndex};

    #[test]
    fn stores_mesh_counts_and_indices() {
        let mesh = Mesh::new(8, 12, 6).expect("valid mesh");

        assert_eq!(mesh.vertex_count(), 8);
        assert_eq!(mesh.edge_count(), 12);
        assert_eq!(mesh.face_count(), 6);
        assert_eq!(MeshVertexIndex(1).0, 1);
        assert_eq!(MeshFaceIndex(2).0, 2);
    }
}
