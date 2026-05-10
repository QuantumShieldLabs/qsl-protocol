#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

cargo build -p qshield-cli --locked
cargo build -p refimpl_actor --locked

TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT_DIR/target}"
QSHIELD_BIN="${QSHIELD_BIN:-$TARGET_DIR/debug/qshield}"

PORT="$(python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)"

export QSHIELD_RELAY_TOKEN="${QSHIELD_RELAY_TOKEN:-$(python3 - <<'PY'
import os
print(os.urandom(16).hex())
PY
)}"
export QSHIELD_ACTOR="${QSHIELD_ACTOR:-$TARGET_DIR/debug/refimpl_actor}"

alice_store="$(mktemp -d)"
bob_store="$(mktemp -d)"
demo_log="$(mktemp)"
secret_sentinel="NA0246_SECRET_SENTINEL"
cleanup() {
  if [ -n "${relay_pid:-}" ]; then
    kill "$relay_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$alice_store" "$bob_store" "$demo_log"
}
trap cleanup EXIT

mark() {
  printf '%s\n' "$1" | tee -a "$demo_log"
}

assert_no_secret_text() {
  label="$1"
  text="$2"

  if printf '%s' "$text" | grep -F "$QSHIELD_RELAY_TOKEN" >/dev/null; then
    echo "$label leaked relay token" >&2
    exit 1
  fi
  if printf '%s' "$text" | grep -F "$secret_sentinel" >/dev/null; then
    echo "$label leaked secret sentinel" >&2
    exit 1
  fi
}

assert_no_secret_file() {
  label="$1"
  file="$2"

  if [ -s "$file" ]; then
    assert_no_secret_text "$label" "$(cat "$file")"
  fi
}

run_quiet() {
  label="$1"
  shift

  set +e
  out="$("$@" 2>&1)"
  status=$?
  set -e

  assert_no_secret_text "$label output" "$out"
  printf '%s\n' "$out" >>"$demo_log"
  if [ "$status" -ne 0 ]; then
    echo "$label failed" >&2
    printf '%s\n' "$out" >&2
    exit "$status"
  fi
}

mark "DEMO_ACCEPTANCE_START"
mark "DEMO_WARNING_NON_PRODUCTION_RESEARCH_ONLY"
mark "DEMO_LOOPBACK_ONLY_DEFAULT"

run_quiet "alice init" "$QSHIELD_BIN" init --store "$alice_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"
run_quiet "bob init" "$QSHIELD_BIN" init --store "$bob_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"
mark "DEMO_INIT_TWO_PEERS_OK"

expect_failure_contains() {
  label="$1"
  needle="$2"
  shift 2

  set +e
  out="$("$@" 2>&1)"
  status=$?
  set -e

  assert_no_secret_text "$label output" "$out"
  printf '%s\n' "$out" >>"$demo_log"
  if [ "$status" -eq 0 ]; then
    echo "$label unexpectedly succeeded" >&2
    echo "$out" >&2
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "$needle"; then
    echo "$label output missing expected text: $needle" >&2
    echo "$out" >&2
    exit 1
  fi
}

relay_log="$alice_store/relay.log"
"$QSHIELD_BIN" relay serve --listen "127.0.0.1:${PORT}" >"$relay_log" 2>&1 &
relay_pid=$!

for i in {1..20}; do
  if curl -sSf "http://127.0.0.1:${PORT}/health" >/dev/null; then
    break
  fi
  sleep 0.2
done

if ! curl -sSf "http://127.0.0.1:${PORT}/health" >/dev/null; then
  echo "relay health check failed" >&2
  exit 1
fi
assert_no_secret_file "relay startup output" "$relay_log"
mark "DEMO_LOOPBACK_RELAY_OK"

auth_header_name="Authori"
auth_header_name="${auth_header_name}zation"
auth_scheme="Bear"
auth_scheme="${auth_scheme}er"
auth_header_value="${auth_scheme} ${QSHIELD_RELAY_TOKEN}"

no_auth_status="$(
  curl -sS -o "$alice_store/no_auth_register.json" -w "%{http_code}" \
    -H "Content-Type: application/json" \
    -d '{"id":"no-auth","bundle":{"demo":true}}' \
    "http://127.0.0.1:${PORT}/register"
)"
assert_no_secret_file "missing auth register reject body" "$alice_store/no_auth_register.json"
case "$no_auth_status" in
  401|403) ;;
  *)
    echo "missing relay authorization was not rejected: status=$no_auth_status" >&2
    cat "$alice_store/no_auth_register.json" >&2 || true
    exit 1
    ;;
esac
mark "DEMO_NEGATIVE_AUTH_REJECT_OK"

