# use-cell

Cell primitives for the RustUse geometry workspace.

`use-cell` provides small cell descriptors for representation-oriented geometry. Cell-complex
collections live in `use-geometric-complex`, which focuses on geometric complexes rather than complex numbers.

## Example

```rust
use use_cell::Cell;

let cell = Cell::new(2, 4).unwrap();

assert_eq!(cell.dimension(), 2);
assert_eq!(cell.boundary_count(), 4);
```
