#!/usr/bin/env bash
set -euo pipefail

NA_ID=""
MERGE_SHA=""
EXPECT_READY="0"
EXPECT_STATE="DONE"
PR_NUM=""
REF="HEAD"

usage() {
  cat <<USAGE
Usage: $0 --na NA-XXXX [--sha <merge_sha>] [--expect-ready 0|1] [--expect-state DONE|READY] [--pr <num>] [--ref <git_ref>]
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --na)
      NA_ID="${2:-}"
      shift 2
      ;;
    --sha)
      MERGE_SHA="${2:-}"
      shift 2
      ;;
    --expect-ready)
      EXPECT_READY="${2:-}"
      shift 2
      ;;
    --expect-state)
      EXPECT_STATE="${2:-}"
      shift 2
      ;;
    --pr)
      PR_NUM="${2:-}"
      shift 2
      ;;
    --ref)
      REF="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$NA_ID" ]]; then
  echo "error: --na is required" >&2
  usage >&2
  exit 2
fi

if [[ "$EXPECT_READY" != "0" && "$EXPECT_READY" != "1" ]]; then
  echo "error: --expect-ready must be 0 or 1" >&2
  exit 2
fi

if [[ "$EXPECT_STATE" != "DONE" && "$EXPECT_STATE" != "READY" ]]; then
  echo "error: --expect-state must be DONE or READY" >&2
  exit 2
fi

if [[ ! -f NEXT_ACTIONS.md ]]; then
  echo "error: run from repository root" >&2
  exit 2
fi

if [[ -x scripts/ci/hygiene_sentinel.sh ]]; then
  scripts/ci/hygiene_sentinel.sh || true
else
  echo "WARN: scripts/ci/hygiene_sentinel.sh missing or not executable" >&2
fi

echo "== post_merge_verify =="
echo "verify_ref=$REF"
echo "head_now=$(git rev-parse HEAD)"

if ! git rev-parse --verify -q "$REF" >/dev/null; then
  echo "error: --ref '$REF' not found" >&2
  exit 2
fi

NEXT_ACTIONS_TMP="$(mktemp)"
TRACEABILITY_TMP="$(mktemp)"
trap 'rm -f "$NEXT_ACTIONS_TMP" "$TRACEABILITY_TMP"' EXIT

if ! git show "${REF}:NEXT_ACTIONS.md" >"$NEXT_ACTIONS_TMP"; then
  echo "error: could not read NEXT_ACTIONS.md from ref '$REF'" >&2
  exit 2
fi

if git show "${REF}:TRACEABILITY.md" >"$TRACEABILITY_TMP" 2>/dev/null; then
  :
else
  : >"$TRACEABILITY_TMP"
fi

READY_LINES="$(rg -n 'Status:\s*READY' "$NEXT_ACTIONS_TMP" || true)"
if [[ -n "$READY_LINES" ]]; then
  READY_COUNT="$(printf '%s\n' "$READY_LINES" | wc -l | tr -d ' ')"
else
  READY_COUNT=0
fi

echo "ready_count=$READY_COUNT"
if [[ -n "$READY_LINES" ]]; then
  echo "ready_lines:"
  printf '%s\n' "$READY_LINES"
else
  echo "ready_lines: <none>"
fi

if [[ "$READY_COUNT" != "$EXPECT_READY" ]]; then
  echo "FAIL: expected READY count $EXPECT_READY, got $READY_COUNT" >&2
  exit 1
fi

echo "na_state_lines:"
rg -n "### ${NA_ID}|Status:\s*${EXPECT_STATE}" "$NEXT_ACTIONS_TMP" || {
  echo "FAIL: missing ${NA_ID} with Status: ${EXPECT_STATE}" >&2
  exit 1
}

if [[ -n "$MERGE_SHA" ]]; then
  echo "na_evidence_sha_lines:"
  rg -n "$MERGE_SHA" "$NEXT_ACTIONS_TMP" "$TRACEABILITY_TMP" || {
    echo "FAIL: merge SHA not found in NEXT_ACTIONS.md/TRACEABILITY.md" >&2
    exit 1
  }
fi

if [[ -n "$PR_NUM" ]] && command -v gh >/dev/null 2>&1; then
  echo "name_only_diff_pr_${PR_NUM}:"
  gh pr diff "$PR_NUM" --name-only || true
fi

echo "git_status_porcelain:"
git status --porcelain
if [[ -n "$(git status --porcelain)" ]]; then
  echo "FAIL: tree not clean" >&2
  exit 1
fi

echo "OK"
