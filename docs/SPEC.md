# .norm Format Specification

> The authoritative definition of the `.norm` file format.

A `.norm` file is a structured declaration placed in a project directory. An AI agent collects it before working there so it knows the directory's conventions — required files, single sources of truth, update order, document lifecycle, references. norm-spec defines this format and ships a validator. It is **format- and tooling-only**; how a consumer collects, injects, and obeys `.norm` is the consumer's concern, not the spec's.

## Frozen CLI surface

Gate B has frozen the public form of the five initial commands; their production
implementation is complete locally. Global help/version and all five commands
execute the compiled binary across all 82 frozen cases without skips. The
authoritative behavior inventory is `tests/contract/requirements.tsv` plus
`tests/contract/manifest.tsv`, with rationale in `docs/decisions.md`
(D006–D012) and working detail in `docs/planning/gate-b-contract.md`.

```bash
# Create a .norm from a profile template
norm init --profile module --output docs/modules/example/.norm

# Overwrite an existing .norm
norm init --profile module --output docs/modules/example/.norm --force

# Emit the versioned init machine envelope
norm init --profile module --output docs/modules/example/.norm --json --pretty

# Validate all .norm files (strict)
norm validate --all --strict

# Emit the versioned validation machine envelope
norm validate --all --strict --json --pretty

# Validate against an explicit schema directory
norm validate --all --strict --schema-dir schema/

# Downgrade unknown top-level keys to warnings
norm validate --all --compat-keys

# Validate pre-A1 compatibility files ('# Title + YAML')
norm validate --all --strict --legacy-format

# Parse a .norm to JSON (frontmatter + body)
norm parse docs/.norm --pretty

# Collect inherited .norm for a target directory
norm collect --root . --target docs/modules/example --pretty

# Structural scan (raw material for writing .norm; no inference)
norm scan --root . --text
```

Machine responses use `norm-spec/parse/v1`, `norm-spec/collect/v1`,
`norm-spec/validate/v1`, `norm-spec/init/v1`, `norm-spec/scan/v1`, or the common
`norm-spec/error/v1` envelope. JSON object key order is not semantic; ordered
collections remain ordered. Output paths are root-relative, and exit codes are
`0` for success, `1` for input, data, validation, or operation failure, and `2`
for usage, configuration, or CLI root-containment failure. A completed
validation report keeps `norm-spec/validate/v1` even at exit `1`; preflight
machine failures use `norm-spec/error/v1`.

Filesystem operations canonicalize roots and targets. They do not recursively
follow directory symbolic links, reject a `.norm` that is itself a symbolic
link, and reject operation targets outside the declared root. Scan reports
untraversed directory links deterministically.

## Concepts

### What a .norm is
A `.norm` declares the conventions of the directory it lives in. An agent's interaction model: **collect** (walk up the directory tree gathering `.norm` files) → **read** (frontmatter constraints + body explanation) → **obey** (during work) → **validate** (optional, after output).

### Directory inheritance
`.norm` inherits up the directory tree. Collecting from a target walks upward; the closest `.norm` is the most specific.

```
docs/modules/mcp/.norm    ← most specific
docs/modules/.norm
docs/.norm                ← least specific
```

How collection and priority are resolved is the consumer's choice. norm-spec's
`collect` command defines the canonical CLI traversal behavior.

## File format (A1)

A `.norm` is a **YAML frontmatter** block delimited by `---` fences, followed by a free-form **Markdown body**.

```yaml
---
metadata:
  layer: build-tools                # free-form label of this directory's role
  scope: build-tools/               # directory this .norm covers
  version: "1.0"                    # MAJOR.MINOR
  description: Build tooling        # optional human description

template:
  required_files: [Makefile]        # file/directory structure requirements

agent_rules:
  update_order: [...]               # agent behavior rules
  reference_policy:
    target_must_exist: true

strong_references: [...]            # cross-directory references
---

# Build Tools

Free-form Markdown. The agent reads this too — use it for prose explanation,
examples, and rationale that does not belong in the structured frontmatter.
```

**Frontmatter vs body**
- **Frontmatter (YAML):** all machine-enforced fields — `metadata`, `template`, `agent_rules`, `scope`, `cross_references`, `strong_references`.
- **Body (Markdown):** human-readable explanation; the format is free.

**Format rules**
1. The first non-blank line must be `---`; the frontmatter ends at the next `---` line. Everything after is the body (may be empty).
2. Top-level frontmatter keys: `metadata`, `template`, `agent_rules`, `scope`, `cross_references`, `strong_references`. Unknown keys are rejected unless `--compat-keys` downgrades them to warnings. The validator suggests the closest known key on typos.
3. `metadata.version` is required, in `"MAJOR.MINOR"` format.
4. Encoding: UTF-8.

