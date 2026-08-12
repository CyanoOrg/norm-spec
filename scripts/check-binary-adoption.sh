#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 ]]; then
  echo "usage: scripts/check-binary-adoption.sh <norm-candidate>" >&2
  exit 2
fi

candidate="$1"
if [[ ! -f "$candidate" ]] || [[ -L "$candidate" ]]; then
  echo "standalone candidate is unavailable or unsafe: $candidate" >&2
  exit 2
fi
candidate_dir="$(cd "$(dirname "$candidate")" && pwd -P)"
candidate="$candidate_dir/$(basename "$candidate")"

adoption_root="$(mktemp -d "${TMPDIR:-/tmp}/norm-binary-adoption.XXXXXX")"
cleanup() {
  case "$adoption_root" in
    "${TMPDIR:-/tmp}"/norm-binary-adoption.*)
      if [[ -d "$adoption_root" ]] && [[ ! -L "$adoption_root" ]]; then
        rm -rf -- "${adoption_root:?}"
      fi
      ;;
    *) echo "refusing to remove unexpected binary-adoption path: $adoption_root" >&2 ;;
  esac
}
trap cleanup EXIT

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

version_stdout="$adoption_root/version.stdout"
version_stderr="$adoption_root/version.stderr"
"$candidate" --version >"$version_stdout" 2>"$version_stderr"
version_line="$(<"$version_stdout")"
if [[ -s "$version_stderr" ]] || [[ ! "$version_line" =~ ^norm[[:space:]][0-9A-Za-z.+-]+$ ]]; then
  echo "standalone version discovery returned the wrong streams or identity" >&2
  exit 1
fi
version="${version_line#norm }"

compatibility="$("$candidate" compatibility)"
assert_contains "$compatibility" '"apiVersion":"norm-spec/compatibility/v1"' "compatibility discovery"
assert_contains "$compatibility" "\"version\":\"$version\"" "compatibility discovery"
assert_contains "$compatibility" '"id":"norm-spec/rust-api/v1"' "compatibility discovery"
assert_contains "$compatibility" '"suite":"norm-spec/a1-cli/v1"' "compatibility discovery"

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

printf '%s\n' \
  '---' \
  'metadata:' \
  '  layer: adoption' \
  '  scope: docs/' \
  '  version: "1.0"' \
  'agent_rules:' \
  '  reference_policy:' \
  '    target_must_exist: true' \
  '    allow_external: false' \
  '    require_description: true' \
  'strong_references:' \
  '  - type: source' \
  '    target: missing.md' \
  '    sync: auto_pull' \
  '    validation: strict' \
  '    description: Deliberately missing standalone-adoption fixture' \
  '---' \
  '' \
  '# Invalid standalone convention' >"$project_root/docs/.norm"

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

echo "Standalone binary adoption passed without plugin or source checkout."
