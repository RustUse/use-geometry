# use-johnson-solid

Johnson solid identifiers for the RustUse geometry workspace.

`use-johnson-solid` models the `J1` through `J92` identifiers and selected names without storing
coordinates or construction algorithms.

## Example

```rust
use use_johnson_solid::{JohnsonSolid, JohnsonSolidId};

let solid = JohnsonSolid::new(JohnsonSolidId::new(1).expect("valid id"));

assert_eq!(solid.id().label(), "J1");
assert_eq!(solid.name(), Some("square pyramid"));
```
