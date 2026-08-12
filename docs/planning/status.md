# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B and Gate C are complete. Gate D integration
  readiness planning is active.
- Integration state: local and remote `main` are synchronized at `c84aafb`;
  D1 implementation, hosted verification, and documentation are integrated.
- Current resume point: D1 high-level Rust consumption and packaging are
  complete. The maintainer approved the exact D2/D3 compatibility and
  conformance protocol recorded by D015; Batch 2 implementation is next.
- The maintainer confirmed standalone-first product positioning, one canonical
  framework-neutral Skill in this repository, explicit failure without engine
  fallback, downstream host adapters, and standalone adoption as Gate D scope.
- D012 embeds the seven repository-owned templates without ambient discovery
  and fixes scan's infrastructure ignore set, naming precedence, ordering,
  recurrence limit, coverage rounding, and no-inference boundary.
- `norm-spec-core` owns parse, collection rules, Schema/profile/semantic
  validation, response contracts, and deterministic scan aggregation. The
  repository-root `norm-spec` facade owns packaged Schema/template assets and
  explicit collect/validation filesystem orchestration. `norm-spec-cli` owns
  arguments, presentation, exits, and scan/init write orchestration.
- All 4 global, 13 parse, 11 collect, 35 validate, 9 init, and 10 scan cases
  execute the compiled binary in isolated roots with no skips: 82 of 82 frozen
  cases.
- All five production commands and the D1 consumer-resolvable Rust API are
  functional. Compatibility, conformance, standalone adoption, and Skill work
  remain open; D1 completion does not claim those later results.
- D014 selected a repository-root `norm-spec` facade after an isolated D1.1
  spike. The facade layout packaged the existing root assets, passed Rust 1.97
  workspace candidate verification with the unpublished core package, and ran
  collect plus validation from an unrelated exact-revision Git consumer.
- The production facade now exposes typed collect and validation requests,
  packaged Schema/template access, completed validation results, and typed
  pre-evaluation failures without process exit codes. The CLI delegates to it
  and retains all 82 frozen observable cases.
- `scripts/check-packages.sh` verifies required package contents, verifies the
  core → facade → CLI workspace candidate through Cargo's temporary registry,
  and runs collect plus validation from an unrelated exact-revision Git
  consumer. Hosted D1 evidence is maintainer-confirmed green.
- The private GitHub repository and hosted Actions are green through D1
  candidate `07a95c6`. Public visibility remains a maintainer checkpoint.
- D015 selects machine-default compatibility discovery,
  exact identifier membership, a locked 82-case A1 bundle, a separate
  `norm-spec-conformance` binary, explicit complete/incomplete reports, and no
  candidate, bundle, source, or version fallback. Observable implementation
  remains open until Batch 2 is green.

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

Gate D1 local verification on 2026-08-12:

- `cargo clippy --workspace --all-targets --all-features -- -D warnings` →
  green.
- `cargo test --workspace --all-features` → 53 Rust test functions passed;
  all 82 compiled-binary cases still executed without skips.
- `cargo package --workspace --allow-dirty` → core, facade, and CLI verified in
  dependency order; their candidates contained 13, 31, and 14 files.
- `bash scripts/check-packages.sh` → all required Schema/template assets were
  present, no CLI contract bundle leaked into the facade, and an unrelated
  exact-revision Git consumer completed two-level collect plus validation.
- `cargo fmt --check`, `RUSTDOCFLAGS='-D warnings' cargo doc --workspace
  --no-deps`, and `bash scripts/check-public-history.sh` → green.
- strict repository self-validation → seven files, zero errors, zero warnings;
  structural scan → 34 directories and seven `.norm` files.
- Hosted Linux, macOS, and Windows CI for D1 candidate `07a95c6` →
  maintainer-confirmed green. This verification record is documentation-only
  and does not change the tested implementation candidate.

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
- [x] Implement the selected facade, CLI delegation, and permanent external
      package-consumer checks.
- [x] Approve the D2/D3 command, envelope, bundle identity, report, and exit
      protocol; record D015 before implementation.
- [ ] Add versioned compatibility discovery and arbitrary-candidate
      conformance without skips after D015.
- [ ] Prove standalone adoption from an isolated candidate without a plugin.
- [ ] Add and validate the canonical framework-neutral norm-spec Skill without
      parser, collect, or validation fallback.
- [ ] Complete cross-platform distribution and release-readiness review
      (Gate E).
