Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0327 Metadata Runtime qshield Demo Retry Cadence Normalization Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0327 adds bounded qshield embedded relay/demo retry-cadence normalization
for the qshield receive demo path and an executable harness proving the
authorized retry policy. The implementation is opt-in for demo/test execution
through `QSHIELD_DEMO_RETRY_CADENCE=1`, uses deterministic test mode through
`QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE=1`, and records only a secret-safe local
retry ledger under the qshield demo store.

The harness proves invalid retry bounds, empty poll bounded cadence, stale and
duplicate ack fail-closed behavior, valid ack-once behavior, no remote delete
before local verification, no accepted local state or plaintext output on
invalid retry, no secret retry artifacts, and the qshield-demo-only boundary.

Selected successor:

`NA-0328 -- Metadata Runtime qshield Demo Bounded Jitter Authorization Plan`

## Live NA-0327 Scope

The live queue item is `NA-0327 -- Metadata Runtime qshield Demo Retry Cadence
Normalization Implementation Harness`, status `READY`, with goals G1 through
G5.

Allowed work:

- bounded qshield embedded relay/demo retry-cadence normalization harness or
  exact prerequisite stop;
- proof for invalid retry bounded behavior, empty poll bounded behavior, stale
  ack fail-closed behavior, valid ack-once behavior, no remote delete before
  local verify, no accepted state/output on invalid retry, and no secret retry
  artifacts;
- proof that qshield embedded relay/demo timing evidence remains distinct from
  qsl-server and qsl-attachments production timing;
- proof that prohibited claim families remain prohibited.

Forbidden work:

- qsl-server or qsl-attachments implementation;
- qsc/qsp/protocol/crypto/key-schedule implementation;
- dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration changes;
- jitter, batching, cover traffic, transport padding expansion, send
  scheduling, production service timing behavior, or NA-0328 implementation.

## Inherited NA-0326 Authorization

NA-0326 authorized only a future qshield embedded relay/demo retry-cadence
normalization implementation harness. It defined policy
`qshield_demo_retry_cadence_v1`, a 60 second local window, a maximum of four
invalid candidate attempts per window, and a 500 ms / 1000 ms / 2000 ms capped
backoff sequence after the first immediate attempt.

NA-0326 also required:

- valid ack deletes exactly one candidate;
- duplicate and stale ack attempts fail closed or deterministic no-op;
- no remote candidate delete before local verification;
- invalid retry creates no accepted local state and no plaintext output;
- retry artifacts do not contain route tokens, raw handles, raw ack/candidate
  IDs, plaintext or padding sentinels, passphrases, raw key material, panic, or
  backtrace text;
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.

## Inherited NA-0324 Instrumentation Evidence

NA-0324 added bounded qshield embedded relay/demo instrumentation around send,
candidate fetch, local verify, ack/commit, invalid retry, output
classification, queue cadence, padding/size classes, and ordering/correlation
classes. That instrumentation was measurement evidence, not retry
normalization, and did not prove qsl-server or qsl-attachments production
timing.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0327 entry.
- `docs/governance/evidence/NA-0326_metadata_runtime_qshield_demo_retry_cadence_normalization_authorization.md`.
- `tests/NA-0326_metadata_runtime_qshield_demo_retry_cadence_normalization_authorization_testplan.md`.
- `docs/governance/evidence/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix.md`.
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
- `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`.
- `DECISIONS.md` and `TRACEABILITY.md`.

Search coverage included retry cadence, backoff, invalid retry, empty poll,
stale ack, duplicate ack, valid ack, remote delete, local verify, accepted
state, output, artifact, secret, sentinel, qshield, qsl-server,
qsl-attachments, production, timing-hidden, traffic-hidden, metadata-free,
untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Implementation Summary or Blocker

No implementation blocker was found inside the allowed qshield surface.

