# use-polyhedral-net

Polyhedral net primitives for the RustUse geometry workspace.

`use-polyhedral-net` owns polyhedral-net records and intentionally avoids the shorter crate name
`use-net`, which belongs to another RustUse domain.

## Example

```rust
use use_polyhedral_net::{NetEdge, PolyhedralNet};

let net = PolyhedralNet::new(6, vec![NetEdge::new(0, 1)]).expect("valid net");

assert_eq!(net.face_count(), 6);
```
