# use-coordinate

Coordinate primitives for the RustUse geometry workspace.

`use-coordinate` provides small axis and coordinate value types for geometry crates that need a
lightweight coordinate vocabulary without a larger algebra layer.

## Example

```rust
use use_coordinate::{Axis2, Coordinate2};

let point = Coordinate2::new(2.0, 3.0);

assert_eq!(point.component(Axis2::X), 2.0);
assert_eq!(point.as_tuple(), (2.0, 3.0));
```
