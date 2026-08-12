# Authoring `.norm` conventions

Use this reference when creating a new convention or changing its scope,
profile, required artifacts, reference policy, or lifecycle rules. The format
authority remains the installed `norm` engine and the product specification;
this guide does not authorize manual parsing or validation fallback.

## 1. Choose placement and scope

Place `.norm` in the directory whose work it governs. Collection walks from the
target directory toward the explicit project root, so a deeper declaration is
more specific while outer declarations still apply.

Set `metadata.scope` to a project-relative description of the intended area.
Keep the body concise: explain why the rules exist or how a human should apply
them; keep machine-enforced structure in YAML frontmatter.

Before writing, collect the parent scope. Preserve compatible inherited rules
and identify any conflict that the new declaration would introduce.

## 2. Choose a profile only when it fits

Profile selection precedence is:

1. an explicit validation `--profile` option;
2. `metadata.profile`;
3. best-effort matching from `metadata.layer`.

Use `metadata.profile` when the human layer name should remain domain-specific
but a known profile supplies useful checks. Omit the profile when none fits;
`metadata.layer` is free-form and the base Schema remains valid.

Embedded `init` templates are starters, not proof that a project already
satisfies their required files, directories, generated values, or references.
Review and customize every generated value before treating the convention as
adopted.

## 3. Write the A1 document

An A1 file contains exactly:

1. optional UTF-8 BOM or leading blank lines;
2. an opening `---` line;
3. one YAML mapping;
4. a closing `---` line;
5. an optional Markdown body.

Always provide `metadata.layer`, `metadata.scope`, and
`metadata.version`. Use a quoted version such as `"1.0"` so YAML does not
coerce it to a number. Add a description when it clarifies the governed area.

Select only fields that express real project rules. Avoid copying a large
profile example whose files, lifecycle, or references do not exist.

## 4. Declare verifiable project structure

For `template.required_files` and `template.required_directories`, use paths
relative to the directory containing `.norm`. Create those artifacts or remove
the declarations; do not leave aspirational requirements in a supposedly valid
project.

For each single source of truth, name one authoritative source and a distinct
domain. Do not declare duplicate ownership for the same domain.

Keep `agent_rules.update_order` operational and ordered. Lifecycle states and
transitions must use unique identifiers, resolve every source and target, and
leave terminal states without outgoing transitions. Free-text `requires`
entries are gates to verify, not evidence that a transition happened.

## 5. Resolve references from the correct base

Local targets beginning with `/` resolve from the explicit project root.
Relative targets resolve from the directory containing the `.norm`. All local
targets must stay inside the project root.

When `target_must_exist` is true, create the target before validation. When
`require_description` is true, describe why each strong reference exists.
External targets remain invalid unless the declaration explicitly allows them.

## 6. Verify the authored result

Use the canonical workflow in `SKILL.md`:

1. parse the exact file;
2. strictly validate it against the explicit project root;
3. collect a representative target beneath it;
4. inspect the most-specific-first order and every applicable convention;
5. repair all errors and strict warnings before completion.

If the engine is unavailable or any command fails, stop and repair the engine
or input. Do not substitute a YAML library, a manual parent walk, or visual
inspection as a passing result.
