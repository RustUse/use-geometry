# use-affine

Affine-space helpers for the RustUse geometry workspace.

`use-affine` starts with small affine combination helpers rather than a full affine algebra system.

## Example

```rust
use use_affine::affine_combination_2d;
use use_point::Point2;

let midpoint = affine_combination_2d(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0), 0.5);

assert_eq!(midpoint, Point2::new(2.0, 1.0));
```
