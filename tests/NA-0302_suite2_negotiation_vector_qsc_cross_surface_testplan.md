# NA-0302 Suite-2 Negotiation Vector and qsc Cross-Surface Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-17

Goals: G1, G2, G3, G4, G5

## Objective

Add executable Suite-2 negotiation vector proof and focused qsc cross-surface
fail-closed proof without changing protocol, wire, crypto state-machine,
handshake, key schedule, dependency, workflow, service, desktop, website,
README, START_HERE, docs/public, branch-protection, or public-safety
configuration.

## Protected Invariants

- Valid Suite-2 negotiation vectors accept only the expected Suite-2 tuple.
- Unsupported suite/version/algorithm/parameter/flag vectors reject
  deterministically.
- Downgrade-like input rejects deterministically.
- Malformed negotiation input rejects deterministically.
- Rejected inputs do not mutate accepted negotiation state in the refimpl
  vector harness.
- Rejected qsc receive-path inputs do not mutate persisted Suite-2 session
  state and do not write plaintext output.
- Reject paths do not panic and do not emit backtraces.
- Reject paths do not leak plaintext, sentinel strings, route tokens, or
  secret-like material.
- No runtime protocol, crypto, service, website, workflow, Cargo, dependency,
  README, START_HERE, docs/public, branch-protection, or public-safety drift.

## Allowed Scope

- `inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0302_suite2_negotiation_vectors.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `tests/NA-0302_suite2_negotiation_vector_qsc_cross_surface_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsl/qsl-client/qsc/src/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc-desktop/**`
- `website/**`
- external website sources
- runtime protocol, crypto, handshake, key schedule, QSP wire-format, demo,
  service, or desktop implementation paths

## Dedicated Vector Requirements

- Include one valid Suite-2 vector.
- Include one unsupported suite vector.
- Include one downgrade/version vector.
- Include unsupported algorithm and unsupported flag/parameter vectors.
- Include one malformed negotiation vector.
- Keep the vector pack deterministic, bounded, and schema-valid under
  `QSHIELD-P4-VECTOR-SET-1`.

## Refimpl / Vector Requirements

- Load the NA-0302 vector pack from `inputs/suite2/vectors/**`.
- Verify schema identity and Suite-2 protocol tuple.
- Accept the valid vector and mutate accepted state only on success.
- Reject each negative vector with the expected reason code.
- Repeat each reject and prove deterministic output.
- Prove no accepted-state mutation on rejected vectors.
- Emit:
  - `NA0302_VECTOR_SCHEMA_OK`
  - `NA0302_VALID_SUITE2_VECTOR_OK`
  - `NA0302_UNSUPPORTED_SUITE_VECTOR_REJECT_OK`
  - `NA0302_DOWNGRADE_VECTOR_REJECT_OK`
  - `NA0302_UNSUPPORTED_PARAMETER_VECTOR_REJECT_OK`
  - `NA0302_MALFORMED_NEGOTIATION_VECTOR_REJECT_OK`
  - `NA0302_VECTOR_NO_MUTATION_ON_REJECT_OK`
  - `NA0302_NO_PANIC_OK`
  - `NA0302_NO_SECRET_LEAK_OK`
  - `NA0302_SUITE2_NEGOTIATION_VECTOR_HARDENING_OK`

## qsc Cross-Surface Requirements

- Use qsc test-only integration coverage.
- Use existing qsc CLI, relay, QSE envelope, and Suite-2 session storage
  behavior.
- Mutate queued Suite-2 wire payloads into unsupported suite,
  downgrade/version, unsupported flags, and malformed input cases.
- Assert fail-closed qsc receive markers.
- Assert no `recv_commit`, no output file, no panic/backtrace, no sentinel or
  route-token leakage, and unchanged persisted Suite-2 session state.
- Emit `NA0302_QSC_CROSS_SURFACE_OK`.

## Blocked-qsc Handling

If qsc cross-surface proof cannot run through existing public/test APIs, record
the exact missing test surface and emit `NA0302_QSC_CROSS_SURFACE_BLOCKED`
instead of weakening or hiding the gap. This lane implemented the qsc proof, so
the blocked marker is not expected.

## No-Mutation Requirements

- Refimpl: compare accepted negotiation state before and after every rejected
  vector.
- qsc: decrypt and compare `Suite2SessionState::snapshot_bytes()` before and
  after every rejected qsc receive attempt.

## No Panic / Leak Requirements

- Panic/backtrace wording must be absent from reject output.
- Plaintext and malformed sentinels must be absent from reject output.
- qsc route tokens must be absent from reject output.

## Required Local Checks

1. `python3 scripts/ci/validate_suite2_vectors.py`
2. `cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture`
3. `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture`
4. `cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture`
5. `cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture`
6. `cargo audit --deny warnings`
7. `cargo tree -i rustls-webpki --locked`
8. `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
9. `python3 formal/run_model_checks.py`
10. `scripts/ci/metadata_conformance_smoke.sh`
11. `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
12. `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
13. `scripts/ci/demo_cli_smoke.sh`
14. `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
15. `python3 scripts/ci/qsl_evidence_helper.py queue`
16. `python3 scripts/ci/qsl_evidence_helper.py decisions`
17. `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with the allowed paths above.
18. `python3 scripts/ci/qsl_evidence_helper.py link-check`
19. `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
20. Goal-lint PR body validation or accepted equivalent preflight.
21. Classifier proof for the changed path set.

## CI Expectations

- Required PR checks attach and complete successfully.
- `public-safety` remains required and green.
- Suite-2 vector validation must include the NA-0302 vector file.
- qsc test paths may trigger full-suite checks; any full-suite failure must be
  handled fail-closed unless it is conclusively transient and recoverable within
  the bounded retry policy.

## Successor Handoff

After NA-0302 merges and post-merge public-safety is green, close out NA-0302
and restore NA-0303 only as a separate closeout. NA-0303 should not be
implemented by the closeout and should keep protocol/crypto semantic changes
forbidden unless a future directive authorizes an exact fix lane.
