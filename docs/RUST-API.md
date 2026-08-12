# Rust Consumer API

## Boundary

The `norm-spec` package is the public, filesystem-aware Rust facade. It owns:

- explicit root and target orchestration;
- root containment and `.norm` symbolic-link handling;
- packaged root and profile Schemas;
- packaged authoring templates;
- completed collect and validation response models;
- typed failures when an operation cannot complete.

`norm-spec-core` remains the deterministic semantic implementation. Consumers
normally depend on `norm-spec`; they do not copy core modules, Schema files, or
CLI adapters. `norm-spec-cli` uses the same facade and only adds arguments,
presentation, JSON serialization, and process exit codes.

## Dependency

Gate D validates exact-revision Git consumption before registry publication:

```toml
[dependencies]
norm-spec = { git = "https://github.com/CyanoOrg/norm-spec", rev = "<exact-commit>" }
```

Do not use a sibling `path = "../norm-spec"` dependency as integration
evidence. crates.io publication and its core → facade → CLI order are Gate E
checkpoints; the current package names are not a publication claim.

After `0.1.0-rc.1` actually resolves on crates.io, use the exact reviewed RC:

```toml
[dependencies]
norm-spec = "=0.1.0-rc.1"
```

Do not switch from the Git revision merely because a package name or one
upstream crate is visible. The facade version itself must resolve, and the
consumer must compare the compatibility identities it requires.

## Collect

```rust
use std::path::Path;
use norm_spec::{CollectRequest, collect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = collect(CollectRequest::new(
        Path::new("project"),
        Path::new("docs/module"),
    ))?;
    for convention in response.norms {
        println!("{}", convention.path);
    }
    Ok(())
}
```

A relative collect target is resolved from the explicit root. Results preserve
most-specific-first inheritance. Missing, outside-root, unreadable, malformed,
or symbolic-link convention inputs return `ApiError`; they never return an
empty successful ruleset.

## Validate

```rust
use std::path::Path;
use norm_spec::{ValidateRequest, validate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new("project");
    let one = validate(ValidateRequest::path(root, Path::new("docs/.norm")))?;
    let all = validate(ValidateRequest::all(root))?;
    assert_eq!(one.summary.files, 1);
    assert!(all.summary.files >= one.summary.files);
    Ok(())
}
```

Relative validation inputs and explicit Schema directories are resolved from
the validation root. Builders opt into legacy input, unknown-key compatibility,
an explicit profile, or an explicit Schema directory. Strict-warning policy is
not part of evaluation; callers decide whether completed warnings are fatal.

Validation diagnostics are successful `ValidateResponse` values, even when the
response contains errors. `ApiError` is reserved for failures that prevent
evaluation. `FailureClass::Usage` identifies invalid containment or Schema
configuration; `FailureClass::Operation` identifies unavailable paths, I/O,
or parsing failures. Neither classification embeds a process exit code.

## Release assets

`embedded_schema_bundle`, `schema_bundle_from_dir`, `profile_template`, and
`PROFILE_NAMES` expose the same release-owned assets used by the CLI. Explicit
Schema loading remains offline and fails if required resources are unavailable.

The permanent package gate runs:

```bash
bash scripts/check-packages.sh
```

It checks facade and runner package contents, verifies core, facade, and CLI
together via an isolated temporary workspace registry, and builds an unrelated
consumer from the exact current Git revision.

## Machine compatibility and conformance

Rust API availability does not imply CLI compatibility. An installed candidate
reports exact machine identity without help-text scraping:

```bash
norm compatibility --pretty
```

The `norm-spec/compatibility/v1` response names the accepted format, public
Rust surface, machine APIs, bundle/report protocols, frozen suite count, and
exact contract digest. Consumers compare required identifiers directly;
product SemVer is identity data, not a protocol substitute.

The CLI package also installs an independent runner:

```bash
norm-spec-conformance \
  --candidate /exact/path/to/norm \
  --contract-dir /exact/path/to/exported-bundle \
  --pretty
```

The bundle must match `norm-spec/contract-bundle/v1` byte identities. Missing,
altered, extra, symbolic-link, or unsafe-path content fails before execution.
An executable but incompatible candidate still runs all 82 cases; mismatches
produce a complete failure report, while unavailable execution evidence
produces an explicit incomplete error. There is no sibling, network, registry,
help-text, copied-Schema, empty-success, or skip fallback.

Repository maintainers regenerate, export, and verify the canonical bundle
with:

```bash
bash scripts/update-contract-lock.sh
bash scripts/export-contract-bundle.sh path/to/empty-destination
bash scripts/check-contract-bundle.sh
```
