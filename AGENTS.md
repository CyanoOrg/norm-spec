# AGENTS.md

Entry point for humans and agents working in norm-spec.

## Purpose

This repository is the canonical Rust implementation of the `.norm` format and
tooling. It owns the specification, schemas, templates, behavior contract, Rust
core library, and `norm` CLI. It must remain independent of any agent framework
or consumer.

The repository is self-contained. New semantics must be declared here and
implemented once in `norm-core`; consumers must not fork the parser or
validator. Earlier private prototypes are not runtime dependencies or public
release history.

## Current state

Version `0.1.0-alpha.1` has completed Gate B and Gate C: governance,
architecture, the language-neutral five-command behavior contract, and all
five production commands are in place. All 82 frozen cases execute the
compiled binary without skips, and the integrated candidate is green on hosted
Linux, macOS, and Windows CI.

Gate D is active. D1 Rust consumption plus D2 compatibility discovery and D3
consumer-neutral conformance are complete; hosted quality, Linux, macOS, and
Windows CI for Batch 2 candidate `1326dbe` is maintainer-confirmed. D4
standalone adoption and the canonical framework-neutral Skill under D013 remain
open. Host-specific injection and enforcement stay downstream.

Read first:

- `docs/planning/status.md` for live state.
- `docs/planning/v0.1-execution.md` for the active plan.
- `docs/planning/gate-d-design.md` for the active integration-readiness design.
- `docs/ARCHITECTURE.md` for code boundaries.
- `docs/decisions.md` for immutable decisions.

## Common commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
cargo package --workspace
cargo run -p norm-spec-cli -- compatibility --pretty
cargo run -p norm-spec-cli -- validate --all --strict
cargo run -p norm-spec-cli -- scan --root . --text
bash scripts/check-contract-bundle.sh
bash scripts/check-packages.sh
bash scripts/check-public-history.sh
```

The Rust CLI now validates this repository without a legacy runtime. Keep that
self-check green together with `docs/SPEC.md` and the contract fixtures.

## Sources of truth

| Concern | Source of truth |
|---|---|
| project workflow and quality gates | `AGENTS.md` |
| `.norm` format | `docs/SPEC.md` |
| architecture and dependency direction | `docs/ARCHITECTURE.md` |
| rationale | `docs/decisions.md` |
| milestones | `ROADMAP.md` |
| shipped changes | `CHANGELOG.md` |
| in-flight state | `docs/planning/status.md` |
| machine-readable rules | `schema/` |
| behavior contract | `tests/contract/` |
| public Rust consumer API | `docs/RUST-API.md` |

## Work loop

plan → decide → implement → test → review → merge → archive → release review → release.

Non-trivial scope or architectural changes require a decision record before
implementation. A green regression suite is a merge gate, not proof that a
release is complete.

## Branching

Use trunk-based development. `main` must stay releasable.

- `feat/*` for features.
- `fix/*` for bug fixes.
- `docs/*` for documentation-only work.
- `refactor/*` for behavior-preserving restructuring.
- `test/*` for test infrastructure or coverage.
- `chore/*` for maintenance and tooling.

Do not create `develop` or long-lived release branches. Bootstrap commits may
land directly on `main`; subsequent non-trivial work uses a short-lived branch
and returns with a fast-forward merge after all gates are green.

## Commits

Use Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`,
`chore:`, `perf:`, `build:`, and `ci:`. Each commit must express one semantic
change and leave the repository internally consistent.

Stage files explicitly; never use `git add -A`. Run `git diff --cached --check`
before committing. Do not mix generated artifacts, unrelated cleanup, or
dependency updates into feature commits.

## Versioning and releases

This public Rust project starts an independent product line at
`0.1.0-alpha.1`. Earlier prototype histories and tags are not part of this
repository. Use independent version identifiers for the CLI/crates, the
`.norm` format, and machine-output protocols. A CLI release must not silently
redefine the format.

- Release tag: `vX.Y.Z` on `main`.
- Pre-release tags: `vX.Y.Z-alpha.N`, then `-beta.N` or `-rc.N` when warranted.
- Commit `Cargo.lock` because this workspace ships binaries.
- Pin the development toolchain in `rust-toolchain.toml`.
- Declare and test an MSRV before the first public alpha.
- Promote `[Unreleased]` in `CHANGELOG.md` only during release preparation.

## Rust code rules

- Forbid unsafe code workspace-wide unless a future decision explicitly creates
  a narrowly audited exception.
- `norm-core` is deterministic and library-first: no process exits, terminal
  output, environment reads, or hidden network access.
- `norm-cli` owns argument parsing, filesystem orchestration, presentation, and
  exit codes.
- Production library code must not use `unwrap` or `expect` for recoverable
  input and I/O failures.
- Machine output must be versioned and deterministically ordered.
- Never downgrade validation or return an empty ruleset after an error.
- Public APIs require rustdoc. Examples must compile.
- Any parser, schema, validator, or output-contract change requires tests in the
  same commit.

## Testing

Use unit tests for local invariants, integration tests for crate boundaries,
and black-box contract tests for CLI behavior. `tests/contract/` is language
neutral and must cover accepted inputs, rejected inputs, exit codes, structured
errors, path behavior, ordering, and platform normalization.

A parity test that cannot run is a failure, not a skip. Cross-platform behavior
must be exercised on Linux, macOS, and Windows before release.

## Documentation

English is primary for repository materials. Keep `README.zh-CN.md` aligned for
the user-facing overview. Decision records are append-only. Planning and status
documents are mutable working records and must not be presented as shipped fact.

When the format changes, update in order: decision → `docs/SPEC.md` → `schema/`
→ templates → contract fixtures → implementation → integration docs → changelog.

## `.norm` awareness

Before operating in a directory, collect `.norm` files from that directory to
the repository root and honor their frontmatter and body. The Rust `collect`
and `validate` commands are the executable self-hosting paths; the repository's
specification and contract fixtures remain authoritative.
