# use-inversion

Geometric inversion primitives for the RustUse geometry workspace.

`use-inversion` stores the center and radius of an inversion circle. It intentionally does not try
to implement a full circle-inversion engine in this first pass.

## Example

```rust
use use_inversion::Inversion;
use use_point::Point2;

let inversion = Inversion::try_new(Point2::origin(), 2.0).expect("valid inversion");

assert_eq!(inversion.radius(), 2.0);
assert_eq!(inversion.radius_squared(), 4.0);
```
