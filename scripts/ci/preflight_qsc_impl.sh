#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f Cargo.toml ]]; then
  echo "error: run from repository root (Cargo.toml missing)" >&2
  exit 2
fi

if [[ -x scripts/ci/hygiene_sentinel.sh ]]; then
  scripts/ci/hygiene_sentinel.sh --fail-on-tmp --fail-on-main-pin
else
  echo "error: scripts/ci/hygiene_sentinel.sh missing or not executable" >&2
  exit 2
fi

BRANCH="$(git branch --show-current)"
HEAD_SHA="$(git rev-parse HEAD)"

echo "== preflight_qsc_impl =="
echo "branch=$BRANCH"
echo "head_sha=$HEAD_SHA"
echo "name_only_diff(origin/main...HEAD):"
if git rev-parse --verify -q origin/main >/dev/null; then
  git diff --name-only origin/main...HEAD || true
else
  git diff --name-only HEAD~1...HEAD || true
fi

echo "required_gates:"
echo "  cargo fmt -p qsc -- --check"
echo "  cargo test -p qsc --locked"
echo "  cargo clippy -p qsc --all-targets -- -D warnings"

if ! command -v cargo >/dev/null 2>&1; then
  echo "WARN: cargo not available in environment; commands printed above" >&2
  exit 0
fi

cargo fmt -p qsc -- --check
cargo test -p qsc --locked
cargo clippy -p qsc --all-targets -- -D warnings

echo "OK"
