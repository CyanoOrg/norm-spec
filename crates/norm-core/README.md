# norm-spec-core

`norm-spec-core` is the deterministic semantic engine for the `.norm` project
convention format. It owns parsing, inheritance rules, Schema/profile and
semantic validation, structural scan aggregation, and versioned response
models.

Most applications should depend on the higher-level
[`norm-spec`](https://crates.io/crates/norm-spec) facade. It adds explicit,
root-contained filesystem collection and validation plus packaged Schema and
template assets. Humans, scripts, and non-Rust consumers normally use the
[`norm-spec-cli`](https://crates.io/crates/norm-spec-cli) package and its
`norm` executable.

The core crate intentionally does not read the filesystem or environment,
start processes, choose terminal output, access the network, or encode process
exit policy. Callers supply observations and resources explicitly. Failures do
not become empty successful rulesets or skipped validation.

The `.norm` specification, compatibility identifiers, public API guide, source,
and contribution policy are maintained in the canonical
[CyanoOrg/norm-spec](https://github.com/CyanoOrg/norm-spec) repository.

License: MIT.
