# use-polygon

Polygon primitives for the RustUse geometry workspace.

`use-polygon` stores ordered planar vertices and provides a simple shoelace-area helper.

## Example

```rust
use use_point::Point2;
use use_polygon::Polygon;

let polygon = Polygon::new(vec![
    Point2::new(0.0, 0.0),
    Point2::new(4.0, 0.0),
    Point2::new(0.0, 3.0),
]);

assert_eq!(polygon.area(), 6.0);
```
