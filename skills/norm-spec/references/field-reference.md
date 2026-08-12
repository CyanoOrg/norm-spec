# `.norm` field reference

Use this concise map while authoring or diagnosing A1 declarations. Consult the
versioned engine response for actual validity; this reference summarizes intent
and does not replace Schema or semantic validation.

## Required metadata

| Field | Meaning |
|---|---|
| `metadata.layer` | Free-form human label for the governed directory role. |
| `metadata.scope` | Project-relative description of the governed area. |
| `metadata.version` | Convention-document version string, commonly `"1.0"`. |
| `metadata.profile` | Optional explicit known profile; takes precedence over layer matching. |
| `metadata.description` | Optional short explanation of the convention. |

Known profiles are `root`, `module`, `epic`, `architecture`, `task`, `test`,
and `convention`. Profiles add recommended checks; an unknown free-form layer
still receives base Schema and semantic validation.

## Template fields

| Field | Meaning |
|---|---|
| `required_files` | Files that must exist beside the declaration. |
| `optional_files` | Documented files that are allowed but not required. |
| `required_directories` | Directories that must exist beside the declaration. |
| `file_structure` | Required human structure for governed documents. |
| `naming_convention` | Human naming rules to obey during work. |
| `scope_definition` | Human description of how work scope is recorded. |
| `single_source_of_truth` | Ordered `source` and `for` ownership pairs. |

Required path checks are rooted at the directory containing the `.norm`, not
at the process working directory.

## Agent rules

| Field | Meaning |
|---|---|
| `update_order` | Ordered steps for keeping sources and projections aligned. |
| `archiving` | Project-specific archive format and date-source rules. |
| `module_version_sync` | Version synchronization obligations. |
| `document_lifecycle` | Structured states, initial state, terminals, transitions, and gates. |
| `reference_policy` | Existence, external-target, and description requirements. |
| `test_naming` | Test filename or identifier convention. |
| `no_test_results` | Whether generated test-result artifacts are forbidden here. |

For a structured lifecycle, state IDs must be unique; `initial`, `terminal`,
and transition endpoints must resolve; transitions must be unique; terminal
states cannot have outgoing transitions. A transition `requires` list defines
preconditions but does not record current state or prove completion.

## Scope boundaries

`scope.covers` lists responsibilities owned by the directory. `scope.NOT`
names explicit exclusions and should point to their actual owners when useful.
`scope.subdirectories` may explain delegation below the current directory.

These fields express human responsibility boundaries. Validation can check
their shape, not whether every operation honored them.

## References

`cross_references` describe adjacent boundaries. `strong_references` add
explicit type, synchronization direction, validation strength, and a human
description.

Reference targets resolve as follows:

- `/path` starts at the explicit project root;
- `relative/path` starts beside the declaring `.norm`;
- local targets may not escape the project root;
- URI-like external targets require `allow_external: true`;
- `target_must_exist: true` makes a missing local target an error;
- `require_description: true` requires strong-reference descriptions.

Duplicate reference targets and duplicate single-source domains are semantic
errors even when the YAML and Schema shape are otherwise valid.

## Stable diagnostic families

Use the exact machine-emitted code and `field` to guide repair. Common families
include:

- `norm/parse/*` for A1 fences, YAML, or document-shape failures;
- `norm/schema/*` for base or profile Schema failures;
- `norm/profile/*` for profile recommendations and selection;
- `norm/reference/*` for target, containment, duplicate, or description issues;
- `norm/lifecycle/*` for invalid state-machine structure;
- `norm/path/*` for unavailable, unreadable, outside-root, or symlink inputs.

Do not branch automation on mutable human messages. A missing or unknown code
does not authorize fallback; retain the output and repair or upgrade the
compatible engine.