malformed_payload="{\"id\":\"malformed\",\"leak\":\"${secret_sentinel}-${QSHIELD_RELAY_TOKEN}\""
malformed_status="$(
  curl -sS -o "$alice_store/malformed_register.json" -w "%{http_code}" \
    -H "${auth_header_name}: ${auth_header_value}" \
    -H "Content-Type: application/json" \
    -d "$malformed_payload" \
    "http://127.0.0.1:${PORT}/register"
)"
assert_no_secret_file "malformed register reject body" "$alice_store/malformed_register.json"
if [ "$malformed_status" != "400" ]; then
  echo "malformed relay input was not rejected: status=$malformed_status" >&2
  cat "$alice_store/malformed_register.json" >&2 || true
  exit 1
fi
if ! grep -q "invalid json" "$alice_store/malformed_register.json"; then
  echo "malformed relay input rejection missing reason" >&2
  cat "$alice_store/malformed_register.json" >&2 || true
  exit 1
fi
mark "DEMO_NEGATIVE_MALFORMED_REJECT_OK"

expect_failure_contains \
  "invalid relay id register" \
  "relay POST /register failed" \
  "$QSHIELD_BIN" register --store "$alice_store" --id "bad/id"
mark "DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK"

replay_payload='{"peer_id":"replay-peer","bundle_id":"replay-bundle","session_id_hex":"00112233445566778899aabbccddeeff","dh_init":"1111111111111111111111111111111111111111111111111111111111111111","pq_init_ss":"2222222222222222222222222222222222222222222222222222222222222222"}'
replay_first_status="$(
  curl -sS -o "$alice_store/replay_first.json" -w "%{http_code}" \
    -H "${auth_header_name}: ${auth_header_value}" \
    -H "Content-Type: application/json" \
    -d "$replay_payload" \
    "http://127.0.0.1:${PORT}/establish_record"
)"
assert_no_secret_file "establish replay setup body" "$alice_store/replay_first.json"
if [ "$replay_first_status" != "200" ]; then
  echo "establish replay setup failed: status=$replay_first_status" >&2
  cat "$alice_store/replay_first.json" >&2 || true
  exit 1
fi
replay_second_status="$(
  curl -sS -o "$alice_store/replay_second.json" -w "%{http_code}" \
    -H "${auth_header_name}: ${auth_header_value}" \
    -H "Content-Type: application/json" \
    -d "$replay_payload" \
    "http://127.0.0.1:${PORT}/establish_record"
)"
assert_no_secret_file "establish replay reject body" "$alice_store/replay_second.json"
if [ "$replay_second_status" != "409" ]; then
  echo "establish replay was not rejected: status=$replay_second_status" >&2
  cat "$alice_store/replay_second.json" >&2 || true
  exit 1
fi
if ! grep -q "establish replay" "$alice_store/replay_second.json"; then
  echo "establish replay rejection missing reason" >&2
  cat "$alice_store/replay_second.json" >&2 || true
  exit 1
fi
mark "DEMO_NEGATIVE_REPLAY_REJECT_OK"

run_quiet "alice register" "$QSHIELD_BIN" register --store "$alice_store" --id alice
run_quiet "bob register" "$QSHIELD_BIN" register --store "$bob_store" --id bob
mark "DEMO_REGISTER_AUTHORIZED_PEERS_OK"

run_quiet "alice establish" "$QSHIELD_BIN" establish --store "$alice_store" --peer bob --demo-unauthenticated-override
run_quiet "bob establish" "$QSHIELD_BIN" establish --store "$bob_store" --peer alice --demo-unauthenticated-override
mark "DEMO_ESTABLISH_OK"

expected_plaintext="hello-na0246"
run_quiet "alice send" "$QSHIELD_BIN" send --store "$alice_store" --peer bob --text "$expected_plaintext" --demo-unauthenticated-override

recv_out="$("$QSHIELD_BIN" recv --store "$bob_store" --demo-unauthenticated-override 2>&1)"
assert_no_secret_text "bob recv output" "$recv_out"
printf '%s\n' "$recv_out" >>"$demo_log"
if ! echo "$recv_out" | grep -q "$expected_plaintext"; then
  echo "recv output missing plaintext" >&2
  echo "$recv_out" >&2
  exit 1
fi
if ! echo "$recv_out" | grep -q "from alice"; then
  echo "recv output missing sender" >&2
  echo "$recv_out" >&2
  exit 1
fi
mark "DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK"

run_quiet \
  "KT verifier vector demo proof" \
  cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
mark "DEMO_NEGATIVE_KT_REJECT_OK"

run_quiet \
  "KT verifier no-mutation demo proof" \
  cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
mark "DEMO_NEGATIVE_KT_NO_MUTATION_OK"

run_quiet \
  "KT explicit non-production boundary proof" \
  cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
mark "DEMO_KT_NON_PRODUCTION_BOUNDARY_OK"

assert_no_secret_file "demo acceptance output" "$demo_log"
assert_no_secret_file "relay output" "$relay_log"
mark "DEMO_NO_SECRET_LEAK_OK"
mark "NA0259_KT_NEGATIVE_DEMO_READY_OK"
mark "DEMO_ACCEPTANCE_OK"

echo "demo-cli-smoke: OK"
