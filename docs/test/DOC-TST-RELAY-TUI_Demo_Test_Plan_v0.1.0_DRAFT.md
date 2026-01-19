# DOC-TST-RELAY-TUI — Relay + Linux TUI Demo Test Plan v0.1.0 DRAFT

External relay repository (transport-only): https://github.com/QuantumShieldLabs/qsl-server


Status: DRAFT  
Date: 2026-01-18  
Goals: G2, G3

## 1. Scope
This plan covers a transport-only relay/server and a Linux TUI demo client that exercise existing protocol behavior.
This work must not introduce protocol-core, wire-format, cryptographic, or state-machine changes.

## 2. Invariants (must hold)
- Fail-closed on invalid inputs (no panics).
- Deterministic rejects/errors (stable identifiers/messages where applicable).
- No mutation on reject for any stateful operation.
- No secret logging (keys, nonces, shared secrets, plaintext payloads, or transcript secrets).
- Relay is transport-only: forwards/persists opaque payloads without interpreting crypto content.

## 3. Manual demo steps (smoke)
1) Start relay (local).
2) Start two demo clients (TUI or CLI) pointed at relay.
3) Execute establish/handshake flow using existing protocol.
4) Send/receive a small set of messages.
5) Verify deterministic error handling for malformed relay requests and invalid client inputs.

## 4. Negative cases (manual)
- Relay receives malformed request → deterministic error, no crash, no secret logging.
- Client sends malformed protocol message → deterministic reject; no mutation on reject.
- Relay persistence unavailable → deterministic error; no protocol changes required.

## 5. Future automation hooks (placeholder)
- Add harness-driven smoke tests for relay forward/persist.
- Add CI demo-smoke job once implementation exists (not part of this DRAFT).

## External implementation location (NA-0050)

The transport-only relay/server is implemented out-of-tree to enforce protocol-core isolation:

- Local path (dev): /home/victor/work/qsl/qsl-server

Operational note:
- The relay must remain transport-only and must not interpret protocol messages.
- Payload logging is forbidden; only metadata may be logged.

Minimal run (manual):
- In qsl-server:
  - cargo run
  - server listens on 0.0.0.0:8080

Planned integration:
- A later step will add demo wiring from the Linux TUI client to this relay without changing protocol semantics.

## Harness/Actors Relay Adapter (NA-0050)

### Local (offline-safe) adapter use
Environment variables (defaults are local/offline-safe):
- QSL_TRANSPORT=relay_http
- QSL_RELAY_BASE_URL=http://127.0.0.1:8080
- QSL_RELAY_CHANNEL=demo
- QSL_RELAY_TIMEOUT_SECS=5
- QSL_RELAY_MAX_POLL_SECS=10

Notes:
- The adapter treats payloads as opaque bytes and never logs payload content.
- CI must not depend on any remote relay endpoint.

### Remote demo (explicit opt-in)
Remote relay use must be explicit and optional:
- QSL_ALLOW_REMOTE=1
- QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080
- QSL_TRANSPORT=relay_http
- QSL_RELAY_CHANNEL=har-<RUN_ID>

Golden command (interop over relay_http; opaque bytes only):
- Build actor: `cargo build -q -p refimpl_actor --release`
- Use a local actors manifest that points to `target/release/refimpl_actor` (do not edit repo files).
- Run:
  - `export QSL_ALLOW_REMOTE=1`
  - `python3 tests/harness/4b/runner.py interop --out <OUT_DIR> --run-id <RUN_ID> --git-commit <HEAD> --phase2-zip <PHASE2_ZIP> --phase3-zip <PHASE3_ZIP> --actors <ACTORS_LOCAL>`

Expected:
- `[4B] interop: passing=4 total_results=4`

## Linux TUI demo (NA-0051)

Local mode (default):
- `cargo run -p qsl-tui -- --mode local`

Relay mode (opt-in required):
- `QSL_ALLOW_REMOTE=1 \
   QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080 \
   QSL_RELAY_CHANNEL=demo \
   cargo run -p qsl-tui -- --mode relay --relay-channel demo`

Notes:
- Relay is transport-only; encryption/decryption happens client-side.
- Remote use requires explicit opt-in: `QSL_ALLOW_REMOTE=1`.
