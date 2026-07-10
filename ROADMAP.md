Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-07-10

# QuantumShield Roadmap

## Current posture

QuantumShield remains a research-stage protocol and demo system. It is not production-ready, and project artifacts must not claim production readiness until the release-readiness gates in `GOALS.md`, the canonical specs, conformance vectors, formal checks, demo acceptance criteria, and public-safety gates all support that claim.

The cryptographic core is now correctness-complete: the Suite-2 DH+PQ composition is unified on a single root (NA-0626), independently analyzed in a CI-gated ProVerif symbolic model (NA-0627, `docs/design/DOC-G4-002`), and the last known correctness gap — the RFC 7748 §6.1 non-contributory-DH check — is closed (NA-0628). No open P1 remains and there is no known correctness gap in the crypto core. The remaining gate on any post-compromise / production claim is now review, not engineering: independent human review plus the bounded ENG-0035 formal follow-up. Forward work is hardening, metadata (ENG-0022/ENG-0037), and the TUI/GUI + private-server product direction (ENG-0036).

## Roadmap principle

Governance supports engineering. Future work should normally produce at least one of:

- executable behavior
- invariant tests
- conformance vectors
- demo acceptance behavior
- release-hardening automation

Pure governance-only PRs are exceptional and should be limited to queue integrity, CI deadlock recovery, traceability required by implementation, or release-control decisions that unblock executable work without weakening fail-closed gates.

## 30-day priorities

- Strategic-docs truth-up (this lane, WF-0018 / D566): keep the strategic, program, and public/review-facing docs current with live truth, and assemble the external-review bundle on accurate inputs (the package now records the ProVerif analysis, the single-root composition, and the contributory-DH guard).
- ENG-0019 remediation: retire or neutralize the auth-unsafe `qsp` reference implementation so CI and the release provenance chain stop blessing it; the cheapest sub-item (stop shipping `refimpl_actor` in `release_artifacts/`) is a one-line change.
- Cheap hardening sweep: ENG-0032 / ENG-0033 (apps hygiene + public-safety gate hardening) and the NA-0627 CI-cost path-filter.

## 60-day priorities

- Constant-time hardening: ENG-0014 (qsl-server token compare) and the related constant-time family (ENG-0003/0005/0008/0015).
- ENG-0035 / Tamarin: the multi-epoch unrolling — pursued only if the post-compromise claim is being sought (a review of a model with a known non-terminating query reviews the wrong artifact).
- Commission independent human review — the standing prerequisite that no internal proof discharges — of the composition, the DOC-G4-002 abstraction table, and the contributory guard.

## 90-day priorities

- Metadata: cover-traffic / boundary-cadence (ENG-0022/ENG-0027) and sealed-sender (ENG-0037, analysis-first via a relay/sender-metadata audit) — the flagship "beat Signal on metadata" work, now un-parked as the crypto core reaches its completion point.
- Product direction: the TUI/GUI, and the token-gated private-server deployment with a setup-time public/private mode toggle (ENG-0036) — access control, not E2EE or metadata protection.
- Clear non-production limits: preserve explicit research/demo labels until release evidence and independent review prove otherwise.

## Non-goals

- Do not use roadmap paperwork as a substitute for implementation, tests, vectors, or release-hardening automation.
- Do not normalize branch-protection exceptions or public-safety bypasses.
- Do not start unapproved implementation lanes outside the sole READY item in `NEXT_ACTIONS.md`.
