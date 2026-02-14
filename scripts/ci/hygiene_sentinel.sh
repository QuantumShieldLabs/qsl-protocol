#!/usr/bin/env bash
set -euo pipefail

REQUIRE_CLEAN=0
FAIL_ON_TMP=0
FAIL_ON_MAIN_PIN=0

usage() {
  cat <<'USAGE'
Usage: scripts/ci/hygiene_sentinel.sh [--require-clean] [--fail-on-tmp] [--fail-on-main-pin]
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --require-clean)
      REQUIRE_CLEAN=1
      shift
      ;;
    --fail-on-tmp)
      FAIL_ON_TMP=1
      shift
      ;;
    --fail-on-main-pin)
      FAIL_ON_MAIN_PIN=1
      shift
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

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT" ]]; then
  echo "error: not inside a git repository" >&2
  exit 2
fi

cd "$ROOT"

echo "== hygiene_sentinel =="
echo "repo_root=$ROOT"

echo "git_status_porcelain:"
STATUS_OUT="$(git status --porcelain || true)"
if [[ -n "$STATUS_OUT" ]]; then
  printf '%s\n' "$STATUS_OUT"
else
  echo "<clean>"
fi

if [[ "$REQUIRE_CLEAN" -eq 1 && -n "$STATUS_OUT" ]]; then
  echo "FAIL: --require-clean set and working tree is dirty" >&2
  exit 1
fi

echo "worktree_prune:"
PRUNE_OUT="$(git worktree prune 2>&1 || true)"
if [[ -n "$PRUNE_OUT" ]]; then
  printf '%s\n' "$PRUNE_OUT"
else
  echo "<no prune output>"
fi

echo "worktree_list_porcelain:"
WT_PORCELAIN="$(git worktree list --porcelain)"
printf '%s\n' "$WT_PORCELAIN"

CANONICAL_PATH="$(printf '%s\n' "$WT_PORCELAIN" | awk '$1=="worktree"{print substr($0,10); exit}')"
if [[ -z "$CANONICAL_PATH" ]]; then
  echo "FAIL: could not determine canonical worktree path" >&2
  exit 1
fi
echo "canonical_worktree=$CANONICAL_PATH"

TMP_ISSUES=0
MAIN_PIN_ISSUES=0
PRUNABLE_ISSUES=0

while IFS=$'\t' read -r WT_PATH WT_BRANCH WT_DETACHED WT_PRUNABLE; do
  [[ -z "$WT_PATH" ]] && continue

  if [[ "$WT_PRUNABLE" == "1" ]]; then
    PRUNABLE_ISSUES=$((PRUNABLE_ISSUES + 1))
    echo "WARN: prunable worktree entry detected: $WT_PATH"
    echo "  remediation: git worktree prune"
  fi

  if [[ "$WT_PATH" == /tmp/* ]]; then
    TMP_ISSUES=$((TMP_ISSUES + 1))
    echo "WARN: /tmp worktree detected: $WT_PATH"
    echo "  remediation: git worktree remove \"$WT_PATH\""
    echo "  remediation: git worktree prune"
  fi

  if [[ "$WT_BRANCH" == "refs/heads/main" && "$WT_PATH" != "$CANONICAL_PATH" ]]; then
    MAIN_PIN_ISSUES=$((MAIN_PIN_ISSUES + 1))
    echo "WARN: main branch pinned outside canonical worktree: $WT_PATH"
    echo "  canonical: $CANONICAL_PATH"
    echo "  remediation: git worktree remove \"$WT_PATH\""
    echo "  remediation: git worktree prune"
  fi
done < <(
  printf '%s\n' "$WT_PORCELAIN" | awk '
    $1=="worktree" {
      if (path != "") print path "\t" branch "\t" detached "\t" prunable;
      path=substr($0,10); branch=""; detached="0"; prunable="0"; next
    }
    $1=="branch"   { branch=$2; next }
    $1=="detached" { detached="1"; next }
    $1=="prunable" { prunable="1"; next }
    END {
      if (path != "") print path "\t" branch "\t" detached "\t" prunable;
    }'
)

echo "summary: tmp_issues=$TMP_ISSUES main_pin_issues=$MAIN_PIN_ISSUES prunable_entries=$PRUNABLE_ISSUES"

if [[ "$FAIL_ON_TMP" -eq 1 && "$TMP_ISSUES" -gt 0 ]]; then
  echo "FAIL: --fail-on-tmp set and /tmp worktrees were found" >&2
  exit 1
fi

if [[ "$FAIL_ON_MAIN_PIN" -eq 1 && "$MAIN_PIN_ISSUES" -gt 0 ]]; then
  echo "FAIL: --fail-on-main-pin set and main is pinned outside canonical worktree" >&2
  exit 1
fi

echo "OK"
