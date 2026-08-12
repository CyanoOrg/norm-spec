#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
adoption_root="$(mktemp -d "${TMPDIR:-/tmp}/norm-standalone-adoption.XXXXXX")"

cleanup() {
  case "$adoption_root" in
    "${TMPDIR:-/tmp}"/norm-standalone-adoption.*)
      if [[ -d "$adoption_root" ]] && [[ ! -L "$adoption_root" ]]; then
        rm -rf -- "${adoption_root:?}"
      fi
      ;;
    *)
      echo "refusing to remove unexpected adoption path: $adoption_root" >&2
      ;;
  esac
}
trap cleanup EXIT

remove_adoption_tree() {
  local target="$1"
  case "$target" in
    "$adoption_root"/*) ;;
    *)
      echo "refusing to remove path outside the adoption root: $target" >&2
      exit 1
      ;;
  esac
  if [[ ! -d "$target" ]] || [[ -L "$target" ]]; then
    echo "refusing to remove an absent, non-directory, or linked path: $target" >&2
    exit 1
  fi
  rm -rf -- "${target:?}"
}

assert_contains() {
  local value="$1"
  local expected="$2"
  local label="$3"
  if [[ "$value" != *"$expected"* ]]; then
    echo "$label omitted required text: $expected" >&2
    echo "$value" >&2
    exit 1
  fi
}

cd "$repo_root"
package_target="$adoption_root/package-target"
CARGO_TARGET_DIR="$package_target" cargo package --workspace --allow-dirty

version="$(cargo pkgid -p norm-spec-cli | sed 's/.*@//')"
if [[ -z "$version" ]]; then
  echo "failed to resolve the norm-spec-cli package version" >&2
  exit 1
fi

package_dir="$package_target/package"
package_sources="$adoption_root/package-sources"
mkdir -p "$package_sources"
for package in norm-spec-core norm-spec norm-spec-cli; do
  archive="$package_dir/$package-$version.crate"
  if [[ ! -f "$archive" ]]; then
    echo "packaged candidate archive is unavailable: $archive" >&2
    exit 1
  fi
  tar -xzf "$archive" -C "$package_sources"
done

core_source="$package_sources/norm-spec-core-$version"
facade_source="$package_sources/norm-spec-$version"
cli_source="$package_sources/norm-spec-cli-$version"
for source_dir in "$core_source" "$facade_source" "$cli_source"; do
  if [[ ! -d "$source_dir" ]] || [[ -L "$source_dir" ]]; then
    echo "unpacked package source is unavailable or unsafe: $source_dir" >&2
    exit 1
  fi
done

install_root="$adoption_root/install"
install_target="$adoption_root/install-target"
cargo_facade_source="$facade_source"
cargo_core_source="$core_source"
if command -v cygpath >/dev/null 2>&1; then
  cargo_facade_source="$(cygpath -m "$facade_source")"
  cargo_core_source="$(cygpath -m "$core_source")"
fi
CARGO_TARGET_DIR="$install_target" cargo install \
  --path "$cli_source" \
  --root "$install_root" \
  --locked \
  --config "patch.crates-io.norm-spec.path=\"$cargo_facade_source\"" \
  --config "patch.crates-io.norm-spec-core.path=\"$cargo_core_source\""

exe_suffix=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) exe_suffix=".exe" ;;
esac
candidate="$install_root/bin/norm$exe_suffix"
if [[ ! -x "$candidate" ]]; then
  echo "installed standalone candidate is unavailable: $candidate" >&2
  exit 1
fi

project_root="$adoption_root/project"
mkdir -p \
  "$project_root/development" \
  "$project_root/prd" \
  "$project_root/modules" \
  "$project_root/docs/prd"
touch \
  "$project_root/product-strategy.md" \
  "$project_root/docs/conventions.md" \
  "$project_root/docs/status.md"

version_stdout="$adoption_root/version.stdout"
version_stderr="$adoption_root/version.stderr"
"$candidate" --version >"$version_stdout" 2>"$version_stderr"
if [[ -s "$version_stderr" ]] || [[ "$(<"$version_stdout")" != "norm $version" ]]; then
  echo "standalone version discovery returned the wrong streams or identity" >&2
  exit 1
fi

compatibility="$("$candidate" compatibility)"
assert_contains "$compatibility" '"apiVersion":"norm-spec/compatibility/v1"' "compatibility discovery"
assert_contains "$compatibility" '"id":"norm-spec/rust-api/v1"' "compatibility discovery"
assert_contains "$compatibility" '"suite":"norm-spec/a1-cli/v1"' "compatibility discovery"

initial_scan="$("$candidate" scan --root "$project_root")"
assert_contains "$initial_scan" '"apiVersion":"norm-spec/scan/v1"' "initial scan"
assert_contains "$initial_scan" '"dirs_with_norm":0' "initial scan"

root_init="$("$candidate" init --profile root --output "$project_root/.norm" --json)"
assert_contains "$root_init" '"apiVersion":"norm-spec/init/v1"' "root init"
assert_contains "$root_init" '"profile":"root"' "root init"

nested_init="$("$candidate" init --profile convention --output "$project_root/docs/.norm" --json)"
assert_contains "$nested_init" '"apiVersion":"norm-spec/init/v1"' "nested init"
assert_contains "$nested_init" '"profile":"convention"' "nested init"

collected="$("$candidate" collect --root "$project_root" --target "$project_root/docs")"
assert_contains "$collected" '"apiVersion":"norm-spec/collect/v1"' "nested collect"
case "$collected" in
  *'"path":"docs/.norm"'*'"path":".norm"'*) ;;
  *)
    echo "nested collect did not return most-specific-first conventions" >&2
    echo "$collected" >&2
    exit 1
    ;;
esac

valid="$("$candidate" validate --all --root "$project_root" --strict --json)"
assert_contains "$valid" '"apiVersion":"norm-spec/validate/v1"' "strict validation"
assert_contains "$valid" '"errors":0' "strict validation"
assert_contains "$valid" '"warnings":0' "strict validation"

cp tests/adoption/invalid-reference.norm.invalid "$project_root/docs/.norm"

# The final smoke must not retain the packaged source or its build tree. The
# installed binary and isolated project are the only remaining runtime inputs.
remove_adoption_tree "$package_sources"
remove_adoption_tree "$install_target"

failure_stdout="$adoption_root/failure.stdout"
failure_stderr="$adoption_root/failure.stderr"
set +e
(
  cd "$project_root"
  "$candidate" validate --all --root . --strict --json
) >"$failure_stdout" 2>"$failure_stderr"
failure_status=$?
set -e

if [[ "$failure_status" -ne 1 ]]; then
  echo "invalid standalone input returned exit $failure_status instead of 1" >&2
  exit 1
fi
if [[ -s "$failure_stderr" ]]; then
  echo "invalid standalone machine validation wrote to stderr" >&2
  exit 1
fi
failure="$(<"$failure_stdout")"
assert_contains "$failure" '"apiVersion":"norm-spec/validate/v1"' "invalid validation"
assert_contains "$failure" '"code":"norm/reference/not-found"' "invalid validation"
assert_contains "$failure" '"path":"docs/.norm"' "invalid validation"

echo "Packaged standalone adoption passed without plugin or source checkout."
