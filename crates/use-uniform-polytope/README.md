# use-uniform-polytope

Uniform polytope classification primitives for the RustUse geometry workspace.

`use-uniform-polytope` stores broad family metadata for uniform polytopes. It does not enumerate or
construct every uniform polytope.

## Example

```rust
use use_uniform_polytope::{UniformPolytope, UniformPolytopeKind};

let polytope = UniformPolytope::new(3, UniformPolytopeKind::Archimedean).expect("valid");

assert_eq!(polytope.dimension(), 3);
assert_eq!(polytope.kind(), UniformPolytopeKind::Archimedean);
```
