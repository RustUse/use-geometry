# use-distance

Point-to-point distance and midpoint helpers for the RustUse geometry workspace.

## Example

```rust
use use_distance::{distance_2d, midpoint_2d};
use use_point::Point2;

let left = Point2::new(0.0, 0.0);
let right = Point2::new(3.0, 4.0);

assert_eq!(distance_2d(left, right), 5.0);
assert_eq!(midpoint_2d(left, right), Point2::new(1.5, 2.0));
```