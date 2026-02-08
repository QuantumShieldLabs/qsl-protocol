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

peer_alice="$out/peer_alice"
peer_bob="$out/peer_bob"
out_alice="$out/out_alice"
out_bob="$out/out_bob"
mkdir -p "$peer_alice" "$peer_bob" "$out_alice" "$out_bob"
chmod 700 "$peer_alice" "$peer_bob" "$out_alice" "$out_bob"

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

# Normalize env payloads in case secrets are supplied as KEY=value.
relay_url="$(printf '%s' "$RELAY_URL" | sed -E 's/^[[:space:]]*RELAY_URL[[:space:]]*=[[:space:]]*//')"
relay_token="$(printf '%s' "$RELAY_TOKEN" | sed -E 's/^[[:space:]]*RELAY_TOKEN[[:space:]]*=[[:space:]]*//')"

relay_addr="$relay_url"
case "$relay_addr" in
  http://*|https://*) : ;;
  *) relay_addr="http://$relay_addr" ;;
esac

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

run_qsc_step() {
  local actor="$1"
  local step="$2"
  local log_file="$3"
  shift 3
  local tmp="$out/.${actor}_${step}.tmp"
  local home=""
  local peer=""
  if [ "$actor" = "alice" ]; then
    home="$peer_alice"
    peer="bob"
  else
    home="$peer_bob"
    peer="alice"
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
    export QSC_PASSPHRASE="na0108-${actor}-vault-passphrase"
    export RELAY_URL="$relay_addr"
    export RELAY_TOKEN="$relay_token"
    export QSC_RELAY_TOKEN="$relay_token"
    unset QSC_ALLOW_SEED_FALLBACK
    unset QSC_QSP_SEED
    mkdir -p "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    chmod 700 "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    "${qsc_cmd[@]}" "$@"
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
  if run_qsc_step "$actor" vault_init "$log_file" vault init --non-interactive --passphrase-env QSC_PASSPHRASE --key-source passphrase; then
    return 0
  fi
  if mark_grep 'event=error code=vault_exists' "$log_file" >/dev/null 2>&1; then
    return 0
  fi
  echo "vault init failed for $actor" >&2
  exit 1
}

# initialize secure stores and clear stale outboxes
run_vault_init alice "$alice_log"
run_vault_init bob "$bob_log"
run_qsc_step alice pre_abort "$alice_log" send abort
run_qsc_step bob pre_abort "$bob_log" send abort

# four-step handshake over relay inbox
run_qsc_step alice hs_init "$alice_log" handshake init --as alice --peer bob --relay "$relay_addr"
run_qsc_step bob hs_poll_1 "$bob_log" handshake poll --as bob --peer alice --relay "$relay_addr" --max 4
run_qsc_step alice hs_poll_2 "$alice_log" handshake poll --as alice --peer bob --relay "$relay_addr" --max 4
run_qsc_step bob hs_poll_3 "$bob_log" handshake poll --as bob --peer alice --relay "$relay_addr" --max 4

# confirm both sides are established
run_qsc_step alice hs_status "$alice_log" handshake status --peer bob
run_qsc_step bob hs_status "$bob_log" handshake status --peer alice
assert_marker_present 'event=handshake_status status=established peer=bob' "$alice_log" "alice handshake status is not established"
assert_marker_present 'event=handshake_status status=established peer=alice' "$bob_log" "bob handshake status is not established"
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
  run_qsc_step alice "send_ab_${i}" "$alice_log" send --transport relay --relay "$relay_addr" --to bob --file "$alice_payload"
  i=$((i + 1))
done
run_qsc_step bob recv_from_alice "$bob_recv_log" receive --transport relay --relay "$relay_addr" --mailbox bob --from alice --max "$recv_max" --out "$out_bob"

# Re-handshake with bob as initiator to validate reverse-direction live session before bob->alice send.
run_qsc_step bob hs2_init "$bob_log" handshake init --as bob --peer alice --relay "$relay_addr"
run_qsc_step alice hs2_poll_1 "$alice_log" handshake poll --as alice --peer bob --relay "$relay_addr" --max 4
run_qsc_step bob hs2_poll_2 "$bob_log" handshake poll --as bob --peer alice --relay "$relay_addr" --max 4
run_qsc_step alice hs2_poll_3 "$alice_log" handshake poll --as alice --peer bob --relay "$relay_addr" --max 4

i=1
while [ "$i" -le "$send_attempts" ]; do
  run_qsc_step bob "send_ba_${i}" "$bob_log" send --transport relay --relay "$relay_addr" --to alice --file "$bob_payload"
  i=$((i + 1))
done
run_qsc_step alice recv_from_bob "$alice_recv_log" receive --transport relay --relay "$relay_addr" --mailbox alice --from bob --max "$recv_max" --out "$out_alice"

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
sed -E "s/${relay_esc}/RELAY_URL_REDACTED/g; s/${token_esc}/RELAY_TOKEN_REDACTED/g" "$markers" > "$redacted"

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
  echo "normalized_subset_sha256=$(sha256sum "$subset" | awk '{print $1}')"
} > "$summary"

rm -f "$redacted"
exit 0
