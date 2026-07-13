# NA-0642 Testplan — qsl-server Durability (D578, D-1265)

Goals: G4, G5
Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-13

## Objective

CROSS-REPO lane: the tests live in the qsl-server repo (merged via PR #61,
merge `8e4ea278`; required `rust` check green at merge) — this plan records
the matrix the qsl-protocol closeout relies on. Server-side durability only;
the qsc client is untouched (ENG-0040 is the owed client lane).

## Validation matrix (all executed in qsl-server at commit `ddde687`; suite
totals: 100 passed / 0 failed across 25 test binaries; cargo fmt --check,
cargo clippy -q -- -D warnings, and both repo CI guard scripts clean)

| # | Check | Method | Result |
|---|-------|--------|--------|
| 1 | Restart durability under HARD KILL (operator-required form) | real binary, SIGKILL (`Child::kill`) between the push-200 and restart; same on-disk store; legacy pull after restart | PASS — 32 KiB payload byte-identical, same msg id; provable only via synchronous=FULL |
| 2 | Crash between pull and ack under HARD KILL | lease-pull, SIGKILL mid-lease, restart | PASS — lease survives restart (immediate re-pull 204), message reappears byte-identical after expiry, ack deletes, final 204 |
| 3 | Delivered messages stay forgotten (negative control) | legacy-deliver, SIGKILL, restart, pull | PASS — 204, no resurrection |
| 4 | Delete-on-acknowledged-pull | lease-pull → no delete; in-flight invisible to both modes; ack → delete; re-ack idempotent (acked=0) | PASS (5 tests) |
| 5 | Leased-only ack scope | ack before any pull deletes nothing; duplicate msg_id (NA-0275): ack kills only the leased copy, the unleased copy still delivers; cross-route ack = 0 | PASS |
| 6 | Redelivery on lost ack | 1s lease, no ack, re-pull after expiry | PASS — same message, byte-identical |
| 7 | Retention-TTL expiry | 1s TTL: push → sleep → pull 204; sweep entry point reports expired=2/route removed; route slot released | PASS |
| 8 | Retention non-vacuity (2 controls) | default TTL survives the same sleep; post-expiry delivery still works on the same server | PASS — expiry is retention, not broken delivery |
| 9 | Backward compat with the non-acking client | legacy pull: exact top-level field set `{items}` and item field set `{id,data}`; delete-on-deliver drain without ack | PASS — byte-identical contract; NA-0640 e2e unaffected at pin-bump time |
| 10 | Concurrency/contention (previously untested) | 8 pushers × 25 + 4 legacy pullers: exactly-once set equality; 4 racing lease-pullers over 50: no double-lease; ack-all drains | PASS |
| 11 | At-rest token privacy | raw db+wal+shm bytes: route token ABSENT; payload PRESENT (right-file control) | PASS — SHA-256 keying holds on disk |
| 12 | Config fail-closed | STORE_PATH missing/unopenable → ERR_INVALID_CONFIG_STORE_PATH; RETENTION_TTL_SECS / PULL_LEASE_SECS 0/garbage → fail-closed; ceilings clamp | PASS |
| 13 | ROUTE_IDLE_TTL_MS retirement | any value (incl. formerly-invalid) → warn-and-ignore, relay starts | PASS — a stale relay.env cannot brick a restart |
| 14 | Fail-closed ack inputs | `?ack=bogus` → ERR_BAD_ACK_MODE; bad JSON → ERR_BAD_ACK_BODY; empty/oversized ids → ERR_BAD_ACK_IDS; missing route header → ERR_MISSING_ROUTE_TOKEN | PASS |
| 15 | Log redaction on the new event | `event=retention_expired` carries fnv channel_id/count/ttl_secs; token, auth, payload absent | PASS |
| 16 | Pre-existing suites | all 17 pre-existing test binaries (smoke, relay_smoke, idempotency, abuse/rate, na0349, na0598, hardening/logging, no_secrets) | PASS unchanged (na0347's retired idle-TTL block moved to retention, recorded flip) |

## Recorded-contract flips (deliberate; operator-ratified at merge)

- `config_semantics` zero-env start → fail-closed (STORE_PATH required).
- NA-0281 `route_lifecycle_ttl*` retired with the idle-discard contract;
  drain-release contracts carried forward verbatim.
- NA-0347 purge block → retention mechanism.

## Out of scope (recorded, not tested here)

Client-side ack behavior (ENG-0040); the pin bump + NA-0640 e2e re-run
(ENG-0041); production hardening — auth (ENG-0036), TLS, constant-time compare
(ENG-0014), signed releases (ENG-0039). A PASS asserts durability under the
tested scenarios only.
