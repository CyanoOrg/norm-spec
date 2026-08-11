# Gate B — Behavior Contract Working Document

> Status: frozen on 2026-08-11 (D008). The authoritative contract lives in
> `docs/SPEC.md`, `schema/`, `tests/contract/`, and `docs/decisions.md`. Amend
> observable behavior only through a new decision plus synchronized contract
> assets and tests.

## Purpose

Gate B freezes the observable behavior of the five initial CLI commands
(`parse`, `collect`, `validate`, `init`, `scan`) before any parser or validator
implementation. This document records that freeze: flag
matrix, machine envelopes, exit codes, error codes, fixture inventory, runner
specification, resolved decisions, and closure evidence.

## Baseline statement

The contract is grounded in the A1 behavior captured from the private Python
prototype. Per D002 the prototype is a **reference input only** — not a runtime
dependency, not an authority, and not imported into this repository. The Rust
product ports observable behavior and records intentional deviations and
filesystem semantics in D006–D008.

## Command flag matrix

| Command | Flags | Notes |
|---|---|---|
| `parse` | `path` (positional; a directory resolves to `<dir>/.norm`), `--legacy-format`, `--pretty` | |
| `collect` | `--root` (default `.`), `--target` (required), `--legacy-format`, `--pretty` | |
| `validate` | `path?` (positional), `--all`, `--root` (default `.`), `--schema-dir`, `--profile`, `--strict`, `--legacy-format`, `--compat-keys`, `--json`, `--pretty` | `--compat` alias is not ported; `--pretty` requires `--json` (D006) |
| `init` | `--profile` (required), `--output` (default `.norm`), `--force`, `--json`, `--pretty` | `--pretty` requires `--json` (D006) |
| `scan` | `--root` (default `.`), `--pretty`, `--text` | |

Global flags: `--version` / `-V`, `--help` / `-h`.

Flag and input-path rules:

- `validate` requires exactly one of positional `path` or `--all`.
- `validate --pretty` and `init --pretty` require `--json`.
- `scan --text` conflicts with `--pretty`.
- `--root`, positional paths, `--schema-dir`, and init `--output` resolve from
  the current working directory. A relative collect `--target` resolves from
  its canonical `--root` (D006).
- Root and target containment uses canonical paths; missing input paths fail
  before command-specific parsing or traversal.

## Machine envelopes

Every machine-mode stdout payload is one JSON object containing a top-level
`apiVersion`; object-key order is not semantic. `parse`, `collect`, and `scan`
default to machine mode, while `validate` and `init` select it with `--json`.
`--pretty` changes whitespace only. Human-only output (`scan --text`, progress,
the default `validate` report, and the default `init` confirmation) is not a
versioned envelope.

Handled failures that prevent a command-specific machine response emit exactly
one `norm-spec/error/v1` object to stdout, followed by a newline, and leave
stderr empty. A completed `validate --json` evaluation emits
`norm-spec/validate/v1` even when validation errors or strict warnings make its
exit status `1`; preflight failures such as a missing input or schema use
`norm-spec/error/v1`. Human-mode usage and operation failures emit their
diagnostic on stderr; regular validation results and summaries remain on
stdout. Panics, aborts, and operating-system process failures are outside the
machine protocol and must never be converted into an empty success response.

### `norm-spec/parse/v1`

```json
{
  "apiVersion": "norm-spec/parse/v1",
  "frontmatter": { "metadata": { "layer": "example", "scope": "./", "version": "1.0" } },
  "body": "# Minimal project\n\nA valid A1 document with a Markdown body."
}
```

### `norm-spec/collect/v1`

```json
{
  "apiVersion": "norm-spec/collect/v1",
  "root": ".",
  "target": "docs/modules/example",
  "norms": [
    { "path": "docs/modules/example/.norm", "frontmatter": { "...": "..." }, "body": "..." },
    { "path": "docs/.norm", "frontmatter": { "...": "..." }, "body": "..." }
  ]
}
```

`path`, `root`, and `target` are relative to the project root (D006). `norms`
are ordered most-specific first.

### `norm-spec/validate/v1`

```json
{
  "apiVersion": "norm-spec/validate/v1",
  "root": ".",
  "results": [
    { "path": "docs/.norm", "status": "ok", "errors": [], "warnings": [] },
    { "path": "schema/.norm", "status": "warning", "errors": [], "warnings": [ { "code": "norm/profile/unknown-explicit", "message": "..." } ] }
  ],
  "summary": { "files": 2, "errors": 0, "warnings": 1 }
}
```

