#!/usr/bin/env bash
set -euo pipefail

NA_ID=""
PR_NUM=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --na)
      NA_ID="${2:-}"
      shift 2
      ;;
    --pr)
      PR_NUM="${2:-}"
      shift 2
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

if [[ ! -f NEXT_ACTIONS.md ]]; then
  echo "error: run from repository root (NEXT_ACTIONS.md missing)" >&2
  exit 2
fi

if [[ -x scripts/ci/hygiene_sentinel.sh ]]; then
  scripts/ci/hygiene_sentinel.sh --require-clean --fail-on-tmp --fail-on-main-pin
else
  echo "error: scripts/ci/hygiene_sentinel.sh missing or not executable" >&2
  exit 2
fi

BRANCH="$(git branch --show-current)"
HEAD_SHA="$(git rev-parse HEAD)"

READY_LINES="$(rg -n 'Status:\s*READY' NEXT_ACTIONS.md || true)"
if [[ -n "$READY_LINES" ]]; then
  READY_COUNT="$(printf '%s\n' "$READY_LINES" | wc -l | tr -d ' ')"
else
  READY_COUNT=0
fi

if [[ "$READY_COUNT" -gt 1 ]]; then
  echo "FAIL: READY_COUNT=$READY_COUNT (>1)" >&2
  printf '%s\n' "$READY_LINES"
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  CLEAN_TREE="no"
else
  CLEAN_TREE="yes"
fi

echo "== preflight_governance =="
echo "branch=$BRANCH"
echo "head_sha=$HEAD_SHA"
echo "ready_count=$READY_COUNT"
if [[ -n "$READY_LINES" ]]; then
  echo "ready_lines:"
  printf '%s\n' "$READY_LINES"
else
  echo "ready_lines: <none>"
fi

echo "git_status_porcelain:"
git status --porcelain || true

echo "diff_stat_worktree_vs_head:"
git diff --stat || true

echo "diff_stat_staged_vs_head:"
git diff --cached --stat || true

echo "pr_body_snippet:"
echo "Branch: $BRANCH"
echo "Commit: $HEAD_SHA"
if git rev-parse --verify -q origin/main >/dev/null; then
  echo "Name-only diff (origin/main...HEAD):"
  git diff --name-only origin/main...HEAD || true
else
  echo "Name-only diff (HEAD~1...HEAD):"
  git diff --name-only HEAD~1...HEAD || true
fi

if [[ -n "$NA_ID" ]]; then
  echo "na_lookup:"
  rg -n "${NA_ID}|Status:\s*(READY|DONE|BACKLOG)" NEXT_ACTIONS.md TRACEABILITY.md || true
fi

if [[ -n "$PR_NUM" ]] && command -v gh >/dev/null 2>&1; then
  echo "pr_scope_guard($PR_NUM):"
  gh pr diff "$PR_NUM" --name-only || true
fi

echo "clean_tree=$CLEAN_TREE"
if [[ "$CLEAN_TREE" != "yes" ]]; then
  echo "FAIL: working tree is not clean" >&2
  exit 1
fi

echo "OK"
