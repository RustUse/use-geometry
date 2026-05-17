# use-congruence

Congruence helpers for the RustUse geometry workspace.

`use-congruence` begins with tolerance-aware length comparison primitives and leaves full shape
congruence algorithms to future focused crates.

## Example

```rust
use use_congruence::CongruenceTolerance;

let tolerance = CongruenceTolerance::new(0.01).expect("non-negative tolerance");

assert!(tolerance.matches_lengths(3.0, 3.005));
```
