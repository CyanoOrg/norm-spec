# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B and Gate C are complete. Gate D integration
  readiness planning is active.
- Integration branch: local `main` at `edd08c6`; the maintainer-confirmed
  hosted Gate C baseline remains `71c2a88` until the local Gate D planning
  commits are pushed with an implementation candidate.
- Current topic: `codex/feat-gate-d-api`; D1 high-level Rust consumption and
  packaging are active.
- The maintainer confirmed standalone-first product positioning, one canonical
  framework-neutral Skill in this repository, explicit failure without engine
  fallback, downstream host adapters, and standalone adoption as Gate D scope.
- D012 embeds the seven repository-owned templates without ambient discovery
  and fixes scan's infrastructure ignore set, naming precedence, ordering,
  recurrence limit, coverage rounding, and no-inference boundary.
- `norm-core` now owns parse, collect, schema/profile/semantic validation, init
  and scan response contracts, plus deterministic scan aggregation. `norm-cli`
  owns arguments, embedded assets, filesystem orchestration, presentation, and
  exits.
- All 4 global, 13 parse, 11 collect, 35 validate, 9 init, and 10 scan cases
  execute the compiled binary in isolated roots with no skips: 82 of 82 frozen
  cases.
- All five production commands are functional. Gate D consumer-resolvable API,
  compatibility, conformance, standalone adoption, and Skill work remain open;
  Gate C completion does not claim those later gates.
- D014 selected a repository-root `norm-spec` facade after an isolated D1.1
  spike. The facade layout packaged the existing root assets, passed Rust 1.97
  workspace candidate verification with the unpublished core package, and ran
  collect plus validation from an unrelated exact-revision Git consumer.
- The production facade, CLI delegation, and permanent packaging tests are not
  implemented yet; the spike is boundary evidence, not Gate D1 completion.
- The private GitHub repository and hosted Actions are established and green
  through integrated Gate C. Public visibility remains a maintainer checkpoint.

## Verification

Full Gate C local verification on 2026-08-11:

- `cargo fmt --check` → green.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` →
  green.
- `cargo test --workspace --all-features` → 48 Rust test functions passed. The
  static 91-requirement / 82-case inventory gate and all 82 compiled-binary
  cases passed with asserted streams, protocols, output data, and exits.
- All seven embedded templates parse as A1, validate strictly against their
  profile schemas, and the init integration test proves byte-for-byte output.
- `cargo run -p norm-spec-cli -- validate --all --strict --json` → five files,
  zero errors, zero warnings.
- `cargo run -p norm-spec-cli -- scan --root . --text` → 27 directories, five
  `.norm` files, no symlinks, and `0.185` coverage; infrastructure directories
  were excluded.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` → green.
- `bash scripts/check-public-history.sh` → green across the current index and
  all reachable commits.
- `cargo install --path crates/norm-cli --root <isolated> --locked` → green;
  the installed binary initialized a module template, strictly validated it,
  and scanned its isolated root with full coverage.
- Hosted Linux, macOS, and Windows evidence for the final init/scan branch and
  integrated Gate C `main` is maintainer-confirmed green.

## Open-source readiness

- [x] Independent public `0.1` history and product identity.
- [x] Security policy, code of conduct, and structured issue/PR templates.
- [x] High-confidence secret, sensitive filename, and private-path history scan.
- [x] GitHub Actions references pinned to full commit SHAs.
- [x] Private GitHub repository creation, initial `main` push, and hosted
      Actions.
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
- [x] Implement A1/legacy parse and all executable global/parse cases.
- [x] Implement deterministic path-scoped collect and all executable collect
      cases.
- [x] Implement schema, profile, and semantic validation plus all executable
      validate cases.
- [x] Implement self-contained init and deterministic structural scan plus all
      executable init/scan cases.
- [x] Decide standalone-first product and canonical Skill ownership (D013).
- [x] Select the packaged high-level Rust consumer boundary without
      sibling-path or copied-asset coupling (D014 and the isolated D1.1 spike).
- [ ] Implement the selected facade, CLI delegation, and permanent external
      package-consumer checks.
- [ ] Add versioned compatibility discovery and arbitrary-candidate
      conformance without skips.
- [ ] Prove standalone adoption from an isolated candidate without a plugin.
- [ ] Add and validate the canonical framework-neutral norm-spec Skill without
      parser, collect, or validation fallback.
- [ ] Complete cross-platform distribution and release-readiness review
      (Gate E).
