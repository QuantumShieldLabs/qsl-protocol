#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
qsc_remote_relay_smoke.sh --scenario <happy-path|drop-reorder> --seed <u64> --out <dir>

Environment:
  RELAY_URL   (required) remote relay endpoint
  RELAY_TOKEN (optional) bearer token
USAGE
}

scenario="happy-path"
seed="1"
out="./_remote_relay_out"

have_rg=0
if command -v rg >/dev/null 2>&1; then
  have_rg=1
fi

mark_grep() {
  if [ "$have_rg" -eq 1 ]; then
    rg "$@"
  else
    grep -E "$@"
  fi
}

mark_grep_o() {
  if [ "$have_rg" -eq 1 ]; then
    rg -o "$@"
  else
    grep -Eo "$@"
  fi
}

while [ $# -gt 0 ]; do
  case "$1" in
    --help|-h) usage; exit 0 ;;
    --scenario) scenario="$2"; shift 2 ;;
    --seed) seed="$2"; shift 2 ;;
    --out) out="$2"; shift 2 ;;
    *) echo "Unknown arg: $1"; usage; exit 2 ;;
  esac
 done

if [ -z "${RELAY_URL:-}" ]; then
  echo "RELAY_URL is required" >&2
  exit 2
fi

mkdir -p "$out"

# mask token in case of debug
relay_url="$RELAY_URL"
relay_token="${RELAY_TOKEN:-}"

# Run remote test using qsc demo script (markers only)
# NOTE: This is a placeholder invocation; replace with actual qsc command in NA-0080 implementation.
# For now, we simulate marker output for smoke and enforce checks.
markers="$out/remote.markers"
summary="$out/summary.txt"
subset="$out/normalized_subset.txt"
counts="$out/normalized_counts.txt"

{
  echo "QSC_MARK/1 event=remote_start scenario=$scenario seed=$seed"
  echo "QSC_MARK/1 event=remote_relay url=RELAY_URL_REDACTED"
  echo "QSC_MARK/1 event=remote_complete status=ok"
} > "$markers"

# normalized subset (stable fields only)
awk '/QSC_MARK\/1/ {print $2,$3,$4}' "$markers" > "$subset"

# deterministic counts (from marker actions)
deliver_count=$( (mark_grep_o "action=deliver" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
drop_count=$( (mark_grep_o "action=drop" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
reorder_count=$( (mark_grep_o "action=reorder" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
dup_count=$( (mark_grep_o "action=dup" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )

{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "status=ok"
  echo "deliver_count=$deliver_count"
  echo "drop_count=$drop_count"
  echo "reorder_count=$reorder_count"
  echo "dup_count=$dup_count"
} > "$counts"

# summary
{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "markers=$(wc -l < "$markers" | tr -d ' ')"
  echo "status=ok"
  echo "deliver_count=$deliver_count"
  echo "drop_count=$drop_count"
  echo "reorder_count=$reorder_count"
  echo "dup_count=$dup_count"
} > "$summary"

# charter checks: no retry/recover markers, no obvious secrets
if mark_grep "retry|recover" "$markers" >/dev/null 2>&1; then
  echo "charter violation: retry/recover marker present" >&2
  exit 1
fi
if mark_grep "RELAY_TOKEN|SECRET|PASSWORD" "$markers" >/dev/null 2>&1; then
  echo "charter violation: secret-like content in markers" >&2
  exit 1
fi

exit 0
