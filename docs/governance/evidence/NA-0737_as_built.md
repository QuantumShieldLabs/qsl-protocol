# NA-0737 — AS BUILT — ENG-0192, THE FIXTURE ADDRESSING REPAIR

**Lane:** NA-0737 · **Decision:** D-1372 · **Ruling:** the Director's NA-0737 STOP 001 ruling of
2026-08-15 (§A–§E) · **Base:** main `ac5cf636b93d46a38692ab8f707c3c4cec83f64b`, verified UNMOVED by
`git ls-remote` against the **NAMED** GitHub remote, run **bare and unpiped**, rc 0, exactly one
line; open-PR set **measured EMPTY** at every measurement.
**Class:** offered, not declared — a Director act. Candidate: `REMOTE_RELAY_ROUND_TRIP_PROVEN_PASS`.
**Scope:** one script, **two argument values**, **+2/−2**, plus records. **Zero product source
bytes.** No `.github/**`, no workflow, no dependency, no lock, no test weakened, skipped or deleted,
no standing rule minted.

⚠ **ON REDACTION — THIS LANE PUBLISHES ITS PREIMAGES, AND THE DISCRIMINATOR MATTERS.** NA-0736
redacted its route tokens because they addressed **live mailboxes on the AWS relay**; R336 §A then
established that *a hash is not checkable without its preimage*, so a redaction buys the appearance
of proof while destroying it. **This lane's tokens address an ephemeral local SQLite store that was
destroyed at the end of the run**, so there is no capability to withhold and full recomputability is
available. Publishing them therefore costs nothing and buys checkability.
⚠ This **sidesteps rather than answers** R336 §C's open question (are route tokens CAPABILITIES or
ADDRESSES?) — that remains unmeasured and is not pre-answered here.

---

## 1. THE QUESTION AND THE ANSWER

