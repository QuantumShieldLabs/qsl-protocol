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

**Update (NA-0627 / ENG-0028, D-1249): ProVerif 2.05 is now the selected cryptographic-model tool**, and a model of the Suite-2 DH+PQ composition lives in `formal/proverif/` (§8 below). The bounded Python explorers below are **not** superseded: they remain the fast, always-on regression guard for control-plane *logic*. ProVerif is the layer above, reasoning about secrecy and authentication against an active adversary — the properties the explorers explicitly do not cover (§5).

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

The fourth model (NA-0625 / ENG-0023, Operator Decision 4) checks the **Suite-2 root-composition
slice** — the layer beneath the SCKA logic model, where the classical DH ratchet, the PQ reseed,
and the SCKA advertisement compose over the shared root and the directional chains:
- root convergence: after any delivered in-order schedule both parties hold the same root, and
  per party the PQ-path root and the DH-ratchet root agree (`recv.rk == dh.rk`);
- PQ healing survives a subsequent DH boundary (every absorbed epoch secret stays in the root
  lineage) — the D560 amendment property, at epoch granularity;
- chain continuity under the authenticated ADV's chain-consume: `nr` advances exactly once per
  delivered frame including an advertisement, so an in-order schedule opens no receive-chain gap;
- send/receive schedule coherence after a reseed RECEIVE (header keys and PQ chains, both
  directions); and
- reject implies no state mutation, including the §8.5.1 HK-downgrade boundary frame, a spoofed
  ADV header, and an ADVAUTH MAC computed under a foreign root.

Crypto is abstracted to injective tuple hashes (a key *is* its derivation history), so the model
reasons about agreement and coherence, not computational secrecy. The ProVerif composition model
remains ENG-0028's own lane.

Authoritative sources for meaning:
- DOC-CAN-003 §8.5 (boundaries; §8.5.1 NHK, §8.5.3 reseed, §8.5.4 advertisement)
- DOC-CAN-004 §3 (SCKA control plane)
- NA-0625 design-lock evidence (ENG-0023)

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

## 8. ProVerif composition model (`formal/proverif/`) — NA-0627 / ENG-0028

The layer above the bounded explorers: a **symbolic (Dolev-Yao) model of the Suite-2 DH+PQ
composition as shipped post-NA-0626**, in ProVerif 2.05 (version-pinned, D-1249).

Files:
- `suite2_dhpq_lib.pvl` — the shared theory, the 20-label derivation alphabet, and the
  **fidelity map**: every derivation names the shipped refimpl function and the DOC-CAN section
  it implements. Loaded by every model via `proverif -lib`, so the encoding cannot drift.
- `suite2_dhpq_main.pv` — Q1 message-key secrecy, Q2 injective transcript agreement, Q6 the
  control plane (a planted advertisement is never tracked), Q7 the guard-form zero-key query.
- `suite2_dhpq_q3_pq_reseed_healing.pv` — PQ healing across a **PQ reseed**, all classical DH
  secrets compromised.
- `suite2_dhpq_q4_combined_healing.pv` — PQ healing across the **combined DH+PQ boundary**, plus
  Q7's combined-boundary arm.
- `suite2_dhpq_q5_dh_healing.pv` — classical healing across a **DH boundary**, ML-KEM
  decapsulation key compromised.
- `run_proverif_checks.py` — the fail-closed gate.

**Q3 + Q4 + Q5 together are the hybrid claim** (security survives if *either* primitive survives).
Quoting one direction alone misstates the composition.

Running locally (requires `proverif` 2.05 on `PATH`):

- `python3 formal/proverif/run_proverif_checks.py`

The gate asserts the expected `RESULT` line **per query** — not a zero exit — and its **first**
assertion is the **tool sanity pair**: a positive control that must prove (`is true.`) and a
negative control that must refute (`is false.`). *A verifier that only ever answers "true" is
worse than no verifier*: if the negative control returns `true`, the gate stops before any
protocol model runs. Several expected results are deliberately `is false.` — the healing models'
canaries (pre-heal traffic must be readable under the modeled leak, or the healing green beside
it would be vacuous) and the reseed/combined frames' own bodies, which ride the pre-reseed key
schedule by design. Asserting the reds is what proves the model models the compromise.

### Scope limits (read before quoting any result)

`docs/design/DOC-G4-002` is the authoritative record: per query, what was proved, under which
abstraction, and **what it does not license**. In short: one session, two parties, establishment
authentication **assumed**, in-order receives only, one root-advancing DH epoch per model,
idealized KDF/AEAD/X25519-group/ML-KEM, and **no low-order or small-subgroup reasoning of any
kind** — a symbolic DH theory cannot express it (ENG-0034 answers that question outside the
model; the non-termination at the 2-boundary bound is ENG-0035).

A green symbolic result is **necessary input to**, not **sufficient grounds for**, any
post-quantum / Triple-Ratchet / post-compromise claim. Independent human review remains an open
prerequisite.
