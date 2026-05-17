# Releasing

This workspace supports a manual, safety-first publish flow. It does not publish automatically on
normal pushes.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for the geometry workspace crates.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Publish order

Publish the workspace in dependency order:

1. `use-geometry-core`
2. `use-coordinate`
3. `use-dimension`
4. `use-angle`
5. `use-point`
6. `use-distance`
7. `use-orientation`
8. `use-bounds`
9. `use-line`
10. `use-segment`
11. `use-circle`
12. `use-sphere`
13. `use-hypersphere`
14. `use-torus`
15. `use-triangle`
16. `use-containment`
17. `use-congruence`
18. `use-similarity`
19. `use-intersection`
20. `use-dihedral`
21. `use-inversion`
22. `use-ray`
23. `use-plane`
24. `use-hyperplane`
25. `use-transform`
26. `use-affine`
27. `use-projection`
28. `use-reflection`
29. `use-curve`
30. `use-conic`
31. `use-polyline`
32. `use-bezier`
33. `use-spline`
34. `use-rectangle`
35. `use-polygon`
36. `use-simplex`
37. `use-orthotope`
38. `use-polytope`
39. `use-polyhedron`
40. `use-schlafli`
41. `use-regular-polytope`
42. `use-uniform-polytope`
43. `use-archimedean`
44. `use-catalan-solid`
45. `use-johnson-solid`
46. `use-wythoff`
47. `use-coxeter`
48. `use-incidence`
49. `use-projective`
50. `use-configuration`
51. `use-duality`
52. `use-surface`
53. `use-manifold`
54. `use-mesh`
55. `use-face`
56. `use-cell`
57. `use-complex`
58. `use-hull`
59. `use-triangulation`
60. `use-tessellation`
61. `use-voronoi`
62. `use-delaunay`
63. `use-folding`
64. `use-crease`
65. `use-origami`
66. `use-linkage`
67. `use-unfolding`
68. `use-rigid-folding`
69. `use-polyhedral-net`
70. `use-geometry`

The `Publish` workflow uses that order automatically when `crate = all`.

## Dry-run publish

Use the `Publish` workflow with:

- `crate = all` or one specific crate name from the publish order above
- `dry_run = true`

This is the default mode and the safest way to validate publish packaging before a real release.

## Manual publish

Use the `Publish` workflow with:

- `crate = all` or one specific crate name from the publish order above
- `dry_run = false`

The workflow runs formatting, linting, tests, and `cargo check` before it attempts any publish
step.

## Post-initial-release automation

After the first manual crates.io release for every publishable workspace crate exists, the
repository can use the `release-plz` workflows for follow-up releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request based on `release-plz.toml` and the current
  changelog rules

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Purpose: confirms every publishable workspace crate already exists on crates.io, then runs
  `release-plz release`

Real `release-plz` publishes still require `CARGO_REGISTRY_TOKEN` unless the repository later
moves to trusted publishing.

## Local dry-run example

```sh
for crate in \
  use-geometry-core \
  use-coordinate \
  use-dimension \
  use-angle \
  use-point \
  use-distance \
  use-orientation \
  use-bounds \
  use-line \
  use-segment \
  use-circle \
  use-sphere \
  use-hypersphere \
  use-torus \
  use-triangle \
  use-containment \
  use-congruence \
  use-similarity \
  use-intersection \
  use-dihedral \
  use-inversion \
  use-ray \
  use-plane \
  use-hyperplane \
  use-transform \
  use-affine \
  use-projection \
  use-reflection \
  use-curve \
  use-conic \
  use-polyline \
  use-bezier \
  use-spline \
  use-rectangle \
  use-polygon \
  use-simplex \
  use-orthotope \
  use-polytope \
  use-polyhedron \
  use-schlafli \
  use-regular-polytope \
  use-uniform-polytope \
  use-archimedean \
  use-catalan-solid \
  use-johnson-solid \
  use-wythoff \
  use-coxeter \
  use-incidence \
  use-projective \
  use-configuration \
  use-duality \
  use-surface \
  use-manifold \
  use-mesh \
  use-face \
  use-cell \
  use-complex \
  use-hull \
  use-triangulation \
  use-tessellation \
  use-voronoi \
  use-delaunay \
  use-folding \
  use-crease \
  use-origami \
  use-linkage \
  use-unfolding \
  use-rigid-folding \
  use-polyhedral-net \
  use-geometry; do
  cargo publish -p "$crate" --dry-run
done
```

## Semver notes

- Patch bumps are for compatible fixes and small additive maintenance changes.
- Minor bumps are for additive API changes during `0.x` development.
- Major bumps are for breaking changes once the crate stabilizes at `1.0.0`.
- Pre-release identifiers should remain intentional and explicit.

## Permanent version warning

Published crates.io versions are permanent. You cannot replace an already published version with
new contents, so verify crate metadata, README examples, and changelog entries before any real
publish.
