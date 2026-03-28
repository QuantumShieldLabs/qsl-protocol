# QSC Local Two-Client Runbook (One Machine, Two Isolated Clients)

This runbook validates end-to-end CLI reliability with two local clients.

Safety rules:
- Do not paste real secrets into logs.
- Use placeholders for all secret values.
- Keep each client in its own isolated config/state directory.

## 1) Prereqs
- Rust toolchain installed (`cargo`, `rustc`).
- Repo root at `qsl-protocol/`.
- Two terminal windows (`Alice`, `Bob`) and one relay terminal.

## 2) Build / install
```bash
cd /path/to/qsl-protocol
cargo build -p qsc --release --locked
./target/release/qsc --help
```

## 3) Start a local relay (no relay code edits)
Use qsc's local relay server:
```bash
./target/release/qsc relay serve \
  --port 0 \
  --seed 0 \
  --drop-pct 0 \
  --dup-pct 0 \
  --reorder-window 0 \
  --fixed-latency-ms 0 \
  --jitter-ms 0
```

Expected marker includes an assigned port:
```text
QSC_MARK/1 event=relay_listen port=<PORT> seed=0
```

Set:
- `<RELAY_URL>=http://127.0.0.1:<PORT>`

Connectivity check (safe):
```bash
curl -s -o /dev/null -w "%{http_code}\n" -H "X-QSL-Route-Token: test_mailbox" "<RELAY_URL>/v1/pull?max=1"
```
Expected: `204`.

## 4) Create two client identities / account material
Use two isolated config roots:
- Alice: `QSC_CONFIG_DIR=/tmp/qsc-alice`
- Bob: `QSC_CONFIG_DIR=/tmp/qsc-bob`

Create dirs with strict perms:
```bash
install -d -m 700 /tmp/qsc-alice /tmp/qsc-bob /tmp/qsc-alice-out /tmp/qsc-bob-out
```

Initialize vault + identity in each terminal:
```bash
# Alice terminal
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc vault init --non-interactive --key-source mock
./target/release/qsc identity rotate --confirm
./target/release/qsc identity show
```

```bash
# Bob terminal
export QSC_CONFIG_DIR=/tmp/qsc-bob
./target/release/qsc vault init --non-interactive --key-source mock
./target/release/qsc identity rotate --confirm
./target/release/qsc identity show
```

Record shareable verification codes:
- `<ALICE_VERIFICATION_CODE>`
- `<BOB_VERIFICATION_CODE>`

Important:
- Verification code is shareable.
- It is not a relay token.

## 5) Two isolated state directories (must not collide)
Use separate directories:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice   # Alice only
export QSC_CONFIG_DIR=/tmp/qsc-bob     # Bob only
```

Quick isolation proof:
```bash
ls -la /tmp/qsc-alice
ls -la /tmp/qsc-bob
```
The stores are independent; do not reuse one directory for both clients.

## 6) Setup relay and mailbox tokens per client
Set each client's own inbox token:
```bash
# Alice terminal
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc relay inbox-set --token <ALICE_ROUTE_TOKEN>
```

```bash
# Bob terminal
export QSC_CONFIG_DIR=/tmp/qsc-bob
./target/release/qsc relay inbox-set --token <BOB_ROUTE_TOKEN>
```

## 7) Exchange verification codes
Each side uses the code from `identity show`:
- Alice receives `<BOB_VERIFICATION_CODE>`
- Bob receives `<ALICE_VERIFICATION_CODE>`

## 8) Add contacts (both directions)
```bash
# Alice adds Bob
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc contacts add \
  --label bob \
  --fp <BOB_VERIFICATION_CODE> \
  --route-token <BOB_ROUTE_TOKEN>
```

```bash
# Bob adds Alice using aligned peer label "bob"
# (current CLI receive context resolves by local contact label)
export QSC_CONFIG_DIR=/tmp/qsc-bob
./target/release/qsc contacts add \
  --label bob \
  --fp <ALICE_VERIFICATION_CODE> \
  --route-token <ALICE_ROUTE_TOKEN>
