# use-angle

Angle primitives for the RustUse geometry workspace.

`use-angle` stores angles in radians and provides direct degree conversion helpers.

## Example

```rust
use use_angle::Angle;

let angle = Angle::from_degrees(180.0);

assert_eq!(angle.radians(), core::f64::consts::PI);
assert_eq!(Angle::from_radians(core::f64::consts::PI).degrees(), 180.0);
```
