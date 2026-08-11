# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B contract definition is next.
- Integration branch: `main`; Node 24 CI maintenance is integrated and
  verified. Non-trivial Gate B work uses a short-lived branch.
- Current objective: freeze the five-command behavior contract (parse,
  collect, validate, init, scan) in one pass before implementing parser
  behavior. The matrix is grounded in the A1 baseline captured from the
  private Python prototype (D002, reference only) and detailed in
  `docs/planning/gate-b-contract.md`. It covers flags, versioned stdout
  machine envelopes, stderr, exit codes, and cross-cutting cases (BOM, leading
  blanks, malformed fences, pre-A1 compatibility input, unknown keys,
  profiles, semantic errors, symlinks, outside-root paths, and cross-platform
  path normalization). Intentional deviations from the baseline are recorded
  in D006; symbolic-link containment and reporting are resolved in D007.
- The private GitHub repository, initial `main` push, and first hosted Actions
  run are complete and green.
- GitHub repository bootstrap is complete; public visibility remains a
  maintainer checkpoint after the initial functional slice.
- No production CLI subcommand is complete.

## Verification

Gate B contract-foundation verification on 2026-08-11:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 4 tests passed, including static
  manifest coverage for all six initial machine protocols.
- `RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- Legacy strict validation against this repository → 13 files, 0 errors,
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

- [ ] Freeze the five-command behavior contract (parse, collect, validate, init,
      scan) in one pass.
- [x] Establish all six protocol identifiers and an initial statically checked
      manifest with one success case per command plus one machine error case.
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
