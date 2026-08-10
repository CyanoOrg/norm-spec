# norm-spec

Rust implementation of the `.norm` project-convention format and tooling.

The project is being rebuilt around one deterministic semantic engine and a
portable `norm` binary. The existing format, schemas, templates, and behavior
contract are migrated before new format work begins.

> Status: `0.4.0-alpha.1` bootstrap. Governance and contracts are present;
> parser, validator, and production CLI behavior are not yet complete.

## Planned crates

- `norm-core`: parsing, collection, schemas, and semantic validation.
- `norm-cli`: filesystem orchestration and the `norm` command.

See `ROADMAP.md`, `docs/ARCHITECTURE.md`, and
`docs/planning/v0.4-execution.md` before contributing.

## License

MIT © 2026 Wade
