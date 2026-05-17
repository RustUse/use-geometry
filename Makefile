.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run release-readiness verify

SBOM_CRATE := use-geometry
PUBLISH_CRATES := \
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
	use-geometry

help:
	@printf "%s\n" \
		"help               Show available repository tasks" \
		"fmt                Check formatting with rustfmt" \
		"check              Run cargo check for the workspace" \
		"lint               Run clippy with warnings denied" \
		"test               Run workspace tests with all features" \
		"test-minimal       Run workspace tests with no default features" \
		"build              Build the workspace with all features" \
		"doc                Build workspace docs without dependencies" \
		"examples           Check all examples" \
		"audit              Run cargo-audit" \
		"deny               Run cargo-deny" \
		"sbom               Generate a CycloneDX SBOM" \
		"publish-dry-run    List package contents and dry-run publish each publishable crate" \
		"release-readiness  Run the pre-release validation path" \
		"verify             Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/$(SBOM_CRATE)/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run:
	@set -e; \
	for crate in $(PUBLISH_CRATES); do \
		cargo package --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

release-readiness: verify examples test-minimal publish-dry-run

verify: fmt lint test build
