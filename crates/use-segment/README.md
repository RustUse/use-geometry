# use-segment

Small finite line segment primitives for the RustUse geometry workspace.

## Example

```rust
use use_point::Point2;
use use_segment::Segment2;

let segment = Segment2::try_new(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0))?;

assert_eq!(segment.midpoint(), Point2::new(2.0, 1.0));
assert_eq!(segment.point_at(0.25), Point2::new(1.0, 0.5));
# Ok::<(), use_geometry_core::GeometryError>(())
```
