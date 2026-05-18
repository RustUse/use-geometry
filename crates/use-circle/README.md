# use-circle

Small circle primitives for the RustUse geometry workspace.

## Example

```rust
use use_circle::Circle;
use use_point::Point2;

let circle = Circle::try_new(Point2::new(0.0, 0.0), 2.5)?;

assert_eq!(circle.center(), Point2::new(0.0, 0.0));
assert_eq!(circle.radius(), 2.5);
assert!(circle.contains_point(Point2::new(1.0, 1.0)));
# Ok::<(), use_coordinate::GeometryError>(())
```
