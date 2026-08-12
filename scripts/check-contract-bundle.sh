#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
check_root="$(mktemp -d "${TMPDIR:-/tmp}/norm-contract-check.XXXXXX")"

cleanup() {
  case "$check_root" in
    "${TMPDIR:-/tmp}"/norm-contract-check.*) rm -rf -- "${check_root:?}" ;;
    *) echo "refusing to remove unexpected contract-check path: $check_root" >&2 ;;
  esac
}
trap cleanup EXIT

cd "$repo_root"
bash scripts/update-contract-lock.sh
if ! git diff --exit-code -- tests/contract/bundle.lock.json; then
  echo "contract bundle lock is stale; regenerate and commit it" >&2
  exit 1
fi

bash scripts/export-contract-bundle.sh "$check_root/bundle"
cargo build -p norm-spec-cli --bins

target_root="${CARGO_TARGET_DIR:-$repo_root/target}"
if [[ "$target_root" != /* ]]; then
  target_root="$repo_root/$target_root"
fi
candidate="$target_root/debug/norm"
runner="$target_root/debug/norm-spec-conformance"
if [[ ! -x "$candidate" ]] || [[ ! -x "$runner" ]]; then
  echo "built contract candidate or runner is unavailable" >&2
  exit 1
fi

"$runner" --candidate "$candidate" --contract-dir "$check_root/bundle"
echo "Locked contract bundle and arbitrary-candidate conformance passed."
