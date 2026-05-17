# use-bounds

Axis-aligned bounds helpers for the RustUse geometry workspace.

## Example

```rust
use use_bounds::Aabb2;
use use_point::Point2;

let bounds = Aabb2::from_points(Point2::new(4.0, 1.0), Point2::new(1.0, 3.0));

assert_eq!(bounds.min(), Point2::new(1.0, 1.0));
assert_eq!(bounds.max(), Point2::new(4.0, 3.0));
assert!(bounds.contains_point(Point2::new(2.0, 2.0)));
```