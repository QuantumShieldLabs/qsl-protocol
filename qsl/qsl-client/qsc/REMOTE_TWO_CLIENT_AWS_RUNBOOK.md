# QSC Remote Two-Client AWS Runbook (One Workstation, Two Isolated Clients)

Purpose: run a real-world operator simulation against an external relay.

Safety rules:
- Never paste relay tokens into shell history or logs.
- Use placeholders in notes: `<AWS_RELAY_URL>`, `<ALICE_TOKEN_FILE>`, `<BOB_TOKEN_FILE>`, `<ALICE_INBOX_TOKEN>`, `<BOB_INBOX_TOKEN>`.
- Keep Alice and Bob in separate config directories.

## 1) Preconditions
- AWS relay endpoint reachable at `<AWS_RELAY_URL>`.
- Two relay bearer tokens provisioned out-of-band.
- Two inbox route tokens (22-128 chars, `[A-Za-z0-9_-]` only).
- Rust toolchain installed.

## 2) Build
```bash
cd /path/to/qsl-protocol
cargo build -p qsc --release --locked
./target/release/qsc --help
```

## 3) Create isolated state roots
```bash
install -d -m 700 /tmp/qsc-aws-alice /tmp/qsc-aws-bob /tmp/qsc-aws-alice-out /tmp/qsc-aws-bob-out
export ALICE_CFG=/tmp/qsc-aws-alice
export BOB_CFG=/tmp/qsc-aws-bob
```

## 4) Initialize vault + identity (both clients)
```bash
# Alice
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' \
  ./target/release/qsc vault init --non-interactive --key-source passphrase --passphrase-env QSC_PASSPHRASE
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE identity rotate --confirm
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE identity show

# Bob
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc vault init --non-interactive --key-source passphrase --passphrase-env QSC_PASSPHRASE
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE identity rotate --confirm
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE identity show
```

The `identity show` verification code is shareable out-of-band. It is not a relay token.

## 5) Configure relay endpoint + token file (TUI command mode)
Use headless script mode for deterministic setup:
```bash
cat >/tmp/alice.setup.tui <<'SCRIPT'
/relay set endpoint <AWS_RELAY_URL>
/relay set token-file <ALICE_TOKEN_FILE>
/exit
SCRIPT
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT_FILE=/tmp/alice.setup.tui \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE tui

cat >/tmp/bob.setup.tui <<'SCRIPT'
/relay set endpoint <AWS_RELAY_URL>
/relay set token-file <BOB_TOKEN_FILE>
/exit
SCRIPT
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT_FILE=/tmp/bob.setup.tui \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE tui
```

Set per-client inbox route tokens:
```bash
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE relay inbox-set --token <ALICE_INBOX_TOKEN>
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE relay inbox-set --token <BOB_INBOX_TOKEN>
```

## 6) Add contacts, verify, trust
```bash
# Alice adds Bob
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE contacts add --label bob --fp <BOB_VERIFICATION_CODE> --route-token <BOB_INBOX_TOKEN>

# Bob adds Alice as verified first (not trusted)
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE contacts add --label alice --fp <ALICE_VERIFICATION_CODE> --route-token <ALICE_INBOX_TOKEN> --verify
```

Negative gate check (expected block):
```bash
echo 'pre-trust test' >/tmp/pretrust.txt
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE send --transport relay --relay <AWS_RELAY_URL> --to alice --file /tmp/pretrust.txt
```
Expected markers include `QSC_SEND_BLOCKED reason=no_trusted_device` and remediation markers.

Trust device and continue:
```bash
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE contacts device list --label alice
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE contacts device trust --label alice --device <ALICE_DEVICE_ID_12> --confirm
```

## 7) Handshake before first encrypted exchange
```bash
# Bob initiates
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake init --as self --peer alice --relay <AWS_RELAY_URL>

# Alice polls and responds
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake poll --as self --peer bob --relay <AWS_RELAY_URL> --max 8

# Bob finalizes
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake poll --as self --peer alice --relay <AWS_RELAY_URL> --max 8

# Alice completes responder side
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake poll --as self --peer bob --relay <AWS_RELAY_URL> --max 8
```

