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
    *) echo "Unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

case "$scenario" in
  happy-path|drop-reorder) ;;
  *) echo "invalid scenario: $scenario" >&2; exit 2 ;;
esac

if [ -z "${RELAY_URL:-}" ]; then
  echo "RELAY_URL is required" >&2
  exit 2
fi

umask 077
mkdir -p "$out"
chmod 700 "$out"

# Mask token in case of debug output.
relay_url="$(printf '%s' "$RELAY_URL" | sed -E 's/^[[:space:]]*RELAY_URL[[:space:]]*=[[:space:]]*//')"
relay_token="$(printf '%s' "${RELAY_TOKEN:-}" | sed -E 's/^[[:space:]]*RELAY_TOKEN[[:space:]]*=[[:space:]]*//')"

# Normalize relay URL for qsc relay inbox base URL.
relay_addr="$relay_url"
case "$relay_addr" in
  http://*|https://*) : ;;
  *) relay_addr="http://$relay_addr" ;;
esac

# Isolated, per-run qsc state to avoid polluting real config or reusing stale state.
run_tag="${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-0}-${scenario}-${seed}"
run_tag="$(printf '%s' "$run_tag" | tr -c 'a-zA-Z0-9_-' '-')"
qsc_home="$out/qsc_state_${run_tag}"
secret_dir="$qsc_home/passphrases"
mkdir -p "$qsc_home/.config" "$qsc_home/.local/share" "$qsc_home/.local/state" "$qsc_home/.cache" "$secret_dir"
chmod 700 "$qsc_home" "$qsc_home/.config" "$qsc_home/.local/share" "$qsc_home/.local/state" "$qsc_home/.cache" "$secret_dir"

passphrase_file="$secret_dir/relay.passphrase"
printf '%s\n' "na0551-${run_tag}-relay-vault-passphrase" > "$passphrase_file"
chmod 600 "$passphrase_file"

bob_route_token="route_token_bob_${run_tag}"

payload="$out/payload.txt"
echo "hello" > "$payload"

markers="$out/remote.markers"
summary="$out/summary.txt"
subset="$out/normalized_subset.txt"
counts="$out/normalized_counts.txt"
status="ok"

if [ -x "target/debug/qsc" ]; then
  qsc_cmd=("target/debug/qsc")
else
  qsc_cmd=("cargo" "run" "-p" "qsc" "--")
fi

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

run_qsc_step() {
  local step="$1"
  local log_file="$2"
  shift 2
  local tmp="$out/.${step}.tmp"

  set +e
  (
    export QSC_SCENARIO="$scenario"
    export QSC_SEED="$seed"
    export QSC_QSP_SEED="$seed"
    export QSC_ALLOW_SEED_FALLBACK=1
    export XDG_CONFIG_HOME="$qsc_home/.config"
    export XDG_DATA_HOME="$qsc_home/.local/share"
    export XDG_STATE_HOME="$qsc_home/.local/state"
    export XDG_CACHE_HOME="$qsc_home/.cache"
    export QSC_CONFIG_DIR="$qsc_home/.qsc"
    if [ -n "$relay_token" ]; then
      export RELAY_TOKEN="$relay_token"
      export QSC_RELAY_TOKEN="$relay_token"
    else
      unset RELAY_TOKEN
      unset QSC_RELAY_TOKEN
    fi
    mkdir -p "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    chmod 700 "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    if [ "$step" = "vault_init" ]; then
      "${qsc_cmd[@]}" "$@"
    else
      "${qsc_cmd[@]}" --unlock-passphrase-file "$passphrase_file" "$@"
    fi
  ) >"$tmp" 2>&1
  local rc=$?
  set -e

  cat "$tmp" >> "$log_file"
  emit_markers_from_log "$tmp"
  rm -f "$tmp"
  return "$rc"
}

assert_marker_present() {
  local pattern="$1"
  local file="$2"
  local msg="$3"
  if ! mark_grep "$pattern" "$file" >/dev/null 2>&1; then
    echo "$msg" >&2
    exit 1
  fi
}

run_setup_step() {
  local step="$1"
  local expected_marker="$2"
  local msg="$3"
  shift 3
  local log_file="$out/setup_${step}.log"
  if ! run_qsc_step "$step" "$log_file" "$@"; then
    echo "$msg" >&2
    exit 1
  fi
  assert_marker_present "$expected_marker" "$log_file" "$msg"
}

