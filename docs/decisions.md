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

1. **Versioned machine envelopes.** All machine-mode stdout from `parse`,
   `collect`, `validate`, `init`, and `scan` is wrapped in a versioned envelope
   containing a top-level `apiVersion`, e.g. `norm-spec/parse/v1`. JSON object
   key order is not part of the contract. The prototype emitted bare JSON with
   no version stamp.
2. **Normalized output paths.** Machine output paths are relative to the
   relevant root, not host-absolute. The prototype embedded `Path.resolve()`
   absolute paths, which are not portable as a byte-for-byte contract.
3. **No deprecated `--compat` alias.** Only `--compat-keys` ships. The
   prototype kept a hidden `--compat` alias; it is not imported into the
   independent product line (D005).
4. **Explicit schema discovery.** Schema resolution uses an embedded copy or
   an explicit `--schema-dir`. The prototype's multi-path implicit fallback
   search is not reproduced.
5. **Explicit machine modes for human-first commands.** `parse`, `collect`, and
   `scan` keep JSON as their default output (`scan --text` selects human
   output). `validate` and `init` keep human output by default and add `--json`;
   `--pretty` formats JSON and requires `--json` for those two commands.
   Handled machine-mode failures emit one `norm-spec/error/v1` envelope on
   stdout and no human diagnostic on stderr.
6. **Reject ambiguous flag combinations.** `validate` requires exactly one of a
   positional path or `--all`; `scan --text` conflicts with `--pretty`; and
   `validate` / `init --pretty` without `--json` is a usage error. The prototype
   silently chose one mode for some of these combinations.
7. **Resolve collect targets from their declared root.** A relative
   `collect --target` is joined to `--root`; other relative CLI paths are
   resolved from the current working directory. The prototype resolved both
   collect arguments independently from the current working directory.

**Exit-code categories are normalized deliberately.** The prototype used the
same numeric values but classified some equivalent failures differently by
command (for example, an outside-root collect target returned `1`, while a
missing validate path returned `2`). The Rust CLI makes the categories uniform:
`0` success, `1` input/data/validation/operation failure (including strict-mode
warnings), and `2` CLI usage, configuration, or root-containment failure. This
normalization is an intentional deviation, not inherited behavior.

**Context.** D002 requires intentional deviations from the captured A1
behavior to be recorded before implementation. The prototype is a reference
input per D002, not a runtime dependency or authority; this decision records
where the Rust product deliberately differs.

**Rationale.** Recording deviations up front keeps the behavior contract
reviewable, lets consumers rely on stable versioned envelopes and portable
paths, and prevents the rewrite from inheriting accidental prototype behavior.

## D007 — Contain symbolic links without silently following them

**Decision.** Filesystem roots and operation targets are canonicalized before
containment checks. A target resolving outside its canonical root fails with
exit `2` and `norm/path/outside-root`. Recursive collect, validate, and scan do
not descend through directory symbolic links. A `.norm` file that is itself a
symbolic link is rejected with exit `1` and `norm/path/symlink-norm`; scan
reports skipped symbolic links in deterministic path order.

Reference targets declared inside `.norm` are also canonicalized. A reference
escaping the project root is a validation failure (exit `1`) with
`norm/reference/outside-root`, rather than a CLI containment error.

**Context.** The private prototype did not define portable symbolic-link
semantics. Following links would permit a scan or convention read to escape the
declared project root, while silently ignoring them would hide relevant
filesystem state from users and consumers.

**Rationale.** Rejecting convention-file links and reporting untraversed links
is deterministic, auditable, and implementable on Linux, macOS, and Windows.
It preserves a strict trust boundary without treating all symlink presence as
fatal.

## D008 — Freeze Gate B as a language-neutral observable contract

**Decision.** The five-command Gate B surface is frozen by
`tests/contract/requirements.tsv`, `tests/contract/manifest.tsv`, its fixtures
and expected results, and the protocol identifiers exported by `norm-core`.
Every requirement must resolve to at least one case, every flag must be
exercised, and missing assets or applicable runners fail rather than skip. A
future observable change requires a new decision plus synchronized contract
assets and tests.

