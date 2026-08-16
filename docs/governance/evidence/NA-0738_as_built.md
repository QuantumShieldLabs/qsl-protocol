# NA-0738 — AS BUILT: THE ONE-SESSION ROUND-TRIP EXPERIMENT

**Lane:** NA-0738. **Decision:** D-1373. **Base:** main `62752adfae34dbfc667ddb6e822085029ceed305`.
**Directive:** the Director's formalization brief of 2026-08-16, banked verbatim under SR-14 before
anything consumed it — `BRIEF_NA0738_ONE_SESSION_ROUND_TRIP_20260816T015037Z.md`, sha256
`8ab993b93b660517ee95b7b6d2d2eb2c0178b91434cab188ecedd8d8ba8a43f3`, 149 lines, mode 444.

⚠ **This document exists partly so that the SR-15 cold read's substance stops living only under
`/srv`** (D-1 / R331.1). It carries the read's theorem, its argument, and its corrections into repo
truth, cited by sha.

---

## 1. PREMISES THE SEAT OWNS — MEASURED AND REPORTED BEFORE RUNNING

| # | premise | measurement |
|---|---|---|
| a | main UNMOVED, **bare and unpiped**, against the **NAMED** github remote | `git ls-remote github refs/heads/main` rc 0, exactly one line, `62752adf…d305` — equal to the brief's stated base. The seat's `origin` is a local path and was **not** used for this. Open-PR set `[]`, **MEASURED EMPTY**, re-measured at the edit |
| b | the committed script's sha256 and line count **at this base**, re-measured not carried | `c885dcf09033cce082290a6856d6835b6dddbebfc90eb4de816bf7fcd9670eef`, **482 lines**, 18319 bytes; git blob `0fa7f8cfd65bb7692c7e41c78fe5505b1a830470` as an independent route. ⚠ This equals one of the two figures the brief said not to carry — correctly, because that figure is the **post-ENG-0192-repair** script and #1752 has merged. It is reported as re-measured, not as agreeing |
| c | the cold read's findings file: sha256 and mode verified **BEFORE** reading | `a5b7324e68146dc2f859fcfd3a63ca3951ca8d3cdaf13bdd4724307901a3b1b4`, mode **444**, 534 lines, 35145 bytes — hashed and stat'd before the file was opened, and **re-verified unchanged after** |
| d | the harness: technique re-used, output re-derived; every anchor asserted against `cat -A` bytes; the anchor gate proven able to FAIL | see §3 |
| e | loopback per NA-0737's Door A: rev, locally-generated bearer, zero secrets read, environment delta reported | see §4 |

---

## 2. THE HYPOTHESIS UNDER TEST — ⚠ THE COLD READ'S, NOT THE DIRECTOR'S AND NOT THE SEAT'S

> **A peer reads `established` ⟺ it has ORIGINATED a user send AND RECEIVED a message on the
> CURRENT session. Both peers read `established` ⟺ a complete bidirectional user round trip has
> occurred on ONE session, with no handshake between the halves.**

Derived in the read's §2 from five sourced links, each re-verified at this base by this seat:

- **L1 — the predicate.** `qsl/qsl-client/qsc/src/handshake/mod.rs:1302-1310`: branch 1 returns
  `established_recv_only` when the send chain is unkeyed (`hs_send_ready_from_session`, `:1298`);
  branch 2 returns `awaiting_peer_confirm` when `st.recv.nr == 0`; branch 3 returns `established`.
  ⇒ **`established` ⟺ send chain KEYED **and** `recv.nr != 0`**, and branch 1 short-circuits, so a
  peer with an unkeyed send chain reads `established_recv_only` **no matter how much it has
  received**.
- **L2 — a handshake REPLACES the session; it does not mutate one.** `hs_build_session` (`:1129`)
  builds a fresh state; both stores are unconditional `qsp_session_store(peer, &st)` (`:1883`,
  `:2098`), one blob per peer, with no existing-session check on the init path.
- **L3 — the handshake keys the INITIATOR's send chain, not the responder's.** Measured in both
  roles across every run in this lane.
- **L4 — receiving advances `recv.nr` and explicitly leaves the send chain untouched** (refimpl
  `suite2/ratchet.rs`, *"Commit receive + DH state (send chain untouched)"*).
