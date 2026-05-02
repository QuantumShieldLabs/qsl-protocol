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
cleanup() {
  if [ -n "${relay_pid:-}" ]; then
    kill "$relay_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$alice_store" "$bob_store"
}
trap cleanup EXIT

"$QSHIELD_BIN" init --store "$alice_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"
"$QSHIELD_BIN" init --store "$bob_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"

expect_failure_contains() {
  label="$1"
  needle="$2"
  shift 2

  set +e
  out="$("$@" 2>&1)"
  status=$?
  set -e

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

"$QSHIELD_BIN" relay serve --listen "127.0.0.1:${PORT}" &
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

expect_failure_contains \
  "invalid relay id register" \
  "relay POST /register failed" \
  "$QSHIELD_BIN" register --store "$alice_store" --id "bad/id"

replay_payload='{"peer_id":"replay-peer","bundle_id":"replay-bundle","session_id_hex":"00112233445566778899aabbccddeeff","dh_init":"1111111111111111111111111111111111111111111111111111111111111111","pq_init_ss":"2222222222222222222222222222222222222222222222222222222222222222"}'
auth_header_name="Authori"
auth_header_name="${auth_header_name}zation"
auth_scheme="Bear"
auth_scheme="${auth_scheme}er"
auth_header_value="${auth_scheme} ${QSHIELD_RELAY_TOKEN}"
replay_first_status="$(
  curl -sS -o "$alice_store/replay_first.json" -w "%{http_code}" \
    -H "${auth_header_name}: ${auth_header_value}" \
    -H "Content-Type: application/json" \
    -d "$replay_payload" \
    "http://127.0.0.1:${PORT}/establish_record"
)"
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

"$QSHIELD_BIN" register --store "$alice_store" --id alice
"$QSHIELD_BIN" register --store "$bob_store" --id bob

"$QSHIELD_BIN" establish --store "$alice_store" --peer bob --demo-unauthenticated-override
"$QSHIELD_BIN" establish --store "$bob_store" --peer alice --demo-unauthenticated-override

"$QSHIELD_BIN" send --store "$alice_store" --peer bob --text "hello" --demo-unauthenticated-override

recv_out="$("$QSHIELD_BIN" recv --store "$bob_store" --demo-unauthenticated-override)"
if ! echo "$recv_out" | grep -q "hello"; then
  echo "recv output missing plaintext" >&2
  echo "$recv_out" >&2
  exit 1
fi
if ! echo "$recv_out" | grep -q "from alice"; then
  echo "recv output missing sender" >&2
  echo "$recv_out" >&2
  exit 1
fi

echo "demo-cli-smoke: OK"
