#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_vector::Vector3;

/// A polyhedron vertex.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: Vector3,
}

impl Vertex {
    /// Creates a vertex.
    #[must_use]
    pub const fn new(position: Vector3) -> Self {
        Self { position }
    }

    /// Returns the vertex position.
    #[must_use]
    pub const fn position(self) -> Vector3 {
        self.position
    }
}

/// A polyhedron edge represented by vertex indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    start: usize,
    end: usize,
}

impl Edge {
    /// Creates an edge between two vertex indices.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the edge endpoints.
    #[must_use]
    pub const fn endpoints(self) -> (usize, usize) {
        (self.start, self.end)
    }
}

/// A polyhedron face record represented by vertex indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyhedronFace {
    vertices: Vec<usize>,
}

impl PolyhedronFace {
    /// Creates a face record.
    #[must_use]
    pub const fn new(vertices: Vec<usize>) -> Self {
        Self { vertices }
    }

    /// Returns the face vertex indices.
    #[must_use]
    pub fn vertices(&self) -> &[usize] {
        &self.vertices
    }
}

/// A polyhedron count summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polyhedron {
    vertex_count: usize,
    edge_count: usize,
    face_count: usize,
}

impl Polyhedron {
    /// Creates a polyhedron from positive vertex, edge, and face counts.
    #[must_use]
    pub const fn from_counts(
        vertex_count: usize,
        edge_count: usize,
        face_count: usize,
    ) -> Option<Self> {
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

    /// Returns `V - E + F`.
    #[must_use]
    pub const fn euler_characteristic(self) -> isize {
        self.vertex_count as isize - self.edge_count as isize + self.face_count as isize
    }
}

#[cfg(test)]
mod tests {
    use super::{Edge, Polyhedron, PolyhedronFace, Vertex};
    use use_vector::Vector3;

    #[test]
    fn stores_polyhedron_records() {
        let vertex = Vertex::new(Vector3::new(1.0, 2.0, 3.0));
        let edge = Edge::new(0, 1);
        let face = PolyhedronFace::new(vec![0, 1, 2]);
        let polyhedron = Polyhedron::from_counts(8, 12, 6).expect("valid counts");

        assert_eq!(vertex.position(), Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(edge.endpoints(), (0, 1));
        assert_eq!(face.vertices(), &[0, 1, 2]);
        assert_eq!(polyhedron.euler_characteristic(), 2);
    }
}
