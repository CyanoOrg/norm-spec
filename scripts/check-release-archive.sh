#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 3 ]]; then
  echo "usage: scripts/check-release-archive.sh <archive> <expected-rust-target> <expected-source-revision>" >&2
  exit 2
fi

archive="$1"
expected_target="$2"
expected_source_revision="$3"
case "$expected_target" in
  x86_64-unknown-linux-gnu|aarch64-apple-darwin|x86_64-apple-darwin|x86_64-pc-windows-msvc) ;;
  *)
    echo "unsupported expected release target: $expected_target" >&2
    exit 2
    ;;
esac
if [[ ! "$expected_source_revision" =~ ^[0-9a-f]{40}$ ]]; then
  echo "expected source revision must be 40 lowercase hexadecimal characters" >&2
  exit 2
fi

host="$(rustc -vV | sed -n 's/^host: //p' | tr -d '\r')"
if [[ "$host" != "$expected_target" ]]; then
  echo "release target $expected_target must execute on a matching native host; found $host" >&2
  exit 1
fi

if [[ ! -f "$archive" ]] || [[ -L "$archive" ]] || [[ "$archive" != *.tar.gz ]]; then
  echo "release archive is unavailable, linked, or has the wrong suffix: $archive" >&2
  exit 2
fi
archive_dir="$(cd "$(dirname "$archive")" && pwd -P)"
archive="$archive_dir/$(basename "$archive")"
checksum="$archive.sha256"
if [[ ! -f "$checksum" ]] || [[ -L "$checksum" ]]; then
  echo "release checksum is unavailable or unsafe: $checksum" >&2
  exit 2
fi

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
expected_checksum="$archive_digest  $(basename "$archive")"
if [[ "$(<"$checksum")" != "$expected_checksum" ]]; then
  echo "release archive checksum does not match" >&2
  exit 1
fi

check_root="$(mktemp -d "${TMPDIR:-/tmp}/norm-release-check.XXXXXX")"
cleanup() {
  case "$check_root" in
    "${TMPDIR:-/tmp}"/norm-release-check.*)
      if [[ -d "$check_root" ]] && [[ ! -L "$check_root" ]]; then
        rm -rf -- "${check_root:?}"
      fi
      ;;
    *) echo "refusing to remove unexpected release-check path: $check_root" >&2 ;;
  esac
}
trap cleanup EXIT

members="$check_root/archive-members.txt"
tar -tzf "$archive" >"$members"
if [[ ! -s "$members" ]]; then
  echo "release archive is empty" >&2
  exit 1
fi
while IFS= read -r member; do
  member="${member#./}"
  case "$member" in
    ""|/*|../*|*/../*|*/..|*\\*)
      echo "release archive contains an unsafe path: $member" >&2
      exit 1
      ;;
  esac
done <"$members"

archive_stem="$(basename "$archive" .tar.gz)"
if [[ "$archive_stem" != norm-spec-*-$expected_target ]]; then
  echo "release archive name does not bind the expected target" >&2
  exit 1
fi
top_levels="$(sed 's#^\./##; s#/.*##' "$members" | sort -u)"
if [[ "$top_levels" != "$archive_stem" ]]; then
  echo "release archive must contain exactly its versioned top-level directory" >&2
  exit 1
fi

tar -xzf "$archive" -C "$check_root"
release_root="$check_root/$archive_stem"
if [[ ! -d "$release_root" ]] || [[ -L "$release_root" ]]; then
  echo "extracted release root is unavailable or unsafe" >&2
  exit 1
fi
linked="$(find "$release_root" -type l -print -quit)"
if [[ -n "$linked" ]]; then
  echo "release archive contains a symbolic link: $linked" >&2
  exit 1
fi

actual_inventory="$check_root/actual-inventory.txt"
expected_inventory="$check_root/expected-inventory.txt"
find "$release_root" -type f -print \
  | sed "s#^$release_root/##" \
  | LC_ALL=C sort >"$actual_inventory"
printf '%s\n' \
  'LICENSE' \
  'README.md' \
  'README.zh-CN.md' \
  'bin/norm' \
  'bin/norm-spec-conformance' \
  'contract/bundle.lock.json' \
  'release-manifest.json' \
  'skills/norm-spec/SKILL.md' \
  'skills/norm-spec/references/authoring.md' \
  'skills/norm-spec/references/field-reference.md' >"$expected_inventory"
if [[ "$expected_target" == "x86_64-pc-windows-msvc" ]]; then
  sed -e 's#bin/norm$#bin/norm.exe#' \
    -e 's#bin/norm-spec-conformance$#bin/norm-spec-conformance.exe#' \
    "$expected_inventory" >"$expected_inventory.windows"
  mv "$expected_inventory.windows" "$expected_inventory"
