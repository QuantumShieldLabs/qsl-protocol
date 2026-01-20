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

Headless mode (non-interactive shells/CI-safe):
- `cargo run -p qsl-tui -- --headless --mode local`
- `QSL_ALLOW_REMOTE=1 \
   QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080 \
   QSL_RELAY_CHANNEL=demo \
   cargo run -p qsl-tui -- --headless --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo`

Interactive relay mode (opt-in required; needs real TTY/PTY):
- `QSL_ALLOW_REMOTE=1 \
   QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080 \
   QSL_RELAY_CHANNEL=demo \
   cargo run -p qsl-tui -- --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo`

Notes:
- Relay is transport-only; encryption/decryption happens client-side.
- Remote use requires explicit opt-in: `QSL_ALLOW_REMOTE=1`.

## DEMO-0001 Evidence (Headless)

This section captures the authoritative successful “golden run” for the Linux demo client in a non-interactive environment
(headless mode), both locally and over the public relay.

Evidence bundle (local, outside repo):

- OUT: /home/victor/work/qsl/_forensics/demo0001_headless_resume_20260119T053032Z
- Protocol repo HEAD: be0f97e0f3343f0129004a3ccbeddae2a4c1fd9b
- Isolated toolchain root: /home/victor/work/qsl/_forensics/rust_demo0001_20260119T052423Z
- Built binary: /home/victor/work/qsl/_forensics/rust_demo0001_20260119T052423Z/target/release/qsl-tui

Golden commands (headless):

1) Local (no relay):

- qsl-tui --headless --mode local

Expected evidence lines:

- QSL_TUI_HEADLESS_START mode=Local ...
- QSL_TUI_HEADLESS_OK plaintext=hello
- RC_LOCAL=0

2) Relay (explicit remote opt-in required):

- QSL_ALLOW_REMOTE=1 qsl-tui --headless --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo-20260119T053032Z

Expected evidence lines:

- QSL_TUI_HEADLESS_START mode=Relay base_url=http://qsl.ddnsfree.com:8080 channel=demo-20260119T053032Z
- QSL_TUI_HEADLESS_OK plaintext=hello
- RC_RELAY=0

Relay deployment snapshot checklist (run on the AWS host to freeze “what is running”):

- sudo systemctl status qsl-server --no-pager
- sudo systemctl cat qsl-server --no-pager
- sudo systemctl show qsl-server -p User,Group,WorkingDirectory,ExecStart,Restart,NoNewPrivileges,ProtectSystem,ProtectHome,ReadWritePaths --no-pager
- sudo ss -ltnp | grep -E ':8080\\b' || true
- sudo journalctl -u qsl-server -n 80 --no-pager
- sudo -u qslrelay -H bash -lc 'cd /opt/qsl-server/repo && printf "DEPLOYED_HEAD=%s\\n" "$(git rev-parse HEAD)" && git log -1 --oneline'


## NA-0053 Metadata reality + padding mitigation (qsl-tui)

### What is protected
- Message content is encrypted end-to-end by the QSL protocol.

### What is still visible
- Relay learns channel identifiers, ciphertext sizes, and timing.
- Network layer can observe source IP (unless you use a proxy/Tor).

### Mitigation implemented (client-layer)
- Size padding buckets inside the encrypted payload reduce ciphertext size correlation.
- Bucket sizes (bytes): 256, 512, 1024, 2048, 4096, 8192.
- Padding does not hide timing or IP; it only reduces size-based leakage.

### Golden commands (headless)

Local (no relay):

- qsl-tui --headless --mode local

Relay (explicit opt-in required):

- QSL_ALLOW_REMOTE=1 qsl-tui --headless --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo-na0053-<UTC>

Expected output markers:

- QSL_TUI_HEADLESS_START ...
- QSL_TUI_HEADLESS_PAD plain=<n> padded=<m> bucket=<b>
- QSL_TUI_HEADLESS_OK plaintext=hello

## NA-0054 Metadata visibility demo (qsl-tui)

Purpose: make metadata tradeoffs explicit in demo output (plaintext vs ciphertext length, padding bucket, and mode).

### Commands (headless)

Basic mode (no padding):

- qsl-tui --headless --mode local --privacy-mode basic

Padded mode (bucketed padding inside ciphertext):

- qsl-tui --headless --mode local --privacy-mode padded

Relay (explicit opt-in required; padded mode):

- QSL_ALLOW_REMOTE=1 qsl-tui --headless --mode relay --privacy-mode padded --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo-na0054-<UTC>

Expected output markers:

- QSL_TUI_META plaintext_len=<n> ciphertext_len=<m> bucket=<b> mode=<basic|padded>
- QSL_TUI_META_NOTE content_encrypted=true metadata_exposed=channel,timing,packet_size,ip mitigation=<none|padding_buckets_only>
