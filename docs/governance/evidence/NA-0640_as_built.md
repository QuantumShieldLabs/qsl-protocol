# NA-0640 As-Built — Full-Stack E2E Integration (D576, D-1263)

Goals: G1, G2, G3, G4
Status: Supporting evidence (lane closeout)
Owner: QSL governance
Last-Updated: 2026-07-13

## 1. What this lane built

The FIRST test that exercises the real product stack together — two isolated qsc
clients, the REAL qsl-server relay, and the REAL qsl-attachments service, all
in-process, in the STANDARD `cargo test -p qsc` suite. Zero production-source change:
both services are consumed as-is at pinned revs.

Artifacts:
- `qsl/qsl-client/qsc/tests/NA_0640_full_stack_e2e.rs` — the e2e round-trips.
- `qsl/qsl-client/qsc/tests/NA_0640_tui_focus_semantics.rs` — the two TUI focus
  assertions ported in-suite (part 2 of D576; preserve-then-retire order held).
- `qsl/qsl-client/qsc/tests/common/mod.rs` — `start_qsl_server` helper (+
  `QslRelayTestServer`, readiness probe), mirroring `start_attachment_server`.
- `qsl/qsl-client/qsc/Cargo.toml` — the qsl-server DEV-ONLY git dependency pin.
- `.github/workflows/relay-ui-integration.yml` — DELETED (part 3; after part 2 green).
- This document; `tests/NA-0640_e2e_integration_full_stack_testplan.md`; WF-0023 on the
  ledger (filed + closed-as-paid); DECISIONS D-1263; TRACEABILITY; journal; NEXT_ACTIONS.

## 2. The qsl-server pin (Phase 0/1 record)

- Rev pinned: `19b9b02dbe1f2ae9bc246ff3a16890e56c073c3e` — qsl-server main HEAD at both
  directive landing (2026-07-12) and lane execution (2026-07-13); "current-or-intended"
  was satisfied as CURRENT, no operator re-decision needed.
- Public API verified at the pinned rev: `pub fn app(state: AppState) -> Router`
  (`src/lib.rs:311`); `AppState` constructors (`:255-283`); `Limits` (`:94`, `new` `:117`);
  `ResourceControls` (`:140`, `Default` `:147`).
- Helper design decision: `AppState::new_with_auth_and_controls` is called EXPLICITLY —
  qsl-server's `AppState::new` / `new_with_controls` read the ambient `RELAY_TOKEN` env
  (`src/lib.rs:255-272`), which a test must never depend on; the helper pins auth per
  call (`relay_token: Option<&str>` parameter).

## 3. Dev-only confirmation (Phase 1/5 record)

`cargo tree -p qsc -e normal` (the PRODUCTION dependency graph) captured before and
after the pin: **byte-identical** (483 lines, sha256
`e9a35849ded03c659c9a74c48c5eea3d21f72c42124c657fafc95a652fcdcbb1` both sides).
Cargo.lock additions: exactly three packages — `qsl-server` (the pinned rev),
`tower-http 0.5.2`, `uuid 1.23.5` — all reachable only via the dev edge. The shipped
binary's dependency graph is unchanged.

## 4. What the e2e test exercises (and its limits)

`NA_0640_full_stack_e2e.rs`, two tests, serialized by a file-local guard:

1. `full_stack_message_and_attachment_round_trip_open_relay`:
   - REAL qsl-server in-process (open mode), REAL qsl-attachments in-process, two qsc
     client config dirs with real rotated identities, cross-pinned contacts, distinct
     inbox route tokens.
   - Message: A `send` → real relay `/v1/push` → B `receive` (from `/v1/pull`) →
     **plaintext byte-match** (`recv_1.bin` == sent bytes) + receipt round-trip back to
     A (`QSC_DELIVERY state=peer_confirmed`).
   - Attachment: 6 MiB + 321 B payload (>4 MiB ⇒ the REAL attachment path;
     `QSC_ATTACHMENT_SERVICE` set + `--attachment-service`) — upload sessions on the
     real service, descriptor through the real relay, B downloads and the file is
     **byte-verified** against the payload; `QSC_FILE_DELIVERY` reaches
     `accepted_by_relay` → `peer_confirmed`.
2. `full_stack_message_round_trip_token_auth_relay`:
   - Same message round-trip with the server in BEARER-TOKEN mode (`RELAY_BEARER` on the
     server, `QSC_RELAY_TOKEN` on the clients).
   - Negative: an isolated third client with the WRONG bearer is REJECTED by the real
     server (send fails, never reaches `accepted_by_relay`) — the enforcement is
     exercised, not assumed.

