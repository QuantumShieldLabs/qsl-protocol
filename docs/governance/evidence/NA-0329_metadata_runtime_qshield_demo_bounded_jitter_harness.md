# NA-0329 Metadata Runtime qshield Demo Bounded Jitter Harness

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

## Executive Summary

NA-0329 implements the bounded qshield embedded relay/demo jitter harness
authorized by NA-0328. The implementation is opt-in, local-demo scoped, and
deterministic under test mode. It extends the existing NA-0327 qshield receive
retry ledger with bounded jitter evidence for empty polls and invalid candidate
retries.

The harness proves deterministic jitter bounds, invalid retry bounds, empty
poll bounds, stale/duplicate ack fail-closed behavior, valid ack/delete
compatibility, retry-cadence cap preservation, no remote delete before local
verification, no accepted local state or plaintext output on invalid jitter, no
secret jitter artifacts, and the qshield demo versus production-service
boundary.

This evidence does not claim that timing metadata or traffic shape is hidden.
This evidence does not claim anonymity, metadata-free behavior, untraceable
behavior, or production readiness. This evidence does not claim public-internet
readiness. This evidence does not claim external-review completion.

## Live NA-0329 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: implement the bounded qshield embedded relay/demo jitter harness
  authorized by D-0638 and NA-0328 evidence, or stop on an exact prerequisite.

Allowed implementation scope used by this lane:

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/tests/na_0329_metadata_runtime_bounded_jitter.rs`
- this evidence file
- `tests/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope preserved:

- no qsl-server changes;
- no qsl-attachments changes;
- no qsc/qsp/protocol/crypto/key-schedule changes;
- no dependency, Cargo, workflow, branch-protection, public-safety, website,
  README, START_HERE, or docs/public changes;
- no batching, cover traffic, broad queue scheduling, transport padding, or
  production-service timing implementation.

## Inherited NA-0328 Authorization

NA-0328 D-0638 authorized only future opt-in qshield embedded relay/demo bounded
jitter with these relevant constraints:

- policy name: `qshield_demo_bounded_jitter_v1`;
- opt-in switch: `QSHIELD_DEMO_BOUNDED_JITTER=1`;
- deterministic test mode: `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`;
- first implementation maximum added jitter: 250 ms per eligible event;
- retry-cadence composition cap: 2250 ms for invalid and empty-poll classes;
- no added jitter between successful local verification and remote ack/delete
  in the first implementation harness;
- no increase to the NA-0327 four-attempt invalid-candidate cap;
- no accepted local state and no plaintext output on invalid retry;
- no secret-bearing jitter artifacts.

## Inherited NA-0327 Retry-Cadence Proof

NA-0327 added the opt-in `qshield_demo_retry_cadence_v1` receive retry ledger:

- invalid candidate attempts are capped at four per 60 second local window;
- retry classes remain 0 ms, 500 ms, 1000 ms, and 2000 ms;
- empty poll remains bounded at the 2000 ms retry class;
- valid ack deletes exactly one candidate;
- duplicate and stale ack attempts fail closed or no-op deterministically;
- invalid retries do not delete remote candidates before local verification;
- invalid retries produce no accepted local state and no plaintext output;
- retry artifacts avoid route tokens, raw handles, raw ack IDs, plaintext,
  padding sentinels, passphrases, raw key material, panic text, and backtraces.

NA-0329 composes bounded jitter with this ledger rather than replacing it.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `tests/NA-0328_closeout_restore_na0329_testplan.md`
- `docs/governance/evidence/NA-0328_metadata_runtime_qshield_demo_bounded_jitter_authorization.md`
- `tests/NA-0328_metadata_runtime_qshield_demo_bounded_jitter_authorization_testplan.md`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- qshield NA-0318, NA-0319, NA-0320, NA-0322, NA-0324, and NA-0327 harnesses

Search coverage included bounded jitter, jitter, deterministic test mode,
invalid retry, empty poll, ack retry, valid delivery, remote delete, accepted
state, output, artifact, secret, qshield, demo, qsl-server, qsl-attachments,
no production-readiness claim, no timing-hidden claim, no traffic-hidden claim,
no metadata-free claim, no untraceable claim, `FUTURE_GATE`, and `NOT_READY`.