A completed `validate --json` evaluation emits `norm-spec/validate/v1` even
when validation errors or strict warnings produce exit `1`. Only failures that
prevent evaluation (for example, a missing input or schema) use
`norm-spec/error/v1`. JSON success comparisons are exact after object-key
normalization; diagnostic subset comparisons retain ordered arrays and require
stable codes, fields, and machine-actionable suggestions while allowing human
messages to improve.

Human output is frozen by ordered semantic cues rather than byte-for-byte
prototype text. The Rust output uses stable ASCII classifications and error
codes, root-relative paths, and summary counts; it does not make Unicode glyphs
or host-absolute paths part of compatibility. This is an intentional baseline
deviation alongside D006.

**Context.** Initial protocol seeds proved only that six envelope identifiers
existed. They did not prove flag completeness, cross-cutting error behavior,
human/machine stream selection, filesystem layout semantics, or traceability
from an obligation to a fixture. The Python prototype also mixed stable data
with host paths and presentation details that should not constrain an
independent Rust product.

**Rationale.** A language-neutral requirement-to-case inventory lets the Rust
engine and downstream consumers share one reviewable compatibility surface
without copying implementation. Static completeness can be enforced before
the CLI exists; Gate C then replaces presence checks with executable assertions
one vertical command slice at a time.

## D009 — Use maintained, bounded dependencies for the first parse slice

**Decision.** The canonical parser uses Serde data models and
`serde-saphyr` 0.0.29 with only its `deserialize` feature enabled. Parse
responses use `serde_json::Value`, and the CLI uses the stable `clap` 4 derive
surface. Dependency resolution is committed in `Cargo.lock`; no YAML
serializer, general-purpose error framework, or command-test framework is
added for this slice.

The A1 parser accepts UTF-8 with an optional leading byte-order mark, skips
leading blank lines before the opening fence, requires a closing fence, and
trims the Markdown body. With `--legacy-format`, the contiguous leading blank
and `#` comment/title lines are retained as the trimmed response body while the
remaining document is parsed as YAML frontmatter. Without that flag, the same
input fails as `norm/parse/not-a1`. This makes the body already frozen by the
Gate B legacy fixture explicit rather than treating it as parser trivia.

**Context.** The first Gate C vertical slice needs a production YAML parser,
stable JSON envelopes, and a complete global/parse CLI surface. The previously
common `serde-yaml` line is discontinued, while a framework-specific parser or
hand-written YAML subset would either add maintenance risk or silently narrow
the A1 format. The Gate B fixture for the pre-A1 format preserves its leading
Markdown title in the parse response.

**Rationale.** `serde-saphyr` provides direct Serde deserialization, malformed
input handling without parser panics, no library `unsafe` code, and bounded
parsing defaults while allowing the unused serializer graph to be disabled.
`clap` supplies conventional cross-platform help, version, and subcommand
behavior. Keeping the dependency set small and recording the legacy split
before implementation preserves D002's behavior-first rule and D008's frozen
contract.

## D010 — Compile Draft 7 schemas from explicit offline resources

**Decision.** The first validator slice uses `jsonschema` 0.49.9 with default
features disabled and the explicit `jsonschema::draft7` builder. Format checks
are enabled. Schema compilation receives only caller-supplied in-memory
resources keyed by absolute schema identifier; unresolved `$ref` values fail
compilation. The dependency's HTTP, file, asynchronous retrieval, macro, and
TLS features are not enabled.

`norm-core` owns Draft 7 evaluation and converts structured keyword, instance
path, schema path, and unexpected-property data into norm-spec diagnostics. A
dependency-provided display message is never a stable machine-contract field.
`norm-cli` may read the packaged schema bundle or an explicit `--schema-dir`,
but the core library does not read files, inspect environment variables, or
access the network. Enabling a retrieval feature or allowing an implicit
fallback requires a new decision.

**Context.** Gate C validation must evaluate the existing Draft 7 root and
profile schemas, including profile-relative `../norm-schema.json` references,
without weakening D006's explicit schema discovery or the architecture's
deterministic core boundary. The dependency's defaults include HTTP, file, and
TLS retrieval, so accepting its default feature set would create hidden I/O.
`boon` was also considered; it supports Draft 7 and structured output, but its
loader and compatibility surface do not isolate the forbidden retrieval paths
as explicitly for this use case.

