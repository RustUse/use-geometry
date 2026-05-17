# use-voronoi

Voronoi diagram primitives for the RustUse geometry workspace.

`use-voronoi` stores site-indexed cells and diagram summaries. It does not yet generate diagrams.

## Example

```rust
use use_point::Point2;
use use_voronoi::{VoronoiCell, VoronoiDiagram};

let cell = VoronoiCell::new(0, vec![Point2::origin()]);
let diagram = VoronoiDiagram::new(vec![cell]);

assert_eq!(diagram.cell_count(), 1);
```
