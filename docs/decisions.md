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

## D015 — Freeze compatibility discovery and arbitrary-candidate conformance

**Decision.** The additive `norm compatibility [--pretty]` command is
machine-only and emits `norm-spec/compatibility/v1`. Its required fields expose
the exact product version, accepted format identifiers, the
`norm-spec/rust-api/v1` surface and package version, supported machine API
identifiers, and the `norm-spec/a1-cli/v1` suite identity: bundle API, report
API, case count, and SHA-256 contract digest. Compact JSON is the default and
`--pretty` changes whitespace only. Identifier arrays are emitted lexically;
consumers compare membership rather than order, ignore unknown fields, and
fail closed on absent or unknown required identifiers. Product SemVer never
substitutes for a protocol or suite identifier.

The 82-case A1 CLI suite remains distinct from compatibility discovery. The
new command has its own compatibility-v1 fixtures, while conformance performs
discovery as a preflight before executing the frozen suite. Source revision or
build metadata is omitted until it can be injected reproducibly; an exact Git
revision or artifact digest remains an external transport pin.

The `norm-spec-cli` package also ships a separate
`norm-spec-conformance` binary. It accepts an explicit candidate and exact
contract directory, emits compact `norm-spec/conformance/v1` JSON by default,
and never searches siblings, the network, a registry, or environment fallback.
The exported contract bundle contains the manifest, requirements,
expectations, fixtures, layouts, and canonical Schema resources required by
execution. A versioned lock enumerates portable relative paths and lowercase
SHA-256 file digests; a suite digest binds that ordered inventory, suite ID,
and case count. The lock excludes itself, rejects unsafe paths and unlisted or
altered files, and is generated from canonical repository sources rather than
maintained as a second Schema authority.

Conformance reports distinguish `pass`/complete/exit `0`,
`fail`/complete/exit `1`, and `error`/incomplete/exit `2`. An executable but
incompatible candidate still runs every case. Case mismatches do not stop the
suite. Candidate, bundle, setup, or runner failures expose stable
`norm/conformance/*` issue codes, explicit not-executed counts, and no skip or
empty-success state. Candidate identity comes only from a valid compatibility
response; help text, the runner version, ambient checkout metadata, and raw
host paths are not substitutes.

**Context.** Gate C executes the frozen contract only through
`CARGO_BIN_EXE_norm`, and the explicit Schema case still reads the repository
root. D1 made the Rust facade externally consumable but did not give an
installed binary a self-describing compatibility surface or let an independent
runner verify an arbitrary candidate. A direct extraction of the integration
test would therefore preserve hidden source-tree coupling and would not prove
the bundle presented to a downstream consumer.

**Rationale.** Exact identifiers prevent SemVer guesses from becoming
compatibility policy. A locked, exportable bundle binds claims to the precise
assets executed, while a separate runner can verify installed, packaged, or
downstream-pinned candidates without trusting their own test harness. Explicit
complete and incomplete states preserve all available mismatch evidence and
fail closed when execution cannot be proven.

## D016 — Publish native, self-verifying release archives and start at Rust 1.97

**Decision.** The first release archive set contains four native targets:
`x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`,
`x86_64-apple-darwin`, and `x86_64-pc-windows-msvc`. Each target is built and
tested on a fixed GitHub-hosted runner of the same operating system and
architecture. A moving `*-latest` label, a cross-compiled binary that was not
executed, or an operating-system-only name is not sufficient release evidence.

Every archive has one versioned top-level directory and contains `norm`,
`norm-spec-conformance`, the exact locked A1 contract bundle, the canonical
framework-neutral Skill, `README.md`, `README.zh-CN.md`, `LICENSE`, and a
machine-readable `release-manifest.json`. The manifest uses
`norm-spec/release-artifact/v1` and binds the product version, Rust target,
source revision, executable paths, contract suite/count/digest, and Skill path.
The archive filename includes the product version and exact Rust target. A
lowercase SHA-256 checksum is emitted beside each archive.

