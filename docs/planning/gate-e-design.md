# Gate E Distribution and Release-Readiness Design

## Goal

Turn the Gate D product into downloadable, target-specific candidates that can
prove their own contents and behavior after extraction, then perform the
release-quality review needed before a maintainer may authorize `v0.1.0`.

D016 defines the artifact contract, initial targets, MSRV, and publication
boundary. Gate E implementation does not itself create a tag, GitHub Release,
or registry publication.

## Observed baseline entering Gate E

Gate D is complete on `main` at `4555dae`:

- all 82 frozen command cases and 68 Rust test functions are green locally;
- the final Gate D implementation candidate `7e052ae` is maintainer-confirmed
  green on hosted quality, Linux, macOS, and Windows CI;
- core, facade, and CLI package candidates pass in an isolated registry;
- standalone adoption succeeds after its temporary package sources disappear;
- compatibility discovery and arbitrary-candidate conformance are versioned,
  locked, complete, and fail closed;
- the canonical Skill passed both repository tests and one real non-plugin
  OpenCode adoption against pi-norm-spec.

The remaining distribution gaps are concrete:

- CI tests source checkouts but produces no downloadable candidate;
- `macos-latest` does not state an architecture and all current runner labels
  are moving aliases;
- no release manifest binds version, target, source revision, contract, and
  Skill in one artifact;
- no extracted-archive test proves executable permissions, exact contents,
  compatibility, conformance, and standalone use together;
- Rust 1.97 is declared but not yet documented and tested as the supported
  minimum;
- installation describes source/Git development candidates only, and upgrade,
  checksum, archive, and rollback procedures are missing;
- no complete release-quality review has compared scope, documentation,
  behavior, packages, compatibility, and release contents.

## Scope and checkpoints

### Gate E implementation scope

- four native target archives defined by D016;
- a versioned release manifest and checksum per archive;
- repository-owned staging, packaging, and extracted-artifact verification;
- fixed-runner hosted candidate builds with downloadable workflow evidence;
- a distinct MSRV job on Rust 1.97.1;
- binary, source/Git, Skill, and Rust facade installation/upgrade guidance;
- a recorded release-quality review against the exact integrated candidate.

### Explicit maintainer checkpoints

- approving any new release framework or signing dependency;
- changing the product, format, Rust API, machine API, suite, or manifest
  contract after D016;
- changing public repository visibility;
- enabling tag-triggered GitHub Release publication;
- publishing `norm-spec-core`, `norm-spec`, or `norm-spec-cli` to a registry;
- promoting `[Unreleased]`, changing the product version, tagging `v0.1.0`, or
  declaring the stable release complete.

Candidate artifacts uploaded by ordinary CI are temporary review inputs. They
do not satisfy a publication checkpoint and are not described as releases.

## E1 — Artifact contract

### Supported candidates

| Rust target | Native hosted runner | Archive |
|---|---|---|
| `x86_64-unknown-linux-gnu` | `ubuntu-22.04` | `.tar.gz` |
| `aarch64-apple-darwin` | `macos-15` | `.tar.gz` |
| `x86_64-apple-darwin` | `macos-15-intel` | `.tar.gz` |
| `x86_64-pc-windows-msvc` | `windows-2022` | `.tar.gz` |

The same archive format keeps construction and verification identical. The
Windows executables retain `.exe`; Unix executable modes are stored inside the
repository-created tarball before workflow upload.

The Linux artifact is a glibc build produced on Ubuntu 22.04. Gate E does not
claim musl, older-glibc portability, Linux ARM64, Windows ARM64, package-manager
installation, code signing, or notarization. Those are later additive
distribution decisions backed by their own execution evidence.

### Archive name and layout

For version `<version>` and target `<target>`:

```text
norm-spec-<version>-<target>.tar.gz
norm-spec-<version>-<target>.tar.gz.sha256
└── norm-spec-<version>-<target>/
    ├── bin/
    │   ├── norm[.exe]
    │   └── norm-spec-conformance[.exe]
    ├── contract/
    │   └── ... locked norm-spec/a1-cli/v1 bundle ...
    ├── skills/
    │   └── norm-spec/
    │       ├── SKILL.md
    │       └── references/
    │           ├── authoring.md
    │           └── field-reference.md
    ├── LICENSE
    ├── README.md
    ├── README.zh-CN.md
    └── release-manifest.json
```

No source checkout, Cargo build tree, VCS metadata, host metadata, or additional
Skill file may appear. Canonical assets are copied only into the candidate
staging directory; repository sources remain their single sources of truth.

### Manifest

`release-manifest.json` uses `norm-spec/release-artifact/v1` and contains:

- exact product version and Rust target;
- exact 40-character lowercase Git source revision;
- relative executable paths;
- compatibility API identifier;
- contract bundle/report/suite identifiers, case count, and suite digest;
- relative contract and Skill paths.

Keys and arrays are deterministic. Paths use `/` separators. All paths are
relative, contained, and present. Product version does not substitute for any
protocol identifier.

