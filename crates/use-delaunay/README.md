# use-delaunay

Delaunay triangulation primitives for the RustUse geometry workspace.

`use-delaunay` owns Delaunay-specific records while `use-triangulation` owns general triangulation
vocabulary.

## Example

```rust
use use_delaunay::DelaunayTriangulation;

let triangulation = DelaunayTriangulation::new(vec![[0, 1, 2]]);

assert_eq!(triangulation.triangle_count(), 1);
```
