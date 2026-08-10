# A1 baseline

The initial public compatibility baseline is the A1 `.norm` format documented
in this repository. Its authority is self-contained:

- `docs/SPEC.md` defines syntax and semantics;
- `schema/` defines machine-readable structural rules;
- `templates/profiles/` provides conforming examples;
- `tests/contract/` captures stable inputs, outputs, errors, ordering, and exit
  behavior.

The bootstrap fixtures seed that contract. Gate B expands coverage across the
complete initial CLI surface before parser and validator implementation lands.
Required fixtures and applicable binaries must execute successfully; missing
inputs are failures rather than skips.
