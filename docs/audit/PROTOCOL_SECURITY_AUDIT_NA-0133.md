# NA-0133 Protocol Security Audit + SPQR/Triple Ratchet Gap Analysis

## Scope and Method
This is a docs-only audit artifact for NA-0133. No code changes are included.

Claim labels used in this report:
- Established
- Partially established
- Not established

Evidence policy:
- Every claim points to code and/or tests.
- If direct evidence is missing, the claim is marked Not established.

## Security Property Table

| Property | Target definition (from threat model) | Status | Evidence |
|---|---|---|---|
| Authentication / identity binding | Peer identity and session establishment are cryptographically bound | Partially established | Handshake signs transcript-bound messages and verifies signatures (`qsl/qsl-client/qsc/src/main.rs:8381`, `qsl/qsl-client/qsc/src/main.rs:8399`, `qsl/qsl-client/qsc/src/main.rs:8682`, `qsl/qsl-client/qsc/src/main.rs:8842`). TOFU/mismatch reject behavior is tested (`qsl/qsl-client/qsc/tests/identity_binding.rs:40`, `qsl/qsl-client/qsc/tests/identity_binding.rs:154`). Formal proof and independent audit evidence are not present in-repo. |
| Transcript binding (handshake integrity) | Session progression bound to validated transcript context | Established | Transcript MAC/hash and confirm MAC derivation are explicit (`qsl/qsl-client/qsc/src/main.rs:8310`, `qsl/qsl-client/qsc/src/main.rs:8318`, `qsl/qsl-client/qsc/src/main.rs:8358`, `qsl/qsl-client/qsc/src/main.rs:8366`). Reject paths for bad transcript/confirm are explicit (`qsl/qsl-client/qsc/src/main.rs:8677`, `qsl/qsl-client/qsc/src/main.rs:8826`). Tamper and out-of-order rejects are tested (`qsl/qsl-client/qsc/tests/handshake_mvp.rs:207`, `qsl/qsl-client/qsc/tests/handshake_mvp.rs:241`, `qsl/qsl-client/qsc/tests/handshake_mvp.rs:295`, `qsl/qsl-client/qsc/tests/handshake_mvp.rs:514`). |
| Replay resistance (messages/receipts/files) | Replay/reorder/injection are rejected or handled without false claims | Established | Ratchet replay rejection markers are explicit (`qsl/qsl-client/qsc/src/main.rs:7108`, `qsl/qsl-client/qsc/src/main.rs:9727`). Message-state replay rejects are explicit (`qsl/qsl-client/qsc/src/main.rs:6001`) and tested (`qsl/qsl-client/qsc/tests/message_state_model.rs:298`). File transfer replay and tamper rejects are tested (`qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:323`, `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:251`). |
| Forward secrecy (FS, classical) | Past ciphertext remains protected after later key compromise | Partially established | Ratchet send/recv advancement exists (`qsl/qsl-client/qsc/src/main.rs:10104`, `qsl/qsl-client/qsc/src/main.rs:9576`) and behavior is tested (`qsl/qsl-client/qsc/tests/ratchet_step.rs:100`, `qsl/qsl-client/qsc/tests/ratchet_step.rs:139`). No formal FS proof in this repo and no explicit compromise simulation proving FS boundary conditions. |
| Forward secrecy (FS, PQ-resilience) | FS includes post-quantum assumptions for relevant attacker model | Not established | PQ material appears in handshake/session derivation (`qsl/qsl-client/qsc/src/main.rs:8326`, `qsl/qsl-client/qsc/src/main.rs:8428`) and session state includes `ck_pq` (`qsl/qsl-client/qsc/src/main.rs:6426`). However, evidence of ongoing PQ ratchet evolution comparable to SPQR is not established in qsc-level protocol flow. |
| Post-compromise security (PCS, classical) | Security recovery after compromise and honest key evolution | Partially established | Ongoing ratchet state evolution is present (`qsl/qsl-client/qsc/src/main.rs:10104`, `qsl/qsl-client/qsc/src/main.rs:9576`) with replay/ordering protections (`qsl/qsl-client/qsc/src/main.rs:7108`, `qsl/qsl-client/qsc/src/main.rs:7126`). No explicit compromise-and-recovery test scenario in current test suite. |
| Post-compromise security (PCS, PQ-resilience) | PCS recovery remains robust under PQ attacker model | Not established | Current in-repo evidence does not show a continuous PQ ratchet layer with periodic PQ refresh comparable to SPQR/Triple Ratchet. |
| Key separation / domain separation | Distinct labels/contexts separate key derivations and contexts | Established | Labeled KMAC derivations are pervasive for handshake/session and metadata contexts (`qsl/qsl-client/qsc/src/main.rs:6422`, `qsl/qsl-client/qsc/src/main.rs:6423`, `qsl/qsl-client/qsc/src/main.rs:8310`, `qsl/qsl-client/qsc/src/main.rs:8331`, `qsl/qsl-client/qsc/src/main.rs:8363`, `qsl/qsl-client/qsc/src/main.rs:7164`). |
| Downgrade resistance / negotiation | No silent security downgrade during algorithm/protocol selection | Partially established | Protocol uses fixed version/type checks during handshake decode (`qsl/qsl-client/qsc/src/main.rs:7355`, `qsl/qsl-client/qsc/src/main.rs:7402`, `qsl/qsl-client/qsc/src/main.rs:7451`) and rejects invalid frames. There is no explicit in-band algorithm negotiation transcript binding evidence in qsc-level code, so downgrade resistance is only partial. |
| Deterministic reject + no-mutation on reject | Invalid paths fail closed without state mutation | Established | Reject paths are explicit across handshake/message/file flows (`qsl/qsl-client/qsc/src/main.rs:6001`, `qsl/qsl-client/qsc/src/main.rs:6698`, `qsl/qsl-client/qsc/src/main.rs:8637`). No-mutation tests exist for protocol/timeline/session/file reject scenarios (`qsl/qsl-client/qsc/tests/handshake_mvp.rs:207`, `qsl/qsl-client/qsc/tests/message_state_model.rs:199`, `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:126`, `qsl/qsl-client/qsc/tests/timeline_store.rs:121`, `qsl/qsl-client/qsc/tests/session_state_at_rest.rs:174`). |

