# use-dimension

Dimension primitives for the RustUse geometry workspace.

`use-dimension` provides small wrappers for spatial dimension counts without committing callers to a
large type-level dimension system.

## Example

```rust
use use_dimension::Dimension;

let dimension = Dimension::new(3).expect("positive dimension");

assert_eq!(dimension.value(), 3);
assert!(dimension.is_spatial());
```
