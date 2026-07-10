# NA-0628 — ENG-0034 test plan: reject non-contributory (low-order) X25519 on every LIVE DH path

Goals: G1, G2, G4
Directive: QSL-DIR-2026-07-10-565 (D565) as amended by **AMENDMENT 1 (D565-A1)**. Decision: D-1251.
Base: `main == 1fdd5b9b`.

## 0. What is under test

RFC 7748 §6.1: X25519 is deliberately non-contributory — a small-subgroup peer point yields an
all-zero shared secret rather than an error, so protocols requiring contributory behaviour MUST check
the all-zero DH **output**. Before this lane the repo never did. The six LIVE sites now do:

| # | Site | Function | Reason on reject |
|---|---|---|---|
| 1 | `suite2/ratchet.rs:1306` | `send_boundary` | `REJECT_S2_DH_NONCONTRIBUTORY` |
| 2 | `suite2/ratchet.rs:1475` | `recv_dh_boundary` | `REJECT_S2_DH_NONCONTRIBUTORY` |
| 3 | `suite2/ratchet.rs:1885` | `send_combined_boundary` | `REJECT_S2_DH_NONCONTRIBUTORY` |
| 4 | `suite2/ratchet.rs:2390` | `recv_combined_boundary` | `REJECT_S2_DH_NONCONTRIBUTORY` |
| 5+6 | `qsc/src/handshake/mod.rs:801` `hs_dh_shared` | covers `:1449` initiator, `:1877` responder | `dh_noncontributory` (qsc-local marker, NOT canonical) |

**Key adversarial fact this plan pins:** the pre-existing `is_zero32(&parsed.dh_pub)` ingress screen
(`ratchet.rs:1420`, `:2317`) rejects only the **all-zero encoding** — one of Curve25519's eight
low-order points. The other **seven** pass it and drive `dh_out = 0`. Every negative case below
therefore uses `u=1` (`0100…00`), never the all-zero encoding: a test that used the all-zero encoding
would pass for the wrong reason and prove nothing about the new guard.

## 1. Unit / co-located tests

### 1.1 refimpl (`cargo test -p quantumshield_refimpl`)
- `send_boundary_rejects_noncontributory_peer_key_no_mutation` — peer's stored `DHr` is low-order;
  `send_boundary` returns `Err("REJECT_S2_DH_NONCONTRIBUTORY")` and the caller's state snapshot is
  byte-identical.
- `recv_dh_boundary_rejects_noncontributory_dh_pub_no_mutation` — the peer ratchets honestly but
  advertises a low-order `DH_pub` (stub `LowOrderPubDh`, real X25519 for `dh()`), so the frame is
  correctly sealed under the pre-boundary `NHK_s` and authenticates. Only the DH OUTPUT check can
  catch it. Asserts reject reason and byte-identical state.
- `send_combined_boundary_rejects_noncontributory_peer_key_no_mutation` — same as (1) on the combined
  DH+PQ sender.
- `recv_combined_boundary_rejects_noncontributory_dh_pub_no_mutation` — combined frame with a
  low-order `DH_pub`; the guard must fire **before** the PQ reseed is applied, so no target/tombstone
  state moves. Asserts reject reason and byte-identical state.
- `noncontributory_guard_is_not_shadowed_by_the_dh_pub_encoding_check` — an all-zero **encoding**
  still yields `REJECT_S2_HDR_AUTH_FAIL`, pinning the two guards as distinct so a future refactor
  cannot collapse them and silently lose seven encodings.
- `na0628_every_dh_call_site_is_guarded_or_allowlisted` — the anti-regression scan (§3).

### 1.2 qsc (`cargo test -p qsc`)
- `establishment_dh_rejects_every_low_order_peer_key` — all **eight** classical low-order encodings,
  against **two** structurally different clamped scalars, must return `Err("dh_noncontributory")`.
  This runs against the real `x25519-dalek`, so it is a property of the shipped primitive, not of a
  model.
- `seven_of_eight_low_order_keys_evade_the_encoding_check` — pins the reason the output check is
  required at all.
- `establishment_dh_accepts_an_honest_peer_key` — negative control: the guard cannot be passing by
  rejecting everything.
- `length_errors_keep_their_distinct_marker` — the new marker does not swallow `handshake_dh_len`,
  whose callers map it to the `dh_failed` log reason.

### 1.3 Mutation proof of the unit tests (performed; evidence in the proof root)
With all four refimpl guards deleted, tests 1.1(a)–(d) **all fail**. Restored, all pass. A test that
cannot fail proves nothing.

## 2. Conformance vectors (CI-durable; run by the REQUIRED `suite2-vectors` check)

Both are ADDITIVE: cloned from an accepting vector with **exactly one field changed**. Neither edits
any existing vector. Both are executed by the existing runners with **zero workflow change**, because
the runners select by tag from their default file.

| Vector | File | Op | Changed field | Expect |
|---|---|---|---|---|
| `S2-RECV-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001` | `qshield_suite2_pq_reseed_vectors_v1.json` | `suite2.combined_boundary.run` | `message.new_dh_pub → 0100…00` | `ok:false`, `REJECT_S2_DH_NONCONTRIBUTORY` |
| `S2-SEND-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001` | `qshield_suite2_scka_logic_vectors_v1.json` | `suite2.send_combined_boundary` | `dh_state.dhr → 0100…00` | `ok:false`, `REJECT_S2_DH_NONCONTRIBUTORY` |

