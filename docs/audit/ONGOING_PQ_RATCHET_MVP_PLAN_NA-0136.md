# NA-0136 Ongoing PQ Ratchet MVP Plan (Design -> Tests First)

## Scope
This is a docs-only implementation plan derived from NA-0135. No code changes are included.

## Adjacency Scan (Repo-Local Grounding)
This plan is grounded in the current protocol/client implementation and tests:

- Handshake and ratchet implementation anchors:
  - Handshake decode/type/version guards: `qsl/qsl-client/qsc/src/main.rs:7348`, `qsl/qsl-client/qsc/src/main.rs:7395`, `qsl/qsl-client/qsc/src/main.rs:7444`
  - Handshake init/poll/status flow: `qsl/qsl-client/qsc/src/main.rs:8541`, `qsl/qsl-client/qsc/src/main.rs:8606`, `qsl/qsl-client/qsc/src/main.rs:8492`
  - Transcript/PQ derivation helpers: `qsl/qsl-client/qsc/src/main.rs:8310`, `qsl/qsl-client/qsc/src/main.rs:8326`, `qsl/qsl-client/qsc/src/main.rs:8363`, `qsl/qsl-client/qsc/src/main.rs:8428`
  - Ratchet send/recv advancement and replay reject: `qsl/qsl-client/qsc/src/main.rs:9576`, `qsl/qsl-client/qsc/src/main.rs:9727`, `qsl/qsl-client/qsc/src/main.rs:10104`

- Message/file truth-state implementation anchors:
  - qsp ACTIVE/INACTIVE truth mapping: `qsl/qsl-client/qsc/src/main.rs:6080`
  - Message reject/no-mutation paths: `qsl/qsl-client/qsc/src/main.rs:6001`
  - File-transfer reject/no-mutation paths: `qsl/qsl-client/qsc/src/main.rs:6698`