Release verification starts from the extracted archive rather than the source
tree. It must check the exact inventory and manifest, execute both binaries,
match the compatibility response to the bundled contract identity, run all 82
conformance cases to a complete passing report, and complete the standalone
adoption smoke with no checkout or ambient asset fallback. An uploaded workflow
artifact is review evidence, not a release; GitHub Release publication, tags,
and registry publication remain explicit maintainer checkpoints.

The initial minimum supported Rust version is `1.97`, declared through Cargo's
`rust-version`. CI verifies the complete workspace and package/adoption gates
with Rust `1.97.1`, the pinned patch toolchain used to develop this release.
This is a conservative supported floor, not a claim that older compilers fail.
Lowering it requires a separately recorded policy change and full verification
of code, dependencies, packages, documentation, and binaries on the proposed
toolchain. Raising it is a compatibility change that must be documented before
release.

The repository initially owns small build and verification scripts rather than
adding a release-framework dependency. Candidate archives are built on pushes
and pull requests for review. A tag-triggered publication workflow is deferred
until the artifact contract has passed hosted CI and the maintainer approves
the release procedure.

**Context.** Gate D proves package candidates, an installed CLI, arbitrary
candidate conformance, and a canonical Skill, but it does not produce a single
download whose identity and contents can be verified after extraction. The
existing cross-platform job uses moving runner labels and tests source
checkouts; it neither distinguishes macOS architectures nor uploads release
candidates. Cargo already declares Rust 1.97 and the repository pins 1.97.1,
but Gate E has not yet named that declaration as the supported minimum or
isolated it as an explicit CI contract.

GitHub workflow artifact transport does not preserve executable permissions
when it creates its own archive, so the repository must upload an already
constructed archive. Product SemVer alone cannot identify architecture,
source, contract content, or assisted workflow content. Introducing automated
publication before those boundaries are exercised would make the first public
release the test of the release process.

**Rationale.** Native execution on exact targets makes portability claims
reviewable, while separate Intel and Apple Silicon archives avoid hiding a
macOS architecture assumption. Shipping the conformance data and Skill beside
the binaries makes one download useful for standalone and assisted adoption
without creating another semantic implementation. A versioned manifest,
contract digest, source revision, and outer checksum bind different layers of
identity instead of overloading the CLI version. Keeping candidate generation
separate from publication permits repeated hosted verification without
granting a workflow release authority. Rust 1.97 is the lowest version the
project currently promises and proves; a lower aspirational number would be a
compatibility claim without evidence.

## D017 — Rehearse the public release boundary with v0.1.0-rc.1

**Decision.** The first public norm-spec release is `v0.1.0-rc.1`, not the
stable `v0.1.0`. The RC contains the Gate E product without new format or
behavior scope and rehearses the irreversible public-distribution chain:
repository visibility, exact release artifacts, the three crates.io packages,
registry installation, release download, and canonical Skill installation.
An RC defect is repaired in `v0.1.0-rc.N+1`; the published RC version is never
overwritten. Stable promotion follows only after the public RC evidence is
complete and requires a separate maintainer authorization.

One release-preparation commit is the identity anchor. The hosted quality,
MSRV, fixed-platform, and four native artifact jobs must all pass for that exact
commit. `main`, annotated tag `v0.1.0-rc.1`, GitHub Pre-release source, every
archive's `sourceRevision`, and the `.cargo_vcs_info.json` revision in all three
crate packages must resolve to the same commit. A merge, squash, rebase, or
documentation edit that changes the commit invalidates earlier artifacts and
requires a new complete hosted run.

The GitHub repository becomes public only after the RC candidate and its public
history, workflow logs, and candidate inventories have passed review. Public
visibility precedes the tag and package uploads so registry metadata and
release documentation resolve to the canonical public source. Main and tag
rules, private vulnerability reporting, community files, Actions history, and
temporary artifacts are reviewed at that checkpoint. Visibility change,
ruleset configuration, tag creation/push, GitHub Release creation, and every
registry upload remain explicit maintainer actions.

