# use-folding

Folding primitives for the RustUse geometry workspace.

`use-folding` provides fold states and assignments without enforcing a particular folding solver.

## Example

```rust
use use_angle::Angle;
use use_folding::{FoldAssignment, FoldState};

let fold = FoldState::new(FoldAssignment::Mountain, Angle::from_degrees(90.0));

assert_eq!(fold.assignment(), FoldAssignment::Mountain);
```
