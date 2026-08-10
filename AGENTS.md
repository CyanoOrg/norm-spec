# AGENTS.md

Entry point for humans and agents working in the Rust implementation of
norm-spec.

## Purpose

This repository is the future canonical implementation of the `.norm` format
and tooling. It owns the specification, schemas, templates, behavior contract,
Rust core library, and `norm` CLI. It must remain independent of any agent
framework or consumer.

The legacy Python repository is a migration oracle only. New semantics must be
declared here and implemented once in `norm-core`; consumers must not fork the
parser or validator.

## Current state

The repository is in bootstrap. Version `0.4.0-alpha.1` establishes governance,
architecture, and cross-language contracts before porting behavior. No CLI
subcommand is complete until the execution plan and tests say so.

Read first:

- `docs/planning/status.md` for live state.
- `docs/planning/v0.4-execution.md` for the active plan.
- `docs/ARCHITECTURE.md` for code boundaries.
- `docs/decisions.md` for immutable decisions.

## Common commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

Before the Rust validator is complete, validate repository `.norm` files with
the legacy Python CLI recorded in `docs/planning/status.md`.

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
| behavior compatibility | `tests/contract/` |

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

The Rust product continues the existing norm-spec line at `0.4.0-alpha.1`.
Use independent Semantic Versioning for the CLI/crates, the `.norm` format,
and machine-output protocols. A CLI release must not silently redefine the
format.

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
the repository root and honor their frontmatter and body. Until the Rust CLI is
self-hosting, the legacy Python CLI is the compatibility oracle.
