#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
candidate_root="$(mktemp -d "${TMPDIR:-/tmp}/norm-spec-external.XXXXXX")"

cleanup() {
  case "$candidate_root" in
    "${TMPDIR:-/tmp}"/norm-spec-external.*)
      rm -rf -- "${candidate_root:?}"
      ;;
    *)
      echo "refusing to remove unexpected candidate path: $candidate_root" >&2
      ;;
  esac
}
trap cleanup EXIT

cd "$repo_root"

require_package_entry() {
  local package_name="$1"
  local package_list="$2"
  local entry="$3"

  if ! grep -Fqx "$entry" "$package_list"; then
    echo "$package_name package omitted required entry: $entry" >&2
    exit 1
  fi
}

require_manifest_line() {
  local package_name="$1"
  local manifest="$2"
  local line="$3"

  if ! grep -Fqx "$line" "$manifest"; then
    echo "$package_name normalized manifest omitted required metadata: $line" >&2
    exit 1
  fi
}

package_list="$candidate_root/norm-spec-package.txt"
cargo package --list --allow-dirty -p norm-spec >"$package_list"

required_assets=(
  "LICENSE"
  "README.md"
  "README.zh-CN.md"
  "crates/norm-api/src/lib.rs"
  "schema/norm-schema.json"
  "schema/profiles/architecture.json"
  "schema/profiles/convention.json"
  "schema/profiles/epic.json"
  "schema/profiles/module.json"
  "schema/profiles/root.json"
  "schema/profiles/task.json"
  "schema/profiles/test.json"
  "templates/profiles/architecture.norm"
  "templates/profiles/convention.norm"
  "templates/profiles/epic.norm"
  "templates/profiles/module.norm"
  "templates/profiles/root.norm"
  "templates/profiles/task.norm"
  "templates/profiles/test.norm"
)
for asset in "${required_assets[@]}"; do
  require_package_entry "norm-spec" "$package_list" "$asset"
done

if grep -Fq "tests/contract/" "$package_list"; then
  echo "norm-spec facade package unexpectedly contains the CLI contract bundle" >&2
  exit 1
fi

core_package_list="$candidate_root/norm-spec-core-package.txt"
cargo package --list --allow-dirty -p norm-spec-core >"$core_package_list"
for core_asset in \
  "LICENSE" \
  "README.md" \
  "src/lib.rs" \
  "src/protocol.rs"; do
  require_package_entry "norm-spec-core" "$core_package_list" "$core_asset"
done

cli_package_list="$candidate_root/norm-spec-cli-package.txt"
cargo package --list --allow-dirty -p norm-spec-cli >"$cli_package_list"
for cli_asset in \
  "LICENSE" \
  "README.md" \
  "src/main.rs" \
  "src/bin/norm-spec-conformance.rs" \
  "src/bin/conformance/bundle.rs" \
  "src/bin/conformance/model.rs" \
  "src/bin/conformance/mod.rs" \
  "src/bin/conformance/suite.rs"; do
  require_package_entry "norm-spec-cli" "$cli_package_list" "$cli_asset"
done

package_target="$candidate_root/package-target"
CARGO_TARGET_DIR="$package_target" cargo package --workspace --allow-dirty

version="$(cargo pkgid -p norm-spec | sed 's/.*#//' | sed 's/.*@//')"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([+-][0-9A-Za-z.-]+)?$ ]]; then
  echo "failed to resolve a safe workspace package version: $version" >&2
  exit 1
fi

core_package="$package_target/package/norm-spec-core-$version"
facade_package="$package_target/package/norm-spec-$version"
cli_package="$package_target/package/norm-spec-cli-$version"
for package_dir in "$core_package" "$facade_package" "$cli_package"; do
  if [[ ! -d "$package_dir" ]]; then
    echo "packaged source directory is unavailable: $package_dir" >&2
    exit 1
  fi
done

