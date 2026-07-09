# NA-0625 — closeout test plan (D-1246)

Directive: QSL-DIR-2026-07-09-562 (D562). Implementation decision: D-1245. Closes ENG-0023.

## Closeout markers
- ENG-0023 marked DONE in `docs/ops/IMPROVEMENT_LEDGER.md` (both gaps closed; the relay-inbox
  advertisement-injection vector filed at NA-0624 is eliminated).
- New ledger items filed by this lane: **ENG-0030** (reseed RECEIVE leaves the receiver's SEND key
  schedule stale — caller-owned coherence; qsc mitigation load-bearing), **ENG-0031** (§8.5.1 vs
  §8.5.4: ADV boundary header NHK or HK), **WF-0014** (a vector-freeze scope claim must be proved
  against the vector BYTES, and the Phase-5 gate list must be derived mechanically from the
  workflows a change touches).
- `NEXT_ACTIONS.md`: NA-0625 -> `Status: DONE` + OUTCOME; successor block restored; LIVE QUEUE
  header updated (`READY=<successor> | HIGHEST_NA=… | HIGHEST_D=1246`). Exactly one `^Status: READY`.
- `TRACEABILITY.md` + `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` map the closeout and the restoration.
- Response filed (`NA0625_suite2_spec_alignment_*_D562.md`); directive archived with Operator Decision 5.

## Post-merge verification (Phase 6)
1. `main` fast-forwarded to merge commit `4b3e4fda` (impl PR #1528); `HEAD == origin/main == main`, tree clean.
2. Main-push workflows all SUCCESS **at job level**, including the two event-filtered suites that
   SKIP on pull_request by design and therefore only exercise on the main push:
   `qsc-linux-full-suite` and `macos-qsc-full-serial`.
3. Post-merge spot re-verification on `main`:
   - all 15 suite2 vector runners green (incl. `e2e_recv 4/4`, `scka_logic 19/19`, `pq_reseed 7/7`);
   - `python3 scripts/ci/validate_suite2_vectors.py` OK;
   - `python3 formal/run_model_checks.py` OK (root-composition slice: 15,494 states, 6 shapes);
   - `cargo test -p quantumshield_refimpl` 112/112.

## What this lane proved (carried forward as the regression surface)
- The PQ-CTXT boundary header authenticates under the §8.5.1 NHK on both sides; an HK-sealed frame
  is rejected generically (`REJECT_S2_HDR_AUTH_FAIL`), byte-pinned in
  `S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001` and exercised e2e on the real client.
- An advertisement is cryptographically bound to the session before it is tracked; a planted
  advertisement injected into the relay inbox is rejected and never tracked
  (`scka_e2e_spoofed_adv_injection_rejected_never_tracked`).
- The ADV consumes its chain slot: `[ADV, reseed]` round-trips in one pack and `mkskipped` stays
  empty in order — the NA-0624 pack-exclusion rule and control-slot growth are retired.
- Reject ⇒ no state mutation on every new path (unit, vector, e2e, and model level).
- The seed-model runtime equivalence is byte-for-byte unchanged.

## Standing claim boundary (unchanged)
No post-quantum, Triple-Ratchet, post-compromise, self-healing, crypto-complete,
security-completion, production-readiness, or public-readiness claim. ENG-0023 hardened the control
plane; it did not lift the boundary. The DH+PQ composition still awaits independent analysis
(ENG-0028). The Decision-4 bounded model abstracts crypto to injective tuple hashes and therefore
proves agreement and coherence, **not** secrecy.