`status` is one of `ok`, `warning`, `error`. Each diagnostic carries a stable
`code` (see Error codes), an affected `field` when one is available, and a
human `message`. A diagnostic may also carry a stable `suggestion` when the
consumer needs a machine-actionable correction (for example, `agent_rules`
for a misspelled key or `MAJOR.MINOR` for a version-format error).

Human output is not a machine compatibility surface. Human-output fixtures use
ordered required lines: commands may improve surrounding prose, but command
identity, stable error code, affected path, result classification, and summary
counts remain observable.

### `norm-spec/init/v1`

```json
{
  "apiVersion": "norm-spec/init/v1",
  "profile": "module",
  "path": "docs/modules/example/.norm",
  "action": "created"
}
```

`action` is `created` for a new file and `overwritten` when `--force` replaces
an existing file.

### `norm-spec/scan/v1`

```json
{
  "apiVersion": "norm-spec/scan/v1",
  "root": ".",
  "directory_count": 3,
  "directories": [ { "path": ".", "depth": 0, "file_count": 2, "has_norm": true } ],
  "symlinks": [ { "path": "linked-docs", "kind": "directory", "action": "not-followed" } ],
  "naming": { "directories": { "kebab-case": 1 }, "files": { "lowercase": 2 } },
  "recurring_filenames": [ { "name": "README.md", "dir_count": 2 } ],
  "norm_coverage": { "total_dirs": 3, "dirs_with_norm": 1, "ratio": 0.333 }
}
```

`recurring_filenames` lists names appearing in two or more directories, capped
at twenty. `root` is relative (`.` when the scan root is the project root).

### `norm-spec/error/v1`

```json
{
  "apiVersion": "norm-spec/error/v1",
  "command": "parse",
  "error": {
    "code": "norm/parse/missing-closing-fence",
    "message": "A1 frontmatter opened without a closing fence.",
    "path": ".norm"
  }
}
```

`error.code` and any present `path` or `field` are stable machine fields.
`message` is human-readable and may improve without a protocol version change.

## Exit codes

Normalized deliberately from the baseline and frozen (D006).

| Code | Meaning |
|---|---|
| `0` | Success. No errors; under `--strict`, no warnings either. |
| `1` | Input, data, validation, or operation failure: missing input, parse/schema/profile/semantic error, unknown init profile, existing init output without `--force`, invalid scan root, rejected `.norm` symlink, or a warning under `--strict`. |
| `2` | CLI usage, configuration, or containment failure: unknown/conflicting flag, missing required argument, schema not found, or an operation target resolving outside root. |

Reference paths declared by a parsed `.norm` are data, so a missing or
outside-root reference remains a validation failure (`1`). The CLI target/root
containment check happens before reading `.norm` data and returns `2`.

## Error codes

Stable machine strings. Human wording may improve; codes and field paths may
not change without a protocol decision. Frozen Gate B list:

| Code | Meaning |
|---|---|
| `norm/parse/empty` | Empty file or empty frontmatter. |
| `norm/parse/missing-closing-fence` | A1 frontmatter opened with no closing `---`. |
| `norm/parse/not-a1` | First non-blank line is not `---`; needs `--legacy-format`. |
| `norm/parse/root-type` | Parsed frontmatter root is not a mapping. |
| `norm/parse/yaml` | YAML parse error in frontmatter. |
| `norm/schema/unknown-key` | Unknown top-level key (error by default; warn with `--compat-keys`; suggest closest match). |
| `norm/schema/version-format` | `metadata.version` is not `MAJOR.MINOR`. |
| `norm/profile/unknown-explicit` | `metadata.profile` names no known profile. |
| `norm/profile/recommended-field` | A selected profile omits a recommended field. |
| `norm/semantic/lifecycle` | Lifecycle state machine invariant violated. |
| `norm/semantic/lifecycle-legacy` | Legacy lifecycle string-list form remains accepted but should migrate. |
| `norm/semantic/ssot-duplicate` | `single_source_of_truth` domain declared more than once. |
| `norm/reference/not-found` | Reference target does not exist. |
| `norm/reference/outside-root` | Reference target escapes the project root. |
| `norm/reference/duplicate` | The same typed reference target is declared more than once. |
| `norm/reference/description-required` | Reference policy requires a missing description. |
| `norm/reference/external-not-allowed` | External reference is disallowed by policy. |
| `norm/path/outside-root` | Operation target is outside the root. |
| `norm/path/not-found` | Required input path does not exist. |
| `norm/path/not-directory` | An operation requiring a directory received another path type. |
| `norm/path/symlink-norm` | A `.norm` path is a symbolic link and is rejected. |
| `norm/init/unknown-profile` | Requested init profile does not exist. |
| `norm/init/output-exists` | Init output exists and `--force` was not supplied. |
| `norm/usage/missing-argument` | Required argument missing. |
| `norm/usage/conflicting-arguments` | Mutually exclusive arguments were supplied together. |
| `norm/usage/schema-not-found` | Schema directory or file not found. |

