# use-mesh

Mesh primitives for the RustUse geometry workspace.

`use-mesh` starts with lightweight mesh count and index records.

## Example

```rust
use use_mesh::Mesh;

let mesh = Mesh::new(8, 12, 6).expect("valid counts");

assert_eq!(mesh.edge_count(), 12);
```
