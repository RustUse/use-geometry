# use-spline

Spline primitives for the RustUse geometry workspace.

`use-spline` starts with control-point containers and leaves interpolation algorithms for later
focused work.

## Example

```rust
use use_point::Point2;
use use_spline::Spline2;

let spline = Spline2::new(vec![Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]).unwrap();

assert_eq!(spline.control_point_count(), 2);
```