run_send_once() {
  local idx="$1"
  local log_file="$out/send_${idx}.log"
  local rc=0
  if run_qsc_step "send_${idx}" "$log_file" send --transport relay --relay "$relay_addr" --to bob --file "$payload"; then
    rc=0
  else
    rc=$?
  fi
  echo "$rc" > "$out/send_${idx}.rc"
  return "$rc"
}

run_abort() {
  local idx="$1"
  local log_file="$out/abort_${idx}.log"
  run_qsc_step "abort_${idx}" "$log_file" send abort
}

ensure_outbox_clear() {
  run_abort "pre"
}

initialize_remote_relay_state() {
  run_setup_step vault_init 'event=vault_init' "vault initialization failed before remote relay interaction" \
    vault init --non-interactive --key-source passphrase --passphrase-file "$passphrase_file"
  run_setup_step vault_status 'event=vault_status present=true' "vault status missing after remote relay initialization" \
    vault status
  run_setup_step contacts_add 'event=contacts_add ok=true' "contact-store initialization failed before remote relay interaction" \
    contacts add --label bob --fp fp-remote-relay-bob --route-token "$bob_route_token"
  run_setup_step contacts_device_list 'event=contacts_device_list label=bob count=1' "contact-store validation failed before remote relay interaction" \
    contacts device list --label bob
}

{
  echo "QSC_MARK/1 event=remote_start scenario=$scenario seed=$seed"
  echo "QSC_MARK/1 event=protocol_mode mode=seed_fallback_test"
  echo "QSC_MARK/1 event=remote_relay url=RELAY_URL_REDACTED"
} > "$markers"

initialize_remote_relay_state
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

if mark_grep "event=error code=protocol_inactive" "$markers" >/dev/null 2>&1; then
  echo "protocol_inactive encountered in remote relay smoke lane" >&2
  exit 1
fi
if mark_grep "event=error code=contacts_store_invalid" "$markers" >/dev/null 2>&1; then
  echo "contacts_store_invalid encountered after deterministic contact-store setup" >&2
  exit 1
fi

# normalized subset (stable fields only)
awk '/QSC_MARK\/1/ {print $2,$3,$4,$5,$6}' "$markers" > "$subset"

# deterministic counts (from marker actions)
deliver_count=$( (mark_grep_o "action=deliver" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
drop_count=$( (mark_grep_o "action=drop" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
reorder_count=$( (mark_grep_o "action=reorder" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )
dup_count=$( (mark_grep_o "action=dup" "$markers" 2>/dev/null || true) | wc -l | tr -d ' ' )

expected_deliver_min=1
expected_drop_count=0
expected_reorder_count=0
expected_dup_count=0
expected_drop_or_reorder_min=0
if [ "$scenario" = "drop-reorder" ]; then
  expected_drop_count=-1
  expected_reorder_count=-1
  expected_drop_or_reorder_min=1
fi

{
  echo "protocol_mode=seed_fallback_test"
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "status=$status"
  echo "expected_deliver_min=$expected_deliver_min"
  echo "expected_drop_count=$expected_drop_count"
  echo "expected_reorder_count=$expected_reorder_count"
  echo "expected_dup_count=$expected_dup_count"
  echo "expected_drop_or_reorder_min=$expected_drop_or_reorder_min"
  echo "deliver_count=$deliver_count"
  echo "drop_count=$drop_count"
  echo "reorder_count=$reorder_count"
  echo "dup_count=$dup_count"
} > "$counts"

# summary
{
  echo "protocol_mode=seed_fallback_test"
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
  if [ "$deliver_count" -lt "$expected_deliver_min" ] || [ "$drop_count" -ne "$expected_drop_count" ] || [ "$reorder_count" -ne "$expected_reorder_count" ] || [ "$dup_count" -ne "$expected_dup_count" ]; then
    echo "counts failed happy-path expectations" >&2
    exit 1
  fi
else
  if [ "$deliver_count" -lt "$expected_deliver_min" ] || [ $((drop_count + reorder_count)) -lt "$expected_drop_or_reorder_min" ]; then
    echo "counts failed drop-reorder expectations" >&2
    exit 1
  fi
fi

if [ "$status" != "ok" ]; then
  exit 1
fi

exit 0
