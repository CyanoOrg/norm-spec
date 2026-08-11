# Behavior contract

This directory is the language-neutral compatibility surface for norm-spec.
Inputs are `.norm` files or filesystem layouts; expected results describe
canonical data, error codes, ordering, and exit status.

Rules:

- A fixture change is a contract change and must be reviewed as such.
- Failure to execute an applicable implementation is a test failure, not a skip.
- Absolute temporary paths are normalized before comparison.
- Ordered collections remain ordered; JSON object-key order is ignored.
- Human wording may improve, but stable error codes and field paths may not
  change without a protocol decision.

The initial fixtures establish the A1 baseline. Gate B freezes every documented
CLI behavior before parser implementation begins.

`requirements.tsv` is the obligation inventory. Every requirement ID must be
named by at least one `manifest.tsv` case. The manifest has eleven columns:

1. `case_id`
2. `command`
3. `args_json`
4. `fixture`
5. `stdout`
6. `stdout_match`
7. `stderr`
8. `stderr_match`
9. `api_version`
10. `exit_code`
11. comma-separated `covers` requirement IDs

Argument lists are JSON arrays of strings. Portable placeholders are
`{fixture}`, `{root}`, `{outside}`, `{schema}`, `{output}`, `{missing}`, and
`{version}`. Use `-` when an asset or protocol does not apply; an absent stream
must use the `empty` match mode.

JSON success fixtures use `json-exact`: object-key order is ignored but array
order and values are exact. `json-subset` is for diagnostic results: objects
are recursive subsets and arrays are ordered subsequences, allowing additional
diagnostics without weakening the required code, field, or suggestion.
Human-output `contains` fixtures are ordered required lines; `template` also
expands the portable placeholders. `exact` is reserved for byte-for-byte human
output.

Layout fixtures under `layouts/` are three-column TSV recipes with instructions
`dir`, `file`, `copy`, `outside-dir`, `symlink-dir`, and `symlink-file`.
Runners materialize them in isolated roots; `outside-dir` creates a sibling
outside the root and binds `{outside}` to it. A single-file fixture is copied to
an isolated `.norm`, so expected paths never expose the repository checkout.

The Rust Gate B integrity test rejects missing or duplicate requirements/cases,
unknown placeholders and match modes, invalid layout instructions, absent
assets, protocol/command/exit inconsistencies, and incomplete success, error,
or human-mode coverage. Gate C replaces presence checks with executable command
assertions one vertical slice at a time.

Intentionally malformed inputs use a suffix such as `.norm.invalid` so
repository-wide dogfood validation does not mistake them for live conventions;
contract runners materialize the `.norm` filename in an isolated fixture root.
