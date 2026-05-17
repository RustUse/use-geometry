# use-rectangle

Rectangle primitives for the RustUse geometry workspace.

`use-rectangle` provides axis-aligned rectangles as a planar-region layer over `use-bounds`.

## Example

```rust
use use_point::Point2;
use use_rectangle::Rectangle;

let rectangle = Rectangle::from_corners(Point2::new(0.0, 0.0), Point2::new(4.0, 2.0));

assert_eq!(rectangle.area(), 8.0);
```
