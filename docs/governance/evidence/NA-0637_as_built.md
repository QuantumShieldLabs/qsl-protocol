# NA-0637 as-built — Audit-methodology coverage finding (directive D573, decision D-1260)

Lane class: governance/ledger ONLY. Zero source/formal/vector/canonical/`.github` change; no claim
movement. This lane FILES (WF-0019, WF-0020) and SCOPES (the NA-0609B re-examination, ON-DECK); it
does not re-audit, does not fix, and does not reopen NA-0609B's filed findings/remediations
(ENG-0003/ENG-0004 stay closed).

## 0. Phase 0 proof

- qwork proof `/srv/qbuild/work/NA-0637/.qwork/startup.qsl-protocol.json`: `startup_result=OK`,
  lane NA-0637, head == origin/main == `5d31f108ac304878e01d27ee71eaab66f9e3a338`,
  worktree/index/untracked clean, `ready_count=1`, `queue_top_ready=NA-0637`.
- CONFIRM-LIVE: directive `QSL-DIR-2026-07-12-573` (D573) is the highest directive; D-1260 absent
  from `DECISIONS.md` before this lane (D-1259 highest); highest WF = WF-0018 → WF-0019 and
  WF-0020 free, exactly as the amended D573 expects; exactly one `^Status: READY` (NA-0637);
  STATE line `READY=NA-0637 | HIGHEST_NA=0637 | HIGHEST_D=1259`.
- The drop re-verified in this workspace (see §6).

## 1. The claim (NA-0609B) — established read-only, quoted exactly

NA-0609B ("qsc Handshake and Identity Read-Only Security Audit", directive D543, LITE-CEREMONY,
D-1213, merged 2026-07-06) closed with result:

> `QSC_HANDSHAKE_IDENTITY_AUDIT_COMPLETE_NO_P0_P1_THREE_P3_HARDENING`

(`docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md:20`;
`tests/NA-0609B_qsc_handshake_identity_security_audit_testplan.md:54`; `TRACEABILITY.md:1556`;
the NA-0609B lane block, `NEXT_ACTIONS.md` §"NA-0609B").

The evidence doc's top-line verdict (`:22`):

> "No P0 or P1 finding was substantiated. The handshake seam is well-constructed and fail-closed."

The enumerated sound-claims (lane block; identically in `TRACEABILITY.md:1556`): the handshake seam

> "verified sound on transcript binding, hybrid handshake + all-zero DH guard, fail-closed ML-DSA
> verify, downgrade/suite-context binding, dual-pin identity model, no-mutation-on-reject, atomic
> writes, and replay rejection."

The two claims at the heart of this finding, verbatim from the evidence doc's "Verified Sound"
section:

> "Identity binding: a dual-pin model — the primary pin is checked against the KEM identity
> fingerprint (`identity_fingerprint_from_pk(kem_pk)`, the same value `identity show` displays),
> with the ML-DSA signing-key fingerprint (`hs_sig_fingerprint`) as a separate optional pin. A
> mismatch fails closed with `peer_mismatch` (`hs_require_primary_identity_pin`, 896) and is
> checked before any persistence (1491, 1721-1729, 1823)."

> "Signatures: `hs_sig_verify` (866) uses ML-DSA-65 and is fail-closed on both invalid and error;
> the A2 confirm signature is verified over the transcript hash and confirm MAC (`hs_sig_msg_a2`,
> 857) before acceptance (1683-1688)."

## 2. The reality (ENG-0038) — the P1 lived in exactly that seam

ENG-0038 (filed 2026-07-11 by NA-0632, D-1256; ledger entry `docs/ops/IMPROVEMENT_LEDGER.md`
§ENG-0038): the shipped `QSC.HS.*` handshake did NOT authenticate the responder to the initiator —
an active on-path attacker could impersonate the responder, and the out-of-band verified code did
not prevent it. Found by NA-0632, fixed by NA-0633 (C1, D-1257) + NA-0634 (D-1258), formally
modeled by NA-0636 (D-1259).

The defect was not adjacent to the audited seam; it was INSIDE the two certified mechanisms:

- The "separate optional pin" (`sig_fp`) was **structurally always `None`** — no product path ever
  populated it (`contacts_add`/`contacts_device_add` set `sig_fp: None` even with `verify=true`),
  so the optional check **always skipped** (`handshake/mod.rs:1532`, `contacts/mod.rs:1047/:1053/
  :1110`, `identity/mod.rs:634-641`).
- The "primary pin ... checked before any persistence" was, on the initiator side,
  **TAUTOLOGICAL**: `pending.peer_fp` is set from `identity_read_pin(peer)` at initiate and
  re-compared to that same pin at B1 — the responder's KEM key is never sent or used B→A, so the
  check binds nothing about the live peer (`handshake/mod.rs:1241/:1295/:1527`).
- The fail-closed ML-DSA verify was **self-referential for identity**: the initiator verified the
  B1 signature under `resp.sig_pk` — the key the responder itself SENT — which verifies for ANY
  key (`handshake/mod.rs:1509`).

Each observation NA-0609B recorded about these mechanisms was locally accurate — the pins exist,
the checks fire, the rejects are fail-closed. What inspection did not surface is that on the live
initiator path the optional pin never binds (nothing populates it) and the primary pin binds
vacuously (it compares a stored value to itself). Only an end-to-end adversarial exercise — a
wrong responder actually attempting the handshake — reveals that; that test did not exist until
NA-0633 (`tests/NA_0633_eng0038_reproduction.rs`).

