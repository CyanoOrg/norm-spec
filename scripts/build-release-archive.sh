#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 2 ]]; then
  echo "usage: scripts/build-release-archive.sh <rust-target> <output-directory>" >&2
  exit 2
fi

target="$1"
output_dir="$2"
case "$target" in
  x86_64-unknown-linux-gnu|aarch64-apple-darwin|x86_64-apple-darwin|x86_64-pc-windows-msvc) ;;
  *)
    echo "unsupported release target: $target" >&2
    exit 2
    ;;
esac

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "release archives require a clean tracked worktree" >&2
  exit 1
fi

source_revision="$(git rev-parse HEAD)"
if [[ ! "$source_revision" =~ ^[0-9a-f]{40}$ ]]; then
  echo "failed to resolve a full lowercase Git source revision" >&2
  exit 1
fi

host="$(rustc -vV | sed -n 's/^host: //p' | tr -d '\r')"
if [[ "$host" != "$target" ]]; then
  echo "release target $target must execute on a matching native host; found $host" >&2
  exit 1
fi

version="$(cargo pkgid -p norm-spec-cli | sed 's/.*@//')"
if [[ -z "$version" ]] || [[ "$version" == *[!0-9A-Za-z.+-]* ]]; then
  echo "failed to resolve a safe norm-spec-cli package version" >&2
  exit 1
fi

if [[ -e "$output_dir" ]] && [[ ! -d "$output_dir" ]]; then
  echo "release output is not a directory: $output_dir" >&2
  exit 2
fi
if [[ -L "$output_dir" ]]; then
  echo "release output must not be a symbolic link: $output_dir" >&2
  exit 2
fi
mkdir -p "$output_dir"
output_dir="$(cd "$output_dir" && pwd -P)"

archive_stem="norm-spec-$version-$target"
archive="$output_dir/$archive_stem.tar.gz"
checksum="$archive.sha256"
if [[ -e "$archive" ]] || [[ -e "$checksum" ]]; then
  echo "refusing to overwrite an existing release candidate: $archive_stem" >&2
  exit 1
fi

stage_parent="$(mktemp -d "${TMPDIR:-/tmp}/norm-release-stage.XXXXXX")"
cleanup() {
  case "$stage_parent" in
    "${TMPDIR:-/tmp}"/norm-release-stage.*)
      if [[ -d "$stage_parent" ]] && [[ ! -L "$stage_parent" ]]; then
        rm -rf -- "${stage_parent:?}"
      fi
      ;;
    *) echo "refusing to remove unexpected release staging path: $stage_parent" >&2 ;;
  esac
}
trap cleanup EXIT

stage_root="$stage_parent/$archive_stem"
mkdir -p "$stage_root/bin" "$stage_root/contract" "$stage_root/skills"

cargo build --release --locked --target "$target" -p norm-spec-cli --bins

exe_suffix=""
if [[ "$target" == "x86_64-pc-windows-msvc" ]]; then
  exe_suffix=".exe"
fi
target_root="${CARGO_TARGET_DIR:-$repo_root/target}"
if [[ "$target_root" != /* ]] && [[ ! "$target_root" =~ ^[A-Za-z]:[/\\] ]]; then
  target_root="$repo_root/$target_root"
fi
binary_root="$target_root/$target/release"
for binary in norm norm-spec-conformance; do
  source_binary="$binary_root/$binary$exe_suffix"
  if [[ ! -f "$source_binary" ]] || [[ -L "$source_binary" ]]; then
    echo "built release binary is unavailable or unsafe: $source_binary" >&2
    exit 1
  fi
  cp "$source_binary" "$stage_root/bin/$binary$exe_suffix"
  chmod 755 "$stage_root/bin/$binary$exe_suffix"
done

bash scripts/export-contract-bundle.sh "$stage_root/contract" >/dev/null
cp -R skills/norm-spec "$stage_root/skills/norm-spec"
cp LICENSE README.md README.zh-CN.md "$stage_root/"

lock="$stage_root/contract/bundle.lock.json"
bundle_api="$(sed -n 's/^[[:space:]]*"apiVersion": "\([^"]*\)",*/\1/p' "$lock" | head -1)"
suite="$(sed -n 's/^[[:space:]]*"suite": "\([^"]*\)",*/\1/p' "$lock" | head -1)"
case_count="$(sed -n 's/^[[:space:]]*"caseCount": \([0-9][0-9]*\),*/\1/p' "$lock" | head -1)"
contract_digest="$(sed -n 's/^[[:space:]]*"contractDigest": "\([^"]*\)",*/\1/p' "$lock" | head -1)"
if [[ "$bundle_api" != "norm-spec/contract-bundle/v1" ]] \
  || [[ "$suite" != "norm-spec/a1-cli/v1" ]] \
  || [[ "$case_count" != "82" ]] \
  || [[ ! "$contract_digest" =~ ^sha256:[0-9a-f]{64}$ ]]; then
  echo "exported contract bundle has an unexpected identity" >&2
  exit 1
fi

norm_path="bin/norm$exe_suffix"
conformance_path="bin/norm-spec-conformance$exe_suffix"
manifest="$stage_root/release-manifest.json"
printf '%s\n' \
  '{' \
  '  "apiVersion": "norm-spec/release-artifact/v1",' \
  '  "product": {' \
  '    "name": "norm-spec",' \
  "    \"version\": \"$version\"" \
  '  },' \
  "  \"target\": \"$target\"," \
  "  \"sourceRevision\": \"$source_revision\"," \
  '  "executables": {' \
  "    \"norm\": \"$norm_path\"," \
  "    \"conformance\": \"$conformance_path\"" \
  '  },' \
  '  "compatibilityApi": "norm-spec/compatibility/v1",' \
  '  "contract": {' \
  "    \"path\": \"contract\"," \
  "    \"bundleApi\": \"$bundle_api\"," \
  '    "reportApi": "norm-spec/conformance/v1",' \
  "    \"suite\": \"$suite\"," \
  "    \"caseCount\": $case_count," \
  "    \"contractDigest\": \"$contract_digest\"" \
  '  },' \
  '  "skill": {' \
  '    "path": "skills/norm-spec"' \
  '  }' \
  '}' >"$manifest"

COPYFILE_DISABLE=1 tar -czf "$archive" -C "$stage_parent" "$archive_stem"

sha256_file() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  else
    echo "no supported SHA-256 command is available" >&2
    return 1
  fi
}

archive_digest="$(sha256_file "$archive")"
if [[ ! "$archive_digest" =~ ^[0-9a-f]{64}$ ]]; then
  echo "release archive returned an invalid SHA-256 digest" >&2
  exit 1
fi
printf '%s  %s\n' "$archive_digest" "$(basename "$archive")" >"$checksum"

echo "Built release candidate: $archive"
echo "Built release checksum: $checksum"
