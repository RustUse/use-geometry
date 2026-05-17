# use-reflection

Reflection primitives for the RustUse geometry workspace.

`use-reflection` starts with simple axis-aligned point reflections and can grow into line and plane
reflections later.

## Example

```rust
use use_point::Point2;
use use_reflection::AxisReflection2;

assert_eq!(AxisReflection2::AcrossX.reflect(Point2::new(2.0, 3.0)), Point2::new(2.0, -3.0));
```
