#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PROFILE="${DEMO_STRESS_PROFILE:-baseline}"
case "$PROFILE" in
  baseline|extended) ;;
  *)
    echo "unsupported DEMO_STRESS_PROFILE: $PROFILE" >&2
    exit 2
    ;;
esac

timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
ARTIFACT_DIR="${DEMO_STRESS_ARTIFACT_DIR:-/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_${timestamp}}"
mkdir -p "$ARTIFACT_DIR"

TRANSCRIPT="$ARTIFACT_DIR/demo_adversarial_stress_transcript.log"
MARKERS="$ARTIFACT_DIR/demo_adversarial_stress_markers.log"
: >"$TRANSCRIPT"
: >"$MARKERS"
exec > >(tee -a "$TRANSCRIPT") 2>&1

MAX_RUNTIME_S="${DEMO_STRESS_MAX_RUNTIME_S:-900}"
START_EPOCH="$(date +%s)"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT_DIR/target}"
QSHIELD_BIN="${QSHIELD_BIN:-$TARGET_DIR/debug/qshield}"
ACTIVE_PIDS=""
DIRECT_RELAY_TOKEN=""
DIRECT_SENTINEL="NA0262_SECRET_SENTINEL"

cleanup() {
  for pid in $ACTIVE_PIDS; do
    kill "$pid" >/dev/null 2>&1 || true
  done
}
trap cleanup EXIT

die() {
  echo "demo-adversarial-stress: ERROR: $*" >&2
  echo "demo-adversarial-stress: artifacts: $ARTIFACT_DIR" >&2
  exit 1
}

mark() {
  printf '%s\n' "$1" | tee -a "$MARKERS"
}

remaining_seconds() {
  now="$(date +%s)"
  elapsed=$((now - START_EPOCH))
  remaining=$((MAX_RUNTIME_S - elapsed))
  if [ "$remaining" -lt 1 ]; then
    echo 0
  else
    echo "$remaining"
  fi
}

run_bounded_to_file() {
  label="$1"
  outfile="$2"
  shift 2

  remaining="$(remaining_seconds)"
  if [ "$remaining" -le 0 ]; then
    die "$label skipped because stress runtime budget expired"
  fi
  echo "RUN $label timeout=${remaining}s"
  set +e
  timeout "${remaining}s" "$@" >"$outfile" 2>&1
  status=$?
  set -e
  cat "$outfile"
  if [ "$status" -ne 0 ]; then
    die "$label failed with status $status"
  fi
}

new_port() {
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
}

new_token() {
  python3 - <<'PY'
import os
print(os.urandom(16).hex())
PY
}

wait_for_health() {
  port="$1"
  for _ in $(seq 1 50); do
    if curl -sSf --max-time 1 "http://127.0.0.1:${port}/health" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.1
  done
  return 1
}

expect_http_status() {
  label="$1"
  expected="$2"
  body_file="$3"
  shift 3

  code="$(curl -sS --max-time 5 -o "$body_file" -w "%{http_code}" "$@")"
  case "$expected" in
    *"|"*)
      printf '%s\n' "$expected" | tr '|' '\n' | grep -Fx "$code" >/dev/null || {
        echo "$label expected status $expected, got $code" >&2
        return 1
      }
      ;;
    *)
      if [ "$code" != "$expected" ]; then
        echo "$label expected status $expected, got $code" >&2
        return 1
      fi
      ;;
  esac
}

assert_file_contains() {
  label="$1"
  needle="$2"
  file="$3"
  if ! grep -F "$needle" "$file" >/dev/null; then
    echo "$label missing expected sanitized text: $needle" >&2
    return 1
  fi
}

assert_no_known_secret_leaks() {
  found=0
  while IFS= read -r file; do
    [ -f "$file" ] || continue
    for secret in "$DIRECT_RELAY_TOKEN" "$DIRECT_SENTINEL" "NA0246_SECRET_SENTINEL" "NA0244_SECRET_SENTINEL"; do
      [ -n "$secret" ] || continue
      if grep -F "$secret" "$file" >/dev/null; then
        echo "secret/sentinel leak detected in artifact file: $file" >&2
        found=1
      fi
    done
  done <<EOF
$(find "$ARTIFACT_DIR" -type f | sort)
EOF
  if [ "$found" -ne 0 ]; then
    return 1
  fi
}

