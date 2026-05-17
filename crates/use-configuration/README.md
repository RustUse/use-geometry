# use-configuration

Geometric configuration primitives for the RustUse geometry workspace.

`use-configuration` stores lightweight configuration metadata, including the Schlafli double six,
with incidence support from `use-incidence`. It does not implement cubic-surface algorithms.

## Example

```rust
use use_configuration::SchlafliDoubleSix;

let double_six = SchlafliDoubleSix::new();

assert_eq!(double_six.configuration().point_count(), 12);
assert_eq!(double_six.incidence_structure().line_count(), 30);
```
