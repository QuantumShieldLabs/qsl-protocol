# NA-0644 as-built — ENG-0040 qsc ack-client (D580, D-1267)

Lane: NA-0644 per QSL-DIR-2026-07-13-580 (D580, APPROVED), seated by promotion PR #1567.
Base main: `8b9cd774`. Date: 2026-07-14. Result class: **QSC_ACK_CLIENT_PASS** (limits in §6).

## §1 What shipped

The qsc client can now speak the NA-0642 acknowledged-pull contract, OPT-IN:

- **Selector** (design-lock Q1, operator-approved): `receive --ack-mode <legacy|lease>`
  (clap `value_enum`, `cmd/mod.rs` `AckMode`), absent → `Legacy`. Flag-only this lane;
  the config key belongs to the ENG-0043 default-flip lane.
- **Lease pull**: `relay_inbox_pull_mode` — `GET /v1/pull?max=N&ack=lease` in lease mode;
  the legacy URL string is character-identical to the pre-lane one. The public
  `relay_inbox_pull` (used by handshake + TUI) delegates with `Legacy` and is unchanged
  for its callers.
- **Dedup store** (Q2, per-item durable): NEW `src/dedup/mod.rs` — `RelaySeenIds`,
  a per-mailbox JSON file `<cfg_dir>/relay_seen_ids_v1_<hash>.json` (hash of the
  normalized route token; the raw token never appears in a filename), written ONLY via
  `write_atomic` (the NA-0639-exercised temp→fsync→rename seam). Prune at load: entries
  older than 31 days (strictly beyond the relay's 30-day retention ceiling — no
  redelivery can outlive its entry) and a 65,536-entry cap (evict oldest). Missing file
  = empty; corrupt/wrong-version = empty + `dedup_store_reset` warning (a dupe slipping
  past an empty store is still caught by the backstop, not reprocessed).
- **Dedup check BEFORE unpack**: first statement of the per-item loop — a seen id is
  acked-and-skipped (`recv_dup_skipped`), never unpacked.
- **Ack batching** (Q3): `pending_acks` accumulates per item; ONE
  `flush_pending_acks` immediately after the pull loop — BEFORE attachment resume (a
  long content download must not hold acks past the lease clock; a descriptor item is
  durable at its pending-record commit) and BEFORE `flush_batched_receipts`. The POST
  (`relay_inbox_ack`, `AckReq{ids}`) chunks at ≤4096 (`RELAY_ACK_MAX_IDS` = the server's
  `MAX_ACK_IDS`). An ack failure is a WARNING (`ack_failed`) and a normal exit — never a
  failed receive: everything queued is already durably persisted; the lease expires and
  the redelivery is deduped.
- **Old-server tolerance** (Q4): HTTP 404 on the ack → `AckOutcome::LegacyComplete` →
  one `ack_legacy_complete` info marker, remaining chunks short-circuited, success —
  no error, no retry, no re-wait, no persisted server-version state.
- **The backstop** (Q3 of the operator approval; lease mode ONLY): the
  `qsp_replay_reject` arm no longer process-exits — it emits the LOUD
  `ack_replay_unrecoverable` marker, records the id, acks it, and continues. Legacy
  mode keeps the process-exit unchanged. See §4 (the seam) and ENG-0042.

## §2 The ordering audit (persist+seen before ack; dedup before unpack)

The invariant: **an id enters `pending_acks` only after (a) the item's own durable
commit and (b) its seen-entry is durably on disk.** Mechanically:

1. **Dedup before unpack** — the seen-check is the FIRST statement of the per-item loop
   in `receive_pull_and_write`, before `qsp_unpack_for_peer`. A recognized dupe never
   reaches the ratchet.
2. **Persist before seen-record** — `record_seen_and_queue_ack` is called at exactly the
   SEVEN success exits of the item body: the six `continue` sites (SCKA control;
   attachment descriptor; file chunk/manifest; attachment confirm; file confirm;
   delivered-receipt ack) and the plain-message loop tail. Each site is reachable only
   AFTER that item's `commit_unpack_state` (and, for plain messages, after the
   `write_atomic` payload write — a failed write process-exits before the call). Every
   failure path process-exits without reaching a record site.
3. **Seen-record durable before ack-eligibility** — `RelaySeenIds::record` returns Ok
   only after `write_atomic` of the seen file succeeds; only then is the id pushed to
   `pending_acks`. A failed seen-write is fail-closed (`dedup_store_write_failed`
   process-exit): the id is never acked, the lease expires, the redelivery is handled.
