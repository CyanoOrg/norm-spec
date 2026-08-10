# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B contract definition is next.
- Integration branch: `main`; Node 24 CI maintenance is verified and ready to
  integrate.
- Current objective: freeze the five-command behavior contract (parse,
  collect, validate, init, scan) in one pass before implementing parser
  behavior. The matrix is grounded in the A1 baseline captured from the
  private Python prototype (D002, reference only) and detailed in
  `docs/planning/gate-b-contract.md`. It covers flags, versioned stdout
  machine envelopes, stderr, exit codes, and cross-cutting cases (BOM, leading
  blanks, malformed fences, pre-A1 compatibility input, unknown keys,
  profiles, semantic errors, symlinks, outside-root paths, and cross-platform
  path normalization). Intentional deviations from the baseline are recorded
  in D006; symbolic-link semantics remain an open decision and do not block
  the rest of the contract.
- The private GitHub repository, initial `main` push, and first hosted Actions
  run are complete and green.
- GitHub repository bootstrap is complete; public visibility remains a
  maintainer checkpoint after the initial functional slice.
- No production CLI subcommand is complete.

## Verification

Bootstrap verification on 2026-08-10:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 3 tests passed.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- The Node 24 GitHub-hosted verification run was green across `quality`, Linux,
  macOS, and Windows with no annotations.

## Open-source readiness

- [x] Independent public `0.1` history and product identity.
- [x] Security policy, code of conduct, and structured issue/PR templates.
- [x] High-confidence secret, sensitive filename, and private-path history scan.
- [x] GitHub Actions references pinned to full commit SHAs.
- [x] Private GitHub repository creation, initial `main` push, and first hosted
      Actions run.
- [x] Node 24 Action pins verified green on GitHub without annotations.
- [ ] Configure `main` protection when repository visibility or the
      organization plan permits it.

## Open work

- [ ] Freeze the five-command behavior contract (parse, collect, validate, init,
      scan) in one pass.
- [ ] Decide symbolic-link semantics (follow, reject, or report) before
      collect, validate, and scan implementation (D006 open item).
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Integrate the Rust engine with `pi-norm-spec`.
- [ ] Complete cross-platform release-readiness review.
