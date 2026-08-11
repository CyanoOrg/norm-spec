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

## Observed baseline entering Gate D

Gate C is complete on `main` at `71c2a88`:

- all five production commands are implemented;
- all 82 frozen cases execute the compiled binary without skips;
- repository self-validation and isolated `cargo install --path` smoke are
  green;
- hosted Linux, macOS, and Windows CI is maintainer-confirmed green.

Gate D entered implementation with these concrete gaps:

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

Green workspace tests therefore proved the source checkout, not yet a complete
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

### D1.1 result — 2026-08-12

D014 selects the workspace-root facade with library source under
`crates/norm-api/src/`. The isolated spike established:

- a root package includes the existing `schema/` and `templates/` sources
  without copies;
- extending `norm-spec-core` or adding a nested runtime crate cannot include
  `../../schema/**` or `../../templates/**` in its package;
- an unrelated Cargo project consumed the root facade at an exact local Git
  revision and completed a two-level collect plus validation run;
- Rust 1.97 `cargo package --workspace` staged the unpublished core package in
  a temporary registry and verified both core and facade outside their source
  directories;
- the currently unregistered package names are observations only, not a
  reservation or publication claim.

Production work must retain exact, root-anchored package include patterns so a
root `README.md` pattern does not accidentally include nested files with the
same basename. CLI packaging joins the workspace-package gate after its
private Schema orchestration has moved behind the facade.

### D1 implementation result — 2026-08-12

The production result now matches D014:

- the repository-root `norm-spec` package exposes typed collect and validation
  requests, packaged Schema/template access, versioned response models, and
  typed pre-evaluation failures without process exit codes;
- `norm-spec-cli` delegates collect, validation, Schema, and template behavior
  to the facade while retaining presentation, argument, and exit policy;
- all 82 frozen CLI cases remain executable without skips;
- the permanent package gate verifies core, facade, and CLI together via
  Cargo's temporary workspace registry;
- an unrelated Cargo project consumes the exact current Git revision and runs
  inherited collection plus packaged validation without a sibling path;
- `docs/RUST-API.md` declares the consumer boundary and does not claim D2/D3
  compatibility or conformance work.

D1 is complete. Hosted Linux, macOS, and Windows CI for candidate `07a95c6` is
maintainer-confirmed green. This later verification record is documentation
only; D2/D3 protocol design remains the next human checkpoint.

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

### D2 protocol proposal — awaiting maintainer approval

The proposed discovery command is additive and machine-only:

```text
norm compatibility [--pretty]
```

Compact JSON is the default. `--pretty` changes whitespace only. The proposal
does not add a redundant `--json` selector or a second human presentation that
consumers could accidentally scrape. Gate B froze the five initial commands,
not a permanent five-command ceiling; this command receives separate
compatibility-v1 fixtures and cross-platform tests.

The proposed compact response is structurally equivalent to:

```json
{
  "apiVersion": "norm-spec/compatibility/v1",
  "product": {
    "name": "norm-spec",
    "version": "0.1.0-alpha.1"
  },
  "formats": [
    "norm-spec/a1"
  ],
  "rustApi": {
    "id": "norm-spec/rust-api/v1",
    "package": "norm-spec",
    "version": "0.1.0-alpha.1"
  },
  "machineApis": [
    "norm-spec/collect/v1",
    "norm-spec/compatibility/v1",
    "norm-spec/error/v1",
    "norm-spec/init/v1",
    "norm-spec/parse/v1",
    "norm-spec/scan/v1",
    "norm-spec/validate/v1"
  ],
  "conformance": {
    "bundleApi": "norm-spec/contract-bundle/v1",
    "reportApi": "norm-spec/conformance/v1",
    "suite": "norm-spec/a1-cli/v1",
    "caseCount": 82,
    "contractDigest": "sha256:<64-lowercase-hex>"
  }
}
```

The current product version is illustrative; the compiled candidate supplies
its own exact package version. The initial envelope deliberately omits source
commit and build metadata. A Git revision or artifact digest remains an
external transport pin until the build can inject and reproduce source
identity rather than infer it from a checkout.

The proposed compatibility rules are:

- the envelope, format, Rust API, machine APIs, bundle API, report API, and
  suite are
  compared by exact identifier, not product-version inference;
- `caseCount` and `contractDigest` must both match the requested frozen suite;
- product and Rust package versions are exact identity and diagnostic data,
  not substitutes for the identifiers;
- producers emit identifier arrays in lexical order, while consumers compare
  them as sets and do not treat array or object-key order as compatibility;
- consumers ignore unknown object fields but fail closed when a required field
  or required identifier is absent, malformed, or unknown;
- additive machine API identifiers may appear in a compatibility-v1 response;
  changing the meaning of an existing identifier requires that identifier to
  advance;
- no path, environment, network, registry, or ambient repository state enters
  the response.

The compatibility command is not recursively included in the 82-case A1 CLI
suite that it reports. Its own compatibility-v1 fixtures prove the command and
envelope; the D3 runner performs that check as a preflight and then executes
the already frozen 82 cases.

## Result D3 — Consumer-neutral conformance

Provide a cross-platform runner that accepts an arbitrary `norm` candidate and
executes the exact frozen contract assets owned by this repository. The
proposed invocation shape is:

