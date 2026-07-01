#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
qsc_remote_handshake_smoke.sh --scenario <happy-path|drop-reorder> --seed <u64> --out <dir>

Environment:
  RELAY_URL   (required) remote relay endpoint
  RELAY_TOKEN (required) bearer token for auth-gated relay
USAGE
}

scenario="happy-path"
seed="1"
out="./_remote_handshake_out"

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
if [ -z "${RELAY_TOKEN:-}" ]; then
  echo "RELAY_TOKEN is required" >&2
  exit 2
fi

umask 077
mkdir -p "$out"
chmod 700 "$out"

# Normalize env payloads in case secrets are supplied as KEY=value.
relay_url="$(printf '%s' "$RELAY_URL" | sed -E 's/^[[:space:]]*RELAY_URL[[:space:]]*=[[:space:]]*//')"
relay_token="$(printf '%s' "$RELAY_TOKEN" | sed -E 's/^[[:space:]]*RELAY_TOKEN[[:space:]]*=[[:space:]]*//')"

relay_addr="$relay_url"
case "$relay_addr" in
  http://*|https://*) : ;;
  *) relay_addr="http://$relay_addr" ;;
esac

# Avoid cross-run mailbox/session collisions on shared remote relays.
run_tag="${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-0}-${scenario}-${seed}"
run_tag="$(printf '%s' "$run_tag" | tr -c 'a-zA-Z0-9_-' '-')"
proto_alice="alice-${run_tag}"
proto_bob="bob-${run_tag}"
alice_route_token="route_token_alice_${run_tag}"
bob_route_token="route_token_bob_${run_tag}"

state_root="$out/state_${run_tag}"
peer_alice="$state_root/peer_alice"
peer_bob="$state_root/peer_bob"
out_alice="$out/out_alice"
out_bob="$out/out_bob"
secret_root="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/qsc_remote_handshake_${run_tag}"
secret_dir="$secret_root/passphrases"
mkdir -p "$peer_alice" "$peer_bob" "$out_alice" "$out_bob" "$secret_dir"
chmod 700 "$state_root" "$peer_alice" "$peer_bob" "$out_alice" "$out_bob" "$secret_root" "$secret_dir"

alice_passphrase_file="$secret_dir/alice.passphrase"
bob_passphrase_file="$secret_dir/bob.passphrase"
printf '%s\n' "na0551-${run_tag}-alice-vault-passphrase" > "$alice_passphrase_file"
printf '%s\n' "na0551-${run_tag}-bob-vault-passphrase" > "$bob_passphrase_file"
chmod 600 "$alice_passphrase_file" "$bob_passphrase_file"

markers="$out/markers"
summary="$out/summary.txt"
subset="$out/normalized_subset.txt"
counts="$out/normalized_counts.txt"
alice_log="$out/alice.log"
bob_log="$out/bob.log"
alice_recv_log="$out/alice_recv.log"
bob_recv_log="$out/bob_recv.log"

: > "$markers"
: > "$summary"
: > "$subset"
: > "$counts"
: > "$alice_log"
: > "$bob_log"
: > "$alice_recv_log"
: > "$bob_recv_log"

if [ -x "target/debug/qsc" ]; then
  qsc_cmd=("target/debug/qsc")
else
  qsc_cmd=("cargo" "run" "-p" "qsc" "--")
fi

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

mark_count() {
  local pattern="$1"
  if [ "$have_rg" -eq 1 ]; then
    rg -c "$pattern" "$markers" || true
  else
    grep -Ec "$pattern" "$markers" || true
  fi
}

marker_values() {
  local key="$1"
  local values=""
  if [ "$have_rg" -eq 1 ]; then
    values=$(
      (rg -o "${key}=[^ ]+" "$markers" 2>/dev/null || true) \
        | sed -E "s/^${key}=//" \
        | sort -u \
        | paste -sd, -
    )
  else
    values=$(
      (grep -Eo "${key}=[^ ]+" "$markers" 2>/dev/null || true) \
        | sed -E "s/^${key}=//" \
        | sort -u \
        | paste -sd, -
    )
  fi
  if [ -z "$values" ]; then
    echo "diagnostic_unavailable"
  else
    echo "$values"
  fi
}

