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
12. `use-triangle`
13. `use-containment`
14. `use-congruence`
15. `use-similarity`
16. `use-intersection`
17. `use-ray`
18. `use-plane`
19. `use-hyperplane`
20. `use-transform`
21. `use-affine`
22. `use-projection`
23. `use-reflection`
24. `use-curve`
25. `use-conic`
26. `use-polyline`
27. `use-bezier`
28. `use-spline`
29. `use-rectangle`
30. `use-polygon`
31. `use-simplex`
32. `use-orthotope`
33. `use-polytope`
34. `use-polyhedron`
35. `use-regular-polytope`
36. `use-surface`
37. `use-mesh`
38. `use-face`
39. `use-cell`
40. `use-hull`
41. `use-triangulation`
42. `use-tessellation`
43. `use-voronoi`
44. `use-delaunay`
45. `use-folding`
46. `use-crease`
47. `use-origami`
48. `use-linkage`
49. `use-unfolding`
50. `use-rigid-folding`
51. `use-polyhedral-net`
52. `use-geometry`

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
  use-triangle \
  use-containment \
  use-congruence \
  use-similarity \
  use-intersection \
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
  use-regular-polytope \
  use-surface \
  use-mesh \
  use-face \
  use-cell \
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
