# use-projective

Projective geometry primitives for the RustUse geometry workspace.

`use-projective` stores homogeneous coordinate records for projective points, lines, and planes. It
does not attempt deep algebraic geometry.

## Example

```rust
use use_projective::ProjectivePoint;

let point = ProjectivePoint::new(vec![1.0, 2.0, 1.0]).expect("valid point");

assert_eq!(point.dimension(), 2);
```