The sibling checksum file contains lowercase SHA-256, two spaces, and the
archive basename. It binds transport bytes; the manifest and contract lock bind
semantic inventory inside those bytes. Signing is not claimed in v0.1.0.

## E2 — Extracted-artifact verification

The verifier accepts an explicit archive, expected target, and expected source
revision. It creates a temporary directory, checks the checksum, extracts
without using the checkout as a runtime source, and fails on an unexpected or
missing inventory item.

It then proves:

1. `norm --version` exactly matches the manifest product version;
2. `norm compatibility` reports `norm-spec/compatibility/v1` and exactly the
   bundled suite identity, count, and digest;
3. `norm-spec-conformance` executes the extracted `norm` against the extracted
   contract and returns pass, complete, 82 executed, zero failed, zero not
   executed;
4. the canonical Skill inventory is exact and every relative reference exists;
5. an unrelated temporary project completes scan, init, nested collect, and
   strict validation using only the extracted `norm`;
6. a missing strong reference returns the stable non-zero
   `norm/reference/not-found` machine failure after staging inputs other than
   the archive have been removed.

Archive verification is mandatory on the same native runner that built it.
Downloading and rechecking all four artifacts in an aggregate job may be added
later, but it is not a substitute for native binary execution.

## E3 — MSRV contract

The initial MSRV is Rust 1.97. Workspace manifests already declare
`rust-version = "1.97"`; the exact verification toolchain is Rust 1.97.1.

The MSRV job runs on `ubuntu-22.04` and must include:

- `cargo check --workspace --all-targets --all-features --locked`;
- `cargo test --workspace --all-features --locked`;
- package-candidate verification;
- standalone adoption from packaged sources.

The regular quality lane may move to a newer compiler later, but the MSRV lane
must remain on 1.97.x until a recorded decision changes the supported floor.
Edition 2024's language minimum is not evidence that this dependency graph and
full product work on that minimum.

## E4 — Hosted candidate workflow

Ordinary push and pull-request CI gains a candidate matrix using the fixed
runners in E1. Each job:

1. checks out the exact workflow revision;
2. installs the pinned Rust 1.97.1 toolchain;
3. builds both release binaries with `--locked`;
4. stages and archives the exact D016 contents;
5. verifies the archive from extraction on the native runner;
6. uploads the already-created `.tar.gz` and `.sha256` files with a
   commit-pinned artifact action.

The workflow upload is intentionally not a GitHub Release. The action must not
re-archive the candidate in a way that loses Unix executable modes. Artifact
names include the exact target and cannot collide within one run.

No `release:` event, tag trigger, `contents: write`, registry token, signing
secret, or publication command is added in the implementation batches.

## E5 — Installation and upgrade contract

Documentation must distinguish four lanes:

- downloaded binary archive for ordinary standalone and Skill use;
- exact Git revision or checkout for development use;
- `cargo install` from a registry only after publication exists;
- Rust facade dependency by exact version or revision.

Binary installation includes checksum verification, archive extraction, PATH
placement, `norm --version`, compatibility inspection, and optional complete
conformance. Upgrade instructions require verifying the new candidate before
replacing the old executable, retaining the previous archive for rollback,
and rechecking downstream exact compatibility requirements. A changed product
version alone never proves compatibility.

The canonical Skill is installed from the same reviewed artifact and keeps its
relative files intact. Installing or upgrading it still does not claim host
injection or enforcement.

## E6 — Release-quality review and completion evidence

Before release authorization, review the exact integrated commit for:

- **Scope:** every Gate E item maps to an implementation or an explicit
  deferred, non-claimed target;
- **Documentation:** README languages, integration/install/upgrade guidance,
  architecture, decisions, roadmap, plan, status, changelog, and CLI help agree;
- **Behavior:** all 82 frozen cases, compatibility fixtures, conformance
  negative paths, and standalone/Skill workflows remain green;
- **Packaging:** core, facade, CLI, four release archives, exact inventory,
  checksums, and extracted verification are green;
- **Compatibility:** product, format, Rust API, machine APIs, conformance suite,
  release manifest, and MSRV are named independently and consistently;
- **History/security:** public-history checks and artifact inventory reveal no
  private path, secret, local metadata, or unintended generated content;
- **Hosted evidence:** quality, MSRV, and all four native candidate jobs are
  green for the same commit.

Local success hands the branch to hosted CI. Hosted success permits merge and
Gate E implementation closure, but it does not authorize stable release.

## Planned semantic commits

1. `docs(decisions): define the Gate E distribution contract`
2. `build(release): assemble versioned candidate archives`
3. `test(release): verify extracted candidates and MSRV`
4. `ci(release): build native candidate artifacts`
5. `docs(install): document release installation and upgrades`
6. `docs(status): record the Gate E candidate`

The final documentation record is made only after its cited local evidence is
current. Hosted results are recorded in a later documentation-only closure
commit after maintainer confirmation.
