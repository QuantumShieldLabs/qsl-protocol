#!/usr/bin/env bash
set -euo pipefail

PR="${1:-}"
if [[ -z "$PR" ]]; then
  PR="$(gh pr view --json number -q .number)"
fi

# Required checks (must be present and passing)
REQ=(
  "suite2-ci/suite2-vectors"
  "Goal compliance/goal-lint"
  "qshield-ci/ci-4a"
  "qshield-ci/ci-4b"
  "qshield-ci/ci-4c"
  "qshield-ci/ci-4d"
  "qshield-ci/ci-4d-dur"
)

# Fetch checks JSON
json="$(gh pr checks "$PR" --json name,state 2>/dev/null)"

fail=0
for name in "${REQ[@]}"; do
  state="$(python3 - <<PY
import json
j=json.loads('''$json''')
for r in j:
    if r.get("name")== "$name":
        print(r.get("state",""))
        break
PY
)"
  if [[ "$state" != "SUCCESS" ]]; then
    echo "ERROR: required check not SUCCESS: $name (state=$state)" >&2
    fail=1
  fi
done

if [[ "$fail" -ne 0 ]]; then
  echo "Refusing to proceed." >&2
  exit 2
fi

echo "OK: all required checks SUCCESS for PR #$PR"
