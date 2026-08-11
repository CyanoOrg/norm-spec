# norm-spec

The `.norm` project-convention format, implemented as a Rust library and a
portable `norm` CLI.

The project starts with one deterministic semantic engine. Its specification,
schemas, templates, fixtures, and machine contracts live in this repository;
framework adapters consume those contracts without becoming format authorities.

> Status: `0.1.0-alpha.1`. The full Gate C implementation is complete locally:
> global help/version and all five commands execute all 82 frozen cases without
> skips. Hosted cross-platform CI for the final init/scan slice is pending.

## Crates

- `norm-core`: parsing, collection, schemas, and semantic validation.
- `norm-cli`: filesystem orchestration and the `norm` command.

## Development usage

Run directly from the checkout:

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
cargo run -p norm-spec-cli -- collect --root . --target path/to/directory --pretty
cargo run -p norm-spec-cli -- validate --all --strict
cargo run -p norm-spec-cli -- init --profile module --output path/to/.norm
cargo run -p norm-spec-cli -- scan --root . --text
```

Or install the current development binary from this checkout:

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
norm collect --root . --target path/to/directory --pretty
norm validate --all --strict
norm init --profile module --output path/to/.norm
norm scan --root . --text
```

This is an alpha development install. All five frozen subcommands are
functional; distribution, consumer-conformance, and release-readiness work
remain in later gates.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.1-execution.md` before contributing.

## License

MIT © 2026 Wade
