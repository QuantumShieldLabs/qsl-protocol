# NA-0636 test plan — `QSC.HS.*` bounded handshake-authentication model

Goals: G1, G2, G4

Directive: QSL-DIR-2026-07-11-572 (D572). Decision: D-1259. Lane class: `formal/` + governance —
**no protocol/source/wire/crypto/state-machine change**, therefore **no new Rust test and no
vector change**. The deliverable is an executable model; its "tests" are the model's own
fail-closed assertions, exercised by the existing formal runner.

## What runs, and where

| Gate | Command | Status |
|---|---|---|
| Full bounded-model suite (local) | `python3 formal/run_model_checks.py` | **green, exit 0** — all 5 models |
| The new model alone (local) | `python3 formal/model_qsc_handshake_authentication_bounded.py` | **green, exit 0** |
| CI | `.github/workflows/formal.yml` → job `formal-scka-model` (REQUIRED) → `run_model_checks.py` | picked up automatically |

Registration is by **explicit import + call** in `run_model_checks.py` `main()` — the established
house pattern. `formal.yml` executes the runner, so **no workflow change was needed** (and none was
made; the committed `.claude/settings.json` denies `Write(.github/**)`).

## Properties asserted (fail-closed — any violation aborts the runner non-zero)

- **P1 — mutual-auth binding.** For every reachable accepting state (initiator-accept and
  responder-commit): the counterparty holds BOTH the KEM secret and the signing secret of the
  identity the pinned code authenticates, and the bound pair is exactly the presented pair.
  *Result: 80 responder + 60 initiator accepting states, all bound.*
- **P2 — wrong-signing-key rejection.** Every configuration where the peer holds the correct pinned
  KEM identity but presents a wrong signing identity must reject deterministically, with no commit
  and no success output (NA-0634's `hs_require_sig_identity_pin`). *Result: 720 configurations, all
  reject.*
- **P3 — reverse-pin redundancy (THE ENG-0038 OBLIGATION).** With the responder→initiator sig-pin
  OPTIONAL exactly as landed (skip-on-absent), search for any reachable responder-commit in which
  the initiator's presented signing key is NOT bound to the verified code. *Result: **0 of 80** —
  redundancy PROVEN over the bound; obligation discharged affirmatively.*
- **P4 — fail-closed reject hygiene.** Every reject carries a reason label from a fixed declared set
  (an undeclared label is itself a model failure), commits no session, emits no success output,
  mutates no durable state. *Result: 10,720 responder + 10,740 initiator reject assertions.*

## Negative / non-vacuity controls (the ones that make a green result mean something)

A "no counterexample found" claim is worth only what the search could have found (WF-0017). Both
controls **must** find the flaw, or the model fails closed:

- **Faithfulness anchor — the model reproduces the REAL ENG-0038 flaw.** Rewind to
  pre-C1 + pre-NA-0634 (no responder-KEM possession binding; sig-pin optional with `sig_fp`
  structurally absent) ⇒ **54 impersonation traces**; canonical witness = an adversary that stole
  **nothing**, signing B1 with its own generated keypair, making the initiator commit
  `authenticated=true` to a peer holding neither of Bob's identity secrets (the NA-0632 §2.2
  scenario, verbatim). Under the **landed** rules: **0**.
- **P3 non-vacuity.** Rewind **only** the primary pin to its pre-NA-0634 KEM-only form ⇒ the P3
  search surfaces **128 unbound-signing-key commits** that a required reverse pin would have caught.
- **ENG-0038 regression witnesses (must stay 0 under the landed rules).** Initiator-accepts by an
  adversary holding (a) neither of the responder's identity secrets — the original flaw; (b) the
  responder's KEM identity secret but NOT its signing secret — the NA-0634 half. *Both 0.*
- **Gate fail-closed check.** Re-running the P3 machinery against the pre-fix rules returns
  `redundant=False` and the `emit_*` guard raises `QSC_HS_HANDSHAKE_AUTH_MODEL_GAP_FOUND` ⇒ the
  runner exits non-zero. **A disproof turns the formal check RED; it cannot silently pass.**

## The bound (exhaustively enumerated; no reduction ⇒ PASS, not bounded-PARTIAL)

3 identities (2 honest + 1 adversary) · 3 KEM keys × 3 signing keys · 5 contact-pin states
(FULL / legacy KEM-only / bare-combined / bare-single / absent) explored **independently at A1 and
A2** (mid-run re-pin) · 3 message producers · **all 16** adversary compromise subsets of the four
honest identity secrets · 3 reverse-pin modes (as-landed-optional / required-counterfactual /
disabled) ⇒ **10,800 responder + 10,800 initiator configurations**, fully enumerated. Termination is
structural (a finite product, no fixpoint) — the ENG-0035 non-termination constraint is avoided by
construction, not by cutting the bound.

## Explicitly NOT covered (do not quote a result beyond this)

- Cryptographic security, side channels, post-compromise or PQ guarantees, refimpl equivalence.
- **Fingerprint collision-resistance is ASSUMED, not proved** — codes are injective structured
  tokens. The P3 verdict *depends* on the combined code covering the signing key injectively; the
  non-vacuity counterfactual makes that dependency visible rather than hiding it.
- Multi-session / concurrent-handshake / cross-session replay: one bounded handshake per
  configuration. Concurrent/competing pending handshake records are likewise not modeled.
- **The contact-store DEVICE INDIRECTION.** The model assumes a single coherent
  `(pin_code, kem_stored, sig_fp)` triple. The shipped code resolves the pin reads through a
  primary-device indirection (`identity_read_pin` vs `identity_read_sig_pin` /
  `identity_read_peer_kem_pk`), whose coherence is upheld by `normalize_contact_record` running on
  every store load/save — a fact established by **reading the code, not by the model**. Primary-device
  selection, and a change of primary device mid-handshake, are unmodeled. The verdict is **argued** to
  survive (binding flows from the REQUIRED primary pin; a stale `sig_fp` can only false-reject, never
  admit — fail-closed) but that survival is **REASONED, not model-verified**. Recorded as a known
  unmodeled slice on the ENG-0038 ledger entry; extending the model to it is a candidate follow-up
  lane. See `docs/governance/evidence/NA-0636_as_built.md` §1.2.
- The **composition** of authentication with suite negotiation/downgrade (each is modeled on its own;
  their composition is modeled by neither).
- The NA-0635 prekey redesign (a separate GATED lane) — not modeled here.

## Suites expected unchanged (formal/ + governance lane) — confirmed

No Rust source, vector, canonical-spec, `qsp`, server, or workflow file is touched, so the Suite-2
vector suites, `qsc` suites, and protocol suites are expected to be unaffected. Scope-guard diff
confirms the changed-file list matches the D572 allowed-path list exactly.
