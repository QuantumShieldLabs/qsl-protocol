Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-04-30

# QuantumShield Roadmap

## Current posture

QuantumShield remains a research-stage protocol and demo system. It is not production-ready, and project artifacts must not claim production readiness until the release-readiness gates in `GOALS.md`, the canonical specs, conformance vectors, formal checks, demo acceptance criteria, and public-safety gates all support that claim.

The immediate recovery sequence is now complete enough to resume forward engineering: the dependency advisory was remediated, the `send_commit` regression was repaired without restoring retired mock-provider behavior, `public-safety` was restored as a required check and completed green after PR `#723`, and the fail-closed KT verifier implementation merged through PR `#708`.

## Roadmap principle

Governance supports engineering. Future work should normally produce at least one of:

- executable behavior
- invariant tests
- conformance vectors
- demo acceptance behavior
- release-hardening automation

Pure governance-only PRs are exceptional and should be limited to queue integrity, CI deadlock recovery, traceability required by implementation, or release-control decisions that unblock executable work without weakening fail-closed gates.

## 30-day priorities

- Public-safety red-main deadlock prevention hardening: implement bounded executable gate tests and helper validation so known non-advisory repair lanes do not require branch-protection exceptions.
- KT closeout and successor hygiene: keep KT verifier evidence tied to `DOC-CAN-008`, D-0440, and the merged PR `#708` proof without reopening closed recovery branches.
- SCKA persistence and monotonicity next: turn the existing normative requirements into stronger restart, rollback, tombstone, and monotonicity vectors.
- Downgrade-resistance vectors: expand negative coverage where both peers support Suite-2 and any fallback or transcript mismatch must reject.
- No-state-mutation rejection proofs: prioritize stateful reject paths where failed verification, failed decrypt, rollback, or malformed input could otherwise advance durable state.
- Demo acceptance: define and execute one-command acceptance that proves valid send/decrypt and invalid/downgrade/malformed rejects without production-readiness overclaims.

## 60-day priorities

- Conformance vector expansion: add high-value KT, SCKA, downgrade, no-mutation, metadata, and demo acceptance vectors before broad feature expansion.
- Demo hardening: make the demo easier to inspect and repeat while keeping unsafe overrides explicit and non-production.
- Metadata conformance expansion: extend store-permission, token-auth, padding, identifier, queue-cap, and error-surface checks.
- Release-readiness evidence: accumulate reproducible local and CI evidence for each G1-G5 gate, including model-check and public-safety results.

## 90-day priorities

- External review package: prepare a bounded, self-contained protocol/review bundle with current limits and known gaps.
- Reproducible conformance harness: make vector execution repeatable across local Linux, CI Linux, and macOS where relevant.
- Demo app reviewability: keep demo paths clear enough that reviewers can trace valid and invalid flows without reading unrelated implementation seams.
- Clear non-production limits: preserve explicit research/demo labels until release evidence proves otherwise.

## Non-goals

- Do not use roadmap paperwork as a substitute for implementation, tests, vectors, or release-hardening automation.
- Do not normalize branch-protection exceptions or public-safety bypasses.
- Do not start unapproved implementation lanes outside the sole READY item in `NEXT_ACTIONS.md`.
