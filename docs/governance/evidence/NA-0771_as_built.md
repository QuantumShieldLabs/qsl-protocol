# NA-0771 — AS-BUILT — THE ENG-0252 REPAIR

**Decision:** `D-1412`. **Base:** qsl-protocol main `4a4dfdc9df642b29631196f38c63bd12a127b713`.
**Governing kickoff:** `KICKOFF_ENG0252_repair_20260829.md` sha256
`22e5bcb994b268f74c43b6cb7d60ab20addc82607d52b5668b2eb25a91b89881`.

## 1. THE DEFECT, AND WHAT IT COST TO REACH IT

`ENG-0252`: a frame carrying nothing but the public 16-byte `session_id` and arbitrary
bytes destroyed the addressed handshake pending record **before any cryptographic
operation of any kind**. Reachability was MEASURED by running instruments, not argued:

| frame | bytes | site | what it needed |
|---|---|---|---|
| forged v2 RESP | 6438 | `:1783` initiator | the published route token + the cleartext session id |
| malformed v2 CONFIRM | **9** | `:2247` responder | the route token alone — **no session id at all** |
| zero-MAC v2 CONFIRM | 3375 | `:2101` responder | route token + session id + a PUBLIC 9-byte constant |

Each destroyed a live record on unmodified main. Each leaves it untouched after the repair.

## 2. THE CHANGE

ONE product file, `qsl/qsl-client/qsc/src/handshake/mod.rs` — **2 insertions, 20
deletions**; diff sha256 `2863b8309cb36adc38e6069f81f26d50eec44c47a64ce12ef013e0b9c2fed3e7`.

**THE INVARIANT, WHICH IS THE DELIVERABLE:** *a pending record is destroyed only when a
session was stored, or when the record itself will not parse.* Ten `hs_pending_clear`
calls are deleted; `:1781` and `:1785` become `continue`, matching the responder's
`:2059-2065` and `:2070-2073` which already were. `hs_pending_clear` goes from **14 call
sites to 4** — `:1759` and `:2039` (class iv, the local record will not parse) and
`:1944` and `:2175` (class i, after `qsp_session_store` returned Ok).

**WHY AN INVARIANT AND NOT A LIST.** Every deleted site decided destruction on one of: a
suite context (a public compile-time constant, or a value supplied by the peer), a DECODE
FAILURE, or a MAC that FAILED. **None is proof of anything about the sender.** The two
survivors are the only facts the client knows on its own. A count of four is checkable by
a reader in one grep; ten line numbers are not.

## 3. WHAT IS PINNED IN THE TREE

`qsl/qsl-client/qsc/tests/na0771_eng0252_arms.rs`, registered in **both** shard manifests:

* **A1** the initiator site, with `assert_eq!(before, after)` — the strong form; a
  non-empty assertion cannot distinguish "survived" from "replaced".
* **A2** the responder site reached by nine bytes, shipping **both** controls: a
  `suite-required` initiator makes a `legacy-compat` responder's pending wire-explicit
  (positive), a `legacy-compat` initiator must leave it null (negative).
* **A3** the responder site reached by a zero-MAC confirm.
* **A4** on a **real in-process `qsl-server`** with real lease expiry, at **N=1** (the
  handshake completes with a poison frame at the head) and **N=4** (it does not).
* **the count guard**, whose own comment names the three mechanisms it does not catch.

`na_0313`'s five re-aimed assertions use the strong form; two keep emptiness assertions
because at those sites the record was never written — `assert_no_pending` conflates
"cleared" with "never written", which is why the disposition had to be instrumented and
read in one run rather than bisected.

## 4. THE RESIDUAL, NAMED AS ANOTHER ENTRY'S

For **N >= `--max`** (default 4) poison frames the poll still makes no progress: the relay
delivers in insertion order (`store.rs:723` at locked rev `131d63f4`), a lease moves no
message (`:752`), and rejects are never acked, so the pull returns only poison. **That is
`ENG-0198`'s recorded budget-exhaustion shape, OPEN and pre-existing.** This lane does not
repair it. A4's N=4 case pins it as a BOUNDARY so the lane that does find a red arm rather
than a silent pass. **The repair is better than main at every N** — main destroys the
record at every N. Relay figures carried as measured (`D-1411` `DV-11`): TTL 604800 s,
lease 60 s, Lease by default on both field machines.

## 5. THE THREE COLD READS, AND THE FIVE ERRORS THEY FOUND

| file | sha256 |
|---|---|
| `FINDINGS_SR15_NA0771_20260829T160919Z.md` | `1b142fd97a37c67d9c125488f0aaca3ed72bf930da4b0c283894854fd5899a42` |
| `FINDINGS_SR15_NA0771_ADDENDUM_20260829T162513Z.md` | `563510c4c3a2d134cea19817f90dfb3ea3256b699504cb364fac6d743dd48ce2` |
| `FINDINGS_SR15_NA0771_R2_20260829T175308Z.md` | `cbfb0c6dd21e7656bdb29fa0c2debd110daffd0945591a76c3b9c26cf3408c0b` |

The specification was **RETURNED TWICE** and every refutation measured true. All five
errors were the seat's, and each is an SR-16 row (374–379):

1. The census applied the initiator's precondition to the responder — the responder's
   pending context comes from the **wire** (`:2432`, decoded with `admit_context = true`
   at `:539`), so sites called unreachable were reachable.
2. The claim that the `continue` **removed** the head-of-line block — it raises the
   threshold from one poison frame to `--max`.
3. `:1856` declared unreachable on a hypothetical, when it is reachable for an
   `ExplicitV2` initiator pending.
4. An **impossible** interaction banked as the lane's sharpest finding —
   `hs_contexts_match` passing forces equal explicitness, so the `continue` could never
   open `:1856`. Withdrawn.
5. The sealed stop's own stamping pass corrupted two carried documents by +50 bytes each,
   and its seal verified anyway.

## 6. THE METHOD DEFECT, AND THE RULE MINTED FOR IT

**`SR-26` (a'')** — a carried document is verified by its own **whole-file** digest, after
stamping, and the stamping pass never writes inside one. Minted on STOP 004, whose
self-digest construction masks every 64-hex run and was therefore blind to a digest
written over a carried `@@SELFDIGEST@@` literal. ⚠ The first cure had the same blindness:
a `[^-]+` fence label skipped exactly the two hyphenated documents the defect was about,
caught only because the instrument printed its **cardinality** (six carries where eight
were specified) and not merely its verdict.

## 7. CLAIM BOUNDARY

`:2071` is not driven on the shipped mode; its witness runs under `suite-required`. The
initiator mirror at `:1856` is reachable on the algebra and is not driven. No deployed
relay, no rig, no field machine, no GUI: `n = 0` network calls for traffic — A4 uses a real
`qsl-server` in-process, which is a real implementation and not a deployment. The
envelope-wrapping hop (`invite/mod.rs::decode_envelope_resp`) is read, not run: the arms
inject raw frames.