## Cross-cutting fixture inventory

Each case needs at least one fixture under `tests/contract/fixtures/` and an
expected result under `tests/contract/expected/`. Invalid inputs use the
`.norm.invalid` suffix.

- BOM (valid after strip).
- Leading blank lines before the opening `---` (valid).
- Missing closing fence (invalid).
- Empty frontmatter (invalid).
- Frontmatter root not a mapping (invalid).
- Pre-A1 `# Title + YAML` without `--legacy-format` (invalid) and with it (valid).
- Unknown top-level key without `--compat-keys` (invalid) and with it (warning);
  closest-match suggestion asserted.
- `metadata.version` bad format (invalid) with MAJOR.MINOR hint.
- Free-form `layer` accepted; explicit `metadata.profile` respected; unknown
  explicit profile warned.
- Lifecycle: unique states, reachable initial/terminal/targets, no duplicate
  transitions, terminal has no outgoing edge; legacy string-list form warned.
- SSOT duplicate domain (invalid).
- References: missing target, outside-root target, duplicate target, missing
  description under `require_description`, external under `allow_external=false`.
- Collect target outside root (invalid).
- Missing input paths and path-type mismatches.
- Conflicting output flags and `validate path` combined with `--all`.
- Symlinks: canonical target containment, outside-root rejection, `.norm`
  symlink rejection, and deterministic reporting of untraversed directory
  symlinks (D007).
- Scan ignores infrastructure directories (`.git`, `node_modules`,
  `__pycache__`, build output) and reports naming, recurring filenames, and
  coverage.

## Runner specification

The language-neutral `tests/contract/requirements.tsv` inventories every Gate B
obligation. `tests/contract/manifest.tsv` binds each case to command arguments,
a fixture or layout recipe, stdout/stderr expectations and comparison modes, a
machine protocol when applicable, an exit code, and one or more requirement
IDs. A Rust integration test under `crates/norm-cli/tests/` validates that all
requirements are covered and all referenced assets are present during Gate B.

Manifest argument lists are JSON arrays with portable placeholders such as
`{fixture}`, `{root}`, `{outside}`, `{schema}`, and `{output}`. Layout recipes
are tab-separated, language-neutral instructions for constructing directory,
file, outside-root, and symbolic-link cases in an isolated temporary root.

Command runners land with each vertical implementation slice in Gate C. They
invoke the `norm` binary, compare stdout as canonical JSON (object-key order
ignored, ordered arrays preserved), normalize temporary and host-specific
paths, and check exit codes and streams. Success envelopes compare canonical
data. Error envelopes compare `apiVersion`, `command`, `error.code`, and any
present `path` / `field`; `message` must be non-empty but its exact wording is
not compared.

Discipline:

- A missing manifest, fixture, or expected file is a test failure (`panic`),
  never a skip.
- `#[ignore]` is forbidden for contract tests.
- Gate B is complete only when every requirement has at least one manifest
  case, all five commands have success contracts, human and machine modes are
  represented, and machine failures include `norm-spec/error/v1` coverage.
- Gate C replaces static presence checks with executable command assertions one
  command at a time; an applicable but unavailable runner fails rather than
  skipping.

## Resolved decisions

- **Symbolic links.** Canonical containment, no recursive directory-link
  traversal, rejected `.norm` links, and deterministic scan reporting (D007).
- **Contract freeze.** Requirement-to-case traceability, machine response
  selection, comparison semantics, and human-output stability (D008).

## Closure evidence

Gate B closed on 2026-08-11 with 91 requirements, 82 manifest cases, 54 stream
expectations, 32 direct fixture files, and 12 layout recipes. The Rust static
integrity test proves complete requirement and flag coverage, per-command
machine success/error coverage, applicable human-mode coverage, valid
protocol/exit/stream combinations, portable placeholders and paths, valid
layout instructions, and the absence of missing or orphan expected assets.
Executable command assertions replace these presence checks during Gate C.
