Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0309 qsc Handshake Suite-ID Formal Model Testplan

## Objective

Add executable bounded formal/model properties for future qsc handshake
suite-id semantics and select the exact NA-0310 successor without implementing
qsc runtime, QHSM/QSP wire-format, production handshake, crypto, key schedule,
dependency, service, website, README, START_HERE, workflow, branch-protection,
or public-safety changes.

## Protected invariants

- NA-0309 remains the sole READY item until separate closeout.
- D-0597 exists exactly once after the patch and D-0598 is absent.
- Existing SCKA and Suite-2 negotiation formal checks remain active.
- The qsc suite-id model is bounded and does not claim runtime implementation.
- Missing explicit qsc suite-id admission implementation remains visible.
- Persisted Suite-2 state is not presented as explicit admission evidence.

## Allowed scope

- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `formal/README.md`
- `docs/governance/evidence/NA-0309_qsc_handshake_suite_id_formal_model_properties.md`
- `tests/NA-0309_qsc_handshake_suite_id_formal_model_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc/qsl runtime implementation paths.
- QHSM/QSP wire-format implementation paths.
- production handshake, crypto state-machine, and key schedule paths.
- vector fixtures, refimpl oracle implementation, Cargo files, workflows,
  scripts, services, apps, qsc-desktop, website, README, START_HERE,
  docs/public, branch-protection, and public-safety configuration.

## Model property requirements

The model must cover:

- valid v2 Suite-2 parameter block accept;
- explicit legacy v1 compatibility accept;
- legacy v1 required-mode reject;
- unsupported, downgraded, stripped, mismatched, duplicate, unknown critical,
  noncanonical, malformed, and inconsistent tuple rejects;
- transcript binding reject;
- key-context binding reject;
- no mutation, no output, no recv_commit, no secret leak on reject;
- accepted valid Suite-2 preservation;
- no downgrade path from suite-required mode to compatibility; and
- deterministic reason labels for every reject.

## Marker requirements

The direct model and `formal/run_model_checks.py` must emit:

- `NA0309_MODEL_VALID_V2_SUITE2_OK`
- `NA0309_MODEL_LEGACY_COMPATIBILITY_OK`
- `NA0309_MODEL_LEGACY_REQUIRED_REJECT_OK`
- `NA0309_MODEL_UNSUPPORTED_SUITE_REJECT_OK`
- `NA0309_MODEL_DOWNGRADE_REJECT_OK`
- `NA0309_MODEL_STRIPPED_SUITE_REJECT_OK`
- `NA0309_MODEL_MISMATCH_REJECT_OK`
- `NA0309_MODEL_DUPLICATE_REJECT_OK`
- `NA0309_MODEL_UNKNOWN_CRITICAL_REJECT_OK`
- `NA0309_MODEL_NONCANONICAL_REJECT_OK`
- `NA0309_MODEL_MALFORMED_REJECT_OK`
- `NA0309_MODEL_TRANSCRIPT_BINDING_OK`
- `NA0309_MODEL_KEY_CONTEXT_OK`
- `NA0309_MODEL_NO_MUTATION_ON_REJECT_OK`
- `NA0309_MODEL_NO_OUTPUT_ON_REJECT_OK`
- `NA0309_MODEL_NO_SECRET_LEAK_OK`
- `NA0309_MODEL_NO_DOWNGRADE_PATH_OK`
- `NA0309_MODEL_REASON_LABELS_OK`
- `NA0309_QSC_HANDSHAKE_SUITE_ID_FORMAL_MODEL_OK`

## Existing model preservation requirements

`formal/run_model_checks.py` must continue to run:

- SCKA bounded model checks; and
- Suite-2 negotiation downgrade/no-mutation model checks.

## No-mutation/no-output/no-leak requirements

For every modeled reject:

- accepted-state snapshot remains unchanged;
- `output_emitted` is false;
- `recv_commit` is false;
- `secret_leak` is false; and
- reason label is deterministic and does not include sentinel material.

## Coverage matrix requirements

Evidence must map each model property and NA-0308 vector/refimpl/qsc harness
category to one of:

- PROVEN_MODEL
- READY_FOR_VECTOR
- READY_FOR_REFIMPL
- READY_FOR_QSC_HARNESS
- BLOCKED
- FUTURE_GATE

## Successor-selection requirements

If the executable model succeeds, select:

NA-0310 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle

If the model is blocked, select:

NA-0310 -- qsc Handshake Suite-ID Model Blocker Resolution

NA-0310 must not be implemented in this lane.

## Claim-boundary requirements

Evidence and PR text must not claim production readiness, public internet
readiness, external review completion, anonymity, metadata-free behavior,
untraceability, cryptographic completion, runtime qsc suite-id implementation,
or complete protocol proof.

## Backup-impact requirements

Record whether the patch changes evidence locations, response paths, source
roots, excluded backup paths, or creates non-rebuildable artifacts outside the
current backup scope. Expected result: no backup-plan update if changes stay
under the qsl-protocol worktree.

## Required local checks

- `git diff --check`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- runnable NA-0304/NA-0303/NA-0302/NA-0301/NA-0300 focused harnesses where
  feasible.
- queue, decisions, scope-guard, link-check, leak-scan, classifier, overclaim
  scan, and goal-lint proof.

## CI expectations

Required checks must attach and complete green before merge. `public-safety`
must remain required and complete green before merge and after merge.

## Successor handoff

NA-0310 should turn the model properties into vector schema and refimpl oracle
requirements. It should not implement qsc runtime or QHSM/QSP production wire
changes unless a future directive explicitly authorizes that scope.
