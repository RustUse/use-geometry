# use-catalan-solid

Catalan solid metadata for the RustUse geometry workspace.

`use-catalan-solid` groups Catalan solids and records their dual Archimedean solids where that
metadata is direct.

## Example

```rust
use use_archimedean::ArchimedeanSolid;
use use_catalan_solid::CatalanSolid;

let solid = CatalanSolid::RhombicDodecahedron;

assert_eq!(solid.dual_archimedean(), ArchimedeanSolid::Cuboctahedron);
```
