# use-plane

Plane primitives for the RustUse geometry workspace.

`use-plane` models a 3D plane with the equation `normal.dot(point) + offset = 0`.

## Example

```rust
use use_plane::Plane3;
use use_vector::Vector3;

let plane = Plane3::try_new(Vector3::new(0.0, 0.0, 1.0), -2.0)?;

assert_eq!(plane.evaluate(Vector3::new(0.0, 0.0, 2.0)), 0.0);
# Ok::<(), use_coordinate::GeometryError>(())
```
