# use-regular-polytope

Regular polytope primitives for the RustUse geometry workspace.

`use-regular-polytope` groups regular polygons, Platonic solids, and the convex regular
four-dimensional polytopes. Named objects such as the 24-cell, 600-cell, and 120-cell are represented
inside this family crate rather than as standalone crates. Schlafli notation is provided by the
`use-schlafli` crate.

## Example

```rust
use use_regular_polytope::{PlatonicSolid, RegularPolygon, RegularPolytope4};

let polygon = RegularPolygon::new(6, 2.0).expect("valid polygon");

assert_eq!(polygon.side_count(), 6);
assert_eq!(PlatonicSolid::Icosahedron.face_count(), 20);
assert_eq!(RegularPolytope4::TwentyFourCell.schlafli_symbol().to_string(), "{3, 4, 3}");
```
