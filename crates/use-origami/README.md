# use-origami

Origami model primitives for the RustUse geometry workspace.

`use-origami` provides model summaries and leaves foldability algorithms to focused crates.

## Example

```rust
use use_origami::OrigamiModel;

let model = OrigamiModel::new(8, 4).expect("positive counts");

assert_eq!(model.crease_count(), 8);
```