```text
norm-spec-conformance \
  --candidate <path-to-norm> \
  --contract-dir <exact-contract-bundle> \
  [--pretty]
```

Compact JSON is the default and `--pretty` changes whitespace only. The runner
is proposed as a second binary in the `norm-spec-cli` package, not a `norm`
subcommand or part of the public Rust facade. That keeps candidate behavior
separate from the independent process, fixture, and comparison machinery that
verifies it.

### Exact contract bundle

The current executable test is not repository-independent: it resolves the
candidate through `CARGO_BIN_EXE_norm` and the explicit Schema case through the
repository-root `schema/` tree. D3 therefore needs a deterministic exported
bundle rather than a renamed copy of the integration test.

The proposed bundle contains:

```text
bundle.lock.json
requirements.tsv
manifest.tsv
expected/
fixtures/
layouts/
schema/
```

`schema/` is copied into a disposable export from its canonical repository
source; it is not checked in as another editable Schema tree. The lock lists
every execution-owned file by portable relative path and lowercase SHA-256.
Paths are lexical, use `/`, and may not be absolute or contain `..`.

The suite digest is SHA-256 over this UTF-8 identity stream:

```text
norm-spec/contract-digest/v1
suite=norm-spec/a1-cli/v1
cases=82
<path> NUL <lowercase-file-sha256>
...
```

The `NUL` separator is one zero byte; every logical row ends with one LF byte.
File rows are in lexical path order. The lock file itself is excluded to avoid
recursion. Repository attributes pin execution-owned text to LF before the
digest is frozen. The runner embeds the expected suite ID, count, and digest,
then rejects a missing, altered, extra, or path-unsafe bundle entry. Exporting
from source and verifying an exported directory become permanent gates; no
sibling lookup or network repair exists.

### D3 report proposal

The proposed success report is structurally equivalent to:

```json
{
  "apiVersion": "norm-spec/conformance/v1",
  "suite": {
    "id": "norm-spec/a1-cli/v1",
    "caseCount": 82,
    "contractDigest": "sha256:<64-lowercase-hex>"
  },
  "candidate": {
    "name": "norm-spec",
    "version": "0.1.0-alpha.1",
    "compatibility": "compatible"
  },
  "status": "pass",
  "complete": true,
  "summary": {
    "declared": 82,
    "executed": 82,
    "passed": 82,
    "failed": 0,
    "notExecuted": 0
  },
  "issues": [],
  "failures": []
}
```

`issues` contain stable `norm/conformance/*` codes for preflight, bundle, or
runner failures. `failures` contain a manifest case ID and one or more stable
checks from `execution`, `exit`, `stdout`, `stderr`, and `apiVersion`; mutable
messages and raw host paths are not compatibility fields. Known isolated paths
are normalized, and the machine report does not copy arbitrary candidate
streams that could leak host-absolute paths.

Initial issue codes are frozen as:

- `norm/conformance/usage`;
- `norm/conformance/candidate-unavailable`;
- `norm/conformance/compatibility-unavailable`;
- `norm/conformance/candidate-incompatible`;
- `norm/conformance/bundle-unavailable`;
- `norm/conformance/bundle-mismatch`;
- `norm/conformance/bundle-unsafe-path`;
- `norm/conformance/case-setup`;
- `norm/conformance/runner-unsupported`.

Each issue has required `code` and non-empty `message` fields. Each case
failure has required `caseId` and non-empty ordered `checks`; one case produces
at most one failure entry. Candidate `name` and `version` are strings when a
valid compatibility response provides them and JSON `null` otherwise.
Candidate `compatibility` is exactly `compatible`, `incompatible`, or
`unavailable`.

Summary invariants are `executed = passed + failed` and
`declared = executed + notExecuted`. `complete` is true only when every
declared case was executed and no global runner or bundle issue prevented the
suite. A compatibility issue may make the overall status `fail` while the
suite remains complete and all case checks pass.

The proposed status and exit rules are:

- `pass`, complete, exit `0`: compatibility preflight passes and all 82 cases
  execute and match;
- `fail`, complete, exit `1`: the candidate is executable and every case is
  attempted, but compatibility or one or more case checks fail;
- `error`, incomplete, exit `2`: runner usage, candidate availability, bundle
  integrity, fixture materialization, or process support prevents complete
  execution.

An incompatible but executable candidate is still exercised across all 82
cases so the report does not hide behavioral evidence. A missing or invalid
candidate/bundle yields explicit `notExecuted` counts and an incomplete error,
never a skip or successful empty result. Case mismatches do not stop later
cases. Issues use stable preflight order; case failures use manifest order and
the fixed check order shown above. Candidate version substitution comes only
from a valid compatibility response and never falls back to scraping help or
guessing from the runner version. The candidate path is resolved before case
working directories change and is never emitted as a host-absolute report
field.

The `norm-spec/conformance/v1` report therefore includes suite identity,
candidate identity, totals, failures, and completion state.

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

Batch 1 is complete. Candidate `07a95c6` passed hosted Linux, macOS, and Windows
CI; the documentation-only verification record may be fast-forwarded without a
second hosted run under the repository's agreed documentation policy.

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
bash scripts/check-packages.sh
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

1. **D1.1 package layout** — satisfied by the maintainer's authorization to
   continue into implementation and the evidence recorded in D014 after the
   packaging spike.
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