```

## 9) Verify and trust (device-aware)
`VERIFIED` and `TRUSTED` are different:
- `VERIFIED`: code matched.
- `TRUSTED`: send-authorized.

List devices and trust the intended one:
```bash
# Alice trusts Bob device
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc contacts device list --label bob
./target/release/qsc contacts device trust --label bob --device <BOB_DEVICE_ID_12> --confirm
```

```bash
# Bob trusts Alice device (stored under local label "bob")
export QSC_CONFIG_DIR=/tmp/qsc-bob
./target/release/qsc contacts device list --label bob
./target/release/qsc contacts device trust --label bob --device <ALICE_DEVICE_ID_12> --confirm
```

Optional primary-device controls:
```bash
./target/release/qsc contacts device primary show --label bob
./target/release/qsc contacts device primary set --label bob --device <BOB_DEVICE_ID_12> --confirm
```

## 10) Send / receive message (honest status)
Create payload and send from Alice:
```bash
echo "hello from alice" > /tmp/qsc-msg.txt
export QSC_CONFIG_DIR=/tmp/qsc-alice
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc send \
  --transport relay \
  --relay <RELAY_URL> \
  --to bob \
  --file /tmp/qsc-msg.txt \
  --receipt delivered
```

Expected marker on send:
- `QSC_DELIVERY state=accepted_by_relay ...`

Bob receives and emits receipt:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-bob
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc receive \
  --transport relay \
  --relay <RELAY_URL> \
  --mailbox <BOB_ROUTE_TOKEN> \
  --from bob \
  --max 4 \
  --out /tmp/qsc-bob-out \
  --emit-receipts delivered \
  --receipt-mode immediate
```

Alice processes confirmation:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc receive \
  --transport relay \
  --relay <RELAY_URL> \
  --mailbox <ALICE_ROUTE_TOKEN> \
  --from bob \
  --max 4 \
  --out /tmp/qsc-alice-out
```

Honest semantics:
- `accepted_by_relay` appears after relay accepts send.
- `peer_confirmed` appears only after valid receipt processing.

## 11) Send / receive file (honest status)
```bash
echo "file payload" > /tmp/qsc-file.bin
export QSC_CONFIG_DIR=/tmp/qsc-alice
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc file send \
  --transport relay \
  --relay <RELAY_URL> \
  --to bob \
  --path /tmp/qsc-file.bin \
  --receipt delivered
```

Validated-deployment post-`w0` controls:
- `QSC_ATTACHMENT_SERVICE=<ATTACHMENT_SERVICE_URL>` supplies the validated attachment-service endpoint.
- `file send --attachment-service <ATTACHMENT_SERVICE_URL>` remains an explicit override/diagnostic for the streaming path; by itself it does not activate the validated post-`w0` send defaults.
- With `QSC_ATTACHMENT_SERVICE` set, qsc now uses the post-`w0` retired defaults for validated deployments: new `<= 4 MiB` sends use `w2`, and attachment-service-backed receive defaults legacy payload handling to retired.
- `QSC_LEGACY_IN_MESSAGE_STAGE=w2` is optional redundancy on that validated lane.
- `QSC_LEGACY_IN_MESSAGE_STAGE=w0` and `w1` are retired on validated post-`w0` deployments; qsc rejects them with `legacy_in_message_stage_retired_post_w0`.
- `> 4 MiB` sends are unchanged by `w0|w2`; they stay on the validated-deployment attachment-first policy from `NA-0202A`.
- `--legacy-in-message-stage w2` is allowed as an explicit no-op confirmation for one send; `w0` and `w1` reject on the validated post-`w0` lane.
- If `w2` selects the attachment path and no validated attachment-service config exists, qsc fails closed with `attachment_service_required`; it does not retry silently on the legacy path.
- `--attachment-service <ATTACHMENT_SERVICE_URL>` on receive activates the validated post-`w0` attachment-fetch lane and makes retired legacy receive the default contract.
- `--legacy-receive-mode retired` is still allowed explicitly but is redundant on that lane.
- `--legacy-receive-mode coexistence` is retired there and rejects with `legacy_receive_mode_retired_post_w0`.
- Residual legacy `file_chunk` / `file_manifest` payloads then fail closed with `event=legacy_receive_reject code=legacy_receive_retired_post_w0 ...`, `event=file_xfer_reject code=legacy_receive_retired_post_w0 ...`, and `event=error code=legacy_receive_retired_post_w0`.
- In retired mode qsc must not reconstruct legacy files, append receive timeline state, emit file completion receipts, or advance `peer_confirmed`.

Expected send transitions:
- `QSC_MARK/1 event=file_send_policy stage=w2 size_class=legacy_sized|above_threshold path=attachment`
- `QSC_FILE_DELIVERY state=accepted_by_relay ...`
- `QSC_FILE_DELIVERY state=awaiting_confirmation ...`

Validated-deployment W2 default example for a legacy-sized file:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
export QSC_ATTACHMENT_SERVICE=<ATTACHMENT_SERVICE_URL>
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc file send \
  --transport relay \
  --relay <RELAY_URL> \
  --to bob \
  --path /tmp/qsc-file.bin \
  --receipt delivered
```

