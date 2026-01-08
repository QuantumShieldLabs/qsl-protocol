#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

cargo build -p qshield-cli
cargo build -p refimpl_actor
export QSHIELD_ACTOR="${QSHIELD_ACTOR:-$(pwd)/target/debug/refimpl_actor}"

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

tmp_store="$(mktemp -d)"
alice_store="${tmp_store}/alice"
bob_store="${tmp_store}/bob"

PORT_PUBLIC="$(python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)"

set +e
./target/debug/qshield relay serve --listen "0.0.0.0:${PORT_PUBLIC}" >/dev/null 2>&1
rc_public=$?
set -e
if [ "$rc_public" -eq 0 ]; then
  echo "expected non-loopback bind to fail without allow-public" >&2
  exit 1
fi

set +e
./target/debug/qshield relay serve --listen "0.0.0.0:${PORT_PUBLIC}" --allow-public >/dev/null 2>&1
rc_public=$?
set -e
if [ "$rc_public" -eq 0 ]; then
  echo "expected non-loopback bind to fail without explicit unsafe acknowledgement" >&2
  exit 1
fi

./target/debug/qshield relay serve --listen "127.0.0.1:${PORT}" &
relay_pid=$!
cleanup() {
  if [ -n "${relay_pid:-}" ]; then
    kill "$relay_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$tmp_store" || true
}
trap cleanup EXIT

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

./target/debug/qshield init --store "$alice_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN" \
  --padding-enable --padding-buckets "512,1024,2048" >/dev/null
./target/debug/qshield init --store "$bob_store" --relay-url "http://127.0.0.1:${PORT}" --relay-token "$QSHIELD_RELAY_TOKEN" \
  --padding-enable --padding-buckets "512,1024,2048" >/dev/null

dir_mode="$(stat -c %a "$alice_store")"
if [ "$dir_mode" != "700" ]; then
  echo "expected store dir mode 700, got $dir_mode" >&2
  exit 1
fi
cfg_mode="$(stat -c %a "${alice_store}/config.json")"
if [ "$cfg_mode" != "600" ]; then
  echo "expected config mode 600, got $cfg_mode" >&2
  exit 1
fi
state_mode="$(stat -c %a "${alice_store}/state.json")"
if [ "$state_mode" != "600" ]; then
  echo "expected state mode 600, got $state_mode" >&2
  exit 1
fi

./target/debug/qshield register --store "$alice_store" --id alice >/dev/null
./target/debug/qshield register --store "$bob_store" --id bob >/dev/null

payload='{"id":"token-check","bundle":{"demo":true}}'
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  --data "$payload")"
if [ "$code" != "401" ] && [ "$code" != "403" ]; then
  echo "expected 401/403 without token, got $code" >&2
  exit 1
fi

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$payload")"
if [ "$code" != "200" ]; then
  echo "expected 200 with token, got $code" >&2
  exit 1
fi

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/send" \
  -H "Content-Type: application/json" \
  --data '{"to":"alice","from":"bob","msg":"00"}')"
if [ "$code" != "401" ] && [ "$code" != "403" ]; then
  echo "expected 401/403 send without token, got $code" >&2
  exit 1
fi

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/poll" \
  -H "Content-Type: application/json" \
  --data '{"id":"alice","max":1}')"
if [ "$code" != "401" ] && [ "$code" != "403" ]; then
  echo "expected 401/403 poll without token, got $code" >&2
  exit 1
fi

poll_resp="$(curl -s -X POST "http://127.0.0.1:${PORT}/poll" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data '{"id":"alice","max":1}')"
python3 - <<'PY' "$poll_resp"
import json, sys
data = json.loads(sys.argv[1])
msgs = data.get("msgs") or []
if len(msgs) != 0:
    raise SystemExit("expected empty queue after unauth send")
