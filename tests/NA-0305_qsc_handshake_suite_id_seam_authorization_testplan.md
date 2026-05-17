Status: Supporting
Owner: qsl-protocol maintainers
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0305 qsc Handshake Suite-ID Seam Authorization Testplan

## Objective

Produce evidence-bound authorization planning for the smallest safe qsc
handshake suite-id seam or successor proof lane, without implementing the seam
in NA-0305.

## Protected invariants

- No qsc suite-id seam implementation in NA-0305.
- No silent protocol semantic change.
- No silent crypto state-machine change.
- No production handshake implementation change.
- No QSP wire-format or key schedule change.
- No dependency, workflow, service, website, docs/public, README, START_HERE,
  branch-protection, or public-safety configuration drift.
- The NA-0304 missing-suite-id-field blocker remains visible.
- Inferred Suite-2 state is not presented as explicit qsc handshake suite-id
  admission evidence.
- No unsupported public-internet, external-review, anonymity, metadata-free, or
  untraceable claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md`
- `tests/NA-0305_qsc_handshake_suite_id_seam_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime source under `qsl/qsl-client/qsc/src/**`
- qsc integration tests under `qsl/qsl-client/qsc/tests/**`
- QSP/refimpl/protocol-core implementation source
- crypto state-machine implementation
- production handshake implementation
- key schedule implementation
- QSP wire-format implementation
- `Cargo.toml` or `Cargo.lock`
- `.github/**`
- `scripts/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsc-desktop/**`
- website or external website paths
- README, START_HERE, docs/public
- branch-protection or public-safety configuration

## Prior blocker review requirements

The evidence must review NA-0304 and preserve these findings:

- valid qsc handshake activation persists Suite-2 state with protocol version
  `0x0500` and suite id `0x0002`;
- qsc `QHSM` A1/B1/A2 frames have no explicit suite-id field;
- unsupported, downgrade-like, and malformed qsc handshake suite-id admission
  inputs remain blocked by the missing field; and
- the missing seam must remain a visible gap.

## Option-analysis requirements

The evidence must classify:

- test-only frame builder/parser helper;
- mutated existing `QHSM` frame fixture;
- production `QHSM` frame schema addition;
- existing qsc public/test API seam;
- refimpl/vector-only evidence; and
- formal/model-only evidence.

Each option must state feasibility, likely changed files, production behavior
impact, wire-format impact, whether it can be test-only, whether it proves
admission or only post-admission state, overclaim risk, and recommended
successor shape.

## Future-lane selection requirements

The selected successor must be exact. If no existing test-only qsc handshake
suite-id admission seam exists, the successor must be a design/authorization or
blocker lane rather than an implementation lane.

Selected expected successor:

NA-0306 -- qsc Handshake Suite-ID Wire-Format Change Authorization Plan

## Claim-boundary requirements

The evidence must explicitly reject:

- treating generic frame-length rejects as suite-id-specific admission behavior;
- treating persisted Suite-2 state as explicit qsc handshake suite-id admission
  evidence;
- hiding the missing suite-id seam;
- smuggling a production wire/schema change into NA-0305; and
- implying production readiness, public-internet readiness, external-review
  completion, anonymity, metadata-free behavior, or untraceability.

## Required local checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- `cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `git diff --check`
- direct overclaim phrase scan over changed lines
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md --allowed tests/NA-0305_qsc_handshake_suite_id_seam_authorization_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint using a PR body containing `Goals: G1, G2, G3, G4, G5`
- `bash scripts/ci/classify_ci_scope.sh <changed_paths>`

## CI expectations

Required CI and public-safety must complete normally. Because NA-0305 is
governance/testplan-only, full-suite cost-control skips are acceptable only
when the classifier and public-safety jobs report intentional skip behavior.

## Successor handoff

After the NA-0305 PR merges and post-merge public-safety is green, a separate
closeout may mark NA-0305 DONE and restore exactly one READY successor:

NA-0306 -- qsc Handshake Suite-ID Wire-Format Change Authorization Plan

The closeout must not implement NA-0306.
