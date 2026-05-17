# use-surface

Surface primitives for the RustUse geometry workspace.

`use-surface` provides small descriptors for surface families and parameter-domain patches.

## Example

```rust
use use_surface::{SurfaceKind, SurfacePatch};

let patch = SurfacePatch::new(SurfaceKind::Parametric, 2.0, 3.0).expect("positive domain");

assert_eq!(patch.parameter_area(), 6.0);
```
