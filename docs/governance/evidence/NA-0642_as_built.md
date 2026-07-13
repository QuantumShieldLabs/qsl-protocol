# NA-0642 As-Built — qsl-server Durability (D578, D-1265)

Goals: G4, G5
Status: Supporting evidence (lane closeout)
Owner: QSL governance
Last-Updated: 2026-07-13

## 1. What this lane shipped, and where

The first Tier-1 build lane of DOC-PROG-003 §5 (self-host operator-path, step
1; the durability half of the §4 production-relay split), executed under the
CORRECTED satellite governance model recorded in D578: qsl-server has no
queue/directive authority, so this qsl-protocol lane carried the governance
and the CODE landed in the satellite:

- **qsl-server PR #61** — branch `na-0642-durable-queue`, base = the pin
  `19b9b02dbe1f2ae9bc246ff3a16890e56c073c3e` (fresh clone; the stale qbuild
  mirror was not used), commit `ddde687`, **merge commit `8e4ea278`**, the
  single required `rust` check (fmt + cargo test + clippy -D warnings + two
  repo guard scripts) GREEN at merge; merge-commit per qsl-server CLAUDE.md.
  19 files, +2532/−534.
- **qsl-server DECISIONS D-0011** — the pull-contract decision, on the
  D-0009/D-0010 recorded-decision surface.
- This repo: governance closeout only (D-1265; this file; the testplan; the
  ledger filings ENG-0039/0040/0041; NEXT_ACTIONS/TRACEABILITY/journal).

Forbidden-surface proof: NO qsc/qsl-attachments/qsl-protocol SOURCE edit; NO
wire MESSAGE-SEMANTIC or E2EE change (payloads opaque; relay blind); NOT
per-client auth (ENG-0036), NOT TLS, NOT sealed-sender (ENG-0037); the qsc
dev-dep pin NOT bumped (ENG-0041 records the owed bump).

## 2. Design-lock → as-built (operator-approved 2026-07-13; zero deviations,
one additive detail)

| Locked | As built |
|---|---|
| Crate: rusqlite + `bundled` | `rusqlite = "0.32"` bundled; sha2 for route keys |
| Pragmas: WAL, synchronous=FULL, foreign_keys=ON | as locked (`src/store.rs::open`); a push 200 = fsynced |
| Schema v1: meta / routes(SHA-256 key) / messages(seq FIFO, msg_id NOT unique, BLOB body, enqueued_at, leased_until) | as locked; ADDITIVE detail: `routes.log_id` (the FNV-64 id already used in logs) so retention-expiry logging stays per-route and redacted — the raw token cannot be recovered from the SHA-256 key at sweep time |
| STORE_PATH required, no default, `:memory:` allowed | fail-closed `ERR_INVALID_CONFIG_STORE_PATH` (missing OR unopenable) |
| RETENTION_TTL_SECS default 604800, ceiling 2592000, zero rejected | as locked; lazy sweep on push/pull + 60s background task |
| PULL_LEASE_SECS default 60, ceiling 3600, zero rejected | as locked |
| Ack model B, OPT-IN per pull; legacy byte-identical | as locked (§3) |
| Sub-choices: token SHA-256 at rest; ROUTE_IDLE_TTL_MS warn-and-ignore; leased count against MAX_QUEUE_DEPTH | all as locked |

## 3. The wire contract as shipped (qsl-server D-0011)

- `GET /v1/pull?max=N` (no ack param): BYTE-IDENTICAL legacy contract —
  atomic delete-and-return, `{items:[{id,data}]}` with EXACTLY those fields
  (guarded by `na0642_backward_compat.rs` asserting the exact field sets),
  204 on empty. Today's non-acking qsc client drains unchanged.
- `GET /v1/pull?max=N&ack=lease`: returns WITHOUT deleting; each returned
  message leased until now+PULL_LEASE_SECS; in-flight messages invisible to
  BOTH modes while the lease is live. Unknown mode → 400 `ERR_BAD_ACK_MODE`.
- `POST /v1/pull/ack {"ids":[...]}`: deletes ONLY leased copies; idempotent;
  route-scoped; empty/oversized list → 400 `ERR_BAD_ACK_IDS`; malformed body
  → 400 `ERR_BAD_ACK_BODY`. Unleased duplicate copies (NA-0275 contract)
  survive an ack — proven by test.
- Retention: undelivered messages expire after RETENTION_TTL_SECS regardless
  of lease; delivered+acked messages deleted immediately — reliable, NOT an
  archive. Nothing precludes E2EE read receipts (opaque payloads through the
  same queue; the ENG-0040 successor surface).

## 4. Test evidence (all green at merge: 100 passed / 0 failed / 25 binaries;
fmt + clippy -D warnings + both guard scripts clean)

**Operator-required HARD-KILL proofs** (`tests/na0642_durability_restart.rs`;
the REAL compiled binary via CARGO_BIN_EXE; `Child::kill()` = SIGKILL on unix
— no graceful shutdown, no flush-on-exit; on-disk store in a temp dir):

1. `pushed_message_survives_hard_kill_and_restart` — 32 KiB payload; SIGKILL
   between the push-200 and any pull; restart on the same store: delivered
   byte-identical with the same msg id; subsequent pull 204. Passes ONLY
   because synchronous=FULL fsyncs before the 200.
