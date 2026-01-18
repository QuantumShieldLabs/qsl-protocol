# DOC-TST-RELAY-TUI — Relay + Linux TUI Demo Test Plan v0.1.0 DRAFT

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
