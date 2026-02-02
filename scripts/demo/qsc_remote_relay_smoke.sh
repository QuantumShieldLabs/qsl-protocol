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

# Normalize relay URL to host:port for qsc (strip scheme/path)
relay_addr="$relay_url"
case "$relay_addr" in
  http://*|https://*)
    relay_addr="${relay_addr#*://}"
    relay_addr="${relay_addr%%/*}"
    ;;
esac

# Isolated qsc state to avoid polluting real config
qsc_home="$out/qsc_state"
mkdir -p "$qsc_home/.config" "$qsc_home/.local/share" "$qsc_home/.local/state" "$qsc_home/.cache"
chmod 700 "$qsc_home" "$qsc_home/.config" "$qsc_home/.local/share" "$qsc_home/.local/state" "$qsc_home/.cache"

payload="$out/payload.txt"
echo "hello" > "$payload"

markers="$out/remote.markers"
summary="$out/summary.txt"
subset="$out/normalized_subset.txt"
counts="$out/normalized_counts.txt"
status="ok"

# Determine bounded number of sends to make hostile scenario observable
send_count=2
if [ "$scenario" = "drop-reorder" ]; then
  send_count=8
fi

emit_markers_from_log() {
  local log_file="$1"
  if [ -f "$log_file" ]; then
    mark_grep "^QSC_MARK/1" "$log_file" >> "$markers" || true
  fi
}

run_send_once() {
  local idx="$1"
  local log_file="$out/send_${idx}.log"
  local rc=0
  (
    export QSC_SCENARIO="$scenario"
    export QSC_SEED="$seed"
    export XDG_CONFIG_HOME="$qsc_home/.config"
    export XDG_DATA_HOME="$qsc_home/.local/share"
    export XDG_STATE_HOME="$qsc_home/.local/state"
    export XDG_CACHE_HOME="$qsc_home/.cache"
    cargo run -p qsc -- send --transport relay --relay "$relay_addr" --to bob --file "$payload"
  ) 2>&1 | tee "$log_file"
  rc=${PIPESTATUS[0]}
  echo "$rc" > "$out/send_${idx}.rc"
  emit_markers_from_log "$log_file"
  return "$rc"
}

run_abort() {
  local idx="$1"
  local log_file="$out/abort_${idx}.log"
  (
    export XDG_CONFIG_HOME="$qsc_home/.config"
    export XDG_DATA_HOME="$qsc_home/.local/share"
    export XDG_STATE_HOME="$qsc_home/.local/state"
    export XDG_CACHE_HOME="$qsc_home/.cache"
    cargo run -p qsc -- send abort
  ) 2>&1 | tee "$log_file"
  emit_markers_from_log "$log_file"
}

ensure_outbox_clear() {
  run_abort "pre"
}
{
  echo "QSC_MARK/1 event=remote_start scenario=$scenario seed=$seed"
  echo "QSC_MARK/1 event=remote_relay url=RELAY_URL_REDACTED"
} > "$markers"

ensure_outbox_clear

for i in $(seq 1 "$send_count"); do
  if run_send_once "$i"; then
    true
  else
    if [ "$scenario" = "happy-path" ]; then
      run_abort "$i"
      if run_send_once "${i}_retry"; then
        true
      else
        status="fail"
        break
      fi
    fi
    run_abort "$i"
  fi
done

echo "QSC_MARK/1 event=remote_complete status=$status" >> "$markers"

# normalized subset (stable fields only)
awk '/QSC_MARK\/1/ {print $2,$3,$4,$5,$6}' "$markers" > "$subset"

# deterministic counts (from marker actions)
deliver_count=$( (mark_grep_o "action=deliver" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
drop_count=$( (mark_grep_o "action=drop" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
reorder_count=$( (mark_grep_o "action=reorder" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
dup_count=$( (mark_grep_o "action=dup" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )

{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "status=$status"
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
  echo "status=$status"
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
if [ "$scenario" = "happy-path" ]; then
  if [ "$deliver_count" -le 0 ] || [ "$drop_count" -ne 0 ] || [ "$reorder_count" -ne 0 ] || [ "$dup_count" -ne 0 ]; then
    echo "counts failed happy-path expectations" >&2
    exit 1
  fi
else
  if [ "$deliver_count" -le 0 ] || [ "$drop_count" -le 0 -a "$reorder_count" -le 0 ]; then
    echo "counts failed drop-reorder expectations" >&2
    exit 1
  fi
fi

if [ "$status" != "ok" ]; then
  exit 1
fi

exit 0