- **L5 — only a peer's own first USER send seeds its send chain.** `qsc/src/lib.rs:1898`,
  `chain_unseeded && !boundary_permitted`, with `boundary_permitted = origination.may_originate()`.
  ⛳ **Observed directly in this lane** as `event=qsp_dh_ratchet dir=send reason=first_send` in
  `bob.log`, immediately before bob's status flips to `established`.

**Expectations were sealed BEFORE the run**, at `evidence/SEAL_RUN_EXPECTATIONS.md`
(`fc5851f3455cbc00343961aa2d4eae76dda6cf1396bb4ded0b8500a1a9256a3d`, 103 lines, 444) and
`evidence/SEAL_RUN2_EXPECTATIONS.md`
(`c1761f61ed07fc13a0ed54259fa52099ad9d309c7a404bba3ce67505331af186`, 91 lines, 444).

---

## 3. THE INSTRUMENT

**The arrangement** — the single structural difference from every prior run is that the
re-handshake does **not** sit between the two halves of the round trip:

    hs1 (alice initiator, as committed)   -> X0
    alice -> bob  send                    -> X1
    bob   receive                         -> X2
    bob   -> alice send                   -> X3   <- the responder's FIRST USER SEND
    alice receive                         -> X4   <- THE DATUM
    hs2 RELOCATED here (verbatim bytes)   -> X5   <- ENG-0143's owed RE-HANDSHAKE row

`hs2` is **RELOCATED, not omitted** (the brief permits either and requires the choice be stated):
relocating costs nothing, preserves the re-handshake coverage ENG-0143 asks any predicate-shipping
lane to owe, and is the only variant that yields an X5 at all.

| artifact | sha256 | lines |
|---|---|---|
| builder `evidence/build_harness_na0738.py` | `162789d31646ff7e5c67fe447b2eb09bbb2a5dc2e3fe052f942a7c0797fba689` | 270 |
| harness `evidence/harness_na0738.sh` | `68032fa7c336010ebedb7f226b3d1261b9fbe176235d24617ec38d24da4a2810` | 595 |
| control `evidence/harness_CONTROL_na0737shape.sh` | `f52a63e2d18576c0c24271cbc89429d994ea06542e6cfbbcb0f9dce58e3edc07` | 538 |

⛳ **The control was RE-DERIVED, not written:** it was produced by executing **NA-0737's own banked
builder** against this base, and `cmp` against NA-0737's banked `harness_AFTER.sh` returns **rc 0** —
the control **is** NA-0737's instrument, bit for bit.

**Anchors.** Twelve, each asserted PRESENT and UNIQUE against the input's own bytes before any
mutation, each needle built from `cat -A` output rather than from a model of the text, and each
prefix anchor carrying a **trailing space** (NA-0737's NC-1 caught that a prefix without one also
matches a drifted step name). Sealed line numbers, all HIT: A1 300 · A2 351 · A3 352 · A4 372 ·
A5 375 · A6a 377 · A6b 378 · A6c 379 · A6d 380 · A6e 381 · A7 385 · A8 388. Two further
**structural** assertions guard the block being MOVED: the five `hs2` lines must be consecutive and
followed by a blank separator, and the block must sit strictly between bob's receive and bob's send.

**⚠ THE ANCHOR GATE WAS PROVEN ABLE TO FAIL — four tamper controls, each non-vacuous first.**
Every tampered input was `cmp`-ed against the original and asserted to DIFFER before either outcome
was trusted (a tampered file that is byte-identical makes its control vacuous, and that is the
finding rather than the result it prints):

| control | tamper | result |
|---|---|---|
| NC-1 | step name drifted to `recv_from_alice_TAMPERED_X` | **rc 1** — `ANCHOR FAILURE [A5]: expected exactly 1 match, found 0` |
| NC-2 | an exact anchor duplicated | **rc 1** — `ANCHOR FAILURE [A2]: … found 2 at 1-based lines [351, 352]` |
| NC-3 | a blank line inserted inside the `hs2` block | **rc 1** — `ANCHOR FAILURE [A6 block]: the five hs2 lines are not consecutive: [377, 378, 379, 381, 382]` |
| NC-4 | builder re-run on its **own output** (idempotence refusal) | **rc 1** — `ANCHOR FAILURE [A2]: … found 0` |

