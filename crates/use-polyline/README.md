# use-polyline

Polyline primitives for the RustUse geometry workspace.

`use-polyline` stores ordered 2D vertices and provides a simple path-length helper.

## Example

```rust
use use_point::Point2;
use use_polyline::Polyline2;

let polyline = Polyline2::new(vec![Point2::new(0.0, 0.0), Point2::new(3.0, 4.0)]);

assert_eq!(polyline.length(), 5.0);
```
