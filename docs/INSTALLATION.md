# Installation and Upgrade Guide

norm-spec supports standalone binaries, source/Git development installs, a
Rust facade, and the canonical framework-neutral Skill. All lanes use the same
Rust semantic engine. An agent plugin is not required.

Release candidate `0.1.0-rc.1` is public and validated from exact commit
`5c781964`. Its three Rust packages are on crates.io, and four native archives
with sibling checksums are attached to the GitHub Pre-release. CI candidate
artifacts remain temporary review evidence and must not be confused with those
release assets. The reviewed RC scope and compatibility identities are recorded
in `docs/releases/v0.1.0-rc.1.md`.

## Install from crates.io

Install both CLI executables with the exact reviewed version:

```bash
cargo install norm-spec-cli --version '=0.1.0-rc.1' --locked
norm --version
norm compatibility --pretty
```

This source compiles locally and therefore requires Rust 1.97 or newer. Use a
native release archive when an end user should not need a Rust toolchain.

## Choose a release archive

The initial archive set is native and architecture-specific:

| System | Rust target | Archive suffix |
|---|---|---|
| Linux x86-64 with glibc | `x86_64-unknown-linux-gnu` | `.tar.gz` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `.tar.gz` |
| macOS Intel | `x86_64-apple-darwin` | `.tar.gz` |
| Windows x86-64 | `x86_64-pc-windows-msvc` | `.tar.gz` |

The Linux binary is built on Ubuntu 22.04. This release line does not yet claim
musl, older-glibc, Linux ARM64, Windows ARM64, code signing, notarization, or a
package-manager formula.

An archive is named:

```text
norm-spec-<version>-<rust-target>.tar.gz
```

Download both the archive and its sibling `.sha256` file. A GitHub Actions
candidate expires and requires repository access; it is not a durable install
source. A GitHub Release asset is the durable source after release approval.

## Verify before extraction

Keep the archive and checksum in the same directory. On Linux:

```bash
sha256sum -c norm-spec-<version>-<rust-target>.tar.gz.sha256
```

On macOS:

```bash
shasum -a 256 -c norm-spec-<version>-<rust-target>.tar.gz.sha256
```

On Windows PowerShell:

```powershell
$Archive = "norm-spec-<version>-x86_64-pc-windows-msvc.tar.gz"
$Expected = (Get-Content "$Archive.sha256").Split()[0].ToLowerInvariant()
$Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
if ($Actual -ne $Expected) { throw "norm-spec checksum mismatch" }
```

A checksum detects changed transport bytes; it is not a publisher signature.
Compare the checksum with the value on the reviewed release record, not with an
unrelated mirror or message.

Maintainers and CI can run the repository verifier when a matching checkout is
available:

```bash
scripts/check-release-archive.sh \
  norm-spec-<version>-<rust-target>.tar.gz \
  <rust-target> \
  <exact-40-character-source-revision>
```

This gate checks the checksum, safe paths, exact inventory, release manifest,
source revision, compatibility identity, all 82 conformance cases, and a
source-free standalone adoption.

## Inspect and run the extracted candidate

Extract without flattening the versioned top-level directory:

```bash
tar -xzf norm-spec-<version>-<rust-target>.tar.gz
cd norm-spec-<version>-<rust-target>
```

The archive contains:

```text
bin/norm[.exe]
bin/norm-spec-conformance[.exe]
contract/
skills/norm-spec/
release-manifest.json
LICENSE
README.md
README.zh-CN.md
```

Before copying anything into `PATH`, run it in place:

```bash
./bin/norm --version
./bin/norm compatibility --pretty
./bin/norm-spec-conformance \
  --candidate ./bin/norm \
  --contract-dir ./contract \
  --pretty
```

On Windows, use `bin/norm.exe` and `bin/norm-spec-conformance.exe`. Require a
successful `norm-spec/compatibility/v1` response and a complete passing
`norm-spec/conformance/v1` report with 82 executed cases. Product SemVer does
not substitute for format, Rust API, machine API, suite, or digest identity.

After verification, copy both executables to a directory already on `PATH`, or
add this versioned `bin/` directory to `PATH`. Keep the original archive and
checksum until the upgrade has been exercised against real projects.

## Install the canonical Skill

Copy the entire extracted `skills/norm-spec/` directory into a location
supported by the chosen agent host. Preserve `SKILL.md` and both files under
`references/` together. Make the matching verified `norm` executable available
to the host process.

The Skill calls the engine and fails closed when it is missing or incompatible.
Installing the Skill does not prove automatic session injection, path-aware
tool interception, or enforcement; those are downstream host-adapter claims.

## Install a development candidate from source

Use an exact reviewed checkout:

```bash
cargo install --path crates/norm-cli --locked
```

Or pin the canonical Git repository to one exact revision:

```bash
cargo install --git https://github.com/CyanoOrg/norm-spec \
  --rev <exact-commit> \
  --package norm-spec-cli \
  --locked
```

Both commands install `norm` and `norm-spec-conformance`. Source installation
requires Rust 1.97 or newer; Rust 1.97.1 is the exact initial MSRV verification
toolchain. Do not install a moving branch for production use.

## Use the Rust facade

Use the exact reviewed public facade release:

```toml
[dependencies]
norm-spec = "=0.1.0-rc.1"
```

For source-development work, an exact Git revision remains available as an
alternative. Do not depend on a moving branch. `docs/RUST-API.md` defines the
request, response, failure, asset, and compatibility boundary.

## Upgrade safely

Treat an upgrade as a new candidate, not an overwrite:

1. download the new target-specific archive and checksum beside the old one;
2. verify its checksum and exact source/release record;
3. extract into a new versioned directory;
4. run version, compatibility, and complete conformance in place;
5. compare every exact identifier and digest required by scripts, Skills, Rust
   consumers, and downstream adapters;
6. run `norm validate --all --root <project> --strict --json --pretty` against
   representative projects before changing `PATH`;
7. switch `PATH` or replace binaries only after those checks pass;
8. retain the previous archive, checksum, and configuration for rollback.

If a required identifier changes, stop and follow that release's migration
notes. A higher product version is not evidence that an existing adapter is
compatible. If validation or consumer smoke fails, restore the previous
versioned `bin/` directory and investigate before retrying.

When upgrading the Skill, replace its whole directory from the same reviewed
archive as the engine. Do not mix a new `SKILL.md` with old reference files.

## Remove an installation

Remove only the exact `norm` and `norm-spec-conformance` files or versioned
directory that you installed. Remove the canonical Skill directory separately
from the host's supported Skill location. Project `.norm` files are project
data and are never removed as part of uninstalling the tool.