The actor independently asserts, for every combined-boundary reject shape, that `snapshot_bytes()` is
unchanged — so the receive vector proves "rejects **and** does not mutate" mechanically.

**Mutation proof (performed):** with the guards removed,
- the receive vector fails (`missing reason_code in error message`), and
- the send vector fails with **`vector expects failure but actor returned ok`** — i.e. the unguarded
  sender silently ratchets on a degenerate shared secret. That is the vulnerability, demonstrated.

**Reachability deviation, recorded not adapted:** the conformance actor's only ratchet entry points
are `recv_boundary_in_order`, `recv_wire`, `recv_pq_reseed`, `send_combined_boundary`,
`send_pq_advertise`, `send_pq_reseed`. Sites 1 (`send_boundary`) and 2 (`recv_dh_boundary`) are
therefore **not reachable from any actor op**; a vector for them would require a new op in
`tools/actors/**`, a boundary-FORBIDDEN path. They are covered by §1.1 instead. Their guards are
identical in form and proof to site 4, which IS vector-proved end to end.

## 3. Anti-regression scan (Operator Decision 2(c), amended by D565-A1.5)

Every `.dh(` call site in the repo must be either (i) followed by a fail-closed all-zero output check,
or (ii) listed in `ALLOWED_UNGUARDED_DH` per-site (file + enclosing function) with a **written
reason**. Per-file counts are **PINNED** — 22 sites across 8 files.

**Proved able to fail (all three mandated mutations, performed):**
1. a synthetic unguarded `.dh(` in unlisted code (`suite2/state.rs`) → FAIL;
2. an allowlist entry whose reason is blank → FAIL;
3. **count drift inside an already-allowlisted file** (`qsp/ratchet.rs` 4 → 5) → FAIL.

Mutation 3 is what makes the allowlist safe: without pinned counts the next unguarded site would hide
inside an allowlisted file.

**Known limitation, stated not papered over.** No CI job runs `cargo test -p quantumshield_refimpl`
(the only `cargo test` invocations in `.github/workflows/**` are qsc's, plus miri). The scan and the
refimpl guard tests are a **lane/local gate, not a PR gate**. The CI-durable proof of the guard is §2.
Wiring the refimpl tests into an existing job is a one-line `.github/**` change — forbidden by this
lane's Result boundary and filed for the successor. **No claim is made that a future call site
"cannot" forget the guard.**

## 4. WF-0014 byte claim (machine-asserted; a prose note is not acceptable)

`docs/governance/evidence/NA-0628_vector_byte_scan.py`, against `git show HEAD:<file>` for all 19
`inputs/suite2/**` vector files:

1. no pre-existing vector id removed;
2. all **162** pre-existing vectors byte-identical (per-vector canonical sha256);
3. every file outside the two-file append allowlist byte-identical at FILE level;
4. the new-id set is exactly the two ids in §2;
5. cross-set sha256 over the sorted pre-existing hashes matches the HEAD baseline
   (`460f97e30cc8dc9ee07a424e11aa3c3e64e79ce949b1fb21160d5d5253e4b12a`).

**Result: PASSED.** The scan is itself mutation-proved: it exits non-zero on a flipped byte in an
untouched file, on an altered pre-existing vector inside an appended file, and on a smuggled extra id.

Reading recorded: the claim is **per-vector**. A purely additive append changes a file's bytes while
changing no transcript, and the STOP's rationale ("the guard would be rejecting an honest transcript")
is a statement about transcripts. Both appended files round-trip byte-exactly under
`json.dumps(data, indent=2) + "\n"`, so every pre-existing line is preserved verbatim.

## 5. Gates (derived mechanically from the touched surfaces)

| Gate | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo build --workspace --all-targets --locked` (WF-0013) | PASS |
| `cargo clippy -p quantumshield_refimpl --all-targets -- -D warnings` | PASS |
| `cargo clippy -p qsc --all-targets -- -D warnings` | PASS |
| `cargo test -p quantumshield_refimpl --locked` | PASS |
| `cargo test -p qsc --locked` | see response file |
| `python3 formal/run_model_checks.py` | PASS (15,032 states / 9 shapes) |
| `python3 formal/proverif/run_proverif_checks.py` (NA-0627 gate, ~24 min) | must stay green, UNCHANGED |
| `suite2-vectors` runners (pqreseed 12/12, scka-logic 21/21) | PASS |

## 6. DoD 5 — Q7

**Q7 cannot be honestly strengthened, so `formal/proverif/**` is unchanged.** `zeroG` is an
algebra-free constant (extending ProVerif's DH theory with a degenerate element makes the tool
diverge, D-1249), and Q7 mirrors the shipped `is_zero32(DH_pub)` **ingress encoding** check. The new
guard is on the DH **output**, whose degeneracy is exactly the algebraic fact abstraction **A4** masks.
A model that cannot represent a low-order point cannot represent the condition the new guard tests.
Q7 remains, in every artifact, explicitly **not an attack-existence proof**.

## 7. What this lane does NOT prove

- It does not move the claim boundary. Post-compromise / Triple-Ratchet language remains blocked by
  the A1–A8 abstractions, ENG-0035's 2-epoch unrolling gap, and **independent human review**.
- It does not harden `qsp/**` (ENG-0019, re-rated P2, unfolded to its own directive).
- It does not make the anti-regression scan a PR gate (§3).
- Q7 is not, and never was, an attack-existence proof.
