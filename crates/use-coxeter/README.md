# use-coxeter

Coxeter diagram primitives for the RustUse geometry workspace.

`use-coxeter` stores nodes, labeled edges, and diagram counts. It does not implement Coxeter group
operations.

## Example

```rust
use use_coxeter::{CoxeterDiagram, CoxeterEdge, CoxeterNode};

let diagram = CoxeterDiagram::new(
    vec![CoxeterNode::new(0), CoxeterNode::new(1)],
    vec![CoxeterEdge::new(0, 1, Some(3)).unwrap()],
).unwrap();

assert_eq!(diagram.node_count(), 2);
```
