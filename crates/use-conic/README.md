# use-conic

Conic primitives for the RustUse geometry workspace.

`use-conic` begins with named conic-family value types and lightweight validation. It does not
attempt symbolic equation manipulation or fitting algorithms.

## Example

```rust
use use_conic::{ConicKind, Ellipse};
use use_point::Point2;

let ellipse = Ellipse::new(Point2::origin(), 4.0, 2.0).expect("positive radii");

assert_eq!(ellipse.kind(), ConicKind::Ellipse);
assert_eq!(ellipse.major_radius(), 4.0);
```
