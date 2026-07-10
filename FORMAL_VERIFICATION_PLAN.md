# Formal Verification Plan (Living)

Goals: G4

This plan satisfies GOALS.md (G4) and defines the verification roadmap.

## Near-term (immediately)
Maintain a machine-readable list of security properties to prove:
- message-key secrecy under active attacker
- authentication of peers and transcript binding
- post-compromise security recovery properties (classical + PQ contribution)
- downgrade resistance and fail-closed negotiation
- rollback detection / persistence invariants (modeled as adversary control of stored state)

## Incremental modeling approach
1. Establish a CI-gated executable model surface (`formal/`) and run a first bounded model.
2. Key schedule model: encode G1 always-hybrid derivation and show secrecy under assumptions.
3. Negotiation model: encode G3 transcript-bound capability commitment and show no silent fallback.
4. SCKA epochs model: encode G2 monotonicity and deterministic acceptance rules (beyond the initial bounded logic model).
5. Expand CI integration: run model checks on PRs that touch protocol surfaces, and add broader models as they mature.

## Tooling
- **Immediate (shipped):** a bounded state-space explorer in Python under `formal/` to enforce key SCKA logic invariants and to establish the CI lane.
- **Primary (SELECTED — ProVerif, D-1249 / NA-0627):** ProVerif **2.05** (version-pinned) for cryptographic properties (secrecy/authentication/PCS) beyond the bounded logic model. Rationale recorded in DECISIONS.md at D-1249: ProVerif's applied-pi/Horn-clause abstraction terminates well on key-exchange secrecy and injective-agreement queries and has first-class `phase` support for the post-compromise/healing queries this protocol needs. **Recorded tradeoff:** Tamarin's multiset rewriting handles unbounded ratchet state more naturally; ENG-0035 records where that bit (see Current status).

CI integration:
- `.github/workflows/formal.yml` executes `python3 formal/run_model_checks.py` fail-closed (the bounded Python explorers — the fast, always-on regression guard).
- `.github/workflows/formal.yml` also executes `python3 formal/proverif/run_proverif_checks.py` fail-closed (the ProVerif composition model). The gate asserts the expected `RESULT` line **per query**, not a zero exit, and its **first** assertion is the tool sanity pair: a positive control that must prove and a negative control that must refute. If the negative control ever returns `true`, the verifier is vacuously accepting and the job fails.

## Current status
- A bounded, crypto-agnostic SCKA logic model exists under `formal/` and is executed in CI (formal-ci).
- **The tooling milestone is DISCHARGED (NA-0627 / ENG-0028, D-1249).** A ProVerif model of the Suite-2 DH+PQ composition **as shipped post-NA-0626** lives in `formal/proverif/` and is CI-gated. It covers establishment (§8.2), the DH boundary (§8.5.2), the PQ reseed (§8.5.3), the combined DH+PQ boundary (§8.5.2+§8.5.3), and the authenticated SCKA advertisement (§8.5.4 + ADVAUTH). Proved: message-key secrecy and injective transcript agreement under an active Dolev-Yao adversary; PQ healing across a reseed and across a combined boundary with **all** classical DH secrets compromised; classical healing across a DH boundary with the ML-KEM decapsulation key compromised; and that a planted advertisement is never tracked.
- **Read the results with their limits.** `docs/design/DOC-G4-002` records, per query, what was proved, under which abstraction, and **what it does not license**. A green symbolic result is necessary input to — not sufficient grounds for — any post-quantum / Triple-Ratchet / post-compromise claim (Operator Decision 4: the claim boundary is UNCHANGED). Independent human review remains an open prerequisite.
- **Two findings, both filed, neither fixed in that analysis lane:**
  - **ENG-0034 (P2):** the X25519 DH output is never checked for the all-zero (non-contributory) value, and only one of Curve25519's small-order encodings is rejected on ingress. A symbolic DH model **cannot** decide this (the theory idealizes the group), so it was answered by code inspection against RFC 7748 §6.1. It blocks post-compromise claim language until fixed.
  - **ENG-0035 (P3):** ProVerif does not terminate on the design-locked 2-boundary unrolling. The main model was reduced to one DH boundary + one PQ reseed — the reduction is stated in the model header and no query was weakened — and the **Tamarin option is re-presented** for that query shape.
- Next candidates: the ENG-0034 remediation lane (refimpl + negative vectors); optionally a Tamarin lane for the multi-epoch unrolling (ENG-0035); extending the model to out-of-order/skip-window receives (abstraction A7).
