Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0330 Metadata Runtime qshield Demo Batching Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0330 authorizes only a future bounded qshield embedded
relay/demo batching implementation harness, defines exact future semantics and
proof requirements, selects one exact NA-0331 successor, and introduces no
runtime batching implementation or privacy/readiness overclaim.

## Protected Invariants

- NA-0330 is authorization/design only.
- No batching implementation is included in NA-0330.
- No cover traffic implementation is included in NA-0330.
- No production service timing behavior is included in NA-0330.
- No qshield runtime source changes are included in NA-0330.
- NA-0327 retry-cadence caps and fail-closed behavior remain preserved.
- NA-0329 bounded-jitter caps and fail-closed behavior remain preserved.
- No remote delete before local verification remains preserved.
- Valid ack deletes exactly once.
- Stale and duplicate ack behavior remains fail-closed or deterministic no-op.
- No accepted local state or plaintext output on invalid future batch member.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- No claim that timing metadata or traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim.

## Allowed Scope

- `docs/governance/evidence/NA-0330_metadata_runtime_qshield_demo_batching_authorization.md`
- `tests/NA-0330_metadata_runtime_qshield_demo_batching_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Batching implementation.
- Cover traffic implementation.
- Runtime timing mitigation implementation.
- Broad queue scheduling, send scheduling, receive scheduling, or transport
  padding implementation.
- qshield runtime source changes.
- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Cargo or dependency changes.
- Workflow, branch-protection, public-safety, website, README, START_HERE, or
  docs/public changes.
- NA-0331 implementation.

## Prior Bounded-Jitter Review Requirements

Before accepting the patch, confirm:

- live `NEXT_ACTIONS.md` shows READY NA-0330;
- NA-0329 is DONE;
- D-0640 exists exactly once;
- D-0641 exists exactly once;
- D-0642 was absent at startup;
- NA-0329 implemented only opt-in qshield embedded relay/demo bounded jitter;
- NA-0329 preserved retry-cadence bounds, ack-after-verify, no remote delete
  before local verification, and no output/state on invalid jitter;
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.

## Batching Semantic Design Requirements

The evidence must define:

- policy name `qshield_demo_batching_v1`;
- opt-in switch `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- logical test clock `QSHIELD_DEMO_BATCHING_NOW_MS`;
- maximum batch size 4;
- maximum send-side batch window 750 ms;
- receive candidate batch size at most 4;
- ack batch size at most 4 locally verified ack IDs;
- no long sleeps in deterministic test mode;
- no receive-side batch wait that extends NA-0327 retry cadence or NA-0329
  retry-plus-jitter caps in the first future harness;
- no batch wait after local verification before ack/delete;
- send batch all-or-nothing validation before mutation;
- receive batch ordered-prefix processing and stop at first invalid candidate;
- ack batch all-or-nothing validation before deletion;
- retention and purge rules for any local demo batch staging state;
- no-output/no-state boundaries for invalid batch members.

## Abuse / DoS / Latency Matrix Requirements

The evidence must include a matrix with:

- scenario;
- risk;
- proposed bound;
- future test;
- failure mode;
- stop condition;
- compatibility impact;
- claim boundary.

Required scenarios:

- one valid message;
- multiple valid messages;
- one invalid candidate in batch;
- mixed valid/invalid candidates;
- repeated invalid batch;
- empty batch/poll;
- stale ack in batch;
- duplicate ack in batch;
- slow valid receiver;
- local demo stress;
- production qsl-server equivalent as future-gated;
- qsl-attachments equivalent as future-gated.

## Future Implementation-Boundary Requirements

The evidence must list future allowed qshield files:

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- existing qshield test helper paths under `apps/qshield-cli/tests` only if
  already established.

The evidence must list future forbidden files:

- `qsl-server/**`
- `qsl-attachments/**`
- `qsc/**`
- `qsp/**`
- protocol, crypto, and key-schedule implementation paths
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `qsc-desktop/**`
- branch-protection and public-safety configuration.

## Marker-Plan Requirements

The evidence must require these future marker candidates:

- `NA0331_BATCHING_AUTHORIZATION_OK`
- `NA0331_BATCH_POLICY_OK`
- `NA0331_DETERMINISTIC_TEST_BATCHING_OK`
- `NA0331_VALID_SINGLE_MESSAGE_UNCHANGED_OK`
- `NA0331_VALID_BATCH_DELIVERY_OK`
- `NA0331_BATCH_ORDERING_PRESERVED_OK`
- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`
- `NA0331_PARTIAL_INVALID_BATCH_FAIL_CLOSED_OK`
- `NA0331_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0331_BOUNDED_JITTER_STILL_BOUNDED_OK`
- `NA0331_NO_ACCEPTED_STATE_ON_INVALID_BATCH_OK`
- `NA0331_NO_OUTPUT_ON_INVALID_BATCH_OK`
- `NA0331_NO_SECRET_BATCH_ARTIFACT_OK`
- `NA0331_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0331_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0331_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0331_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0331_NO_METADATA_FREE_CLAIM_OK`

## Production-Boundary Requirements

The evidence must state:

- qshield embedded relay/demo batching is local/demo only;
- qsl-server production batching requires a cross-repo lane;
- qsl-attachments batching or object timing requires a cross-repo lane;
- public internet timing remains future-gated;
- external review is recommended before stronger claims;
- website/public language must remain conservative;
- no claim of hidden timing metadata or hidden traffic shape.

## Claim-Boundary Requirements

The evidence, decision, traceability, journal, PR body, and response must not
claim:

- batching is implemented by NA-0330;
- timing metadata is hidden;
- traffic shape is hidden;
- prohibited wording: metadata-free behavior;
- prohibited wording: anonymity;
- prohibited wording: untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion.

Any mention of those phrase families must be negated, future-gated,
boundary-only, or clearly prohibited.

## Backup-Impact Requirements

- Tracked evidence must remain under the qsl-protocol worktree covered by
  `/srv/qbuild/work`.
- No durable evidence location outside the current backup scope may be created.
- The D132 preservation bundle must not be deleted.
- If a new durable evidence location outside current backup scope is required,
  stop and recommend a backup-plan update.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0329_metadata_runtime_bounded_jitter -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0327_metadata_runtime_retry_cadence_normalization -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh`
- `bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `bash scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `bash scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- `cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1`
- queue/decision checks
- exact allowed-path scope guard
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof for changed paths
- PR-body preflight or goal-lint-compatible proof

## CI Expectations

Required PR checks must pass normally before merge, including
`public-safety`. Cost-control skips are acceptable only when CI reports them as
skipped and public-safety remains green. No admin bypass, squash, rebase,
direct push, branch deletion command, branch-protection mutation, or
public-safety mutation is allowed.

## Successor Handoff

If NA-0330 merges and post-merge public-safety is green, closeout may restore
exactly one successor:

`NA-0331 -- Metadata Runtime qshield Demo Batching Implementation Harness`

The closeout must not implement NA-0331.
