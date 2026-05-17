# use-curve

Curve primitives for the RustUse geometry workspace.

`use-curve` provides small shared curve vocabulary: normalized parameters and sampled points.

## Example

```rust
use use_curve::{CurveParameter, CurveSample2};
use use_point::Point2;

let sample = CurveSample2::new(CurveParameter::new(0.5).unwrap(), Point2::new(1.0, 2.0));

assert_eq!(sample.parameter().value(), 0.5);
```