## Protocol Walkthrough (Grounded)

### 1) Handshake flow
- CLI surface: `handshake init|poll|status` (`qsl/qsl-client/qsc/src/main.rs:8541`, `qsl/qsl-client/qsc/src/main.rs:8606`, `qsl/qsl-client/qsc/src/main.rs:8492`).
- Message structures and strict decoders: init/response/confirm with fixed magic/version/type (`qsl/qsl-client/qsc/src/main.rs:7348`, `qsl/qsl-client/qsc/src/main.rs:7395`, `qsl/qsl-client/qsc/src/main.rs:7444`).
- Transcript binding and auth:
  - transcript MAC/hash derivation (`qsl/qsl-client/qsc/src/main.rs:8310`, `qsl/qsl-client/qsc/src/main.rs:8318`)
  - signature message binding (`qsl/qsl-client/qsc/src/main.rs:8381`, `qsl/qsl-client/qsc/src/main.rs:8389`)
  - confirm MAC verification (`qsl/qsl-client/qsc/src/main.rs:8825`).
- Tests:
  - success path (`qsl/qsl-client/qsc/tests/handshake_mvp.rs:79`)
  - tamper/out-of-order/replay reject-no-mutation (`qsl/qsl-client/qsc/tests/handshake_mvp.rs:207`, `qsl/qsl-client/qsc/tests/handshake_mvp.rs:241`, `qsl/qsl-client/qsc/tests/handshake_mvp.rs:393`).

### 2) Session activation criteria (ACTIVE/INACTIVE truthfulness)
- Status mapping is explicit and reasoned (`qsl/qsl-client/qsc/src/main.rs:6080` to `qsl/qsl-client/qsc/src/main.rs:6098`).
- ACTIVE requires a valid stored session (`qsl/qsl-client/qsc/src/main.rs:6088`).
- Tests for status truth reasons (`qsl/qsl-client/qsc/tests/qsp_status_truthy.rs:62`, `qsl/qsl-client/qsc/tests/qsp_status_truthy.rs:80`, `qsl/qsl-client/qsc/tests/qsp_status_truthy.rs:98`).

### 3) Ratchet structure (send/recv chains + skipped handling)
- Ratchet send/recv advancement events (`qsl/qsl-client/qsc/src/main.rs:10104`, `qsl/qsl-client/qsc/src/main.rs:9576`).
- Replay/out-of-order reject markers (`qsl/qsl-client/qsc/src/main.rs:7108`, `qsl/qsl-client/qsc/src/main.rs:7110`).
- Skipped-message handling markers (`qsl/qsl-client/qsc/src/main.rs:9582`, `qsl/qsl-client/qsc/src/main.rs:9586`).
- Tests for in-order and out-of-order behavior (`qsl/qsl-client/qsc/tests/ratchet_step.rs:100`, `qsl/qsl-client/qsc/tests/ratchet_step.rs:139`).

### 4) Receipt semantics and truthful delivery states
- Delivered receipt request/ack parsing paths (`qsl/qsl-client/qsc/src/main.rs:6625`, `qsl/qsl-client/qsc/src/main.rs:7043`, `qsl/qsl-client/qsc/src/main.rs:9616`).
- Receipt disabled path explicit (`qsl/qsl-client/qsc/src/main.rs:9277`, `qsl/qsl-client/qsc/src/main.rs:9720`).
- Message state reject-on-invalid transitions (`qsl/qsl-client/qsc/src/main.rs:6001`).
- Tests for roundtrip and replay reject semantics (`qsl/qsl-client/qsc/tests/receipts_delivered.rs:168`, `qsl/qsl-client/qsc/tests/message_state_model.rs:298`).

