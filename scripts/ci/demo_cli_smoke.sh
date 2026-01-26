#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

cargo build -p qshield-cli --locked
cargo build -p refimpl_actor --locked

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
export QSHIELD_ACTOR="${QSHIELD_ACTOR:-$ROOT_DIR/target/debug/refimpl_actor}"

alice_store="$(mktemp -d)"
bob_store="$(mktemp -d)"
cleanup() {
  if [ -n "${relay_pid:-}" ]; then
    kill "$relay_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$alice_store" "$bob_store"
}
trap cleanup EXIT

./target/debug/qshield init --store "$alice_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"
./target/debug/qshield init --store "$bob_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN"

./target/debug/qshield relay serve --listen "127.0.0.1:${PORT}" &
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

./target/debug/qshield register --store "$alice_store" --id alice
./target/debug/qshield register --store "$bob_store" --id bob

./target/debug/qshield establish --store "$alice_store" --peer bob --demo-unauthenticated-override
./target/debug/qshield establish --store "$bob_store" --peer alice --demo-unauthenticated-override

./target/debug/qshield send --store "$alice_store" --peer bob --text "hello" --demo-unauthenticated-override

recv_out="$(./target/debug/qshield recv --store "$bob_store" --demo-unauthenticated-override)"
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