PY

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X GET "http://127.0.0.1:${PORT}/bundle/alice")"
if [ "$code" != "401" ] && [ "$code" != "403" ]; then
  echo "expected 401/403 bundle without token, got $code" >&2
  exit 1
fi

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/consume" \
  -H "Content-Type: application/json" \
  --data '{"id":"alice"}')"
if [ "$code" != "401" ] && [ "$code" != "403" ]; then
  echo "expected 401/403 consume without token, got $code" >&2
  exit 1
fi

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/consume" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data '{"id":"missing"}')"
if [ "$code" != "404" ]; then
  echo "expected 404 consume missing bundle, got $code" >&2
  exit 1
fi

big_payload="$(python3 - <<'PY'
import json
print(json.dumps({"id":"big","bundle":{"blob":"x"*70000}}))
PY
)"
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data-binary "$big_payload")"
if [ "$code" != "413" ]; then
  echo "expected 413 for oversized body, got $code" >&2
  exit 1
fi

demo_dh_pub_missing="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-dh-pub"
peer = b"bob-missing"
print(hashlib.sha256(label + peer).hexdigest())
PY
)"
pq_kem_pub_missing="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-pq-kem-pub"
peer = b"bob-missing"
print(hashlib.sha256(label + peer).hexdigest())
PY
)"
pq_prekey_id_missing="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-pq-prekey-id"
peer = b"bob-missing"
print(int.from_bytes(hashlib.sha256(label + peer).digest()[:4], "big"))
PY
)"
missing_bind_payload="$(python3 - <<PY
import json
print(json.dumps({"id":"bob-missing","bundle":{"dh_pub":"$demo_dh_pub_missing","pq_kem_pub_id":"$pq_kem_pub_missing","pq_prekey_id":$pq_prekey_id_missing,"demo":True}}))
PY
)"
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$missing_bind_payload")"
if [ "$code" != "200" ]; then
  echo "expected 200 for missing binding register, got $code" >&2
  exit 1
fi

set +e
err_missing="$(./target/debug/qshield establish --store "$alice_store" --peer bob-missing --demo-unauthenticated-override 2>&1)"
rc_missing=$?
set -e
if [ "$rc_missing" -eq 0 ]; then
  echo "expected establish to fail with missing identity binding" >&2
  exit 1
fi
echo "$err_missing" | grep -Eq "identity binding|bundle missing identity binding" || {
  echo "expected identity binding error, got: $err_missing" >&2
  exit 1
}
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X GET "http://127.0.0.1:${PORT}/bundle/bob-missing" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN")"
if [ "$code" != "200" ]; then
  echo "expected bundle to remain after missing binding reject, got $code" >&2
  exit 1
fi

demo_dh_pub_mismatch="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-dh-pub"
peer = b"bob-mismatch"
print(hashlib.sha256(label + peer).hexdigest())
PY
)"
pq_kem_pub_mismatch="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-pq-kem-pub"
peer = b"bob-mismatch"
print(hashlib.sha256(label + peer).hexdigest())
PY
)"
pq_prekey_id_mismatch="$(python3 - <<'PY'
import hashlib
label = b"qshield-demo-pq-prekey-id"
peer = b"bob-mismatch"
print(int.from_bytes(hashlib.sha256(label + peer).digest()[:4], "big"))
PY
)"
mismatch_bind_payload="$(python3 - <<PY
import json
print(json.dumps({"id":"bob-mismatch","bundle":{"id":"mallory","dh_pub":"$demo_dh_pub_mismatch","pq_kem_pub_id":"$pq_kem_pub_mismatch","pq_prekey_id":$pq_prekey_id_mismatch,"demo":True}}))
PY
)"
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$mismatch_bind_payload")"
if [ "$code" != "200" ]; then
  echo "expected 200 for mismatched binding register, got $code" >&2
  exit 1
fi