Retired legacy-stage override reject example:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
export QSC_ATTACHMENT_SERVICE=<ATTACHMENT_SERVICE_URL>
export QSC_LEGACY_IN_MESSAGE_STAGE=w0
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc file send \
  --transport relay \
  --relay <RELAY_URL> \
  --to bob \
  --path /tmp/qsc-file.bin
```

Expected reject:
- `QSC_MARK/1 event=file_xfer_reject code=legacy_in_message_stage_retired_post_w0 ...`

Bob receive + emit completion confirm:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-bob
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc receive \
  --transport relay \
  --relay <RELAY_URL> \
  --attachment-service <ATTACHMENT_SERVICE_URL> \
  --mailbox <BOB_ROUTE_TOKEN> \
  --from bob \
  --max 8 \
  --out /tmp/qsc-bob-out \
  --emit-receipts delivered \
  --receipt-mode immediate \
  --file-confirm-mode complete-only
```

Residual legacy receive reject on the validated post-`w0` lane:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-bob
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc receive \
  --transport relay \
  --relay <RELAY_URL> \
  --attachment-service <ATTACHMENT_SERVICE_URL> \
  --mailbox <BOB_ROUTE_TOKEN> \
  --from bob \
  --max 8 \
  --out /tmp/qsc-bob-out
```

Expected reject markers include:
- `QSC_MARK/1 event=legacy_receive_reject code=legacy_receive_retired_post_w0 ...`
- `QSC_MARK/1 event=file_xfer_reject code=legacy_receive_retired_post_w0 ...`
- `QSC_MARK/1 event=error code=legacy_receive_retired_post_w0`

Alice processes file confirmation:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc receive \
  --transport relay \
  --relay <RELAY_URL> \
  --mailbox <ALICE_ROUTE_TOKEN> \
  --from bob \
  --max 8 \
  --out /tmp/qsc-alice-out
```

Expected:
- `QSC_FILE_DELIVERY state=peer_confirmed ...`

## 12) Negative tests (fail-closed + remediation)
Unknown contact must block:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc send \
  --transport relay \
  --relay <RELAY_URL> \
  --to unknown \
  --file /tmp/qsc-msg.txt
```
Expected markers include:
- `QSC_SEND_BLOCKED reason=unknown_contact ...`
- `QSC_TRUST_REMEDIATION reason=unknown_contact step=add_contact ...`

No-trust must block with no mutation:
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc contacts add --label eve --fp <BOB_VERIFICATION_CODE> --route-token <BOB_ROUTE_TOKEN>
QSC_QSP_SEED=1 QSC_ALLOW_SEED_FALLBACK=1 QSC_MARK_FORMAT=plain \
./target/release/qsc send \
  --transport relay \
  --relay <RELAY_URL> \
  --to eve \
  --file /tmp/qsc-msg.txt
```
Expected markers include:
- `QSC_SEND_BLOCKED reason=no_trusted_device ...`
- `QSC_TRUST_REMEDIATION reason=no_trusted_device step=trust_device ...`
- no send mutation markers (`send_prepare`/routing attempt) on blocked path.

## 13) Receipt mode quick toggles
Receipt policy is controllable on `receive`:
- `--receipt-mode off`
- `--receipt-mode batched --receipt-batch-window-ms <MS> --receipt-jitter-ms <MS>`
- `--receipt-mode immediate`

File completion policy:
- `--file-confirm-mode off`
- `--file-confirm-mode complete-only`

## 14) Optional TUI launch
```bash
export QSC_CONFIG_DIR=/tmp/qsc-alice
./target/release/qsc tui --transport relay --relay <RELAY_URL>
```
Use existing trusted contacts and relay settings from CLI setup.

## 15) Cleanup
```bash
pkill -f "qsc relay serve" || true
rm -rf /tmp/qsc-alice /tmp/qsc-bob /tmp/qsc-alice-out /tmp/qsc-bob-out
rm -f /tmp/qsc-msg.txt /tmp/qsc-file.bin
```
