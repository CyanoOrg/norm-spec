# Gate B — Behavior Contract Working Document

> Status: mutable working document. The authoritative contract lives in
> `docs/SPEC.md`, `schema/`, `tests/contract/`, and `docs/decisions.md`. This
> file coordinates the Gate B freeze and is superseded once the contract
> fixtures and runner land.

## Purpose

Gate B freezes the observable behavior of the five initial CLI commands
(`parse`, `collect`, `validate`, `init`, `scan`) before any parser or validator
implementation. This document is the working surface for that freeze: flag
matrix, machine envelopes, exit codes, error codes, fixture inventory, runner
specification, and open decisions.

## Baseline statement

The contract is grounded in the A1 behavior captured from the private Python
prototype. Per D002 the prototype is a **reference input only** — not a runtime
dependency, not an authority, and not imported into this repository. The Rust
product ports observable behavior and records intentional deviations in D006.

## Command flag matrix

| Command | Flags | Notes |
|---|---|---|
| `parse` | `path` (positional; a directory resolves to `<dir>/.norm`), `--legacy-format`, `--pretty` | |
| `collect` | `--root` (default `.`), `--target` (required), `--legacy-format`, `--pretty` | |
| `validate` | `path?` (positional), `--all`, `--root` (default `.`), `--schema-dir`, `--profile`, `--strict`, `--legacy-format`, `--compat-keys` | `--compat` alias is not ported (D006) |
| `init` | `--profile` (required), `--output` (default `.norm`), `--force` | |
| `scan` | `--root` (default `.`), `--pretty`, `--text` | |

Global flags: `--version` / `-V`, `--help` / `-h`.

## Machine envelopes

Every machine-readable stdout payload is a JSON object whose first field is
`apiVersion`. Human-only output (`--text`, progress, the `validate` summary) is
not a versioned envelope.

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
`code` (see Error codes) and a human `message`.

### `norm-spec/init/v1`

```json
{
  "apiVersion": "norm-spec/init/v1",
  "profile": "module",
  "path": "docs/modules/example/.norm",
  "created": true
}
```

### `norm-spec/scan/v1`

```json
{
  "apiVersion": "norm-spec/scan/v1",
  "root": ".",
  "directory_count": 3,
  "directories": [ { "path": ".", "depth": 0, "file_count": 2, "has_norm": true } ],
  "naming": { "directories": { "kebab-case": 1 }, "files": { "lowercase": 2 } },
  "recurring_filenames": [ { "name": "README.md", "dir_count": 2 } ],
  "norm_coverage": { "total_dirs": 3, "dirs_with_norm": 1, "ratio": 0.333 }
}
```

`recurring_filenames` lists names appearing in two or more directories, capped
at twenty. `root` is relative (`.` when the scan root is the project root).

## Exit codes

Inherited from the baseline and frozen (D006).

| Code | Meaning |
|---|---|
| `0` | Success. No errors; under `--strict`, no warnings either. |
| `1` | Data or validation failure: parse error, schema/profile/semantic error, or a warning under `--strict`. |
| `2` | Usage or configuration error: unknown flag, missing required argument, schema not found, path outside root. |

## Error codes

Stable machine strings. Human wording may improve; codes and field paths may
not change without a protocol decision. Seed list, to expand during fixture
work:

| Code | Meaning |
|---|---|
| `norm/parse/empty` | Empty file or empty frontmatter. |
| `norm/parse/missing-closing-fence` | A1 frontmatter opened with no closing `---`. |
| `norm/parse/not-a1` | First non-blank line is not `---`; needs `--legacy-format`. |
| `norm/parse/yaml` | YAML parse error in frontmatter. |
| `norm/schema/unknown-key` | Unknown top-level key (warn unless `--compat-keys`; suggest closest match). |
| `norm/schema/version-format` | `metadata.version` is not `MAJOR.MINOR`. |
| `norm/profile/unknown-explicit` | `metadata.profile` names no known profile. |
| `norm/semantic/lifecycle` | Lifecycle state machine invariant violated. |
| `norm/semantic/ssot-duplicate` | `single_source_of_truth` domain declared more than once. |
| `norm/reference/not-found` | Reference target does not exist. |
| `norm/reference/outside-root` | Reference target escapes the project root. |
| `norm/path/outside-root` | Operation target is outside the root. |
| `norm/usage/missing-argument` | Required argument missing. |
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
- Scan ignores infrastructure directories (`.git`, `node_modules`,
  `__pycache__`, build output) and reports naming, recurring filenames, and
  coverage.

## Runner specification

The runner is Rust integration tests under `crates/norm-cli/tests/`, consuming
the language-neutral `tests/contract/{fixtures,expected}/` directory. It invokes
the `norm` binary, compares stdout as canonical JSON (object-key order ignored,
ordered arrays preserved), normalizes temporary and host-specific paths, and
checks exit codes and stable error codes.

Discipline:

- A missing fixture or expected file is a test failure (`panic`), never a skip.
- `#[ignore]` is forbidden for contract tests.
- The runner specification is frozen in Gate B; executable test bodies land
  with each command's implementation in Gate C (red to green), keeping `main`
  green during Gate B.

## Open decisions

- **Symbolic links.** Follow, reject, or report? Not yet decided (D006 open
  item). Does not block the rest of Gate B; must be decided before collect,
  validate, and scan implementation.
