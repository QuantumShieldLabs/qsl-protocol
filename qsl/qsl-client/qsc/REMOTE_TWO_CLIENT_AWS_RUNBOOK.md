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

Credential-pack preflight:
- Recreate `/tmp/qsc-aws-round2.env` from a sanctioned source before every AWS directive. Do not assume `/tmp` survived from a prior run.
- Minimum required keys for the clean proof:
  - `AWS_RELAY_URL`
  - `ALICE_RELAY_BEARER_TOKEN`
  - `BOB_RELAY_BEARER_TOKEN`
  - `QSC_AWS_VAULT_PASSPHRASE`
- Keep `/tmp/qsc-aws-round2.env` at `0600`.
- Validate relay auth with a secret-safe relay probe before onboarding. If auth fails, stop and repair the pack instead of reusing old mailbox state.

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
/relay test
/exit
SCRIPT
QSC_CONFIG_DIR="$ALICE_CFG" QSC_PASSPHRASE='<ALICE_PASSPHRASE>' QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT_FILE=/tmp/alice.setup.tui \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE tui

cat >/tmp/bob.setup.tui <<'SCRIPT'
/relay set endpoint <AWS_RELAY_URL>
/relay set token-file <BOB_TOKEN_FILE>
/relay test
/exit
SCRIPT
QSC_CONFIG_DIR="$BOB_CFG" QSC_PASSPHRASE='<BOB_PASSPHRASE>' QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT_FILE=/tmp/bob.setup.tui \
  ./target/release/qsc --unlock-passphrase-env QSC_PASSPHRASE tui
