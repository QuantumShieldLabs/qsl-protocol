Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# DOC-AUD-002 — qsc Handshake Execution Security Audit v0.1.0 DRAFT

## Scope and authority used

- Queue authority: `NA-0220 — qsc Handshake Execution Security Audit (read-only, evidence-first)` from live `NEXT_ACTIONS.md`.
- Supporting audit guidance: `docs/audit/DOC-AUD-001_qsc_Director_Ready_Crypto_and_Code_Audit_Program_v0.1.0_DRAFT.md`.
- Canonical protocol authority:
  - `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`
  - `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md`
- Read-only code/test surfaces reviewed:
  - `qsl/qsl-client/qsc/src/handshake/mod.rs`
  - `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
  - `qsl/qsl-client/qsc/src/identity/mod.rs`
  - `qsl/qsl-client/qsc/src/fs_store/mod.rs`
  - `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
  - `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
  - `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
  - `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
  - `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- Lane posture: read-only and evidence-first. No runtime changes are authorized by this audit.

## Methods used

- Built a file/symbol inventory over the allowed handshake, identity, protocol-state, and fs-store surfaces.
- Traced every handshake accept path to the durable-write points:
  - `hs_pending_store(...)`
  - `qsp_session_store(...)`
  - `handshake_status(...)`
- Compared those commit paths to the canonical base-handshake requirements in `DOC-CAN-003` §§0.2, 6.3, and 6.6.
- Reviewed the direct negative-coverage tests for tamper, replay, out-of-order input, and pinned-identity mismatch.
- Reviewed operator-visible status behavior against the staged handshake timeline.

## Read-only evidence commands/tests used

- `rg --files qsl/qsl-client/qsc/src/handshake qsl/qsl-client/qsc/src/protocol_state qsl/qsl-client/qsc/src/identity qsl/qsl-client/qsc/src/fs_store qsl/qsl-client/qsc/tests | rg 'handshake_|qsp_protocol_gate|desktop_gui_contract_na0215b|qsl/qsl-client/qsc/src/handshake/|qsl/qsl-client/qsc/src/protocol_state/|qsl/qsl-client/qsc/src/identity/|qsl/qsl-client/qsc/src/fs_store/'`
- `rg -n "transcript|identity|mismatch|pinned|session|confirm|reject|replay|downgrade" qsl/qsl-client/qsc/src/handshake qsl/qsl-client/qsc/src/protocol_state qsl/qsl-client/qsc/src/identity qsl/qsl-client/qsc/src/fs_store qsl/qsl-client/qsc/tests/handshake_*.rs qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `nl -ba qsl/qsl-client/qsc/src/handshake/mod.rs | sed -n '1,1280p'`
- `nl -ba qsl/qsl-client/qsc/src/protocol_state/mod.rs | sed -n '1,520p'`
- `nl -ba qsl/qsl-client/qsc/src/identity/mod.rs | sed -n '1,460p'`
- `nl -ba docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md | sed -n '30,45p'`
- `nl -ba docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md | sed -n '354,405p'`
- `cargo test -p qsc --locked handshake_two_party_establishes_session -- --exact --nocapture`
  - PASS
- `cargo test -p qsc --locked handshake_pinned_identity_mismatch_fails -- --exact --nocapture`
  - PASS
- `cargo test -p qsc --locked handshake_rejects_tampered_transcript_no_mutation -- --exact --nocapture`
  - PASS

## Findings by severity

### P1

#### Finding ID

- `NA0220-F001`

#### Severity

- `P1`

#### Exact surfaces

- Files/functions:
  - `qsl/qsl-client/qsc/src/handshake/mod.rs`
  - `perform_handshake_poll_with_tokens(...)`
  - initiator `B1` accept path around the `identity_read_sig_pin(...)` / `qsp_session_store(...)` sequence
  - responder `A1` accept path around the `identity_read_pin(...)`, `identity_read_sig_pin(...)`, and `hs_pending_store(...)` sequence
  - responder `A2` accept path around the `identity_read_pin(...)`, `identity_read_sig_pin(...)`, and `qsp_session_store(...)` sequence
- Tests:
  - `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
  - `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- Spec sections:
  - `DOC-CAN-003` §0.2
  - `DOC-CAN-003` §6.3
  - `DOC-CAN-003` §6.6

#### Violated claim/invariant

- Suite-2 state must not be committed until the base handshake has authenticated peer identity at the system layer.
- The base handshake transcript must provide authenticated commitment to the Suite-2 negotiation tuple and required establishment inputs before session state is treated as authenticated.

#### Why it matters

- Current qsc handshake accept paths proceed on `identity_unknown` and still write durable pending/session state.
- The code hardcodes `authenticated=true` into `init_from_base_handshake(...)` even though the qsc-local handshake transcript does not prove the full canonical base-handshake contract.
- That lets unauthenticated first-contact or unpinned peers influence durable handshake/session state while the implementation claims a stronger authenticated-establishment posture than the audited proof supports.

#### Evidence/proof

- `DOC-CAN-003` §0.2 requires authenticated peer identity before Suite-2 state is committed.
- `DOC-CAN-003` §6.3 requires authenticated commitment to peer identity, `(protocol_version=0x0500, suite_id=0x0002)`, `session_id`, and the base-handshake inputs before state commit.
- In `qsl/qsl-client/qsc/src/handshake/mod.rs`:
  - initiator `B1` handling emits `identity_unknown` on `Ok(None)` from `identity_read_sig_pin(...)` and still proceeds to `qsp_session_store(...)`;
  - responder `A1` handling accepts `Ok(None)` from both `identity_read_pin(...)` and `identity_read_sig_pin(...)`, then persists `hs_pending_store(...)`;
  - responder `A2` handling emits `identity_unknown` on missing pins and still proceeds to `qsp_session_store(...)`;
  - `hs_build_session(...)` calls `init_from_base_handshake(...)` with `authenticated=true`.
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs` proves the happy-path handshake succeeds with route setup only; the test does not pin peer identity before establishment.
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs` proves pinned mismatches reject, but only after a pin exists.

#### Minimal fix direction

- Reject handshake establishment before any `hs_pending_store(...)` or `qsp_session_store(...)` when authenticated peer identity is absent.
- Stop asserting authenticated base-handshake inputs unless the qsc-local handshake actually binds the required canonical establishment tuple, or extend the qsc handshake state initializer inputs so the authenticated claim is true.
- Add explicit negative regressions for unpinned/unknown peers and for missing authenticated base-handshake commitment.

#### Proof gap

- No regression currently proves `identity_unknown` is rejected with zero pending/session mutation.
- No regression proves reject behavior when the canonical base-handshake commitment tuple is absent, because qsc currently hardcodes the authenticated flag and suite constants internally.

#### Recommended directive shape

- Implementation-only remediation lane limited to the qsc handshake seam and its direct tests.

### P2

#### Finding ID

- `NA0220-F002`

#### Severity

- `P2`

#### Exact surfaces

- Files/functions:
  - `qsl/qsl-client/qsc/src/handshake/mod.rs`
  - `perform_handshake_poll_with_tokens(...)`
  - `handshake_status(...)`
- Tests:
  - `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
  - `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
  - `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- Spec/queue concerns:
  - `NA-0220` core audit question on operator-visible status truth

