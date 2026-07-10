# NA-0629 — DESIGN-LOCK (WF-0018 strategic-docs truth-up per D566)

Goals: G4, G5. Directive: QSL-DIR-2026-07-10-566 (D566, APPROVED). Base: `main == 17c9dc19`
(NA-0628 closeout; ENG-0034 closed at e9439df7). Recorded 2026-07-10.

**Claim rule (Operator Decision 1): the claim boundary does NOT move. Every edit below either states
merged ENGINEERING or corrects a FACT about a now-closed blocker. No claim STATUS and no
"research-stage / Triple-Ratchet-style / not-proven" sentence changes. Fail-closed: any drift = STOP.**

Two tiers: **[SAFE]** (factual/evidence, executed by the lane) and **[CLAIM-ADJ]** (posture/analysis
wording near the boundary — the exact before/after IS the operator-approval artifact; not executed
until approved).

---

## [SAFE] items (executed by the lane)

### S1 — `FORMAL_VERIFICATION_PLAN.md` (ENG-0034 closed; claim still blocked)
- `ENG-0034 (P2)` bullet, final sentence — BEFORE: *"It blocks post-compromise claim language until
  fixed."* AFTER: *"**Closed at NA-0628 (D-1251/D-1252): every LIVE X25519 DH output now fails closed
  on the all-zero value (RFC 7748 §6.1), with `REJECT_S2_DH_NONCONTRIBUTORY`, additive negative
  vectors, and a byte-scan. The CODE obstacle to post-compromise language is removed; the claim
  remains blocked by the A1–A8 abstractions, ENG-0035, and independent human review.**"* — mark the
  bullet `ENG-0034 (P2, CLOSED)`.
- "Next candidates" line — BEFORE names "the ENG-0034 remediation lane"; AFTER drops it (done) and
  leads with "a Tamarin lane for the multi-epoch unrolling (ENG-0035), IFF the post-compromise claim
  is pursued."

### S2 — `docs/public/EXTERNAL_REVIEW_PACKAGE.md` (add the strongest evidence)
- The evidence sentence (line ~13) currently ends at "executable formal/model checks." APPEND a
  clause + a package row: the **CI-gated ProVerif 2.05 symbolic analysis of the Suite-2 DH+PQ
  composition** (NA-0627; `docs/design/DOC-G4-002`), the **single-root DH+PQ composition** (NA-0626),
  and the **RFC 7748 §6.1 contributory-DH guard** (NA-0628). Claim disclaimers (line 15, "does not
  claim … a proven true Triple Ratchet") reused VERBATIM.

### S3 — `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` (G1/G4 evidence, STATUS untouched)
- Add the ProVerif layer + single-root composition to the G1 and G4 evidence columns. **Every gate
  STATUS cell (PARTIAL/NOT_READY/PROVEN) byte-unchanged** (Operator Decision 1).

### S4 — `docs/public/PROGRESS.md` + a new dated entry
- New `docs/public/progress/2026-07-10.md` summarizing the crypto-core completion arc (SCKA → single
  root → ProVerif → contributory DH) with proof pointers; engineering-evidence tone; the existing
  no-readiness / no-proven-Triple-Ratchet disclaimer reused. Index it in PROGRESS.md.

### S5 — `STATUS.md` → deprecate-to-stub (Operator Decision 2)
- Replace the stale ledger (last real update 2026-03-02 / NA-0177 READY) with a compatibility stub
  pointing at `NEXT_ACTIONS.md` (live queue) + `DECISIONS.md`, matching the `QSL_PUBLIC_RELEASE_PLAN.md`
  precedent verbatim in structure.

### S6 — superseded-by pointers
- `docs/audit/METADATA_MITIGATIONS_ROADMAP_NA-0137.md` + `ONGOING_PQ_RATCHET_ROADMAP_NA-0135.md`:
  add a `Superseded-By:` header (metadata → ENG-0022/0027; PQ ratchet → the NA-0619..0628 arc). No
  body rewrite.

### S7 — `docs/public/INDEX.md` / `WEBSITE_CLAIM_MATRIX.md` mechanical consistency
- Date/pointer refresh only to stay consistent with S2–S4. **`WEBSITE_CLAIM_MATRIX.md` STATUS column
  byte-unchanged** (a Phase-4 assertion).

### S8 — Doc-staleness lint (Operator Decision 3 = YES)
- `tests/NA-0629_doc_staleness_lint.py` (or a `tests/` check): flags a strategic doc whose
  `Last-Updated` trails main's highest `D-####` by more than a threshold. Lightweight, **not** a
  required CI check (it rides the lane gate + local runs, like the NA-0628 anti-regression scan).

---

## [CLAIM-ADJ] items — EXACT before/after for operator approval (NOT executed until approved)

### C1 — `ROADMAP.md` "Current posture", 2nd paragraph (the recovery-PR framing)
BEFORE (verbatim):
> The immediate recovery sequence is now complete enough to resume forward engineering: the
> dependency advisory was remediated, the `send_commit` regression was repaired without restoring
> retired mock-provider behavior, `public-safety` was restored as a required check and completed green
> after PR `#723`, and the fail-closed KT verifier implementation merged through PR `#708`.

AFTER (proposed):
> The cryptographic core is now correctness-complete: the Suite-2 DH+PQ composition is unified on a
> single root (NA-0626), independently analyzed in a CI-gated ProVerif symbolic model (NA-0627), and
> the last known correctness gap — the RFC 7748 §6.1 non-contributory-DH check — is closed (NA-0628).
> No open P1 remains and there is no known correctness gap in the crypto core. **The remaining gate on
> any post-compromise / production claim is now review, not engineering: independent human review plus
> the bounded ENG-0035 formal follow-up.** Forward work is hardening, metadata (ENG-0022/0037), and the
> TUI/GUI + private-server product direction (ENG-0036).

**The 1st paragraph ("remains a research-stage protocol … not production-ready …") is KEPT VERBATIM.**
No claim STATUS moves; this states merged engineering + names the review gate.

### C2 — `ROADMAP.md` 30/60/90-day priorities
Refresh the dated lists so they reference live lanes (NA-0619..0628 done; external-review bundle,
ENG-0019, ENG-0035, WF-0018 next) rather than the April KT/SCKA-vector framing. Non-goals section
KEPT. (Full before/after enumerated at execution; no claim wording involved — mechanical currency.)

### C3 — `docs/program/DOC-PROG-001` §2 + §3 "Merged work now" column
§2 "current merged workstreams": ADD the single-root DH+PQ composition + the ProVerif assurance layer.
§3 gate table, **"Merged work now" column only** (G1 and G4 rows): add the composition + ProVerif.
**The "Still required before any release claim" column is KEPT VERBATIM** — still review-gated.

### C4 — `docs/design/DOC-G4-002` §7 — STALE claim statement (found live at design-lock; NOT in the
original D566 item list — surfaced for a scope decision)
BEFORE (verbatim, §7 last paragraph):
> **ENG-0034 independently blocks post-compromise language** until the contributory check exists: the
> classical half of PCS is currently voidable by the peer.

