# use-face

Face primitives for the RustUse geometry workspace.

`use-face` owns the generic `Face` type for the geometry facade and keeps face boundaries as index
lists.

## Example

```rust
use use_face::Face;

let face = Face::new(vec![0, 1, 2]).expect("at least three vertices");

assert_eq!(face.edge_count(), 3);
```