```

Expected headless relay-test markers after setup:
- `QSC_TUI_RELAY_TEST result=started code=pending`
- `QSC_TUI_RELAY_TEST result=ok code=relay_authenticated`
- `event=tui_relay_test_done ok=true reason=relay_authenticated`

If the probe fails, prefer the explicit `QSC_TUI_RELAY_TEST ... code=<...>` marker over the generic command-result line when deciding the next operator action.

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
  --max 600 --max-file-size 2000000 --max-file-chunks 80 --out /tmp/qsc-aws-alice-out --emit-receipts delivered --receipt-mode immediate --file-confirm-mode complete-only
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

Round-2 validation checkpoint:
- a Bob -> Alice small-file run must stay clean even though the sender uses `--to alice` and Alice receives with `--from bob`
- receiver should emit `event=file_xfer_manifest id=<redacted> ok=true` and `event=file_xfer_complete id=<redacted> ok=true`
- receiver should not emit `manifest_mismatch` on a fresh, clean run

Medium-file clean-proof controls:
- Use fresh mailbox route tokens for every clean rerun. Do not reuse prior-run tokens when classifying medium-file integrity.
- Always prove the Bob -> Alice small-file control on the same clean mailbox state before attempting the 1.2MB transfer.
- Treat the medium-file baseline as FAILED only when the clean small-file control passed first.
- Current clean-AWS status after Directive 121:
  - small-file control: PASS
  - medium-file 1.2MB at `--chunk-size 32768`: FIXED fail-closed reject
    - expected sender marker: `event=file_xfer_reject code=file_xfer_chunk_bound_invalid`
    - rationale: 32768-byte chunks overflow the current Suite-2 wire body-length field once file metadata is serialized
  - medium-file 1.2MB at `--chunk-size 16384` with `receive --max-file-size 2000000 --max-file-chunks 80`: receiver PASS
    - expected receiver markers:
      - `event=file_xfer_manifest id=<redacted> ok=true`
      - `event=file_xfer_complete id=<redacted> ok=true`
      - `event=file_confirm_send kind=coarse_complete ... ok=true`
    - the original `QSC_FILE_INTEGRITY_FAIL reason=qsp_verify_failed action=rotate_mailbox_hint` should not appear on the 16384 path
  - open follow-on after the boundary fix:
    - Bob's final confirmation pull can still fail with `event=qsp_unpack code=qsp_replay_reject ok=false` before `peer_confirmed`
    - treat that as a separate issue family; do not collapse it back into the original 32768 boundary failure

Robustness expectations during file send:
- transient push failures can emit deterministic retries:
  - `QSC_FILE_PUSH_RETRY attempt=1 backoff_ms=50 reason=...`
  - `QSC_FILE_PUSH_RETRY attempt=2 backoff_ms=100 reason=...`
- retry budget is bounded; repeated push failure remains fail-closed (no false success).

Integrity failure remediation (receiver side):
- on integrity reject, expect deterministic marker:
  - `QSC_FILE_INTEGRITY_FAIL reason=manifest_mismatch action=purge_partials`
  - or `QSC_FILE_INTEGRITY_FAIL reason=qsp_verify_failed action=rotate_mailbox_hint`
- after purge, rerun starts cleanly from first chunk (`event=file_xfer_reset reason=rerun_detected` when stale state is retired).
- if failures persist, rotate mailbox token(s), clear stale queue, and retry on fresh mailbox state.

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
- `error code=manifest_mismatch` on file receive:
  - expect `QSC_FILE_INTEGRITY_FAIL ... action=purge_partials` and rerun from first chunk.
- `error code=qsp_verify_failed` in receive:
  - first prove the clean small-file control on fresh mailbox tokens.
  - if the failing path uses `--chunk-size 32768`, reject it client-side and rerun at `--chunk-size 16384` with explicit receive bounds:
    - `--max-file-size 2000000`
    - `--max-file-chunks 80`
  - if `qsp_verify_failed` still appears on the 16384 path after that, treat it as a new issue family and capture fresh boundary evidence rather than reusing older mailbox state.
- `error code=qsp_replay_reject` on the sender's medium-file confirmation pull:
  - do not misreport `peer_confirmed`
  - treat it as a separate follow-on after the 32768 boundary fix, not as proof that the receiver-side medium file still failed integrity
- repeated `relay_inbox_push_failed`:
  - expect bounded retry markers; if exhaustion occurs, reduce burst size (`--chunk-size`) and retry.

## 12) Real-world scenario matrix snapshot (AWS run, NA-0184)
Legend: PASS = expected behavior observed. PARTIAL = constrained by relay/runtime limits. FAIL = mismatch or unresolved reliability gap.

Mandatory setup controls (required before running this matrix):
- Use `--relay https://<host>` (scheme required for handshake/send/receive parity).
- Create output directories with strict permissions (`0700`) to avoid `error code=unsafe_parent_perms`.
- Always rotate both inbox route tokens for each isolated rerun; reusing AWS mailboxes causes stale traffic contamination and invalidates the audit.
- If a run produces decode/integrity faults, rotate both inbox route tokens and re-run on fresh mailboxes before filing a product issue.

- P0-1 small file (1-10KB): PASS (clean mailbox path)
  - sender: `accepted_by_relay` then `awaiting_confirmation`
  - receiver: no integrity rejection in clean run
- P0-2 medium file (1-5MB): PARTIAL
  - 32768 path: now fails closed at send with `file_xfer_chunk_bound_invalid`
  - 16384 path: receiver completes cleanly with explicit receive bounds
  - sender `peer_confirmed` remains OPEN because the final confirmation pull can still hit `qsp_replay_reject`
- P0-3 large file (chunk stress): PARTIAL
  - observed `relay_inbox_push_failed` during sustained chunk bursts under external relay load
- P0-4 receiver restart mid-transfer: PARTIAL
  - bounded restart behavior works; reliability depends on mailbox hygiene and relay push continuity
- P0-5 sender restart after relay acceptance before confirm: PASS
  - state remained honest (no false peer-confirm)
- P0-6 wrong-device confirm attempt (primary-only): NOT RUN
  - this run used one trusted device per alias; run dedicated multi-device scenario for this check
- P0-7 transient disconnect/reconnect: PASS
  - invalid relay endpoint produced deterministic push failure; valid endpoint resumed operations
- P0-8 receiver offline send: PASS
  - sender showed relay acceptance semantics without false peer-confirm

## 13) AWS issue ledger reference
- See `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md` for `AWS-FILE-*` entries with severity, repro, and fix-or-file direction.

