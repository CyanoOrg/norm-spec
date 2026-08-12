# Development Status

## Resume here

- Stage: `0.1.0-alpha.1`; Gate B, Gate C, and Gate D are complete. Gate E
  distribution and release-readiness implementation is complete at hosted
  candidate `3ccde86`. Stable release authorization remains open.
- Integration state: final Gate D candidate `7e052ae` is maintainer-confirmed
  green on hosted quality, Linux, macOS, and Windows CI. This hosted/adoption
  record is documentation-only and may be fast-forwarded without a second
  hosted run under the repository's agreed documentation policy.
- Current resume point: fast-forward this hosted evidence record to `main` and
  archive the Gate E branch. Then perform the separate maintainer release
  checkpoint before any version promotion, `[Unreleased]` promotion, tag,
  GitHub Release, registry publication, or stable-release declaration.
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
- All five initial production commands, compatibility discovery, the D1
  consumer-resolvable Rust API, the independent conformance runner, packaged
  standalone adoption, and the canonical Skill are functional locally.
- D014 selected a repository-root `norm-spec` facade after an isolated D1.1
  spike. The facade layout packaged the existing root assets, passed Rust 1.97
  workspace candidate verification with the unpublished core package, and ran
  collect plus validation from an unrelated exact-revision Git consumer.
- The production facade now exposes typed collect and validation requests,
  packaged Schema/template access, completed validation results, and typed
  pre-evaluation failures without process exit codes. The CLI delegates to it
  and retains all 82 frozen observable cases.
- `scripts/check-packages.sh` verifies required facade and runner contents,
  verifies the core → facade → CLI workspace candidate through a fresh isolated
  Cargo registry, and runs collect plus validation from an unrelated
  exact-revision Git consumer. Hosted D1 evidence is maintainer-confirmed green.
- The private GitHub repository and hosted Actions are green through final Gate
  E implementation candidate `3ccde86`. Public visibility remains a maintainer
  checkpoint.
- D015's machine-default compatibility discovery, exact identifier membership,
  locked 82-case A1 bundle, separate `norm-spec-conformance` binary, explicit
  complete/incomplete reports, and no-fallback rules are implemented and green
  locally plus on hosted quality, Linux, macOS, and Windows CI.
- D016 fixes four native release targets, a versioned archive manifest,
  checksum and source binding, extracted self-verification, Rust 1.97 as the
  conservative initial MSRV, temporary candidate CI without publication
  authority, and explicit maintainer release checkpoints.
- Repository-owned scripts build a clean-revision archive and verify its
  checksum, safe paths, exact inventory, manifest/source/contract identity,
  both executables, complete conformance, exact canonical Skill, and unrelated
  standalone adoption from the extracted candidate.
- CI now uses fixed OS labels, has a distinct Rust 1.97.1 MSRV job, and builds
  native `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`,
  `x86_64-apple-darwin`, and `x86_64-pc-windows-msvc` candidates. It uploads
  the preconstructed archives and checksums directly with read-only repository
  permissions; no tag, release, registry, or signing authority was added.

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

Gate D2/D3 local verification on 2026-08-12:

- `cargo fmt --check` and strict workspace Clippy → green.
- `cargo test --workspace --all-features` → 63 Rust test functions passed; the
  original 82-case CLI contract remains green without skips.
- exact `norm-spec/compatibility/v1` compact and pretty fixtures → green and
  cross-checked against the bundle lock.
- `bash scripts/check-contract-bundle.sh` → the 108-file inventory regenerated
  with no diff, exported with its lock, and the arbitrary current candidate
  executed 82 of 82 cases with pass/complete status.
- six conformance black-box paths → green: exact pass; missing candidate;
  tampered bundle; unsafe lock path; compatible behavioral mismatch; and
  missing compatibility with all behavior cases still executed.
