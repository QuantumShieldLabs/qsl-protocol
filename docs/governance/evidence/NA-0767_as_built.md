# NA-0767 — AS BUILT: the inviter-completion measurement lane

Bases: qsl-protocol main `94e50aa7512abee74dadb9f6b21bdcf89fdc194a`, qsl-desktop main
`11f695dfcb3e6c1f3b3ff78a14eee71e878b0439`, each re-derived bare and unpiped at the NAMED `github`
remote with all 40 digits compared and the comparator negative-controlled. **Measurement and filing
only — zero product source bytes, zero committed files edited, nothing repaired, no scan class added.**
Decision `D-1408`. ⚠ Every figure below was measured when this file was written, not carried from a draft.

## 1. THE RIG, AND BOTH ENVIRONMENT KNOBS

| item | value |
|---|---|
| `qsl-server` rev | `37ec82072cbbd68e4eaba83e192282fbcb96e5b4` — ⚠ the rev NA-0740 derived as the AWS box's; **NOT directly re-derivable** (the box carries no provenance file). Supporting evidence: the deployed binary `sha256 3439aa04ef0e1b9a69d7d27fae1fb021cd8a48982c68a028dd0562f22bc9ce72` is unchanged since 2026-08-11 02:05, pre-dating that derivation; `37ec8207` is dated 2026-07-29, is an ancestor of main, and main has 4 commits since it, all LICENSE/CI/records with **no transport change**. |
| `qsc` | built in this lane's own seat at protocol main `94e50aa7` |
| **`MAX_BODY_BYTES`** | **65536** — set explicitly, and ECHOED BACK by `/v1/server-info`, compared BY EQUALITY |
| **`PULL_LEASE_SECS`** | **60** — set explicitly, and PROVEN from the store's own `leased_until` column (route touched `1787858716`, `leased_until 1787858776`). ⚠ `/v1/server-info` does NOT advertise it (verified: no `pull_lease` key in the response). |
| run tags | `na0767-20260827T191901Z` (MISS run, preserved), `…192424Z` (A), `…192804Z` (B), `…193050Z` (C) |
| transport | **loopback, plain HTTP**, bearer generated locally from `/dev/urandom`, **zero secrets read, the AWS relay never contacted for traffic** |
| identities / mailboxes | FRESH vaults + `identity rotate` per run; VIRGIN run-tagged mailboxes; the store deleted between runs |

**Auth gate proven live before any measurement, BOTH ARMS:** `GET /v1/server-info` → **401**
unauthenticated, **200** with the bearer, the two bodies `cmp` rc=1. Re-proven on the fresh store
before runs B and C.

## 2. THE HARNESS — written from scratch; NO committed script edited or copied

| file | lines | role |
|---|---|---|
| `harness/env.sh` | 26 | the rig environment and both knobs |
| `harness/census.py` | 56 | read-only store census; complete routes/rows/unexplained accounting |
| `harness/parse_marker.py` | 25 | marker → `key=value` JSON, so every comparison is an EQUALITY on the extracted value |
| `harness/run_flow.sh` | 131 | the six ordered checkpoints, both peers observed at each |
| `harness/run_flowB.sh` | 105 | the discriminating arm — identical through X3, NO tick beat |
| `harness/run_flowC.sh` | 45 | the duty-cycle window: shipped finish-scan on a fixed beat, store sampled 1/s |
| `SEAL_001/002/003` | 40/24/24 | predictions, sealed 444 BEFORE their runs |

⚠ The store was read **read-only via SQLite URI mode**, never a `cp` (a `cp` of a live SQLite DB can
preserve an EMPTY file that `ls` and `sha256sum` both call healthy).

## 3. THE HANDSHAKE, ENUMERATED FROM THE ENGINE'S OWN EMISSION SITES

THREE messages; the two sides build their sessions at DIFFERENT ones.

| # | msg | sender → receiver | receiver's act | builds a session? |
|---|---|---|---|---|
| 1 | A1 (`HsInit`) | redeemer → inviter's invite SLOT | `invite_accept` → responder branch `handshake/mod.rs:2429`: `hs_pending_store` with `pending_session`, pushes B1 | **NO — pending only** |
| 2 | B1 (`HsResp`) | inviter → redeemer's inbox | `invite_finish` → initiator branch: **`qsp_session_store` `:1928`**, pushes A2 | **YES — the redeemer's** |
| 3 | A2 (`HsConfirm`) | redeemer → **inviter's ORDINARY inbox** | requires an act consuming an A2: responder branch **`qsp_session_store` `:2167`** | **YES — the inviter's, HERE AND NOWHERE ELSE** |

The module's ONLY two `qsp_session_store` sites are `:1928` and `:2167`. The only path reaching
`:2167` is `perform_handshake_poll_with_tokens` fed an A2 — reachable as `handshake poll`, and from
**nothing the desktop registers** (43 commands, 0 handshake verbs; positive control `invite`=7).

## 4. THE MEASURED TABLE — both peers, six checkpoints, every value by EQUALITY

The inviter reads `no_session / peer_confirmed=no / send_ready=no / send_ready_reason=no_session /
pinned=true` at X2, X3 and X4 — **the operator's field line reproduced exactly** — while the redeemer
reads `awaiting_peer_confirm / send_ready=yes` from X3 onward. Full table in `STOP_NA0767_001`.
**Store accounting complete at every checkpoint of every run: `unexplained_routes=0
unexplained_rows=0`, 14 accounting lines, no `*** UNEXPLAINED ***` marker anywhere.**