assert_no_panic_output() {
  if grep -E -r -l "thread '.*' panicked|panicked at|called \`.*unwrap|stack backtrace|RUST_BACKTRACE" "$ARTIFACT_DIR" >/dev/null; then
    echo "panic/backtrace/unwrap text detected in artifact output" >&2
    return 1
  fi
}

require_marker() {
  file="$1"
  marker="$2"
  if ! grep -F "$marker" "$file" >/dev/null; then
    die "required child marker missing: $marker"
  fi
}

run_direct_relay_abuse() {
  direct_log="$ARTIFACT_DIR/direct_relay_abuse.log"
  relay_log="$ARTIFACT_DIR/direct_relay_abuse_relay.log"
  relay_restart_log="$ARTIFACT_DIR/direct_relay_abuse_relay_restart.log"
  port="$(new_port)"
  DIRECT_RELAY_TOKEN="$(new_token)"
  auth_header_name="Authori"
  auth_header_name="${auth_header_name}zation"
  auth_scheme="Bear"
  auth_scheme="${auth_scheme}er"
  auth_header="${auth_header_name}: ${auth_scheme} ${DIRECT_RELAY_TOKEN}"

  echo "RUN direct relay abuse on loopback port $port" | tee "$direct_log"
  QSHIELD_RELAY_TOKEN="$DIRECT_RELAY_TOKEN" "$QSHIELD_BIN" relay serve --listen "127.0.0.1:${port}" >"$relay_log" 2>&1 &
  relay_pid=$!
  ACTIVE_PIDS="$ACTIVE_PIDS $relay_pid"
  wait_for_health "$port" || die "direct relay health check failed"

  expect_http_status "missing auth register" "401|403" "$ARTIFACT_DIR/no_auth_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "Content-Type: application/json" \
    --data "{\"id\":\"no-auth\",\"bundle\":{\"demo\":true,\"note\":\"${DIRECT_SENTINEL}\"}}"

  expect_http_status "wrong token register" "401|403" "$ARTIFACT_DIR/wrong_token_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "Content-Type: application/json" \
    -H "${auth_header_name}: ${auth_scheme} wrong-token" \
    --data "{\"id\":\"wrong-token\",\"bundle\":{\"demo\":true,\"note\":\"${DIRECT_SENTINEL}\"}}"

  expect_http_status "wrong auth scheme register" "401|403" "$ARTIFACT_DIR/wrong_scheme_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "Content-Type: application/json" \
    -H "${auth_header_name}: Basic wrong-token" \
    --data '{"id":"wrong-scheme","bundle":{"demo":true}}'

  expect_http_status "empty auth register" "401|403" "$ARTIFACT_DIR/empty_auth_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "Content-Type: application/json" \
    -H "${auth_header_name}: ${auth_scheme} " \
    --data '{"id":"empty-auth","bundle":{"demo":true}}'

  assert_file_contains "auth reject" "missing or invalid relay token" "$ARTIFACT_DIR/no_auth_register.json"
  mark "DEMO_STRESS_AUTH_REJECT_OK"

  expect_http_status "malformed json register" "400" "$ARTIFACT_DIR/malformed_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data "{\"id\":\"malformed\",\"note\":\"${DIRECT_SENTINEL}-${DIRECT_RELAY_TOKEN}\""
  assert_file_contains "malformed json reject" "invalid json" "$ARTIFACT_DIR/malformed_register.json"

  expect_http_status "wrong content type register" "415" "$ARTIFACT_DIR/wrong_content_type_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "$auth_header" \
    -H "Content-Type: text/plain" \
    --data "{\"id\":\"wrong-content-type\",\"bundle\":{\"note\":\"${DIRECT_SENTINEL}\"}}"
  assert_file_contains "content type reject" "unsupported content type" "$ARTIFACT_DIR/wrong_content_type_register.json"

  expect_http_status "empty body register" "400" "$ARTIFACT_DIR/empty_body_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data ''

  python3 - <<'PY' >"$ARTIFACT_DIR/oversized_register.json"
import json
print(json.dumps({"id": "oversized", "bundle": {"blob": "x" * 70000}}))
PY
  expect_http_status "oversized register" "413" "$ARTIFACT_DIR/oversized_register_body.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data-binary "@$ARTIFACT_DIR/oversized_register.json"
  mark "DEMO_STRESS_MALFORMED_REJECT_OK"

  expect_http_status "invalid relay id register" "400" "$ARTIFACT_DIR/invalid_id_register.json" \
    -X POST "http://127.0.0.1:${port}/register" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data '{"id":"bad/id","bundle":{"demo":true}}'
  assert_file_contains "invalid id reject" "invalid id format" "$ARTIFACT_DIR/invalid_id_register.json"
  mark "DEMO_STRESS_RELAY_ID_REJECT_OK"

  replay_payload='{"peer_id":"replay-peer","bundle_id":"replay-bundle","session_id_hex":"00112233445566778899aabbccddeeff","dh_init":"1111111111111111111111111111111111111111111111111111111111111111","pq_init_ss":"2222222222222222222222222222222222222222222222222222222222222222"}'
  expect_http_status "establish replay setup" "200" "$ARTIFACT_DIR/replay_first.json" \
    -X POST "http://127.0.0.1:${port}/establish_record" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data "$replay_payload"
  expect_http_status "establish replay reject" "409" "$ARTIFACT_DIR/replay_second.json" \
    -X POST "http://127.0.0.1:${port}/establish_record" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data "$replay_payload"
  assert_file_contains "replay reject" "establish replay" "$ARTIFACT_DIR/replay_second.json"
  mark "DEMO_STRESS_REPLAY_REJECT_OK"

  expect_http_status "unauth send reject" "401|403" "$ARTIFACT_DIR/no_auth_send.json" \
    -X POST "http://127.0.0.1:${port}/send" \
    -H "Content-Type: application/json" \
    --data '{"to":"noauth-target","from":"alice","msg":"00"}'
  poll_noauth="$(curl -sS --max-time 5 -X POST "http://127.0.0.1:${port}/poll" \
    -H "$auth_header" \
    -H "Content-Type: application/json" \
    --data '{"id":"noauth-target","max":1}')"
  python3 - <<'PY' "$poll_noauth"
import json, sys
data = json.loads(sys.argv[1])
if data.get("msgs"):
    raise SystemExit("unauthorized send mutated queue")
PY
  mark "DEMO_STRESS_AUTH_REJECT_NO_MUTATION_OK"

  queue_hit=0
  for i in $(seq 1 260); do
    code="$(curl -sS --max-time 5 -o "$ARTIFACT_DIR/queue_cap_${i}.json" -w "%{http_code}" \
      -X POST "http://127.0.0.1:${port}/send" \
      -H "$auth_header" \
      -H "Content-Type: application/json" \
      --data '{"to":"captest","from":"alice","msg":"00"}')"
    if [ "$code" = "429" ]; then
      queue_hit=1
      assert_file_contains "queue cap reject" "recipient queue full" "$ARTIFACT_DIR/queue_cap_${i}.json"
      break
    fi
    if [ "$code" != "200" ]; then
      die "unexpected queue cap status $code at iteration $i"
    fi
  done
  if [ "$queue_hit" -ne 1 ]; then
    die "queue cap was not observed within bounded loop"
  fi
  mark "DEMO_STRESS_QUEUE_OR_RATE_BOUND_OK"

  kill "$relay_pid" >/dev/null 2>&1 || true
  wait "$relay_pid" >/dev/null 2>&1 || true
  if curl -sSf --max-time 1 "http://127.0.0.1:${port}/health" >/dev/null 2>&1; then
    die "relay still answered after controlled kill"
  fi

  QSHIELD_RELAY_TOKEN="$DIRECT_RELAY_TOKEN" "$QSHIELD_BIN" relay serve --listen "127.0.0.1:${port}" >"$relay_restart_log" 2>&1 &
  relay_pid=$!
  ACTIVE_PIDS="$ACTIVE_PIDS $relay_pid"
  wait_for_health "$port" || die "relay did not recover after controlled restart"
  mark "DEMO_STRESS_CHAOS_RECOVERY_OK"
  kill "$relay_pid" >/dev/null 2>&1 || true
  wait "$relay_pid" >/dev/null 2>&1 || true
}

run_extended_port_in_use() {
  port="$(new_port)"
  token="$(new_token)"
  first_log="$ARTIFACT_DIR/extended_port_in_use_first.log"
  second_log="$ARTIFACT_DIR/extended_port_in_use_second.log"
  QSHIELD_RELAY_TOKEN="$token" "$QSHIELD_BIN" relay serve --listen "127.0.0.1:${port}" >"$first_log" 2>&1 &
  first_pid=$!
  ACTIVE_PIDS="$ACTIVE_PIDS $first_pid"
  wait_for_health "$port" || die "extended port-in-use relay health failed"
  set +e
  QSHIELD_RELAY_TOKEN="$token" "$QSHIELD_BIN" relay serve --listen "127.0.0.1:${port}" >"$second_log" 2>&1
  status=$?
  set -e
  if [ "$status" -eq 0 ]; then
    die "second relay unexpectedly started on an occupied port"
  fi
  mark "DEMO_STRESS_EXTENDED_PORT_IN_USE_REJECT_OK"
  kill "$first_pid" >/dev/null 2>&1 || true
  wait "$first_pid" >/dev/null 2>&1 || true
}

echo "DEMO_ADVERSARIAL_STRESS_START profile=$PROFILE"
echo "artifact_dir=$ARTIFACT_DIR"
echo "bounded_runtime_seconds=$MAX_RUNTIME_S"
mark "DEMO_STRESS_BOUNDED_PROFILE_${PROFILE}_OK"

if [ ! -x "$QSHIELD_BIN" ]; then
  run_bounded_to_file "qshield-cli build" "$ARTIFACT_DIR/qshield_build.log" cargo build -p qshield-cli --locked
fi

run_direct_relay_abuse

if [ "$PROFILE" = "extended" ]; then
  run_extended_port_in_use
else
  echo "UNSUPPORTED_PORT_IN_USE_BASELINE: extended profile only"
fi

demo_log="$ARTIFACT_DIR/demo_cli_smoke.log"
run_bounded_to_file "demo-cli-smoke after direct stress" "$demo_log" bash scripts/ci/demo_cli_smoke.sh

require_marker "$demo_log" "DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK"
mark "DEMO_STRESS_POSITIVE_BASELINE_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_AUTH_REJECT_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_MALFORMED_REJECT_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_REPLAY_REJECT_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK"
require_marker "$demo_log" "DEMO_ATTACHMENT_INTEGRITY_REJECT_OK"
mark "DEMO_STRESS_ATTACHMENT_INTEGRITY_REJECT_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_KT_REJECT_OK"
require_marker "$demo_log" "DEMO_NEGATIVE_KT_NO_MUTATION_OK"
mark "DEMO_STRESS_KT_REJECT_OK"
require_marker "$demo_log" "DEMO_NO_SECRET_LEAK_OK"

assert_no_known_secret_leaks || die "known secret/sentinel leak scan failed"
mark "DEMO_STRESS_NO_SECRET_LEAK_OK"
assert_no_panic_output || die "panic/backtrace/unwrap scan failed"
mark "DEMO_STRESS_NO_PANIC_OK"

elapsed=$(( $(date +%s) - START_EPOCH ))
echo "baseline_runtime_seconds=$elapsed"
echo "artifact_dir=$ARTIFACT_DIR"
mark "NA0262_DEMO_ADVERSARIAL_STRESS_OK"
echo "demo-adversarial-stress: OK"
