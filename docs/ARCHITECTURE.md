# Architecture

## Goals

norm-spec has one implementation of format semantics, usable by the CLI and
framework consumers without Python or Node.js runtime dependencies.

The architecture prioritizes:

1. deterministic cross-platform behavior;
2. explicit, versioned machine contracts;
3. consumer independence;
4. complete errors instead of silent fallback;
5. a small auditable dependency graph.

## Crate boundaries

### `norm-spec`

The repository-root package is the public, high-level Rust facade. It owns
explicit filesystem collection and validation orchestration plus the packaged
Schema and template assets. It returns typed request failures without process
exit codes and reuses `norm-spec-core` for every semantic decision.

Relative collect and validation targets resolve from the caller-supplied root.
The facade performs no environment, network, sibling-checkout, or framework
discovery.

### `norm-spec-core` (`crates/norm-core`)

Owns parsing, typed format data, schema validation, semantic validation,
collection rules, normalized paths, and versioned response models.

It does not own terminal output, process exits, environment discovery, update
checks, or framework-specific enforcement. File reads enter through explicit
interfaces so behavior can be tested without ambient process state.

### `norm-cli`

Owns CLI arguments, human-readable reporting, JSON serialization, and stable
exit codes. It delegates filesystem collect and validation plus release-owned
Schema/template access to `norm-spec`; both packages may depend on
`norm-spec-core`, and reverse dependencies are forbidden.

The facade embeds the release schema bundle and may load an explicitly selected
bundle from disk. It passes parsed JSON resources into `norm-core`; the core
compiles Draft 7 with format checks and an in-memory-only reference retriever.
Missing references fail compilation rather than triggering filesystem or
network fallback.

The facade also embeds the seven init templates. The CLI performs explicit
filesystem traversal for scan. Template selection never discovers ambient
files. Scan passes portable directory and symlink observations into
`norm-spec-core`, which owns naming classification, ordering, recurrence, and
coverage aggregation.

## Dependency direction

```text
norm-cli ──> norm-spec ──> norm-spec-core
    └──────────────────────> norm-spec-core
consumers ──> norm-spec or versioned norm-cli contracts
norm-spec-core ──X filesystems, process policy, and consumer frameworks
```

## Contract layers

The project versions three layers independently:

- product/crate version: Rust packages and CLI release;
- format compatibility: accepted `.norm` syntax and fields;
- machine API version: JSON output envelopes and error codes.

Arrays whose order carries meaning stay ordered. Object-key order is not a
semantic contract. Contract tests compare canonical data and stable fields,
not host-specific absolute paths or incidental error wording.

## Compatibility boundary

The A1 specification and fixtures in this repository define the initial
compatibility baseline. Contract coverage is expanded before parser and
validator behavior lands. External implementations may provide research input,
but they are not runtime dependencies or authorities for this project.