## Implementation Summary

Implemented in `apps/qshield-cli/src/commands/recv.rs`:

- added `qshield_demo_bounded_jitter_v1` policy constants;
- added `QSHIELD_DEMO_BOUNDED_JITTER` and
  `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE` gates;
- extended the demo retry ledger with optional jitter policy metadata;
- recorded per-entry retry delay, jitter delay, composed delay, and jitter
  class;
- selected deterministic test-mode jitter by hashing only non-secret local
  class data, attempt number, optional short candidate tag, and a fixed test
  label;
- selected non-test jitter from non-secret local class data and local clock;
- added jitter to empty-poll classes and invalid-candidate retry attempts;
- preserved the first invalid-candidate attempt at zero added jitter so valid
  candidate processing is not delayed by the invalid-retry class before local
  verification;
- capped composed retry plus jitter delay at 2250 ms;
- skipped real sleeps when either retry test mode or jitter test mode is set.

No qshield relay server ack/delete logic changed.

## Bounded Jitter Policy

The policy is opt-in:

- disabled by default;
- enabled by `QSHIELD_DEMO_BOUNDED_JITTER=1`;
- demo/test retry ledger is active if either retry cadence or bounded jitter is
  enabled;
- maximum added jitter is 250 ms;
- composed invalid/empty-poll delay is capped at 2250 ms.

The selected jitter does not use route tokens, raw ack IDs, plaintext,
candidate bodies, passphrases, or key material. The existing candidate tag is a
short digest-derived local tag and is not a raw ack ID.

## Deterministic Test Mode

`QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1` makes jitter selection deterministic
and disables real sleeps for jitter-enabled runs. The NA-0329 harness combines
this with `QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE=1` so CI does not depend on
wall-clock sleeps or nondeterministic timing.

## Future/Actual Implementation Boundary

Actual implementation:

- qshield embedded relay/demo receive retry ledger only;
- empty-poll jitter;
- invalid-candidate retry jitter;
- deterministic local-demo test evidence.

Not implemented:

- qsl-server production timing behavior;
- qsl-attachments production timing or object-size timing behavior;
- cover traffic;
- batching;
- transport padding changes;
- broad queue scheduling;
- qsc/qsp/protocol/crypto/key-schedule behavior.

## Abuse/DoS/Latency/Compatibility Behavior

- Invalid candidate attempts remain capped at four per 60 second local window.
- Empty poll remains successful and bounded.
- Jitter adds no more than 250 ms per eligible class.
- Retry plus jitter is capped at 2250 ms for the covered classes.
- Test mode avoids long sleeps and nondeterministic CI timing.
- If the retry/jitter ledger cannot be read, parsed, serialized, or written
  while enabled, the existing fail-closed retry-ledger error path remains in
  force.
- Valid ack/delete semantics are unchanged.

## Valid Path Proof

Harness:
`apps/qshield-cli/tests/na_0329_metadata_runtime_bounded_jitter.rs`

The valid path test queues two relay candidates, verifies repeated
candidate-polling does not delete the first candidate, a valid ack deletes
exactly one candidate, and the second candidate remains. It emits:

- `NA0329_VALID_DELIVERY_UNCHANGED_OK`

## Invalid Retry Jitter Proof

The invalid retry test queues a syntactically invalid candidate and runs
`qshield recv` under:

- `QSHIELD_DEMO_RETRY_CADENCE=1`
- `QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE=1`
- `QSHIELD_DEMO_BOUNDED_JITTER=1`
- `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`

It proves:

- retry delays remain 0, 500, 1000, 2000 ms;
- invalid-candidate retry jitter is 0-250 ms;
- composed delay remains at or below 2250 ms;
- the fifth invalid attempt fails closed at the retry cap;
- no local accepted state changes;
- no plaintext output appears;
- the candidate remains queued before local verification succeeds.

Markers:

- `NA0329_INVALID_RETRY_JITTER_BOUNDED_OK`
- `NA0329_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0329_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0329_NO_ACCEPTED_STATE_ON_INVALID_JITTER_OK`
- `NA0329_NO_OUTPUT_ON_INVALID_JITTER_OK`

