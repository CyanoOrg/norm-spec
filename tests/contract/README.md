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

The initial fixtures establish the A1 baseline. Gate B expands them to every
documented CLI behavior before parser implementation begins.

`manifest.tsv` is the language-neutral inventory. Each non-comment row binds a
case ID, command, fixture (or `-` when no input fixture is needed), expected
result, machine protocol, and exit code. The Rust Gate B integrity test checks
that these paths exist, case IDs are unique, and every initial protocol has at
least one expected result. Command execution assertions replace these static
checks one vertical slice at a time during Gate C.

Intentionally malformed inputs use a suffix such as `.norm.invalid` so
repository-wide dogfood validation does not mistake them for live conventions;
contract runners materialize the `.norm` filename in an isolated fixture root.
