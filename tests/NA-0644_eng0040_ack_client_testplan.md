# NA-0644 testplan — ENG-0040 qsc ack-client (D580, D-1267)

Scope: prove the OPT-IN acknowledged-pull (lease) client — the persist-then-ack
ordering, msg_id dedup under redelivery, old-server tolerance, the seam handling —
while the LEGACY default stays byte-identical. All positive scenarios run against the
REAL pinned qsl-server (`8e4ea278`) in-process with a REAL 1-second pull lease
(`start_qsl_server_with_store` / `StoreConfig::pull_lease_secs` — real lease expiry,
real redelivery, no simulation). The only test double is a thin recording proxy that
shapes the ACK ROUTE (drop / stall / 404) — the behaviors of a lossy network and of the
pre-durability relay.

## The suite: `qsl/qsl-client/qsc/tests/NA_0644_ack_client.rs` (6 tests)

| # | Test | Proves |
|---|------|--------|
| a | `legacy_default_sends_no_ack_param_and_never_acks` | Without `--ack-mode`, the pull URL is the exact pre-lane string (`/v1/pull?max=…`, no `ack=` — every URI recorded via proxy), ZERO ack POSTs, no lease-mode marker in the output. The backward-compat guard in miniature. |
| b | `lease_happy_path_acks_and_deletes_server_side` | Persist → ack → the server DELETES. Deletion (not lease-invisibility) proven by waiting past the lease: an unacked copy would reappear; the acked one stays gone. |
| c | `lost_ack_redelivery_is_deduped_not_reprocessed` | **THE LANE-PROVER.** Run 1: ack dropped by the network → `ack_failed` WARNING, exit 0, both payloads byte-persisted. Lease expires → REAL redelivery. Run 2: every id `recv_dup_skipped`, zero new recv files, zero reprocessing, zero process-exit, re-acked. Run 3: queue drained for good. |
| d | `crash_between_persist_and_ack_redelivery_deduped` | The ack POST is stalled; WHILE it is in flight the payload is already byte-identical on disk (persist-BEFORE-ack observed directly); SIGKILL mid-stall; redelivery deduped cleanly; drained. |
| e | `old_server_ack_404_is_legacy_complete` | The pre-durability relay (ack param stripped upstream + 404 on the ack route): ONE `ack_legacy_complete` info marker, no error, no retry, payload intact, nothing redelivered, nothing lost. |
| f | `commit_before_write_seam_acked_loudly_no_poison_loop` | Must-have #2 (ENG-0042 handling): a crash forced exactly in the commit→write gap (rename target occupied → `recv_write_failed` AFTER the ratchet commit); the redelivered, cryptographically unrecoverable envelope is acked LOUDLY (`ack_replay_unrecoverable`) and the queue drains — bounded behavior, no poison redelivery loop. |

## Non-vacuity (WF-0017)

Red-run (reverted before commit): with `RelaySeenIds::contains` neutered to `false` and
the lease backstop disabled, test (c) FAILS with exactly today's failure mode —
`qsp_unpack code=qsp_replay_reject` → `error code=qsp_replay_reject` → process-exit 1
(46.36s). The test can fail when dedup is absent, and what it catches is the current
process-exit. Output preserved in `docs/governance/evidence/NA-0644_as_built.md` §7.
Both neuters reverted (verified zero occurrences) and (c) re-ran green before commit.

## Results (local, 2026-07-14, base main `8b9cd774`)

- `cargo test --test NA_0644_ack_client`: **6 passed / 0 failed (223.68s)**, first full run.
- Red-run (reverted): **(c) FAILED as designed** with `error code=qsp_replay_reject`.
- NA-0640 e2e (`cargo test --test NA_0640_full_stack_e2e`), ZERO edits: **2 passed /
  0 failed (118.62s)** — the legacy default proven unchanged end-to-end (the e2e does
  not run on PRs; the local run is the gate).
- Full `cargo test -p qsc` (closeout run, final tree): **609 passed / 0 failed / 3
  ignored (pre-existing `#[ignore]`) across all 150 test-result sets, exit 0** — the
  NA-0643 baseline (603/0/3 across 149 sets) plus exactly this lane's 6 new tests in
  1 new set. Zero regressions.
