# use-manifold

Basic manifold vocabulary for the RustUse geometry workspace.

`use-manifold` provides metadata records for dimensions and boundary kinds. It does not attempt to
be a differential-geometry engine.

## Example

```rust
use use_manifold::{BoundaryKind, Manifold, ManifoldDimension};

let manifold = Manifold::new(ManifoldDimension::new(2).unwrap(), BoundaryKind::WithoutBoundary);

assert_eq!(manifold.dimension().value(), 2);
```
