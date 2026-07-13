# NA-0640 Testplan — Full-Stack E2E Integration (D576, D-1263)

Goals: G1, G2, G3, G4
Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-13

## Objective

Prove the real product stack interoperates: two qsc clients + the REAL qsl-server
(in-process, dev-only pin `19b9b02d`) + the REAL qsl-attachments (in-process, existing
pin `1e1ae272`), message + >4 MiB attachment round-trips, in the standard suite; port
the two TUI focus-semantics assertions in-suite; retire the drifted relay-ui workflow.

## Test matrix

| # | Test | Stack | Asserts |
|---|------|-------|---------|
| 1 | `NA_0640_full_stack_e2e.rs::full_stack_message_and_attachment_round_trip_open_relay` | REAL qsl-server (open) + REAL qsl-attachments + 2 qsc clients | message: `accepted_by_relay` → B receipt (`QSC_RECEIPT ... kind=message`) → plaintext byte-match (`recv_1.bin`) → A `peer_confirmed`; attachment (6 MiB+321): `QSC_FILE_DELIVERY accepted_by_relay`+`awaiting_confirmation` → B download byte-verified → A `peer_confirmed`; no `/v1/` leak in outputs |
| 2 | `NA_0640_full_stack_e2e.rs::full_stack_message_round_trip_token_auth_relay` | REAL qsl-server (bearer-token) + 2 qsc clients | same message round-trip under `QSC_RELAY_TOKEN`; NEGATIVE: isolated wrong-bearer client send FAILS and never reaches `accepted_by_relay` |
| 3 | `NA_0640_tui_focus_semantics.rs::unfocused_inbound_increments_counter_only` | mock inbox + headless TUI | `event=tui_message_event peer=bob` with `total=1 mode=buffer unread=1` |
| 4 | `NA_0640_tui_focus_semantics.rs::focused_inbound_appends_to_stream` | mock inbox + headless TUI | `event=tui_message_event peer=bob` with `total=1 mode=append unread=0` |

Tests 1-2 serialized by a file-local mutex (subprocess-heavy; mirrors the attachment
suites). Tests 3-4 preserve the ONLY coverage of the focus-routing pair previously held
by the dead relay-ui workflow (see WF-0023); scripts drive the CURRENT focus key model
(`/key tab` toggles home_focus INTO Main — `account.rs:440-443`).

## Verification gates

- `cargo test -p qsc --test NA_0640_full_stack_e2e` — green (2/2, first run, unmodified
  product source).
- `cargo test -p qsc --test NA_0640_tui_focus_semantics` — green (2/2).
- Full local `cargo test -p qsc --locked` — the real merge gate (`qsc-linux-full-suite`
  SKIPS on PRs); result recorded in the lane response.
- Dev-only pin proof: `cargo tree -p qsc -e normal` byte-identical before/after
  (sha256 `e9a35849...cbcb1`); Cargo.lock adds only `qsl-server`, `tower-http 0.5.2`,
  `uuid 1.23.5` (dev edge).
- Workflow retirement gated on tests 3-4 green FIRST (preserve-then-retire).

## Boundaries honored

No production source changed (qsc/qsl-server/qsl-attachments consumed as-is); an
interop failure would have been an ENG filing + STOP, not a patch; `.github` write under
the operator-provided per-lane settings.local.json override (deleted at closeout).

## Refs

D576 (`QSL-DIR-2026-07-12-576`); D-1263; WF-0023; `docs/governance/evidence/NA-0640_as_built.md`.
