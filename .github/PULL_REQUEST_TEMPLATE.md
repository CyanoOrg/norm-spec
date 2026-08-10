## Summary

Describe the problem and the resulting behavior.

## Contract impact

- [ ] No `.norm` format, schema, CLI, or machine-protocol change.
- [ ] Contract-affecting changes include a decision, fixtures/tests, and
      compatibility guidance.

## Verification

List the commands and relevant end-to-end scenarios you ran.

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`
- [ ] `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps`
- [ ] `bash scripts/check-public-history.sh`
- [ ] Documentation and changelog are updated when needed.
