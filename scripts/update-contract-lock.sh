#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
contract_source="$repo_root/tests/contract"
lock_path="$contract_source/bundle.lock.json"
inventory_file="$(mktemp "${TMPDIR:-/tmp}/norm-contract-inventory.XXXXXX")"
digest_file="$(mktemp "${TMPDIR:-/tmp}/norm-contract-digest.XXXXXX")"
output_file="$(mktemp "${TMPDIR:-/tmp}/norm-contract-lock.XXXXXX")"

cleanup() {
  for temporary in "$inventory_file" "$digest_file" "$output_file"; do
    case "$temporary" in
      "${TMPDIR:-/tmp}"/norm-contract-*) rm -f -- "${temporary:?}" ;;
      *) echo "refusing to remove unexpected temporary path: $temporary" >&2 ;;
    esac
  done
}
trap cleanup EXIT

if ! command -v shasum >/dev/null 2>&1; then
  echo "shasum is required to update the contract lock" >&2
  exit 1
fi

{
  cd "$contract_source"
  find expected fixtures layouts -type f -print
  printf '%s\n' manifest.tsv requirements.tsv
  cd "$repo_root"
  find schema -type f -name '*.json' -print
} | LC_ALL=C sort | while IFS= read -r relative_path; do
  case "$relative_path" in
    *[!A-Za-z0-9._/-]*|/*|*'..'*|*'//'*)
      echo "unsafe contract path: $relative_path" >&2
      exit 1
      ;;
  esac
  case "$relative_path" in
    schema/*) source_path="$repo_root/$relative_path" ;;
    *) source_path="$contract_source/$relative_path" ;;
  esac
  file_digest="$(shasum -a 256 "$source_path" | awk '{print $1}')"
  printf '%s\t%s\n' "$relative_path" "$file_digest"
done >"$inventory_file"

file_count="$(wc -l <"$inventory_file" | tr -d ' ')"
if [[ "$file_count" -eq 0 ]]; then
  echo "contract inventory is empty" >&2
  exit 1
fi

{
  printf 'norm-spec/contract-digest/v1\n'
  printf 'suite=norm-spec/a1-cli/v1\n'
  printf 'cases=82\n'
  while IFS=$'\t' read -r relative_path file_digest; do
    printf '%s\0%s\n' "$relative_path" "$file_digest"
  done <"$inventory_file"
} >"$digest_file"

contract_digest="$(shasum -a 256 "$digest_file" | awk '{print $1}')"
{
  printf '{\n'
  printf '  "apiVersion": "norm-spec/contract-bundle/v1",\n'
  printf '  "suite": "norm-spec/a1-cli/v1",\n'
  printf '  "caseCount": 82,\n'
  printf '  "contractDigest": "sha256:%s",\n' "$contract_digest"
  printf '  "files": [\n'
  awk -F '\t' '
    { entries[NR] = sprintf("    {\"path\": \"%s\", \"sha256\": \"%s\"}", $1, $2) }
    END {
      for (row = 1; row <= NR; row++) {
        suffix = row == NR ? "" : ","
        print entries[row] suffix
      }
    }
  ' "$inventory_file"
  printf '  ]\n'
  printf '}\n'
} >"$output_file"

mv "$output_file" "$lock_path"
echo "Updated $lock_path with $file_count files and sha256:$contract_digest"