set +e
err_mismatch="$(./target/debug/qshield establish --store "$alice_store" --peer bob-mismatch --demo-unauthenticated-override 2>&1)"
rc_mismatch=$?
set -e
if [ "$rc_mismatch" -eq 0 ]; then
  echo "expected establish to fail with identity mismatch" >&2
  exit 1
fi
echo "$err_mismatch" | grep -Eq "identity mismatch" || {
  echo "expected identity mismatch error, got: $err_mismatch" >&2
  exit 1
}
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X GET "http://127.0.0.1:${PORT}/bundle/bob-mismatch" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN")"
if [ "$code" != "200" ]; then
  echo "expected bundle to remain after identity mismatch reject, got $code" >&2
  exit 1
fi

cap_rc=1
for i in $(seq 1 260); do
  code="$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "http://127.0.0.1:${PORT}/send" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
    --data '{"to":"captest","from":"alice","msg":"00"}')"
  if [ "$code" = "429" ]; then
    cap_rc=0
    break
  fi
done
if [ "$cap_rc" -ne 0 ]; then
  echo "expected queue cap 429 but did not observe it" >&2
  exit 1
fi

set +e
err_establish_first="$(./target/debug/qshield establish --store "$alice_store" --peer bob 2>&1)"
rc_establish=$?
set -e
if [ "$rc_establish" -eq 0 ]; then
  echo "expected establish to fail without demo unauthenticated override" >&2
  exit 1
fi
echo "$err_establish_first" | grep -F "verify peer identity out-of-band before first establish" >/dev/null || {
  echo "expected first-establish identity warning, got: $err_establish_first" >&2
  exit 1
}

set +e
out_establish_auth="$(./target/debug/qshield establish --store "$alice_store" --peer bob --demo-unauthenticated-override --demo-identity-verified 2>&1)"
rc_establish_auth=$?
set -e
if [ "$rc_establish_auth" -ne 0 ]; then
  echo "expected establish to succeed with demo unauthenticated override" >&2
  exit 1
fi
echo "$out_establish_auth" | grep -F "verify peer identity out-of-band before first establish" >/dev/null && {
  echo "expected identity warning suppressed with --demo-identity-verified, got: $out_establish_auth" >&2
  exit 1
}

code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X GET "http://127.0.0.1:${PORT}/bundle/bob" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN")"
if [ "$code" != "404" ]; then
  echo "expected bundle to be consumed after establish, got $code" >&2
  exit 1
fi

./target/debug/qshield register --store "$bob_store" --id bob >/dev/null

set +e
err_replay="$(./target/debug/qshield establish --store "$alice_store" --peer bob --demo-unauthenticated-override 2>&1)"
rc_replay=$?
set -e
if [ "$rc_replay" -eq 0 ]; then
  echo "expected establish replay to fail" >&2
  exit 1
fi
echo "$err_replay" | grep -Eq "replay|establish replay" || {
  echo "expected replay error, got: $err_replay" >&2
  exit 1
}
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X GET "http://127.0.0.1:${PORT}/bundle/bob" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN")"
if [ "$code" != "200" ]; then
  echo "expected bundle to remain after replay reject, got $code" >&2
  exit 1
fi

set +e
./target/debug/qshield send --store "$alice_store" --peer bob --text hi >/dev/null 2>&1
rc_send=$?
set -e
if [ "$rc_send" -eq 0 ]; then
  echo "expected send to fail without demo unauthenticated override" >&2
  exit 1
fi

./target/debug/qshield send --store "$alice_store" --peer bob --text hi --demo-unauthenticated-override >/dev/null

pad_resp="$(curl -s -X POST "http://127.0.0.1:${PORT}/poll" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data '{"id":"bob","max":1}')"
python3 - <<'PY' "$pad_resp"
import json, sys
data = json.loads(sys.argv[1])
msgs = data.get("msgs") or []
if len(msgs) != 1:
    raise SystemExit("expected one padded message in queue")
