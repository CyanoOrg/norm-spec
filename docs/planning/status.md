# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B contract definition is next.
- Integration branch: `main`; Node 24 CI maintenance awaits GitHub verification.
- Current objective: complete the A1 CLI behavior matrix and contract fixtures
  before implementing parser behavior.
- The private GitHub repository, initial `main` push, and first hosted Actions
  run are complete and green.
- Maintainer checkpoint: push the Node 24 Action pins and confirm the next run
  is green without deprecation annotations; public visibility may follow after
  the initial functional slice.
- No production CLI subcommand is complete.

## Verification

Bootstrap verification on 2026-08-10:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 3 tests passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- The first GitHub-hosted run was green across `quality`, Linux, macOS, and
  Windows; its only annotations were Node 20 Action-runtime deprecations.

## Open-source readiness

- [x] Independent public `0.1` history and product identity.
- [x] Security policy, code of conduct, and structured issue/PR templates.
- [x] High-confidence secret, sensitive filename, and private-path history scan.
- [x] GitHub Actions references pinned to full commit SHAs.
- [x] Private GitHub repository creation, initial `main` push, and first hosted
      Actions run.
- [ ] Verify the Node 24 Action pins on GitHub without deprecation annotations
      (maintainer checkpoint).
- [ ] Configure `main` protection when repository visibility or the
      organization plan permits it.

## Open work

- [ ] Complete the documented A1 CLI behavior matrix.
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Integrate the Rust engine with `pi-norm-spec`.
- [ ] Complete cross-platform release-readiness review.