The crates.io packages publish serially in dependency order:
`norm-spec-core`, then `norm-spec`, then `norm-spec-cli`. After each upload, the
exact version must be resolvable from the public index and the next package's
`cargo publish --dry-run` must succeed. A later package is never published
while its required upstream version is absent. If any upload fails, preserve
the successful immutable publications, stop the release, report the partial
state, and repair forward with the next RC version if repository changes are
required. Do not yank a correct upstream package merely because a downstream
upload failed.

The annotated tag is created only after the exact `main` candidate is green.
The GitHub Pre-release attaches the four D016 archives and four checksum files
built and verified by that commit. Registry publication follows the GitHub
Pre-release so each crate can point users to an existing public source and
download record. The release is not considered complete until a clean external
environment verifies the GitHub archive/checksum/conformance path, registry CLI
installation, registry Rust-facade consumption, docs.rs results, and Skill use
from the same release.

**Context.** Gate E candidate `3ccde86` and integrated `main` `9e71747` proved
the repository-owned build and verification machinery on all four native
targets, but no public GitHub Release or crates.io dependency chain has yet
been exercised. At the decision checkpoint, crates.io resolved no published
version for `norm-spec-core`, `norm-spec`, or `norm-spec-cli`. A real
`norm-spec-core` publish dry-run passed. Facade and CLI dry-runs correctly
failed because their unpublished upstream packages were absent from the public
index, demonstrating that the first registry release is inherently serial.

Publishing `v0.1.0` directly would make registry propagation, package ownership,
public visibility, Release asset attachment, and post-publication installation
part of the stable release experiment. Crates.io versions are immutable, and
the three names are allocated only by actual successful publication rather
than by local availability checks. GitHub visibility changes also expose prior
Actions history and logs, so they are not a clerical side effect of tagging.

**Rationale.** The product and frozen A1 behavior are ready for public use, but
the transport and registry control plane needs one observable rehearsal. An RC
keeps that evidence honest without weakening the artifact or compatibility
bar. Exact commit binding prevents a green branch, tag, archive, and crate from
quietly naming different code. Serial publication and stop-on-partial-failure
rules respect registry immutability, while explicit maintainer checkpoints
keep public and irreversible actions outside ordinary CI authority.

## D018 — Layer main protections and grant exact promotion to a human Team

**Decision.** `CyanoOrg/norm-spec` uses shared organization Teams for access:
`norm-maintainers` has Maintain, `norm-release-managers` has Maintain, and
`norm-automation` has Write. These Teams may also be attached to independent
repositories, but membership does not combine repository CI, release identity,
tags, registry publication, stable-promotion authority, or roadmap ownership.
Automation identities belong only in `norm-automation`; they must not be
members of `norm-release-managers`.

The monolithic `main-protection` ruleset is replaced by three repository
rulesets with distinct bypass boundaries:

- `main-integrity` blocks deletion and non-fast-forward updates, requires
  linear history, and requires signed commits. It has no bypass.
- `main-quality` requires strict success for exactly `quality`, `msrv`,
  `cross-platform (ubuntu-22.04)`, `cross-platform (macos-15)`,
  `cross-platform (windows-2022)`,
  `release-candidate (ubuntu-22.04, x86_64-unknown-linux-gnu)`,
  `release-candidate (macos-15, aarch64-apple-darwin)`,
  `release-candidate (macos-15-intel, x86_64-apple-darwin)`, and
  `release-candidate (windows-2022, x86_64-pc-windows-msvc)`. It has no
  bypass.
- `main-review` requires a pull request, one approval, dismissal of approvals
  when new commits are pushed, and resolution of all review threads. Only the
  organization Team `norm-release-managers` (GitHub Team ID `18981934`) has an
  always-allowed bypass for this layer.

