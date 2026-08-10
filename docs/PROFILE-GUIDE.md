# Profile Guide

> How to pick a profile, and per-profile examples.

Profiles are optional field-recommendation templates for common directory roles (see [SPEC.md](SPEC.md) for the format and the profile mechanism). This guide helps you choose one and shows a full A1 example for each. You may also skip profiles entirely and rely on the core schema + semantic validation.

## Selecting a profile

A profile is selected in one of three ways (highest precedence first):

1. **`norm validate --profile <name>`** — the CLI flag forces a profile.
2. **`metadata.profile`** (optional, decision D13) — explicitly names the profile
   to apply. Use it when your `layer` label differs from the checks you want.
3. **`metadata.layer`** (best-effort) — if the layer value names a known profile,
   that profile's recommended checks load.

The layer never has to match a profile — it is a free-form descriptor.

| Profile | Applicable scenario |
|---------|---------------------|
| root | Project documentation root — overall structure, SSOTs, naming, update order |
| module | Component/package documentation — required files, version source, cross-module refs |
| epic | Work-unit PRDs — file structure, scope definition, archiving, lifecycle |
| architecture | Design documents — drafts, lifecycle, reference policy |
| task | Task/work-item definitions — required files, structure, scope |
| test | Test directories — scope boundaries, NOT-lists, naming |
| convention | Standards/convention directories — required files, SSOTs |

When NOT to use a profile: if your directory role does not resemble any of these, write a free-form `layer` (e.g. `build-tools`, `ci`) and let only the core schema + semantic validation apply. You can always adopt a profile later.

---

## Profile reference

### root

Top-level conventions for a project's documentation. Declares which documentation directories must exist, the project SSOTs, and naming rules.

**Key fields:** `template.required_directories`, `template.single_source_of_truth`, `template.naming_convention`, `agent_rules.update_order`, `strong_references`.

```yaml
---
metadata:
  layer: root
  scope: docs/
  version: "1.0"
  description: Overall documentation structure

template:
  required_directories: [development/, prd/, modules/]
  single_source_of_truth:
    - source: product-strategy.md
      for: product vision, target users, feature priorities
  naming_convention:
    - "kebab-case for directories and filenames"

agent_rules:
  update_order:
    - "Confirm product-strategy.md is the source of truth"
    - "Update downstream documents (plans -> epics -> stories)"
    - "Keep all layers consistent"
---

# Documentation root

Overall structure and sources of truth for project documentation.
```

### module

A single component or package's documentation. Declares required module files, the version source, naming, and cross-module references.

**Key fields:** `template.required_files`, `template.single_source_of_truth`, `template.naming_convention`, `agent_rules.update_order`, `cross_references`.

```yaml
---
metadata:
  layer: module
  scope: docs/modules/mcp/
  version: "1.0"
  description: MCP module documentation structure

template:
  required_files: [ARCHITECTURE.md, DEVELOPLOG.md]
  single_source_of_truth:
    - source: package.json
      for: version number
    - source: ARCHITECTURE.md
      for: architecture documentation index
  naming_convention:
    - "UPPERCASE for top-level filenames"

agent_rules:
  update_order:
    - "Update DEVELOPLOG.md from commits"
    - "Update ARCHITECTURE.md index only if structure changed"
    - "Bump package.json version on release"
---

# MCP module

Documentation structure for the MCP module.
```

### epic

Work-unit PRDs. Declares required Epic file structure, work-scope definition, and archiving rules.

**Key fields:** `template.required_files`, `template.file_structure`, `template.scope_definition`, `agent_rules.archiving`, `agent_rules.update_order`, `strong_references`.

```yaml
---
metadata:
  layer: epic
  scope: docs/prd/epics/
  version: "1.0"
  description: Epic document structure and conventions

template:
  required_files: ["{EpicName}.md"]
  scope_definition:
    description: "Define work scope (repository + branch)"
  single_source_of_truth:
    - source: "Epic .md file"
      for: objectives, scope, story list, progress

agent_rules:
  archiving:
    file_format: "archive/[{YYYY-MM-DD}]-{epic-slug}.md"
    date_source: epic_completed_date
  update_order:
    - "Update Epic overview if scope changes"
    - "Update story list and status"
    - "Update progress percentage"
---

# Epics

Structure and lifecycle for Epic PRDs.
```

### architecture

Design documents. Declares the drafts area, design-decision SSOTs, and a structured lifecycle that separates "design accepted" from "design implemented".

**Key fields:** `template.required_directories`, `agent_rules.document_lifecycle`, `agent_rules.reference_policy`, `strong_references`.

