# use-orientation

Small 2D winding and collinearity helpers for the RustUse geometry workspace.

## Example

```rust
use use_orientation::{Orientation2, orientation_2d};
use use_point::Point2;

assert_eq!(
    orientation_2d(
        Point2::new(0.0, 0.0),
        Point2::new(4.0, 0.0),
        Point2::new(0.0, 3.0),
    ),
    Orientation2::CounterClockwise,
);
```
