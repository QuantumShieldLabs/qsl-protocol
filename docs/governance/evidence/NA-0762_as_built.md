# NA-0762 — AS BUILT: the liveness runway's lane one, re-scoped to a records lane (`D-1403`)

**Lane:** NA-0762 · **Decision:** `D-1403` · **Date:** 2026-08-26 · **Base:** qsl-protocol
`c859fcb9428bdbb81b2e45219f2654c5fc3ea28b`, re-derived bare and unpiped at the NAMED github
remote at the edit and measured UNMOVED.
**Ordered by:** `ORDER_NA0762_rescope_records_20260826.md`, sha256
`1e72e3429989d4c8aadb0bd38be80df3cae8113a8c5f8f2ed32491ce4f238e2b`, 167 l / 10643 B, 444.
**Premises artifact:** `STOP_NA0762_001_20260826T013920Z.md`, sha256
`027a4872f84f53dc5cc688603e56e7316b78c81917df7cc4d072da406a8bf47e`, 48411 B / 646 l, 444.
**Superseded-in-the-open:** the banked brief
`BRIEF_liveness_lane1_producer_acks_20260825.md`, sha256
`769eb3a8213b26edb0d01ba1ce53953ec5107f4a3e5d83eacdbff387ed7d6663` — banked and **unedited**.

⚠ **WHY THIS FILE EXISTS.** The measurements below lived only under `/srv` (D-1 / R331.1). This
doc carries STOP 001's substance into repo truth, citing the stop by sha rather than restating it
as if it were newly derived. It is gitignored (`.gitignore:65`, `**/evidence/`) and therefore
added with `git add -f`.

## 1. WHAT THIS LANE IS, AND WHY IT IS NOT WHAT IT WAS BRIEFED AS

The lane was briefed to BUILD producer acks. Its brief's section 0 asserted that `handshake poll`,
`invite finish` and `invite accept` *"never ack what they consume (by construction, measured)"*.

**That premise measured FALSE at this base.** The acks shipped **2026-08-17** — NA-0742 / `D-1379`
/ PR #1760, impl commit `403432ce4ef20fa22343288bf9cb9de8514d6daa`, class
`INVITE_FLOW_RESIDUE_ZERO_PASS`. The commit's own first line reads *"LANE 2 OF THE
ENG-0142/ENG-0196 REPAIR PROGRAM, IMPLEMENTED"*.

It was caught at the **premises** phase, before any edit, SR-15 cold read or loopback rig was paid
for. The Director re-scoped the lane in the open; this file records the re-scope's substance.

## 2. THE CONSUMPTION POINTS, AS MEASURED AT THIS BASE

`transport::producer_ack` (`transport/mod.rs:3659`) is a `pub(crate)` pass-through over
`relay_inbox_ack`, reached through two module-local emit helpers from **FIVE guarded call sites**:

| # | site | command | consumption point | scope gate |
|---|---|---|---|---|
| 1 | `handshake/mod.rs:2012` | `handshake poll` | initiator branch — after the **A2 push**, not after the commit | `acks_own_frames` |
| 2 | `handshake/mod.rs:2197` | `handshake poll` | responder branch — after the durable commit; no push follows | `acks_own_frames` |
| 3 | `handshake/mod.rs:2475` | `handshake poll` | no-pending branch — after the **B1 push** | `acks_own_frames` |
| 4 | `invite/mod.rs:1394` | `invite accept` | after the slot reads `Redeemed` | `resolve_ack_mode(None) == Lease` |
| 5 | `invite/mod.rs:1482` | `invite finish` | after the poll returns `Ok` (the poll performs the outbound A2 push internally) | `ack_mode == Lease` |

`acks_own_frames = matches!(source, HsPollSource::Relay) && crate::resolve_ack_mode(None) ==
crate::cmd::AckMode::Lease` (`handshake/mod.rs:1718`) — **Lease-only; Legacy byte-unchanged.**
A sixth **pull** site exists, `invite/mod.rs:1426`, the Legacy arm of `invite finish`; it acks
nothing and that is correct, because a delete-on-pull relay leaves nothing leased to ack.

