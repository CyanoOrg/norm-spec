# Decision Records

Decision records are append-only. Corrections or reversals are new decisions
that reference the earlier record.

## D001 — Rust becomes the single canonical semantic engine

**Decision.** Rebuild norm-spec as a Rust library and CLI. After the cutover,
parsing, collection, schema validation, and semantic validation have one
canonical implementation in `norm-core`.

**Context.** The legacy Python CLI and the first TypeScript consumer already
duplicate parser and collection behavior. Permanent multi-language references
would make compatibility harder to reason about.

**Rationale.** One portable binary removes runtime-install friction, while one
library prevents semantic drift. Language-neutral contracts still allow any
consumer to integrate without becoming authoritative.

## D002 — Preserve observable behavior before improving the format

**Decision.** The first Rust milestone ports the released A1 format and current
CLI behavior before introducing format changes. Intentional deviations require
a new decision, migration note, and contract update.

**Context.** Changing both implementation and specification at once would make
regressions indistinguishable from product changes.

**Rationale.** A captured compatibility baseline makes the rewrite reviewable
and gives downstream consumers a stable migration target.

## D003 — Product, format, and machine protocols version independently

**Decision.** Crate/CLI SemVer, `.norm` format compatibility, and structured
output protocol versions are separate identifiers. Structured endpoints use
explicit names such as `norm-spec/collect/v1`.

**Context.** The legacy product version, `metadata.version`, and consumer
compatibility range are easy to conflate, while absolute-path JSON output is
not portable as a byte-for-byte contract.

**Rationale.** Independent versions let implementation releases, additive
format evolution, and protocol compatibility move at the correct cadence.

## D004 — Framework adapters stay downstream and thin

**Decision.** Framework-specific repositories may translate events and present
results, but they consume `norm-core` or a versioned CLI/bridge contract. They
must not reimplement YAML parsing, collection, or validation.

**Context.** pi loads TypeScript/JavaScript extensions, so its adapter cannot be
pure Rust even though its semantic engine can be.

**Rationale.** The boundary keeps norm-spec framework-independent and prevents
consumer convenience code from becoming a second format authority.