## 8) Message flow + honest delivery states
```bash
echo 'hello from bob' >/tmp/msg.txt
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE send --transport relay --relay <AWS_RELAY_URL> --to alice --file /tmp/msg.txt --receipt delivered

QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE receive --transport relay --relay <AWS_RELAY_URL> --mailbox <ALICE_INBOX_TOKEN> --from bob --max 20 --out /tmp/qsc-aws-alice-out --emit-receipts delivered --receipt-mode immediate

QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE receive --transport relay --relay <AWS_RELAY_URL> --mailbox <BOB_INBOX_TOKEN> --from alice --max 20 --out /tmp/qsc-aws-bob-out
```

Expected:
- sender emits `QSC_DELIVERY state=accepted_by_relay ...`
- sender emits `QSC_DELIVERY state=peer_confirmed ...` only after valid receipt arrives

## 9) File flow (small + >1MB)
Small file:
```bash
echo 'small' >/tmp/small.bin
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE file send --transport relay --relay <AWS_RELAY_URL> --to alice --path /tmp/small.bin --receipt delivered
```

Large file (>1MB):
```bash
head -c 1200000 /dev/zero >/tmp/large_1_2mb.bin
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE file send --transport relay --relay <AWS_RELAY_URL> --to alice --path /tmp/large_1_2mb.bin \
  --chunk-size 32768 --max-file-size 2000000 --max-chunks 80 --receipt delivered
```

Receiver pulls with confirmation emission:
```bash
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE receive --transport relay --relay <AWS_RELAY_URL> --mailbox <ALICE_INBOX_TOKEN> --from bob \
  --max 600 --out /tmp/qsc-aws-alice-out --emit-receipts delivered --receipt-mode immediate --file-confirm-mode complete-only
```

Sender pulls confirmations:
```bash
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE receive --transport relay --relay <AWS_RELAY_URL> --mailbox <BOB_INBOX_TOKEN> --from alice \
  --max 600 --out /tmp/qsc-aws-bob-out
```

Expected file state ladder on sender:
- `QSC_FILE_DELIVERY state=accepted_by_relay ...`
- `QSC_FILE_DELIVERY state=awaiting_confirmation ...`
- `QSC_FILE_DELIVERY state=peer_confirmed ...` only after valid completion confirm

## 10) Restart recovery
```bash
# Stop both clients, then resume receives
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake status --peer bob
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE handshake status --peer alice
```

Then repeat message send/receive to verify persisted state is still usable.

## 11) Troubleshooting map (marker -> action)
- `QSC_SEND_BLOCKED reason=unknown_contact`: add contact first.
- `QSC_SEND_BLOCKED reason=no_trusted_device`: list/verify/trust one device.
- `error code=relay_unauthorized`: check token provisioning, token-file path, and token scope.
- `error code=qsp_hdr_auth_failed`: re-run bounded handshake sequence and verify peer labels.
- `error code=manifest_mismatch` or `qsp_verify_failed` on file receive: treat as integrity failure, do not trust file; capture ledger entry.

## 12) Real-world scenario matrix snapshot (AWS run)
Legend: PASS = expected behavior observed. FAIL = mismatch or reliability gap.

- P0-A token/operator errors: PARTIAL PASS
  - missing/invalid route token blocked (PASS)
  - token-file specific diagnostics in CLI path currently weak (FAIL)
- P0-B add/verify/trust gating: PASS
  - verified-not-trusted blocked with deterministic remediation markers
- P0-C offline/delayed peer: PASS (no false delivered; accepted_by_relay observed)
- P0-D restart recovery: PASS for handshake status persistence
- P0-E honest delivery semantics: PASS on message path
- P0-F primary-only policy edges: PASS on routing markers; deeper multi-device validations remain in dedicated tests
- P1-G network variance: PARTIAL (observed relay auth and pull behavior under real endpoint)
- P1-H large file + retry UX: FAIL (integrity failures observed)
- P1-I command discoverability: PASS (core command surface validated in this runbook)

