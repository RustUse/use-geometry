# use-torus

Torus primitives for the RustUse geometry workspace.

`use-torus` stores major and minor radii and exposes direct measurement helpers. It does not cover
knot theory or topology-heavy behavior.

## Example

```rust
use use_torus::Torus;

let torus = Torus::new(3.0, 1.0).expect("valid torus");

assert_eq!(torus.major_radius(), 3.0);
assert_eq!(torus.minor_radius(), 1.0);
```
