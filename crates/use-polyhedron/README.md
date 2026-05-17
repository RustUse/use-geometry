# use-polyhedron

Polyhedron primitives for the RustUse geometry workspace.

`use-polyhedron` is the general 3D polyhedron crate. It starts with vertex, edge, and face records
plus count helpers.

## Example

```rust
use use_polyhedron::Polyhedron;

let polyhedron = Polyhedron::from_counts(8, 12, 6).expect("valid counts");

assert_eq!(polyhedron.euler_characteristic(), 2);
```
