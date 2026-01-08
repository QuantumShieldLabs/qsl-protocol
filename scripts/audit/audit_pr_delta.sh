\
#!/usr/bin/env bash
set -euo pipefail

# Audit helper that focuses on per-PR deltas (merge-commit parent diffs) to avoid
# arbitrary history window requirements.

usage() {
  cat <<'USAGE'
Usage:
  scripts/audit/audit_pr_delta.sh --pr <PR_NUMBER> [--pr <PR_NUMBER> ...] [-o <OUTPUT_PATH>]

Runs a lightweight audit that:
  - preflights gh auth + python3 availability
  - enforces clean working tree (fail-closed)
  - enforces audit-from-main policy (fail-closed)
  - inventories per-PR deltas via merge-commit parent diff (when available)
  - runs local goal-lint via synthesized GITHUB_EVENT_PATH (per PR)
  - runs the formal model checks (if present)
  - prints the first READY NEXT_ACTIONS.md item block

Outputs a markdown report to /tmp by default.
USAGE
}

PRS=()
OUT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --pr)
      PRS+=("$2")
      shift 2
      ;;
    -o|--output)
      OUT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ ${#PRS[@]} -eq 0 ]]; then
  echo "ERROR: at least one --pr <N> is required" >&2
  usage >&2
  exit 2
fi

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

# Clean-tree policy (fail-closed)
if [[ -n "$(git status --porcelain)" ]]; then
  echo "ERROR: working tree is not clean; audit requires a clean repo." >&2
  git status -sb >&2
  exit 1
fi

# Enforce audit-from-main policy (fail-closed)
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$BRANCH" != "main" ]]; then
  echo "ERROR: audit must be run from branch 'main' (current: $BRANCH)." >&2
  git status -sb >&2
  exit 2
fi

TS="$(date -u +%Y%m%dT%H%M%SZ)"
if [[ -z "$OUT" ]]; then
  OUT="/tmp/audit_${TS}.md"
fi

# Prevent accidental in-repo writes (policy: do not write audit artifacts into repo)
REPO_ROOT="$(git rev-parse --show-toplevel)"
case "$OUT" in
  "$REPO_ROOT"/*)
    echo "ERROR: output path is inside repo; choose a path outside the repo (e.g., /tmp/...)." >&2
    echo "  repo_root=$REPO_ROOT" >&2
    echo "  out=$OUT" >&2
    exit 1
    ;;
esac

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

{
  echo "# Local Audit Report (per-PR delta)"
  echo ""
  echo "Generated: ${TS}"
  echo ""

  echo "## Preconditions"
  echo "- gh auth: OK"
  echo "- python3: $(python3 --version 2>&1)"
  echo "- shallow repo: $(git rev-parse --is-shallow-repository 2>/dev/null || echo unknown)"
  echo "- commit count: $(git rev-list --count HEAD 2>/dev/null || echo unknown)"
  echo ""

  echo "## Repo status"
  echo "\`\`\`"
  git status -sb
  echo "\`\`\`"
  echo ""

  for PR in "${PRS[@]}"; do
    echo "## PR #${PR}"
    STATE="$(gh pr view "$PR" --json state -q .state || true)"
    MERGE_OID="$(gh pr view "$PR" --json mergeCommit -q .mergeCommit.oid 2>/dev/null || true)"
    echo "- state: ${STATE:-unknown}"
    echo "- mergeCommit: ${MERGE_OID:-<none>}"
    echo ""

    echo "### PR body (first 20 lines)"
    echo "\`\`\`"
    gh pr view "$PR" --json body -q .body | sed -n '1,20p' || true
    echo "\`\`\`"
    echo ""

    echo "### Delta inventory"
    echo "\`\`\`"
    if [[ -n "$MERGE_OID" ]] && git cat-file -e "$MERGE_OID"^{commit} 2>/dev/null; then
      echo "# merge parent diff: ${MERGE_OID}^1..${MERGE_OID}"
      git diff --name-status "${MERGE_OID}^1..${MERGE_OID}" || true
    else
      BASE_REF_NAME="$(gh pr view "$PR" --json baseRefName -q .baseRefName)"
      HEAD="$(gh pr view "$PR" --json headRefOid -q .headRefOid)"
      git fetch --quiet --all --prune --tags || true
      if BASE_TMP=$(git rev-parse "origin/${BASE_REF_NAME}" 2>/dev/null); then
        BASE="$BASE_TMP"
      elif BASE_TMP=$(git rev-parse "${BASE_REF_NAME}" 2>/dev/null); then
        BASE="$BASE_TMP"
      else
        BASE="<unresolved>"
      fi
      echo "# base/head diff: ${BASE}...${HEAD} (baseRefName=${BASE_REF_NAME})"
      if [[ "$BASE" != "<unresolved>" ]]; then
        git diff --name-status "${BASE}...${HEAD}" || true
      else
        echo "WARN: cannot resolve base SHA locally; skipping base/head diff inventory."
      fi
    fi
    echo "\`\`\`"
    echo ""

    echo "### Local goal-lint"
    echo "\`\`\`"
    "${SCRIPT_DIR}/run_goal_lint_pr.sh" "$PR"
    echo "\`\`\`"
    echo ""
  done

  if [[ -f "formal/run_model_checks.py" ]]; then
    echo "## Formal model checks"
    echo "\`\`\`"
    python3 formal/run_model_checks.py
    echo "\`\`\`"
    echo ""
  else
    echo "## Formal model checks"
    echo "formal/run_model_checks.py not present; skipped."
    echo ""
  fi

  echo "## Next READY item"
  echo "\`\`\`"
  ready_block="$(awk '
    { sub(/\r$/, "", $0) }
    /^[[:space:]]*### NA-/ {
      if (have) { blocks[++n]=section; ready[n]=is_ready }
      section=$0 "\n"; have=1; is_ready=0; next
    }
    have { section = section $0 "\n" }
    have && $0 ~ /Status:[[:space:]]+READY/ { is_ready=1 }
    END {
      if (have) { blocks[++n]=section; ready[n]=is_ready }
      for (i=1;i<=n;i++) if (ready[i]) { printf "%s", blocks[i]; exit }
    }
  ' NEXT_ACTIONS.md)"
  if [[ -z "$ready_block" ]]; then
    echo "No READY item found in NEXT_ACTIONS.md."
  else
    printf "%s\n" "$ready_block"
  fi
  echo "\`\`\`"
} > "$OUT"

echo "Wrote audit report: $OUT" >&2