Local result: **2 passed, 0 failed (116.27s)** on the first run against unmodified
product source — the components interoperate as-is; no ENG filing was warranted.

LIMITS (a PASS asserts interop under these scenarios at the pinned revs — nothing more):
- Auth: open + static bearer; token rotation/revocation untested (ENG-0036 territory).
- Sizes: one small message, one 6 MiB attachment; the 4 MiB threshold boundary sweep
  stays covered by na0197c against the mock inbox.
- Paths: happy-path round-trips + one auth negative. No drop/reorder/fault injection
  against the real relay (the retired remote smokes' scenario territory; candidate
  successor). No multi-device, no concurrent-client contention.
- The remote deployed-relay workflows (remote-relay/remote-handshake) remain red and
  operator-gated (NA-0564/NA-0565) — out of scope here.

## 5. The TUI focus-semantics port (Phase 1 decision + Phase 2 record)

- Decision: a NEW file `NA_0640_tui_focus_semantics.rs` (not an addition to an existing
  TUI test file) — lane provenance stays legible and no passing file is perturbed.
- The two assertions ported (previously ONLY in the `#[ignore]`d
  `relay_ui_integration.rs:126/:172`, unguarded since 2026-02-11):
  unfocused inbound ⇒ `mode=buffer` + `unread=1`; focused inbound ⇒ `mode=append` +
  `unread=0`, on `event=tui_message_event`.
- Mock inbox (`common::start_inbox_server`) per D576 — the semantics are TUI-local.
- Idiom: current na0177 headless-script setup (`/relay set endpoint`, `/relay inbox
  set`, `/contacts add`, `/trust pin`), NOT the stale na-0127 scaffolding (predates the
  route-token migration; sent to an unknown peer with no contact).
- Focus-model finding (test-code adaptation, no product change): the append condition is
  `mode==Normal && inspector==Events && home_focus==Main && selected==peer`
  (`tui/controller/state/account.rs:440-443`). Under the CURRENT key model,
  `/messages select <peer>` leaves `home_focus` on the nav column and `/key tab` toggles
  it INTO Main — the inverse of the na-0127-era scripts. First run demonstrated BOTH
  behaviors fire (inverted against my initial scripts); the scripts were corrected to
  drive current controls. The ASSERTED semantics are unchanged from the originals.
- Local result: **2 passed, 0 failed (18.70s)**.

## 6. Workflow retirement (Phase 3 record)

`.github/workflows/relay-ui-integration.yml` deleted ONLY after §5 was green
(preserve-then-retire held). Its transport leg is superseded by §4 (which is stronger:
in-suite, pinned, byte-verified, auth-negative). Its TUI value is preserved by §5. The
`.github` write was authorized by the operator-provided per-lane
`.claude/settings.local.json` (deleted at closeout, §8).

Note: the `#[ignore]`d `relay_ui_integration.rs` TEST FILE remains in-tree (dormant —
no runner references it). Deleting it was outside D576's allowed paths; recorded in
WF-0023 as a hygiene-sweep candidate.

## 7. Root-cause closure (Phase 4)

WF-0023 filed + closed-as-paid on the ledger. Both halves of the February failure mode
are structurally closed: the coverage now lives in the standard suite (it cannot
silently stop running — `qsc-linux-full-suite` on main-push + the local merge-gate run
execute it), and qsl-server is REV-PINNED (drift becomes a deliberate, visible bump —
exactly like the qsl-attachments precedent on the adjacent Cargo.toml line).

## 8. Validation + closeout proofs (Phases 5-6)

- Local `cargo test -p qsc --locked` full-suite result: recorded in the lane response
  (the real merge gate; `qsc-linux-full-suite` SKIPS on PRs).
- goal-lint: run locally with a synthesized event (NEXT_ACTIONS-only + tests + docs
  diff); result recorded in the lane response.
- Scope guard: the diff touches ONLY the D576 allowed paths (verified via
  `git diff --name-only` against the allowed list in the lane response).
- No production source touched: `git diff --name-only` contains no `src/` path in qsc,
  and the pinned service repos were never modified (consumed by rev).
- Dev-only: §3.
- `.claude/settings.local.json`: DELETED at closeout; verified absent in the lane
  response.
- Result classification: **E2E_INTEGRATION_FULL_STACK_PASS** (limits in §4).
