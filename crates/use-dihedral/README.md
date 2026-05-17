# use-dihedral

Dihedral angle primitives for the RustUse geometry workspace.

`use-dihedral` wraps `use-angle` for angles between two half-planes or faces without attempting
polyhedral angle solving.

## Example

```rust
use use_dihedral::DihedralAngle;

let angle = DihedralAngle::from_degrees(90.0);

assert_eq!(angle.degrees(), 90.0);
```