msg = msgs[0]
pad_len = int(msg.get("pad_len", 0))
bucket = msg.get("bucket")
wire_hex = msg.get("msg", "")
wire_len = len(wire_hex) // 2
if bucket is None:
    raise SystemExit("expected bucket field for padded message")
bucket = int(bucket)
if wire_len != bucket:
    raise SystemExit(f"expected wire_len {wire_len} to match bucket {bucket}")
if pad_len < 0 or pad_len > wire_len:
    raise SystemExit("pad_len out of range")
PY

dup_payload='{"id":"dup-id","bundle":{"demo":true}}'
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$dup_payload")"
if [ "$code" != "200" ]; then
  echo "expected 200 for initial duplicate-id register, got $code" >&2
  exit 1
fi
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$dup_payload")"
if [ "$code" != "409" ]; then
  echo "expected 409 for duplicate-id register, got $code" >&2
  exit 1
fi

invalid_id_payload='{"id":"bad*id","bundle":{"demo":true}}'
code="$(curl -s -o /dev/null -w "%{http_code}" \
  -X POST "http://127.0.0.1:${PORT}/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
  --data "$invalid_id_payload")"
if [ "$code" != "400" ]; then
  echo "expected 400 for invalid id format, got $code" >&2
  exit 1
fi

poll_rl_hit=0
for i in $(seq 1 260); do
  code="$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "http://127.0.0.1:${PORT}/poll" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
    --data '{"id":"rl-poll","max":1}')"
  if [ "$code" = "429" ]; then
    poll_rl_hit=1
    break
  fi
done
if [ "$poll_rl_hit" -ne 1 ]; then
  echo "expected poll rate limit 429 but did not observe it" >&2
  exit 1
fi

register_rl_hit=0
for i in $(seq 1 80); do
  payload="$(printf '{"id":"rl-%s","bundle":{"demo":true}}' "$i")"
  code="$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "http://127.0.0.1:${PORT}/register" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
    --data "$payload")"
  if [ "$code" = "429" ]; then
    register_rl_hit=1
    break
  fi
done
if [ "$register_rl_hit" -ne 1 ]; then
  echo "expected register rate limit 429 but did not observe it" >&2
  exit 1
fi

quota_hit=0
for i in $(seq 1 400); do
  payload="$(printf '{"to":"quota-%s","from":"alice","msg":"00"}' "$i")"
  resp="$(curl -s -w "\n%{http_code}" \
    -X POST "http://127.0.0.1:${PORT}/send" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $QSHIELD_RELAY_TOKEN" \
    --data "$payload")"
  code="$(printf "%s" "$resp" | tail -n1)"
  body="$(printf "%s" "$resp" | head -n -1)"
  if [ "$code" = "429" ]; then
    echo "$body" | grep -F "token quota exceeded" >/dev/null || {
      echo "expected token quota error, got: $body" >&2
      exit 1
    }
    quota_hit=1
    break
  fi
done
if [ "$quota_hit" -ne 1 ]; then
  echo "expected per-token quota 429 but did not observe it" >&2
  exit 1
fi

dir_mode="$(stat -c "%a" "$alice_store")"
cfg_mode="$(stat -c "%a" "$alice_store/config.json")"
state_mode="$(stat -c "%a" "$alice_store/state.json")"
if [ "$dir_mode" != "700" ]; then
  echo "expected store dir mode 700, got $dir_mode" >&2
  exit 1
fi
if [ "$cfg_mode" != "600" ]; then
  echo "expected config mode 600, got $cfg_mode" >&2
  exit 1
fi
if [ "$state_mode" != "600" ]; then
  echo "expected state mode 600, got $state_mode" >&2
  exit 1
fi

./target/debug/qshield rotate --store "$alice_store" >/dev/null
if [ -e "$alice_store/config.json" ] || [ -e "$alice_store/state.json" ]; then
  echo "expected store artifacts removed after rotate" >&2
  exit 1
fi

echo "metadata-conformance-smoke: OK"
