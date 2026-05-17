# use-crease

Crease primitives for the RustUse geometry workspace.

`use-crease` owns crease-line records for folding and origami crates.

## Example

```rust
use use_crease::{Crease, CreaseKind};
use use_point::Point2;

let crease = Crease::new(Point2::origin(), Point2::new(1.0, 0.0), CreaseKind::Mountain);

assert_eq!(crease.kind(), CreaseKind::Mountain);
```
