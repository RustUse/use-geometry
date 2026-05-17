# use-incidence

Incidence geometry primitives for the RustUse geometry workspace.

`use-incidence` stores point-line incidence pairs and compact incidence matrices. It intentionally
avoids advanced finite-geometry algorithms.

## Example

```rust
use use_incidence::{IncidencePair, IncidenceStructure};

let structure = IncidenceStructure::new(2, 1, vec![IncidencePair::new(0, 0)]).unwrap();

assert_eq!(structure.point_count(), 2);
assert_eq!(structure.incidence_count(), 1);
```
