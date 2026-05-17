# use-containment

Containment traits and classifications for the RustUse geometry workspace.

`use-containment` defines small shared vocabulary for inside, boundary, and outside relationships.

## Example

```rust
use use_containment::Containment;

assert!(Containment::Boundary.is_contained());
assert!(!Containment::Outside.is_contained());
```
