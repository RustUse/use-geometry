# use-geometry

Pure geometry primitives and foundational vocabulary for RustUse.

`use-geometry` is the RustUse set for small, composable geometry building blocks. The top-level
`use-geometry` crate is a facade-only crate: it contains crate documentation, public reexports, and
a prelude module. Implementation lives in focused child crates with explicit boundaries.

The set is intentionally broad inside pure geometry. Many crates begin with primitive value types,
count records, or descriptors and can grow into algorithms incrementally without forcing the facade
to become a monolith.

## Taxonomy

| Group                            | Crates                                                                                                                |
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

Choose the facade crate when you want one dependency for the common geometry surface:

```toml
[dependencies]
use-geometry = "0.0.6"
```

Choose child crates directly when you want a narrower dependency surface:

```toml
[dependencies]
use-point = "0.0.6"
use-polygon = "0.0.6"
```

## Usage

```rust
use use_geometry::{Point2, Triangle};

let triangle = Triangle::new(
    Point2::new(0.0, 0.0),
    Point2::new(4.0, 0.0),
    Point2::new(0.0, 3.0),
);

assert_eq!(triangle.area(), 6.0);
assert_eq!(triangle.perimeter(), 12.0);
```

## Scope

- Pure geometry primitives, descriptors, and direct measurement helpers.
- Small geometry algorithms with explicit naming and predictable behavior.
- Validated constructors for external or user-provided numeric input.
- Focused crate boundaries that compose cleanly with other RustUse sets.
- Incremental algorithm growth in child crates, not in the facade.
