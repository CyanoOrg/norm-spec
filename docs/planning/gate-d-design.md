# Gate D Integration Readiness Design

## Goal

Make norm-spec consumable and verifiable outside this repository without a
sibling checkout, copied semantics, hidden assets, or an agent-host plugin.

Gate D closes five product boundaries:

1. a high-level Rust consumer surface;
2. versioned compatibility discovery;
3. a consumer-neutral conformance entry point;
4. standalone adoption from an isolated candidate;
5. one canonical, framework-neutral norm-spec Skill.

D013 defines the standalone and Skill ownership decision. This plan does not
schedule downstream pi, Codex, OpenCode, or other host-adapter work.

## Observed baseline

Gate C is complete on `main` at `71c2a88`:

- all five production commands are implemented;
- all 82 frozen cases execute the compiled binary without skips;
- repository self-validation and isolated `cargo install --path` smoke are
  green;
- hosted Linux, macOS, and Windows CI is maintainer-confirmed green.

Gate D is still open for concrete reasons:

- `norm-spec-core` exposes semantic primitives, but canonical filesystem
  collection and packaged-schema orchestration remain private CLI modules;
- a direct Rust consumer would currently have to duplicate part of collection
  or schema-loading behavior, or start a second CLI process;
- `cargo package --list` for both current packages omits repository-root
  `schema/`, `templates/`, and `tests/contract/` assets referenced by the
  workspace implementation;
- protocol constants exist, but there is no single versioned compatibility
  response that an installed candidate can report;
- the 82-case runner is tied to `CARGO_BIN_EXE_norm` and repository-relative
  fixtures rather than exposed as an arbitrary-candidate conformance command;
- the Rust product has no canonical Skill or Skill validation lane.

Green workspace tests therefore prove the source checkout, not yet a complete
repository-external consumer or distribution boundary.

## Scope boundaries

### In scope

- reuse the canonical Rust semantics through a public high-level API;
- package every release-owned runtime asset required by that API;
- discover exact compatibility without scraping human help output;
- run the frozen CLI contract against an arbitrary candidate binary;
- install a candidate into an isolated root and adopt an unrelated project;
- provide assisted Agent workflows through the canonical Skill;
- preserve the existing five-command behavior and error contracts.

### Out of scope

- pi bridge framing, injection, enforcement, or packaging;
- Codex Hooks, Plugin manifests, session lifecycle, or tool interception;
- any other host-specific adapter or marketplace metadata;
- publishing a public tag, crates.io package, or release artifact;
- claiming that valid `.norm` declarations prove every repository operation
  followed human-language conventions.

Public visibility and publication remain maintainer checkpoints after Gate D
is integrated and hosted CI is green.

## Result D1 — High-level Rust consumer surface

### Required behavior

A Rust consumer must be able to perform canonical collect and validation from
an explicit root and target without:

- depending on `../norm-spec`;
- copying CLI modules, schemas, templates, or fixtures;
- reimplementing canonicalization, symlink, inheritance, or schema selection;
- reading hidden environment state or using network fallback;
- parsing CLI presentation text.

The existing deterministic `norm-spec-core` layer remains free of process
exit, terminal presentation, environment discovery, and network access. The
high-level surface owns explicit filesystem orchestration and release-owned
assets; `norm-spec-cli` delegates to it and retains arguments, human output,
JSON serialization, and exit codes.

### Package-layout checkpoint D1.1

Before moving production code, run a bounded packaging spike and record a new
decision choosing the smallest publishable facade layout. The recommendation
is a public high-level facade package that can include the repository-root
Schema and template sources while depending on `norm-spec-core` for semantics.
The spike must compare at least:

1. a workspace-root facade package with an explicit library path;
2. extending the existing core package while preserving its I/O boundary;
3. a separate runtime crate with a non-duplicated asset ownership model.

Keeping filesystem orchestration only in the CLI and making downstream Rust
consumers start another `norm` process is the control alternative, not the
recommendation. It adds process lifecycle and binary packaging to every Rust
host even though the host already has a Rust bridge.

Select the layout only after all of these pass:

- `cargo package --list` contains every required source and runtime asset;
- `cargo package` verification succeeds without the repository parent tree;
- a temporary external Cargo project builds from the packaged candidate;
- high-level collect and validation run from that external project;
- the existing CLI black-box contract remains unchanged;
- no canonical asset has a second independently edited copy.

The package name and public registry availability are not assumed by this
planning document. They are part of D1.1 and the later publication checkpoint.

## Result D2 — Versioned compatibility discovery

An installed candidate needs one deterministic machine response that reports
what it actually provides. The recommended new envelope is
`norm-spec/compatibility/v1`, containing at least:

- product/crate version;
- accepted format identifiers;
- public Rust consumer-surface identifier or compatible crate range;
- all supported machine API identifiers;
- conformance suite identifier, frozen case count, and contract identity;
- build/source identity only when it can be produced reproducibly.

The response must not use absolute paths, network state, registry availability,
or object-key order as compatibility. A new decision must freeze the envelope,
discovery command, ordering, and compatibility rules before implementation.

The initial recommendation is an additive `norm compatibility --json` command.
Gate B froze the five initial commands, not a permanent five-command ceiling;
the new observable command still requires its own requirements, fixtures, and
cross-platform tests.

## Result D3 — Consumer-neutral conformance

Provide a cross-platform runner that accepts an arbitrary `norm` candidate and
executes the exact frozen contract assets owned by this repository. The
recommended invocation shape is:

```text
norm-spec-conformance \
  --candidate <path-to-norm> \
  --contract-dir <exact-contract-bundle> \
  --json
```