⚠ **A FIGURE OF THIS SEAT'S OWN, CORRECTED.** STOP 001 wrote *"six producer-ack call sites"*. The
measured number is **FIVE**; the six came from reading the pull-site table's row count as an
ack-site count. The stop is immutable and is not edited — the correction lives here, at `D-1403`,
and as prediction-ledger row **282**.

## 3. THE RECORDS GAP THIS LANE CLOSES

Three repo-truth files disagreed, and the one a brief-writer reads was the stale one:

| file | what it said about lane 2 before this lane |
|---|---|
| `DECISIONS.md` | `D-1379` headline: **"IMPLEMENTED"** — correct since 2026-08-17 |
| `NEXT_ACTIONS.md` | `NA-0742` `Status: DONE 2026-08-17`, class `INVITE_FLOW_RESIDUE_ZERO_PASS` — correct |
| `docs/ops/IMPROVEMENT_LEDGER.md` | **"Lane 2's producer acks end it"** and **"...still end the tax"** — future tense, twice |

Measured with a positive control: needle `LANE 2 IS BUILT` = **0** in the `ENG-0142` entry region,
against `LANE 1 IS BUILT` = **1** proving the needle method works.

⚠ **The property, recorded and NOT minted as a rule** (RESTRAINT): *a lane whose ledger entry names
a successor lane owes that entry a beside-amendment at the successor's close, the way
`NEXT_ACTIONS.md` owes a status flip.* The consumer question — which file is authoritative for a
defect's STATE when the three disagree — is stated for the Director, not answered here.

## 4. INSTRUMENTS RE-RUN AT THIS BASE

`cargo test -p qsc --test na0742_invite_finish_scan_producer_acks` →
**12 passed / 0 failed / 0 ignored**, 751.02 s. Both shard manifests carry the file
(`scripts/ci/QSC_SHARD_MANIFEST.txt:81` shard 4; `QSC_SHARD_MANIFEST_MACOS.txt:104` shard 1), so
these run in CI and not only locally.

The two that discharge this entry's promises:
`t3_a_completed_flow_leaves_zero_residue` (raw lease-pulls every mailbox the flow touched and
asserts the residue list EMPTY) and `t4_a_receive_after_a_completed_flow_skips_nothing` (zero
`recv_frame_skipped` after the flow).

⚠ **HONEST BOUND — R2's RED-CAPABLE PROOF IS INERT.**
`t8_the_a2_sig_failure_exit_emits_no_producer_ack` is `#[cfg(qsc_rng_failure_test_seam)]`, and that
seam does **not compile**: `RUSTFLAGS='--cfg qsc_rng_failure_test_seam'` → **4 compile errors** in
`qsl/qsl-client/qsc/src/vault/mod.rs` (`:570`, `:578`, `:728`, `:733`). This **reproduces**
NA-0742's own as-built §12(D) with the identical command — which also confirms the invocation was
the house idiom rather than a bad instrument — and is already filed as **`ENG-0197`** (P3, filed
2026-08-17). `git blame` dates the break to `ba1411491`, **2026-07-15**, a month before NA-0742, so
that arm has never been executable in this shape. **Not repaired here.**

## 5. BOUNDS AND WHAT WAS NOT DONE

- **Zero product source bytes.** Six paths, docs only; the `ENG-0142` amendment is beside-only
  (that file's diff: insertions, **0 deletions**).
- **`ENG-0142` is NOT closed.** Its remainder — message- and unknown-class aborts,
  `relay_inbox_parse_failed`, the three post-unpack content aborts, Legacy — stays open, and its
  disposition is the operator's act at this lane's close.
- **`ENG-0197` and `ENG-0196` are not repaired.** Lane 1's classifier and Legacy are untouched.
- **No loopback rig was built** and **no secrets were read**; the `PULL_LEASE_SECS` freshness
  re-verify and the "which rev does the AWS relay run" question are queued as operator
  one-commands before the tick lane's acceptance flight, not before this lane.
- **No standing rule minted.**