## 13) Issue Ledger (Fix-or-File)

### RW-AWS-001
- Severity: S1
- Area: relay / CLI auth
- Repro: run `send`/`receive` with relay token-file configured in account, without env token vars.
- Expected: auth header sourced from configured token-file.
- Actual: unauthorized before fix.
- Evidence markers: `error code=relay_unauthorized` prior to fix.
- Code anchors: `qsl/qsl-client/qsc/src/main.rs` (`relay_auth_token`), `qsl/qsl-client/qsc/tests/relay_auth_header.rs`.
- Fix direction: FIX_NOW.
- Status: fixed in NA-0183 branch; regression test added.

### RW-AWS-002
- Severity: S1
- Area: TUI relay setup diagnostics
- Repro: run `/relay test` in headless mode.
- Expected: deterministic reason surfaced for operator.
- Actual: generic command error in headless path, no actionable reason in immediate marker output.
- Evidence markers: `event=tui_cmd_result kind=err command=relay_test`.
- Code anchors: `qsl/qsl-client/qsc/src/main.rs` (`poll_relay_test_task`, `handle_tui_command relay test`).
- Fix direction: FILE_NA.
- Acceptance template:
  - Scope: `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`
  - Invariants: no secret leak; deterministic reason mapping
  - Tests: headless TUI `/relay test` reason marker assertions
  - Evidence: local gates + CI + leak counts

### RW-AWS-003
- Severity: S0
- Area: handshake/operator flow
- Repro: send encrypted message before complete handshake cycle.
- Expected: either guided handshake completion or clear actionable fail path.
- Actual: receiver fails with `qsp_hdr_auth_failed`.
- Evidence markers: `error code=qsp_hdr_auth_failed`.
- Code anchors: `qsl/qsl-client/qsc/src/main.rs` (`send_execute`, `receive_execute`, handshake commands).
- Fix direction: FILE_NA.
- Acceptance template:
  - Scope: qsc send/receive UX + tests
  - Invariants: fail-closed, deterministic markers, no implicit unsafe trust
  - Tests: two-client pre/post-handshake send outcomes

### RW-AWS-004
- Severity: S0
- Area: file transfer integrity/reliability
- Repro: after successful message handshake, run `file send` then receiver `receive` with confirms.
- Expected: no integrity failure on valid transfer; eventual `peer_confirmed` when confirm received.
- Actual: `manifest_mismatch` (small file) and `qsp_verify_failed` (>1MB).
- Evidence markers: `error code=manifest_mismatch`, `error code=qsp_verify_failed`.
- Code anchors: `qsl/qsl-client/qsc/src/main.rs` (file send/receive path), `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`.
- Fix direction: FILE_NA.
- Acceptance template:
  - Scope: file send/receive core + tests
  - Invariants: integrity checks fail-closed; no false peer_confirmed
  - Tests: deterministic two-client file receive matrix (small/large/restart)

### RW-AWS-005
- Severity: S2
- Area: route-token UX
- Repro: set short inbox token (`<22 chars`) via `relay inbox-set`.
- Expected: help text communicates token format upfront.
- Actual: immediate `QSC_ERR_ROUTE_TOKEN_INVALID` without prior format hint in command help.
- Evidence markers: `event=error code=QSC_ERR_ROUTE_TOKEN_INVALID`.
- Code anchors: `qsl/qsl-client/qsc/src/main.rs` (`route_token_is_valid`, CLI help docs).
- Fix direction: FILE_NA (docs/UX update).

## 14) Cleanup
```bash
rm -rf /tmp/qsc-aws-alice /tmp/qsc-aws-bob /tmp/qsc-aws-alice-out /tmp/qsc-aws-bob-out
rm -f /tmp/alice.setup.tui /tmp/bob.setup.tui /tmp/msg.txt /tmp/small.bin /tmp/large_1_2mb.bin
```
