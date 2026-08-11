# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B plus Gate C's parse, collect, and validate
  executable vertical slices are complete locally.
- Integration branch: `main` at `09b0488`. The maintainer pushed the collect
  integration, verified every hosted CI job including Windows, and removed the
  remote topic branch.
- Current branch: `codex/feat-gate-c-validate`. It contains the approved Draft
  7 dependency boundary, validation core and CLI, 35 executable validate
  cases, and this documentation closeout. Maintainer push, hosted CI, and
  integration remain the checkpoint for this slice.
- D010 selects `jsonschema` 0.49.9 with default features disabled, explicit
  Draft 7 compilation, format checks, and caller-supplied in-memory resources.
  Missing references fail; the core has no filesystem or network fallback.
- D011 maps generic Draft 7 and profile-required-field failures to the stable
  `norm/schema/invalid` diagnostic while retaining the specialized frozen
  unknown-key and version-format codes.
- `norm-core` now owns structural, profile, lifecycle, single-source, and
  reference validation. `norm-cli` owns embedded/explicit schema loading,
  root-contained traversal, reference resolution, human/JSON presentation,
  strict-warning exits, and preflight failures.
- All 4 global, 13 parse, 11 collect, and 35 validate cases execute the compiled
  binary in isolated roots with no skips. `init` and `scan` remain explicit
  exit-2 stubs and are the next Gate C objective.
- The Rust validator now self-validates all five `.norm` files in this
  repository with both its embedded and explicit repository schema bundles.
- The first hosted Windows validate run exposed string-based expansion of
  composite absolute-path placeholders in the language-neutral test runner.
  Commit `59f32b7` now joins placeholder suffixes with native path APIs; the
  hosted Windows rerun remains pending.
- The private GitHub repository, initial `main` push, and hosted Actions are
  established and green through the collect slice. Public visibility remains
  a maintainer checkpoint after the initial functional slice.

## Verification

Gate C validate-slice local verification on 2026-08-11:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` →
  green.
- `cargo test --workspace --all-features` → 40 Rust test functions passed. In
  addition to the static 91-requirement / 82-case inventory gate, all 63
  currently applicable global, parse, collect, and validate cases execute the
  compiled binary with asserted streams, protocols, output data, and exits.
- `cargo run -p norm-spec-cli -- validate --all --strict --json --pretty` →
  five files, zero errors, zero warnings using embedded schemas.
- `cargo run -p norm-spec-cli -- validate --all --strict --schema-dir schema
  --json` → five files, zero errors, zero warnings using the explicit bundle.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- `cargo install --path crates/norm-cli --root <isolated> --locked` → green;
  the installed binary strictly validated this repository.
- Hosted Linux, macOS, and Windows evidence for the validate slice remains
  pending until the maintainer pushes this branch.

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

- [x] Freeze the five-command behavior contract (parse, collect, validate,
      init, scan) in one pass.
- [x] Establish all six protocol identifiers and a statically checked manifest
      with complete requirement, flag, success/error, and asset coverage.
- [x] Decide symbolic-link containment, rejection, and reporting semantics
      before collect, validate, and scan implementation (D007).
- [x] Select and spike the YAML parser dependency (D009).
- [x] Select and spike the Draft 7 validator dependency (D010).
- [x] Implement A1/legacy parse, canonical structured parse errors, and all
      executable global/parse contract cases.
- [x] Implement deterministic path-scoped collect and all executable collect
      contract cases.
- [x] Implement schema, profile, and semantic validation plus all executable
      validate contract cases.
- [ ] Implement init and structural scan.
- [ ] Provide consumer-resolvable Rust/machine contracts and a conformance
      entry point without sibling-path coupling.
- [ ] Complete cross-platform release-readiness review.
