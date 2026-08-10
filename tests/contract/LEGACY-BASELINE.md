# Legacy baseline

The initial migration baseline is the released Python implementation at:

- repository: legacy `norm-spec`;
- tag: `v0.3.1`;
- commit: `7e32db4`;
- released format: A1, stable since `v0.2.0`.

The bootstrap imports `docs/SPEC.md`, `docs/PROFILE-GUIDE.md`, `schema/`, and
`templates/profiles/` from content unchanged since that tag. Gate B captures
the full black-box CLI behavior before implementation is ported.

Legacy commands used as an oracle must run successfully. Missing Python,
dependencies, fixtures, or binaries fail the compatibility test.