- `bash scripts/check-packages.sh` → fresh isolated package candidates verified
  with 13 core, 31 facade, and 21 CLI files; the second binary and its modules
  are present; the exact-revision external consumer remains green.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`, strict
  self-validation of seven `.norm` files, and public-history scan → green.
- structural scan → 37 directories and seven `.norm` files.
- hosted quality, Linux, macOS, and Windows CI for candidate `1326dbe` →
  maintainer-confirmed green.

Gate D4/D5 local verification on 2026-08-12:

- `cargo fmt --check`, strict workspace Clippy, and rustdoc with warnings denied
  → green.
- `cargo test --workspace --all-features` → 68 Rust test functions passed; the
  original 82-case CLI contract remains green without skips.
- `bash scripts/check-contract-bundle.sh` → the unchanged 108-file lock
  exported successfully and the current candidate executed 82 of 82 cases with
  pass/complete status and the frozen digest.
- `bash scripts/check-packages.sh` → fresh isolated package candidates verified
  with 13 core, 31 facade, and 23 CLI files; the unrelated exact-revision Rust
  consumer completed collect plus validation at candidate `2b8f7ff`.
- `bash scripts/check-standalone-adoption.sh` → a candidate installed from
  unpacked `.crate` sources queried version/compatibility, scanned an empty
  project, initialized root and nested conventions, collected
  most-specific-first, validated strictly, then returned the stable
  `norm/reference/not-found` machine failure after its temporary package source
  and install build tree were removed.
- skill-creator `quick_validate.py` → green. The Skill contract requires exactly
  `SKILL.md` plus two self-contained references, executes all six documented
  CLI examples, and proves isolated installation without host metadata.
- Skill forward tests → nested collection, authoring, invalid-reference repair,
  and missing-CLI fail-closed behavior all green without parser, collect,
  validation, empty-ruleset, or successful-skip fallback.
- strict repository self-validation → seven files, zero errors, zero warnings;
  structural scan → 41 directories, seven `.norm` files, no symlinks, and
  `0.171` coverage after removing the initializer's empty host-metadata
  directory.
- public-history scan → green.
- hosted quality, Linux, macOS, and Windows CI for final candidate `7e052ae` →
  maintainer-confirmed green.

First real non-plugin adoption evidence on 2026-08-12:

- OpenCode `1.18.14` ran from the independent pi-norm-spec repository with the
  canonical Skill installed project-locally and the pi plugin left inactive.
- installed `norm 0.1.0-alpha.1` returned
  `norm-spec/compatibility/v1`; required collect and validate APIs were present,
  with exit `0` and empty stderr.
- collecting `docs/planning/status.md` returned `docs/.norm` then `.norm`, and
  the agent applied both layers rather than replacing collection with a manual
  filesystem walk.
- strict validation returned two files, zero errors, and zero warnings; the
  read-only run modified no project files.
- the assisted review found real downstream narrative drift: pi-norm-spec's
  `AGENTS.md` still described pre-self-hosting manual collection. That follow-up
  belongs to pi-norm-spec and does not change canonical format semantics here.

Gate E local implementation verification on 2026-08-12:

- Rust `1.97.1` (declared MSRV `1.97`) executed the complete local candidate;
  `cargo fmt --check`, strict workspace Clippy, and rustdoc with warnings denied
  were green.
- `cargo test --workspace --all-features --locked` → 68 Rust test functions
  passed; all 82 frozen compiled-binary cases and six conformance negative/pass
  paths remained green.
- the 108-file locked contract bundle regenerated without drift and the current
  arbitrary candidate returned pass, complete, 82 executed, zero failed, and
  zero not executed.
- `scripts/check-packages.sh` and the refactored source-free standalone lane
  remained green for the three package candidates and unrelated consumption.
- the clean Apple Silicon candidate built as
  `norm-spec-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz`; extraction verified its
  checksum, single safe root, exact release inventory, precise source revision,
  compatibility and bundle identities, 82-case conformance, and standalone
  scan/init/collect/strict-validation/stable-failure workflow.
- a negative archive check rejected an intentionally wrong expected source
  revision. Strict self-validation returned seven files, zero errors, and zero
  warnings; the public-history scan remained green.
- scope/docs/behavior/package/compatibility review found no intentional format,
  Rust API, machine API, or frozen behavior change. Linux ARM64, musl,
  older-glibc, Windows ARM64, package-manager distribution, signing,
  notarization, registry publication, GitHub Release, and `v0.1.0` remain
  explicitly unclaimed or maintainer-gated.
- hosted quality, MSRV, fixed Linux/macOS/Windows regression, and all four
  native archive jobs are maintainer-confirmed green for exact candidate
  `3ccde86`.

Gate E hosted verification on 2026-08-12:

- the exact pushed branch and tested implementation candidate was `3ccde86`;
- hosted quality, the Rust 1.97.1 MSRV lane, and fixed Linux, macOS, and Windows
  regression jobs passed;
- native `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`,
  `x86_64-apple-darwin`, and `x86_64-pc-windows-msvc` candidate jobs all built,
  extracted, executed, and verified their D016 archives successfully;
- all nine hosted jobs were maintainer-confirmed green for the same candidate;
- this evidence closes Gate E release-readiness implementation. It does not
  authorize version promotion, tagging, GitHub Release, registry publication,
  or the stable `v0.1.0` release.

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
- [x] Add versioned compatibility discovery and arbitrary-candidate
      conformance without skips after D015; local gates are green.
- [x] Prove standalone adoption from an isolated candidate without a plugin.
- [x] Add and validate the canonical framework-neutral norm-spec Skill without
      parser, collect, or validation fallback.
- [x] Confirm the integrated Batch 3 candidate on hosted quality, Linux, macOS,
      and Windows CI, and close Gate D with real non-plugin adoption evidence.
- [x] Define the self-verifying release archive and Rust 1.97 MSRV contract
      (D016), without granting CI publication authority.
- [x] Build and fully verify the native Apple Silicon archive locally.
- [x] Add fixed-runner MSRV and four-target candidate-artifact CI.
- [x] Complete binary/source/Skill installation, upgrade, rollback, and
      uninstall documentation.
- [x] Confirm the integrated Gate E implementation candidate on hosted quality,
      MSRV, Linux x64, macOS ARM64/Intel, and Windows x64 jobs.
- [x] Complete cross-platform distribution and release-readiness implementation
      review (Gate E).
- [ ] Run the maintainer-authorized `v0.1.0` release procedure.