## Empty Poll Jitter Proof

The empty-poll test runs repeated empty receives under retry plus jitter test
mode. It proves:

- empty receive remains successful with `no messages`;
- retry delays remain 0, 500, 1000, 2000, 2000 ms;
- empty-poll jitter is 0-250 ms;
- composed delay remains at or below 2250 ms;
- no real sleep is required in CI.

Marker:

- `NA0329_EMPTY_POLL_JITTER_BOUNDED_OK`

## Ack Retry Proof

The ack retry test submits duplicate and stale ack IDs after a valid ack. The
relay keeps the remaining candidate queued and returns fail-closed responses for
duplicate/stale ack attempts. NA-0329 does not add post-verify ack jitter.

Marker:

- `NA0329_ACK_RETRY_JITTER_FAIL_CLOSED_OK`

## No Remote Delete Before Verify Proof

The invalid retry test polls the same candidate before and after every invalid
receive attempt and after the cap failure. The ack ID remains the same, proving
the candidate is not deleted remotely before local verification succeeds.

Marker:

- `NA0329_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`

## No Accepted State/Output Proof

The invalid retry test snapshots `state.json` before invalid receive attempts
and compares it after each run. It also scans combined stdout/stderr for
plaintext output and the plaintext sentinel.

Markers:

- `NA0329_NO_ACCEPTED_STATE_ON_INVALID_JITTER_OK`
- `NA0329_NO_OUTPUT_ON_INVALID_JITTER_OK`

## Artifact/Log Safety Proof

The harness scans command output and the jitter ledger for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel;
- actual route token and ack ID;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace text;
- sensitive local absolute path prefixes.

Result:

- `JITTER_ARTIFACT_SECRET_FINDING_COUNT 0`
- `NA0329_NO_SECRET_JITTER_ARTIFACT_OK`

## Harness Markers

The executable harness declares and emits:

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

No required NA-0329 marker is knowingly omitted.

## qshield Embedded Relay/Demo Boundary

This proof is limited to the qshield embedded relay/demo receive surface and
the qshield demo relay ack behavior exercised by the harness. It is local/demo
evidence only.

## qsl-server/qsl-attachments Production Boundary

qsl-server production timing remains unproven and cross-repo-gated.
qsl-attachments production timing and object-size timing behavior remain
unproven and cross-repo-gated.

No qsl-server or qsl-attachments file is changed by NA-0329.

## Claim Boundaries

Allowed statement:

- NA-0329 implements bounded, opt-in qshield embedded relay/demo jitter for the
  covered receive retry classes and proves it with deterministic local tests.

Not allowed and not claimed:

- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim of metadata-free behavior;
- no claim of anonymity;
- no claim of untraceable behavior;
- no claim of production readiness;
- no claim of public-internet readiness;
- no claim of external-review completion;
- no claim that qshield demo proof is production-service proof.

## Selected Successor

Selected successor:

`NA-0330 -- Metadata Runtime qshield Demo Batching Authorization Plan`

Rationale:

- NA-0329 completed the bounded jitter implementation/harness without a
  blocker;
- retry cadence and bounded jitter are now both executable qshield demo-local
  controls;
- NA-0325 ranked batching after bounded jitter as the next qshield demo
  mitigation design lane;
- batching still requires authorization/design before any implementation.

Rejected successors:

- bounded jitter blocker resolution, because NA-0329 completed;
- service timing cross-repo authorization, because qshield demo mitigation
  lanes still have queued design work and production timing remains gated;
- cover traffic implementation, because it is higher cost and not authorized;
- qsl-server or qsl-attachments production timing implementation, because
  those are cross-repo-gated.

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked changes remain inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already covered by the
operator backup scope. Runtime test ledgers are temporary under per-test system
temp directories and are removed by the harness. No durable evidence location
outside the current backup scope is introduced.

## Next Recommendation

Merge the NA-0329 implementation/harness after required local validation and
required PR checks are green. Then close out NA-0329 and restore exactly one
successor:

`NA-0330 -- Metadata Runtime qshield Demo Batching Authorization Plan`
