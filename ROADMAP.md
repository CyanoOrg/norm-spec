# Roadmap

## 0.4 — Rust replacement

Goal: replace the legacy Python runtime with one portable Rust semantic engine
without silently changing the `.norm` format.

### Alpha

- Governance, architecture, and behavior contract.
- A1 parser and versioned parse output.
- Path-scoped collect with deterministic output.
- Draft 7 schema and semantic validation.
- `init` and structural `scan` parity.

### Beta

- Linux, macOS, and Windows release artifacts.
- Legacy Python parity across the frozen fixture set.
- `pi-norm-spec` integration against the Rust engine.
- Self-hosted validation of this repository.

### Stable cutover

- Release-quality review is green.
- Rust becomes canonical; Python becomes read-only migration history.
- Installation and migration documentation is complete.

## Later

Format evolution remains at 0.x until multiple real consumers demonstrate that
the format is stable enough to freeze.
