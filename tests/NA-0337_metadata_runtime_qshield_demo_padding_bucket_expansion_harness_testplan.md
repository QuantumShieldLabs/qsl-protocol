Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0337 Metadata Runtime qshield Demo Padding Bucket Expansion Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qshield embedded relay/demo padding bucket expansion
implementation and executable harness selected by NA-0336.

## Protected Invariants

- Padding bucket expansion remains qshield embedded relay/demo only.
- Valid padded candidates strip/verify before ack.
- Invalid padding rejects before actor decode.
- Invalid config rejects deterministically.
- No remote delete occurs before local verification.
- Invalid padding creates no accepted state or plaintext output.
- Max padded payload remains 8192 bytes.
- Max expanded-policy overhead remains 1023 bytes.
- Artifacts and logs remain secret-free.
- Batching, retry cadence, bounded jitter, and cover prototype bounds remain
  intact.
- qsl-server and qsl-attachments production boundaries remain explicit.

## Allowed Scope

- `apps/qshield-cli/src/config.rs`
- `apps/qshield-cli/src/commands/init.rs`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/tests/na_0337_metadata_runtime_padding_bucket_expansion.rs`
- governance evidence, testplan, decisions, traceability, and rolling journal.

## Forbidden Scope

- qsl-server implementation.
- qsl-attachments implementation.
- qsc/qsp/protocol/crypto/key-schedule implementation.
- Cargo manifest or lockfile changes.
- dependency changes.
- workflow, branch-protection, or public-safety changes.
- website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tools/refimpl, production service, or attachment-size padding changes.

## Prior Authorization Review Requirements

Before implementation, review:

- live NA-0337 scope in `NEXT_ACTIONS.md`;
- NA-0336 padding bucket expansion authorization evidence and testplan;
- NA-0319 identifier/default-padding harness evidence and testplan;
- NA-0335 qshield demo cover prototype evidence and testplan;
- current qshield send/receive/init/relay/config implementation;
- decisions and traceability state.

## Padding Bucket Implementation Requirements

- Policy: `qshield_demo_padding_bucket_expansion_v1`.
- Opt-in profile: `QSHIELD_DEMO_PADDING_BUCKETS=expanded`.
- Bucket table:
  `[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`.
- Select the smallest bucket that can hold the candidate payload.
- Reject payloads or metadata above 8192 bytes when padded.
- Keep no-padding behavior compatible.

## Harness Marker Requirements

The harness must truthfully emit:

- `NA0337_PADDING_BUCKET_AUTHORIZATION_OK`
- `NA0337_PADDING_BUCKET_POLICY_OK`
- `NA0337_DETERMINISTIC_TEST_PADDING_OK`
- `NA0337_VALID_SMALL_MESSAGE_PADDING_OK`
- `NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK`
- `NA0337_VALID_LARGE_MESSAGE_PADDING_OK`
- `NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0337_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0337_PADDING_STRIP_VERIFY_OK`
- `NA0337_PADDING_MALFORMED_REJECT_OK`
- `NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK`
- `NA0337_PADDING_NO_SECRET_ARTIFACT_OK`
- `NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK`
- `NA0337_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0337_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0337_NO_METADATA_FREE_CLAIM_OK`
- `NA0337_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0337_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0337_METADATA_RUNTIME_PADDING_BUCKET_EXPANSION_OK`

## Invalid Config Requirements

Reject deterministically and without writing a config for:

- empty config;
- zero bucket;
- negative or nonnumeric bucket;
- duplicate-after-normalization bucket;
- unsorted bucket table;
- bucket above 8192;
- invalid environment policy value.

Also reject hand-written invalid send config before actor/session work.

## Malformed Padding Requirements

Malformed candidate padding must reject when:

- pad length exceeds candidate length;
- `pad_len > 0` and bucket metadata is missing;
- bucket length does not match candidate byte length;
- bucket exceeds 8192;
- stripped padding bytes are not zero.

## No-Remote-Delete Requirements

On invalid padding receive, the remote candidate remains queued and visible
through `/poll-candidate` with the same ack handle.

## No-Local-Output / State Requirements

On invalid padding receive:

- local state bytes remain unchanged;
- no accepted `from <peer>:` plaintext output is emitted;
- no plaintext or padding sentinel appears in output.

## No-Secret-Artifact Requirements

Scan command output, retry ledger text, and response bodies for:

- route-token sentinel;
- raw-handle sentinel;
- candidate/ack sentinel;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw-key sentinel;
- panic/backtrace markers;
- local sensitive path markers.

Required proof lines:

- `PADDING_ARTIFACT_SECRET_FINDING_COUNT 0`
- `PADDING_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `PADDING_NO_PLAINTEXT_SENTINEL_LEAK_OK`

## Production-Boundary Requirements

Evidence must state:

- qshield embedded relay/demo proof is local/demo only;
- qsl-server production padding remains unimplemented and unproven;
- qsl-attachments production object-size padding remains unimplemented and
  unproven;
- attachment-size padding remains future-gated.

## Claim-Boundary Requirements

Do not claim:

- anonymity;
- metadata-free behavior;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- that timing metadata is hidden;
- that traffic shape is hidden;
- that padding removes all metadata.

## Backup-Impact Requirements

Record whether changes create durable evidence outside the current covered
worktree. Expected result: no backup-plan update required.

## Required Local Checks

- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0337_metadata_runtime_padding_bucket_expansion -- --test-threads=1 --nocapture`
- neighboring qshield metadata runtime harnesses for NA-0335, NA-0331, NA-0329,
  NA-0327, NA-0324, NA-0322, NA-0320, NA-0319, and NA-0318 when feasible
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- demo smoke/stress/soak as directed
- dependency, model, evidence-helper, link, leak, goal-lint, and scope-guard
  checks before PR.

## CI Expectations

The PR may merge only after required checks attach and complete successfully.
`public-safety` must remain required and green before merge and after merge.

## Successor Handoff

If implementation/harness succeeds, select:

`NA-0338 -- Metadata Runtime Attachment Size-Class Authorization Plan`

Do not implement NA-0338 in this lane.
