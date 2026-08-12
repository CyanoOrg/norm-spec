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

package_list="$candidate_root/norm-spec-package.txt"
cargo package --list --allow-dirty -p norm-spec >"$package_list"

required_assets=(
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
  if ! grep -Fqx "$asset" "$package_list"; then
    echo "norm-spec package omitted required asset: $asset" >&2
    exit 1
  fi
done

if grep -Fq "tests/contract/" "$package_list"; then
  echo "norm-spec facade package unexpectedly contains the CLI contract bundle" >&2
  exit 1
fi

cli_package_list="$candidate_root/norm-spec-cli-package.txt"
cargo package --list --allow-dirty -p norm-spec-cli >"$cli_package_list"
for cli_asset in \
  "src/main.rs" \
  "src/bin/norm-spec-conformance.rs" \
  "src/bin/conformance/bundle.rs" \
  "src/bin/conformance/model.rs" \
  "src/bin/conformance/mod.rs" \
  "src/bin/conformance/suite.rs"; do
  if ! grep -Fqx "$cli_asset" "$cli_package_list"; then
    echo "norm-spec-cli package omitted required runner source: $cli_asset" >&2
    exit 1
  fi
done

package_target="$candidate_root/package-target"
CARGO_TARGET_DIR="$package_target" cargo package --workspace --allow-dirty

head_revision="$(git rev-parse HEAD)"
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