**Every status comparison in the harness is BY EQUALITY on the EXTRACTED value.** The harness also
records, for the same bytes, what the COMMITTED substring needle would do — so the 187-day prefix
hazard is **measured live** rather than described.

---

## 4. THE LOOPBACK DOOR

- `qsl-server` at rev **`37ec82072cbbd68e4eaba83e192282fbcb96e5b4`** — the rev NA-0737 measured as
  the AWS box's. ⚠ **This seat did NOT re-measure the AWS box's rev**; doing so requires the relay
  bearer, which the brief forbids reading. Carried with attribution.
- Bearer **generated locally** from `/dev/urandom`, 32 bytes, mode 0600, never printed. **Zero
  secrets read.** `relay.env` was not opened.
- **Auth gate proven live before any run and again on the restarted process:** `/v1/server-info`
  **401** unauthenticated, **200** with the bearer.
- **Environment delta, deliberately REMOVED rather than merely reported:** `MAX_BODY_BYTES=65536`,
  matching the AWS box (NA-0737 measured 1048576 loopback vs 65536 AWS). All frames in this suite
  measure 4279 / 6436 / 3364 / 1024 bytes, far below both bounds, so matching removes a delta
  without imposing a new constraint. Advertised api set
  `["push_v1","pull_v1","pull_ack_lease_v1","invite_v1"]`.
- Plain **HTTP**, no TLS, `RELAY_CA_PEM` not involved. ⇒ **this is not a test of the CI transport
  path.**

---

## 5. THE OBSERVATIONS

### 5.1 The relocated arrangement — runs X2 and X3, **byte-identical** (n=2)

| CP | peer | status (compared by EQUALITY) | send_ready | send_ready_reason | peer_confirmed | == `established`? |
|---|---|---|---|---|---|---|
| X0 | alice | `awaiting_peer_confirm` | yes | *(absent)* | no | no |
| X0 | bob | `established_recv_only` | no | `chainkey_unset` | yes | no |
| X1 | alice | `awaiting_peer_confirm` | yes | *(absent)* | no | no |
| X1 | bob | `established_recv_only` | no | `chainkey_unset` | yes | no |
| X2 | alice | `awaiting_peer_confirm` | yes | *(absent)* | no | no |
| X2 | bob | `established_recv_only` | no | `chainkey_unset` | yes | no |
| X3 | alice | `awaiting_peer_confirm` | yes | *(absent)* | no | no |
| **X3** | **bob** | ⛳ **`established`** | **yes** | *(absent)* | yes | **YES** |
| **X4** | **alice** | ⛳ **`established`** | **yes** | *(absent)* | yes | **YES** |
| **X4** | **bob** | ⛳ **`established`** | **yes** | *(absent)* | yes | **YES** |
| X5 | alice | `established_recv_only` | no | `chainkey_unset` | yes | no |
| X5 | bob | `awaiting_peer_confirm` | yes | *(absent)* | no | no |

Step outcomes all rc 0. Script exit **0**. Payloads `cmp` rc 0 both directions.
`summary.txt`: `status=pass`, `qsp_unpack_ok=true both_directions`, `recv_commit_bob=1`,
`recv_commit_alice=1`.

### 5.2 The control — NA-0737's arrangement, identical environment

| CP | alice | bob |
|---|---|---|
| C0 / C1 / C2 | `awaiting_peer_confirm` `send_ready=yes` | `established_recv_only` `send_ready=no` `chainkey_unset` |
| C3 / C4 / C5 | `established_recv_only` `send_ready=no` `chainkey_unset` | `awaiting_peer_confirm` `send_ready=yes` |

**`established` by EQUALITY: 0 / 12.** Script exit 0 — delivery worked in this arm too. NA-0737's
headline reproduced exactly, C3 swap included.

### 5.3 ⇒ THE A/B

| arrangement | `established` | script exit |
|---|---|---|
| committed (`hs2` between the halves) | **0 / 12** | 0 |
| relocated (`hs2` after the round trip) | **3 / 12**, both peers at X4 | 0 |

Same relay process, same store, same `qsc` binary, same base, same scenario and seed.
**The only variable is where the re-handshake sits.**

### 5.4 The prefix hazard, measured live rather than described

At X0 the harness records both verdicts on the same bytes:

