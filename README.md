# norm-spec

The `.norm` project-convention format, implemented as a Rust library and a
portable `norm` CLI.

The project starts with one deterministic semantic engine. Its specification,
schemas, templates, fixtures, and machine contracts live in this repository;
framework adapters consume those contracts without becoming format authorities.

> Status: `0.1.0-rc.1` release-preparation candidate. Gate C, Gate D, and the
> Gate E distribution implementation are complete, including standalone and
> canonical Skill adoption, Rust 1.97 MSRV verification, and four native
> archive targets. The RC still requires exact-candidate hosted verification;
> public visibility, tagging, GitHub Pre-release, crates.io publication, and
> stable `v0.1.0` remain maintainer checkpoints.

## Crates

- `norm-spec`: packaged, filesystem-aware collect and validation facade.
- `norm-spec-core`: deterministic parsing, collection rules, Schema and
  semantic validation, and versioned response models.
- `norm-spec-cli`: arguments, presentation, exit codes, the `norm` command, and
  the independent `norm-spec-conformance` runner; filesystem semantics and
  release-owned assets are delegated to `norm-spec`.

## Rust API

Until registry publication, pin an exact Git revision:

```toml
[dependencies]
norm-spec = { git = "https://github.com/CyanoOrg/norm-spec", rev = "<exact-commit>" }
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

This is an unpublished RC candidate install. The five initial commands,
compatibility discovery, Rust facade, arbitrary-candidate conformance,
packaged standalone adoption lane, canonical Skill, and target-specific
self-verifying archives are functional. CI artifacts remain review evidence
until a maintainer approves public release actions.

See `docs/INTEGRATION.md` for plugin-free project adoption, canonical Skill
installation, failure behavior, and downstream host-adapter boundaries.

See `docs/INSTALLATION.md` for release archives, checksums, source installation,
MSRV, upgrades, rollback, and uninstall.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.1-execution.md` before contributing.

## License

MIT © 2026 Wade
