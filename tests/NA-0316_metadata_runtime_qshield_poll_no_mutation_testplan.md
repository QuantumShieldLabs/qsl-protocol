Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0316 Metadata Runtime qshield Poll No-Mutation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0316 truthfully classifies the qshield poll no-mutation
blocker inherited from NA-0315 and selects the exact next metadata-runtime
successor without implementing unauthorized runtime behavior.

## Protected Invariants

- Do not hide qshield remote queue mutation.
- Do not present local-only no-mutation as remote no-mutation.
- Do not claim runtime metadata reduction.
- Do not claim anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.
- Do not change protocol, crypto, qsp, qsc, key schedule, service,
  dependency, workflow, public-safety, branch-protection, website, README, or
  START_HERE paths.
- Preserve exactly one READY item during the evidence PR: NA-0316.

## Allowed Scope

- `docs/governance/evidence/NA-0316_metadata_runtime_qshield_poll_no_mutation_blocker_resolution.md`
- `tests/NA-0316_metadata_runtime_qshield_poll_no_mutation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qshield runtime implementation files unless a future directive names exact
  authorized files.
- qsc, qsp, protocol-core, crypto state-machine, key schedule, service,
  website, qsc-desktop, qsl-server, qsl-attachments, workflow, Cargo,
  dependency, README, START_HERE, branch-protection, or public-safety changes.
- NA-0317 implementation.

## Prior Blocker Review Requirements

Confirm that NA-0315 recorded the qshield poll blocker and selected NA-0316
because current `/poll` removes queued messages before local receive-side
padding or decode reject can prove remote no-mutation.

Required evidence:

- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`
- `tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md`
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`
- D-0609 and D-0610 in `DECISIONS.md`

## Poll Behavior Baseline Requirements

Inspect qshield source and record:

- `/send` enqueue behavior;
- `/poll` dequeue behavior;
- receive-side padding validation order;
- receive-side actor decode order;
- attachment receive poll/decode order;
- existing qshield tests and scripts.

The baseline must classify whether current behavior is:

- `PROVEN_REMOTE_MUTATION`;
- `PROVEN_LOCAL_ONLY_NO_MUTATION`;
- `PROVEN_QUEUE_PRESERVING`;
- `INSUFFICIENT_SEAM`;
- `NEEDS_RUNTIME_CHANGE`.

## Semantic Option Requirements

Analyze at least:

- ack/commit-after-local-verify;
- peek-before-delete;
- local-only no-mutation;
- dead-letter/quarantine;
- blocker continuation.

Each option must record feasibility, likely files, scope risk, tests, future
markers, claim boundary, and recommendation.

## Queue Mutation Boundary Requirements

If `/poll` removes queued messages before local verify, evidence must state
that remote queue no-mutation is not proven and must not emit or claim a
remote no-mutation OK marker.

If future behavior changes to ack/commit or peek, tests must prove:

- valid local verify can commit/delete when authorized;
- invalid local verify does not delete the remote queued message, or only
  mutates an explicitly selected safe boundary;
- no partial accepted state remains;
- no secret/sentinel leaks.

## Local Mutation Boundary Requirements

If a local-only successor is chosen, it must prove:

- malformed padding or decode reject creates no accepted local state;
- no plaintext or accepted output is produced on reject;
- any remote queue deletion is explicitly outside the claim boundary.

NA-0316 does not choose local-only as the immediate successor.

## Output / Leak Requirements

Evidence and future tests must preserve:

- no plaintext sentinel on reject;
- no padding sentinel on reject;
- no route token or bearer token in logs or artifacts;
- no panic/backtrace output;
- no long-hex or sensitive endpoint evidence prose.

## Marker / Blocker Requirements

Record marker status without overclaiming:

- `NA0316_QSHIELD_POLL_BEHAVIOR_CLASSIFIED_OK`
- `NA0316_QSHIELD_REMOTE_QUEUE_MUTATION_BOUNDARY_OK`
- `NA0316_QSHIELD_LOCAL_NO_MUTATION_OK`
- `NA0316_QSHIELD_NO_OUTPUT_ON_REJECT_OK`
- `NA0316_QSHIELD_NO_SECRET_LEAK_OK`
- `NA0316_QSHIELD_ACK_COMMIT_REQUIRED_OK`
- `NA0316_QSHIELD_ACK_COMMIT_NOT_AVAILABLE`
- `NA0316_METADATA_RUNTIME_POLL_BLOCKER_RESOLUTION_OK`

Current NA-0316 may record classification evidence, but it must not represent
future executable runtime markers as emitted proof.

## Successor-Selection Requirements

Select exactly one successor. If current qshield poll removes messages before
local verify and runtime changes are not authorized, select:

`NA-0317 -- Metadata Runtime qshield Ack/Commit Poll Semantics Authorization`

Do not implement NA-0317.

## Claim-Boundary Requirements

The evidence must state that NA-0316 does not prove:

- runtime metadata reduction;
- identifier/handle rotation;
- default padding as a runtime default;
- remote no-mutation on invalid local receive;
- anonymity;
- metadata-free behavior;
- untraceability;
- public-internet readiness;
- production readiness;
- external review completion.

## Backup-Impact Requirements

If changes stay under qsl-protocol governance/testplan/journal paths in
`/srv/qbuild/work`, record that no backup-plan update is required.

If durable evidence is created outside the expected worktree or response-file
scope, stop and recommend a backup-plan update.

## Required Local Checks

Run or record why not run:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted NA-0310 refimpl oracle test

## CI Expectations

The PR must keep required checks green, including `public-safety`. If CI is
docs-only cost-controlled, record the classifier result and public-safety
result. Do not merge if required checks are red or missing.

## Successor Handoff

The closeout lane may restore:

`NA-0317 -- Metadata Runtime qshield Ack/Commit Poll Semantics Authorization`

That successor should authorize exact qshield runtime files, API shape, tests,
markers, and stop conditions before implementation begins. It must preserve the
NA-0316 mutation-boundary classification and all metadata/public-claim
boundaries.