2. `leased_message_survives_hard_kill_and_reappears_after_lease_expiry` —
   lease-pull then SIGKILL (the crash-between-pull-and-ack window); restart:
   the lease survived (immediate re-pull 204 — still in-flight); after expiry
   the message reappears byte-identical; ack deletes it; final pull 204.
3. `legacy_delivery_does_not_resurrect_after_restart` — negative control:
   the relay stays forgetful for delivered messages.

**Ack contract** (`na0642_ack_contract.rs`, 5): no-delete-until-ack +
in-flight invisibility + idempotent re-ack; redelivery after lease expiry;
leased-only deletion via the duplicate-msg_id case (ack before any pull
deletes NOTHING; after leasing copy 1 an ack kills only copy 1 — copy 2
delivered later); fail-closed inputs; cross-route ack = acked 0.

**Retention** (`na0642_retention_lifecycle.rs`, 7 + logging, 1): expiry with
TWO non-vacuity controls (default-TTL survival over the same sleep; the pull
path still delivers fresh messages post-expiry); the background sweep entry
point reports exact stats (expired_messages=2, one route removed); route-slot
release via lazy sweep; no resurrection; the NA-0281 drain-release contracts
carried forward verbatim (drain frees the route slot AND resets the rate
bucket); `event=retention_expired channel_id=<fnv> expired_messages= ttl_secs=`
with NO token/auth/payload leak.

**Backward compat** (`na0642_backward_compat.rs`): exact top-level and item
field sets on the legacy path; delete-on-deliver drain without ack.

**Concurrency — NEW coverage** (`na0642_concurrency.rs`, 2): 8 pushers × 25
messages + 4 concurrent legacy pullers → exactly-once (no loss, no duplicate,
set-equality with the pushed ids); 4 racing lease-pullers over 50 messages →
no message leased twice within the window; ack-all drains to 204.

**At-rest privacy** (`na0642_store_privacy.rs`): the raw route token is absent
from the db + wal + shm bytes; the payload-presence control proves the right
file and the verbatim-BLOB write path.

**Config** (`config_semantics.rs`, updated): STORE_PATH missing OR unopenable
→ `ERR_INVALID_CONFIG_STORE_PATH`; RETENTION_TTL_SECS / PULL_LEASE_SECS zero
or garbage → fail-closed; above-ceiling values clamp; ROUTE_IDLE_TTL_MS (any
value, incl. formerly-invalid) → warn-and-ignore, the relay starts.

## 5. Recorded-contract flips (deliberate, operator-ratified)

1. `config_semantics` zero-env start → NOW FAIL-CLOSED (STORE_PATH required;
   the approved durability posture — no silent in-memory fallback).
2. The NA-0281 `route_lifecycle_ttl.rs` + `route_lifecycle_ttl_logging.rs`
   idle-discard tests RETIRED with their contract; the drain-release
   contracts moved verbatim into `na0642_retention_lifecycle.rs`; the retired
   doc-currency test (`route_ttl_config_and_docs_are_explicit`) pinned the
   OLD truth — its README/DOC-SRV-003 subjects are now recorded drift in
   ENG-0039, not silently rewritten (README/DOC-SRV edits were out of D578
   scope).
3. The NA-0347 retention-purge block in
   `qsl_attachments_integration_contract.rs` now exercises the retention
   mechanism (same "purge is bounded" meaning).
4. `StateDirectory=qsl-server` added to the packaged unit — the one flagged
   scope judgment, ratified at merge: ProtectSystem=strict cannot write
   /var/lib/qsl-server without it, and templating the store under
   /opt/qsl-server (the binary dir) would be worse.

## 6. Debt rule execution + filings

- FIXED (the one): `relay.env.example` MAX_QUEUE_DEPTH 256→257 (NA-0598:
  exact-4-MiB attachment = 256 chunks + 1 manifest; the code ceiling was
  already 257 — only the template lagged).
- FILED: **ENG-0039** — the deferred hardening bundle (a)–(f) from the
  2026-07-13 review PLUS the README/DOC-SRV-003 drift this lane knowingly
  created (idle-TTL text now stale; new envs and ack mode undocumented in the
  DOC-SRVs).
- OWED FOLLOW-UPS RECORDED: **ENG-0040** (qsc ack-client lane — the honest
  durability statement: the server mechanism exists; full end-to-end delivery
  durability requires the client to opt in and ack; the current client keeps
  exactly its status-quo contract) and **ENG-0041** (dev-dep pin bump past
  `8e4ea278` + local NA-0640 e2e re-run; the in-process harness needs no
  change since the library constructors default to `:memory:`).

## 7. Result classification and limits

**QSL_SERVER_DURABILITY_PASS.** Durable store-and-forward under the TESTED
scenarios: SIGKILL restart durability; crash-between-pull-and-ack; retention
expiry with non-vacuity controls; backward compat with the non-acking client;
basic contention (exactly-once, no double-lease). NOT claimed: production
hardening in every respect — per-client auth/admission (ENG-0036), TLS,
constant-time bearer compare (ENG-0014/ENG-0039), metrics, signed releases
remain separate/filed. The denied scenarios were not faked: client-side
durability is ENG-0040, not a claim of this lane. Claim boundary UNCHANGED.
