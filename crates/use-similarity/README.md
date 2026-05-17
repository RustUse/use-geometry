# use-similarity

Similarity relation helpers for the RustUse geometry workspace.

`use-similarity` provides small scale-ratio primitives for comparing shapes without implementing a
full shape-matching engine.

## Example

```rust
use use_similarity::SimilarityRatio;

let ratio = SimilarityRatio::new(2.0).expect("positive finite ratio");

assert_eq!(ratio.apply_length(3.0), 6.0);
```