**Pre-A1 compatibility format** (`# Title` + full YAML, no fences) is parsed
only with `--legacy-format`.

## Core schema

### metadata (required)

```yaml
metadata:
  layer: string          # free-form label of this directory's role (profile matching is best-effort)
  profile: string        # optional explicit profile name (overrides layer matching when present)
  scope: string          # directory path this .norm covers
  version: string        # "MAJOR.MINOR"
  description: string    # optional human description
  last_updated: string   # optional ISO 8601 date
```

`layer` is a **free-form descriptive string**, not an enum. If the value matches a known profile name, that profile's recommended checks are loaded (best-effort); otherwise only base schema + semantic validation runs. `profile` (optional, decision D13) explicitly names the profile to apply and takes precedence over `layer`-based matching — use it when your layer label differs from the checks you want. `description` is optional — the body can carry the human description.

### template (recommended)

File and directory structure requirements.

```yaml
template:
  required_files: [string]
  optional_files: [string]
  required_directories: [string]
  optional_directories: [string]
  naming_convention: [string]
  single_source_of_truth:
    - source: string       # file/directory path
      for: string          # what it is the sole source of
  file_structure: [string]      # required sections within document files
  scope_definition: object      # work-scope boundary rules
  diagram_registry:
    path: string
    format: string
```

### agent_rules (recommended)

Rules an AI agent should follow when working in this directory.

```yaml
agent_rules:
  update_order: [string]
  archiving: object
  module_version_sync: [string]
  document_lifecycle: object | [string]   # structured state machine (preferred) or compatibility string list
  reference_policy: object
  test_naming: string
  no_test_results: bool
```

#### document_lifecycle

A machine-verifiable state machine. New norms should use the structured form;
the string-list compatibility form is read with a warning.

```yaml
agent_rules:
  document_lifecycle:
    state_field: document_state
    states:
      - {id: draft}
      - {id: review}
      - {id: accepted}
      - {id: current}
      - {id: superseded}
      - {id: archived}
    initial: draft
    terminal: [archived]
    transitions:
      - {from: draft, to: review}
      - {from: accepted, to: current, requires: ["implementation verified"]}
      - {from: current, to: superseded}
      - {from: superseded, to: archived}
```

The validator checks: state ids are unique; `initial`, `terminal`, and transition targets all resolve; transitions are not duplicated; terminal states have no outgoing edges. `requires` entries are free-text gates (e.g. promotion preconditions). The *current* state of a document is stored in the consumer's catalog/metadata (named by `state_field`); `.norm` only defines the rules to maintain it.

#### reference_policy

```yaml
agent_rules:
  reference_policy:
    target_must_exist: true
    allow_external: false
    require_description: true
```

Applied to both `cross_references` and `strong_references`. With `target_must_exist`, targets beginning `/` resolve from the project root and relative targets resolve from the `.norm`'s own directory; all local targets must stay within the project root. The validator also rejects duplicate references and duplicate SSOT domains.

### scope (recommended for test-like directories)

Responsibility boundaries.

```yaml
scope:
  covers: [string]
  NOT: [string]
  subdirectories: object
```

### cross_references / strong_references

Relationships to other directories.

```yaml
cross_references:
  - target: string
    boundary: string
    sync: string          # auto_pull | auto_push | manual
    validation: string    # strict | loose

strong_references:
  - type: string          # upstream | downstream | cross_reference | source
    target: string
    sync: string
    validation: string
    description: string
```

## Profiles

Profiles are ready-made **field-recommendation templates** for common directory roles. They are optional: a `.norm` may use a profile, or rely on the core schema + semantic validation alone.

Profile validation has two layers:
- **Profile schema** (`schema/profiles/*.json`): extends the core schema. A profile no longer hard-requires a matching `layer`.
- **`profile_rules`** (`required_fields` / `recommended_fields` / recommended sub-fields): missing recommended fields surface as warnings (errors under `--strict`).

`metadata.layer` and profile are **decoupled**: `layer` is a free-form descriptor, profile matching is best-effort, and a profile never rejects a file for declaring a different layer.

| Profile | Applicable scenario |
|---------|---------------------|
| root | Project documentation root — overall structure, SSOTs, naming |
| module | Component/package documentation — required files, version source |
| epic | Work-unit PRDs — file structure, scope, archiving, lifecycle |
| architecture | Design documents — drafts, lifecycle, reference policy |
| task | Task/work-item definitions — required files, structure |
| test | Test directories — scope boundaries, naming |
| convention | Standards/convention directories — required files |

For per-profile field recommendations and full A1 examples, see [PROFILE-GUIDE.md](PROFILE-GUIDE.md).
