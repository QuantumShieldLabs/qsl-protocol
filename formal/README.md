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

The fifth model (NA-0636 / D572) checks the **`QSC.HS.*` handshake AUTHENTICATION slice** as
shipped post-NA-0634 — the establishment step every other model on this page *assumes*
(§8's ProVerif scope limits say "establishment authentication **assumed**"; this is the model
that stops assuming it on the bounded-explorer side):
- identities are opaque `(kem_id, sig_id)` pairs; the single verified code is the injective
  combined fingerprint `CODE(kem_id, sig_id)`, and a contact pin binds BOTH keys;
- the initiator's ACCEPT at B1 requires KEM-possession of the *pinned* responder identity key
  (NA-0633 C1) **and** the NA-0634 REQUIRED responder sig-pin;
- the responder's PRIMARY pin recomputes the combined code from the presented pair, and the
  reverse (responder→initiator) sig-pin is modeled **OPTIONAL, exactly as landed**;
- the contact store ranges over every state provisioning can reach (full / legacy KEM-only /
  bare-code / absent), including **mid-run re-pinning** between A1 and A2, under an adversary
  holding any subset of the honest identity secrets.

It exists to **discharge the open ENG-0038 verification obligation** filed at NA-0634 closeout:
whether the combined primary pin makes the separate optional signing-key pin redundant. That was
asserted on reasoning; this model decides it by exhaustive search (see P13–P16 and §5).

Two counterfactuals keep the result honest, because a "no counterexample found" claim is only
worth what the search could have found (WF-0017):
- **faithfulness anchor** — rewind the landed defences and the model *reproduces the real,
  known ENG-0038 flaw*: an adversary that has stolen **nothing**, signing B1 with a keypair it
  generated itself, makes the initiator commit with `authenticated=true`. Under the landed rules
  that count is **zero**, as is the sharper case of an adversary holding the responder's KEM
  identity secret but not its signing secret.
- **non-vacuity** — rewind *only* the primary pin to its pre-NA-0634 KEM-only form and the P3
  search surfaces an unbound-signing-key commit a required reverse pin would have caught. The
  redundancy verdict is therefore **contingent on the combined code covering the signing key
  injectively** — that coverage is what NA-0634 bought, and a code format that stopped covering
  the signing half would make the reverse pin load-bearing again.

Authoritative sources for meaning:
- DOC-CAN-003 §6.3 (authenticate peer identity before Suite-2 state is committed)
- ENG-0038 (`docs/ops/IMPROVEMENT_LEDGER.md`) and its open verification obligation
- NA-0633 design-lock (C1) and NA-0634 as-built evidence; NA-0636 as-built §1 (the read-only
  extraction of the accept/reject rules this model encodes)

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
- **P13 (QSC.HS mutual-auth binding):** in every reachable accepting state — initiator-accept or
  responder-commit — the accepting party's counterparty holds BOTH the KEM secret and the signing
  secret of the identity whose combined code is pinned. No party accepts a peer whose `kem_id` or
  `sig_id` is not bound to the single verified code.
- **P14 (QSC.HS wrong-signing-key rejection):** a peer presenting the correct KEM identity but a
  wrong signing identity always reaches a deterministic fail-closed reject (the NA-0634 required
  sig-pin), with no commit and no success output.
- **P15 (QSC.HS reverse-pin redundancy — the ENG-0038 obligation):** with the responder→initiator
  sig-pin OPTIONAL exactly as landed, **no** reachable responder-commit leaves the initiator's
  presented signing key unbound to the verified code. The optional reverse pin is therefore
  redundant *given the combined primary pin* — decided by exhaustive search over the bound in §5,
  not by argument.
- **P16 (QSC.HS fail-closed reject hygiene):** every handshake-authentication reject commits no
  session, emits no success output, mutates no durable state, and carries a deterministic reason
  label drawn from a fixed set.

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
- The `QSC.HS.*` handshake-authentication model (NA-0636) proves a bounded authentication-**binding**
  property over an abstract state machine. It is **not** cryptographic security, **not** a
  side-channel property, **not** a post-compromise or PQ guarantee, and **not** a qsc/refimpl
  equivalence claim. Specifically:
  - **fingerprint collision-resistance is ASSUMED, not proved.** Codes are injective structured
    tokens (the standard symbolic abstraction). The P15 redundancy verdict *depends* on the
    combined code binding the signing key injectively; the model makes that dependency visible via
    its pre-fix counterfactual rather than hiding it. A truncated-hash collision analysis is out of
    scope for a crypto-agnostic model.
  - the bound is small and fixed: 3 identities (2 honest + 1 adversary), 3 KEM and 3 signing keys,
    5 contact-pin states per side explored independently at A1 and A2 (mid-run re-pin), 3 message
    producers, all 16 subsets of the four honest identity secrets as adversary compromise, and 3
    reverse-pin modes — **10,800 responder configurations and 10,800 initiator configurations,
    enumerated exhaustively**. Behaviour outside that bound is not covered.
  - it models a single bounded handshake per configuration; it is not a multi-session,
    concurrent-handshake, or cross-session replay proof.
  - it abstracts KEM and signature *possession* as capability sets. It says nothing about the
    computational hardness of ML-KEM or ML-DSA.
  - **it flattens the contact-store DEVICE INDIRECTION.** The model represents the pin store as a
    single coherent `(pin_code, kem_stored, sig_fp)` triple — an *abstraction*, not a proved
    invariant. The shipped code resolves the pin reads through a primary-device indirection whose
    coherence is upheld by `normalize_contact_record` (run on every store load/save); that
    justification was established by **reading the code, not by the model**. Primary-device
    *selection* and a change of primary device mid-handshake are likewise unmodeled. The P1/P3
    results are **argued** to survive this — the signing-key binding flows entirely from the
    REQUIRED primary pin, so a stale `sig_fp` can only false-reject, never admit — but that argument
    is **REASONED, not model-verified**. Recorded as a known unmodeled slice on the ENG-0038 ledger
    entry, with extending the model to it named as a candidate follow-up lane.
  - the **composition** of authentication with suite negotiation/downgrade is covered by neither this
    model nor the negotiation models — each covers its own slice.
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
