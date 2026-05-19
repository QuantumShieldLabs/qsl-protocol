Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0317 Metadata Runtime qshield Ack/Commit Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0317 authorizes an exact qshield ack/commit or equivalent
queue-preserving poll semantic without implementing runtime behavior, and that
it selects one exact NA-0318 successor.

## Protected Invariants

- Do not hide current qshield remote queue mutation.
- Do not present local-only no-mutation as remote no-mutation.
- Do not claim runtime metadata reduction.
- Do not claim anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.
- Do not change qshield runtime files in NA-0317.
- Do not change protocol, crypto, qsc, qsp, key schedule, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, workflow, Cargo,
  dependency, branch-protection, or public-safety paths.
- Preserve exactly one READY item during the authorization PR: NA-0317.

## Allowed Scope

- `docs/governance/evidence/NA-0317_metadata_runtime_qshield_ack_commit_authorization.md`
- `tests/NA-0317_metadata_runtime_qshield_ack_commit_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qshield runtime implementation files in NA-0317.
- NA-0318 implementation.
- qsc, qsp, protocol-core, crypto state-machine, key schedule, service,
  website, qsc-desktop, qsl-server, qsl-attachments, workflow, Cargo,
  dependency, README, START_HERE, branch-protection, or public-safety changes.

## Prior Mutation-Boundary Review Requirements

Evidence must recheck and preserve the NA-0316 finding:

- current qshield `/poll` removes queued messages before local receive-side
  padding or decode reject;
- current boundary remains `PROVEN_REMOTE_MUTATION` and
  `NEEDS_RUNTIME_CHANGE`;
- remote no-mutation must not be claimed from current behavior.

Required sources:

- `docs/governance/evidence/NA-0316_metadata_runtime_qshield_poll_no_mutation_blocker_resolution.md`
- `tests/NA-0316_metadata_runtime_qshield_poll_no_mutation_testplan.md`
- D-0611 and D-0612 in `DECISIONS.md`

## Ack/Commit Semantic Design Requirements

The evidence must define:

- queued, candidate/polled, locally verified, accepted, committed, rejected,
  retried, and quarantined states;
- valid receive ordering with local verification and accepted output/state
  before remote commit;
- invalid receive ordering with no accepted state, no output, no secret leak,
  and no remote delete;
- idempotency for repeated poll, repeated commit, and stale commit;
- bounded handling for repeated invalid messages;
- coarse error visibility;
- compatibility treatment for legacy destructive `/poll`.

## Option-Analysis Requirements

The evidence must analyze:

- Option A: ack/commit-after-local-verify;
- Option B: peek-before-delete;
- Option C: quarantine/dead-letter;
- Option D: local-only no-mutation;
- Option E: blocker continuation.

Each option must record feasibility, future files, tests, markers,
compatibility risk, abuse/retry risk, claim boundary, whether it permits a
future combined identifier/default-padding harness, and recommendation.

## Future Implementation Boundary Requirements

If ack/commit or peek is selected, the evidence must list:

- exact future implementation files;
- exact future forbidden files;
- future tests;
- future markers;
- future stop conditions.

Any qsl-server dependency must be surfaced as a stop condition and successor
change, not hidden in qsl-protocol-only implementation scope.

## Cross-Repo Dependency Assessment Requirements

The evidence must state whether qsl-server changes are required.

Required outcome for qshield-only NA-0318:

- qshield embedded relay implementation can be scoped inside qsl-protocol;
- no qsl-server changes are required for the selected qshield harness;
- any later qsc/qsl-server production relay semantics require a separate
  authorization lane.

## Marker Requirements

Record these future markers without claiming they are emitted by NA-0317:

- `NA0318_QSHIELD_PEEK_OR_POLL_CANDIDATE_OK`
- `NA0318_QSHIELD_LOCAL_VERIFY_BEFORE_COMMIT_OK`
- `NA0318_QSHIELD_ACK_COMMIT_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_REMOTE_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_ACCEPTED_STATE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_OUTPUT_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_SECRET_LEAK_OK`
- `NA0318_QSHIELD_STALE_ACK_FAIL_CLOSED_OK`
- `NA0318_QSHIELD_REPEATED_INVALID_RECV_BOUNDED_OK`
- `NA0318_METADATA_RUNTIME_ACK_COMMIT_OK`

## Successor-Selection Requirements

The evidence must select exactly one NA-0318 successor.

Expected selected successor if no qsl-server dependency is required:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

Do not implement NA-0318 in NA-0317.

## Claim-Boundary Requirements

The evidence must state that NA-0317 does not prove:

- runtime metadata reduction;
- identifier/handle rotation;
- default padding as a runtime default;
- remote queue no-mutation from current destructive `/poll`;
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
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1` if directly runnable
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- full refimpl tests if feasible
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`

## CI Expectations

The PR must keep required checks green, including `public-safety`. If CI is
docs-only cost-controlled, record the classifier result and public-safety
result. Do not merge if required checks are red or missing.

## Successor Handoff

The closeout lane may restore:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

That successor must implement only the qshield ack/commit harness selected by
NA-0317 or stop with exact prerequisite evidence. It must not implement the
broader metadata runtime identifier/default-padding harness.
