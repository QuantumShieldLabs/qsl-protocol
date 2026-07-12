# NA-0638 — NA-0609B Coverage Re-Examination (as built)

Goals: G4, G5

Directive: QSL-DIR-2026-07-12-574 (D574, APPROVED). Decision: D-1261. Base main:
`37914a79` (the NA-0638 promotion merge). Lane class: governance/ledger-only,
READ-ONLY toward the code — this lane MAPPED the coverage behind each of
NA-0609B's nine "verified sound" claims; it re-proved nothing and fixed nothing.

## 1. Method and the audit-time snapshot

NA-0609B (directive D543, D-1213) merged at **`c0b30265`** (PR #1496,
2026-07-06); the audit was read-only toward code, so `c0b30265` is the
audit-time tree every verdict below is judged against. The audit's own method
section states the verdicts were produced by "static reading" with
"cross-reference to the handshake/identity/session-at-rest/adversarial test
suites" (`docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md`
§Scope And Method) — the audit ran nothing itself, so a claim is EXERCISED only
where a pre-existing mechanism was demonstrably capable of finding a
counterexample.

Classification method (WF-0019 standard): for each claim, the audit-time test /
vector / model inventory was enumerated (`git ls-tree`/`git grep` at
`c0b30265`) and the candidate mechanisms READ to determine what they tamper and
assert. **No model or test was built or run to decide any claim** — the D574
mapping boundary was never crossed; at no point did deciding a claim require an
exercise (the two INSPECTED-ONLY verdicts below follow from the ABSENCE of any
candidate mechanism, which reading the inventory establishes). Re-inspection
was used only to CONFIRM what mechanisms do; no verdict was upgraded to
EXERCISED on inspection grounds.

Verdict meaning (do-not-overclaim): EXERCISED asserts only that a
counterexample-capable mechanism EXISTED at audit time — not that the mechanism
is sufficient, and not that the seam is cryptographically secure. Claim 5 is
the standing proof of that distinction.

## 2. The nine-claim verdict table

Claims quoted from `NA-0609B_...harness.md` §Verified Sound (lines 39–73).
Slice overlap = NA-0636's five unmodeled slices (ENG-0038 ledger entry).