Its report uses a new `norm-spec/conformance/v1` envelope and includes suite
identity, candidate identity, totals, failures, and completion state.

Required properties:

- all applicable cases execute; no skip state exists;
- missing candidate, manifest, fixture, expected output, or runner support is
  failure;
- cases execute in deterministic manifest order;
- candidate exit, stdout, and stderr use the existing contract match rules;
- paths in reports are portable and rooted in the isolated execution area;
- the runner can be invoked from Linux, macOS, and Windows;
- pi or another adapter may consume the exact upstream bundle, but must not
  copy it into a new format authority.

The first runner proves CLI/machine compatibility. It does not pretend to test
host-specific injection or enforcement; those remain downstream end-to-end
gates.

## Result D4 — Standalone adoption evidence

Run a packaged or otherwise repository-independent candidate in a disposable
project with no agent plugin and no sibling source checkout. The lane must:

1. query version and compatibility identity;
2. scan a project that initially has no `.norm`;
3. initialize a root convention from an embedded template;
4. add a nested convention and collect it most-specific-first;
5. strictly validate the project;
6. introduce one stable invalid input and assert code, streams, and exit;
7. remove or make unavailable the source checkout before the final smoke;
8. run on all hosted operating systems before Gate D closes.

This proves a real standalone product path. Repository self-validation remains
valuable but is not a substitute.

## Result D5 — Canonical framework-neutral Skill

Create `skills/norm-spec/` with only essential Skill artifacts:

```text
skills/norm-spec/
├── SKILL.md
└── references/
    ├── authoring.md
    └── field-reference.md
```

Use the skill-creator initializer with the `references` resource, then replace
all placeholders. The canonical Skill is framework-neutral, so host-specific
UI metadata such as `agents/openai.yaml` is not retained here unless a future
decision changes that boundary. A Codex Plugin may generate its own thin UI
metadata downstream.

Skill rules:

- YAML frontmatter contains only a concise `name` and trigger-oriented
  `description`;
- the body stays focused on collect, obey, validate, author, and diagnose;
- detailed authoring and field material lives one reference level deep;
- commands call a compatible `norm` candidate and check failure explicitly;
- no manual YAML parse, directory walk, validation, empty ruleset, or successful
  skip fallback exists;
- missing CLI or incompatible protocol produces installation/repair guidance
  and stops the affected workflow;
- automatic discovery or enforcement is never promised.

Validation includes:

- the skill-creator `quick_validate.py` gate;
- repository checks for required files, allowed references, local absolute
  paths, placeholders, and forbidden fallback language;
- execution of every documented CLI example against the current candidate;
- isolated Skill-directory installation with self-contained references;
- forward tests for nested collection, authoring, invalid-reference repair,
  and missing-CLI failure behavior.

The Skill is versioned by the norm-spec product release that contains it. Gate
D does not create a second Skill repository or independent release process.

## Merge-ready implementation batches

### Batch 1 — Rust consumer surface and packaging

1. `docs(decisions): choose Gate D consumer package boundary`
2. `refactor(core): expose reusable high-level inputs where required`
3. `feat(api): add packaged collect and validation facade`
4. `refactor(cli): delegate collect and validation to the facade`
5. `test(packaging): consume the packaged facade externally`
6. `docs(api): document Rust consumer compatibility`

The decision commit follows the bounded spike and precedes production movement.
Behavior-changing and behavior-preserving refactors remain separate.

### Batch 2 — Compatibility and conformance

1. `docs(decisions): define compatibility and conformance protocols`
2. `feat(core): add compatibility response model`
3. `feat(cli): expose compatibility discovery`
4. `test(contract): freeze compatibility cases`
5. `feat(conformance): run arbitrary candidates`
6. `test(conformance): fail closed across the frozen suite`

### Batch 3 — Standalone adoption and canonical Skill

1. `test(adoption): exercise an isolated standalone project`
2. `feat(skill): add the canonical norm-spec workflow`
3. `test(skill): validate commands, references, and failure behavior`
4. `docs(integration): document standalone and assisted adoption`
5. `docs(status): close local Gate D implementation`

Each batch is independently reviewable and locally green. Hosted CI may run per
batch branch; Gate D closes only after the integrated candidate is green on all
three operating systems.

## Quality gates

Every implementation batch runs the existing gates:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
cargo run -p norm-spec-cli -- validate --all --strict --json
bash scripts/check-public-history.sh
```

Gate D additionally requires:

- package-content and package-verification checks;
- an external temporary Cargo consumer;
- the arbitrary-candidate conformance report with every frozen case executed;
- standalone adoption without a source/sibling dependency;
- Skill structural, command, failure, and forward-test evidence;
- Linux, macOS, and Windows hosted evidence for new integration paths.

## Human checkpoints

1. **D1.1 package layout** — approve the facade package boundary after the
   packaging spike and before moving production orchestration.
2. **D2/D3 protocols** — approve the compatibility command/envelope and
   conformance report before freezing new observable behavior.
3. **First real adoption project** — choose a real non-plugin repository after
   the disposable lane is green; it is additional evidence, not a hidden build
   dependency.
4. **Public visibility and upstream pin** — after integrated Gate D hosted CI,
   decide whether to make norm-spec public and allow downstream exact-SHA/tag
   consumption.
5. **Publication** — crates.io, tags, and release artifacts remain Gate E.

## Gate D exit

Gate D is complete only when all five results are implemented, documented, and
verified; the integrated `main` candidate is green on Linux, macOS, and Windows;
the workspace and package/external-consumer paths agree; no task remains hidden
behind a skip, copied asset, sibling path, private fallback, or host plugin.