- alice — equality `EQ_NOT_ESTABLISHED`, substring `SUBSTR_WOULD_EXIT_1`
- **bob — equality `EQ_NOT_ESTABLISHED`, substring `SUBSTR_WOULD_PASS`**

⇒ the committed needle at `:352` **passes for bob on `established_recv_only`** and fails only for
alice. **That is why 187 days of artifacts named alice alone.** See **ENG-0194**.

---

## 6. THE FIRST RUN — A MISS, ITS CAUSE, AND WHY IT IS EVIDENCE RATHER THAN A DISCARD

Run X1 of the relocated arrangement **FAILED**: `recv_from_bob` rc **1**,
`event=qsp_unpack code=qsp_env_decode_failed`, bob's message never reached, script exit 1.
It is preserved at 444 alongside the successful runs. The cause, measured to a source line and a
store row:

1. **`handshake poll` NEVER ACKS.** `event=relay_ack`: **0** in `alice.log`, **0** in `bob.log`,
   **1** in `bob_recv.log`. Across all four runs **every** handshake frame is still resident in the
   relay store at run end. Complete accounting from a read-only query of the store: **8 routes,
   31 residual rows, 0 unexplained by the candidate route tokens.**
2. Those frames therefore survive on a **visibility timeout** alone —
   `PULL_LEASE_SECS_DEFAULT = 60` (`qsl-server/src/store.rs:7`; ceiling 3600 at `:8`).
3. hs1's B1 (6436 B) was enqueued on alice's route at **21:00:36**; her `receive` ran at
   **21:01:55 — 79 s later**, past the timeout ⇒ **the stale handshake frame was redelivered at the
   HEAD of her queue.**
4. `receive` pulled it, could not decode it as a QSP envelope, and
   **`qsc/src/transport/mod.rs:1249` `return Err(CliError::code(code))` aborted the ENTIRE receive.**
   Only `qsp_replay_reject` has a quarantine-and-continue arm; **every other code aborts**, the item
   is never acked, and it is redelivered — permanently at the head.

⇒ **A HEAD-OF-LINE BLOCK: one undecodable frame at the head of a mailbox blocks every message behind
it.** ⚠ It is **product behaviour, not fixture behaviour**, and it is **latent on the committed
arrangement too** — it fires whenever a `receive` runs more than the visibility timeout after a
handshake frame was last pulled; the committed order masks it because `hs2`'s polls re-lease
immediately beforehand. ⚠ Recorded here and **deliberately NOT filed as a ledger entry**: the brief
did not enumerate it, and prioritising the backlog is the operator's act.

⚠⚠ **AND PART OF THE CONFOUND WAS THIS LANE'S OWN INSTRUMENT.** The harness adds **12 `qsc`
invocations** (6 checkpoints × 2 peers), each a debug-build process with a passphrase-KDF vault
unlock; run X1's enqueue span measured **107 s** against the 60 s timeout. **The observer's cost was
of the same order as the timeout it caused to expire.**

**Run X2 therefore changed exactly one named environment value** — `PULL_LEASE_SECS=3600`, the
server's own ceiling — with the harness **byte-identical** (sha re-verified before the run).
R332.1's three conditions were applied to that re-run rather than around it: (a) the failure **names
an external input** (mailbox visibility state); (b) its value **measurably changed**, 60 → 3600;
(c) the run **re-reads it every execution**. **All three hold measurably.** ⚠ Because
`/v1/server-info` does **not** advertise the lease, the intervention's effect was proven from the
store's own `leased_until` column: run-2 rows sit **≈3550 s** ahead of the run, against expired
leases on run-1's rows. **A vacuous intervention would have been the finding, not the result it
printed.**

---

## 7. ⚠⚠ THE SEAT'S OWN SEAL WAS DEFECTIVE

Run X1's sealed decisive combination read: *"X0 reproduces AND X4 reads anything else ⇒ the theorem
is REFUTED."* **Read mechanically, that would have declared REFUTED a theorem the very next run
confirmed** — because X4's own antecedent had failed: alice never received. The combination assumed
the delivery it was measuring, and had no branch for the antecedent failing.

