# use-tessellation

Tessellation primitives for the RustUse geometry workspace.

`use-tessellation` provides small tiling descriptors and tile-count summaries.

## Example

```rust
use use_tessellation::{Tessellation, TessellationKind};

let tessellation = Tessellation::new(TessellationKind::Regular, 12).expect("positive tiles");

assert_eq!(tessellation.tile_count(), 12);
```
