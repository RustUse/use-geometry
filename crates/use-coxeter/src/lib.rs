#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A node in a Coxeter diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoxeterNode {
    index: usize,
}

impl CoxeterNode {
    /// Creates a node with an index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    /// Returns the node index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
}

/// An edge in a Coxeter diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoxeterEdge {
    start: usize,
    end: usize,
    order: Option<usize>,
}

impl CoxeterEdge {
    /// Creates an edge between distinct nodes with an optional Coxeter order of at least three.
    #[must_use]
    pub const fn new(start: usize, end: usize, order: Option<usize>) -> Option<Self> {
        if start == end {
            return None;
        }

        if let Some(order) = order
            && order < 3
        {
            return None;
        }

        Some(Self { start, end, order })
    }

    /// Returns the edge endpoints.
    #[must_use]
    pub const fn endpoints(self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// Returns the optional Coxeter order label.
    #[must_use]
    pub const fn order(self) -> Option<usize> {
        self.order
    }
}

/// A Coxeter diagram record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoxeterDiagram {
    nodes: Vec<CoxeterNode>,
    edges: Vec<CoxeterEdge>,
}

impl CoxeterDiagram {
    /// Creates a diagram whose edge endpoints reference existing node indices.
    #[must_use]
    pub fn new(nodes: Vec<CoxeterNode>, edges: Vec<CoxeterEdge>) -> Option<Self> {
        if edges.iter().all(|edge| {
            nodes.iter().any(|node| node.index() == edge.start)
                && nodes.iter().any(|node| node.index() == edge.end)
        }) {
            Some(Self { nodes, edges })
        } else {
            None
        }
    }

    /// Returns the nodes.
    #[must_use]
    pub fn nodes(&self) -> &[CoxeterNode] {
        &self.nodes
    }

    /// Returns the edges.
    #[must_use]
    pub fn edges(&self) -> &[CoxeterEdge] {
        &self.edges
    }

    /// Returns the node count.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the edge count.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{CoxeterDiagram, CoxeterEdge, CoxeterNode};

    #[test]
    fn validates_coxeter_diagrams() {
        let nodes = vec![CoxeterNode::new(0), CoxeterNode::new(1)];
        let edge = CoxeterEdge::new(0, 1, Some(3)).expect("valid edge");
        let diagram = CoxeterDiagram::new(nodes, vec![edge]).expect("valid diagram");

        assert_eq!(diagram.node_count(), 2);
        assert_eq!(diagram.edge_count(), 1);
        assert_eq!(edge.endpoints(), (0, 1));
        assert_eq!(CoxeterEdge::new(0, 0, None), None);
    }
}
