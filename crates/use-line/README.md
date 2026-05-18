# use-line

Small infinite line primitives for the RustUse geometry workspace.

## Example

```rust
use use_line::Line2;
use use_point::Point2;
use use_vector::Vector2;

let line = Line2::try_from_point_direction(Point2::new(1.0, 2.0), Vector2::new(3.0, 4.0))?;

assert_eq!(line.b(), Point2::new(4.0, 6.0));
assert!(line.contains_point(Point2::new(7.0, 10.0)));
# Ok::<(), use_coordinate::GeometryError>(())
```
