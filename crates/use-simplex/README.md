# use-simplex

Simplex primitives for the RustUse geometry workspace.

`use-simplex` provides general simplex descriptors plus a small `Tetrahedron` type. It does not
duplicate triangle logic from `use-triangle`.

## Example

```rust
use use_simplex::{Simplex, Tetrahedron};
use use_vector::Vector3;

let simplex = Simplex::new(3).expect("valid dimension");
let tetrahedron = Tetrahedron::new([
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 0.0, 1.0),
]);

assert_eq!(simplex.vertex_count(), 4);
assert_eq!(tetrahedron.vertex_count(), 4);
```
