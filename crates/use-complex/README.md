# use-complex

Geometric complex primitives for the RustUse geometry workspace.

`use-complex` is about geometric complexes, cell complexes, and simplicial complexes. It is not a
complex-number crate; complex numbers remain in `git_local/use-math/crates/use-complex`.

## Example

```rust
use use_cell::Cell;
use use_complex::{CellComplex, GeometricComplex, SimplicialComplex};

let cell_complex = CellComplex::new(vec![Cell::new(2, 3).unwrap()]);
let simplicial = SimplicialComplex::new(vec![3]).unwrap();

assert_eq!(cell_complex.cell_count(), 1);
assert_eq!(GeometricComplex::new(2, 1).unwrap().dimension(), 2);
assert_eq!(simplicial.simplex_count(), 1);
```
