# Development Status

## Resume here

- Stage: `v0.1.0-rc.1` is published and validated. Gate B, Gate C, Gate D, and
  Gate E are complete; release Phases 1–7 are complete (public repo, signed tag,
  GitHub Pre-release, serial crates.io publication, and full public validation).
  Stable `v0.1.0` promotion remains a separate maintainer decision after RC soak.
- Current resume point: `v0.1.0-rc.1` shipped from `5c781964` (tag signed and
  verified). The tag, GitHub Pre-release assets, and all three crates bind that
  release commit. Current `main` is `60d98b6`, with documentation-only commits
  `8a3801d` and `60d98b6` after the tag. The public GitHub repo, four native
  archives and checksum assets, and `norm-spec-core` / `norm-spec` /
  `norm-spec-cli` paths are validated end-to-end (Phase 7 green). The D018
  layered rulesets and the D019 family unification are live and verified
  (2026-08-18); `cyano-bot` is back to Write. Next: optional live
  agent-driven Skill adoption and stable `v0.1.0` promotion after RC soak.
  (pi-norm-spec is a separate downstream track, decoupled per D013.)
- Batch (multi-target) collect is decided (D020, 2026-09-03) and held as
  post-stable backlog: `collect/v2` semantics are fixed in
  `docs/planning/batch-collect-proposal.md`; implementation starts after
  stable promotion and rides the minor version line.
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
- The formerly private repository's Gate E implementation candidate `3ccde86`
  was green on hosted Actions before the later public RC candidate was prepared.
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

Phase 3 local verification on 2026-08-12:

- exact release-preparation commit `2e5b8e9025f88b785cfe816c519c6d40df57a4b0`
  on `codex/chore-release-v0.1.0-rc.1` (`6cf1bfa` plus one `fix(test)`); the
  tracked worktree was clean.
- `cargo fmt --check`, strict workspace Clippy, and rustdoc with warnings denied
  were green.
- `cargo test --workspace --all-features --locked` passed 68 Rust test functions
  with 0 failed and 0 ignored; the 82-case compiled-binary contract ran without
  skips.
- the 108-file locked contract bundle regenerated without drift and the current
  candidate returned pass, complete, 82 executed, 82 passed, 0 failed, and 0 not
  executed.
- `NORM_REQUIRE_CLEAN_PACKAGES=1 bash scripts/check-packages.sh` verified all
  three `.crate` candidates (LICENSE/README/metadata plus `.cargo_vcs_info.json`
  with the exact HEAD SHA and correct `path_in_vcs`) and the exact-revision Git
  consumer; the dry-run sequence passed for `norm-spec-core` and stopped at the
  expected absent public upstream for the facade and CLI.
- `bash scripts/check-standalone-adoption.sh` completed source-free adoption.
- strict self-validation returned seven files, zero errors, and zero warnings;
  the public-history scan remained green.
- the clean Apple Silicon candidate built as
  `norm-spec-0.1.0-rc.1-aarch64-apple-darwin.tar.gz`; extraction verified its
  checksum, single safe root, exact release inventory, source revision
  `2e5b8e9…a4b0`, compatibility and bundle identities, 82-case conformance, and
  standalone adoption.
- `check-packages.sh` fix: the `.cargo_vcs_info.json` `sha1` assertion had a
  trailing comma that only matched dirty trees and the strict gate checked a
  never-emitted `dirty: false`; commit `2e5b8e9` corrected both. No package
  content, source, or release identity changed.
- this evidence closes local Phase 3. Hosted Phase 4 handoff, public visibility,
  tag, GitHub Pre-release, registry publication, and stable promotion remain
  maintainer checkpoints.

Phase 4 hosted candidate handoff on 2026-08-12:

- exact candidate `5c781964b6d9b11c52f29e5b6e2bbe13c25a5ee0` on
  `codex/chore-release-v0.1.0-rc.1` was pushed; maintainer-confirmed the
  quality, MSRV, three fixed-platform regression, and four native artifact jobs
  green for that exact commit.
