# NA-0775 (`D-1418`) — AS BUILT: THE ENG-0269 REPAIR

Base: qsl-protocol main `6a5003713914bd6748b7b2a28d014d0b8fe41f53`, re-derived bare and unpiped at
the NAMED github remote. ⚠ The local mirror measured **two merges stale** (`2a4b159d`) at the
derivation; a seat trusting it would have based this lane two commits back.

## 1. WHAT SHIPPED

`perform_handshake_poll_with_tokens` returns a typed `PollOutcome`
(`Consumed | AlreadyComplete | NotConsumed`, `#[must_use]`) across all 21 Ok-shaped exits, instead
of a bare `Ok(())` that made the `ENG-0269` failure exit **byte-identical** to the success exit.
The A2 branch's session commit and pending clear move BELOW the push, behind a session-id-scoped
compare-and-set held under the tree's own cross-process `flock`. The two unparseable-suite-context
exits stop destroying the pending record. Both `invite` callers ack only on
`Consumed | AlreadyComplete`, and `invite_finish`'s bool follows.

**THE CONTRACT, MEASURED FROM THE BYTES AT THE SHIPPED TREE:**
    `return Ok(PollOutcome::…)` statements ... 20   (3 Consumed, 17 NotConsumed)
    the tail, two-armed .................... 1    (AlreadyComplete | NotConsumed)
    Ok-shaped exits ........................ 21
    explicit `return Err` .................. 2
    `?` propagation exits .................. 10
    EVERY EXIT ............................. 33
⚠ `AlreadyComplete` is reachable ONLY through the tail: both detection arms set a flag and
`continue`, so a `Relay` batch keeps being processed rather than returning on the first hit.

## 2. THE SR-05 RUN — THE FULL SUITE, ONCE, ON THE EXACT COMMITTED TREE

    SR-05 TREE : 7301c6d0d6fea893c4203cd7a27f1e86b09a4fda
    SR-05 DIRTY: 0
    STARTED  2026-09-01T17:48:57Z   FINISHED  2026-09-01T20:59:57Z   (3 h 11 m)
    CARGO_EXIT=0

**PR-7 CENSUS, RECONCILED BY UNIQUE BINARY PATH (the `NA-0771` lesson):**
    RUNNING lines (targets executed) : 152
    test-result lines                : 152
    UNIQUE binary PATHS              : 152
    unique target NAMES              : 152
    TOTALS: passed=720  failed=0  ignored=2  filtered_out=0
    NON-OK TARGETS: NONE
⚠ Unique NAMES also measured 152, so name-keying would NOT have lost a target on this run —
`NA-0771`'s trap (`lib` and `bin:qsc` both reported as `qsc`) did NOT fire here. Reported BECAUSE
it came back clean: a green run is not a complete run, and cardinality is the only thing that
tells them apart.

⚠⚠ **THE CODE TREE OF THIS PR IS BYTE-IDENTICAL TO THE TREE THAT RUN TESTED, AND IT IS PROVEN
RATHER THAN ASSERTED:** `git rev-parse <commit>:qsl` yields `f5faa810b3830a9350bea4d480e61596af002c05`
for BOTH `7301c6d0` and this branch's implementation commit.
⚠ **AND THE FIRST ATTEMPT TO BUILD THIS BRANCH FAILED THAT PROOF.** Cherry-picking the three code
commits onto the promotion base AUTO-MERGED and silently produced a different tree — 50 lines
missing from `handshake/mod.rs`, 31 from `invite/mod.rs`, **no conflict, exit 0**. Rebuilt by
checking the tested tree out verbatim. Without the subtree-hash comparison this PR would have
carried code no suite had ever run, citing a 3 h 11 m green as its evidence.

## 3. THE ARMS, AND EVERY RED DEMONSTRATED