**Question (the brief's §0):** does pointing `receive` at the mailbox the sender actually uses make
the round trip work — **and is there a sixth cause behind this suite, or not?**

**Answer: it works, and there is NO SIXTH CAUSE.** With the two values corrected, a message crossed
a relay end to end in **both directions**, was unpacked, committed and acked, and the payloads
arrived **byte-identical**. Everything downstream of the addressing passed on the first attempt.

⚠⚠ **AND THE READER-TRAP THIS RECORD MUST DEFUSE, IN ITS OWN WORDS (D-1370's ENG-0189 precedent):
THIS LANE DOES NOT TURN CI GREEN AND WAS NEVER EXPECTED TO. The committed script still fails at
`:351` — ENG-0191's assertion. A STILL-RED SUITE DOES NOT MEAN THIS REPAIR FAILED, and it does not
reopen ENG-0192. Issue #1745 stays OPEN and is correctly open.**

---

## 2. THE EDIT — TWO VALUES

    scripts/demo/qsc_remote_handshake_smoke.sh
    :375  receive … --mailbox "$proto_bob"    →  --mailbox "$bob_route_token"
    :388  receive … --mailbox "$proto_alice"  →  --mailbox "$alice_route_token"

    sha256 275a3e29…2491 (482 lines)  →  c885dcf0…0eef (482 lines)
    git diff --stat: 1 file changed, 2 insertions(+), 2 deletions(-)

Both variables **already existed** (`:61`, `:62`) and were **already used correctly** at
`relay inbox-set --token` (`:308`/`:310`) and `contacts add --route-token` (`:330`/`:334`).
**Nothing new was introduced; the correct value was on the same page the whole time.**

⚠ **`--from` IS CORRECT AND WAS NOT TOUCHED** — it takes a peer LABEL by design, resolved inside
`qsp_unpack_for_peer`. This was **verified, not merely intended**: the splice parses the `--from`
token out of each changed line before and after and asserts byte-identity. ⛳ **The run then proved
it right to leave alone** — both receipts resolved the peer label
(`QSC_RECEIPT … peer=alice-na0737-2-happy-path-1`, `… peer=bob-na0737-2-happy-path-1`).

The edit was applied by an **asserted splice**, not by hand: an idempotence guard (re-running
returns rc 1), a pre-state sha256 guard, a byte-exact uniqueness guard (`--mailbox` occurs **exactly
2** times in this file, both in `receive`), and a post-state guard that **reconstructs the original
from the result** to prove the diff is confined to the two values.
`bash -n` **rc 0**; `shellcheck -S error` **rc 0**, empty output — both run bare and unpiped.

---

## 3. THE INSTRUMENT — A HARNESS, BECAUSE THE COMMITTED SCRIPT CANNOT REACH THE EDIT SITES

At this pin the committed script **cannot execute `:375`/`:388` at all**: `:351`/`:352` assert
`status=established`, ENG-0191's unreachable assertion, and `set -e` kills the run there. The run
therefore used NA-0735's checkpoint-observation harness, which demotes **those two assertions only**.
⚠ **The committed script's `:351`/`:352` are untouched by this lane.**

**The harness was RE-DERIVED, not reused — and the re-derivation is proven exact.** A builder written
from scratch, applied to the base script, produces a file **byte-identical to NA-0735's banked
harness** (`ddaf4d2d…d729`, 538 lines): `cmp` **rc 0**. That is the validation — it *reproduces a
sealed instrument* rather than reusing it blind or inventing a similar one.

⚠ **ANCHOR-COUNT HONESTY.** NA-0735 reported **12** anchors; its builder is not banked, only its
output, so that decomposition cannot be reproduced by reading and is **not claimed**. This builder
uses **8** anchors. **The equivalence rests on the `cmp`, not on the count matching.** Reporting 12
would have been a number copied rather than measured.

**The 8 re-derived anchors** (each asserted PRESENT and UNIQUE, byte-exact, before any mutation;
the two this lane changes are marked ⚠): A1 `extract_identity_fp() {` `:300` · A2 `:351` alice
assert · A3 `:352` bob assert · A4 `:372` send_ab · ⚠ **A5 `:375` bob receive** · A6 `:381`
hs2_poll_3 · A7 `:385` send_ba · ⚠ **A8 `:388` alice receive**.

⚠⚠ **THE ANCHOR GATE WAS PROVEN ABLE TO FAIL — AND PROVING IT CAUGHT A DEFECT IN THIS LANE'S OWN
BUILDER.** The first negative control altered a step name to `recv_from_alice_TAMPERED_X`; the
builder **still matched it and returned rc 0**, because `startswith("run_qsc_step bob
recv_from_alice")` is also a prefix of the tampered line. **A drifted anchor would have been wrapped
silently — the exact "silent skip" the order forbade.** Cured by requiring a trailing space (a word
boundary). Re-measured after the cure, with every control asserted to differ from the original first
so none is vacuous:

| control | tamper | builder rc |
|---|---|---|
| validation | none (base script) | **0** — output byte-identical to the banked harness |
| NC-1 | step name altered | **1** — `ANCHOR FAILURE [A5] … found 0` |
| NC-2 | anchor duplicated | **1** — `ANCHOR FAILURE [A5] … found 2 at lines [375, 376]` |
| NC-3 | exact-match assert line altered | **1** — `ANCHOR FAILURE [A2] … found 0` |

⛳ **The BEFORE and AFTER harnesses differ in exactly the same two values and nothing else** — so the
repair is the only variable between the two runs.

---

## 4. THE ENVIRONMENT — LOOPBACK, ZERO SECRETS

`qsl-server` at rev **`37ec8207`**, the same rev the AWS box runs, built rc 0 (14.73 s); `qsc` built
from the base rc 0 (22.16 s). The server is **plain HTTP** via `axum::serve` ⇒ **no `RELAY_CA_PEM`
needed**. `RELAY_TOKEN` was **generated locally** and never left the box. **No secret was read**:
`/srv/…/caddy/relay.env` was not opened, no `ssh relay`, no sudo.

The bearer gate was proven live rather than assumed: `/v1/server-info` returns **401
unauthenticated / 200 with bearer**. Advertised api set
`["push_v1","pull_v1","pull_ack_lease_v1","invite_v1"]` — **identical to what NA-0735 measured on the
AWS box**. ⚠ One environment delta, stated rather than glossed: `max_body_bytes` is **1048576** here
vs **65536** on AWS.

The AWS door was measured and **not used**: TCP `:8443` OPEN, `/v1/server-info` **401**
unauthenticated, and this seat holds no `RELAY_TOKEN`. The two local routes to one were refused as
the "invent a path around it" the order forbids.

---

## 5. THE DELTA SYMBOL — `mailbox_hash`, AND THE INSTRUMENT VALIDATED BEFORE USE

`mailbox_hash` is emitted in `recv_start` (`qsl/qsl-client/qsc/src/transport/mod.rs:335`/`:337`/
`:342`) as `route_token_hash8` = `hex(sha512(token)[..4])` (`contacts/mod.rs:5-9`), lowercase
(`lib.rs:2362-2370`), over the **normalized** mailbox — and `normalize_route_token` is `trim()` plus
a validity check only (`adversarial/route.rs:30-37`), no rewriting.

**The hash instrument was validated before it was trusted:** an independent implementation reproduced
**all four** of NA-0735's sealed values exactly (`f4c89d20`, `f9fa4170`, `a53c4170`, `f20f7f9f`), and
a tampered preimage yielded a differing value, so the control is not vacuous.
⚠ **NA-0735's figures were used ONLY as a control on the instrument; none was carried as an
expectation.** This lane computed its own eight values from its own run tags and **sealed them before
any run**. All eight hex digits are compared — NA-0735's near-miss (`a53c4170` / `f9fa4170` sharing
a trailing `4170`) is why a four-digit compare is never enough.

**Two DISTINCT run tags** were used so the red control's frames could not confound the repaired run,
on **the same relay process and the same store**, so the repair is the only variable.

| run | run tag | peer | preimage | expected (sealed) | **measured** |
|---|---|---|---|---|---|
| BEFORE | `na0737-1-happy-path-1` | bob | `bob-na0737-1-happy-path-1` | `6a82f281` | **`6a82f281`** |
| BEFORE | `na0737-1-happy-path-1` | alice | `alice-na0737-1-happy-path-1` | `cfe0c8d7` | **`cfe0c8d7`** |
| AFTER | `na0737-2-happy-path-1` | bob | `route_token_bob_na0737-2-happy-path-1` | `bb36a580` | **`bb36a580`** |
| AFTER | `na0737-2-happy-path-1` | alice | `route_token_alice_na0737-2-happy-path-1` | `0148e064` | **`0148e064`** |

In the AFTER run the two LABEL hashes (`e698e4bd`, `0a744f14`) occur **0 times** in the respective
receive logs. ⛳ **This table alone settles that the repair took**, independently of what followed.

---

## 6. RED FIRST — THE CONTROL THIS ENVIRONMENT DID NOT HAVE

NA-0735's sealed red run exists but ran against AWS. Loopback had **no red control**, so one was
produced: the **unfixed** script, same harness delta, same relay.

    QSC_MARK/1 event=recv_start transport=relay mailbox=redacted mailbox_hash=6a82f281 from=<redacted> max=1
    QSC_MARK/1 event=recv_ack_mode mode=lease
    QSC_MARK/1 event=recv_none

⛳ **NA-0736's signature reproduced on a box with no AWS relay involved:** `recv_ack_mode` — emitted
**before** the pull — straight into `recv_none` **with nothing between**, at rc 0 ⇒ the per-item loop
never executed. `out_bob/` and `out_alice/` **0 entries**; `qsp_unpack` **0** in both polarities;
`recv_commit` **0**; `summary.txt` **0 bytes**; run exit **1** at `:395`.

---

## 7. THE RESULT — DELIVERED, BOTH DIRECTIONS

| observable | BEFORE (red control) | AFTER (repaired) |
|---|---|---|
| script exit code | **1** | **0** |
| `event=recv_none` | 2 | **0** |
| `event=qsp_unpack ok=true` | 0 | **4** |
| `event=qsp_unpack ok=false` | 0 | 0 |
| `event=recv_commit` | 0 | **2** (`count=1` each) |
| `event=recv_item` | 0 | **2** (17 B, 15 B) |
| `event=relay_ack` | 0 | **2** (`sent=2 acked=2`) |
| `out_bob/` · `out_alice/` | **empty** · **empty** | `recv_1.bin` · `recv_1.bin` |
| `summary.txt` | **0 bytes** | `status=pass` |

**Payloads round-tripped byte-identically, verified by `cmp` and not by eye:** alice→bob
`hello-from-alice` **rc 0**; bob→alice `hello-from-bob` **rc 0**. The script's own verdict:
`status=pass · qsp_pack_ok=true both_directions · qsp_unpack_ok=true both_directions ·
recv_commit_bob=1 · recv_commit_alice=1`.

### ⛳ A SECOND, INDEPENDENT PROOF — FROM THE RELAY'S OWN STORE, WITH NO LEASE TAKEN

NA-0736 proved the finding by hashing against sealed **client** logs. This lane proves it again from
the **server** side. `qsl-server` persists `route_key = hex(sha256(token))` and never stores the raw
token, so each candidate mailbox is identifiable by that derivation. Measured **read-only**, with
**no `/v1/pull` issued and no lease taken** (NA-0735 had to disclose a lease for the equivalent
evidence):

| run | mailbox | route exists on relay? | messages remaining |
|---|---|---|---|
| BEFORE | LABEL bob / LABEL alice | **False / False** | 0 / 0 |
| BEFORE | ROUTE bob / ROUTE alice | True / True | **5 / 5 — nothing consumed** |
| AFTER | LABEL bob / LABEL alice | **False / False** | 0 / 0 |
| AFTER | ROUTE bob / ROUTE alice | True / True | **3 / 4 — consumed and acked** |

**4 routes across both runs and ZERO unexplained by the four candidates** — the accounting is
**complete, not a sample**. ⛳ **The mailboxes `receive` was polling never existed as routes at all.**

### BLAST RADIUS, MEASURED RATHER THAN ASSERTED

A full marker-event census across the two runs shows **every handshake-phase count identical**
(`handshake_complete` 4→4, `handshake_send` 6→6, `handshake_status` 14→14, `handshake_pending` 6→6,
`qsp_pack` 2→2, `send_commit` 2→2, `identity_ok` 10→10, `contacts_add` 2→2) while **every changed
count is on the receive path**. **Two values changed exactly what they should and nothing else.**

⚠ **ONE THING THAT LOOKS LIKE A NEW FAILURE AND IS NOT.** The AFTER run carries **2**
`event=handshake_reject reason=handshake_type`. **The red control carries 4.** Pre-existing, it went
**DOWN 4 → 2**, it is in no asserted set, and the run exited 0. Recorded because it is there, not
because it is this lane's.

---

## 8. THE CLAIM BOUNDARY — CARRIED VERBATIM, NOT SMOOTHED

**What this establishes:** the fixture's addressing is now correct, and against a **same-rev**
`qsl-server` a message is delivered end to end, unpacked, committed and acked, both directions,
payload byte-identical.

**What it does NOT establish:**
- ⚠ **It is NOT a CI claim.** SR-20's extension binds: the emitting step's ENVIRONMENT is part of the
  artifact's identity. This ran on **loopback plain HTTP**, on a build box, against a locally chosen
  bearer — **not** in CI's environment, **not** against the AWS relay, **not** through TLS.
- ⚠ **It does not predict the CI outcome**, and the remote proof is unavailable until ENG-0191
  unblocks the suite.
- ⚠ **The run used a harness, not the committed script.** The precise claim is: *everything from
  `:353` to the end of the script passes on loopback.*
- ⚠ **n=1, one scenario** — `happy-path`, seed 1, `send_attempts=1`, `recv_max=1`. **`drop-reorder`
  was not run.**

---

## 9. WHAT WAS NOT DONE

`--from` not touched · `:351`/`:352` not moved or altered in the committed script · **ENG-0191 not
repaired** · WF-0086's coverage gate **not built** · the pull path **not instrumented** (ENG-0193) ·
**#1745 not closed** · NA-0735's sealed evidence not modified · no secret read · no failed step
re-run to green · no `.github/**` · no workflow, dependency or lock change · no test weakened,
skipped or deleted · no standing rule minted · **zero product source bytes** · nothing merged.

⚠ **A disclosed state change:** a local `qsl-server` ran on 127.0.0.1 with a locally generated bearer
and a fresh SQLite store, and was **stopped** at the end of the lane. It touched nothing outside the
seat's own working directory.

---

## 10. EVIDENCE — PRESERVED AT 444, INCLUDING THE RED CONTROL

⚠ **A red run is never discarded, overwritten or summarised away** — it is the thing that makes the
green mean anything.

| artifact | sha256 | lines |
|---|---|---|
| brief (banked verbatim as the first act) | `a8b5959e…46ef` | 118 |
| `SEAL_A_PREMISES` (written before its checks) | `afeb1205…c4b2` | 61 |
| `SEAL_B_RUN_EXPECTATIONS` (written before any run **and** before the edit) | `df417038…f96e` | 131 |
| `build_harness.py` | `b6a9875e…1e6c` | — |
| `apply_edit.py` | `08bc7e56…2363` | — |
| harness BEFORE (== NA-0735's banked harness) | `ddaf4d2d…d729` | 538 |
| harness AFTER | `f52a63e2…dc07` | 538 |
| script BEFORE / AFTER | `275a3e29…2491` / `c885dcf0…0eef` | 482 / 482 |
| **STOP_NA0737_001** | `a1241754…3baa` | 486 |

⚠ **AN EVIDENCE-PRESERVATION DEFECT OF THIS LANE'S OWN, CAUGHT AND CURED — recorded because it is the
most dangerous shape in the set.** The first preservation of the relay store was a plain `cp` of the
SQLite file. It produced a **4096-byte, completely empty database** — every row was in the `-wal` —
and **both `ls -l` and its sha256 reported a healthy preserved artifact.** It was caught only by
**opening the copy and querying it**, and cured with `VACUUM INTO` plus an assertion on row counts
before the copy was trusted. **A clean hash over the wrong bytes is indistinguishable from evidence
until you open it.**

⚠ **D-1 / R331.1:** the operator stop and the sealed run directories live only under `/srv`. This
document is the part that survives machine loss, which is why the substance is carried here and not
merely cited.

---

## 11. TEN SEALED EXPECTATIONS, TEN HITS

E-1 script sha/lines · E-2 edit sites `:375`/`:388` · E-3 variables already present and correctly
used · E-4 `--mailbox` exactly 2× in this file · E-5 `--from` byte-identical · E-6 builder reproduces
the banked harness · E-7 red control in full · E-8 route-token hashes both peers · E-9 delivery
branch (**deliberately not predicted**, so neither outcome could be rationalised afterwards) · E-10
diff shape and lint.

**No expectation missed.** ⚠ **The only things that failed in this lane were this lane's own
instruments**, and each was caught because the expectation was written first — see §3 (the prefix
anchor), §10 (the SQLite copy), and one further case: the relay's `route_key = hex(sha256(token))`
was first compared against the client's `hex(sha512(token)[..4])`, which returned "none of the four
candidates" — *the mismatch is what exposed that `route_key` is a derivation, not the token.*
