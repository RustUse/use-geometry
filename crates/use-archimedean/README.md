# use-archimedean

Archimedean solid metadata for the RustUse geometry workspace.

`use-archimedean` groups named Archimedean solids such as the cuboctahedron inside one family crate.
It does not generate meshes or coordinates.

## Example

```rust
use use_archimedean::ArchimedeanSolid;

let solid = ArchimedeanSolid::Cuboctahedron;

assert_eq!(solid.name(), "cuboctahedron");
assert_eq!(solid.face_configuration(), "3.4.3.4");
```
