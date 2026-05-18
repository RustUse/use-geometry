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

1. `use-coordinate`
2. `use-dimension`
3. `use-angle`
4. `use-point`
5. `use-distance`
6. `use-orientation`
7. `use-bounds`
8. `use-line`
9. `use-segment`
10. `use-circle`
11. `use-sphere`
12. `use-hypersphere`
13. `use-torus`
14. `use-triangle`
15. `use-containment`
16. `use-congruence`
17. `use-similarity`
18. `use-intersection`
19. `use-dihedral`
20. `use-inversion`
21. `use-ray`
22. `use-plane`
23. `use-hyperplane`
24. `use-transform`
25. `use-affine`
26. `use-projection`
27. `use-reflection`
28. `use-curve`
29. `use-conic`
30. `use-polyline`
31. `use-bezier`
32. `use-spline`
33. `use-rectangle`
34. `use-polygon`
35. `use-simplex`
36. `use-orthotope`
37. `use-polytope`
38. `use-polyhedron`
39. `use-schlafli`
40. `use-regular-polytope`
41. `use-uniform-polytope`
42. `use-archimedean`
43. `use-catalan-solid`
44. `use-johnson-solid`
45. `use-wythoff`
46. `use-coxeter`
47. `use-incidence`
48. `use-projective`
49. `use-configuration`
50. `use-duality`
51. `use-surface`
52. `use-manifold`
53. `use-mesh`
54. `use-face`
55. `use-cell`
56. `use-complex`
57. `use-hull`
58. `use-triangulation`
59. `use-tessellation`
60. `use-voronoi`
61. `use-delaunay`
62. `use-folding`
63. `use-crease`
64. `use-origami`
65. `use-linkage`
66. `use-unfolding`
67. `use-rigid-folding`
68. `use-polyhedral-net`
69. `use-geometry`

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