The only reason no false refutation was produced is that the harness independently captured each
step's rc. ⇒ **a sealed expectation can be wrong in its CONDITIONS as well as in its VALUES, and
only a second, independent record of the antecedent tells you which.** The corrected combination,
sealed before run X2, carries the antecedent as its own explicit branch. Recorded as SR-16 row 26.

---

## 8. THE COLD READ'S SUBSTANCE, CARRIED INTO REPO TRUTH

`FINDINGS_SR15_ENG0191_COLD_READ.md`, sha256
`a5b7324e68146dc2f859fcfd3a63ca3951ca8d3cdaf13bdd4724307901a3b1b4`, 534 lines, mode 444. Its
holdings, so they do not live only under `/srv`:

1. **§1 — the option set is incomplete on two independent axes.** It is closed under delete / add /
   delete and contains **no MOVE**, while the defect is an ORDERING defect ⇒ **(d)**. And it had
   silently dropped ENG-0191's own filed retarget option ⇒ **(e)**, *lost rather than refused*.
2. **§2 — the theorem** (carried in §2 above), which predicted **all 12** of NA-0737's values and
   then **X0–X4** of this lane's exactly, including a transition nobody had observed.
3. **§3 — what each option stops proving**, including that `:351`/`:352` are the **sole gate** on
   the fabricated markers at `:353-355` and the hard-coded literals at `:448`/`:449`/`:468`, so
   **(a) as written is not a one-line deletion**.
4. **§4 — `hs2` is the only re-handshake exercised against a real relay**, but only in its **benign**
   configuration; the coverage it provides is **structural**, not triggering, for ENG-0143/ENG-0147.
   Saying otherwise overclaims.
5. **§5 — the Director's elimination is REFUTED**, by three independent arguments (see D-1373 and
   SR-16 row 23).
6. **§6 — (b) works, and it is a DERIVATION, not a proof** — ⚠ and it is **direction-critical**:
   after `hs2`, an extra alice→bob exchange establishes both peers; an extra bob→alice exchange
   **changes nothing**, and the option text does not say which.
7. **§7 — what `established` buys that delivery assertions do not:** it is the only line in the file
   that can distinguish *a live bidirectional session* from *two one-way sessions in sequence* —
   which is exactly what the suite does today.
8. **§8.1 — the "product gap" reading is not supported**: four in-tree fixtures pin the responder's
   post-handshake state deliberately. ⛳ **This lane closes it by measurement.**
9. **§8.2 — the inert unit assertion** ⇒ filed here as **ENG-0194**. ⚠ **Its enumeration measured
   short by one** — four substring consumers tree-wide, not three; the missing one is
   `qsl/qsl-client/qsc/scripts/remote_soak.py:573`. **The correction strengthens the finding.**
10. **§9 — the apparent convergence on (a) is an artefact of measurement selection**, because only
    one arrangement had ever been run. ⛳ **This lane is the test of that claim, and it holds.**

---

## 9. BOUNDS OBSERVED

- **Zero product source bytes.** The committed script is re-verified **byte-identical after all four
  runs** — `c885dcf0…0eef`, 482 lines, git blob `0fa7f8cf…`. The working tree is clean.
- No `.github/**`, no workflow, no dependency, no lock change. **No test weakened, skipped or
  deleted.** No standing rule minted. No fenced ruling edited. `## D-1372` not rewritten.
  ENG-0191's and ENG-0143's existing text added **beside**, never rewritten.
- **NO OPTION RULED. ENG-0191 NOT repaired. ENG-0194's inert assertion NOT repaired. WF-0086's gate
  NOT built. The pull path (ENG-0193) NOT instrumented. #1745 NOT closed.**
- Predecessor sealed evidence: **shas verified only.** All seven NA-0735/NA-0737 stop files verify
  against the shas recorded in their lanes' `LATEST.md`, all mode 444, none modified. The cold read's
  findings file sha was verified **before** reading and **re-verified unchanged after**.
- No secret read. `relay.env` not opened. No sudo. No `qwork`/`qstart`/`qresume`/`qnext`. Nothing
  merged.
- **Network contact, disclosed:** `git ls-remote` / `git fetch` against the named GitHub remote;
  `gh pr list` and `gh api` for the open-PR set and for the public `qsl-desktop` sources re-measured
  for **ENG-0195**. **No QSL relay was contacted; the only relay in this lane is a loopback process
  this seat started and stopped.**
