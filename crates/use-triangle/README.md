# use-triangle

Small triangle primitives and measurements for the RustUse geometry workspace.

## Example

```rust
use use_orientation::Orientation2;
use use_point::Point2;
use use_triangle::{Triangle, triangle_area};

let a = Point2::new(0.0, 0.0);
let b = Point2::new(4.0, 0.0);
let c = Point2::new(0.0, 3.0);
let triangle = Triangle::try_new(a, b, c)?;

assert_eq!(triangle.orientation(), Orientation2::CounterClockwise);
assert_eq!(triangle.area(), triangle_area(a, b, c));
assert_eq!(triangle.perimeter(), 12.0);
# Ok::<(), use_coordinate::GeometryError>(())
```