- Existing tests/vectors relevant to ratchet and reject semantics:
  - `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
  - `qsl/qsl-client/qsc/tests/ratchet_step.rs`
  - `qsl/qsl-client/qsc/tests/message_state_model.rs`
  - `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/timeline_store.rs`
  - `qsl/qsl-client/qsc/tests/receive_no_mutation.rs`
  - `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`
  - `qsl/qsl-client/qsc/tests/relay_dup_no_mutation.rs`
  - `qsl/qsl-client/qsc/tests/relay_reorder_no_mutation.rs`

- Sharp edges to preserve:
  - Deterministic marker contract: `QSC_MARK/1` emission in `qsl/qsl-client/qsc/src/main.rs:11147`
  - Reject paths must remain no-mutation and deterministic.
  - Do not weaken ACTIVE/INACTIVE truthfulness semantics.

## A) Context and Goals
NA-0135 selected a staged ongoing-PQ direction:
- Phase 1 MVP: deterministic periodic PQ rekey epochs.
- Phase 2: SPQR-like sparse ongoing PQ refresh once phase-1 vectors and budgets are proven.

This NA-0136 plan defines MVP design and test requirements for phase 1, with explicit criteria for later phase progression.

Target properties:
- Classical FS/PCS: improve evidence coverage and compromise-recovery guarantees.
- PQ-resilient FS/PCS: move from handshake-only/partial evidence toward ongoing refresh evidence.

## B) Threat-Model Alignment
This MVP addresses threat-model elements from `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md`:
- Active network attacker: replay/injection/reordering of rekey traffic.
- Malicious relay: reorder/drop/duplicate attempts and traffic manipulation.
- Future decryption risk model: long-lived sessions requiring periodic PQ contribution refresh.

MVP does not solve global metadata-hard goals (cover traffic/mixnets); those remain outside this NA.

## C) MVP Definition (Explicit)
Included in MVP:
- Periodic PQ rekey epoch mechanism layered onto current session/ratchet progression.
- Deterministic schedule and deterministic reject/no-mutation behavior.
- Transcript/context binding for each PQ-rekey step.
- Downgrade/feature gating rules and explicit failure behavior.
- Vector/test plan written before implementation.

Excluded from MVP:
- Full SPQR-style sparse continuous PQ ratchet.
- Broad protocol redesign unrelated to PQ rekey cadence.
- Performance deep-dive benchmarking lane (only basic budgets in MVP tests).

MVP done means:
- All required vector categories exist and pass.
- Claims are evidence-backed as Established/Partial/Not established.
- No unresolved P0 deterministic reject/no-mutation gaps for new PQ-rekey paths.

## D) Ratchet Schedule Rules (Explicit)
MVP schedule proposal (deterministic):
- Rekey trigger interval: every `N=64` application messages per direction, or `T=30 minutes` since last successful PQ rekey, whichever occurs first.
- Epoch counter:
  - `pq_epoch` starts at 0 after handshake activation.
  - Each successful PQ rekey increments `pq_epoch` by 1.
- Trigger precedence:
  - If both sides trigger concurrently, tie-break deterministically using lexical order of stable peer labels and monotonic epoch expectation.
- Compromise suspicion trigger:
  - Manual/automatic compromise signal forces immediate rekey attempt and sets `pending_forced_rekey=true` until success/fail-closed outcome.

Determinism requirements:
- No random trigger timing in MVP schedule.
- Duplicate/out-of-window rekey messages reject deterministically.

## E) State Machine and Message Types (Explicit)
Proposed session state additions:
- `pq_epoch: u64`
- `pq_last_rekey_msg_idx_send: u64`
- `pq_last_rekey_msg_idx_recv: u64`
- `pq_last_rekey_unix: u64`
- `pq_rekey_required: bool`
- `pq_rekey_in_flight: bool`
- `peer_pq_capable: bool`
- `min_required_pq_epoch: u64` (policy gate for strict mode)

Structural message additions (design-level):
- `PQ_REKEY_INIT`:
  - fields bound: session id, sender/receiver ids, current transcript hash, `pq_epoch_proposed`, send/recv counters snapshot, KDF domain id.
- `PQ_REKEY_ACK`:
  - fields bound: same session/transcript context, accepted epoch, confirmation authenticator.
- `PQ_REKEY_REJECT`:
  - deterministic reason code only; no state mutation except audit counter/marker.

Transcript binding requirements:
- Every PQ-rekey message binds:
  - current session transcript root/hash
  - current and proposed epoch counters
  - role identity labels and direction
  - strict KDF domain separation labels for rekey-derived material
- Rekey acceptance MUST fail if any bound field mismatches expected local state.

## F) Downgrade/Upgrade Prevention Plan
Capability/version strategy:
- Introduce explicit `pq_rekey_capability=v1` in handshake/session capability context.
- Strict mode (policy-enabled): reject peers without capability after policy activation point.
- Compatibility mode: allow existing sessions but mark status as `PQ_ONGOING_NOT_ESTABLISHED` until rekey-capable peer session exists.

Deterministic reject rules:
- Missing capability in strict mode -> reject with deterministic code, no mutation.
- Epoch regression/rollback -> reject, no mutation.
- Unexpected version -> reject, no mutation.

Backward compatibility stance:
- MVP defaults to compatibility mode unless explicitly configured to strict.
- Strict mode rollout only after vector/test parity and operational confidence.

## G) Test Plan (Core Deliverable)
Implementation NA must include vectors/tests with these categories:

1. PQ PCS/FS vector categories:
- Compromise-window simulation:
  - pre-rekey compromise cannot decrypt post-successful-rekey traffic (within defined model assumptions).
- Epoch advancement validation:
  - successful rekey changes derived context and invalidates old epoch expectations.

2. Replay/rollback tests:
- Duplicate `PQ_REKEY_INIT` and duplicate `PQ_REKEY_ACK` rejects.
- Older-epoch and skipped-epoch attempts reject deterministically.
- Relay reorder/drop/dup patterns validated with no mutation.

3. No-mutation-on-reject coverage:
- All new reject codes must prove persistent state unchanged:
  - session snapshot hash unchanged
  - timeline/state stores unchanged except deterministic audit markers.

4. Deterministic markers and CI support:
- Add marker events for rekey start/accept/reject with stable code fields.
- Preserve existing `QSC_MARK/1` format and semantics.
- CI verification checks must assert marker presence and reject determinism.

5. Basic performance sanity checks:
- Message-size overhead envelope for rekey frames.
- Rekey cadence overhead at nominal traffic rates.
- Simple CPU-time budget checks in test harness (not full perf lane).

Success criteria for test plan:
- Every new failure mode has deterministic reject code and no-mutation test.
- Replay/rollback coverage includes at least one relay-based disorder scenario.

## H) Rollout Plan
Rollout phases:
1. Feature-flagged implementation in compatibility mode.
2. Internal/test lanes validate vectors and marker evidence.
3. Enable strict mode in non-default environments first.
4. Promote strict-mode default only after acceptance gate review.

Half-enabled state avoidance:
- Session metadata must encode capability and epoch state explicitly.
- If peer/session capability unknown, do not claim ongoing PQ established.
- Mixed capability sessions must surface explicit status (not implicit upgrade).

## I) Follow-on NAs Proposed by This Plan
Proposed follow-ons (if not already tracked):
- NA: PQ Rekey Wire/State Schema Freeze
  - Acceptance: finalized state/message schema with backward-compat notes.
- NA: PQ Rekey Vector Pack (Adversarial)
  - Acceptance: replay/rollback/no-mutation vectors merged and CI-gated.
- NA: PQ Rekey Strict-Mode Rollout Gate
  - Acceptance: strict mode enablement checklist + operational guardrails.

These follow-ons remain subordinate to NA-0135/0136 staged decision and must not bypass deterministic reject/no-mutation requirements.
