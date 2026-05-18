Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0315 Metadata Runtime Identifier and Padding Executable Harness Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0315 defines an exact executable harness plan for metadata
runtime identifier/handle rotation and default padding, records the qshield poll
no-mutation blocker, and selects one exact NA-0316 successor without
implementing runtime metadata behavior.

## Protected Invariants

- NA-0315 remains harness-planning and non-runtime fixture/script work only.
- Runtime identifier/handle rotation remains unimplemented.
- Runtime default padding remains unimplemented.
- Existing NA-0291/NA-0293 fixture proof is not represented as runtime proof.
- Metadata runtime gaps remain visible.
- Exactly one READY queue item remains present during Packet J: `NA-0315`.
- If closeout runs, exactly one READY queue item is restored: `NA-0316`.

## Allowed Scope

- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`
- `tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md`
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime metadata implementation.
- Identifier/handle rotation runtime implementation.
- Default padding runtime implementation.
- qsp, protocol-core, crypto state-machine, key schedule, qsc/qsl runtime,
  qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE,
  docs/public, workflow, Cargo/dependency, branch-protection, public-safety
  configuration, branch deletion, and NA-0316 implementation.

## Prior Transition Review Requirements

- Inspect NA-0314 transition evidence and closeout testplan.
- Inspect NA-0291 identifier/padding fixture evidence and harness.
- Inspect NA-0293 sanitized-error/retention fixture evidence and harness.
- Confirm NA-0314 selected NA-0315 and did not implement runtime metadata
  behavior.

## Harness-Scope Decision Requirements

- Record whether NA-0315 is plan-only, non-runtime executable harness-plan
  fixture/script, runtime test-only, or blocked.
- If an executable plan fixture/script is added, prove it changes no runtime
  source and emits only plan-level markers.
- List exact allowed and forbidden files.

## Identifier/Handle Plan Requirements

- Identify peer, bundle, session, route-token, relay queue, message,
  attachment, and contact/device handle classes.
- Require opaque handle boundary proof.
- Require stale, malformed, replayed, and wrong-scope reject proof.
- Require no accepted-state mutation on reject.
- Require no raw handle, route token, bearer token, plaintext sentinel, or
  internal path in logs/artifacts.

## Default Padding Plan Requirements

- Define a default padding profile and bucket table.
- Require invalid bucket config reject.
- Require strip/verify for valid padded input.
- Require malformed padded input, bucket mismatch, too-large pad length, and
  over-limit payload reject.
- Require no accepted-state mutation on reject.
- Require no raw plaintext, sentinel, or unscoped exact sensitive length in
  logs/artifacts.

## qshield Poll / No-Mutation Risk Requirements

- Inspect qshield `/poll` runtime source.
- Record whether polling removes remote queued messages before local
  padding/decode reject.
- If yes, classify the risk as a stop risk for runtime no-mutation proof unless
  a future directive explicitly narrows the proof boundary.
- Select a blocker-resolution successor if the risk blocks the combined runtime
  harness.

## Marker Requirements

Future runtime markers must be recorded:

- `NA0315_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0315_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0315_STALE_HANDLE_REJECT_OK`
- `NA0315_IDENTIFIER_NO_MUTATION_ON_REJECT_OK`
- `NA0315_IDENTIFIER_NO_SECRET_LOG_OK`
- `NA0315_DEFAULT_PADDING_POLICY_OK`
- `NA0315_PADDING_BUCKETS_OK`
- `NA0315_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0315_PADDING_STRIP_VERIFY_OK`
- `NA0315_PADDING_NO_MUTATION_ON_REJECT_OK`
- `NA0315_PADDING_NO_SECRET_LOG_OK`
- `NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK`

Only `NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK` may be emitted as an NA-0315
proof marker. Runtime markers must remain future-gated.

## Successor-Selection Requirements

- Select exactly one NA-0316 successor.
- If qshield poll no-mutation risk blocks the combined harness, select:
  `NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker Resolution`.
- Do not implement NA-0316.

## Claim-Boundary Requirements

- No production readiness, public-internet readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, or complete-proof claim is introduced.
- High-risk wording is allowed only as negated, prohibited, future-gated, or
  explicit not-proven wording.
- Runtime metadata gaps remain visible.

## Backup-Impact Requirements

- List changed paths.
- Confirm durable artifacts remain under qsl-protocol paths in
  `/srv/qbuild/work`.
- Record whether a backup-plan update is required.

## Required Local Checks

- `bash -n scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `python3 -m json.tool inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- verify plan markers and absence of internal sentinel output
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/metadata_conformance_smoke.sh`
- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- NA-0310 refimpl oracle test
- qsc NA-0313, NA-0304, NA-0303, and NA-0302 harnesses if directly runnable
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- qshield-cli build/test if feasible
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- queue, decisions, scope-guard, link-check, leak-scan, classifier, goal-lint,
  and overclaim checks before PR creation

## CI Expectations

- Required checks must attach and complete successfully before merge.
- `public-safety` must remain required and green before merge and after merge.
- Docs/script/input governance cost control may skip unrelated full suites only
  if public-safety classifies the patch accordingly.

## Successor Handoff

If NA-0315 merges and closeout runs, restore:

`NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker Resolution`

The successor must resolve or explicitly scope the qshield poll/remove
no-mutation boundary before any combined runtime identifier/default-padding
harness claims receive-side no-mutation proof.
