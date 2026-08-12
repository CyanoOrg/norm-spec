---
name: norm-spec
description: Collect, apply, validate, author, and diagnose directory-scoped .norm project conventions with a compatible norm CLI. Use before changing files in a .norm-aware project, when creating or repairing .norm declarations, or when explaining inherited rules. Require versioned engine compatibility and stop when the engine is missing, incompatible, or fails.
---

# norm-spec workflow

Use the canonical `norm` engine for all parsing, directory collection, and
validation. Treat its versioned machine responses as evidence; never replace a
failed command with manual YAML parsing, a hand-written directory walk, an
empty ruleset, or a successful skip.

## 1. Verify the engine

Locate the `norm` executable using the current operating environment, then run:

```bash
norm compatibility --pretty
```

Require exit `0`, empty stderr, `norm-spec/compatibility/v1`, format
`norm-spec/a1`, and the machine API identifiers needed by the workflow. Do not
infer compatibility from `norm --version` or help text.

If the executable is missing, the response is malformed or incompatible, or
the command fails, stop the affected workflow. Report the observed failure and
direct the user to install or repair a compatible norm-spec release. Do not
continue by interpreting `.norm` yourself.

## 2. Collect before working

Determine the explicit project root and the exact file or directory target.
Set `NORM_ROOT` and `NORM_TARGET` to those paths, then run:

```bash
norm collect --root "$NORM_ROOT" --target "$NORM_TARGET" --pretty
```

Require exit `0`, empty stderr, and `norm-spec/collect/v1`. Read every returned
`norms` entry in its most-specific-first order. Apply the frontmatter
constraints and body guidance relevant to the intended work; do not discard
outer conventions. If collected rules conflict and the project does not state
how to resolve them, surface the conflict instead of inventing precedence.

Re-collect when the work target changes to another directory scope or when a
`.norm` file changes during the task.

## 3. Obey during the work

Translate the collected declarations into an explicit working checklist:

- preserve required files and directories;
- update declared single sources of truth before their projections;
- follow declared update order and lifecycle transition requirements;
- honor scope boundaries, reference policy, and test naming;
- keep strong and cross-reference targets consistent.

Do not claim that collection automatically enforces these rules. Retain the
command result and the checks you actually performed as evidence.

## 4. Validate before completion

After relevant edits, validate the project using the same explicit root:

```bash
norm validate --all --root "$NORM_ROOT" --strict --json --pretty
```

Require exit `0`, empty stderr, `norm-spec/validate/v1`, and zero errors and
warnings. A non-zero exit blocks completion. Diagnose stable error codes,
repair the source declaration or referenced project artifact, and rerun the
same command; never hide, downgrade, or skip a failed validation.

## 5. Author or revise a convention

Read [authoring.md](references/authoring.md) before creating or substantially
changing a `.norm`. Read [field-reference.md](references/field-reference.md)
when choosing fields, profiles, reference policy, or lifecycle structure.

Prefer an embedded starter when a profile fits. For example, with `NORM_ROOT`
set to the project root:

```bash
norm init --profile convention --output "$NORM_ROOT/docs/.norm" --json --pretty
```

Do not use `--force` unless replacement was explicitly intended and the
existing convention was reviewed. Customize the starter for the real scope and
create its required project artifacts. Then set `NORM_FILE` to the authored
file and run both checks:

```bash
norm parse "$NORM_FILE" --pretty
```

```bash
norm validate "$NORM_FILE" --root "$NORM_ROOT" --strict --json --pretty
```

Require successful versioned responses. Re-collect for a representative nested
target to verify placement and inheritance, not only syntax.

## 6. Diagnose failures

Classify the failure before editing:

- compatibility or executable failure: repair the installation and stop;
- path, containment, or symlink failure: correct the explicit root or target;
- parse or schema failure: repair A1 frontmatter using the field reference;
- semantic or reference failure: repair the named field or project artifact;
- warning under strict mode: resolve it rather than weakening the command.

Use stable codes and fields from machine output. Do not scrape human wording or
raw paths into automation.

## Boundaries

This Skill assists deliberate use of norm-spec. It does not promise automatic
discovery, context injection, mandatory enforcement, tool interception, or
host lifecycle integration. Those capabilities belong to separately versioned
host adapters. The canonical Rust engine remains the only format, collect, and
validation authority.
