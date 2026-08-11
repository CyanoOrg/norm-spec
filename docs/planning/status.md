# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B is complete and Gate C is next.
- Integration branch: `main`; Gate B closed on the short-lived branch
  `codex/docs-gate-b-contract-fixes` and awaits maintainer push/CI/merge.
- Completed objective: the five-command behavior contract (parse, collect,
  validate, init, scan) is frozen before parser implementation. D006 records
  baseline deviations, D007 resolves symbolic-link semantics, and D008 binds
  91 requirements to 82 cases with complete flag, protocol, stream, exit,
  path, layout, and cross-cutting behavior coverage.
- Next objective: Gate C begins with YAML parser and Draft 7 validator
  dependency selection/spikes. Dependency selection is a maintainer checkpoint;
  no dependency or production command implementation has started.
- The private GitHub repository, initial `main` push, and first hosted Actions
  run are complete and green.
- GitHub repository bootstrap is complete; public visibility remains a
  maintainer checkpoint after the initial functional slice.
- No production CLI subcommand is complete.

## Verification

Gate B closure verification on 2026-08-11:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 4 tests passed. The static contract
  gate checks 91 requirements, 82 cases, all documented flags, all six machine
  protocols, every command's machine success/error modes, applicable human
  modes, and all referenced fixture/expected/layout assets.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- Legacy strict validation against this repository → 16 files, 0 errors,
  0 warnings. This is transitional evidence, not Rust self-hosting.
- The Node 24 GitHub-hosted verification run was green across `quality`, Linux,
  macOS, and Windows with no annotations at `origin/main`; the current Gate B
  branch still requires hosted CI after push.

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

- [x] Freeze the five-command behavior contract (parse, collect, validate, init,
      scan) in one pass.
- [x] Establish all six protocol identifiers and a statically checked manifest
      with complete requirement, flag, success/error, and asset coverage.
- [x] Decide symbolic-link containment, rejection, and reporting semantics
      before collect, validate, and scan implementation (D007).
- [ ] Select and spike the YAML parser and Draft 7 validator dependencies.
- [ ] Implement A1 parse and canonical structured errors.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Provide consumer-resolvable Rust/machine contracts and a conformance
      entry point without sibling-path coupling.
- [ ] Complete cross-platform release-readiness review.
