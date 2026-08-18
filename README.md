# norm-spec

The `.norm` project-convention format, and the one deterministic Rust
engine that owns its semantics.

## Why

Every project carries rules nobody wrote down completely — naming habits,
module boundaries, commit granularity, "we don't do it that way here."
They live in reviewers' heads, old decisions, and tribal memory.
Newcomers, human or agent, pay for them at every step: the cost of not
knowing is rework, review friction, and quiet divergence.

The common answer — an always-resident instruction file — treats
convention knowledge as memory: loaded once, expected to stay effective.
But an LLM context behaves like a cache, not memory. Instruction
effectiveness decays with distance and competing tokens; long sessions
lose what the first screen said, and re-reading a flat file spends the
same tokens to re-establish the same ambiguity.

norm-spec starts from a different proposition:

> Project conventions should live on disk as scoped, verifiable
> artifacts — a peer of the code, not a paragraph of prose.

A `.norm` file is layered (directories inherit), scoped (each
convention names its target), and validated (schemas, reference
integrity, single sources of truth). Parsing, collection, and validation
semantics live in this repository's one deterministic engine, together
with its specification, schemas, fixtures, and machine contracts. Host
adapters ([pi-norm-spec](https://github.com/CyanoOrg/pi-norm-spec),
[dsh-norm-spec](https://github.com/CyanoOrg/dsh-norm-spec)) consume
those contracts to deliver exactly the collected conventions into agent
sessions at action time and to check edits afterward — without becoming
format authorities. The result is a closed loop rather than prose:
validated at authoring, scoped at collection, observed at delivery.

## Crates

- `norm-spec`: packaged, filesystem-aware collect and validation facade.
- `norm-spec-core`: deterministic parsing, collection rules, Schema and
  semantic validation, and versioned response models.
- `norm-spec-cli`: arguments, presentation, exit codes, the `norm` command, and
  the independent `norm-spec-conformance` runner; filesystem semantics and
  release-owned assets are delegated to `norm-spec`.

## Rust API

Use the exact reviewed release candidate:

```toml
[dependencies]
norm-spec = "=0.1.0-rc.1"
```

```rust
use std::path::Path;
use norm_spec::{CollectRequest, ValidateRequest, collect, validate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(".");
    let inherited = collect(CollectRequest::new(root, Path::new("docs")))?;
    let checked = validate(ValidateRequest::all(root))?;
    assert!(!inherited.norms.is_empty());
    assert_eq!(checked.summary.errors, 0);
    Ok(())
}
```

See `docs/RUST-API.md` for request, failure, compatibility, and packaging
boundaries.

## Install the release candidate

Install both CLI executables from crates.io:

```bash
cargo install norm-spec-cli --version '=0.1.0-rc.1' --locked
norm --version
norm compatibility --pretty
```

Target-specific archives and their sibling checksums are available from the
GitHub Pre-release. Follow `docs/INSTALLATION.md` before copying either binary
into `PATH`.

## Development usage

Run directly from the checkout:

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
cargo run -p norm-spec-cli -- collect --root . --target path/to/directory --pretty
cargo run -p norm-spec-cli -- validate --all --strict
cargo run -p norm-spec-cli -- init --profile module --output path/to/.norm
cargo run -p norm-spec-cli -- scan --root . --text
cargo run -p norm-spec-cli -- compatibility --pretty
```

Or install the current development binary from this checkout:

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
norm collect --root . --target path/to/directory --pretty
norm validate --all --strict
norm init --profile module --output path/to/.norm
norm scan --root . --text
norm compatibility --pretty
```

`cargo install` also installs `norm-spec-conformance`. It verifies an explicit
candidate against an exact exported or release-provided contract bundle:

```bash
norm-spec-conformance \
  --candidate "$(command -v norm)" \
  --contract-dir path/to/exact-contract-bundle \
  --pretty
```

## Status

`0.1.0-rc.1` is published and validated: signed tag `v0.1.0-rc.1`, all
three crates on crates.io, docs.rs pages, four native archives with
checksums, and the canonical Skill path. Stable `v0.1.0` promotion follows
the RC soak criteria recorded in `ROADMAP.md` — two independent downstream
adapters consuming `0.1.0-rc.1` in published releases, plus a soak window
after the second ships.

## Documentation

- `docs/INTEGRATION.md` — plugin-free project adoption, canonical Skill
  installation, failure behavior, and downstream host-adapter boundaries
- `docs/INSTALLATION.md` — release archives, checksums, source installation,
  MSRV, upgrades, rollback, and uninstall
- `docs/ARCHITECTURE.md` — crate boundaries and what owns each behavior
- `docs/RUST-API.md` — Rust facade requests, failures, and compatibility
- `ROADMAP.md`, `docs/planning/v0.1-execution.md` — milestones and execution
  state to read before contributing

## License

MIT © 2026 Wade
