# Roadmap

## 0.1 — Initial Rust release

Goal: publish a self-contained `.norm` specification, Rust semantic engine, and
portable CLI with explicit format and machine contracts.

### Alpha

- Governance, architecture, and behavior contract.
- A1 parser and versioned parse output.
- Path-scoped collect with deterministic output.
- Draft 7 schema and semantic validation.
- `init` and structural `scan` parity.

### Beta

- Linux, macOS, and Windows release artifacts.
- A1 contract coverage across the frozen fixture set.
- Consumer conformance evidence against a pinned Rust and machine contract.
- Self-hosted validation of this repository.

### Stable

- Release-quality review is green.
- The Rust library and CLI satisfy the documented A1 contract.
- Installation and compatibility documentation is complete.

## Later

The A1 contract remains evolvable until multiple real consumers demonstrate
that the format is stable enough to receive a separate stable-format
identifier.