## 13A) NA-0190 command-surface audit notes
- TUI `/relay test` is now trustworthy again when it emits `QSC_TUI_RELAY_TEST result=ok|err code=<...>`. Prefer that marker over generic command-result text.
- Do not set `QSC_SELF_LABEL` for standard TUI AWS runs. The default TUI handshake identity now aligns with the CLI default identity label (`self`).
- Current clean-AWS status after NA-0191:
  - PASS: relay setup/test, balanced + strict onboarding, clean TUI handshake rerun, bidirectional messaging, small-file bidirectional transfer, restart recovery, explicit negative diagnostics for bad token perms / unsafe output parent / unreachable endpoint
  - OPEN: medium-file Bob -> Alice receive can still fail with `QSC_FILE_INTEGRITY_FAIL reason=qsp_verify_failed action=rotate_mailbox_hint` on fresh route tokens

## 13B) Clean TUI handshake rerun (post-NA-0191)
- Use fresh isolated configs and prefer fresh inbox route tokens when proving a "clean rerun". Reusing older remote mailboxes can surface stale `session_id_mismatch` noise from prior runs even when the current client path is correct.
- Minimal headless sequence after relay setup + add/verify/trust:
  - initiator: `/messages select <peer>` then `/handshake init`
  - responder: `/messages select <peer>` then `/handshake poll`
  - initiator: `/messages select <peer>` then `/handshake poll`
  - responder: `/messages select <peer>` then `/handshake poll`
- Expected secret-safe marker flow:
  - `event=handshake_send msg=A1`
  - `event=handshake_send msg=B1`
  - initiator finalize: `event=handshake_pending ... present=true role=initiator`, `event=handshake_send msg=A2`, `event=handshake_complete ... role=initiator`
  - responder finalize: `event=handshake_pending ... present=true role=responder`, `event=handshake_recv msg=A2 ok=true`, `event=handshake_complete ... role=responder`
- If the initiator or responder poll reports `event=handshake_pending ... present=false role=none` after `B1` has already been emitted on a fresh rerun, treat that as a regression and capture the marker bundle before proceeding.

## 14) Cleanup
```bash
rm -rf /tmp/qsc-aws-alice /tmp/qsc-aws-bob /tmp/qsc-aws-alice-out /tmp/qsc-aws-bob-out
rm -f /tmp/alice.setup.tui /tmp/bob.setup.tui /tmp/msg.txt /tmp/small.bin /tmp/large_1_2mb.bin
```

## 15) Onboarding Scenarios Matrix (NA-0187)
Use two isolated clients (`ALICE_CFG`, `BOB_CFG`) and placeholders only.

- A) Both online, first-time add, balanced mode
  - Set mode: `qsc contacts trust-mode set --mode balanced`
  - Add + verify both directions.
  - Expect trust promotion marker after verify: `QSC_TRUST_PROMOTION result=trusted reason=verified_match ... mode=balanced`.
  - Validate message + file roundtrip and honest delivery ladder.

- B) Wrong verification code -> CHANGED remediation
  - Run verify with wrong code once.
  - Expect marker: `contacts_device_verify ... code=verification_mismatch` and trust block marker with remediation steps.
  - Re-verify with correct code, then trust (strict mode) or auto-trust (balanced mode).

- C) Offline pending then retry
  - Add contact while peer offline.
  - Keep contact and route token; do not re-add.
  - When peer comes online, run verify/trust flow and retry send.

- D) Inbound request lifecycle
  - Unknown inbound should create request marker: `QSC_CONTACT_REQUEST action=created peer=<alias>`.
  - Accept moves into contacts as untrusted/discovered; send still blocked until trust.
  - Ignore removes request only.
  - Block marks peer/device revoked + blocked.

- E) Restart recovery during onboarding
  - Restart one client mid-flow (after add, before verify/trust).
  - Expect request/pending/contact state to persist and remediation to remain deterministic.

- F) Token-file/path UX failures
  - Intentionally set unreadable/missing token-file path.
  - Expect deterministic, actionable error with no secret-bearing output.
