\
#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/audit/run_goal_lint_pr.sh <PR_NUMBER>

Runs tools/goal_lint.py locally for a given GitHub PR by synthesizing the
minimal GITHUB_EVENT_PATH payload required by goal_lint.py.

This wrapper is designed to work both:
  - after merge (prefers mergeCommit parents), and
  - pre-merge (falls back to baseRefName + headRefOid).

Requirements:
  - gh authenticated (gh auth status)
  - python3 available
  - git can resolve required commits locally (best-effort fetch is attempted)
USAGE
}

if [[ $# -ne 1 ]]; then
  usage >&2
  exit 2
fi

PR="$1"

if ! command -v gh >/dev/null 2>&1; then
  echo "ERROR: gh not found in PATH." >&2
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "ERROR: python3 not found in PATH." >&2
  exit 1
fi
if ! gh auth status >/dev/null 2>&1; then
  echo "ERROR: gh not authenticated; run: gh auth login" >&2
  exit 1
fi

# Gather PR metadata (all fields used here are supported by gh pr view).
BODY="$(gh pr view "$PR" --json body -q .body || true)"
STATE="$(gh pr view "$PR" --json state -q .state || true)"
BASE_REF_NAME="$(gh pr view "$PR" --json baseRefName -q .baseRefName || true)"
HEAD_REF_OID="$(gh pr view "$PR" --json headRefOid -q .headRefOid || true)"
MERGE_OID="$(gh pr view "$PR" --json mergeCommit -q .mergeCommit.oid 2>/dev/null || true)"

if [[ -z "$BODY" ]]; then
  echo "ERROR: unable to read PR body via gh; is PR #$PR accessible?" >&2
  exit 1
fi

BASE_SHA=""
HEAD_SHA=""

# Prefer merge commit parents when available (post-merge audit).
if [[ -n "$MERGE_OID" ]]; then
  git fetch --quiet --all --prune --tags || true
  if git cat-file -e "$MERGE_OID"^{commit} 2>/dev/null; then
    BASE_SHA="$(git rev-parse "${MERGE_OID}^1" 2>/dev/null || true)"
    HEAD_SHA="$(git rev-parse "${MERGE_OID}^2" 2>/dev/null || true)"
  fi
fi

# Fallback: use headRefOid and resolve base from baseRefName.
if [[ -z "$BASE_SHA" || -z "$HEAD_SHA" ]]; then
  if [[ -z "$BASE_REF_NAME" || -z "$HEAD_REF_OID" ]]; then
    echo "ERROR: unable to resolve PR base/head metadata via gh." >&2
    echo "  state=$STATE" >&2
    echo "  baseRefName=$BASE_REF_NAME" >&2
    echo "  headRefOid=$HEAD_REF_OID" >&2
    echo "  mergeCommit.oid=$MERGE_OID" >&2
    exit 1
  fi

  git fetch --quiet --all --prune --tags || true

  HEAD_SHA="$HEAD_REF_OID"
  if BASE_TMP=$(git rev-parse "origin/${BASE_REF_NAME}" 2>/dev/null); then
    BASE_SHA="$BASE_TMP"
  elif BASE_TMP=$(git rev-parse "${BASE_REF_NAME}" 2>/dev/null); then
    BASE_SHA="$BASE_TMP"
  else
    BASE_SHA=""
  fi
fi

if [[ -z "$BASE_SHA" || -z "$HEAD_SHA" ]]; then
  echo "ERROR: unable to resolve base/head SHAs for PR #$PR." >&2
  echo "  state=$STATE" >&2
  echo "  baseRefName=$BASE_REF_NAME" >&2
  echo "  headRefOid=$HEAD_REF_OID" >&2
  echo "  mergeCommit.oid=$MERGE_OID" >&2
  exit 1
fi

# Final verification: commits must exist locally.
if ! git cat-file -e "$BASE_SHA"^{commit} 2>/dev/null; then
  echo "ERROR: base SHA not present locally: $BASE_SHA" >&2
  echo "Try: git fetch --all --prune --tags" >&2
  exit 1
fi
if ! git cat-file -e "$HEAD_SHA"^{commit} 2>/dev/null; then
  echo "ERROR: head SHA not present locally: $HEAD_SHA" >&2
  echo "Try: git fetch --all --prune --tags" >&2
  exit 1
fi

EVENT_FILE="/tmp/goal_lint_event_pr${PR}_$$.json"

# Create minimal GitHub event payload expected by tools/goal_lint.py
BODY_ESCAPED="$BODY" BASE_ENV="$BASE_SHA" HEAD_ENV="$HEAD_SHA" EVENT_ENV="$EVENT_FILE" \
python3 - <<'PY'
import json, os
body = os.environ.get("BODY_ESCAPED", "")
base = os.environ["BASE_ENV"]
head = os.environ["HEAD_ENV"]
event_path = os.environ["EVENT_ENV"]
event = {"pull_request": {"body": body, "base": {"sha": base}, "head": {"sha": head}}}
with open(event_path, "w", encoding="utf-8") as f:
    json.dump(event, f)
PY

echo "Running goal-lint for PR #$PR using:" >&2
echo "  base=$BASE_SHA" >&2
echo "  head=$HEAD_SHA" >&2

GITHUB_EVENT_PATH="$EVENT_FILE" python3 tools/goal_lint.py

rm -f "$EVENT_FILE" || true
