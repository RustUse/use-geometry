# use-geometry

Pure geometry primitives and foundational vocabulary for RustUse.

`use-geometry` is the RustUse set for small, composable geometry building blocks. The top-level
`use-geometry` crate is a feature-gated facade-only crate: it contains crate documentation, public
reexports, child-crate namespace modules, and a prelude module. Implementation lives in focused
child crates with explicit boundaries.

The `0.2.1` facade release depends on geometry line primitives through `use-line 0.2.1`
and geometric complex primitives through `use-geometric-complex 0.0.6`. Most child
geometry crates remain on their already-published `0.0.6` versions.

The set is intentionally broad inside pure geometry. Many advanced families begin with primitive
value types, metadata records, or descriptors so algorithms can grow inside child crates without
turning the facade into a monolith. Named objects belong inside family crates rather than standalone
object crates.

## Taxonomy

| Group                                              | Crates                                                                                                                                                |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Foundations                                        | `use-coordinate`, `use-dimension`, `use-angle`, `use-point`                                                                                           |
| Affine and Euclidean primitives                    | `use-bounds`, `use-line`, `use-ray`, `use-segment`, `use-plane`, `use-hyperplane`, `use-circle`, `use-sphere`, `use-hypersphere`, `use-torus` |
| Transformations                                    | `use-transform`, `use-affine`, `use-projection`, `use-reflection`, `use-inversion`                                                                    |
| Metric and relational geometry                     | `use-distance`, `use-orientation`, `use-intersection`, `use-containment`, `use-congruence`, `use-similarity`, `use-dihedral`                          |
| Curves and conics                                  | `use-conic`, `use-curve`, `use-polyline`, `use-bezier`, `use-spline`                                                                                  |
| Surfaces and manifolds                             | `use-surface`, `use-manifold`                                                                                                                         |
| Planar regions                                     | `use-triangle`, `use-rectangle`, `use-polygon`                                                                                                        |
| Polytopes and solids                               | `use-simplex`, `use-orthotope`, `use-polytope`, `use-polyhedron`, `use-regular-polytope`, `use-archimedean`, `use-catalan-solid`, `use-johnson-solid` |
| Polytope notation and classification               | `use-schlafli`, `use-wythoff`, `use-coxeter`, `use-uniform-polytope`                                                                                  |
| Incidence, projective geometry, and configurations | `use-incidence`, `use-projective`, `use-configuration`, `use-duality`                                                                                 |
| Representations and complexes                      | `use-mesh`, `use-face`, `use-cell`, `use-geometric-complex`                                                                                           |
| Constructions and decompositions                   | `use-hull`, `use-triangulation`, `use-tessellation`, `use-voronoi`, `use-delaunay`                                                                    |
| Folding and unfolding                              | `use-folding`, `use-crease`, `use-origami`, `use-linkage`, `use-unfolding`, `use-rigid-folding`, `use-polyhedral-net`                                 |

## Placement Notes

- The facade crate only reexports child crates.
- Named objects live in family crates: cuboctahedron in `use-archimedean`, 24-cell, 600-cell, and 120-cell in `use-regular-polytope`, and Schlafli double six in `use-configuration`.
- Schlafli symbols live in `use-schlafli`.
- Hypersphere and 3-sphere vocabulary lives in `use-hypersphere`; ordinary three-dimensional sphere measurements live in `use-sphere`.
- Dihedral angles live in `use-dihedral`.
- Infinite line primitives publish as `use-line` starting at `0.2.0`; the Rust library name is `use_line`.
- Geometric complex primitives publish as `use-geometric-complex`; the Rust library name remains `use_complex`.
- Polyhedral nets use `use-polyhedral-net`; `use-net` is reserved for the RustUse networking set.
- `use-vector` remains in the sibling `use-math` workspace and is used by child crates that need vector primitives.
- `use-geode` remains in `use-math` and is not part of spatial geometry.

## Installation

Choose the facade crate when you want one dependency for the common geometry surface:

```toml
[dependencies]
use-geometry = "0.2.1"
```

Choose child crates directly when you want a narrower dependency surface:

```toml
[dependencies]
use-point = "0.0.6"
use-schlafli = "0.0.6"
use-regular-polytope = "0.0.6"
```

You can also keep the facade but enable only selected child features:

```toml
[dependencies]
use-geometry = { version = "0.2.1", default-features = false, features = ["point", "schlafli", "regular-polytope"] }
```

## Usage

```rust
use use_geometry::{Point2, RegularPolytope4, Sphere, Triangle};

let triangle = Triangle::new(
    Point2::new(0.0, 0.0),
    Point2::new(4.0, 0.0),
    Point2::new(0.0, 3.0),
);
let sphere = Sphere::new(3.0).expect("valid sphere");

assert_eq!(triangle.area(), 6.0);
assert_eq!(sphere.diameter(), 6.0);
assert_eq!(RegularPolytope4::TwentyFourCell.schlafli_symbol().to_string(), "{3, 4, 3}");
```

## Scope

- Pure geometry primitives, descriptors, notation, and direct measurement helpers.
- Small geometry algorithms with explicit naming and predictable behavior.
- Validated constructors for external or user-provided numeric input.
- Focused crate boundaries that compose cleanly with other RustUse sets.
- Incremental algorithm growth in child crates, not in the facade.
