# norm-spec-cli

`norm-spec-cli` installs two portable executables:

- `norm` parses, collects, validates, initializes, scans, and reports exact
  compatibility for `.norm` project conventions;
- `norm-spec-conformance` verifies an explicit `norm` candidate against an
  exact locked A1 contract bundle without sibling, network, or registry
  fallback.

The package uses the same canonical Rust engine as the public
[`norm-spec`](https://crates.io/crates/norm-spec) facade. It does not require an
agent plugin. A framework-neutral canonical Skill is distributed in the
target-specific release archives rather than embedded as hidden host metadata.

After registry publication, install one exact reviewed version with:

```bash
cargo install norm-spec-cli --version '=<exact-release-version>' --locked
norm --version
norm compatibility --pretty
```

The conformance runner requires an explicit contract bundle. Target-specific
GitHub Release archives contain both binaries, the matching locked bundle, the
canonical Skill, a versioned release manifest, and a SHA-256 checksum.

Installation, checksum, compatibility, upgrade, and rollback guidance is in
the canonical [CyanoOrg/norm-spec](https://github.com/CyanoOrg/norm-spec)
repository.

License: MIT.