4. **Ack last** — `flush_pending_acks` runs once, after the pull loop, so every ack the
   server receives refers to items whose payload/state AND seen-entry are on disk.
5. **Never delete-equivalent before ack** — the client never deletes anything; the
   server deletes only leased copies and only on the ack (route-scoped, idempotent,
   the NA-0275 unleased-duplicate contract — verified at 8e4ea278 `store.rs` in the
   read-only investigation).

Crash placement → outcome:
- before/inside unpack → nothing consumed, no ack → redelivery → normal processing;
- after commit, before payload write (plain msg) → THE SEAM (§4): key consumed, payload
  absent → backstop acks loudly on redelivery — bounded at one message;
- after payload write, before seen-write → no ack → redelivery → seen-miss → the
  envelope replays into `qsp_replay_reject` → the backstop acks loudly (the payload
  file from the crashed run IS on disk; the loud marker over-warns — accepted, stated);
- after seen-write, before ack → no ack → redelivery → clean `recv_dup_skipped`
  (proven by tests (c) and (d));
- after ack → server deleted; nothing redelivers.

## §3 Legacy byte-identity (the compat guard)

With `--ack-mode` absent: `seen_ids` is `None` (the dedup check and all seven record
calls are no-ops), `pending_acks` stays empty (the flush returns immediately, no
marker), the pull URL is the character-identical pre-lane string, the replay-reject arm
is unchanged, and no new marker is emitted (`recv_ack_mode` is lease-gated). Proven:
test (a) records every pull URI through a proxy (`/v1/pull?max=…`, no `ack=`), observes
ZERO ack POSTs, and asserts no lease-mode marker in the output; the NA-0640 e2e ran
locally GREEN with ZERO edits (2/2, 118.62s — message + >4 MiB attachment byte-verified,
open + bearer auth, wrong-bearer negative). `NA_0640_full_stack_e2e.rs` and the existing
`start_qsl_server` were not touched; `tests/common/mod.rs` gained only an additive
helper (`start_qsl_server_with_store`) plus one name in an import list.

## §4 The commit-before-write seam (must-have #1: FILED as ENG-0042)

Pre-existing, verified at design-lock: a plain message's ratchet commit
(`commit_unpack_state`) durably consumes the message key BEFORE `write_atomic` writes
the plaintext. A crash in that gap makes that ONE message cryptographically
unrecoverable (the key is consumed and zeroized; redelivery cannot decrypt). This lane
did NOT move the seam; it bounded the blast radius: in lease mode the redelivered
envelope is acked LOUDLY (`ack_replay_unrecoverable`) so the redelivery loop ends
instead of process-exiting forever; in legacy mode nothing changed (and the legacy
server had already deleted — the loss shape is the status quo). **Filed as ENG-0042**;
the real fix (write-before-commit reorder / two-phase persist) interacts with the
no-mutation-on-reject discipline and is its own lane. Must-have #2 is test (f): the
seam handling is PROVEN bounded (crash in the gap → loud ack → queue drains → no
poison loop).

## §5 Test results (all against the REAL pinned qsl-server in-process)

`tests/NA_0644_ack_client.rs` — 6/6 green on the first full run (223.68s), real 1-second
lease (`start_qsl_server_with_store`, `StoreConfig::pull_lease_secs` — real expiry, real
redelivery; the only shaping is a thin recording proxy on the ACK ROUTE):

- (a) `legacy_default_sends_no_ack_param_and_never_acks` — §3.
- (b) `lease_happy_path_acks_and_deletes_server_side` — ack fires, and DELETION (not
  lease-invisibility) is proven by waiting past the lease: the acked message does not
  reappear.
- (c) `lost_ack_redelivery_is_deduped_not_reprocessed` — **THE LANE-PROVER**: run 1
  persists both messages, its ack is dropped (`ack_failed`, exit 0); the lease expires;
  run 2 dedups every redelivered id (`recv_dup_skipped` ≥2, zero new recv files, zero
  `ack_replay_unrecoverable`, exit 0) and re-acks; run 3 (past another lease) proves the
  queue drained for good.
- (d) `crash_between_persist_and_ack_redelivery_deduped` — the ack POST is STALLED, and
  while it is in flight the payload is already byte-identical on disk
  (persist-BEFORE-ack observed directly); SIGKILL mid-stall; redelivery deduped; drained.
