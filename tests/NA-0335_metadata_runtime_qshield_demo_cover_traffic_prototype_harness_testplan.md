Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0335 Metadata Runtime qshield Demo Cover Traffic Prototype Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0335 implements a bounded qshield embedded relay/demo
cover-traffic prototype harness, or records an exact blocker, without changing
production service behavior or strengthening privacy/readiness claims.

## Protected Invariants

- Cover traffic remains qshield embedded relay/demo only.
- Cover traffic is opt-in and disabled by default.
- Real demo messages remain priority over cover.
- Cover candidates are never delivered as user plaintext.
- Cover candidates do not corrupt or replace real messages.
- Cover item generation obeys inherited NA-0333 caps.
- Cover artifacts/logs are secret-free and bounded.
- No recursive cover generation is introduced.
- NA-0318 ack/commit, NA-0319 identifier/default-padding, NA-0320
  sanitized-error/retention, NA-0322 measurement, NA-0324 instrumentation,
  NA-0327 retry cadence, NA-0329 bounded jitter, and NA-0331 batching behavior
  remain intact.

## Allowed Scope

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0335_metadata_runtime_cover_traffic_prototype.rs`
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation changes.
- qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Dependency, Cargo, workflow, branch-protection, or public-safety changes.
- Website, docs/public, README, or START_HERE changes.
- Fixed-rate cover, production-service cover, public-internet cover,
  attachment object cover, or transport padding expansion.
- NA-0336 implementation.

## Prior Authorization Review Requirements

Review and preserve:

- NA-0334 qshield demo cover-traffic prototype authorization.
- NA-0333 cost/quota/retention prerequisite caps.
- NA-0332 risk gate and production-service boundary.
- NA-0331 batching proof.
- NA-0329 bounded-jitter proof.
- NA-0327 retry-cadence proof.
- NA-0324 instrumentation proof.
- NA-0322 measurement proof.
- NA-0320 sanitized-error/retention proof.
- NA-0319 identifier/default-padding proof.
- NA-0318 ack/commit proof.

## Cover Prototype Implementation Requirements

- Implement only qshield demo synthetic local cover, active-session cover, and
  batch-fill cover.
- Require `QSHIELD_DEMO_COVER_TRAFFIC=1`.
- Require deterministic fast tests through
  `QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE=1`.
- Enforce inherited caps before generation.
- Abort before generation when `/srv/qbuild` free disk is below 10 GiB or
  cannot be proven.
- Store retained artifact summaries only in bounded in-memory relay state.
- Keep qshield receive behavior from printing cover as user plaintext.

## Harness Marker Requirements

The executable harness must truthfully emit:

- `NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK`
- `NA0335_SYNTHETIC_LOCAL_COVER_OK`
- `NA0335_ACTIVE_SESSION_COVER_OK`
- `NA0335_BATCH_FILL_COVER_OK`
- `NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK`
- `NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK`
- `NA0335_COVER_ITEM_PURGE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK`
- `NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK`
- `NA0335_REAL_MESSAGE_PRIORITY_OK`
- `NA0335_NO_RECURSIVE_COVER_GENERATION_OK`
- `NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK`
- `NA0335_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0335_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0335_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0335_NO_METADATA_FREE_CLAIM_OK`
- `NA0335_NO_ANONYMITY_CLAIM_OK`
- `NA0335_METADATA_RUNTIME_COVER_TRAFFIC_PROTOTYPE_OK`

If any marker cannot be emitted truthfully, the evidence must record the exact
blocker and the successor must be a blocker-resolution lane.

## Cost / Quota / Retention / Abuse Requirements

Validate:

- 8192 byte payload/item cap.
- 4 item/minute cap.
- 32 item/hour cap.
- 64 item/run cap.
- 512 KiB payload/run cap.
- about 1 MiB request/run cap.
- 16 queued cover items global.
- 4 queued cover items per route.
- 4 retained cover artifact summaries/run.
- 1 MiB retained cover artifact total.
- disk floor fail-closed behavior.
- route/queue/token caps remain enforced.

## Valid Real-Message Requirements

Validate that real qshield demo messages:

- remain non-cover candidates;
- are returned before cover candidates;
- can still be acked by normal ack behavior;
- are not replaced by cover;
- are not deleted by invalid cover operations.

## Cover-Mode Requirements

Synthetic local cover:

- may use the demo cover source;
- is tagged as cover;
- is bounded by payload and queue caps.

Active-session cover:

- requires an explicit source peer;
- is tagged as cover;
- remains local/demo only.

Batch-fill cover:

- may queue up to the existing four-member qshield demo batch boundary;
- coexists with NA-0331 batching;
- does not change batching retry or jitter caps.

## No Recursive Generation Requirements

Validate that only explicit authenticated `/cover-traffic` requests generate
cover. Polling, acking, receive skip/ack, status, purge, batching, retry, and
jitter paths must not generate additional cover.

## No-Secret-Artifact Requirements

Scan harness-visible cover status, purge output, and receive output for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace terms;
- sensitive absolute path fragments.

Required results:

- `COVER_ARTIFACT_SECRET_FINDING_COUNT 0`
- `COVER_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `COVER_ARTIFACT_COUNT_WITHIN_CAP_OK`

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo proof is local/demo only;
- qsl-server production relay cover remains unimplemented and unproven;
- qsl-attachments production object cover remains unimplemented and unproven;
- production cover requires separate cross-repo authorization.

## Claim-Boundary Requirements

Evidence must not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion.

## Backup-Impact Requirements

Validate that no durable evidence location outside the qsl-protocol worktree is
created. Runtime artifacts must be in-memory or temporary test data. If a new
durable location outside current backup scope is required, stop and request a
backup-plan update.

## Required Local Checks

Run at minimum:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0335_metadata_runtime_cover_traffic_prototype -- --test-threads=1 --nocapture`
- prior qshield metadata-runtime harnesses from NA-0318 through NA-0331 where
  present
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- metadata runtime harness scripts where present
- qsc `send_commit`
- formal model checks
- queue/decision/scope/link/leak/goal checks

## CI Expectations

The PR may merge only if required checks, including `public-safety`, pass
normally. No admin bypass, direct push, squash, rebase, branch deletion, or
protection mutation is allowed.

## Successor Handoff

If NA-0335 succeeds, restore:

`NA-0336 -- Metadata Runtime Padding Bucket Expansion Authorization Plan`

If NA-0335 is blocked, restore:

`NA-0336 -- Metadata Runtime qshield Demo Cover Traffic Prototype Blocker Resolution`