**Rationale.** `jsonschema` exposes a draft-specific builder, structured error
data, custom in-memory retrieval, an MSRV below this workspace's Rust 1.97, and
an MIT license. Disabling defaults and proving relative-reference success plus
unregistered-reference failure keeps dependency behavior behind a small,
auditable boundary while preserving the frozen `validate/v1` contract.

## D011 — Stabilize generic Draft 7 failures behind one diagnostic code

**Decision.** `norm/schema/unknown-key` and `norm/schema/version-format` retain
their specialized frozen diagnostics and suggestions. Every other Draft 7 or
profile-required-field failure maps to `norm/schema/invalid`, with a normalized
frontmatter field path when available. The underlying dependency keyword may
appear in the human message, but dependency display text and per-keyword error
names do not become protocol codes.

**Context.** Gate B froze specialized codes for its observable compatibility
cases but did not assign a machine code to other valid Draft 7 failures such as
missing required properties, incorrect types, enums, or formats. The Gate C
implementation cannot silently omit these failures, and exposing a new code
for every dependency keyword would couple `validate/v1` to a third-party enum.

**Rationale.** One generic code completes schema coverage without weakening
validation or expanding the protocol on every dependency release. Stable field
paths preserve machine actionability; specialized cases remain available where
the contract requires a correction hint.

## D012 — Embed init templates and keep structural scan deterministic

**Decision.** `norm init` embeds the seven repository-owned profile templates
at build time and selects them from an explicit name-to-content table. It does
not search the current directory, a sibling checkout, or an environment path,
and an unknown profile fails instead of falling back to an empty or generic
template. Init creates missing parent directories, reports paths relative to
the current working directory when contained, refuses an existing output
without `--force`, and applies D007's `.norm` symbolic-link rejection.

`norm-core` owns scan classification and aggregation over explicit filesystem
observations; `norm-cli` owns the no-follow traversal. Scan ignores only the
declared infrastructure directory names `.git`, `__pycache__`, `.venv`,
`venv`, `node_modules`, `.mypy_cache`, `.pytest_cache`, `.idea`, `.vscode`,
`dist`, `build`, and `target`. Hidden files do not contribute to file counts or
naming, while a regular `.norm` contributes to coverage. Non-root directory
names and visible filename stems are classified, in precedence order, as
`date_prefix`, `kebab-case`, `snake_case`, `UPPER`, `PascalCase`, `camelCase`,
`lowercase`, or `mixed`. An untraversed directory symlink contributes its name
to the directory classification and is reported separately.

Directories and symlinks are ordered by portable relative path. Recurring
filenames appear in at least two directories, sort by descending directory
count and then ascending filename, and are capped at twenty. Coverage ratios
are rounded to three decimal places. The scan reports observations only; it
does not infer sources of truth, lifecycle, or project intent.

**Context.** Gate B froze init and scan protocols plus representative outputs,
but the final Gate C slice still needs package-independent template discovery
and general rules behind the naming, infrastructure, recurrence, and coverage
examples. The private Python prototype is useful reference evidence under
D002, not a runtime dependency or an authority for unspecified behavior.

**Rationale.** Compile-time template selection makes installed binaries
self-contained and fail-closed. Keeping traversal in the CLI and aggregation
in the deterministic core preserves the architecture boundary, while explicit
classification, ordering, and rounding rules make scan results portable across
Linux, macOS, and Windows without turning structural observation into semantic
guessing.

## D013 — Keep norm-spec standalone-first and ship one canonical Skill

**Decision.** norm-spec is a standalone format, Rust engine, CLI, and
conformance product. Its usefulness and release readiness must be demonstrable
without installing pi-norm-spec or any other agent-host plugin. Gate D therefore
includes a repository-external adoption lane that installs a candidate in an
isolated environment and exercises scan, init, collect, validation, stable
failure output, and compatibility discovery without a host adapter.