An exact-candidate promotion still starts with a pull request whose HEAD is the
candidate commit. The required approval must bind to that HEAD, every review
thread must be resolved, all nine strict checks must be green for that exact
commit, and the branch must not be behind `main`. A human release manager may
then fast-forward that exact commit to `main`; squash, merge commits, rebases,
or closure edits that change its SHA invalidate earlier candidate evidence.
Ordinary changes continue through reviewed linear-history pull requests.

Migration is fail-safe. The existing monolithic ruleset remains active while
the three new rulesets are created. Each new rule is read back and its effective
result is verified before the old ruleset is disabled. The old ruleset is
retained disabled for audit and rollback; it is not deleted. The existing
`release-tag-immutable` rule remains active without a bypass.

Direct repository Admin access for `cyano-bot` is temporary migration access.
After the layered rules are active and verified in both `norm-spec` and the
independent `pi-norm-spec` repository, its direct access is reduced to Write.
Organization owners may remain members of the human Teams, but automation does
not acquire human approval or release-promotion authority through ownership.

**Context.** The initial public-release ruleset deliberately combined
integrity, required checks, review, and repository-admin bypass so the exact RC
commit could be fast-forwarded without changing its SHA. That broad bypass also
allowed an administrator or automation identity to bypass signed-commit,
non-fast-forward, deletion, and quality requirements, even though exact
promotion needs an exception only to the pull-request rule. The maintainer has
created the three shared organization Teams and confirmed that the release
manager Team contains human identities while `cyano-bot` remains automation.
The same model has already been exercised on `pi-norm-spec` with an approved
exact-candidate pull request and that repository's own required quality set.

**Rationale.** Separating invariant integrity and quality from review workflow
gives human release managers the narrow capability needed to preserve an exact
candidate SHA without granting a persistent broad administrator bypass. Team
membership makes responsibilities reviewable and reusable while repository-
local rules keep the two products independent. Creating and verifying the new
layers before disabling the old one avoids an unprotected migration window,
and retaining the disabled rule preserves a recoverable audit trail.

## D019 — Family-wide ruleset unification supersedes two D018 execution details

**Decision.** All three family repositories (`norm-spec`, `pi-norm-spec`,
`dsh-norm-spec`) now share one standard ruleset form: `main-integrity`,
`main-quality`, `main-review`, and `release-tag-immutable`, all active
with `bypass_mode: none` on every layer. Two D018 execution details are
superseded:

1. D018 granted `main-review` an always bypass to `norm-release-managers`
   (GitHub Team ID `18981934`). The executed form has no bypass anywhere:
   an approved, green, candidate-head pull request already satisfies the
   pull-request rule for a release manager's fast-forward push (verified
   end-to-end by the dsh-norm-spec `v0.1.0` exact promotion on 2026-08-18),
   so the bypass capability is redundant for the designed loop, and uniform
   no-bypass is simpler to audit across repositories. If a future emergency
   ever requires a no-PR push, re-adding that single-layer bypass is a
   one-rule change recorded here first.
2. D018 retained the old monolithic ruleset disabled for audit and rollback.
   The executed migration deleted it in both `norm-spec` and
   `pi-norm-spec` (pi's disabled leftover was itself removed as cruft on
   2026-08-18). The audit trail lives in this record; rollback is recreating
   a ruleset, a procedure this migration has already exercised.

`cyano-bot` direct repository Admin access is removed from all three
repositories (2026-08-18); its effective access is Write through
`norm-automation`, verified by API readback (effective permission `write`
on each repository) and by real `git push` plus branch deletion on each
repository. Organization ownership stays with the human `cyano-org`
account; automation holds no human approval or release-promotion authority.

