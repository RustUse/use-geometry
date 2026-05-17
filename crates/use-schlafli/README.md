# use-schlafli

Schlafli symbol notation for the RustUse geometry workspace.

`use-schlafli` owns Schlafli symbol values such as `{3, 4, 3}`. It provides lightweight notation
helpers and display formatting without attempting a full parser or classification engine.

## Example

```rust
use use_schlafli::SchlafliSymbol;

let symbol = SchlafliSymbol::new(vec![3, 4, 3]).expect("valid symbol");

assert_eq!(symbol.rank(), 4);
assert_eq!(symbol.to_string(), "{3, 4, 3}");
```
