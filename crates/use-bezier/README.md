# use-bezier

Bezier curve primitives for the RustUse geometry workspace.

`use-bezier` provides small quadratic and cubic Bezier value types with direct evaluation.

## Example

```rust
use use_bezier::QuadraticBezier2;
use use_point::Point2;

let curve = QuadraticBezier2::new(Point2::new(0.0, 0.0), Point2::new(1.0, 2.0), Point2::new(2.0, 0.0));

assert_eq!(curve.point_at(0.5), Point2::new(1.0, 1.0));
```
