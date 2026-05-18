# use-hyperplane

Hyperplane primitives for the RustUse geometry workspace.

`use-hyperplane` keeps n-dimensional hyperplanes small: a coefficient vector and an offset.

## Example

```rust
use use_hyperplane::Hyperplane;

let hyperplane = Hyperplane::try_new(vec![1.0, 0.0, 0.0], -2.0)?;

assert_eq!(hyperplane.dimension(), 3);
assert_eq!(hyperplane.evaluate(&[2.0, 4.0, 5.0]), Some(0.0));
# Ok::<(), use_coordinate::GeometryError>(())
```
