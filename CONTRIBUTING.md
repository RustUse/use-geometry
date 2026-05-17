# Contributing

RustUse/use-geometry is intentionally small and focused. Contributions should keep the public API explicit, typed, pragmatic, and aligned with the facade-plus-child-crate workspace layout.

## Development flow

1. Make the smallest useful change.
2. Add or update tests for public behavior changes.
3. Prefer direct helpers over broad framework abstractions.
4. Put implementation in the focused child crate that owns the behavior; keep the `use-geometry` facade crate reexport-only.
5. Keep public docs aligned with the actual crate behavior, metadata, and release surface.

## Local validation

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Scope guidance

- `use-geometry` is the RustUse geometry set, with a thin facade crate over focused leaf crates.
- Prefer reusable typed primitives over framework-heavy wrappers.
- Keep examples and README usage aligned with the current crate surface.
- Avoid widening the API surface without a clear crate-level use case.