for package_dir in "$core_package" "$facade_package" "$cli_package"; do
  package_name="$(basename "$package_dir" "-$version")"
  manifest="$package_dir/Cargo.toml"
  require_manifest_line "$package_name" "$manifest" 'rust-version = "1.97"'
  require_manifest_line "$package_name" "$manifest" 'publish = ["crates-io"]'
  require_manifest_line "$package_name" "$manifest" 'readme = "README.md"'
  require_manifest_line "$package_name" "$manifest" 'license = "MIT"'
  require_manifest_line \
    "$package_name" \
    "$manifest" \
    'repository = "https://github.com/CyanoOrg/norm-spec"'
  for keyword in ai-agents conventions configuration validation; do
    require_manifest_line "$package_name" "$manifest" "    \"$keyword\","
  done
  require_manifest_line "$package_name" "$manifest" '    "config",'
  require_manifest_line "$package_name" "$manifest" '    "development-tools",'
done

require_manifest_line \
  "norm-spec-core" \
  "$core_package/Cargo.toml" \
  'description = "Canonical parser, collection, and validation engine for .norm files"'
require_manifest_line \
  "norm-spec-core" \
  "$core_package/Cargo.toml" \
  'documentation = "https://docs.rs/norm-spec-core"'
require_manifest_line \
  "norm-spec-core" \
  "$core_package/Cargo.toml" \
  '    "parser-implementations",'

require_manifest_line \
  "norm-spec" \
  "$facade_package/Cargo.toml" \
  'description = "High-level Rust API for collecting and validating .norm files"'
require_manifest_line \
  "norm-spec" \
  "$facade_package/Cargo.toml" \
  'documentation = "https://docs.rs/norm-spec"'
require_manifest_line \
  "norm-spec" \
  "$facade_package/Cargo.toml" \
  '    "parser-implementations",'

require_manifest_line \
  "norm-spec-cli" \
  "$cli_package/Cargo.toml" \
  'description = "Portable command-line interface for norm-spec"'
require_manifest_line \
  "norm-spec-cli" \
  "$cli_package/Cargo.toml" \
  'documentation = "https://docs.rs/norm-spec-cli"'
require_manifest_line "norm-spec-cli" "$cli_package/Cargo.toml" '    "cli",'
require_manifest_line \
  "norm-spec-cli" \
  "$cli_package/Cargo.toml" \
  '    "command-line-utilities",'

cmp "$repo_root/LICENSE" "$core_package/LICENSE"
cmp "$repo_root/LICENSE" "$facade_package/LICENSE"
cmp "$repo_root/LICENSE" "$cli_package/LICENSE"

head_revision="$(git rev-parse HEAD)"
for package_dir in "$core_package" "$facade_package" "$cli_package"; do
  package_name="$(basename "$package_dir" "-$version")"
  vcs_info="$package_dir/.cargo_vcs_info.json"
  require_manifest_line "$package_name" "$vcs_info" "    \"sha1\": \"$head_revision\","
done
require_manifest_line \
  "norm-spec-core" \
  "$core_package/.cargo_vcs_info.json" \
  '  "path_in_vcs": "crates/norm-core"'
require_manifest_line \
  "norm-spec" \
  "$facade_package/.cargo_vcs_info.json" \
  '  "path_in_vcs": ""'
require_manifest_line \
  "norm-spec-cli" \
  "$cli_package/.cargo_vcs_info.json" \
  '  "path_in_vcs": "crates/norm-cli"'

if [[ "${NORM_REQUIRE_CLEAN_PACKAGES:-0}" == "1" ]]; then
  for package_dir in "$core_package" "$facade_package" "$cli_package"; do
    package_name="$(basename "$package_dir" "-$version")"
    require_manifest_line \
      "$package_name" \
      "$package_dir/.cargo_vcs_info.json" \
      '    "dirty": false'
  done
fi

git_url="file://$repo_root"
escaped_git_url="$(printf '%s' "$git_url" | sed 's/[&|]/\\&/g')"
sed \
  -e "s|__NORM_SPEC_GIT_URL__|$escaped_git_url|g" \
  -e "s|__NORM_SPEC_GIT_REV__|$head_revision|g" \
  tests/external-consumer/Cargo.toml.template >"$candidate_root/Cargo.toml"
cp -R tests/external-consumer/src "$candidate_root/src"
cp -R tests/external-consumer/project "$candidate_root/project"

cargo run --manifest-path "$candidate_root/Cargo.toml"

echo "Package verification and exact-revision external consumption passed."
