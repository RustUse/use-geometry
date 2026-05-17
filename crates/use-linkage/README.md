# use-linkage

Linkage primitives for the RustUse geometry workspace.

`use-linkage` provides joint, bar, and linkage count records for geometry-mechanism work.

## Example

```rust
use use_linkage::{Bar, Linkage};

let linkage = Linkage::new(4, vec![Bar::new(0, 1, 2.0).unwrap()]).unwrap();

assert_eq!(linkage.joint_count(), 4);
```