Changed qshield files:

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`

The qshield receive path now supports opt-in demo retry-cadence state:

- ledger path: `.qshield_demo_retry_cadence_v1.json` under the qshield demo
  store;
- policy name: `qshield_demo_retry_cadence_v1`;
- window: 60,000 ms;
- invalid candidate cap: four attempts per candidate tag per window;
- backoff classes: immediate first attempt, then 500 ms, 1000 ms, 2000 ms
  capped;
- deterministic test mode: skips actual sleep while preserving ledger classes.

The local candidate tag is a short digest-derived tag, not a raw ack ID or raw
candidate value. Valid ack success clears the invalid-candidate retry entry.

## Retry-Cadence Policy

Invalid candidate retry:

- first attempt: local verification may run immediately;
- attempts two through four: use 500 ms, 1000 ms, and 2000 ms capped classes;
- fifth and later attempts for the same candidate tag inside the 60 second
  window fail closed before local verification and before any remote ack/delete;
- invalid retry creates no accepted state and no plaintext output.

Empty poll retry:

- repeated empty receive polls use the same immediate, 500 ms, 1000 ms, and
  2000 ms capped classes;
- empty poll stays at the 2000 ms cap instead of failing, because no invalid
  candidate work is being processed.

Ack/commit:

- valid ack deletes exactly one queued candidate;
- duplicate valid ack fails closed after the first commit;
- stale ack fails closed without deleting remaining candidates.

## Future / Actual Implementation Boundary

Actual NA-0327 behavior is bounded to the qshield embedded relay/demo receive
path and is opt-in through demo/test environment variables. It does not change
qsl-server, qsl-attachments, qsc, qsp, protocol, crypto, key schedule, Cargo,
workflow, website, README, START_HERE, branch protection, or public-safety
configuration.

NA-0327 does not implement jitter, batching, cover traffic, broad queue
scheduling, send scheduling, transport padding expansion, service timing, or
NA-0328 behavior.

## Abuse / DoS / Latency / Compatibility Behavior

- Repeated invalid candidate processing is capped at four attempts per 60
  second local window for the same local candidate tag.
- Empty poll cadence is bounded at the 2000 ms class and remains successful.
- Deterministic test mode prevents CI from waiting on real backoff sleeps.
- Valid candidate delivery compatibility is preserved: the retry entry is
  cleared after the valid ack succeeds.
- If retry ledger state cannot be read, parsed, serialized, or written while
  the policy is enabled, qshield fails closed with a coarse retry-cadence state
  error.

## Valid Path Proof

Harness: `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`

The harness sends two queued candidates, proves repeated candidate fetch does
not delete the first candidate, a valid ack removes exactly one candidate, the
second candidate remains, and duplicate ack of the first candidate fails
closed.

Marker:

- `NA0327_VALID_ACK_ONCE_OK`

## Invalid Retry Proof

The harness queues a malformed candidate, runs `qshield recv` under the demo
retry policy five times in deterministic test mode, and proves:

- attempts one through four record delay classes 0, 500, 1000, and 2000 ms;
- attempt five fails closed with the cap reached;
- the same candidate remains queued;
- `state.json` remains unchanged;
- no accepted plaintext output is produced.

Marker:

- `NA0327_INVALID_RETRY_BOUNDED_OK`

## Empty Poll Proof

The harness runs five empty receive polls under the demo retry policy and
proves empty-poll ledger attempts record 0, 500, 1000, 2000, and capped 2000 ms
classes without long sleeps.

Marker:

- `NA0327_EMPTY_POLL_RETRY_BOUNDED_OK`

## Stale Ack Proof

The harness submits a syntactically valid but absent ack ID after one candidate
remains queued and proves the relay returns fail-closed status without deleting
the remaining candidate.

Marker:

- `NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK`

## Duplicate Ack Proof

The harness submits the first valid ack a second time after it already deleted
one candidate and proves the relay returns fail-closed status without deleting
the remaining candidate.

Marker:

- `NA0327_DUPLICATE_ACK_RETRY_FAIL_CLOSED_OK`

## No Remote Delete Before Verify Proof

Repeated `/poll-candidate` calls return the same first ack handle before any
ack is submitted, and invalid receive attempts leave the same candidate queued.

Marker:

- `NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`

## No Accepted State / Output Proof

The invalid retry test stores `state.json` before the invalid receive sequence
and compares it after each invalid attempt and after the cap failure. The
combined stdout/stderr is scanned to ensure no `from alice:` plaintext output
or plaintext sentinel appears.

Markers:

- `NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK`

## Artifact / Log Safety Proof

The harness scans command output and the retry ledger for route token sentinel,
raw handle sentinel, candidate/ack sentinel, actual ack ID, plaintext sentinel,
padding sentinel, passphrase sentinel, raw key sentinel, panic/backtrace text,
and sensitive absolute local path prefixes.

Result:

- `RETRY_ARTIFACT_SECRET_FINDING_COUNT 0`

Marker:

- `NA0327_NO_SECRET_RETRY_ARTIFACT_OK`

## Harness Markers

The harness emits:

- `NA0327_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0327_RETRY_NORMALIZATION_POLICY_OK`
- `NA0327_INVALID_RETRY_BOUNDED_OK`
- `NA0327_EMPTY_POLL_RETRY_BOUNDED_OK`
- `NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK`
- `NA0327_DUPLICATE_ACK_RETRY_FAIL_CLOSED_OK`
- `NA0327_VALID_ACK_ONCE_OK`
- `NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK`
- `NA0327_NO_SECRET_RETRY_ARTIFACT_OK`
- `NA0327_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0327_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0327_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0327_NO_METADATA_FREE_CLAIM_OK`
- `NA0327_METADATA_RUNTIME_RETRY_CADENCE_NORMALIZATION_OK`

## qshield Embedded Relay / Demo Boundary

The proof is qshield embedded relay/demo only. The retry ledger is local demo
state and the executable harness starts the local qshield relay binary.

Marker:

- `NA0327_QSHIELD_DEMO_BOUNDARY_OK`

## qsl-server / qsl-attachments Production Boundary

qsl-server production relay timing and qsl-attachments production object timing
remain unproven and cross-repo-gated. No qsl-server or qsl-attachments file is
changed.

Marker:

- `NA0327_SERVICE_PRODUCTION_BOUNDARY_OK`

## Claim Boundaries

NA-0327 does not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion.

Markers:

- `NA0327_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0327_NO_METADATA_FREE_CLAIM_OK`

## Selected Successor

Selected successor:

`NA-0328 -- Metadata Runtime qshield Demo Bounded Jitter Authorization Plan`

Rationale: retry-cadence normalization is now bounded and executable in the
qshield demo lane. The next safest timing/traffic-shape lane is a separate
authorization plan for bounded jitter, still qshield demo-only and still not a
production service claim.

Rejected alternatives:

- retry-cadence blocker resolution, because NA-0327 completed the bounded demo
  implementation/harness;
- qsl-server or qsl-attachments production timing, because those require
  cross-repo service authorization;
- cover traffic, batching, or padding expansion as the immediate successor,
  because each has broader cost, abuse, or behavior scope than a bounded jitter
  authorization plan.

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked evidence stays under the
qsl-protocol worktree covered by `/srv/qbuild/work`; runtime retry ledgers are
temporary qshield demo store artifacts created under test temp directories and
are not committed. The preserved D132 bundle was not deleted or modified.

## Next Recommendation

Merge NA-0327 only after required local validation and required GitHub checks
pass. After merge and green post-merge public-safety, close NA-0327 and restore
exactly one READY item:

`NA-0328 -- Metadata Runtime qshield Demo Bounded Jitter Authorization Plan`
