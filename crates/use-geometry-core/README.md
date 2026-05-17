# use-geometry-core

Shared low-level geometry validation errors for the RustUse geometry workspace.

`use-geometry-core` stays intentionally small. It exists so focused geometry crates can share
`GeometryError` without routing implementation through the `use-geometry` facade crate.

## Example

```rust
use use_geometry_core::GeometryError;

assert_eq!(
    GeometryError::validate_tolerance(0.25),
    Ok(0.25)
);
assert_eq!(
    GeometryError::validate_tolerance(-0.25),
    Err(GeometryError::NegativeTolerance(-0.25))
);
```