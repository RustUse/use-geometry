# use-geometry

`use-geometry` is the facade crate for the RustUse geometry workspace.

It reexports the focused child crates that make up the set so callers can depend on one crate for
pure geometry primitives, descriptors, validation errors, and direct measurement helpers while the
implementation stays split into explicit child crates.

The facade contains only crate-level documentation, public reexports, and the `prelude` module.

## Taxonomy

| Group                            | Reexported crates                                                                                                     |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Foundations                      | `use-geometry-core`, `use-coordinate`, `use-dimension`, `use-angle`, `use-point`                                      |
| Affine primitives                | `use-bounds`, `use-line`, `use-ray`, `use-segment`, `use-plane`, `use-hyperplane`                                     |
| Transformations                  | `use-transform`, `use-affine`, `use-projection`, `use-reflection`                                                     |
| Metric relations                 | `use-distance`, `use-orientation`, `use-intersection`, `use-containment`, `use-congruence`, `use-similarity`          |
| Curves and conics                | `use-circle`, `use-conic`, `use-curve`, `use-polyline`, `use-bezier`, `use-spline`                                    |
| Planar regions                   | `use-triangle`, `use-rectangle`, `use-polygon`                                                                        |
| Polytopes and solids             | `use-simplex`, `use-orthotope`, `use-polytope`, `use-polyhedron`, `use-regular-polytope`                              |
| Surfaces and representations     | `use-surface`, `use-mesh`, `use-face`, `use-cell`                                                                     |
| Constructions and decompositions | `use-hull`, `use-triangulation`, `use-tessellation`, `use-voronoi`, `use-delaunay`                                    |
| Folding and unfolding            | `use-folding`, `use-crease`, `use-origami`, `use-linkage`, `use-unfolding`, `use-rigid-folding`, `use-polyhedral-net` |

## Installation

```toml
[dependencies]
use-geometry = "0.0.6"
```

## Example

```rust
use use_geometry::{
  Aabb2, Circle, Line2, Orientation2, Point2, Polygon, Segment2, Triangle, distance_2d,
  try_orientation_2d,
};

let a = Point2::try_new(0.0, 0.0)?;
let b = Point2::try_new(4.0, 0.0)?;
let c = Point2::try_new(0.0, 3.0)?;

let segment = Segment2::try_new(a, b)?;
let triangle = Triangle::try_new(a, b, c)?;
let circle = Circle::try_new(a, 3.0)?;
let line = Line2::try_from_points(a, b)?;
let bounds = Aabb2::from_points(a, c);
let polygon = Polygon::new(vec![a, b, c]);

assert_eq!(segment.midpoint(), Point2::new(2.0, 0.0));
assert_eq!(triangle.area(), 6.0);
assert_eq!(polygon.area(), 6.0);
assert_eq!(try_orientation_2d(a, b, c)?, Orientation2::CounterClockwise);
assert_eq!(distance_2d(a, b), 4.0);
assert!(circle.contains_point(Point2::new(0.0, 3.0)));
assert!(line.contains_point(Point2::new(2.0, 0.0)));
assert!(bounds.contains_point(Point2::new(0.0, 1.5)));
# Ok::<(), use_geometry::GeometryError>(())
```

## Notes

- Root exports are direct glob reexports from child crates.
- The prelude reexports the same facade surface.
- `use-vector` remains in the `use-math` workspace and is used directly by child crates that need
  vector types.
