# use-polytope

Polytope concepts for the RustUse geometry workspace.

`use-polytope` provides small n-dimensional polytope descriptors and intentionally avoids a full
computational geometry engine.

## Example

```rust
use use_polytope::Polytope;

let polytope = Polytope::new(4, 8, 16).expect("valid counts");

assert_eq!(polytope.dimension(), 4);
assert_eq!(polytope.vertex_count(), 8);
```
