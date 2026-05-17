# use-sphere

Ordinary sphere primitives for the RustUse geometry workspace.

`use-sphere` models three-dimensional Euclidean spheres. Higher-dimensional sphere vocabulary lives
in `use-hypersphere`.

## Example

```rust
use use_sphere::Sphere;

let sphere = Sphere::new(3.0).expect("valid sphere");

assert_eq!(sphere.radius(), 3.0);
assert_eq!(sphere.diameter(), 6.0);
```