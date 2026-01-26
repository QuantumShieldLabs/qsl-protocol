#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WF="$ROOT/.github/workflows/release-auth.yml"

if [[ ! -f "$WF" ]]; then
  echo "[release-auth] no release-auth workflow found; skipping policy check"
  exit 0
fi

if ! grep -q "attest-build-provenance" "$WF"; then
  echo "[release-auth] ERROR: release-auth workflow missing provenance attestation step" >&2
  exit 2
fi

if ! grep -q "sha256sum" "$WF"; then
  echo "[release-auth] ERROR: release-auth workflow missing checksum generation" >&2
  exit 2
fi

if ! grep -q -- "--locked" "$WF"; then
  echo "[release-auth] ERROR: release-auth workflow missing --locked builds" >&2
  exit 2
fi

echo "[release-auth] OK"