This repository will also own one framework-neutral `norm-spec` Skill as a
Gate D result. The Skill is the canonical cognitive workflow for collecting,
interpreting, validating, authoring, and diagnosing `.norm` conventions. It is
released with this product rather than as a separate repository or independent
release line. Its instructions must invoke a compatible versioned CLI or Rust
contract. A missing or failed engine is explicit failure: the Skill must not
reimplement YAML parsing, directory inheritance, schema validation, or return
an empty ruleset or successful skip.

Host lifecycle integration remains downstream. Session or turn events, tool
target extraction, automatic context injection, enforcement, feedback,
permissions, user interface, and marketplace packaging belong in separately
versioned adapters after a host-specific capability and spike checkpoint. This
repository may document generic integration contracts, but it does not schedule
or contain pi, Codex, OpenCode, or other host implementation work.

The Skill is an assisted-use surface, not an enforcement boundary. Project
instructions may improve discovery, and a host may select the Skill explicitly
or implicitly, but neither mechanism proves mandatory path-aware collection or
unbypassable tool policy. Those claims require evidence from the relevant host
adapter.

**Context.** Gate C made all five CLI commands executable and completed the 82
frozen cases, but self-validation and repository-internal tests do not prove
that an installed candidate works in an unrelated project. The private Python
prototype included a useful `collect → obey → validate → author` Skill, but it
also allowed manual parse/collect fallback and described the Skill as an
enforcement layer. Carrying those fallbacks into the Rust product would create
a second semantic implementation and overstate what a discoverable instruction
bundle can guarantee.

The first downstream runtime adapter is still incomplete, and additional host
adapters have different event, tool, trust, and distribution surfaces. Making
plugins the only meaningful use path would turn the independent norm-spec
product into an internal SDK and couple its roadmap to host churn.

**Rationale.** A deterministic CLI and conformance boundary give humans,
automation, CI, and custom consumers value before any plugin exists. Keeping
the canonical Skill beside the specification and commands makes its workflow
reviewable against the same release while avoiding a second repository for one
small artifact. Keeping automatic host behavior downstream preserves the
single-engine rule, independent versioning, and honest evidence boundaries.

## D014 — Package the high-level Rust facade at the workspace root

**Decision.** The public high-level Rust package is `norm-spec`, rooted at the
repository workspace with an explicit library path under
`crates/norm-api/src/`. It owns explicit filesystem orchestration and embeds
the canonical root `schema/` and `templates/` assets from their existing
single-source locations. It depends on `norm-spec-core`, which remains the
deterministic, I/O-free semantic package. `norm-spec-cli` will depend on the
facade for collect and validation orchestration while retaining argument
parsing, presentation, JSON serialization, and exit-code mapping.

The facade exposes typed requests, versioned response models, and a typed
failure classification that distinguishes invalid usage from an unavailable
operation without exposing process exit codes. All roots, targets, optional
schema directories, and behavior flags are supplied explicitly; the facade
does not inspect environment state or perform network or sibling discovery.

Package verification is performed as a workspace candidate so Cargo can stage
the unpublished `norm-spec-core` dependency in its temporary registry before
verifying `norm-spec`. Exact-revision Git consumption is a separate Gate D
check. Registry publication order remains a Gate E concern: core precedes the
facade and CLI. A registry name lookup found no current `norm-spec` or
`norm-spec-core` record, but that observation is not a reservation or release
claim.

**Context.** A bounded D1.1 spike compared three layouts. A workspace-root
package included the existing Schema and template trees, passed Rust 1.97
workspace package verification with its core dependency, and built an
unrelated exact-Git-revision consumer that collected and validated two
inherited conventions. Cargo excluded `../../schema/**` and
`../../templates/**` from both an extended core package and a new nested
runtime package because package contents cannot escape their package root.

Moving the canonical assets under a nested runtime crate would make that
layout packageable, but it would relocate repository-level sources of truth
and broaden the change without improving the public boundary. Keeping the
filesystem adapter private to the CLI would force Rust consumers to manage a
second process and binary distribution. Copying or generating a second asset
tree would create split authority.

**Rationale.** The workspace root is the only compared package root that
naturally contains the existing release assets and can provide one public
facade without copies or parent-tree access. The explicit dependency direction
keeps semantics single-sourced in core, gives CLI and library consumers the
same orchestration, and preserves independent tests for Git consumption,
package contents, and eventual registry publication.
