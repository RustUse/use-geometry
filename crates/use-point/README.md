# use-point

Validated 2D point primitives for the RustUse geometry workspace.

`use-point` keeps coordinate validation and point-to-point operations separate from higher-level
geometry shapes. It composes with `use-vector` for translation and point differences.

## Example

```rust
use use_point::Point2;
use use_vector::Vector2;

let start = Point2::try_new(1.0, 2.0)?;
let end = start.translate(Vector2::new(3.0, 4.0));

assert_eq!(end, Point2::new(4.0, 6.0));
assert_eq!(end - start, Vector2::new(3.0, 4.0));
# Ok::<(), use_coordinate::GeometryError>(())
```
