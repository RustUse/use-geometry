# use-transform

Transformation primitives for the RustUse geometry workspace.

`use-transform` provides small 2D and 3D transform value types without introducing a full matrix
library dependency.

## Example

```rust
use use_transform::{Transform2, Translation};

let transform = Transform2::translation(Translation::new(2.0, 3.0, 0.0));

assert_eq!(transform.apply_point((1.0, 1.0)), (3.0, 4.0));
```
