# NA-0631 — DESIGN-LOCK (external-review-bundle assembly per D568)

Goals: G4, G5. Directive: QSL-DIR-2026-07-10-568 (D568, APPROVED). Base: `main == 27efec5a`. 2026-07-10.

**Claim rule (Operator Decision 1): the claim boundary does NOT move.** The lane strengthens how the
EVIDENCE is presented to a reviewer. Fail-closed: the "What Is Not Proven" and "Safe Public Wording"
sections are held BYTE-STABLE, and no `docs/canonical/**` or `DOC-G4-002` is edited (Decision 3 —
reference the abstraction table, do not inline).

## Staleness verified live (the gap the lane closes)
`docs/public/EXTERNAL_REVIEW_PACKAGE.md` (306 lines) exists; NA-0629 updated only its Executive
Summary. Re-verified at Phase 0: "What Is Currently Proven" has 0 ProVerif rows; "Reproducible
Commands" has 0 ProVerif/refimpl entries; "Evidence Artifact Index" has 0 DOC-G4-002 references. So a
reviewer following the package cannot reach the ProVerif analysis.

## Pinned edits (all additive; the claim-stable sections are untouched)
1. **What Is Currently Proven** — +3 rows: single-root composition (NA-0626), ProVerif symbolic
   analysis (NA-0627, with the symbolic-over-A1–A8 boundary), contributory-DH guard (NA-0628).
2. **Reproducible Commands** — +2 rows: `python3 formal/proverif/run_proverif_checks.py`,
   `cargo test -p quantumshield_refimpl --locked`.
3. **Evidence Artifact Index** — +DOC-G4-002 (call out §2 abstraction table + §7), the ProVerif runner,
   the NA-0628 design-lock + testplan, the NA-0627 code-inspection record.
4. **Recent PR Evidence Table** — +#1533/#1534, #1536/#1537, #1539, #1541.
5. **Known Gaps** — +independent-review-is-THE-gate row, +ENG-0035 2-epoch row (existing rows kept).
6. **Reading order** — a "How to read this package" block near the top, routing to DOC-G4-002 §2 first.
7. **WF-0018 ledger status-line** fixed (header said DONE, inner Status said open).

## Reproducer proof (D568 Phase 4)
- `cargo test -p quantumshield_refimpl --locked` → PASS (89 lib incl. `na0628_every_dh_call_site…`).
- `python3 formal/proverif/run_proverif_checks.py` → re-run this lane; result recorded in the closeout
  journal (17 assertions, sanity pair first). Also CI-gated green on every non-docs main-push.

## Claim-boundary proof (Operator Decision 1)
- "What Is Not Proven" sha256 (first 16): `0313efc327499560` — BYTE-UNCHANGED after edits.
- "Safe Public Wording" sha256 (first 16): `a8242ee01f028e85` — BYTE-UNCHANGED after edits.
- No `docs/canonical/**` and no `DOC-G4-002` diff. Diff grep: no unguarded affirmative post-compromise /
  production / proven / crypto-complete / quantum-proof / vulnerability-free claim.