## SPQR / Triple Ratchet Gap Analysis

### External references
- Signal SPQR announcement: <https://signal.org/blog/spqr/>
- Signal PQXDH specification: <https://signal.org/docs/specifications/pqxdh/>
- NIST PQC conference slide deck (Signal post-quantum ratcheting): <https://csrc.nist.gov/csrc/media/events/2025/sixth-pqc-standardization-conference/post-quantum%20ratcheting%20for%20signal.pdf>

### Signal summary (for comparison)
- PQXDH: post-quantum-capable initial key agreement.
- SPQR / Triple Ratchet: ongoing post-quantum ratcheting layer in addition to Double Ratchet-style evolution, targeting stronger long-lived PQ security properties under future compromise assumptions.

### QSL comparison (current evidence)
- QSL shows PQ material in handshake/session derivation and state fields (`qsl/qsl-client/qsc/src/main.rs:8326`, `qsl/qsl-client/qsc/src/main.rs:8428`, `qsl/qsl-client/qsc/src/main.rs:6426`).
- QSL also shows ongoing ratchet advancement and replay handling (`qsl/qsl-client/qsc/src/main.rs:10104`, `qsl/qsl-client/qsc/src/main.rs:9576`, `qsl/qsl-client/qsc/src/main.rs:7108`).
- Gap: this audit did not locate explicit evidence of a dedicated, continuous SPQR-like PQ ratchet refresh mechanism with explicit protocol semantics/documented guarantees equivalent to Signal's published approach.

Conclusion for SPQR-equivalent ongoing PQ ratcheting:
- Status: Not established.

### Engineering/design gap statement
Current evidence supports hybrid/PQ-influenced initialization and ratchet machinery, but does not establish an explicit SPQR-equivalent ongoing PQ ratchet construction, security definition mapping, and validation suite tied to those definitions.

## Findings and Follow-on NAs

### P0-1: Ongoing PQ ratchet equivalence to SPQR not established
- Severity: P0
- Evidence:
  - No explicit SPQR-equivalent protocol section in current audit artifacts or test lane.
  - QSL PQ-related derivations exist but do not, by themselves, establish SPQR-equivalent continuous PQ ratcheting guarantees (`qsl/qsl-client/qsc/src/main.rs:8326`, `qsl/qsl-client/qsc/src/main.rs:6426`).
- Follow-on NA recommendation:
  - NA title: "Design decision: ongoing PQ ratchet model (SPQR-like or alternative)"
  - Acceptance criteria:
    - explicit construction and state machine
    - explicit security-property mapping (FS/PCS classical + PQ)
    - deterministic reject rules for all invalid transitions.

### P1-1: FS/PCS claims lack explicit compromise-recovery test vectors
- Severity: P1
- Evidence:
  - Ratchet behavior tests exist (`qsl/qsl-client/qsc/tests/ratchet_step.rs:100`) but no explicit compromise-and-recovery vectors were found.
- Follow-on NA recommendation:
  - NA title: "Protocol adversarial vectors for FS/PCS boundary conditions"
  - Acceptance criteria:
    - deterministic tests simulating key compromise windows
    - recovery expectations codified with fail-closed assertions.

### P1-2: Downgrade-resistance claim is only partial
- Severity: P1
- Evidence:
  - Version/type frame checks are explicit (`qsl/qsl-client/qsc/src/main.rs:7355`, `qsl/qsl-client/qsc/src/main.rs:7402`) but explicit algorithm-negotiation downgrade defense proof is not documented.
- Follow-on NA recommendation:
  - NA title: "Negotiation and downgrade-resistance audit lane"
  - Acceptance criteria:
    - explicit negotiation model (or explicit no-negotiation invariant)
    - downgrade test vectors and deterministic reject proofs.

### P2-1: Security claim maturity labels are not yet surfaced as a maintained scoreboard
- Severity: P2
- Evidence:
  - This document introduces labels, but no recurring CI/report pipeline enforces upkeep.
- Follow-on NA recommendation:
  - NA title: "Security claim scoreboard automation"
  - Acceptance criteria:
    - machine-readable claim inventory
    - CI check requiring evidence links for any status promotion.

## Reproduction Commands (Audit Evidence Collection)
- `rg -n "handshake_|hs_|qsp_status|ratchet_|message_state_reject|file_xfer_" qsl/qsl-client/qsc/src/main.rs`
- `rg -n "handshake|ratchet|receipt|replay|no_mutation|file_transfer" qsl/qsl-client/qsc/tests`
- `rg -n "### NA-0133|Status:\s*READY" NEXT_ACTIONS.md`

## Unknowns and Assumptions
- This audit is limited to in-repo evidence and does not claim external formal verification unless explicitly linked.
- "Not established" does not mean "false"; it means evidence is not currently sufficient in this repository artifacts set.