**Context.** D018 was designed before the third repository existed and before
any exact promotion had run under layered rules. A family-wide governance
audit on 2026-08-18 found the
three repositories had drifted: `norm-spec` still monolithic,
`pi-norm-spec` carrying a disabled leftover beside its layered sets, and
`dsh-norm-spec` layered but requiring only three of its nine checks. The
audit unified all three to the standard form above, aligned repository
settings (wikis off; head branches never auto-deleted, so future release or
maintenance branches cannot be silently removed), and retired the temporary
`cyano-bot` Admin grants.

**Rationale.** Uniform layered rulesets make the three repositories
operationally identical: the same merge loop, the same release-promotion
path, and one audit answer instead of three. The first stable release
executed under these rules (dsh-norm-spec `v0.1.0`) demonstrated that
SHA-exact promotion does not need the pull-request bypass, so the narrowest
enforceable form wins.

## D020 — Batch multi-target collect ships as collect/v2 after stable promotion

**Decision.** Collection is extended to accept a bounded set of targets in
one operation: a repeatable `--target` on the CLI and a multi-target
machine request under a new protocol identifier `collect/v2`, listed in
the compatibility manifest and discoverable through the handshake.
`collect/v1` and the locked 82-case A1 bundle stay frozen. The work is
post-stable backlog: implementation starts only after stable `v0.1.0`
promotion and rides the minor version line.

The v2 semantics are fixed as follows (full rationale in
`docs/planning/batch-collect-proposal.md`, revised 2026-09-03):

1. Per-scope sections, not a flat merge: one ordered group per target,
   most-specific-first within a section, request order across sections.
2. Shared `.norm` files are deduplicated by reference: the first section
   in request order carries the full entry; later sections carry a
   reference entry (path and content digest; exact shape fixed with the
   v2 contract fixtures) at that file's exact chain position.
3. Targets are normalized before any identity decision (file targets
   begin at their parent directory); repeated targets and file/parent
   equivalents collapse into the earlier section. Ancestor-overlapping
   sets stay distinct scopes and share bytes through references.
4. Validation is whole-request and up front: a malformed target
   (absolute or escaping) or a missing project root fails the entire
   request with one `norm-spec/error/v1` error. There is no
   partial-success shape and no per-target "unavailable" state; a scope
   with no `.norm` files is an empty section and a success.
5. One size budget spans the whole batch with the existing
   fail-not-truncate posture, plus a protocol maximum of 8 targets
   enforced as request validation.
6. Identical requests produce byte-identical responses.
7. Per-target contained-relative validation carries over unchanged.

The locked A1 contract bundle is not mutated: v2 cases ship under a new
contract bundle identity, extending the D015 discovery model. The exact
bundle identity is fixed when the v2 fixtures land. Implementation
follows the repository update order: this decision, then the protocol
identifier and manifest entry, contract fixtures, `norm-core` engine,
facade and CLI, compatibility and integration docs, changelog.

**Context.** The dsh-norm-spec post-0.1.0 runtime review (2026-09-03)
filed `docs/planning/batch-collect-proposal.md`: single-target collect
forces adapter consumers to spawn N processes per step and to fork
cross-chain merge semantics at the projection layer, against the rule
that collection semantics are declared here once. The joint review with
the adapter track pinned every open question, and the maintainer adopted
the recommended positions on 2026-09-03. The same review leaves the
reference adapter's interim projection merge mirroring these semantics
(dsh-norm-spec target-context plan WS2), so its later adoption of v2 is
a mechanical migration plus a deliberate compatibility-pin bump.

**Rationale.** Sections preserve the only orderings that carry meaning —
specificity within a chain and request order across chains — instead of
inventing a cross-scope precedence rule. Dedupe-by-reference keeps the
bytes bounded for prompt-sized budgets while per-scope chain positions
stay exact. Up-front whole-request validation matches the
never-downgrade posture: batch targets come from normalized directories
of real work, so an invalid target is consumer state drift that should
fail loudly, not a per-entry status to paper over. A new version
identifier, rather than an optional field inside v1, keeps one version
on one response shape and leaves the frozen contract untouched.
