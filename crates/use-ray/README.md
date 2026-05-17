# use-ray

Ray primitives for the RustUse geometry workspace.

`use-ray` models half-infinite 2D rays with a point origin and non-zero direction vector.

## Example

```rust
use use_point::Point2;
use use_ray::Ray2;
use use_vector::Vector2;

let ray = Ray2::try_new(Point2::new(1.0, 2.0), Vector2::new(3.0, 0.0))?;

assert_eq!(ray.point_at(2.0), Point2::new(7.0, 2.0));
# Ok::<(), use_geometry_core::GeometryError>(())
```