Proof gap as recorded on the ENG-0038 ledger entry: `src/adversarial/binding_fuzz.rs` covered only
frame decoding + pin-string comparison; `tests/kem_signature_transcript_binding_negative.rs`
required a hand-injected `sig_fp` (no product path set one) and tested only a wrong-pinned value;
the ProVerif model (DOC-G4-002) covered the ratchet composition — the `qsc` handshake
authentication was UNMODELED.

The contradiction is already recorded on the ledger (ENG-0038 entry): "⚠ RE-TESTS AND CONTRADICTS
a prior 'verified' conclusion. ENG-0001 / NA-0609B concluded 'the verification-fingerprint model
is COHERENT … there is no KEM-vs-SIG binding flaw.' … It does not hold on current code for the
initiator→responder direction."

## 3. The five slices STILL unmodeled after the fix chain (NA-0636, ledger §ENG-0038)

Recorded 2026-07-12, before the NA-0636 merge, at operator direction:

1. The contact-store DEVICE INDIRECTION (the substantive one — the store-coherence justification
   was established by reading the code, NOT model-verified).
2. Cross-session replay.
3. Concurrent pendings.
4. Composition with suite negotiation / downgrade.
5. Fingerprint collision-resistance (ASSUMED — the load-bearing case: the P3 discharge is
   contingent on it).

These are the freshest, concrete coverage gaps on the audited seam and the natural first targets
of the re-examination (§5).

## 4. The prior lesson (WF-0005) — this is the second methodology miss from the same audit

WF-0005 ("Audits must check for cfg-gated alternate definitions before calling a function a
no-op"; done at NA-0609D, D-1216, 2026-07-06): NA-0609B reported ENG-0004 (directory-fsync no-op)
by reading only the `#[cfg(not(unix))]` stub and missing the real `#[cfg(unix)]` implementation —
a false POSITIVE from incomplete inspection. WF-0019 records the inverse and more serious case: a
false NEGATIVE ("verified sound") from inspection that could not see a vacuous binding. Same
audit, same root cause — conclusions outrunning the mechanism that produced them.

## 5. The re-examination scope (SCOPED here, NOT executed — recorded ON-DECK in NEXT_ACTIONS)

A bounded, read-only successor lane (its own directive; the operator promotes) that revisits each
NA-0609B "verified sound" conclusion and answers, per claim: was the verdict backed by an
EXERCISED mechanism (a test, vector, or model run demonstrably capable of finding a
counterexample) or by INSPECTION alone?

- The claim list (from §1): transcript binding; hybrid handshake + all-zero DH guard; fail-closed
  ML-DSA verify; downgrade/suite-context binding; dual-pin identity model (already CONTRADICTED by
  ENG-0038 — the calibration point); no-mutation-on-reject; atomic writes; replay rejection.
- Seed targets: NA-0636's five unmodeled slices (§3), device indirection first.
- Method: the NA-0636 bounded-model + WF-0017 non-vacuity discipline — prefer a bounded exercise
  over re-inspection; a re-inspection may only CONFIRM a verdict, never upgrade it to "exercised".
- Output: a per-claim verdict table (EXERCISED / INSPECTED-ONLY / CONTRADICTED) + new ledger
  findings where warranted. Fixes are separate lanes. No reopening of ENG-0003/0004.

## 6. The drop, confirmed (D571 Decision 4 / Phase 5 vs NA-0634 / D-1258)

- D571 (REV 4) Decision 4 (directive file, "Release gates" section): "**File an AUDIT-METHODOLOGY
  finding (re-scoped from the original missing-test filing).** … The tracked item MUST record the
  methodology lesson" — with the summary line "the audit-methodology coverage finding (file now,
  re-scoped)".
- D571 Phase table, Phase 5 (line 325): "Audit-methodology finding: file the coverage blind spot
  (not just the missing test) per Decision 4; schedule the bounded re-examination of NA-0609B's
  coverage claims."
- NA-0634 closed 2026-07-11 at D-1258. `docs/governance/evidence/NA-0634_as_built.md`: **zero**
  occurrences of audit-methodology/coverage/0609 (grep-verified in this workspace). D-1258 records
  the finding only under "Successor (WF-0003) — PROPOSED, NOT PROMOTED" as a candidate lane — i.e.
  as future work, not as the executed in-lane filing Decision 4/Phase 5 required. The ledger
  gained no WF item.
- The omission surfaced only at the 2026-07-12 read-only live-state check run after NA-0636
  closed — an ad-hoc mechanism, not a guaranteed one.

## 7. Filed by this lane

- **WF-0019** — the coverage finding (`docs/ops/IMPROVEMENT_LEDGER.md` §WF-0019).
- **WF-0020** — the process item recording the drop (`docs/ops/IMPROVEMENT_LEDGER.md` §WF-0020).
- The re-examination successor scoped ON-DECK (`NEXT_ACTIONS.md`, ON DECK item 0a).

## 8. Validation results

Recorded at PR time (see `tests/NA-0637_audit_methodology_coverage_finding_testplan.md` for the
commands and outputs): scope guard — every changed path within the D573 allowed list; goal-lint —
PASS locally against a synthesized PR event; private-material scan — no secret/private value in
the diff (class-only content); single-READY invariant restored (READY=NONE at closeout, exactly
zero `^Status: READY` lines, the operator promotes the successor); D-1260 present exactly once.

## 9. Claim boundary

UNCHANGED. This lane moves no security claim: it records that a past internal audit's assurance
was weaker than stated — which narrows, not widens, what the project claims. Independent external
review remains an open prerequisite for any public security claim.