#### Violated claim/invariant

- Operator-visible markers and status surfaces should not overstate handshake success relative to the durable peer state actually achieved.

#### Why it matters

- The initiator persists the session and clears pending state before the responder consumes `A2`.
- `handshake_status(...)` reports `status=established` solely from local session presence, so local status can imply mutual completion while the responder still has no durable session.
- Desktop/TUI layers that mirror this status can present stronger readiness than the audited state machine actually guarantees at that instant.

#### Evidence/proof

- In `qsl/qsl-client/qsc/src/handshake/mod.rs`, the initiator `B1` accept path stores via `qsp_session_store(...)`, clears pending, sends `A2`, and emits `handshake_complete` before the responder processes `A2`.
- In `qsl/qsl-client/qsc/src/handshake/mod.rs`, `handshake_status(...)` maps any successful `qsp_session_load(...)` to `status=established` or `status=established_recv_only`.
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs` shows that after Alice processes `B1`, `alice/bob` session state already exists while `bob/alice` session state still does not.
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs` and `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs` validate status only after the full exchange, so the midpoint status truth is currently untested.

#### Minimal fix direction

- Represent the midpoint explicitly, such as `awaiting_peer_confirm`, or otherwise distinguish local session material from fully confirmed mutual establishment.
- Add a targeted CLI/TUI/desktop regression that checks status after the initiator stores state but before the responder confirms.

#### Proof gap

- No existing regression exercises `handshake status` at the midpoint between initiator local commit and responder final commit.
- No desktop-side contract test proves that sidecar/UI surfaces distinguish local readiness from peer-confirmed readiness during that window.

#### Recommended directive shape

- Bounded implementation lane for handshake-status/marker truth plus targeted qsc and qsc-desktop regressions.

## Consolidated answer to the five core audit questions

1. Are all accept paths bound to the correct transcript material?
   - Partially. The qsc-local handshake binds its own `A1/B1/A2` transcript elements, but it does not establish the full canonical base-handshake commitment required by `DOC-CAN-003` before treating the resulting Suite-2 state as authenticated.
2. Do identity mismatches reject fail-closed without partial accept?
   - Yes when a pin already exists; the targeted mismatch regression passes. No for unknown/unpinned peers, because the current code emits `identity_unknown` and still commits pending/session state.
3. Can malformed, replayed, or downgraded handshake inputs mutate session or protocol state?
   - Malformed/tampered and pinned-mismatch paths are covered and currently reject without session mutation in the exercised cases. Canonical downgrade/authenticated-establishment proof is still missing, and unauthenticated first-contact input can mutate pending/session state because the implementation accepts `identity_unknown`.
4. Can invalid handshake input leave disk state partially updated?
   - For the exercised transcript-tamper and pinned-mismatch cases, no. For unauthenticated first-contact input, yes in the canonical sense: responder pending state and later full session state can still be written before the spec-required authentication boundary is met.
5. Do operator-visible markers or desktop-facing status surfaces overstate handshake success?
   - Yes. The current local status model can report the initiator as established before the responder has durably completed the handshake, and there is no midpoint regression proving the UI surfaces stay honest there.

## Recommended remediation shapes

- Remediation shape 1:
  - Scope: `qsl/qsl-client/qsc/src/handshake/**`, direct tests only
  - Goal: reject unknown/unpinned establishment attempts before pending/session mutation and make the authenticated base-handshake claim truthful
  - Required proof: negative regressions for unknown-peer reject/no-mutation and any newly introduced authenticated-establishment contract
- Remediation shape 2:
  - Scope: handshake status/marker surfaces and the direct qsc/qsc-desktop status tests
  - Goal: prevent midpoint local state from being reported as fully established without peer-confirmed completion
  - Required proof: targeted CLI/TUI/desktop regression for the post-`B1`, pre-responder-confirm window

## Explicit conclusion

P0/P1 findings found; remediation required before further confidence claims.

Finding counts for this audited surface:

- `P0`: 0
- `P1`: 1
- `P2`: 1
- `P3`: 0