| # | Claim (audit doc line) | Verdict | Basis (mechanism named; all paths at `c0b30265`, under `qsl/qsl-client/qsc/`) | NA-0636 slice overlap |
|---|---|---|---|---|
| 1 | Transcript binding (:43) | **EXERCISED** | `tests/handshake_security_closure.rs::handshake_rejects_tampered_transcript_no_mutation` (tampers a live B1 in the relay channel → reject + no session); `tests/kem_signature_transcript_binding_negative.rs::kem_ciphertext_and_transcript_mutation_reject_without_completed_session` (mutates a live B1 KEM ciphertext → `pq_decap_failed`/`REJECT_QSC_HS_TRANSCRIPT_CONTEXT`, no session either side, no A2 emitted); `tests/handshake_mvp.rs::handshake_a2_tamper_rejects_no_mutation` (+ `handshake_out_of_order_*`) covering the responder-confirm path; formal `formal/model_qsc_kem_signature_transcript_binding_bounded.py`, CI-live in `formal.yml` (PR-triggered) at audit time. A broken transcript check turns these red. | Slice 4 (composition with negotiation) is a residue, tracked under ENG-0038 |
| 2 | Hybrid handshake + all-zero DH guard (:48) | **INSPECTED-ONLY** | At `c0b30265`, `all_zero` appears ONLY in `src/handshake/mod.rs` (the guard and its call sites). Zero tests, zero models, zero fuzz targets reference the guard or feed a degenerate key on any path; deleting the guard would have turned nothing red. The hybrid combine is exercised only by happy-path establishment (proves it works, not that it fails closed). The class was real: NA-0628/ENG-0034 (post-audit, 2026-07-10, PR #1536) later added `establishment_dh_rejects_every_low_order_peer_key`, `seven_of_eight_low_order_keys_evade_the_encoding_check` (which DEMONSTRATES the audit-time defence was also narrow: 7 of 8 low-order keys evade the encoding check the guard sat behind) and the four ratchet-boundary noncontributory tests. **The settling exercise already landed** — filed as WF-0021, closed-as-paid. | none |
| 3 | Fail-closed ML-DSA verify (:51) | **EXERCISED** | Invalid half: `tests/handshake_mvp.rs::handshake_b1_signature_tamper_rejects_no_mutation` (flips a byte at `hs_resp_sig_offset()` in a real B1 → reject + no session) and `handshake_a2_signature_tamper_rejects_no_mutation` (A2 confirm signature). Error half: `tests/b1_signature_provider_rng_failure.rs` + `tests/a2_signature_provider_rng_failure.rs` (provider failure → fail-closed). Both certified halves ("invalid and error") had named exercises. Recorded caveat: these prove verify fails closed under the PRESENTED key; they cannot see key-provenance vacuity — that is claim 5, and it was the flaw. | none (the provenance seam is claim 5) |
| 4 | Downgrade / suite-context binding (:54) | **EXERCISED** | Wire-level: `tests/kem_signature_transcript_binding_negative.rs::replay_and_suite_confusion_reject_without_session_mutation` (injects a downgraded suite parameter block `0x0403/0x0001` into a live A1 → `REJECT_QSC_HS_DOWNGRADE`, no B1 emitted, no session); `tests/na_0302_suite2_negotiation_cross_surface.rs`, `na_0304_handshake_suite_id_negotiation.rs`, `na_0313_handshake_suite_id_parameter_block.rs`; formal `model_qsc_handshake_suite_id_bounded.py` + `model_suite2_negotiation_bounded.py`, CI-live at audit time. | Slice 4: the COMPOSITION of negotiation with authentication is covered by neither model — residue tracked under ENG-0038, folded into successor 0b |
| 5 | Identity binding — dual-pin model (:57) | **CONTRADICTED** | ENG-0038 (found NA-0632/D-1256, fixed NA-0633/NA-0634 at D-1257/D-1258, modeled NA-0636/D-1259). The calibration point, pre-decided by D574. Coverage color (the WF-0019 lesson in one row): pin-mismatch mechanisms EXISTED and PASSED at audit time — `tests/handshake_security_closure.rs::handshake_pinned_identity_mismatch_fails`, `tests/identity_binding.rs::pinned_mismatch_rejected_no_mutation` — but they exercised wrong-VALUE pins; the flaw lived in the pin-ABSENT/tautological region (`sig_fp` structurally `None`, primary pin compared to itself), which no mechanism could reach. A mechanism existing is not a mechanism being counterexample-capable for the claim. | Slices 1/2/3 all sit on this seam; the NA-0636 model now covers the core, the slices remain |
| 6 | No-mutation-on-reject (:63) | **EXERCISED** | Dedicated: `tests/handshake_provider_error_no_mutation.rs::pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state`; `tests/handshake_security_closure.rs::handshake_unknown_peer_rejects_without_pending_or_session_state`; `tests/receive_no_mutation.rs`. Pervasive: every negative test in claims 1/3/4/9 asserts no-session/no-pending after reject, spanning tamper, replay, out-of-order, downgrade, provider-error, and unknown-peer reject classes across all three accept sub-paths. | Slice 3 (concurrent pendings): interleaved rejects unexercised — residue tracked under ENG-0038, folded into successor 0b |
| 7 | Operator markers (:67) — the 9th claim, omitted by WF-0019's enumeration | **EXERCISED** | The load-bearing half — `handshake_complete` never emitted without a committed session — is asserted on reject paths: `tests/kem_signature_transcript_binding_negative.rs:453` (`!text.contains("event=handshake_complete")` alongside `assert_no_session`); happy-path tests assert marker + session together. Named inspection-only residue (low materiality): the strict in-process emission ORDER relative to the store call is indistinguishable black-box unless the store itself fails (no store-stage fault injection existed), and the `sig_status ok=true` per-signature semantics had no dedicated negative. Residue folded into WF-0022's harness scope note. The WF-0019 8-vs-9 discrepancy is recorded here: the omission was benign — this claim's verdict is EXERCISED. | none |
| 8 | At-rest persistence / atomic writes (:69) | **INSPECTED-ONLY** | Exercised neighbors existed: `tests/fs_store_contract_na0217b.rs` (permission modes 0700/0600, symlink reject fail-closed, lock contention, leftover-tmp cleanup), `tests/session_state_at_rest.rs` (at-rest tamper → decrypt fail-closed). But the CORE certified property — "file content is never partially written" across the temp-write → `sync_all` → rename window — had NO mechanism: no crash/fault injection existed on any store path, so a reordering or partial-write regression would have turned nothing red. History rhyme recorded: this same bullet's neighborhood produced ENG-0004 and then WF-0005 — the false positive was also corrected by MORE inspection (NA-0609D re-read the cfg-gated fsync variant; it did not exercise the window either). Settling exercise: a crash-window fault-injection harness — filed as WF-0022, open, successor 0c. | none |
| 9 | Replay rejection (:72) | **EXERCISED** | `tests/kem_signature_transcript_binding_negative.rs::replay_and_suite_confusion_reject_without_session_mutation` re-posts a captured live A1 after the handshake has progressed → `REJECT_QSC_HS_REPLAY`, asserts no duplicate B1 is emitted and no session mutation — the certified sentence ("reject replayed A1 when a pending/session already exists") exercised verbatim; plus `tests/handshake_mvp.rs::handshake_a2_replay_rejects_no_mutation` (A2 replay) and `signature_wrong_identity_and_cross_message_replay_reject_without_session_mutation` (cross-message). | Slice 2 (CROSS-SESSION replay) unexercised at audit time and still unmodeled — residue tracked under ENG-0038, folded into successor 0b |

**Tally: 6 EXERCISED (claims 1, 3, 4, 6, 7, 9) · 2 INSPECTED-ONLY (claims 2, 8) · 1 CONTRADICTED (claim 5).**

## 3. What the table means (and does not)

The audit's coverage was substantially better than the WF-0019 filing could
assume: six of nine claims sat on real, named, counterexample-capable
end-to-end mechanisms that were CI-live at audit time. The failure that
produced ENG-0038 was not a barren test suite — it was that the one claim
whose mechanisms were vacuous for the flaw class (claim 5: wrong-value pins
exercised, pin-absent/tautological region unreachable) was certified in the
same breath and the same words as the six that were genuinely exercised. That
is the precise WF-0019 lesson, now with its per-claim boundary drawn: a
soundness list is only as strong as its weakest claim's coverage, and nothing
in the audit's format distinguished them.

None of this moves any claim. EXERCISED rows are not security proofs; the
INSPECTED-ONLY rows are not defects (claim 2's code was later hardened for a
class the audit never asserted; claim 8's code was re-verified sound by
NA-0609D). ENG-0003/0004 stay closed; NA-0609B's filed record is unaltered.

## 4. Findings filed

- **WF-0021** (claim 2): the all-zero/degenerate-DH guard had zero exercised
  coverage at audit time; the settling exercise (low-order rejection unit +
  boundary tests) already landed post-audit via NA-0628/ENG-0034 —
  **closed-as-paid on filing**, recorded so the coverage history is on the
  ledger rather than implicit in a diff.
- **WF-0022** (claim 8): the crash-window atomicity of `write_atomic` has
  never been exercised by any mechanism — the settling exercise is a
  fault-injection harness (kill/fault between temp-write, `sync_all`, and
  rename; assert old-XOR-new content, never mixed, plus marker/store-ordering
  residue from claim 7) — **open**, successor 0c.

Residues on already-ledgered seams (cross-session replay, negotiation×auth
composition, concurrent pendings) are NOT re-filed — they are NA-0636's slices
2/4/3 under ENG-0038; duplicating them would split their tracking. They are
folded into successor 0b's scope instead.

## 5. Exercise successors scoped ON-DECK (not executed, not promoted)

- **0b — bounded-model extension: device indirection first.** Extend the
  NA-0636 `QSC.HS.*` model to the contact-store device indirection +
  primary-device selection rule (slice 1 — the substantive one; converts the
  "REASONED — NOT MODEL-VERIFIED" survival argument on the ENG-0038 entry into
  a search result), then the enumerated follow-on dimensions: cross-session
  replay (slice 2), concurrent pendings (slice 3), negotiation×authentication
  composition (slice 4). Its own directive; formal/ lane.
- **0c — WF-0022 crash-window fault-injection harness** for `fs_store`
  `write_atomic` (+ the claim-7 store-stage ordering residue). LITE lane; its
  own directive.

## 6. Scope proof

Files changed by this lane: `docs/governance/evidence/NA-0638_as_built.md`
(this file), `docs/ops/IMPROVEMENT_LEDGER.md` (WF-0021, WF-0022, WF-0019
status closure), `tests/NA-0638_na0609b_coverage_reexamination_testplan.md`,
`NEXT_ACTIONS.md` (DONE block; 0b/0c ON-DECK; READY=NONE), `DECISIONS.md`
(D-1261), `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` — and
nothing else. No source, no `formal/`, no `tests/*.rs`, no vectors, no
canonical, no `.github/**`. Result:
`NA0609B_COVERAGE_REEXAMINATION_COMPLETE`.
