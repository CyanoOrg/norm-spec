# Development Status

## Resume here

- Stage: `0.1.0-alpha.1` bootstrap.
- Branch: `docs/v0.1-public-line`.
- Current objective: establish governance, Rust workspace boundaries, A1
  behavior fixtures, and CI before implementing parser behavior.
- No production CLI subcommand is complete.

## Verification

Bootstrap verification on 2026-08-10:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 3 tests passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.

## Open work

- [ ] Complete the documented A1 CLI behavior matrix.
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Integrate the Rust engine with `pi-norm-spec`.
- [ ] Complete cross-platform release-readiness review.