AFTER (proposed):
> **ENG-0034 is now closed (NA-0628, D-1251/D-1252):** every LIVE X25519 DH output is checked for the
> all-zero (non-contributory) value and fails closed (RFC 7748 §6.1), so the classical half of PCS is
> no longer voidable by the peer at the code level. Post-compromise language remains blocked by the
> A1–A8 abstractions, ENG-0035, and independent human review — the analysis result is unchanged.

This corrects a FACT (the check now exists); it does NOT move the claim (§7's drafted sentences and
their "not a post-quantum / not proven / not a substitute for review" boundary are untouched). **It is
claim-boundary-bearing, so it needs the same operator approval as C1/C3. If the operator prefers to
keep DOC-G4-002 out of this lane, S1's FORMAL_VERIFICATION_PLAN update stands alone and DOC-G4-002 §7
is deferred — but then the analysis record carries a statement that is factually false on current
main, which is the exact drift this lane exists to fix.**

---

## Proof that no claim moves (Operator Decision 1 discharge)
- No gate STATUS cell changes (S3, S7 assert byte-equality on the STATUS columns).
- No "research-stage / Triple-Ratchet-style / not-proven / no-post-quantum / no-post-compromise /
  no-production" sentence is removed or weakened anywhere. C1's 1st paragraph, C3's "still required"
  column, and C4's §7 drafted sentences are all KEPT VERBATIM.
- Every edit is additive-evidence (S2–S4, C1-second-half, C3) or a factual correction of a now-closed
  blocker (S1, C4). The default — NO CHANGE to the claim — holds.

## Verification plan (Phase 4)
Link/anchor check; `git diff` shows no `docs/canonical/**`; a byte-equality assert on the
`WEBSITE_CLAIM_MATRIX.md` STATUS column and the `RELEASE_READINESS_EVIDENCE_MAP.md` gate STATUS cells
vs `HEAD`; the doc-staleness lint runs green on the freshly-updated set and is proved to FAIL on a
synthetic stale doc.
