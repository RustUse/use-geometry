# use-wythoff

Wythoff notation primitives for the RustUse geometry workspace.

`use-wythoff` stores Wythoff symbols as notation metadata. It does not implement Wythoff
construction algorithms.

## Example

```rust
use use_wythoff::WythoffSymbol;

let symbol = WythoffSymbol::new("3 | 4 2").expect("valid symbol");

assert_eq!(symbol.to_string(), "3 | 4 2");
```