- all eight uploaded files matched `norm-spec-0.1.0-rc.1-<target>.tar.gz` plus a
  sibling `.sha256`; each archive manifest carried `version=0.1.0-rc.1`,
  `sourceRevision=5c781964…5ee0`, `suite=norm-spec/a1-cli/v1`, `caseCount=82`,
  and `contractDigest=sha256:3d94441e…e2eb`, with a single safe root and no
  symlinks or unsafe paths. The downloaded `aarch64-apple-darwin` archive passed
  the full native check (82/82 conformance and adoption) after stripping macOS
  browser quarantine from the ad-hoc-signed binary.
- the 9-job Actions log tree was scanned for private keys, token prefixes,
  `/Users/<local>` paths, registry credentials, other-repo git URLs, and emails:
  zero matches. All actions are pinned to full SHAs; the run fetched `5c781964`
  from `CyanoOrg/norm-spec`; remaining SHAs were runner-image commits, artifact
  upload digests, and content checksums.
- `main` was fast-forwarded to `5c781964…5ee0` with no squash, rebase, or
  closure commit; release identity unchanged. Maintainer-confirmed the
  exact-`main` CI run green; `main` SHA does not differ from the candidate.
- this evidence closes Phase 4.

Phase 5 public-repository checkpoint on 2026-08-12:

- all retained Actions runs (24, 2026-08-10 through the tag run) had logs and
  artifacts scanned: zero private keys, tokens, maintainer-local paths, registry
  credentials, non-`CyanoOrg/norm-spec` git URLs, or emails; all actions pinned
  to full SHAs.
- `CyanoOrg/norm-spec` was changed from private to public; write access stayed
  admin-only and the audit was clean.
- `main` ruleset (`main-protection`) and `v*` tag ruleset
  (`release-tag-immutable`) configured: `main` blocks force-push/deletion,
  requires linear history, signed commits, the nine CI status checks, and a PR
  (one approval) for non-admins, with repository-admin bypass; `v*` tags block
  update and deletion. (GitHub Free private repos cannot enforce rulesets, so
  visibility preceded ruleset configuration.)
- private vulnerability reporting enabled; public smoke green (anonymous clone
  to `5c781964`; `cargo build` + `cargo test --workspace --all-features` → 68
  passed / 0 failed; `check-public-history.sh` green; compatibility and strict
  self-validation clean).

Phase 6 RC publication on 2026-08-12:

- annotated, signed tag `v0.1.0-rc.1` created at `5c781964` and pushed (GitHub
  verifies the signature `valid`).
- GitHub Pre-release `v0.1.0-rc.1` (`prerelease=true`) with the eight
  archive/checksum files built by the tagged commit's CI run; asset names and
  sizes match the tag run and each manifest carries `sourceRevision 5c781964`.
- crates.io serial publication with a fresh `--dry-run` between uploads:
  `norm-spec-core` → `norm-spec` → `norm-spec-cli`, all owned by `bravetwo`, no
  `--no-verify`, no partial failure.

Phase 7 public validation on 2026-08-12:

- clean-environment `cargo install norm-spec-cli --version '=0.1.0-rc.1' --locked`
  installed both binaries; `norm compatibility` reported the expected suite (82
  cases) and digest.
- a registry-only Rust consumer `norm-spec = "=0.1.0-rc.1"` (isolated
  `CARGO_HOME`, outside the workspace) completed collect and strict validation.
- the public `aarch64-apple-darwin` Release archive passed checksum, manifest
  identity (`5c781964`), and the full native check (82/82 conformance and
  adoption); the other three targets were native-verified by CI on the tag run.
- docs.rs rendered all three crates (`doc_status: true`).
- the canonical Skill from the public release archive drove the released engine
  end-to-end (compatibility, collect with inheritance, strict validate clean).
- optional remaining: a live agent-driven Skill adoption session (functional
  equivalent passed).

`v0.1.0-rc.1` is published and validated end-to-end. Stable `v0.1.0` promotion
remains a separate maintainer decision after RC soak.

RC soak criteria (recorded 2026-08-18): at least two independent
downstream adapters consume `0.1.0-rc.1` in published releases, followed
by a soak window after the second consumer ships. The first consumer is
`dsh-norm-spec` `0.1.0` stable (published 2026-08-18 against the pinned
rc.1 payload); the second is `pi-norm-spec`'s first public beta (its
E3/E4 remaining). Once both are public, promotion becomes a maintainer
decision on timing alone.

