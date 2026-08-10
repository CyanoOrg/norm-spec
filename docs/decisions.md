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

## D005 — Start an independent public Rust product line

**Decision.** This repository begins the public norm-spec product at
`0.1.0-alpha.1` with its own Git history. It does not import prototype commits,
branches, tags, or release numbers. The future `CyanoOrg/norm-spec` repository
is the canonical public collaboration and release authority; any Gitea copy is
a mirror or separately named legacy archive. This supersedes D001's cutover
wording, D002's migration-target framing, and any assumption that this product
continues an earlier version line; their single-engine and behavior-first
principles remain in force.

**Context.** A private Python prototype established useful A1 design inputs,
but it was never a public GitHub project. Importing its product history would
make new contributors interpret superseded implementation and release choices
as part of the Rust project's active lineage.

**Rationale.** A clean public history makes ownership and version meaning
unambiguous. The A1 specification and fixtures needed for compatibility are
maintained self-contained in this repository, so preserving product behavior
does not require preserving an earlier implementation's Git ancestry.

## D006 — Intentional deviations from the A1 behavior baseline

**Decision.** The Rust product ports the observable A1 behavior captured from
the private Python prototype (per D002), with the following explicit
deviations. Each is part of the public contract and must not regress silently.

1. **Versioned machine envelopes.** All machine-readable stdout from `parse`,
   `collect`, `validate`, `init`, and `scan` is wrapped in a versioned envelope
   whose first field is `apiVersion`, e.g. `norm-spec/parse/v1`. The prototype
   emitted bare JSON with no version stamp.
2. **Normalized output paths.** Machine output paths are relative to the
   relevant root, not host-absolute. The prototype embedded `Path.resolve()`
   absolute paths, which are not portable as a byte-for-byte contract.
3. **No deprecated `--compat` alias.** Only `--compat-keys` ships. The
   prototype kept a hidden `--compat` alias; it is not imported into the
   independent product line (D005).
4. **Explicit schema discovery.** Schema resolution uses an embedded copy or
   an explicit `--schema-dir`. The prototype's multi-path implicit fallback
   search is not reproduced.

**Exit codes are inherited, not deviated.** The Rust CLI freezes the
prototype's three-tier exit code: `0` success, `1` data or validation failure
(including strict-mode warnings), `2` usage or configuration error.

**Context.** D002 requires intentional deviations from the captured A1
behavior to be recorded before implementation. The prototype is a reference
input per D002, not a runtime dependency or authority; this decision records
where the Rust product deliberately differs.

**Open item — symbolic links.** Symbolic-link handling is not a deviation
because the prototype has no defined behavior to port. The Rust product must
choose follow, reject, or report semantics before collect, validate, and scan
implementation; that decision is recorded separately and does not block the
rest of the Gate B contract.

**Rationale.** Recording deviations up front keeps the behavior contract
reviewable, lets consumers rely on stable versioned envelopes and portable
paths, and prevents the rewrite from inheriting accidental prototype behavior.