- (e) `old_server_ack_404_is_legacy_complete` — the proxy strips the ack param (the old
  server deserializes `PullQuery{max}` only) and 404s the ack route: one
  `ack_legacy_complete`, no `ack_failed`, no error, payload intact, nothing redelivered,
  nothing lost.
- (f) `commit_before_write_seam_acked_loudly_no_poison_loop` — must-have #2: the rename
  target is occupied so the run crashes exactly in the commit→write gap
  (`recv_write_failed` AFTER the ratchet commit, before any ack); the redelivery is
  acked LOUDLY (`ack_replay_unrecoverable` + `relay_ack`) and the queue drains — the
  caveat converted into proven bounded behavior.

**Non-vacuity (WF-0017)**: red-run with the dedup check and backstop locally neutered
(reverted before commit): test (c) FAILS with exactly today's failure mode —
`qsp_unpack code=qsp_replay_reject ok=false` → `ratchet_replay_reject` →
`error code=qsp_replay_reject` → process-exit 1. Output preserved in §7. The test can
fail if dedup is absent, and what it catches is the current process-exit.

NA-0640 e2e local (the merge-relevant gate; it does not run on PRs): **2 passed /
0 failed (118.62s), zero edits**. Full `cargo test -p qsc` (closeout run, final tree):
**609 passed / 0 failed / 3 ignored (pre-existing) across all 150 test-result sets,
exit 0** — the NA-0643 baseline (603/149) plus exactly this lane's 6 new tests in 1
new set. Zero regressions.

## §6 Result classification and limits

**QSC_ACK_CLIENT_PASS**, scoped honestly:
- PASS asserts OPT-IN lease-mode delivery durability with dedup UNDER THE TESTED
  SCENARIOS. Lease is NOT the default (by D580 design; the flip is ENG-0043, owed) and
  not every relay supports it (old servers fall back to legacy-complete).
- **The claim is: lease mode + dedup close the client's pull→persist crash window
  EXCEPT the bounded, pre-existing, now-FILED commit-before-write seam (ENG-0042)** —
  one message per crash, handled loudly, redelivery loop ends. NOT "delivery is now
  durable, full stop."
- A stale seen-set (crash after payload write, before seen-write) makes a redelivered
  fully-persisted item take the LOUD backstop path instead of the clean skip — the
  marker over-warns; nothing is lost or reprocessed. Accepted and stated.
- Duplicate-id copies pushed legitimately (NA-0275; `x-msg-id` is client-suppliable)
  are ack-and-skipped in lease mode — strictly better than today's process-exit;
  recorded as behavior, not claimed as a feature.
- Claim boundary otherwise UNCHANGED.

## §7 Red-run output (reverted neuter; preserved verbatim, trimmed)

```
test lost_ack_redelivery_is_deduped_not_reprocessed ... FAILED
command failed: ["receive", …, "--ack-mode", "lease"]
QSC_MARK/1 event=recv_start transport=relay mailbox=redacted mailbox_hash=ac393123 from=bob max=8
QSC_MARK/1 event=recv_ack_mode mode=lease
QSC_MARK/1 event=qsp_unpack code=qsp_replay_reject ok=false
QSC_MARK/1 event=ratchet_replay_reject msg_idx=2
QSC_MARK/1 event=error code=qsp_replay_reject
test result: FAILED. 0 passed; 1 failed; … finished in 46.36s
```
Neuter: `RelaySeenIds::contains` → `false` + the lease backstop disabled; both reverted
(zero occurrences in the committed tree) and the test re-ran green before commit.

## §8 Files changed

Product: `src/dedup/mod.rs` (NEW), `src/transport/mod.rs`, `src/cmd/mod.rs`,
`src/main.rs`. Tests: `tests/NA_0644_ack_client.rs` (NEW), `tests/common/mod.rs`
(additive-only). Governance: this file, `tests/NA-0644_eng0040_ack_client_testplan.md`,
`docs/ops/IMPROVEMENT_LEDGER.md` (ENG-0040 closed; ENG-0042 + ENG-0043 filed),
`DECISIONS.md` (D-1267), `TRACEABILITY.md`, `NEXT_ACTIONS.md`,
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. NO qsl-server change (contract fixed at
`8e4ea278`), NO E2EE/ratchet/wire-message-semantic change, NO default flip, NO
`formal/`, vectors, canonical, `.github`.
