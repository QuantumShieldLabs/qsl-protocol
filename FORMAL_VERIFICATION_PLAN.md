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
- **Primary (next):** ProVerif and/or Tamarin (select and record in DECISIONS.md once chosen) for cryptographic properties (secrecy/authentication/PCS) beyond the bounded logic model.

CI integration:
- `.github/workflows/formal.yml` executes `python3 formal/run_model_checks.py` fail-closed.

## Current status
- A bounded, crypto-agnostic SCKA logic model exists under `formal/` and is executed in CI (formal-ci).
- The next formalization milestone is selecting ProVerif vs Tamarin and introducing a first cryptographic model aligned to Suite-2 transcript binding and the hybrid key schedule.
