# NA-0329 Metadata Runtime qshield Demo Bounded Jitter Harness Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

## Objective

Validate that NA-0329 implements bounded, opt-in qshield embedded relay/demo
jitter only within the authorized qshield demo receive surface and proves the
policy with deterministic executable tests.

## Protected Invariants

- qshield embedded relay/demo-only boundary.
- Opt-in behavior only; no default production behavior change.
- Deterministic test mode and bounded CI runtime.
- NA-0327 retry-cadence caps and fail-closed behavior remain intact.
- NA-0318 ack-after-verify and no-delete-before-verify behavior remain intact.
- No accepted local state or plaintext output on invalid jitter.
- No secret jitter artifacts or logs.
- qsl-server and qsl-attachments production timing remains unproven and
  cross-repo-gated.
- No claim that timing metadata or traffic shape is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim.

## Allowed Scope

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/tests/na_0329_metadata_runtime_bounded_jitter.rs`
- `docs/governance/evidence/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness.md`
- `tests/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Cargo or dependency changes.
- Workflow, branch-protection, public-safety, website, README, START_HERE, or
  docs/public changes.
- Batching implementation.
- Cover traffic implementation.
- Transport padding changes.
- Broad queue scheduling.
- Production-service timing behavior.

## Prior Authorization Review Requirements

Before accepting the patch, confirm:

- live `NEXT_ACTIONS.md` shows READY NA-0329;
- NA-0328 is DONE;
- D-0638 exists exactly once;
- D-0639 exists exactly once;
- D-0640 was absent at startup;
- NA-0328 authorized future opt-in `qshield_demo_bounded_jitter_v1`;
- NA-0327 retry-cadence proof is preserved.

## Bounded Jitter Implementation Requirements

- Policy name is `qshield_demo_bounded_jitter_v1`.
- Opt-in switch is `QSHIELD_DEMO_BOUNDED_JITTER=1`.
- Deterministic test mode is `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`.
- Added jitter is 0-250 ms.
- Retry plus jitter is capped at 2250 ms for invalid and empty-poll classes.
- Jitter selection must not derive from route tokens, raw ack IDs, plaintext,
  candidate bodies, passphrases, or key material.
- No added jitter may occur between successful local verification and remote
  ack/delete in this implementation.

## Harness Marker Requirements

The harness must declare and truthfully emit:

- `NA0329_BOUNDED_JITTER_AUTHORIZATION_OK`
- `NA0329_JITTER_POLICY_OK`
- `NA0329_DETERMINISTIC_TEST_JITTER_OK`
- `NA0329_INVALID_RETRY_JITTER_BOUNDED_OK`
- `NA0329_EMPTY_POLL_JITTER_BOUNDED_OK`
- `NA0329_ACK_RETRY_JITTER_FAIL_CLOSED_OK`
- `NA0329_VALID_DELIVERY_UNCHANGED_OK`
- `NA0329_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0329_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0329_NO_ACCEPTED_STATE_ON_INVALID_JITTER_OK`
- `NA0329_NO_OUTPUT_ON_INVALID_JITTER_OK`
- `NA0329_NO_SECRET_JITTER_ARTIFACT_OK`
- `NA0329_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0329_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0329_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0329_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0329_NO_METADATA_FREE_CLAIM_OK`
- `NA0329_METADATA_RUNTIME_BOUNDED_JITTER_OK`

## Abuse/DoS/Latency Requirements

- Invalid candidate retry remains capped at four attempts per local window.
- Empty poll remains bounded and successful.
- Composed delay remains at or below 2250 ms.
- Test mode avoids long sleeps and nondeterministic CI behavior.
- Ledger read/parse/write failures remain fail-closed when policy is enabled.

## Valid Path Requirements

- Valid ack deletes exactly one candidate.
- Remaining candidate stays queued.
- Repeated candidate polling before ack does not delete the candidate.
- No post-verify ack/delete jitter is introduced.

## Invalid Retry Jitter Requirements

- Invalid candidate retry jitter is recorded only for retry attempts after the
  first invalid candidate attempt.
- Jitter is 0-250 ms.
- Retry delays remain 0, 500, 1000, and 2000 ms.
- Fifth invalid attempt fails closed at the retry cap.
- Candidate remains remotely queued before local verification succeeds.

## Ack Retry Requirements

- Duplicate ack returns fail-closed response after the candidate was already
  deleted.
- Stale ack returns fail-closed response.
- Neither duplicate nor stale ack deletes the remaining candidate.

## No-Remote-Delete Requirements

- Candidate polling returns the same ack handle before local verification.
- Invalid receive attempts do not submit ack/delete.
- Cap failure does not submit ack/delete.

## No-Local-Output/State Requirements

- `state.json` remains byte-identical after invalid receive attempts.
- Combined stdout/stderr contains no plaintext output from invalid receives.
- Invalid receive output contains no plaintext sentinel.

## No-Secret-Artifact Requirements

Scan command output and ledger text for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel;
- actual route token;
- actual ack ID;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace text;
- sensitive local absolute path prefixes.

Required result:

- `JITTER_ARTIFACT_SECRET_FINDING_COUNT 0`

## Production-Boundary Requirements

- qshield embedded relay/demo proof remains local/demo only.
- qsl-server production timing remains unproven and cross-repo-gated.
- qsl-attachments production timing and object-size timing behavior remain
  unproven and cross-repo-gated.
- No qsl-server or qsl-attachments files are changed.

## Claim-Boundary Requirements

The evidence and PR body must not claim:

- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim of metadata-free behavior;
- no claim of anonymity;
- no claim of untraceable behavior;
- no claim of production readiness;
- no claim of public-internet readiness;
- no claim of external-review completion.

Any mention of those phrases must be negated, future-gated, or boundary-only.

## Backup-Impact Requirements

- Tracked evidence must remain under the qsl-protocol worktree covered by
  `/srv/qbuild/work`.
- Runtime artifacts must remain temporary and non-durable.
- If a new durable evidence location outside the current backup scope is
  required, stop and recommend a backup-plan update.

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
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- metadata runtime and phase-2 harnesses
- qsc send_commit and model checks
- scope guard, link-check, leak-scan, goal-lint, and classifier proof

## CI Expectations

- Required checks attach to the PR head.
- Required checks complete green before merge.
- `public-safety` remains required and green before and after merge.
- No admin bypass, squash, rebase, direct push, or branch deletion command is
  used.

## Successor Handoff

If NA-0329 succeeds, select:

`NA-0330 -- Metadata Runtime qshield Demo Batching Authorization Plan`

Do not implement NA-0330 in the NA-0329 implementation PR.