## 5. THE LEASE — the two-arm control and the duty cycle

**TWO-ARM CONTROL, arms differing ONLY in whether a tick beat preceded the poll:**

| | ARM A — tick, then poll | ARM B — no tick, same poll |
|---|---|---|
| tick marker | `invite_scan_summary … selected=none classes=handshake` | *(none)* |
| poll emitted | `handshake_recv msg=none ok=true` | `handshake_recv msg=A2 ok=true` → `session_store ok=true` → `handshake_complete role=responder peer_confirmed=yes` |
| poll rc | 0 | 0 |
| inviter after | `no_session` (UNCHANGED) | `established_recv_only`, peer_confirmed=yes |

**DUTY CYCLE (run C, 150 s, shipped finish-scan on a fixed 20 s beat, store sampled 1/s):**
```
samples 150   AVAILABLE 11   LEASED 139     available_fraction = 0.073
rows present throughout: 1   (the A2 NEVER consumed, NEVER acked)
leased_until: None -> 1787859146 -> 1787859211 -> 1787859277   (steps 65 s, 66 s)
```
Shipped beat: `TICK_DEFAULT="instant"`, `TICK_TEMPO.instant={b:20000,j:5000}` ⇒ **15–25 s** against a
**60 s** lease. ⚠ The 20 s beat was FIXED for sampler determinism; the shipped beat is jittered, so
the field window is wider than this one figure. The DIRECTION is robust — every beat in the shipped
range is well under the lease.

⛳ **The shipped marker stream already distinguishes the two cases:** beats that find the frame emit
`classes=handshake`; beats inside the lease emit an EMPTY class list. No surface shows either.

## 6. CLAIM BOUNDARY

CLI-on-loopback, **n=2** complete flows + one 150 s window + one preserved MISS run. **The GUI was
NOT driven** — `ENG-0226`: the desktop harness has no fixture relay, so no scenario can complete a
handshake; the two-real-app route (`QSLD_DATA_DIR` makes it mechanically possible) was priced and
judged a build, not a measurement. **Every claim here is an ENGINE claim, not a shipped-behaviour
claim.** The `connect_status` column is DERIVED via a source-proven equivalence (`qsp_status_tuple`
branches on the same `qsp_session_load` as `handshake_status`), not directly read — no CLI verb
exposes it. Happy path only: one invite, one redeemer, one relay; no adversarial case, no
concurrency, no restart, no expiry, no relay fault injection. **NOT a CI claim.**

## 7. THE RULING, AND WHAT IT CHANGED

`RULING_NA0767_20260827.md`, sha256 `a1bfa391d9bba36f28ea86e1109bb3756198331f50548c882c8ab0f3cfb75d1c`,
banked 444, all 64 digits verified with a negative control on the comparator and the immutability
control run on both arms.

**The Director re-measured every load-bearing structural claim from his own chair before ruling** —
the two `qsp_session_store` sites, the 43-command registry with zero handshake verbs, and the design
doc's sec 1.1 sentence — and reproduced them with **zero discrepancies**.

⚠⚠⚠ **THE ORIGIN IS A DEVIATION FROM A BANKED DESIGN.** `DESIGN_delivery_ladder_metronome_v2_20260825.md`
(sha256 `aba8e2a5f8c388d1c7ac850c7b94790365cc9749e92e1a40b63ff22d056b8c59`) sec 1.1 specifies
*"finish-scan + handshake-poll classes now"*. NA-0763's `R1` installed the finish-scan class ONLY.
Filed as SR-16 rows 356/357 against the Director's chair at his own instruction.

**Design direction ruled: DIRECTION A, no interim step.** The design of record is amended by
`AMENDMENT_1_delivery_ladder_20260827.md`, sha256
`440b101929601704babbe356c5730746a5af2404ce51081e97544dd5f3952feb`, banked 444 beside the design doc,
which is not edited. ⚠ **Cited, not transcribed — the banked file is the text.**

**The five asks:** Q1 two entries (upheld) · Q2 land this turn, records-only · Q3 the engine claim is
sufficient, the GUI harness is a standing want for the messaging epic · Q4 append to `ENG-0198`,
close nothing · Q5 the `established_recv_only` bound is in scope for the successor as a named bound.

## 8. TWO CORRECTIONS MEASURED BY THIS SEAT

⚠ **(a)** The ruling's sec 0 abbreviates the design doc's sha as `aba8e2a5...f088`. Measured
`aba8e2a5…056b8c59`; `f088` occurs nowhere in the digest and no file under `/srv/qbuild/operator/`
hashes to a value ending `f088`. The DOCUMENT is correctly identified (head8, filename and the quoted
sentence all match); only the abbreviation is defective. `AMENDMENT_1` carries the full digest and it
matches this measurement exactly. The full digest is used throughout; the defective one is not
propagated.

⚠ **(b)** The design doc's sec **1.2 (the lease law)** already states that a pulled-unacked item is
invisible until expiry and rules that the steady-state tick *"must not undercut the deployed lease
WHILE never-ack pull paths exist"*. That antecedent measures TRUE — the finish-scan's own contract
says a declined frame is *"not acked"*, and run C measured **zero** `producer_ack` markers across 8
beats — while the shipped beat is 15–25 s against a deployed 60 s lease. So the banked design
anticipated the lease MECHANISM even if not the DISPATCH shape. Reported, not ruled; `AMENDMENT_1`
governs the subject.
