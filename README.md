# norm-spec

The `.norm` project-convention format, implemented as a Rust library and a
portable `norm` CLI.

The project starts with one deterministic semantic engine. Its specification,
schemas, templates, fixtures, and machine contracts live in this repository;
framework adapters consume those contracts without becoming format authorities.

> Status: `0.1.0-alpha.1`. Gate C's first executable slice is complete locally:
> global help/version and `norm parse` implement all 17 frozen global/parse
> cases. Collect, validation, init, and scan are not implemented yet.

## Crates

- `norm-core`: parsing, collection, schemas, and semantic validation.
- `norm-cli`: filesystem orchestration and the `norm` command.

## Development usage

Run directly from the checkout:

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
```

Or install the current development binary from this checkout:

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
```

This is an alpha development install. Only `parse` is currently functional;
the other four frozen subcommands fail explicitly with exit `2` until their
Gate C slices land.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.1-execution.md` before contributing.

## License

MIT © 2026 Wade
