# Integration Guide

norm-spec is useful without an agent plugin. The standalone `norm` CLI is the
canonical executable path for discovering compatibility, scanning an unrelated
project, creating conventions, collecting inheritance, and validating results.
The optional canonical Skill adds an assisted agent workflow while continuing
to call the same engine.

## Integration lanes

| Lane | Use it for | Semantic authority |
|---|---|---|
| Standalone CLI | Humans, scripts, CI, and plugin-free project adoption | `norm` machine protocols |
| Rust facade | In-process Rust collection and validation | `norm-spec/rust-api/v1` |
| Canonical Skill | Assisted collect, obey, validate, author, and diagnosis | Compatible `norm` candidate |
| Host adapter | Automatic lifecycle injection, presentation, or enforcement | Downstream adapter plus pinned norm-spec contract |

The Skill and host adapters must not reimplement YAML parsing, inheritance
collection, Schema selection, or semantic validation. A missing or incompatible
engine is an explicit failure, not an empty ruleset or successful skip.

## Install a development candidate

Until the first public release and registry publication, install from an exact
Git revision or an explicitly reviewed checkout. Do not use an unpinned moving
branch as a production dependency.

For target-specific release archives, checksum and conformance verification,
the canonical Skill, MSRV, upgrades, rollback, and uninstall, see
`docs/INSTALLATION.md`.

From an exact checkout:

```bash
cargo install --path crates/norm-cli --locked
```

From Git, replace the revision with the reviewed commit:

```bash
cargo install --git https://github.com/CyanoOrg/norm-spec \
  --rev <exact-commit> \
  --package norm-spec-cli \
  --locked
```

Verify the installed candidate before using it:

```bash
norm --version
norm compatibility --pretty
```

Require a successful `norm-spec/compatibility/v1` response. Compare exact
format, machine API, Rust API, suite, case count, and contract digest fields
needed by the integration; do not infer compatibility from product version or
help output.

## Adopt an unrelated project without a plugin

Set `PROJECT_ROOT` to the project being adopted. Start by observing structure;
scan does not infer conventions:

```bash
norm scan --root "$PROJECT_ROOT" --text
```

Initialize a suitable embedded starter. The root profile models a documentation
root; choose a different profile or author a base A1 declaration when that
model does not fit:

```bash
norm init --profile root --output "$PROJECT_ROOT/.norm" --json --pretty
```

An initialized template is not automatically valid for the project. Review its
scope, remove irrelevant declarations, replace generated values, and create
every required file, directory, and reference target.

Add a more specific convention where a subtree has distinct rules. For example:

```bash
norm init \
  --profile convention \
  --output "$PROJECT_ROOT/docs/.norm" \
  --json \
  --pretty
```

Collect for the exact work target and inspect every most-specific-first entry:

```bash
norm collect \
  --root "$PROJECT_ROOT" \
  --target "$PROJECT_ROOT/docs" \
  --pretty
```

Complete adoption only after strict project validation succeeds:

```bash
norm validate --all --root "$PROJECT_ROOT" --strict --json --pretty
```

Machine-mode failures use versioned JSON on stdout and a non-zero exit. Keep
stderr empty as the protocol declares. Automation must assert the exit, API
version, stable diagnostic code, and relevant field or path; mutable human
messages are not compatibility fields.

The permanent repository lane `scripts/check-standalone-adoption.sh` performs
this lifecycle from packaged `.crate` candidates. It removes the temporary
package sources and build tree before the final failure smoke, so a source or
sibling checkout cannot silently satisfy runtime asset discovery.

## Use the canonical Skill

The framework-neutral Skill is the self-contained directory
`skills/norm-spec/`:

```text
skills/norm-spec/
├── SKILL.md
└── references/
    ├── authoring.md
    └── field-reference.md
```

Copy that entire directory into a host-supported Skill location, preserving the
relative references. Follow the host's own installation and discovery guidance;
this repository intentionally does not ship host UI metadata or marketplace
configuration. A downstream Plugin may add thin host-specific metadata without
changing the canonical workflow.

Before asking an agent to use the Skill, make a compatible `norm` executable
available in its execution environment. The Skill will:

1. verify `norm-spec/compatibility/v1`;
2. collect for the explicit root and target;
3. translate all applicable declarations into a working checklist;
4. strictly validate relevant results;
5. use engine-backed authoring and diagnostic commands.

The Skill is assisted guidance, not automatic enforcement. Installing it does
not prove session injection, path-aware tool interception, or unbypassable
policy. Those claims require a separately tested host adapter.

## Maintainer verification

Run the product-local Skill and standalone gates together with the normal Rust
quality suite:

```bash
python3 /path/to/skill-creator/scripts/quick_validate.py skills/norm-spec
cargo test -p norm-spec-cli --test skill_contract
cargo test -p norm-spec-cli --test skill_workflow
bash scripts/check-standalone-adoption.sh
```

The repository tests require exactly the three canonical Skill files, reject
host metadata and local paths, install the Skill directory in isolation,
execute every documented Skill CLI example, and exercise nested collection,
authoring, invalid-reference repair, and missing-CLI failure behavior.

## Downstream adapter boundary

Automatic injection and enforcement stay outside this repository. A downstream
adapter must pin an exact upstream revision, tag, or versioned contract; verify
compatibility; preserve fail-closed behavior; and test its own event lifecycle,
path resolution, permissions, tool coverage, and presentation. It must not copy
the parser, schemas, validator, or contract fixtures into a new authority.
