# use-regular-polytope

Regular polytope primitives for the RustUse geometry workspace.

`use-regular-polytope` groups regular polygons and Platonic solids. Platonic solids are represented
by the `PlatonicSolid` enum rather than separate crates.

## Example

```rust
use use_regular_polytope::{PlatonicSolid, RegularPolygon};

let polygon = RegularPolygon::new(6, 2.0).expect("valid polygon");

assert_eq!(polygon.side_count(), 6);
assert_eq!(PlatonicSolid::Icosahedron.face_count(), 20);
```
