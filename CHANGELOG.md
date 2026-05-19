# Changelog

## Unreleased

## [0.2.0](https://github.com/RustUse/use-geometry/compare/use-geometry-v0.1.0...use-geometry-v0.2.0) - 2026-05-19

### Changed

- Released the `use-geometry` facade at `0.2.0` with updated dependency bounds for the geometry crates that now consume `use-vector 0.0.7`.
- Released `use-point`, `use-plane`, `use-orientation`, and `use-ray` at `0.1.0`, and `use-line` at `0.2.1`, preserving the RustUse-wide policy that crates stay below `0.3.0` until production-readiness is intended.

## [0.1.0](https://github.com/RustUse/use-geometry/compare/use-geometry-v0.0.7...use-geometry-v0.1.0) - 2026-05-19

### Changed

- Renamed the geometry line child package from the temporary `use-geometry-line` identity to `use-line` at version `0.2.0` while keeping the Rust library name `use_line`.
- Updated the `use-geometry` facade to publish as `0.1.0` and depend on `use-line` for the `line` feature.
- Documented the replacement path for the temporary `use-geometry 0.0.7` and `use-geometry-line 0.0.6` releases.

## [0.0.7](https://github.com/RustUse/use-geometry/compare/use-geometry-v0.0.6...use-geometry-v0.0.7) - 2026-05-19

### Added

- Added release automation and publish workflow scaffolding for `use-geometry`.
- Added feature-gated child-crate namespaces to the `use-geometry` facade, with one feature per child crate and `default = ["full"]` for the complete geometry surface.
- Added the `use-geometry-line` package identity for the line child crate while preserving its `use_line` Rust library name and the facade `line` feature.
- Added representative facade feature coverage for selected child namespaces and root reexports.

### Changed

- Aligned the crate metadata for crates.io and docs.rs publication.
- Converted the crates.io `use-geometry` package from the old `use-math`-hosted 2D geometry crate into the RustUse geometry workspace facade.
- Published this as a facade-only patch over already-published child crates at `0.0.6`; child crate versions are unchanged for this release.
