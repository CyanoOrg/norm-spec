# norm-spec

The `.norm` project-convention format, implemented as a Rust library and a
portable `norm` CLI.

The project starts with one deterministic semantic engine. Its specification,
schemas, templates, fixtures, and machine contracts live in this repository;
framework adapters consume those contracts without becoming format authorities.

> Status: `0.1.0-alpha.1` bootstrap. Governance and contracts are present;
> parser, validator, and production CLI behavior are not yet complete.

## Planned crates

- `norm-core`: parsing, collection, schemas, and semantic validation.
- `norm-cli`: filesystem orchestration and the `norm` command.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.1-execution.md` before contributing.

## License

MIT © 2026 Wade