run_qsc_step() {
  local actor="$1"
  local step="$2"
  local log_file="$3"
  shift 3
  local tmp="$out/.${actor}_${step}.tmp"
  local home=""
  local peer=""
  local passphrase_file=""
  if [ "$actor" = "alice" ]; then
    home="$peer_alice"
    peer="bob"
    passphrase_file="$alice_passphrase_file"
  else
    home="$peer_bob"
    peer="alice"
    passphrase_file="$bob_passphrase_file"
  fi

  set +e
  (
    export XDG_CONFIG_HOME="$home/.config"
    export XDG_DATA_HOME="$home/.local/share"
    export XDG_STATE_HOME="$home/.local/state"
    export XDG_CACHE_HOME="$home/.cache"
    export QSC_CONFIG_DIR="$home/.qsc"
    export QSC_SELF_LABEL="$actor"
    export QSC_SCENARIO="$scenario"
    export QSC_SEED="$seed"
    export RELAY_URL="$relay_addr"
    export RELAY_TOKEN="$relay_token"
    export QSC_RELAY_TOKEN="$relay_token"
    export QSC_RELAY_PUSH_DIAGNOSTIC=redacted
    unset QSC_ALLOW_SEED_FALLBACK
    unset QSC_QSP_SEED
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
  if [ "$have_rg" -eq 1 ]; then
    rg '^QSC_MARK/1' "$tmp" | sed -E "s/$/ actor=${actor} peer=${peer} step=${step}/" >> "$markers" || true
  else
    grep -E '^QSC_MARK/1' "$tmp" | sed -E "s/$/ actor=${actor} peer=${peer} step=${step}/" >> "$markers" || true
  fi
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

assert_not_present() {
  local pattern="$1"
  local file="$2"
  local msg="$3"
  if mark_grep "$pattern" "$file" >/dev/null 2>&1; then
    echo "$msg" >&2
    exit 1
  fi
}

extract_recv_commit_count() {
  local file="$1"
  local latest=""
  latest=$( (mark_grep -o 'event=recv_commit count=[0-9]+' "$file" 2>/dev/null || true) | tail -n1 | sed -E 's/.*count=([0-9]+).*/\1/' )
  if [ -z "$latest" ]; then
    echo "0"
  else
    echo "$latest"
  fi
}

run_vault_init() {
  local actor="$1"
  local log_file="$2"
  local passphrase_file=""
  if [ "$actor" = "alice" ]; then
    passphrase_file="$alice_passphrase_file"
  else
    passphrase_file="$bob_passphrase_file"
  fi
  if run_qsc_step "$actor" vault_init "$log_file" vault init --non-interactive --passphrase-file "$passphrase_file" --key-source passphrase; then
    if ! run_qsc_step "$actor" vault_status "$log_file" vault status; then
      echo "vault status failed for $actor after initialization" >&2
      exit 1
    fi
    assert_marker_present 'event=vault_status present=true' "$log_file" "vault status missing after init for $actor"
    return 0
  fi
  if mark_grep 'event=error code=vault_exists' "$log_file" >/dev/null 2>&1; then
    if ! run_qsc_step "$actor" vault_status "$log_file" vault status; then
      echo "vault status failed for $actor after existing vault detection" >&2
      exit 1
    fi
    assert_marker_present 'event=vault_status present=true' "$log_file" "vault status missing after existing vault for $actor"
    return 0
  fi
  echo "vault initialization failed for $actor before relay interaction" >&2
  exit 1
}

run_required_qsc_step() {
  local actor="$1"
  local step="$2"
  local log_file="$3"
  local expected_marker="$4"
  local msg="$5"
  shift 5
  if ! run_qsc_step "$actor" "$step" "$log_file" "$@"; then
    echo "$msg" >&2
    exit 1
  fi
  assert_marker_present "$expected_marker" "$log_file" "$msg"
}

extract_identity_fp() {
  local log_file="$1"
  local actor="$2"
  local fp=""
  fp="$(sed -n -E 's/^identity_fp=([^[:space:]]+).*/\1/p' "$log_file" | tail -n1)"
  if [ -z "$fp" ]; then
    echo "identity fingerprint missing for $actor before relay interaction" >&2
    exit 1
  fi
  printf '%s\n' "$fp"
}

# initialize secure stores and clear stale outboxes
run_vault_init alice "$alice_log"
run_vault_init bob "$bob_log"
run_required_qsc_step alice relay_inbox_set "$alice_log" 'event=relay_inbox_set ok=true' \
  "relay inbox setup failed for alice before handshake" relay inbox-set --token "$alice_route_token"
run_required_qsc_step bob relay_inbox_set "$bob_log" 'event=relay_inbox_set ok=true' \
  "relay inbox setup failed for bob before handshake" relay inbox-set --token "$bob_route_token"
run_required_qsc_step alice identity_rotate "$alice_log" 'event=identity_rotate ok=true' \
  "identity initialization failed for alice before handshake" identity rotate --as "$proto_alice" --confirm
run_required_qsc_step bob identity_rotate "$bob_log" 'event=identity_rotate ok=true' \
  "identity initialization failed for bob before handshake" identity rotate --as "$proto_bob" --confirm
alice_fp="$(extract_identity_fp "$alice_log" alice)"
bob_fp="$(extract_identity_fp "$bob_log" bob)"
run_required_qsc_step alice contacts_add_bob "$alice_log" 'event=contacts_add ok=true' \
  "contact route setup failed for alice before handshake" \
  contacts add --label "$proto_bob" --fp "$bob_fp" --route-token "$bob_route_token"
run_required_qsc_step bob contacts_add_alice "$bob_log" 'event=contacts_add ok=true' \
  "contact route setup failed for bob before handshake" \
  contacts add --label "$proto_alice" --fp "$alice_fp" --route-token "$alice_route_token"
run_required_qsc_step alice contacts_device_list_bob "$alice_log" 'event=contacts_device_list .* count=1' \
  "contact route validation failed for alice before handshake" contacts device list --label "$proto_bob"
run_required_qsc_step bob contacts_device_list_alice "$bob_log" 'event=contacts_device_list .* count=1' \
  "contact route validation failed for bob before handshake" contacts device list --label "$proto_alice"
run_qsc_step alice pre_abort "$alice_log" send abort
run_qsc_step bob pre_abort "$bob_log" send abort

# four-step handshake over relay inbox
run_qsc_step alice hs_init "$alice_log" handshake init --as "$proto_alice" --peer "$proto_bob" --relay "$relay_addr"
run_qsc_step bob hs_poll_1 "$bob_log" handshake poll --as "$proto_bob" --peer "$proto_alice" --relay "$relay_addr" --max 4
run_qsc_step alice hs_poll_2 "$alice_log" handshake poll --as "$proto_alice" --peer "$proto_bob" --relay "$relay_addr" --max 4
run_qsc_step bob hs_poll_3 "$bob_log" handshake poll --as "$proto_bob" --peer "$proto_alice" --relay "$relay_addr" --max 4

# confirm both sides are established
run_qsc_step alice hs_status "$alice_log" handshake status --peer "$proto_bob"
run_qsc_step bob hs_status "$bob_log" handshake status --peer "$proto_alice"
assert_marker_present 'event=handshake_status status=established' "$alice_log" "alice handshake status is not established"
assert_marker_present 'event=handshake_status status=established' "$bob_log" "bob handshake status is not established"
# Derived lane marker: ACTIVE is asserted from established handshake status above.
echo "QSC_MARK/1 event=qsp_status status=ACTIVE reason=handshake actor=alice" >> "$markers"
echo "QSC_MARK/1 event=qsp_status status=ACTIVE reason=handshake actor=bob" >> "$markers"

alice_payload="$out/alice_to_bob.txt"
bob_payload="$out/bob_to_alice.txt"
printf 'hello-from-alice\n' > "$alice_payload"
printf 'hello-from-bob\n' > "$bob_payload"

# bidirectional send+receive
send_attempts=1
recv_max=1
if [ "$scenario" = "drop-reorder" ]; then
  send_attempts=4
  recv_max=8
fi

i=1
while [ "$i" -le "$send_attempts" ]; do
  run_qsc_step alice "send_ab_${i}" "$alice_log" send --transport relay --relay "$relay_addr" --to "$proto_bob" --file "$alice_payload"
  i=$((i + 1))
done
run_qsc_step bob recv_from_alice "$bob_recv_log" receive --transport relay --relay "$relay_addr" --mailbox "$proto_bob" --from "$proto_alice" --max "$recv_max" --out "$out_bob"

# Re-handshake with bob as initiator to validate reverse-direction live session before bob->alice send.
run_qsc_step bob hs2_init "$bob_log" handshake init --as "$proto_bob" --peer "$proto_alice" --relay "$relay_addr"
run_qsc_step alice hs2_poll_1 "$alice_log" handshake poll --as "$proto_alice" --peer "$proto_bob" --relay "$relay_addr" --max 4
run_qsc_step bob hs2_poll_2 "$bob_log" handshake poll --as "$proto_bob" --peer "$proto_alice" --relay "$relay_addr" --max 4
run_qsc_step alice hs2_poll_3 "$alice_log" handshake poll --as "$proto_alice" --peer "$proto_bob" --relay "$relay_addr" --max 4

i=1
while [ "$i" -le "$send_attempts" ]; do
  run_qsc_step bob "send_ba_${i}" "$bob_log" send --transport relay --relay "$relay_addr" --to "$proto_alice" --file "$bob_payload"
  i=$((i + 1))
done
run_qsc_step alice recv_from_bob "$alice_recv_log" receive --transport relay --relay "$relay_addr" --mailbox "$proto_alice" --from "$proto_bob" --max "$recv_max" --out "$out_alice"

# fail-closed assertions
assert_not_present 'event=error code=protocol_inactive' "$markers" "protocol_inactive encountered"
assert_not_present 'code=relay_unauthorized' "$markers" "relay_unauthorized encountered"
assert_marker_present 'event=qsp_pack ok=true' "$alice_log" "missing qsp_pack ok=true for alice->bob send"
assert_marker_present 'event=qsp_pack ok=true' "$bob_log" "missing qsp_pack ok=true for bob->alice send"
assert_marker_present 'event=qsp_unpack ok=true' "$bob_recv_log" "missing qsp_unpack ok=true for bob receive"
assert_marker_present 'event=qsp_unpack ok=true' "$alice_recv_log" "missing qsp_unpack ok=true for alice receive"

bob_recv_commit_count="$(extract_recv_commit_count "$bob_recv_log")"
alice_recv_commit_count="$(extract_recv_commit_count "$alice_recv_log")"
if [ "$bob_recv_commit_count" -lt 1 ]; then
  echo "recv_commit count=0 for bob receive" >&2
  exit 1
fi
if [ "$alice_recv_commit_count" -lt 1 ]; then
  echo "recv_commit count=0 for alice receive" >&2
  exit 1
fi

# redact and normalize deterministic subset
relay_esc=$(printf '%s' "$relay_addr" | sed -e 's/[][(){}.*+?^$|\\/]/\\&/g')
token_esc=$(printf '%s' "$relay_token" | sed -e 's/[][(){}.*+?^$|\\/]/\\&/g')
redacted="$out/.markers.redacted"
sed -E \
  -e "s/${relay_esc}/RELAY_URL_REDACTED/g" \
  -e "s/${token_esc}/RELAY_TOKEN_REDACTED/g" \
  -e "s/${proto_alice}/alice/g" \
  -e "s/${proto_bob}/bob/g" \
  "$markers" > "$redacted"

awk '
  /QSC_MARK\/1/ {
    line=$0
    gsub(/ id=[^ ]+/, "", line)
    gsub(/ sid=[^ ]+/, "", line)
    gsub(/ channel=[^ ]+/, "", line)
    gsub(/ seq=[^ ]+/, "", line)
    gsub(/ idx=[^ ]+/, "", line)
    gsub(/ msg_idx=[^ ]+/, "", line)
    gsub(/ ck_idx=[^ ]+/, "", line)
    print line
  }
' "$redacted" > "$subset"

qsp_pack_ok_count="$(mark_count 'event=qsp_pack ok=true')"
qsp_unpack_ok_count="$(mark_count 'event=qsp_unpack ok=true')"
recv_commit_count="$(mark_count 'event=recv_commit')"
handshake_complete_count="$(mark_count 'event=handshake_complete')"
protocol_inactive_count="$(mark_count 'event=error code=protocol_inactive')"
relay_unauthorized_count="$(mark_count 'code=relay_unauthorized')"
relay_push_diagnostic_count="$(mark_count 'event=relay_push_diagnostic')"
relay_push_diagnostic_classes="$(marker_values diagnostic_class)"
relay_push_timeout_phase_classes="$(marker_values timeout_phase_class)"
relay_push_status_classes="$(marker_values status_class)"

{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "handshake_active_alice=true"
  echo "handshake_active_bob=true"
  echo "qsp_pack_ok_count=$qsp_pack_ok_count"
  echo "qsp_unpack_ok_count=$qsp_unpack_ok_count"
  echo "recv_commit_count=$recv_commit_count"
  echo "recv_commit_bob=$bob_recv_commit_count"
  echo "recv_commit_alice=$alice_recv_commit_count"
  echo "handshake_complete_count=$handshake_complete_count"
  echo "protocol_inactive_count=$protocol_inactive_count"
  echo "relay_unauthorized_count=$relay_unauthorized_count"
  echo "relay_push_diagnostic_count=$relay_push_diagnostic_count"
  echo "relay_push_diagnostic_classes=$relay_push_diagnostic_classes"
  echo "relay_push_timeout_phase_classes=$relay_push_timeout_phase_classes"
  echo "relay_push_status_classes=$relay_push_status_classes"
} > "$counts"

{
  echo "status=pass"
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "handshake=ACTIVE(reason=handshake) both_peers"
  echo "qsp_pack_ok=true both_directions"
  echo "qsp_unpack_ok=true both_directions"
  echo "recv_commit_bob=$bob_recv_commit_count"
  echo "recv_commit_alice=$alice_recv_commit_count"
  echo "marker_lines=$(wc -l < "$markers" | tr -d ' ')"
  echo "relay_push_diagnostic_count=$relay_push_diagnostic_count"
  echo "relay_push_diagnostic_classes=$relay_push_diagnostic_classes"
  echo "relay_push_timeout_phase_classes=$relay_push_timeout_phase_classes"
  echo "relay_push_status_classes=$relay_push_status_classes"
  echo "normalized_subset_sha256=$(sha256sum "$subset" | awk '{print $1}')"
} > "$summary"

rm -f "$redacted"
exit 0
