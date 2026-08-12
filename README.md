# norm-spec

The `.norm` project-convention format, implemented as a Rust library and a
portable `norm` CLI.

The project starts with one deterministic semantic engine. Its specification,
schemas, templates, fixtures, and machine contracts live in this repository;
framework adapters consume those contracts without becoming format authorities.

> Status: `0.1.0-alpha.1`. Gate C and Gate D are complete. Packaged standalone
> adoption and the canonical framework-neutral Skill are implemented; final
> candidate `7e052ae` is green on hosted quality, Linux, macOS, and Windows CI.
> A real OpenCode-assisted run also used the Skill against pi-norm-spec without
> the pi plugin. Gate E distribution and release readiness are next.

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

This is an alpha development install. The five initial commands, compatibility
discovery, Rust facade, arbitrary-candidate conformance, packaged standalone
adoption lane, and canonical Skill are functional locally. Distribution remains
Gate E work; Gate D closure still requires integrated hosted CI.

See `docs/INTEGRATION.md` for plugin-free project adoption, canonical Skill
installation, failure behavior, and downstream host-adapter boundaries.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.1-execution.md` before contributing.

## License

MIT © 2026 Wade
