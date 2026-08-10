# Contributing

Read `AGENTS.md` before opening a change.

## Before coding

1. Confirm the change belongs in norm-spec rather than a framework consumer.
2. Check `docs/planning/status.md` and the active execution plan.
3. Record non-trivial scope or architecture decisions in `docs/decisions.md`.
4. Create a short-lived branch with the appropriate prefix.

## Pull-request gate

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
bash scripts/check-public-history.sh
```

A parser, schema, validator, or machine-output change must include contract
coverage. Document intentional compatibility changes and migration guidance.
