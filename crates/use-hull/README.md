# use-hull

Hull primitives for the RustUse geometry workspace.

`use-hull` provides descriptors for convex-hull inputs and algorithms. It does not yet implement
hull construction.

## Example

```rust
use use_hull::{ConvexHull2, HullAlgorithm};
use use_point::Point2;

let hull = ConvexHull2::new(vec![Point2::origin(), Point2::new(1.0, 0.0)]);

assert_eq!(hull.point_count(), 2);
assert_eq!(HullAlgorithm::MonotoneChain.name(), "monotone-chain");
```
