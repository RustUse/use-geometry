# use-duality

Geometric duality primitives for the RustUse geometry workspace.

`use-duality` stores dual relationships as metadata. It does not construct geometric duals
automatically.

## Example

```rust
use use_duality::{Dual, Duality};

let pair = Dual::new("cube", "octahedron");
let duality = Duality::new(pair, true);

assert!(duality.is_involutive());
```
