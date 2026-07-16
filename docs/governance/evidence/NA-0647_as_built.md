# NA-0647 as-built — Website-support public-docs refresh (D583, D-1270)

Lane: NA-0647 per QSL-DIR-2026-07-15-583 (D583, APPROVED), seated by promotion PR #1575.
Base: qsl-protocol `main` `ac7e850c` (the #1575 seating merge; within D583's "ce634a84 or newer").
Docs-only + ONE verification run. No code, vectors, workflows, or formal models changed.

## §1 The two Phase-1 decisions (stated on record before the doc edits)

**Audit-source decision (ITEM 1): option (b) — website-repository source at the current
production commit.** This is an explicit convention change from the doc's previous
live-page-retrieval convention, and is stated in the refreshed matrix itself. Audited
source: the quantumshieldlabs.org website repository, `main` at commit `21a908a4` (the
WEB-0006 Phase-A closeout merge; the site auto-deploys from `main`, and the Phase-A
closeout verified production serving this content). Rationale: commit-stamped source
audit is reproducible byte-for-byte; live retrieval re-audits rendering, not claims.
Repo-side evidence read at qsl-protocol `ac7e850c`. Recorded in the matrix: audit rows
are date/commit-stamped snapshots; a light row re-touch after the website's Phase B is
expected.

**ENG-0038 framing decision (ITEM 2).** The PROGRESS entry frames the arc EXACTLY as:
(a) found by INTERNAL adversarial review, by protocol-trace analysis, not a PoC exploit;
(b) it CONTRADICTED an earlier internal "no such flaw" conclusion (ENG-0001/NA-0609B),
stated as a calibration point; (c) fixed by authenticating the responder against its
pinned identity KEM key; (d) the class retired; (e) discharged by a BOUNDED
machine-checked model (10,800×10,800 configurations, zero unbound commitments,
non-vacuity demonstrated) with FIVE KNOWN UNMODELED SLICES enumerated and on the public
ledger; (f) explicit negative sentences: does NOT establish vulnerability-free status or
that all flaws are found; NOT an unqualified formal verification; external review NOT
yet commissioned and remains a release gate. The entry's own opening paragraph and the
"what this does and does not establish" section both carry the no-claim boundary.

## §2 ITEM 3 — the demo smoke-run (run FIRST, per D583 Phase 2): **SURPRISE FAILURE, recorded + FLAGGED**

Invocation (exactly what the site's RunDemos page instructs, bare, from the repo root at
`ac7e850c`): `./scripts/demo/qsc_demo_local.sh` (defaults: scenario=happy-path, seed=1).
The script cold-built qsc (`cargo build -p qsc`, finished in 20.25s on a warm cache),
started its local Python inbox relay on 127.0.0.1:9123, and ran the two sends and two
receives.

**Outcome: the script exits 0 and prints `DEMO DONE`, but NOTHING IS DELIVERED.**

Script-final summary (`summary.txt`):

```
scenario=happy-path
seed=1
relay_markers=0
alice_markers=1
bob_markers=1
deliver_count=0
drop_count=0
reorder_count=0
dup_count=0
```

Every qsc invocation emitted exactly one marker and did nothing else:

```
alice.log:    QSC_MARK/1 event=error code=vault_locked op=send reason=explicit_unlock_required
bob.log:      QSC_MARK/1 event=error code=vault_locked op=send reason=explicit_unlock_required
alice_recv.log: QSC_MARK/1 event=error code=vault_locked op=receive reason=explicit_unlock_required
bob_recv.log:   QSC_MARK/1 event=error code=vault_locked op=receive reason=explicit_unlock_required
```

`deterministic_subset.txt` event counts: `error=4`. Both receive output directories
empty. Zero deliveries.

**Diagnosis (read-only):** qsc requires an explicit vault unlock per process —
`--unlock-passphrase-file` / `--unlock-passphrase-env`, or the test-only unsafe fixture
mode (`bootstrap_unlock` in `qsl/qsl-client/qsc/src/main.rs:39-56`; the
`require_unlocked` funnel in `src/lib.rs:158`). The demo script passes none of these and
performs no vault/identity setup. Additionally the script MASKS the failure: every qsc
invocation is `|| true`, and the trailer prints `DEMO DONE` unconditionally, so the
exit code is 0 despite total delivery failure. Note `normalized_counts.txt` even writes
`status=ok`.

**Disposition per D583:** recorded here, FLAGGED as **ENG-0045** on the improvement
ledger and as **WCM-110 (OUTDATED, MUST_FIX)** in the refreshed claim matrix. NOT fixed
in this lane (the script is read/run-only in D583; fixing it is scope creep). The
RunDemos "Local Demo" instructions are therefore NOT confirmed current; the PROGRESS
entry carries a publication-time accuracy note instead of citing a confirmed-current
demo path. Raw run artifacts preserved off-tree (operator scratchpad,
`na0647_demo_smoke_run.log` + `na0647_demo_out/`); the repo tree was left clean
(the script's `_demo_out/` + `_demo_payloads/` were removed after capture).

## §3 ITEM 1 — the claim-matrix refresh (docs/public/WEBSITE_CLAIM_MATRIX.md)

Replaced (the AUDIT half only): the header Last-Updated (→ 2026-07-15) and Directive
line (refresh provenance added); the retrieval-timestamp line (→ the audit-source
convention statement, §1 above); the Pages-Checked table (→ the ten current .org routes,
audited from website-repo source at `21a908a4`, including the shared sections/modals and
`src/links.js`); all 18 old WCM-001..018 rows (they audited the retired .dev site — zero
shared pages) → 15 new rows WCM-101..115 stamped `21a908a4`/`ac7e850c`/2026-07-15;
the Repo-Evidence-Consulted list (→ current evidence: NA-0640 as-built + testplan,
NA-0642 testplan + as-built, NA-0646 as-built, formal/README.md, DOC-G4-002, the
canonical drafts, demo/conformance/privacy docs, the qshield-ci workflow, the NA-0647
smoke-run record); the Top-MUST-FIX list (→ 3 items: the WCM-110 demo fix lane;
protecting the boundary band/absence-of-overclaims through Phase B; the Phase-B re-touch
of the two site cards that still describe this matrix as a pending historical .dev
audit). The Classification Legend was left unchanged (it still governs the new rows).

Row outcome summary: 11 SUPPORTED, 2 OUTDATED (WCM-110 the demo instructions; WCM-112
the site's own "matrix refresh pending" cards, which this refresh makes stale by
merging), 1 OUT_OF_SCOPE_FOR_QSL_PROTOCOL (website legal/ops), 1 SUPPORTED absence row
(no overclaims found anywhere in the source sweep). The matrix got shorter and mostly
SUPPORTED, as D583 predicted.

**PRESERVED VERBATIM (the POLICY half):** the `## NA-0539 Repository Claim Policy
Addendum` and `## NA-0541 Progress Claim Policy Addendum` sections in full — both
wording-policy tables, the "Required no-claim boundaries" list, and the "Progress
evidence and correction wording policy" list. **Byte-proof:** the region from the
NA-0539 header through the end of the NA-0541 policy list (everything strictly between
the audit preamble and `## Pages Checked`) extracted from HEAD (`ac7e850c`) and from the
working tree: 5,220 bytes / 57 lines, `diff` EMPTY, sha256 identical both sides:
`3566f215b961112e51b8db4af949ec29eb54a927b5e5dcb306f56d9b154e46a1`. The lane PR's diff
shows zero hunks inside the policy region.

**Link-target verification (WCM-104 evidence):** all 15 repo deep-link targets in the
website's `src/links.js` verified present at qsl-protocol `ac7e850c` via
`git cat-file -e` (docs/public/{INDEX,WEBSITE_CLAIM_MATRIX,RELEASE_READINESS_EVIDENCE_MAP,
EXTERNAL_REVIEW_PACKAGE,SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY}.md; DOC-CAN-003/-004; the
demo/conformance/privacy docs; .github/workflows/ci.yml [workflow name `qshield-ci`];
docs/governance/evidence/NA-0640_as_built.md; tests/NA-0640_e2e_integration_full_stack_testplan.md;
tests/NA-0642_qsl_server_durability_testplan.md). 15/15 OK.

## §4 ITEM 2 — the catch-up PROGRESS entry + coupled touches

New `docs/public/progress/2026-07-15.md`, in the existing entry format (header block;
opening no-claim-boundary paragraph; themed sections with per-lane bold bullets citing
lane IDs + evidence paths; a "what this does and does not establish" section; closing
review invitation). Covers BOTH arcs since 2026-07-10:

- **The ENG-0038 arc** (first public record): NA-0632 finding → NA-0633 fix → NA-0634
  class retirement → NA-0636/D572 bounded-model discharge, framed per §1's decision,
  with a dedicated arc-boundary paragraph ("what this arc establishes / does not
  establish") in addition to the entry-wide boundary section. Links formal/README.md and
  DOC-G4-002 as the reviewer-facing records. Evidence citations:
  NA-0632_adversarial_reanalysis.md, NA-0633_design_lock.md (the NA-0633 tracked
  evidence file), NA-0634_as_built.md, NA-0636_as_built.md.
- **The product arc**: NA-0640 (e2e, dev-harness/controlled-conditions framing), NA-0642
  (durable relay; repo evidence, not a deployed-relay claim; ENG-0039 open), NA-0644
  (ack client; opt-in; the bounded ENG-0042 seam stated), NA-0645 (TUI retirement;
  ENG-0044 stated), NA-0646 (core extraction; engineering-architecture-not-product/SDK
  framing).
- **A publication-time accuracy note** recording the §2 demo-script failure honestly
  (found while preparing this publication; flagged, not fixed here).

Coupled touches in the SAME diff: `docs/public/INDEX.md` — the hardcoded latest-entry
sentence (was "July 10, 2026") + list + inline summary updated to the 2026-07-15 entry
(the old 2026-07-10 summary retained beneath it); `docs/public/PROGRESS.md` — the new
list line + Last-Updated → 2026-07-15.

## §5 Validation

- **Scope guard:** changed paths = docs/public/WEBSITE_CLAIM_MATRIX.md,
  docs/public/progress/2026-07-15.md (new), docs/public/PROGRESS.md,
  docs/public/INDEX.md, plus the governance/closeout set (this file;
  tests/NA-0647_website_support_docs_testplan.md; DECISIONS.md D-1270; TRACEABILITY.md;
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md; docs/ops/IMPROVEMENT_LEDGER.md ENG-0045;
  NEXT_ACTIONS.md). ⊆ the D583 allowed list. ZERO code/script/vector/workflow/formal
  edits; `scripts/demo/qsc_demo_local.sh` untouched (run only);
  EXTERNAL_REVIEW_PACKAGE.md + RELEASE_READINESS_EVIDENCE_MAP.md untouched (next lane);
  no website-repo change; the public-safety gate and its lists untouched.
- **Policy sections byte-preserved:** §3 proof (sha256-identical, diff empty).
- **Relative-markdown-link pre-check** (the public-safety gate's markdown check,
  replicated offline over the four changed public docs): 0 missing targets.
- **Public-safety gate + goal-lint:** run against the lane PR (results recorded in the
  final response; the gate also runs as the required PR check). No gate or gate-list
  edit was made or needed — the wording passes honestly.

## §6 Result classification and the deferred/flagged set

**WEBSITE_DOCS_REFRESH_STOP** — by D583's own classification: "the smoke-run reveals a
demo/site problem needing a fix out of scope (record + flag, do the rest, note in
closeout)". The rest WAS done: the claim-matrix audit is re-targeted with the policy
sections byte-preserved; the PROGRESS entry covers both arcs within the no-claim
boundaries with the coupled touches; the smoke-run outcome is recorded. The STOP
classification records ONLY the flagged demo-path problem (ENG-0045); no wording,
boundary, policy-section, or scope stop occurred.

Flagged: **ENG-0045** (P2): fix `scripts/demo/qsc_demo_local.sh` — vault-unlock
bootstrap + fail-loud behavior — then re-verify and re-touch WCM-110 (and the RunDemos
wording if needed). Its own lane; NOT this one.

Deferred (NOT claimed by this lane): the EXTERNAL_REVIEW_PACKAGE.md refresh
(ENG-0038-blind) + its RELEASE_READINESS_EVIDENCE_MAP.md companion — the COMMITTED NEXT
lane; the website's Phase B content sync (website repo, qsite governance) — expected to
consume this lane's two refreshed docs and re-touch WCM-110/WCM-112.