fi
sed -n 's/^[[:space:]]*{"path": "\([^"]*\)".*/contract\/\1/p' \
  "$release_root/contract/bundle.lock.json" >>"$expected_inventory"
LC_ALL=C sort -u "$expected_inventory" -o "$expected_inventory"
if ! cmp -s "$expected_inventory" "$actual_inventory"; then
  echo "release archive inventory does not match the D016 contract" >&2
  diff -u "$expected_inventory" "$actual_inventory" >&2 || true
  exit 1
fi

version="${archive_stem#norm-spec-}"
version="${version%-$expected_target}"
manifest="$release_root/release-manifest.json"
source_revision="$(sed -n 's/^[[:space:]]*"sourceRevision": "\([0-9a-f]*\)",*/\1/p' "$manifest")"
contract_digest="$(sed -n 's/^[[:space:]]*"contractDigest": "\([^"]*\)".*/\1/p' "$manifest")"
if [[ "$source_revision" != "$expected_source_revision" ]] \
  || [[ ! "$contract_digest" =~ ^sha256:[0-9a-f]{64}$ ]]; then
  echo "release manifest contains an unexpected source or contract identity" >&2
  exit 1
fi
for required_line in \
  '  "apiVersion": "norm-spec/release-artifact/v1",' \
  "    \"version\": \"$version\"" \
  "  \"target\": \"$expected_target\"," \
  '  "compatibilityApi": "norm-spec/compatibility/v1",' \
  '    "bundleApi": "norm-spec/contract-bundle/v1",' \
  '    "reportApi": "norm-spec/conformance/v1",' \
  '    "suite": "norm-spec/a1-cli/v1",' \
  '    "caseCount": 82,' \
  '    "path": "skills/norm-spec"'; do
  if ! grep -Fqx "$required_line" "$manifest"; then
    echo "release manifest omitted the exact field: $required_line" >&2
    exit 1
  fi
done
if ! grep -Fqx "    \"contractDigest\": \"$contract_digest\"" "$manifest"; then
  echo "release manifest has an inconsistent contract digest" >&2
  exit 1
fi
if ! grep -Fqx "  \"contractDigest\": \"$contract_digest\"," \
  "$release_root/contract/bundle.lock.json"; then
  echo "release manifest and contract lock identify different suites" >&2
  exit 1
fi

exe_suffix=""
if [[ "$expected_target" == "x86_64-pc-windows-msvc" ]]; then
  exe_suffix=".exe"
fi
candidate="$release_root/bin/norm$exe_suffix"
runner="$release_root/bin/norm-spec-conformance$exe_suffix"
if [[ ! -f "$candidate" ]] || [[ ! -f "$runner" ]]; then
  echo "release binaries are unavailable" >&2
  exit 1
fi

version_stdout="$check_root/version.stdout"
version_stderr="$check_root/version.stderr"
"$candidate" --version >"$version_stdout" 2>"$version_stderr"
if [[ -s "$version_stderr" ]] || [[ "$(<"$version_stdout")" != "norm $version" ]]; then
  echo "release binary version does not match the archive identity" >&2
  exit 1
fi

compatibility="$("$candidate" compatibility)"
for identity in \
  '"apiVersion":"norm-spec/compatibility/v1"' \
  '"bundleApi":"norm-spec/contract-bundle/v1"' \
  '"reportApi":"norm-spec/conformance/v1"' \
  '"suite":"norm-spec/a1-cli/v1"' \
  '"caseCount":82' \
  "\"contractDigest\":\"$contract_digest\""; do
  if [[ "$compatibility" != *"$identity"* ]]; then
    echo "release compatibility omitted exact identity: $identity" >&2
    exit 1
  fi
done

conformance_stdout="$check_root/conformance.stdout"
conformance_stderr="$check_root/conformance.stderr"
"$runner" \
  --candidate "$candidate" \
  --contract-dir "$release_root/contract" \
  >"$conformance_stdout" 2>"$conformance_stderr"
if [[ -s "$conformance_stderr" ]]; then
  echo "release conformance wrote to stderr" >&2
  exit 1
fi
conformance="$(<"$conformance_stdout")"
for result in \
  '"apiVersion":"norm-spec/conformance/v1"' \
  '"status":"pass"' \
  '"complete":true' \
  '"declared":82' \
  '"executed":82' \
  '"passed":82' \
  '"failed":0' \
  '"notExecuted":0'; do
  if [[ "$conformance" != *"$result"* ]]; then
    echo "release conformance omitted exact result: $result" >&2
    exit 1
  fi
done

script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
bash "$script_root/check-binary-adoption.sh" "$candidate"

echo "Release archive passed checksum, inventory, identity, conformance, and adoption checks."
