#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 ]]; then
  echo "usage: scripts/export-contract-bundle.sh <empty-destination>" >&2
  exit 2
fi

repo_root="$(git rev-parse --show-toplevel)"
contract_source="$repo_root/tests/contract"
destination="$1"

if [[ -e "$destination" ]] && [[ ! -d "$destination" ]]; then
  echo "contract destination is not a directory: $destination" >&2
  exit 2
fi
mkdir -p "$destination"

if find "$destination" -mindepth 1 -print -quit | grep -q .; then
  echo "contract destination must be empty: $destination" >&2
  exit 2
fi

cp "$contract_source/bundle.lock.json" "$destination/bundle.lock.json"
cp "$contract_source/manifest.tsv" "$destination/manifest.tsv"
cp "$contract_source/requirements.tsv" "$destination/requirements.tsv"
cp -R "$contract_source/expected" "$destination/expected"
cp -R "$contract_source/fixtures" "$destination/fixtures"
cp -R "$contract_source/layouts" "$destination/layouts"
mkdir -p "$destination/schema/profiles"
cp "$repo_root/schema/norm-schema.json" "$destination/schema/norm-schema.json"
cp "$repo_root/schema/profiles/"*.json "$destination/schema/profiles/"

echo "Exported the locked A1 CLI contract bundle to $destination"
