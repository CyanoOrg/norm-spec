# Changelog

All notable changes are documented here. The project follows Semantic
Versioning and keeps changes under `[Unreleased]` until release preparation.

## [Unreleased]

### Added

- Rust workspace governance and standalone architecture.
- Frozen Gate B behavior contract: 91 language-neutral requirements, 82 cases,
  cross-cutting fixtures/layouts, and complete command/flag/protocol coverage.
- Independent public `0.1` product line and canonical GitHub repository policy.
- Security and community contribution policies and issue templates.
- Reproducible public-history checks and commit-pinned CI actions.
- Versioned five-command and error protocols with a static integrity gate that
  rejects missing coverage, invalid protocol/stream/exit combinations, and
  missing or orphan assets.
- Maintained deserialize-only YAML parsing, A1 and explicit legacy-format
  support, and versioned parse/error response models in `norm-core`.
- Executable global help/version and `norm parse`, with all 17 frozen
  global/parse cases asserted against the compiled binary in isolated roots.
- Deterministic, root-contained `norm collect` with file targets, explicit
  legacy parsing, canonical outside-root rejection, `.norm` symlink rejection,
  most-specific-first inheritance, and all 11 frozen collect cases executable.
- Offline Draft 7, profile, lifecycle, single-source, and reference validation;
  versioned human/machine `norm validate` output; embedded or explicitly
  selected schema bundles; and all 35 frozen validate cases executable.
- Self-contained `norm init` with seven embedded, strictly validated profile
  templates; deterministic `norm scan` structure, naming, recurrence, symlink,
  and coverage reports; and all 19 frozen init/scan cases executable.
- Repository-root `norm-spec` Rust facade with packaged Schema/template assets,
  typed collect and validation requests and failures, CLI delegation without a
  second orchestration copy, workspace package verification, and an
  exact-revision external Cargo consumer gate.
