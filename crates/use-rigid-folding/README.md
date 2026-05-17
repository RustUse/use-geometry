# use-rigid-folding

Rigid folding primitives for the RustUse geometry workspace.

`use-rigid-folding` owns rigid-fold records and intentionally leaves constraint solving for later.

## Example

```rust
use use_angle::Angle;
use use_rigid_folding::{RigidFold, RigidFoldSequence};

let sequence = RigidFoldSequence::new(vec![RigidFold::new(0, Angle::from_degrees(45.0))]);

assert_eq!(sequence.fold_count(), 1);
```
