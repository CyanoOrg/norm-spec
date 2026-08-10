# Development Status

## Resume here

- Stage: `0.1.0-alpha.1` bootstrap.
- Branch: `chore/open-source-readiness`.
- Current objective: establish governance, Rust workspace boundaries, A1
  behavior fixtures, and CI before implementing parser behavior.
- Current side objective: finish public-repository safeguards before GitHub
  publication.
- No production CLI subcommand is complete.

## Verification

Bootstrap verification on 2026-08-10:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 3 tests passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.

## Open-source readiness

- [x] Independent public `0.1` history and product identity.
- [x] Security policy, code of conduct, and structured issue/PR templates.
- [x] High-confidence secret, sensitive filename, and private-path history scan.
- [x] GitHub Actions references pinned to full commit SHAs.
- [ ] GitHub repository creation, rules, and initial push (maintainer checkpoint).

## Open work

- [ ] Complete the documented A1 CLI behavior matrix.
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Integrate the Rust engine with `pi-norm-spec`.
- [ ] Complete cross-platform release-readiness review.
