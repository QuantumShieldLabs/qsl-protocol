# Formal model checks (CI-gated)

Goals: G4

## 0. Purpose

This directory contains **machine-executable model checks** that serve as an initial, fail-closed verification gate.

The near-term objective is to:
- establish an executable model surface in-repo,
- make the modeled security properties explicit,
- and run these checks in CI so regressions are blocked automatically.

## 1. Tool decision (initial)

We start with a **bounded state-space explorer implemented in Python**.

Rationale:
- zero external dependencies (runs in local dev and GitHub Actions without system packages)
- deterministic execution and stable outputs
- fail-closed by construction (any property violation aborts the job)

This is an intentionally conservative first step that does **not** preclude adopting ProVerif and/or Tamarin later (see FORMAL_VERIFICATION_PLAN.md). It exists to ensure G4 has an executable gate immediately.

## 2. Modeled protocol slices (current)

The first model checks the **SCKA control-plane logic invariants**:
- peer ADV monotonicity acceptance/reject rules
- one-time ciphertext targeting + tombstoning
- “reject implies no state change”
- transactional commit semantics (abstracted; no partial commits)

Authoritative sources for meaning:
- DOC-CAN-004 §§3–6 (SCKA normative logic)
- DOC-TST-005 CAT-SCKA-LOGIC-001, CAT-SCKA-KEM-001 (coverage intent)

The second model checks the **Suite-2 required negotiation/downgrade slice**:
- both peers support Suite-2;
- weaker committed suites are rejected deterministically;
- inconsistent capability commitments are rejected deterministically;
- inconsistent negotiated-suite transcript views are rejected deterministically; and
- rejected inputs leave accepted/durable negotiation state unchanged.

Authoritative sources for meaning:
- DOC-CAN-003 §2 (downgrade resistance and capability commitment)
- DOC-TST-005 CAT-S2-DOWNGRADE-001 (coverage intent)

The third model checks the **future qsc handshake suite-id formal slice**
selected by NA-0307/NA-0308:
- valid QHSM v2 Suite-2 parameter context accepts in suite-id-required mode;
- legacy QHSM v1 accepts only under explicit compatibility mode;
- unsupported, downgraded, stripped, mismatched, duplicate, unknown,
  noncanonical, malformed, and inconsistent suite contexts reject
  deterministically;
- transcript and key-context binding are required in suite-id-required mode;
  and
- every reject has no accepted-state mutation, no output or recv_commit, no
  secret/sentinel leak, and no downgrade path to compatibility mode.

Authoritative sources for meaning:
- NA-0307 qsc handshake suite-id compatibility/transcript design evidence
- NA-0308 qsc handshake suite-id formal/vector design evidence

## 3. Roles and channels (model)

Roles:
- Party A
- Party B

Adversary model (network control):
- can reorder, drop, delay, and replay previously observed messages
- cannot forge an internally “accepted” message without satisfying SCKA’s explicit accept/reject rules

Cryptography model:
- not modeled; ciphertext is treated as an opaque value
- the model is purely about **state-machine invariants and fail-closed logic**

## 4. Properties checked (fail-closed)

The model checks the following properties for all explored executions within the configured bounds:

- **P1 (ADV monotonicity):** a peer ADV is accepted iff its `pq_adv_id` is strictly greater than the local `peer_max_adv_id_seen`.
- **P2 (one-time consumption):** a local advertised key id can be consumed at most once.
- **P3 (tombstones):** a tombstoned target id is always rejected.
- **P4 (no state change on reject):** rejecting a message does not mutate persistent state.
- **P5 (transactional commit):** key consumption and tombstoning occur only on commit (modeled as “accept”).
- **P6 (Suite-2 downgrade reject):** when both peers support Suite-2, any committed negotiated suite other than Suite-2 is rejected.
- **P7 (capability/suite commitment reject):** inconsistent Suite-2 capability commitments or negotiated-suite transcript views are rejected.
- **P8 (negotiation no-mutation on reject):** rejected negotiation attempts leave modeled accepted/durable state unchanged.
- **P9 (qsc suite-id canonicality):** future QHSM v2 suite context must be
  canonical, unique, bounded, and Suite-2-valued before acceptance.
- **P10 (qsc compatibility gate):** legacy QHSM v1 acceptance is explicit
  compatibility only, never explicit suite-id admission.
- **P11 (qsc transcript/key context):** suite-id-required mode accepts only
  when transcript and key-context bindings include the canonical suite context.
- **P12 (qsc reject boundary):** every qsc suite-id reject is deterministic
  and leaves modeled accepted state, output, recv_commit, and leak flags clear.

## 5. Scope limits

- Bounded exploration is not a proof of correctness outside the explored bounds.
- This model does not (yet) cover secrecy, authentication, transcript binding, or key schedule security.
- The negotiation model abstracts authenticated capability evidence and transcript binding into boolean/suite commitments. It does not prove cryptographic authentication or AEAD security.
- The model does not make claims about non-Suite-2 fallback lanes where Suite-2 is not mutually supported.
- The qsc handshake suite-id model describes future explicit semantics only;
  it is not a qsc runtime implementation, QHSM/QSP wire-format change, parser
  memory-safety proof, or cryptographic proof.
- The qsc model does not claim that current persisted qsc Suite-2 state is
  explicit qsc handshake suite-id admission evidence.
- The models are intentionally narrow to establish and expand the CI lane without overclaiming production proof.

## 6. Running locally

From the repo root:

- `python3 formal/run_model_checks.py`

The command exits non-zero on any violation.

## 7. CI integration

The model is executed in CI by:
- `.github/workflows/formal.yml`

The job is intended to be a release gate (G4): if the model fails, CI fails.
