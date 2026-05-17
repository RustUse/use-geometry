# use-intersection

Intersection result primitives for the RustUse geometry workspace.

`use-intersection` starts with shared result enums and traits. It does not try to implement the full
matrix of shape intersection algorithms.

## Example

```rust
use use_intersection::Intersection2;
use use_point::Point2;

let hit = Intersection2::Point(Point2::new(1.0, 2.0));

assert!(!hit.is_empty());
```
