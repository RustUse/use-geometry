# use-hypersphere

Higher-dimensional sphere primitives for the RustUse geometry workspace.

`use-hypersphere` keeps n-sphere vocabulary lightweight: dimension metadata, radius storage, and
simple constructors. Ordinary three-dimensional sphere measurements live in `use-sphere`.

## Example

```rust
use use_hypersphere::{Hypersphere, ThreeSphere};

let sphere = Hypersphere::<3>::new(2.0).expect("valid hypersphere");
let three_sphere = ThreeSphere::new(2.0).expect("valid 3-sphere");

assert_eq!(sphere.dimension(), 3);
assert_eq!(three_sphere.dimension(), 3);
```
