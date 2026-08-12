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

# The full binary smoke must not retain packaged sources or their build tree.
# The installed candidate is the only product runtime input that remains.
remove_adoption_tree "$package_sources"
remove_adoption_tree "$install_target"
bash scripts/check-binary-adoption.sh "$candidate"
echo "Packaged standalone adoption passed without plugin or package source."
