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
- Consumer-resolvable Rust and machine compatibility discovery.
- Consumer-neutral conformance and standalone adoption evidence.
- One canonical framework-neutral norm-spec Skill.

### Beta

- Linux, macOS, and Windows release artifacts.
- A1 contract coverage across the frozen fixture set.
- Real downstream consumer evidence against a pinned Rust and machine contract.
- Self-hosted validation of this repository.

### Stable

- Release-quality review is green.
- The Rust library and CLI satisfy the documented A1 contract.
- Installation and compatibility documentation is complete.
- RC soak satisfied: two independent downstream adapters consume
  `0.1.0-rc.1` in published releases (dsh-norm-spec 0.1.0 stable is the
  first; pi-norm-spec's first public beta is the second) plus a soak
  window after the second ships; promotion then rests with the
  maintainer.

## Later

The A1 contract remains evolvable until multiple real consumers demonstrate
that the format is stable enough to receive a separate stable-format
identifier.