```yaml
---
metadata:
  layer: architecture
  scope: docs/architecture/
  version: "1.0"
  description: Architecture documentation structure

template:
  required_directories: [drafts/]
  single_source_of_truth:
    - source: "Architecture documents"
      for: system design decisions, patterns, trade-offs

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
      - {from: review, to: accepted}
      - {from: accepted, to: current, requires: ["implementation verified"]}
      - {from: current, to: superseded}
      - {from: superseded, to: archived}
  reference_policy:
    target_must_exist: true
    allow_external: false
    require_description: true
---

# Architecture

Design documents and their lifecycle.

`accepted` means the design is approved, not that it is implemented. Use a separate delivery state to track actual implementation. `current` is the live architecture SSOT, reached only after implementation is verified and the current architecture is updated.
```

### task

Task / work-item definitions. Declares the task file, required sections, and the task's scope SSOT.

**Key fields:** `template.required_files`, `template.file_structure`, `template.single_source_of_truth`, `agent_rules.update_order`.

```yaml
---
metadata:
  layer: task
  scope: .pipeline/tasks/epic-101/
  version: "1.0"
  description: Task definition and acceptance criteria

template:
  required_files: [TASK.md]
  file_structure:
    - "Task title and objectives"
    - "Story list with explicit status"
    - "Acceptance criteria (DoD)"
    - "Dependencies and scope boundaries"
  single_source_of_truth:
    - source: TASK.md
      for: task scope, story breakdown and acceptance details

agent_rules:
  update_order:
    - "Update task scope and story details as work progresses"
    - "Record story completion with commit references"
    - "Never infer completion without verification evidence"
---

# Task definitions

Structure for task/work-item documents.
```

### test

Test directories. Declares scope boundaries (including an explicit NOT-list) and naming rules, plus boundaries to neighboring test directories.

**Key fields:** `scope.covers`, `scope.NOT`, `agent_rules.test_naming`, `cross_references`.

```yaml
---
metadata:
  layer: test
  scope: tests/mcp/
  version: "1.0"
  description: MCP protocol layer tests

scope:
  covers:
    - "MCP Transport (Streamable HTTP)"
    - "Resource protocol"
    - "Session lifecycle"
  NOT:
    - "log_management MCP Tool -> tests/logsystem/"
    - "log://console -> tests/companion/logging/"

agent_rules:
  test_naming: "test_{module}_{behavior}.py"

cross_references:
  - target: "../companion/core/"
    boundary: "Integration smoke tests by companion/core"
---

# MCP tests

Scope and boundaries for the MCP test directory.
```

### convention

Standards and convention directories. Declares where each convention topic lives (SSOTs) and that reference targets must resolve.

**Key fields:** `template.required_files`, `template.single_source_of_truth`, `agent_rules.update_order`, `agent_rules.reference_policy`, `strong_references`.

```yaml
---
metadata:
  layer: convention
  scope: docs/development/
  version: "1.0"
  description: Development status, catalog and convention ownership

template:
  required_files: [conventions.md, status.md]
  single_source_of_truth:
    - source: conventions.md
      for: overview index of all governance topics
    - source: status.md
      for: human-readable current progress summary

agent_rules:
  update_order:
    - "Update catalog facts before rendering status summaries"
    - "Never infer delivery state from a document directory alone"
  reference_policy:
    target_must_exist: true
    allow_external: false
    require_description: true

strong_references:
  - type: source
    target: /docs/ROADMAP.md
    sync: auto_pull
    validation: strict
    description: "Roadmap is the narrative source for milestone decisions"
---

# Development conventions

Ownership and sources of truth for governance docs.
```

---

## Consumer integration

norm-spec defines the format and provides tooling; it does not bind to any
specific consumer. For the full adoption path (project-instruction snippet,
Skill install, CLI recipes, pre-commit/GitHub Action), see
[INTEGRATION.md](INTEGRATION.md). The snippet below is the minimum discovery
instruction that makes an agent look for `.norm` in your project.

> **Adopting `.norm` (discovery only).** Before a `.norm` can be collected or
> validated, the agent has to know to look for one. The lowest-cost discovery
> mechanism is a short instruction in your project's agent file — it works with
> **no CLI and no Skill installed**, in any agent framework.

Add this snippet to your project's agent instruction file:

```markdown
## .norm awareness

Before operating in any directory, walk up the directory tree and check for
`.norm` files. If present, read each one's frontmatter and honor its
`template` / `agent_rules` / `references` constraints for the duration of the
task.
```

Where to put it, by framework:

| Framework | File |
|-----------|------|
| Claude Code | `CLAUDE.md` (project root) |
| OpenClaw / Hermes | `AGENTS.md` (workspace or project) |
| Codex | `AGENTS.md` or `.codex/` (per your config) |
| Generic | `AGENTS.md` — the cross-framework common denominator |

norm-spec does **not** auto-inject this — modifying another project's agent
file is too invasive. Copy-paste it once per project. This is the *discovery*
tier of the consumption model; the Skill (Tier 2) automates collection and
validation.
