# use-projection

Projection primitives for the RustUse geometry workspace.

`use-projection` provides naming and tiny helpers for geometric projections without rendering-engine
behavior.

## Example

```rust
use use_point::Point2;
use use_projection::{Projection2, ProjectionKind};

let projection = Projection2::new(ProjectionKind::Orthographic, 2.0).expect("valid projection");

assert_eq!(projection.project_x_axis(Point2::new(3.0, 4.0)), 6.0);
```
