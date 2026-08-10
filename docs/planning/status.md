# Development Status

## Resume here

- Stage: `0.4.0-alpha.1` bootstrap.
- Branch: `main` after bootstrap integration.
- Current objective: establish governance, Rust workspace boundaries, legacy
  behavior fixtures, and CI before porting parser behavior.
- No production CLI subcommand is complete.

## Verification

Bootstrap verification on 2026-08-10:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 3 tests passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- Legacy `norm validate --all --strict` against this repository → 13 files,
  0 errors, 0 warnings.

## Open work

- [ ] Capture the complete black-box Python CLI behavior matrix.
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Port init and structural scan.
- [ ] Integrate the Rust engine with `pi-norm-spec`.
- [ ] Complete cross-platform release and cutover review.
