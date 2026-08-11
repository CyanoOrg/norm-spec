# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B is complete and Gate C's first executable
  vertical slice is implementation-complete locally.
- Integration branch: `main`; current work is on the short-lived branch
  `codex/feat-gate-c-parse`, based on the Gate B head that the maintainer pushed
  and verified in hosted CI. The Gate C branch still awaits push, hosted CI,
  review, and merge.
- Completed objective: the five-command behavior contract (parse, collect,
  validate, init, scan) is frozen before parser implementation. D006 records
  baseline deviations, D007 resolves symbolic-link semantics, and D008 binds
  91 requirements to 82 cases with complete flag, protocol, stream, exit,
  path, layout, and cross-cutting behavior coverage.
- Completed Gate C slice: D009 selects `serde-saphyr`'s deserialize-only surface
  and `clap` 4; `norm-core` implements A1 and explicit legacy parsing with
  versioned parse/error models; `norm parse`, global help, and global version
  are executable. All 4 global and 13 parse manifest cases run as isolated
  black-box tests with no skips.
- Next objective after this branch integrates: implement deterministic,
  root-contained `collect` and its executable contract cases. Draft 7 validator
  selection remains a maintainer checkpoint for the validation slice.
- The private GitHub repository, initial `main` push, and first hosted Actions
  run are complete and green.
- GitHub repository bootstrap is complete; public visibility remains a
  maintainer checkpoint after the initial functional slice.
- `parse` is the first complete production CLI subcommand. `collect`,
  `validate`, `init`, and `scan` remain explicit exit-2 stubs.

## Verification

Gate C parse-slice verification on 2026-08-11:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` → green.
- `cargo test --workspace --all-features` → 19 tests passed. In addition to the
  static 91-requirement / 82-case inventory gate, all 4 global and 13 parse
  cases now execute the compiled binary in isolated roots with asserted
  streams, protocols, output data, and exit codes.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- Legacy strict validation against this repository → 16 files, 0 errors,
  0 warnings. This is transitional evidence, not Rust self-hosting.
- `cargo install --path crates/norm-cli --root <isolated> --locked` → green;
  the installed binary returned the expected version/help, parsed the minimal
  A1 fixture, and returned the frozen JSON error with exit `1` for a malformed
  fence.
- The maintainer reported the Node 24 Gate B run green without annotations.
  Current Gate C cross-platform hosted CI remains pending until push.

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
- [x] Select and spike the YAML parser dependency (D009).
- [ ] Select and spike the Draft 7 validator dependency.
- [x] Implement A1/legacy parse, canonical structured parse errors, and all
      executable global/parse contract cases.
- [ ] Implement path-scoped collect.
- [ ] Implement schema, profile, and semantic validation.
- [ ] Implement init and structural scan.
- [ ] Provide consumer-resolvable Rust/machine contracts and a conformance
      entry point without sibling-path coupling.
- [ ] Complete cross-platform release-readiness review.