| arm | green | red demonstrated |
|---|---|---|
| `t8` — the a2_sig exit emits no producer ack | ok, **test file UNTOUCHED** | product reverted to base → FAILED, reproducing `ENG-0269`'s marker sequence verbatim |
| `t9` — the failure RECOVERS on redelivery (new) | ok | same revert → FAILED |
| `t10a` — the guard refuses an epoch-0 store over an advanced session (new) | ok | — |
| `t10b` — **the permanent negative control** (new) | ok | *is* the red arm: unguarded, `ns` 7 → 0 |
| `t10c` — a different session_id still stores (new) | ok | — |
| `t5f` — a lost ack is retired on retry | ok, **UNTOUCHED** | `AlreadyComplete` mapped back to two-valued → FAILED |
| `t5p` — the poll's orphan stays an orphan | ok, **UNTOUCHED** | — (pins the other half of the caller asymmetry) |
| `handshake_mvp` a2 replay | ok, **UNTOUCHED** | — (it *was* the red that found `E-6`) |
| `na0771_g_clear_sites_are_three_and_named` | ok | — (updated 4→3 by ruling; `a1`–`a4` still green) |

⚠ **ONE RED ARM NOT INDEPENDENTLY DRIVEN, STATED RATHER THAN GLOSSED:** `t9`'s `session_store`
ABSENCE assertion. On the unrepaired tree `t9` fails at the producer-ack assertion first, so the
absence assertion is never reached. The unrepaired capture DOES print `event=session_store ok=true`
on that run, so it would have fired — evidence from a recorded run, but not the same as driving it.

⛳ **`t10a/b/c` ARE NOT SEAM-GATED** and ran inside the SR-05 suite's own lib target, so the
late-landing guard's proof — including its permanent negative control — runs on every required
check. **`t8` and `t9` do not:** no required CI job compiles `--cfg qsc_rng_failure_test_seam`, so
their green lives in this document and not on the board. `WF-0093` is the named successor and is
not built here.

## 4. THE SWEEP `RULING_009` ORDERED — SEALED BEFORE THE EDIT

241 raw candidate assertions narrowed to **15 sites** in the redelivery/replay window, each given a
predicted verdict BEFORE candidate B was written (`EXPECTATION_sweep_candidateB.md`, sha256
`fa2c366404870187fcfd7d513af3c771645bddeb01dfaaf4ef1092c15a5d72c8`).
**Predicted: zero reds. Measured: 15/15 targets, 78 tests, 0 failures.**
⚠ The prediction named its own likeliest error in advance — site 4/5, `t5p`, whose comment PINS the
orphan as permanent. It held, because the ack consequence is **asymmetric by caller**: under
`HsPollSource::Relay` the in-poll acks sit at the `Consumed` exits only and
`handshake_poll_with_tokens` discards the outcome, so a poll emits the new marker and acks NOTHING;
under `Provided` the caller acks on `Consumed | AlreadyComplete`. `t5p` and `t5f` pin opposite
halves of that and are both green.

## 5. WHAT THIS LANE DOES NOT REPAIR

`ENG-0282` (a corrupt RESPONDER pending cannot be overwritten by `invite accept`, because that
store sits behind the no-pending branch — true of the tree today, surfaced because `ENG-0281`'s cure
lets the record survive); `ENG-0283` (`qsc send` holds the store lock ACROSS a network call,
`transport/mod.rs:3748` → `:4005`, no drop between); `ENG-0277`'s four dark `debug_assert`s, whose
drifted cite is corrected beside the entry; `ENG-0267`, whose window this lane NARROWS and does not
close; and the replay guard's unreachability under `LegacyCompat`, which makes a replay and a
"no context to decode against" miss print the same reason — pre-existing, recorded by `NA-0711`,
owned by the reject-vocabulary lane `NA-0708` filed.

## 6. BOUNDS HELD

Zero `.github/**` bytes. Zero dependency changes. No seam-mechanism change. **No test was edited,
weakened, skipped or deleted at any point in this lane** — the only test-file changes are the
ADDITION of `t9`, the ADDITION of the `t10` module, and `NA-0771`'s seal update, which
`RULING_009` sec 2 ordered by name. `qwork`/`qstart`/`qresume`/`qnext` not run; no `sudo`; nothing
merged by the seat.
⚠ The shard manifest needs no update: the census truth (`tests/*.rs` depth 1 + the three fixed
targets) is unchanged — this lane adds no new test TARGET.
⚠ `cargo fmt --check` reports diffs in this file, and in **all 76** qsc source and test files at
BASE as well; the repo is not rustfmt-clean and CI has no fmt gate. Running `cargo fmt` would
reformat pre-existing code this lane has no business touching. Left as-is and recorded.
