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

It checks facade package contents, verifies core, facade, and CLI together via
Cargo's temporary workspace registry, and builds an unrelated consumer from
the exact current Git revision. Compatibility discovery and arbitrary-binary
conformance are separate Gate D2/D3 results and are not implied by this API.