Post-RC repository-governance checkpoint on 2026-08-14:

- D018 accepts the same organization Team model already exercised by the
  independent pi-norm-spec repository: `norm-maintainers` (Maintain),
  `norm-release-managers` (Maintain), and `norm-automation` (Write).
- the target `main-integrity` and `main-quality` layers have no bypass;
  `main-review` grants its only always bypass to human Team ID `18981934`.
  `release-tag-immutable` remains unchanged with no bypass.
- exact promotion still requires an approved candidate-head PR, all threads
  resolved, the branch not behind `main`, and all nine exact-head checks green
  before a release manager fast-forwards that unchanged commit.
- live GitHub migration is pending a fresh API readback. The current
  monolithic `main-protection` ruleset therefore remains authoritative and
  must not be disabled until all three replacements are active and effective.
- after both repositories' layered migrations are read back and verified,
  reduce `cyano-bot` direct repository access from Admin to Write on each.

Family governance unification executed on 2026-08-18 (D019):

- All three family repositories now share the standard four-ruleset form
  (`main-integrity`, `main-quality`, `main-review`,
  `release-tag-immutable`), all active, all `bypass_mode: none`.
- `norm-spec` migrated off the monolithic ruleset (create-verify-delete in
  D018's fail-safe order, minus retention — superseded by D019);
  `pi-norm-spec` shed its disabled leftover; `dsh-norm-spec` raised
  `main-quality` from three to nine required checks (strict).
- `cyano-bot` direct Admin removed on all three repositories; effective
  Write via `norm-automation` verified by API readback and real
  `git push` plus branch deletion on each repository.
- Repository settings aligned: wikis off everywhere; head branches are NOT
  auto-deleted (explicit cleanup by decision — future release or maintenance
  branches must not be silently removed).

## Open-source readiness

- [x] Independent public `0.1` history and product identity.
- [x] Security policy, code of conduct, and structured issue/PR templates.
- [x] High-confidence secret, sensitive filename, and private-path history scan.
- [x] GitHub Actions references pinned to full commit SHAs.
- [x] Private GitHub repository creation, initial `main` push, and hosted
      Actions.
- [x] Node 24 Action pins verified green on GitHub without annotations.
- [x] Configure initial public protection — `main-protection` and
      `release-tag-immutable` are active; the main rule requires signed commits,
      linear history, review, and nine CI checks with its original broad
      repository-admin bypass.
- [x] Record the narrower layered Team model in D018.
- [x] Create and read back `main-integrity`, `main-quality`, and `main-review`;
      verify their effective result before disabling `main-protection`.
      (Completed 2026-08-18; see the D019 unification above.)
- [x] After both independent repository migrations are verified, reduce
      `cyano-bot` direct access from Admin to Write. (Completed 2026-08-18
      via D019, extended family-wide.)

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
- [x] Complete and host-verify the exact `v0.1.0-rc.1` preparation candidate.
- [x] Run the maintainer-authorized public visibility and RC publication
      checkpoints in `docs/planning/v0.1.0-rc.1-release.md`.
- [x] Decide the layered Team protection and exact-promotion model (D018).
- [x] Migrate and verify the three D018 `main` rulesets (2026-08-18; the
      old monolithic ruleset was deleted, not retained — superseded by D019).
- [x] Verify the migrations and reduce `cyano-bot` to Write (2026-08-18;
      extended to all three family repositories per D019).
- [ ] Promote stable `v0.1.0` only after the public RC criteria are
      complete; executable checklist drafted in
      `docs/planning/v0.1.0-stable-promotion.md`.
- [ ] Implement batch multi-target collect as `collect/v2` per D020 and
      `docs/planning/batch-collect-proposal.md` (acceptance matrix in
      `docs/planning/collect-v2-contract-cases.md`): protocol identifier
      and manifest entry, a new contract-bundle identity for v2 cases
      (the locked 82-case A1 bundle is not mutated), `norm-core` engine,
      facade and repeatable `--target` CLI, handshake listing, and
      integration docs. Starts after stable promotion.
