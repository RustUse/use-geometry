# use-triangulation

Triangulation primitives for the RustUse geometry workspace.

`use-triangulation` stores indexed triangle connectivity and construction method labels.

## Example

```rust
use use_triangulation::{Triangulation2, TriangulationMethod};

let triangulation = Triangulation2::new(vec![[0, 1, 2], [0, 2, 3]]);

assert_eq!(triangulation.triangle_count(), 2);
assert_eq!(TriangulationMethod::EarClipping.name(), "ear-clipping");
```
