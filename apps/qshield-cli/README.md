# qshield CLI (Demo Only)

Goals: G1, G2, G3, G4, G5

This is a **non-production demo** CLI for QuantumShield (Suite-2 only).
It is intended for local demonstrations and must not be used as a production client.

## Demo in 5 minutes

```sh
cargo build -p qshield-cli
./target/debug/qshield --help
export QSHIELD_RELAY_TOKEN="$(python3 - <<'PY'\nimport os\nprint(os.urandom(16).hex())\nPY\n)"
./target/debug/qshield init --store /tmp/qshield-demo --relay-token "$QSHIELD_RELAY_TOKEN"
./target/debug/qshield relay serve --listen 127.0.0.1:18080
./target/debug/qshield status --store /tmp/qshield-demo
```

## Notes

- `qshield relay serve` is a local-only relay stub for demo purposes.
- Relay endpoints require a bearer token (set `QSHIELD_RELAY_TOKEN` and/or `--relay-token`).
- Establish/send/recv are demo-only and require `--demo-unauthenticated-override`.

## Two-terminal demo (local relay)

Terminal A (Alice):

```sh
export QSHIELD_RELAY_TOKEN="$(python3 - <<'PY'\nimport os\nprint(os.urandom(16).hex())\nPY\n)"
./target/debug/qshield init --store /tmp/qshield-alice --relay-token "$QSHIELD_RELAY_TOKEN"
./target/debug/qshield register --store /tmp/qshield-alice --id alice
./target/debug/qshield relay serve --listen 127.0.0.1:18080
```

Terminal B (Bob):

```sh
./target/debug/qshield init --store /tmp/qshield-bob --relay-token "$QSHIELD_RELAY_TOKEN"
./target/debug/qshield register --store /tmp/qshield-bob --id bob
./target/debug/qshield establish --store /tmp/qshield-bob --peer alice --demo-unauthenticated-override
./target/debug/qshield send --store /tmp/qshield-bob --peer alice --text "hello" --demo-unauthenticated-override
```

Terminal A (Alice receives):

```sh
./target/debug/qshield establish --store /tmp/qshield-alice --peer bob --demo-unauthenticated-override
./target/debug/qshield recv --store /tmp/qshield-alice --demo-unauthenticated-override
```

## Actor RPC schema (demo-only)

The CLI uses the existing actor JSONL contract (stdin/stdout). These fields are
derived directly from the refimpl actor and the Suite-2 establish runner.

### suite2.establish.run (request)

```json
{
  "msg_type": { "u16": 1 },
  "negotiated": { "protocol_version": 1280, "suite_id": 2 },
  "session_id": "<hex-16-bytes>",
  "dh_init": "<hex-32-bytes>",
  "pq_init_ss": "<hex-32-bytes>",
  "dh_self_pub": "<hex-32-bytes>",
  "dh_peer_pub": "<hex-32-bytes>",
  "authenticated": { "bool": true },
  "role": "A"
}
```

### suite2.e2e.send (request)

```json
{
  "negotiated": { "protocol_version": 1280, "suite_id": 2 },
  "session_id": "<base64url-16-bytes>",
  "plaintext_hex": { "type": "hex", "data": "<hex>" },
  "flags": { "u16": 0 }
}
```

### suite2.e2e.recv (request)

```json
{
  "negotiated": { "protocol_version": 1280, "suite_id": 2 },
  "session_id": "<base64url-16-bytes>",
  "wire_hex": { "type": "hex", "data": "<hex>" }
}
```

## Demo-only notes

- This CLI derives demo establishment inputs deterministically from ids and public key placeholders.
  It is **not** a secure handshake and does **not** claim production security.
- `qshield relay serve` is local-only and in-memory for the demo.
