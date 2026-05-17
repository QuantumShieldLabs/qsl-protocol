# NA-0301 Suite-2 Negotiation Downgrade Harness Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-16

Goals: G1, G2, G3, G4, G5

## Objective

Add bounded executable Suite-2 negotiation, downgrade-like, unsupported input,
no-mutation, no-panic, no-leak, and vector-consistency proof without changing
protocol or crypto implementation semantics.

## Protected Invariants

- Suite-2 valid control path remains accepted in the tested refimpl surface.
- Unsupported suite, unsupported/downgrade-like version, unsupported
  parameter/flag, unsupported algorithm fixture, and malformed inputs fail
  closed.
- Rejected inputs do not mutate accepted negotiation or receive state.
- Reject attempts do not panic and do not expose panic/backtrace wording.
- Reject text does not echo plaintext or negotiation sentinels.
- Existing Suite-2 downgrade and transcript vectors remain consistent with the
  refimpl/vector evaluator behavior.
- No protocol, wire, key-schedule, handshake, downgrade, replay, or crypto-state
  implementation semantics are changed.
- No Cargo/dependency, workflow, public-safety, branch-protection, service,
  desktop, website, README, START_HERE, docs/public, or runtime implementation
  path is changed.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, guaranteed-secure, or complete-proof claim is
  introduced.

## Allowed Scope

- `tools/refimpl/quantumshield_refimpl/tests/na_0301_suite2_negotiation_downgrade.rs`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `tests/NA-0301_suite2_negotiation_downgrade_harness_testplan.md`
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
- `tools/refimpl/quantumshield_refimpl/src/**`
- `qsp/**`
- `qsc/**`
- `qsl/**` implementation source
- `qsl-client/**` implementation source
- `apps/**` implementation source
- `inputs/**` unless future fixture scope explicitly requires it
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- external website sources
- runtime, protocol, crypto, demo, service, or desktop implementation paths

## Suite-2 Control Requirements

- A valid local Suite-2 negotiation fixture accepts.
- A valid Suite-2 wire send/receive path accepts.
- Accepted negotiation and receive state must change on valid control input.
- Required marker: `NA0301_SUITE2_CONTROL_OK`.

## Unsupported Suite Requirements

- Unsupported suite fixture rejects deterministically.
- Unsupported suite wire mutation rejects deterministically.
- Accepted state remains unchanged.
- Required marker: `NA0301_UNSUPPORTED_SUITE_REJECT_OK`.

## Downgrade / Version Requirements

- Suite-1-like fallback fixture rejects deterministically.
- Unsupported version fixture rejects deterministically.
- Downgrade-like wire mutation rejects deterministically.
- Accepted state remains unchanged.
- Required marker: `NA0301_DOWNGRADE_REJECT_OK`.

## Unsupported Parameter / Flag Requirements

- Unsupported flag mutation rejects deterministically.
- Unsupported algorithm fixture rejects deterministically.
- Accepted state remains unchanged.
- Required marker: `NA0301_UNSUPPORTED_PARAMETER_REJECT_OK`.

## Malformed Input Requirements

- Malformed sentinel input rejects deterministically.
- Sentinel is not echoed in reject text.
- Accepted state remains unchanged.
- Required marker: `NA0301_MALFORMED_NEGOTIATION_REJECT_OK`.

## No-Mutation Requirements

- Snapshot accepted negotiation state after valid control input.
- Snapshot accepted `Suite2SessionState` after valid receive.
- Assert all reject cases leave both snapshots unchanged.
- Required marker: `NA0301_NO_MUTATION_ON_REJECT_OK`.

## No Panic / Leak Requirements

- Wrap adversarial wire receives with `catch_unwind`.
- Fail if panic/backtrace wording appears in reject text.
- Fail if plaintext or negotiation sentinels appear in reject text.
- Required markers:
  - `NA0301_NO_PANIC_OK`
  - `NA0301_NO_SECRET_LEAK_OK`

## Refimpl / Vector Consistency Requirements

- Evaluate all existing `CAT-S2-DOWNGRADE-001` vectors.
- Evaluate all existing `CAT-S2-TRANSCRIPT-001` vectors.
- Fail on count drift or reason/output mismatch.
- Emit `NA0301_VECTOR_CONSISTENCY_OK`.

## Required Local Checks

```bash
cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed tools/refimpl/quantumshield_refimpl/tests/na_0301_suite2_negotiation_downgrade.rs --allowed docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md --allowed tests/NA-0301_suite2_negotiation_downgrade_harness_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI Expectations

- Required PR checks attach and complete successfully.
- `public-safety` remains required and green.
- Because a refimpl test path changes, runtime/test suites are expected to run
  rather than docs-only full-suite skips.

## Successor Handoff

NA-0301 remains READY until a separate closeout directive marks it DONE and
restores exactly one successor, expected to be NA-0302 unless live queue text
selects a different successor.
