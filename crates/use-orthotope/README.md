# use-orthotope

Orthotope primitives for the RustUse geometry workspace.

`use-orthotope` provides n-dimensional orthotope descriptors plus 3D `Cuboid` and `Cube` types.

## Example

```rust
use use_orthotope::{Cube, Cuboid};

let cuboid = Cuboid::new(2.0, 3.0, 4.0).expect("positive dimensions");
let cube = Cube::new(3.0).expect("positive edge");

assert_eq!(cuboid.volume(), 24.0);
assert_eq!(cube.volume(), 27.0);
```
