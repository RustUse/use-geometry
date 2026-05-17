# use-cell

Cell-complex primitives for the RustUse geometry workspace.

`use-cell` provides small cells and cell-complex summaries for representation-oriented geometry.

## Example

```rust
use use_cell::{Cell, CellComplex};

let complex = CellComplex::new(vec![Cell::new(2, 4).unwrap()]);

assert_eq!(complex.cell_count(), 1);
```
